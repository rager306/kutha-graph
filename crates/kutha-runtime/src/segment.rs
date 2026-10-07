//! Sealed segments + manifest (M002 S06 / SEG-01). Physical encoding of the event log.
//!
//! Magic `KUTHASEG` v1; header is 44 bytes: magic + u32 version + previous-segment
//! chain tip (32 bytes; `[0; 32]` only for the first segment). Event frames reuse
//! [`crate::wal::frame_bytes`]. Segment *k* starts hashing from the final chain tip
//! of segment *k-1*. This does **not** stop an attacker who rewrites every file
//! including the manifest and recomputes hashes; the manifest tip is not externally
//! anchored. Empty/segmented persist writes this layout in plan 54-02; legacy dirs
//! stay jsonl+WAL.

use crate::store::{json_err, replace_via_tmp};
use crate::wal;
use crc32fast::Hasher;
use kutha_common::{Event, EventId};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::cell::Cell;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

pub(crate) const MAGIC: &[u8; 8] = b"KUTHASEG";
pub(crate) const VERSION: u32 = 1;
/// Magic + u32 v1 + previous-segment chain tip (H0 = 32 zero bytes for seq 1).
#[doc(hidden)]
pub const HEADER_LEN: u64 = 44;
/// Manifest file name (`manifest.json`).
#[doc(hidden)]
pub const MANIFEST_REL: &str = "manifest.json";
const MANIFEST_TMP: &str = "manifest.json.tmp";

/// Rollover threshold: sealed file size including header + frames (trailer extra is allowed).
pub const SEGMENT_MAX_BYTES: u64 = 64 * 1024 * 1024;
/// Rollover threshold: event frames in the active segment.
pub const SEGMENT_MAX_EVENTS: usize = 65536;

thread_local! {
    static TEST_MAX_BYTES: Cell<Option<u64>> = const { Cell::new(None) };
    static TEST_MAX_EVENTS: Cell<Option<usize>> = const { Cell::new(None) };
}

/// Test seam: lower rollover thresholds for this thread. `None` restores the pub const.
#[doc(hidden)]
pub fn set_test_segment_limits(max_bytes: Option<u64>, max_events: Option<usize>) {
    TEST_MAX_BYTES.with(|c| c.set(max_bytes));
    TEST_MAX_EVENTS.with(|c| c.set(max_events));
}

/// Clear the rollover test seam.
#[doc(hidden)]
pub fn reset_test_segment_limits() {
    set_test_segment_limits(None, None);
}

pub(crate) fn effective_max_bytes() -> u64 {
    TEST_MAX_BYTES
        .with(|c| c.get())
        .unwrap_or(SEGMENT_MAX_BYTES)
}

