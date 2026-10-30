use crc32fast::Hasher;
use kutha_common::{Event, EventId};
use std::cell::Cell;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

const MAGIC: &[u8; 8] = b"KUTHAWAL";
const VERSION: u32 = 1;

thread_local! {
    static LOG_FILE_SYNC_COUNT: Cell<usize> = const { Cell::new(0) };
}

pub(crate) fn note_log_file_sync() {
    LOG_FILE_SYNC_COUNT.with(|c| c.set(c.get() + 1));
}

pub(crate) fn log_file_sync_count() -> usize {
    LOG_FILE_SYNC_COUNT.with(|c| c.get())
}

pub(crate) fn reset_log_file_sync_count() {
    LOG_FILE_SYNC_COUNT.with(|c| c.set(0));
}

pub(crate) fn frame_bytes(event: &Event) -> std::io::Result<Vec<u8>> {
    let payload = serde_json::to_vec(event)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    let mut hasher = Hasher::new();
    hasher.update(&payload);
    let crc = hasher.finalize();
    let len = payload.len() as u32;
    let mut out = Vec::with_capacity(8 + payload.len());
    out.extend_from_slice(&len.to_le_bytes());
    out.extend_from_slice(&crc.to_le_bytes());
    out.extend_from_slice(&payload);
    Ok(out)
}

pub(crate) fn write_frames(f: &mut impl Write, events: &[Event]) -> std::io::Result<()> {
    for e in events {
        f.write_all(&frame_bytes(e)?)?;
    }
    Ok(())
}

/// Last WAL frame ending at `file_len` must be exactly `frame_len` bytes (D-04).
pub(crate) fn last_event_id_at_end(
    path: &Path,
    file_len: u64,
    frame_len: u64,
) -> std::io::Result<EventId> {
    if frame_len < 8 || file_len < frame_len {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "WAL tail shorter than last frame",
        ));
    }
    let mut f = File::open(path)?;
    f.seek(SeekFrom::Start(file_len - frame_len))?;
    let mut hdr = [0u8; 8];
    f.read_exact(&mut hdr)?;
    let mut len_bytes = [0u8; 4];
    len_bytes.copy_from_slice(&hdr[0..4]);
    let mut crc_bytes = [0u8; 4];
    crc_bytes.copy_from_slice(&hdr[4..8]);
    let payload_len = u32::from_le_bytes(len_bytes) as u64;
    let crc_stored = u32::from_le_bytes(crc_bytes);
    if payload_len.saturating_add(8) != frame_len {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "WAL last frame length mismatch",
        ));
    }
    let mut payload = vec![0u8; payload_len as usize];
    f.read_exact(&mut payload)?;
    let mut hasher = Hasher::new();
    hasher.update(&payload);
    if hasher.finalize() != crc_stored {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "WAL last frame CRC mismatch",
        ));
    }
    let event: Event = serde_json::from_slice(&payload)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    Ok(event.id)
}

/// Framed durable log (Rocks WAL *cousin*): length + crc + payload.
/// Full persist `sync_all`s here; incremental append `sync_data`s after frames. Not SoT algebra.
pub(crate) fn write_wal(f: &mut File, events: &[Event]) -> std::io::Result<()> {
    f.write_all(MAGIC)?;
    f.write_all(&VERSION.to_le_bytes())?;
    write_frames(f, events)?;
    f.sync_all()?;
    note_log_file_sync();
    Ok(())
}

#[allow(dead_code)] // used by wal.rs unit tests; persist uses write_wal + rename
pub fn append_events(path: &Path, events: &[Event]) -> std::io::Result<()> {
    let mut f = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(path)?;
    write_wal(&mut f, events)
}

fn open_wal_after_header(path: &Path) -> std::io::Result<File> {
    let mut f = File::open(path)?;
    let mut header = [0u8; 12];
    match f.read_exact(&mut header) {
        Ok(()) => {}
        Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "truncated WAL header",
            ));
        }
        Err(e) => return Err(e),
    }
    if header[0..8] != MAGIC[..] {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "bad WAL magic",
        ));
    }
    let mut ver = [0u8; 4];
    ver.copy_from_slice(&header[8..12]);
    if u32::from_le_bytes(ver) != VERSION {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "unsupported WAL version",
        ));
    }
    Ok(f)
}

/// Recover framed events. A file shorter than magic+version is `InvalidData`
/// ("truncated WAL header"). Non-`UnexpectedEof` read errors propagate.
/// Truncated/CRC-bad last record is dropped (kTolerateCorruptedTail analog).
/// A CRC-good frame whose payload is valid JSON but not a modern [`Event`]
/// is `InvalidData` (Op-wire fail-open — do not silently drop legacy meta-ops).
pub fn recover_events(path: &Path) -> std::io::Result<Vec<Event>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut f = open_wal_after_header(path)?;
    let mut out = Vec::new();
    loop {
        let Some(payload) = read_one_frame_payload(&mut f)? else {
            break;
        };
        match serde_json::from_slice::<Event>(&payload) {
            Ok(e) => out.push(e),
            Err(e) => {
                if serde_json::from_slice::<serde_json::Value>(&payload).is_ok() {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("WAL frame is not a modern Event: {e}"),
                    ));
                }
                break;
            }
        }
    }
    Ok(out)
}

