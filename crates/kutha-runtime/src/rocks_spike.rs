//! Optional EventId → `events.jsonl` byte-offset index (ROCK-00).
//! Not wired into `Runtime`, persist, fold, or hydrate.

use kutha_common::{Event, EventId};
use rocksdb::{DB, Options};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Error, ErrorKind, Seek, SeekFrom};
use std::path::Path;

fn rocks_err(err: rocksdb::Error) -> Error {
    Error::new(ErrorKind::Other, err.to_string())
}

fn invalid(message: String) -> Error {
    Error::new(ErrorKind::InvalidData, message)
}

/// Scan `events.jsonl` and store each `EventId` (16 bytes) → `u64` big-endian offset.
pub fn build_index(log_path: impl AsRef<Path>, rocks_dir: impl AsRef<Path>) -> io::Result<usize> {
    let log_path = log_path.as_ref();
    let rocks_dir = rocks_dir.as_ref();
    if rocks_dir.exists() {
        DB::destroy(&Options::default(), rocks_dir).map_err(rocks_err)?;
    }
    let mut opts = Options::default();
    opts.create_if_missing(true);
    let db = DB::open(&opts, rocks_dir).map_err(rocks_err)?;
    let file = File::open(log_path)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut count = 0usize;
    loop {
        let offset = reader.stream_position()?;
        line.clear();
        let n = reader.read_line(&mut line)?;
        if n == 0 {
            break;
        }
        if line.trim().is_empty() {
            continue;
        }
        let event: Event = serde_json::from_str(line.trim_end())
            .map_err(|err| invalid(format!("malformed events.jsonl at byte {offset}: {err}")))?;
        db.put(event.id.as_bytes(), offset.to_be_bytes())
            .map_err(rocks_err)?;
        count += 1;
    }
    db.flush().map_err(rocks_err)?;
    Ok(count)
}

/// Look up a persisted offset. Missing key is `Ok(None)`.
pub fn lookup(rocks_dir: impl AsRef<Path>, id: EventId) -> io::Result<Option<u64>> {
    let db = DB::open_for_read_only(&Options::default(), rocks_dir.as_ref(), false)
        .map_err(rocks_err)?;
    match db.get(id.as_bytes()).map_err(rocks_err)? {
        None => Ok(None),
        Some(bytes) => {
            let arr: [u8; 8] = bytes.as_slice().try_into().map_err(|_| {
                invalid("rocks value is not an 8-byte big-endian offset".to_string())
            })?;
            Ok(Some(u64::from_be_bytes(arr)))
        }
    }
}

/// Read one JSONL event starting at `offset`.
pub fn read_event_at(log_path: impl AsRef<Path>, offset: u64) -> io::Result<Event> {
    let mut file = File::open(log_path.as_ref())?;
    file.seek(SeekFrom::Start(offset))?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let n = reader.read_line(&mut line)?;
    if n == 0 {
        return Err(Error::new(
            ErrorKind::UnexpectedEof,
            format!("no event at offset {offset}"),
        ));
    }
    serde_json::from_str(line.trim_end())
        .map_err(|err| invalid(format!("malformed event at byte {offset}: {err}")))
}