pub(crate) fn effective_max_events() -> usize {
    TEST_MAX_EVENTS
        .with(|c| c.get())
        .unwrap_or(SEGMENT_MAX_EVENTS)
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SealedMeta {
    pub name: String,
    pub first: EventId,
    pub last: EventId,
    pub count: u64,
    pub bytes: u64,
    pub chain: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Manifest {
    pub v: u32,
    pub sealed: Vec<SealedMeta>,
    pub active: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct SealTrailer {
    k: String,
    n: u64,
    first: EventId,
    last: EventId,
    bytes: u64,
    chain: String,
}

fn invalid(msg: impl Into<String>) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::InvalidData, msg.into())
}

pub(crate) fn hex32(digest: [u8; 32]) -> String {
    let mut out = String::with_capacity(64);
    for b in digest {
        out.push_str(&format!("{b:02x}"));
    }
    out
}

pub(crate) fn chain_absorb(h: [u8; 32], frame: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(h);
    hasher.update(frame);
    hasher.finalize().into()
}

fn payload_frame(payload: &[u8]) -> Vec<u8> {
    let mut hasher = Hasher::new();
    hasher.update(payload);
    let crc = hasher.finalize();
    let mut out = Vec::with_capacity(8 + payload.len());
    out.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    out.extend_from_slice(&crc.to_le_bytes());
    out.extend_from_slice(payload);
    out
}

pub(crate) fn segment_name(seq: u32) -> String {
    format!("seg-{seq:08}.log")
}

pub(crate) fn write_header(f: &mut impl Write, prev: [u8; 32]) -> std::io::Result<()> {
    f.write_all(MAGIC)?;
    f.write_all(&VERSION.to_le_bytes())?;
    f.write_all(&prev)?;
    Ok(())
}

fn seal_payload(
    n: u64,
    first: EventId,
    last: EventId,
    bytes: u64,
    chain: &str,
) -> std::io::Result<Vec<u8>> {
    serde_json::to_vec(&SealTrailer {
        k: "seal".to_string(),
        n,
        first,
        last,
        bytes,
        chain: chain.to_string(),
    })
    .map_err(json_err)
}

pub(crate) fn seal_frame(
    n: u64,
    first: EventId,
    last: EventId,
    body_len: u64,
    chain: &str,
) -> std::io::Result<Vec<u8>> {
    let mut bytes = body_len.saturating_add(8).saturating_add(96);
    for _ in 0..8 {
        let payload = seal_payload(n, first, last, bytes, chain)?;
        let total = body_len
            .saturating_add(8)
            .saturating_add(payload.len() as u64);
        if total == bytes {
            return Ok(payload_frame(&payload));
        }
        bytes = total;
    }
    Err(invalid("seal trailer size did not converge"))
}

fn segment_tmp_path(path: &Path) -> PathBuf {
    let mut os = path.as_os_str().to_owned();
    os.push(".tmp");
    PathBuf::from(os)
}

fn write_segment_file(
    path: &Path,
    events: &[Event],
    sealed: bool,
    prev: [u8; 32],
) -> std::io::Result<(Option<SealedMeta>, [u8; 32])> {
    write_segment_file_commit(path, events, sealed, prev, true)
}

/// Test seam (PA-02): write the segment temp sibling then return Err without renaming dest.
#[doc(hidden)]
pub fn abort_write_segment_before_rename(
    path: &Path,
    events: &[Event],
    sealed: bool,
    prev: [u8; 32],
) -> std::io::Result<()> {
    write_segment_file_commit(path, events, sealed, prev, false).map(|_| ())
}

fn write_segment_file_commit(
    path: &Path,
    events: &[Event],
    sealed: bool,
    prev: [u8; 32],
    commit: bool,
) -> std::io::Result<(Option<SealedMeta>, [u8; 32])> {
    if sealed && events.is_empty() {
        return Err(invalid("sealed segment must contain at least one event"));
    }
    let mut tip = prev;
    let mut meta: Option<SealedMeta> = None;
    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("seg.log")
        .to_string();
    let tmp = segment_tmp_path(path);
    replace_via_tmp(path, &tmp, commit, |f| {
        let mut h = prev;
        let mut frames_len = 0u64;
        write_header(f, prev)?;
        for e in events {
            let frame = wal::frame_bytes(e)?;
            h = chain_absorb(h, &frame);
            f.write_all(&frame)?;
            frames_len = frames_len.saturating_add(frame.len() as u64);
        }
        let chain = hex32(h);
        let body_len = HEADER_LEN + frames_len;
        let trailer = if sealed {
            Some(seal_frame(
                events.len() as u64,
                events[0].id,
                events[events.len() - 1].id,
                body_len,
                &chain,
            )?)
        } else {
            None
        };
        let bytes = body_len + trailer.as_ref().map(|t| t.len() as u64).unwrap_or(0);
        if let Some(t) = &trailer {
            f.write_all(t)?;
        }
        tip = h;
        if sealed {
            meta = Some(SealedMeta {
                name: name.clone(),
                first: events[0].id,
                last: events[events.len() - 1].id,
                count: events.len() as u64,
                bytes,
                chain,
            });
        }
        Ok(())
    })?;
    Ok((meta, tip))
}

pub(crate) fn write_manifest(dir: &Path, manifest: &Manifest) -> std::io::Result<()> {
    replace_via_tmp(
        &dir.join(MANIFEST_REL),
        &dir.join(MANIFEST_TMP),
        true,
        |f| {
            serde_json::to_writer(&mut *f, manifest).map_err(json_err)?;
            Ok(())
        },
    )
}

/// Test-only writer: byte-valid sealed segments + active + `manifest.json`.
/// Does not touch `events.jsonl` / `events.wal`.
#[doc(hidden)]
pub fn write_test_segmented(
    dir: &Path,
    events: &[Event],
    sealed_counts: &[usize],
    active_count: usize,
) -> std::io::Result<()> {
    fs::create_dir_all(dir)?;
    let need: usize = sealed_counts.iter().copied().sum::<usize>() + active_count;
    if need != events.len() {
        return Err(invalid(format!(
            "segment layout covers {need} events, got {}",
            events.len()
        )));
    }
    let mut seq = 1u32;
    let mut off = 0usize;
    let mut sealed = Vec::new();
    let mut prev = [0u8; 32];
    for &n in sealed_counts {
        let name = segment_name(seq);
        seq = seq.saturating_add(1);
        let chunk = &events[off..off + n];
        off += n;
        let (meta, tip) = write_segment_file(&dir.join(&name), chunk, true, prev)?;
        prev = tip;
        sealed.push(meta.ok_or_else(|| invalid("sealed write returned no meta"))?);
    }
    let active = segment_name(seq);
    write_segment_file(
        &dir.join(&active),
        &events[off..off + active_count],
        false,
        prev,
    )?;
    write_manifest(
        dir,
        &Manifest {
            v: 1,
            sealed,
            active,
        },
    )
}

/// Scan a well-formed (non-torn) segment that has no trailer yet and rewrite it sealed.
#[doc(hidden)]
pub fn rewrite_segment_with_seal_trailer(path: &Path) -> std::io::Result<()> {
    let bytes = fs::read(path)?;
    let scan = scan_segment(&bytes, false)?;
    if scan.trailer.is_some() {
        return Err(invalid("unexpected seal trailer while rewriting"));
    }
    if scan.consumed != bytes.len() {
        return Err(invalid("trailing bytes in unsealed segment"));
    }
    write_segment_file(path, &scan.events, true, scan.prev)?;
    Ok(())
}

fn u32_le(slice: &[u8]) -> std::io::Result<u32> {
    if slice.len() < 4 {
        return Err(invalid("short u32"));
    }
    let mut buf = [0u8; 4];
    buf.copy_from_slice(&slice[..4]);
    Ok(u32::from_le_bytes(buf))
}

fn trailer_payload(payload: &[u8]) -> Option<SealTrailer> {
    let t: SealTrailer = serde_json::from_slice(payload).ok()?;
    (t.k == "seal").then_some(t)
}

struct Scan {
    events: Vec<Event>,
    chain: [u8; 32],
    prev: [u8; 32],
    trailer: Option<SealTrailer>,
    consumed: usize,
}

fn parse_header(bytes: &[u8]) -> std::io::Result<[u8; 32]> {
    if bytes.len() < HEADER_LEN as usize {
        return Err(invalid("truncated segment header"));
    }
    if bytes[0..8] != MAGIC[..] {
        return Err(invalid("bad segment magic"));
    }
    let ver = u32_le(&bytes[8..12])?;
    if ver != VERSION {
        return Err(invalid("unsupported segment version"));
    }
    let mut prev = [0u8; 32];
    prev.copy_from_slice(&bytes[12..HEADER_LEN as usize]);
    Ok(prev)
}

fn scan_segment(bytes: &[u8], sealed: bool) -> std::io::Result<Scan> {
    let prev = parse_header(bytes)?;
    let mut off = HEADER_LEN as usize;
    let mut events = Vec::new();
    let mut chain = prev;
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
        let frame = &bytes[off..end];
        if let Some(t) = trailer_payload(payload) {
            if sealed && end != bytes.len() {
                return Err(invalid("extra bytes after sealed trailer"));
            }
            return Ok(Scan {
                events,
                chain,
                prev,
                trailer: Some(t),
                consumed: end,
            });
        }
        match serde_json::from_slice::<Event>(payload) {
            Ok(e) => {
                chain = chain_absorb(chain, frame);
                events.push(e);
                off = end;
            }
            Err(_) => {
                if sealed {
                    return Err(invalid("sealed frame is not an Event"));
                }
                break;
            }
        }
    }
    Ok(Scan {
        events,
        chain,
        prev,
        trailer: None,
        consumed: off,
    })
}

