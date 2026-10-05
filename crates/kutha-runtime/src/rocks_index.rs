//! Droppable RocksDB locator lease over the segmented log (ROCK-01).
//! Not wired into `Runtime`, `store::open`, `Persister`, fold, or hydrate.
//!
//! Keys: `e:` + EventId, `d:` + delivery_key UTF-8, `m:v`, `m:tip`.
//! Values are fixed locators / ids / meta — never event payloads.

use crate::segment::{self, HEADER_LEN, MANIFEST_REL, Manifest};
use crate::store;
use crc32fast::Hasher;
use kutha_common::{Event, EventId, Op};
use rocksdb::{DB, Options};
use std::fs::{self, File};
use std::io::{self, Error, ErrorKind, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

const FORMAT_VERSION: u32 = 1;
const LOCATOR_LEN: usize = 16;
const TIP_HEX_LEN: usize = 64;
const TIP_LEN: usize = TIP_HEX_LEN + 8 + 16;
const KEY_MV: &[u8] = b"m:v";
const KEY_TIP: &[u8] = b"m:tip";

/// On-disk frame locator: segment seq, byte offset, frame length (big-endian in Rocks).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Locator {
    pub seq: u32,
    pub offset: u64,
    pub frame_len: u32,
}

struct Located {
    seq: u32,
    offset: u64,
    frame_len: u32,
    event: Event,
}

/// Open RocksDB handle over a store directory. Dropping it does not drop the log.
pub struct SegmentIndex {
    db: DB,
    store_dir: PathBuf,
}

fn rocks_err(err: rocksdb::Error) -> Error {
    Error::new(ErrorKind::Other, err.to_string())
}

fn invalid(message: impl Into<String>) -> Error {
    Error::new(ErrorKind::InvalidData, message.into())
}

fn u32_le(slice: &[u8]) -> io::Result<u32> {
    let arr: [u8; 4] = slice
        .try_into()
        .map_err(|_| invalid("short little-endian u32"))?;
    Ok(u32::from_le_bytes(arr))
}

fn is_seal_payload(payload: &[u8]) -> bool {
    serde_json::from_slice::<serde_json::Value>(payload)
        .ok()
        .and_then(|v| v.get("k").and_then(|k| k.as_str()).map(|s| s == "seal"))
        .unwrap_or(false)
}

fn event_key(id: EventId) -> Vec<u8> {
    let mut k = b"e:".to_vec();
    k.extend_from_slice(id.as_bytes());
    k
}

fn delivery_key_bytes(key: &str) -> Vec<u8> {
    let mut k = b"d:".to_vec();
    k.extend_from_slice(key.as_bytes());
    k
}

fn encode_locator(loc: Locator) -> [u8; LOCATOR_LEN] {
    let mut out = [0u8; LOCATOR_LEN];
    out[0..4].copy_from_slice(&loc.seq.to_be_bytes());
    out[4..12].copy_from_slice(&loc.offset.to_be_bytes());
    out[12..16].copy_from_slice(&loc.frame_len.to_be_bytes());
    out
}

fn decode_locator(bytes: &[u8]) -> io::Result<Locator> {
    if bytes.len() != LOCATOR_LEN {
        return Err(invalid("rocks e: value is not a 16-byte locator"));
    }
    let seq_arr: [u8; 4] = bytes[0..4].try_into().map_err(|_| invalid("locator seq"))?;
    let off_arr: [u8; 8] = bytes[4..12]
        .try_into()
        .map_err(|_| invalid("locator offset"))?;
    let len_arr: [u8; 4] = bytes[12..16]
        .try_into()
        .map_err(|_| invalid("locator frame_len"))?;
    Ok(Locator {
        seq: u32::from_be_bytes(seq_arr),
        offset: u64::from_be_bytes(off_arr),
        frame_len: u32::from_be_bytes(len_arr),
    })
}

fn decode_event_id(bytes: &[u8]) -> io::Result<EventId> {
    let arr: [u8; 16] = bytes
        .try_into()
        .map_err(|_| invalid("value is not a 16-byte EventId"))?;
    Ok(EventId::from_bytes(arr))
}

struct Tip {
    sealed_hex: String,
    count: u64,
    last: EventId,
}

