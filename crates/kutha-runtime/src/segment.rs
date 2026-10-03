//! Sealed segments + manifest (M002 S06 / SEG-01). Physical encoding of the event log.
//!
//! Magic `KUTHASEG` v1; event frames reuse [`crate::wal::frame_bytes`]. Persist still
//! writes the legacy jsonl+WAL layout in plan 54-01.

use crate::store::{json_err, load_events_d04, replace_via_tmp};
use crate::wal;
use crc32fast::Hasher;
use kutha_common::{Event, EventId};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

pub(crate) const MAGIC: &[u8; 8] = b"KUTHASEG";
pub(crate) const VERSION: u32 = 1;
pub(crate) const HEADER_LEN: u64 = 12;
/// Manifest file name (`manifest.json`).
#[doc(hidden)]
pub const MANIFEST_REL: &str = "manifest.json";
const MANIFEST_TMP: &str = "manifest.json.tmp";

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

fn hex32(digest: [u8; 32]) -> String {
    let mut out = String::with_capacity(64);
    for b in digest {
        out.push_str(&format!("{b:02x}"));
    }
    out
}

fn chain_absorb(h: [u8; 32], frame: &[u8]) -> [u8; 32] {
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

fn write_header(f: &mut impl Write) -> std::io::Result<()> {
    f.write_all(MAGIC)?;
    f.write_all(&VERSION.to_le_bytes())?;
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

fn seal_frame(
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

fn write_segment_file(
    path: &Path,
    events: &[Event],
    sealed: bool,
) -> std::io::Result<Option<SealedMeta>> {
    let mut frames = Vec::new();
    let mut h = [0u8; 32];
    for e in events {
        let frame = wal::frame_bytes(e)?;
        h = chain_absorb(h, &frame);
        frames.extend_from_slice(&frame);
    }
    let chain = hex32(h);
    let body_len = HEADER_LEN + frames.len() as u64;
    let trailer = if sealed {
        if events.is_empty() {
            return Err(invalid("sealed segment must contain at least one event"));
        }
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
    {
        let mut f = File::create(path)?;
        write_header(&mut f)?;
        f.write_all(&frames)?;
        if let Some(t) = &trailer {
            f.write_all(t)?;
        }
        f.sync_all()?;
    }
    if sealed {
        Ok(Some(SealedMeta {
            name: path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("seg.log")
                .to_string(),
            first: events[0].id,
            last: events[events.len() - 1].id,
            count: events.len() as u64,
            bytes,
            chain,
        }))
    } else {
        Ok(None)
    }
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
    for &n in sealed_counts {
        let name = segment_name(seq);
        seq = seq.saturating_add(1);
        let chunk = &events[off..off + n];
        off += n;
        let meta = write_segment_file(&dir.join(&name), chunk, true)?
            .ok_or_else(|| invalid("sealed write returned no meta"))?;
        sealed.push(meta);
    }
    let active = segment_name(seq);
    write_segment_file(&dir.join(&active), &events[off..off + active_count], false)?;
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
    let events = scan_events_no_trailer(path)?;
    write_segment_file(path, &events, true)?;
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

fn scan_events_no_trailer(path: &Path) -> std::io::Result<Vec<Event>> {
    let bytes = fs::read(path)?;
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
    let mut off = HEADER_LEN as usize;
    let mut out = Vec::new();
    while off + 8 <= bytes.len() {
        let len = u32_le(&bytes[off..off + 4])? as usize;
        let end = off
            .checked_add(8)
            .and_then(|x| x.checked_add(len))
            .ok_or_else(|| invalid("segment frame overflow"))?;
        if end > bytes.len() {
            return Err(invalid("truncated segment frame"));
        }
        let payload = &bytes[off + 8..end];
        let mut hasher = Hasher::new();
        hasher.update(payload);
        let crc_stored = u32_le(&bytes[off + 4..off + 8])?;
        if hasher.finalize() != crc_stored {
            return Err(invalid("segment frame CRC mismatch"));
        }
        if trailer_payload(payload).is_some() {
            return Err(invalid("unexpected seal trailer while rewriting"));
        }
        out.push(serde_json::from_slice(payload).map_err(json_err)?);
        off = end;
    }
    if off != bytes.len() {
        return Err(invalid("trailing bytes in unsealed segment"));
    }
    Ok(out)
}

fn trailer_payload(payload: &[u8]) -> Option<SealTrailer> {
    let t: SealTrailer = serde_json::from_slice(payload).ok()?;
    (t.k == "seal").then_some(t)
}

/// RED stub: a present manifest does not change the load. Jsonl/WAL remain SoT.
pub(crate) fn load_events(dir: &Path) -> std::io::Result<Vec<Event>> {
    load_events_d04(dir)
}