pub(crate) fn parse_seg_seq(name: &str) -> std::io::Result<u32> {
    let rest = name
        .strip_prefix("seg-")
        .and_then(|s| s.strip_suffix(".log"))
        .ok_or_else(|| invalid(format!("invalid segment name {name}")))?;
    if rest.len() != 8 || !rest.as_bytes().iter().all(|b| b.is_ascii_digit()) {
        return Err(invalid(format!("invalid segment name {name}")));
    }
    rest.parse::<u32>()
        .map_err(|_| invalid(format!("invalid segment name {name}")))
}

pub(crate) fn read_manifest(dir: &Path) -> std::io::Result<Manifest> {
    let f = File::open(dir.join(MANIFEST_REL))?;
    let m: Manifest = serde_json::from_reader(f).map_err(json_err)?;
    if m.v != 1 {
        return Err(invalid("unsupported manifest version"));
    }
    Ok(m)
}

fn verify_sealed(
    path: &Path,
    meta: &SealedMeta,
    expected_prev: [u8; 32],
) -> std::io::Result<(Vec<Event>, [u8; 32])> {
    // Documented order: missing → chain link → scan (CRC/trailer/truncated/extra)
    // → count → first/last → bytes → hash chain → manifest fields.
    if !path.exists() {
        return Err(invalid(format!("missing sealed segment {}", meta.name)));
    }
    let bytes = fs::read(path)?;
    let file_len = bytes.len() as u64;
    let prev = parse_header(&bytes)?;
    if prev != expected_prev {
        return Err(invalid("chain link mismatch"));
    }
    let scan = scan_segment(&bytes, true)?;
    let t = scan
        .trailer
        .ok_or_else(|| invalid("truncated sealed segment (no trailer)"))?;
    if t.n != scan.events.len() as u64 {
        return Err(invalid("sealed trailer count mismatch"));
    }
    let first = scan
        .events
        .first()
        .ok_or_else(|| invalid("sealed segment must contain at least one event"))?
        .id;
    let last = scan
        .events
        .last()
        .ok_or_else(|| invalid("sealed segment must contain at least one event"))?
        .id;
    if t.first != first || t.last != last {
        return Err(invalid("sealed trailer first/last mismatch"));
    }
    if t.bytes != file_len {
        return Err(invalid("sealed trailer bytes mismatch"));
    }
    let chain = hex32(scan.chain);
    if t.chain != chain {
        return Err(invalid("sealed hash chain mismatch"));
    }
    if meta.first != t.first
        || meta.last != t.last
        || meta.count != t.n
        || meta.bytes != file_len
        || meta.chain != t.chain
    {
        return Err(invalid(format!(
            "manifest disagrees with sealed segment {}",
            meta.name
        )));
    }
    Ok((scan.events, scan.chain))
}