fn sealed_hex(man: &Manifest) -> String {
    man.sealed
        .last()
        .map(|s| s.chain.clone())
        .filter(|c| c.len() == TIP_HEX_LEN)
        .unwrap_or_else(|| "0".repeat(TIP_HEX_LEN))
}

fn encode_tip(tip: &Tip) -> io::Result<[u8; TIP_LEN]> {
    if tip.sealed_hex.len() != TIP_HEX_LEN {
        return Err(invalid("sealed chain hex must be 64 characters"));
    }
    let mut out = [0u8; TIP_LEN];
    out[..TIP_HEX_LEN].copy_from_slice(tip.sealed_hex.as_bytes());
    out[TIP_HEX_LEN..TIP_HEX_LEN + 8].copy_from_slice(&tip.count.to_be_bytes());
    out[TIP_HEX_LEN + 8..].copy_from_slice(tip.last.as_bytes());
    Ok(out)
}

fn decode_tip(bytes: &[u8]) -> io::Result<Tip> {
    if bytes.len() != TIP_LEN {
        return Err(invalid("m:tip value is not a fixed meta record"));
    }
    let hex = std::str::from_utf8(&bytes[..TIP_HEX_LEN])
        .map_err(|_| invalid("m:tip sealed chain is not UTF-8 hex"))?
        .to_string();
    let count_arr: [u8; 8] = bytes[TIP_HEX_LEN..TIP_HEX_LEN + 8]
        .try_into()
        .map_err(|_| invalid("m:tip count"))?;
    let last = decode_event_id(&bytes[TIP_HEX_LEN + 8..])?;
    Ok(Tip {
        sealed_hex: hex,
        count: u64::from_be_bytes(count_arr),
        last,
    })
}

fn require_segmented(store_dir: &Path) -> io::Result<()> {
    if store_dir.join(MANIFEST_REL).is_file() {
        return Ok(());
    }
    Err(invalid("run store::migrate first"))
}

fn scan_file(path: &Path, seq: u32, sealed: bool) -> io::Result<Vec<Located>> {
    let bytes = fs::read(path)?;
    if (bytes.len() as u64) < HEADER_LEN {
        return Err(invalid("truncated segment header"));
    }
    let mut off = HEADER_LEN as usize;
    let mut out = Vec::new();
    loop {
        if off == bytes.len() {
            if sealed {
                return Err(invalid("truncated sealed segment (no trailer)"));
            }
            break;
        }
        if off + 8 > bytes.len() {
            if sealed {
                return Err(invalid("truncated sealed segment"));
            }
            break;
        }
        let len = u32_le(&bytes[off..off + 4])? as usize;
        let crc_stored = u32_le(&bytes[off + 4..off + 8])?;
        let end = off
            .checked_add(8)
            .and_then(|x| x.checked_add(len))
            .ok_or_else(|| invalid("segment frame overflow"))?;
        if end > bytes.len() {
            if sealed {
                return Err(invalid("truncated sealed segment"));
            }
            break;
        }
        let payload = &bytes[off + 8..end];
        let mut hasher = Hasher::new();
        hasher.update(payload);
        if hasher.finalize() != crc_stored {
            if sealed {
                return Err(invalid("sealed frame CRC mismatch"));
            }
            break;
        }
        if is_seal_payload(payload) {
            break;
        }
        let event: Event = serde_json::from_slice(payload)
            .map_err(|err| invalid(format!("segment frame is not an Event: {err}")))?;
        let frame_len = u32::try_from(end - off).map_err(|_| invalid("frame length overflow"))?;
        out.push(Located {
            seq,
            offset: off as u64,
            frame_len,
            event,
        });
        off = end;
    }
    Ok(out)
}

fn located_committed(store_dir: &Path) -> io::Result<(Manifest, Vec<Located>, Tip)> {
    require_segmented(store_dir)?;
    let verified = segment::load_events(store_dir)?;
    let man = segment::read_manifest(store_dir)?;
    let mut located = Vec::new();
    for entry in &man.sealed {
        let seq = segment::parse_seg_seq(&entry.name)?;
        located.extend(scan_file(&store_dir.join(&entry.name), seq, true)?);
    }
    let active_seq = segment::parse_seg_seq(&man.active)?;
    located.extend(scan_file(&store_dir.join(&man.active), active_seq, false)?);
    if located.len() != verified.len()
        || located
            .iter()
            .zip(verified.iter())
            .any(|(row, ev)| row.event.id != ev.id)
    {
        return Err(invalid(
            "index scan disagrees with the verified segment reader",
        ));
    }
    let kept = store::committed_prefix(verified).len();
    located.truncate(kept);
    let last = located
        .last()
        .map(|row| row.event.id)
        .unwrap_or_else(EventId::nil);
    let tip = Tip {
        sealed_hex: sealed_hex(&man),
        count: located.len() as u64,
        last,
    };
    Ok((man, located, tip))
}