/// Recover WAL frame payloads as JSON values (M068 Op-wire).
///
/// Unlike [`recover_events`], a payload that is valid JSON but not a modern
/// [`Event`] is kept so callers can remap legacy meta-ops. Truncated/CRC-bad
/// tails still stop the scan. Returns `(values, dropped_tail)` where
/// `dropped_tail` is 1 when unread bytes remain after the stop (CE #13).
pub(crate) fn recover_json_values(
    path: &Path,
) -> std::io::Result<(Vec<serde_json::Value>, usize)> {
    if !path.exists() {
        return Ok((Vec::new(), 0));
    }
    let mut f = open_wal_after_header(path)?;
    let mut out = Vec::new();
    loop {
        let Some(payload) = read_one_frame_payload(&mut f)? else {
            break;
        };
        match serde_json::from_slice::<serde_json::Value>(&payload) {
            Ok(v) => out.push(v),
            Err(_) => break,
        }
    }
    let meta_len = f.metadata()?.len();
    let pos = f.stream_position()?;
    let dropped_tail = usize::from(pos < meta_len);
    Ok((out, dropped_tail))
}

fn read_one_frame_payload(f: &mut File) -> std::io::Result<Option<Vec<u8>>> {
    let mut hdr = [0u8; 8];
    match f.read_exact(&mut hdr) {
        Ok(()) => {}
        Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(None),
        Err(e) => return Err(e),
    }
    let len = u32::from_le_bytes(
        hdr[0..4]
            .try_into()
            .expect("frame header has a 4-byte length prefix"),
    ) as usize;
    let crc_stored = u32::from_le_bytes(
        hdr[4..8]
            .try_into()
            .expect("frame header has a 4-byte checksum"),
    );
    let mut payload = vec![0u8; len];
    match f.read_exact(&mut payload) {
        Ok(()) => {}
        Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(None),
        Err(e) => return Err(e),
    }
    let mut hasher = Hasher::new();
    hasher.update(&payload);
    if hasher.finalize() != crc_stored {
        return Ok(None);
    }
    Ok(Some(payload))
}

/// Test/helper: write a WAL whose frames are raw JSON payloads (may be legacy Ops).
#[doc(hidden)]
pub fn write_json_payload_wal(path: &Path, payloads: &[&[u8]]) -> std::io::Result<()> {
    let mut f = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(path)?;
    f.write_all(MAGIC)?;
    f.write_all(&VERSION.to_le_bytes())?;
    for payload in payloads {
        let mut hasher = Hasher::new();
        hasher.update(payload);
        let crc = hasher.finalize();
        let len = payload.len() as u32;
        f.write_all(&len.to_le_bytes())?;
        f.write_all(&crc.to_le_bytes())?;
        f.write_all(payload)?;
    }
    f.sync_all()?;
    note_log_file_sync();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use kutha_common::{Event, EventId, Op};

    #[test]
    fn recover_drops_truncated_tail() {
        let dir = std::env::temp_dir().join(format!("kutha-wal-{}", {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        }));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("events.wal");
        let e0 = Event::new(
            Op::Retract {
                event_id: EventId::nil(),
            },
            0,
        );
        let e1 = Event::new(
            Op::Retract {
                event_id: EventId::from_u128(1),
            },
            1,
        );
        append_events(&path, &[e0.clone(), e1.clone()]).unwrap();
        let mut bytes = std::fs::read(&path).unwrap();
        bytes.truncate(bytes.len().saturating_sub(3));
        std::fs::write(&path, &bytes).unwrap();
        let recovered = recover_events(&path).unwrap();
        assert_eq!(recovered.len(), 1);
        assert_eq!(recovered[0].id, e0.id);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn recover_rejects_legacy_meta_op_frame() {
        let dir = std::env::temp_dir().join(format!("kutha-wal-legacy-{}", {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        }));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("events.wal");
        let legacy = br#"{"id":"00000000-0000-0000-0000-000000000000","op":{"AllowRelation":{"name":"x","valid_from":0,"valid_to":null}},"ingested_at":1,"object_ids":[]}"#;
        write_json_payload_wal(&path, &[legacy]).unwrap();
        let err = recover_events(&path).expect_err("legacy meta-op must fail-open");
        assert_eq!(err.kind(), std::io::ErrorKind::InvalidData);
        assert!(
            err.to_string().contains("not a modern Event"),
            "{err}"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }
}