fn read_active(path: &Path, expected_prev: [u8; 32]) -> std::io::Result<Vec<Event>> {
    if !path.exists() {
        return Err(invalid("missing active segment"));
    }
    let bytes = fs::read(path)?;
    let prev = parse_header(&bytes)?;
    if prev != expected_prev {
        return Err(invalid("chain link mismatch"));
    }
    let scan = scan_segment(&bytes, false)?;
    Ok(scan.events)
}

/// Load events from `manifest.json` + named segments. Never reads jsonl/WAL.
///
/// Check order: missing/invalid names, sealed+active sequence contiguity,
/// then per sealed file: chain link, scan (CRC/trailer/truncated), count, bytes,
/// hash chain, manifest fields; then the active header chain-link.
pub(crate) fn load_events(dir: &Path) -> std::io::Result<Vec<Event>> {
    let manifest = read_manifest(dir)?;
    let mut seqs = Vec::with_capacity(manifest.sealed.len());
    for entry in &manifest.sealed {
        seqs.push(parse_seg_seq(&entry.name)?);
    }
    let active_seq = parse_seg_seq(&manifest.active)?;
    if let Some(&first) = seqs.first() {
        for (i, &seq) in seqs.iter().enumerate() {
            if seq != first.saturating_add(i as u32) {
                return Err(invalid("sealed sequence not contiguous"));
            }
        }
        if active_seq != first.saturating_add(seqs.len() as u32) {
            return Err(invalid("sealed sequence not contiguous"));
        }
    }
    let mut prev_tip = [0u8; 32];
    let mut all = Vec::new();
    for entry in &manifest.sealed {
        let (evs, tip) = verify_sealed(&dir.join(&entry.name), entry, prev_tip)?;
        prev_tip = tip;
        all.extend(evs);
    }
    let active_evs = read_active(&dir.join(&manifest.active), prev_tip)?;
    all.extend(active_evs);
    Ok(all)
}