fn write_rows(db: &DB, located: &[Located]) -> io::Result<()> {
    for row in located {
        db.put(
            &event_key(row.event.id),
            encode_locator(Locator {
                seq: row.seq,
                offset: row.offset,
                frame_len: row.frame_len,
            }),
        )
        .map_err(rocks_err)?;
        if let Op::Assert {
            delivery_key: Some(k),
            ..
        } = &row.event.op
        {
            if !k.is_empty() {
                db.put(delivery_key_bytes(k), row.event.id.as_bytes())
                    .map_err(rocks_err)?;
            }
        }
    }
    Ok(())
}

fn write_meta(db: &DB, tip: &Tip) -> io::Result<()> {
    db.put(KEY_MV, FORMAT_VERSION.to_be_bytes())
        .map_err(rocks_err)?;
    db.put(KEY_TIP, encode_tip(tip)?).map_err(rocks_err)?;
    Ok(())
}

fn destroy_dir(rocks_dir: &Path) -> io::Result<()> {
    if rocks_dir.exists() {
        DB::destroy(&Options::default(), rocks_dir).map_err(rocks_err)?;
    }
    Ok(())
}

fn open_db(rocks_dir: &Path, create: bool) -> io::Result<DB> {
    let mut opts = Options::default();
    opts.create_if_missing(create);
    DB::open(&opts, rocks_dir).map_err(rocks_err)
}

fn stored_version(db: &DB) -> io::Result<u32> {
    match db.get(KEY_MV).map_err(rocks_err)? {
        None => Err(invalid("missing m:v")),
        Some(bytes) => {
            let arr: [u8; 4] = bytes
                .as_slice()
                .try_into()
                .map_err(|_| invalid("m:v is not u32"))?;
            Ok(u32::from_be_bytes(arr))
        }
    }
}

fn stored_tip(db: &DB) -> io::Result<Tip> {
    match db.get(KEY_TIP).map_err(rocks_err)? {
        None => Err(invalid("missing m:tip")),
        Some(bytes) => decode_tip(&bytes),
    }
}

fn read_frame(store_dir: &Path, loc: Locator) -> io::Result<Event> {
    let path = store_dir.join(segment::segment_name(loc.seq));
    let mut file = File::open(&path).map_err(|err| {
        if err.kind() == ErrorKind::NotFound {
            invalid(format!("locator segment missing: {}", path.display()))
        } else {
            err
        }
    })?;
    file.seek(SeekFrom::Start(loc.offset))?;
    let mut buf = vec![0u8; loc.frame_len as usize];
    file.read_exact(&mut buf)?;
    if buf.len() < 8 {
        return Err(invalid("short frame"));
    }
    let payload_len = u32_le(&buf[0..4])? as usize;
    let crc_stored = u32_le(&buf[4..8])?;
    if payload_len.saturating_add(8) != loc.frame_len as usize {
        return Err(invalid("frame length mismatch"));
    }
    let payload = &buf[8..];
    let mut hasher = Hasher::new();
    hasher.update(payload);
    if hasher.finalize() != crc_stored {
        return Err(invalid("frame CRC mismatch"));
    }
    serde_json::from_slice(payload).map_err(|err| invalid(format!("frame is not an Event: {err}")))
}

impl SegmentIndex {
    /// Index the committed prefix of a segmented store. Rejects legacy jsonl dirs.
    pub fn build(store_dir: impl AsRef<Path>, rocks_dir: impl AsRef<Path>) -> io::Result<Self> {
        let store_dir = store_dir.as_ref().to_path_buf();
        let rocks_dir = rocks_dir.as_ref().to_path_buf();
        let (_man, located, tip) = located_committed(&store_dir)?;
        destroy_dir(&rocks_dir)?;
        let db = open_db(&rocks_dir, true)?;
        write_rows(&db, &located)?;
        write_meta(&db, &tip)?;
        db.flush().map_err(rocks_err)?;
        Ok(Self { db, store_dir })
    }

