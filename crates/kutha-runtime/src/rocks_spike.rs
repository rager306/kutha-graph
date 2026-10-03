//! Optional EventId → `events.jsonl` byte-offset index (ROCK-00).
//! Not wired into `Runtime`, persist, fold, or hydrate.

use kutha_common::{Event, EventId};
use std::io;
use std::path::Path;

/// Scan `events.jsonl` and store each `EventId` (16 bytes) → `u64` big-endian offset.
pub fn build_index(_log_path: impl AsRef<Path>, _rocks_dir: impl AsRef<Path>) -> io::Result<usize> {
    unimplemented!("ROCK-00 RED: EventId-offset index")
}

/// Look up a persisted offset. Missing key is `Ok(None)`.
pub fn lookup(_rocks_dir: impl AsRef<Path>, _id: EventId) -> io::Result<Option<u64>> {
    unimplemented!("ROCK-00 RED: EventId-offset lookup")
}

/// Read one JSONL event starting at `offset`.
pub fn read_event_at(_log_path: impl AsRef<Path>, _offset: u64) -> io::Result<Event> {
    unimplemented!("ROCK-00 RED: read event at offset")
}