pub(crate) struct ActiveTip {
    pub events: usize,
    pub chain: [u8; 32],
    pub first_id: Option<EventId>,
    pub file_len: u64,
}

pub(crate) fn inspect_active(path: &Path) -> std::io::Result<ActiveTip> {
    let bytes = fs::read(path)?;
    let scan = scan_segment(&bytes, false)?;
    Ok(ActiveTip {
        events: scan.events.len(),
        chain: scan.chain,
        first_id: scan.events.first().map(|e| e.id),
        file_len: bytes.len() as u64,
    })
}

fn split_events(events: &[Event]) -> std::io::Result<Vec<&[Event]>> {
    if events.is_empty() {
        return Ok(vec![events]);
    }
    let max_bytes = effective_max_bytes();
    let max_events = effective_max_events();
    let mut chunks = Vec::new();
    let mut start = 0usize;
    let mut count = 0usize;
    let mut bytes = HEADER_LEN;
    for (i, e) in events.iter().enumerate() {
        let flen = wal::frame_bytes(e)?.len() as u64;
        let overflow = count > 0 && (count >= max_events || bytes.saturating_add(flen) > max_bytes);
        if overflow {
            chunks.push(&events[start..i]);
            start = i;
            count = 0;
            bytes = HEADER_LEN;
        }
        bytes = bytes.saturating_add(flen);
        count += 1;
    }
    chunks.push(&events[start..]);
    Ok(chunks)
}

/// Rewrite `dir` as sealed segments + one active + `manifest.json`. Does not touch jsonl/WAL.
pub(crate) fn persist_log(dir: &Path, events: &[Event]) -> std::io::Result<()> {
    fs::create_dir_all(dir)?;
    let chunks = split_events(events)?;
    let mut seq = 1u32;
    let mut sealed = Vec::new();
    let last_i = chunks.len().saturating_sub(1);
    let mut active = segment_name(seq);
    let mut prev = [0u8; 32];
    for (i, chunk) in chunks.iter().enumerate() {
        let name = segment_name(seq);
        seq = seq.saturating_add(1);
        let is_sealed = i != last_i;
        let (meta, tip) = write_segment_file(&dir.join(&name), chunk, is_sealed, prev)?;
        prev = tip;
        if let Some(meta) = meta {
            sealed.push(meta);
        }
        if i == last_i {
            active = name;
        }
    }
    write_manifest(
        dir,
        &Manifest {
            v: 1,
            sealed,
            active,
        },
    )
}