    /// Open if `m:tip` matches the log tip; otherwise destroy `rocks_dir` and rebuild.
    pub fn open_or_rebuild(
        store_dir: impl AsRef<Path>,
        rocks_dir: impl AsRef<Path>,
    ) -> io::Result<Self> {
        let store_dir_buf = store_dir.as_ref().to_path_buf();
        let rocks_dir_buf = rocks_dir.as_ref().to_path_buf();
        if rocks_dir_buf.exists() {
            match Self::try_open(store_dir_buf.clone(), rocks_dir_buf.clone()) {
                Ok(idx) => {
                    if idx.tip_matches_log()? && idx.tip_event_ok() {
                        return Ok(idx);
                    }
                }
                Err(_) => {}
            }
            destroy_dir(&rocks_dir_buf)?;
        }
        Self::build(store_dir_buf, rocks_dir_buf)
    }

    fn try_open(store_dir: PathBuf, rocks_dir: PathBuf) -> io::Result<Self> {
        require_segmented(&store_dir)?;
        let db = open_db(&rocks_dir, false)?;
        if stored_version(&db)? != FORMAT_VERSION {
            return Err(invalid("unsupported rocks-index format"));
        }
        let _ = stored_tip(&db)?;
        Ok(Self { db, store_dir })
    }

    fn tip_matches_log(&self) -> io::Result<bool> {
        let stored = stored_tip(&self.db)?;
        let (_man, _located, current) = located_committed(&self.store_dir)?;
        Ok(stored.sealed_hex == current.sealed_hex
            && stored.count == current.count
            && stored.last == current.last)
    }

    fn tip_event_ok(&self) -> bool {
        match stored_tip(&self.db) {
            Ok(tip) if tip.count == 0 => true,
            Ok(tip) => matches!(self.get_event(tip.last), Ok(Some(ev)) if ev.id == tip.last),
            Err(_) => false,
        }
    }

    /// Index only events after the recorded tip when the log only grew.
    pub fn catch_up(&self) -> io::Result<usize> {
        let stored = stored_tip(&self.db)?;
        let (_man, located, current) = located_committed(&self.store_dir)?;
        if (located.len() as u64) < stored.count {
            return Err(invalid("log is shorter than the recorded index tip"));
        }
        if stored.count > 0 {
            let prefix_last = located
                .get(stored.count as usize - 1)
                .map(|row| row.event.id);
            if prefix_last != Some(stored.last) {
                return Err(invalid("catch_up prefix last EventId diverged"));
            }
        }
        if located.len() as u64 == stored.count && current.last == stored.last {
            return Ok(0);
        }
        let tail = &located[stored.count as usize..];
        write_rows(&self.db, tail)?;
        write_meta(&self.db, &current)?;
        self.db.flush().map_err(rocks_err)?;
        Ok(tail.len())
    }

    pub fn locate(&self, id: EventId) -> io::Result<Option<Locator>> {
        match self.db.get(event_key(id)).map_err(rocks_err)? {
            None => Ok(None),
            Some(bytes) => Ok(Some(decode_locator(&bytes)?)),
        }
    }

    pub fn get_event(&self, id: EventId) -> io::Result<Option<Event>> {
        let Some(loc) = self.locate(id)? else {
            return Ok(None);
        };
        let event = read_frame(&self.store_dir, loc)?;
        if event.id != id {
            return Err(invalid(
                "locator points at a frame whose id differs from the key",
            ));
        }
        Ok(Some(event))
    }

    pub fn by_delivery_key(&self, key: &str) -> io::Result<Option<EventId>> {
        match self.db.get(delivery_key_bytes(key)).map_err(rocks_err)? {
            None => Ok(None),
            Some(bytes) => Ok(Some(decode_event_id(&bytes)?)),
        }
    }

    /// Scan `e:` keys against a stored digest (no log IO). ROCK-02 / Phase 61.
    ///
    /// RED stub: always `Ok` until the digest check lands. Poisoned locators
    /// stay invisible here; `get_event` still re-verifies the frame.
    pub fn verify_full(&self) -> io::Result<()> {
        Ok(())
    }
}
