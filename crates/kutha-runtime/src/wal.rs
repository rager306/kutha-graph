use crc32fast::Hasher;
use kutha_common::Event;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

const MAGIC: &[u8; 8] = b"KUTHAWAL";
const VERSION: u32 = 1;

/// Framed durable log (Rocks WAL *cousin*): length + crc + payload.
/// Truncated/corrupt last record is dropped (kTolerateCorruptedTail analog). Not SoT algebra.
pub fn append_events(path: &Path, events: &[Event]) -> std::io::Result<()> {
    let mut f = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(path)?;
    f.write_all(MAGIC)?;
    f.write_all(&VERSION.to_le_bytes())?;
    for e in events {
        let payload = serde_json::to_vec(e)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        let mut hasher = Hasher::new();
        hasher.update(&payload);
        let crc = hasher.finalize();
        let len = payload.len() as u32;
        f.write_all(&len.to_le_bytes())?;
        f.write_all(&crc.to_le_bytes())?;
        f.write_all(&payload)?;
    }
    f.sync_all()?;
    Ok(())
}

pub fn recover_events(path: &Path) -> std::io::Result<Vec<Event>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut f = File::open(path)?;
    let mut magic = [0u8; 8];
    if f.read_exact(&mut magic).is_err() {
        return Ok(Vec::new());
    }
    if &magic != MAGIC {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "bad WAL magic",
        ));
    }
    let mut ver = [0u8; 4];
    f.read_exact(&mut ver)?;
    if u32::from_le_bytes(ver) != VERSION {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "unsupported WAL version",
        ));
    }
    let mut out = Vec::new();
    loop {
        let mut hdr = [0u8; 8];
        match f.read_exact(&mut hdr) {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
            Err(e) => return Err(e),
        }
        let len = u32::from_le_bytes(hdr[0..4].try_into().unwrap()) as usize;
        let crc_stored = u32::from_le_bytes(hdr[4..8].try_into().unwrap());
        let mut payload = vec![0u8; len];
        match f.read_exact(&mut payload) {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
            Err(e) => return Err(e),
        }
        let mut hasher = Hasher::new();
        hasher.update(&payload);
        if hasher.finalize() != crc_stored {
            break;
        }
        match serde_json::from_slice::<Event>(&payload) {
            Ok(e) => out.push(e),
            Err(_) => break,
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use kutha_common::{Event, Op};

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
        let e0 = Event::new(Op::Retract { fact_seq: 0 }, 0);
        let e1 = Event::new(Op::Retract { fact_seq: 1 }, 1);
        append_events(&path, &[e0.clone(), e1.clone()]).unwrap();
        let mut bytes = std::fs::read(&path).unwrap();
        bytes.truncate(bytes.len().saturating_sub(3));
        std::fs::write(&path, &bytes).unwrap();
        let recovered = recover_events(&path).unwrap();
        assert_eq!(recovered.len(), 1);
        assert_eq!(recovered[0].id, e0.id);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
