//! Droppable RocksDB locator lease over the segmented log (ROCK-01).
//! Not wired into `Runtime`, `store::open`, `Persister`, fold, or hydrate.
//!
//! Keys: `e:` + EventId, `d:` + delivery_key UTF-8, `m:v`, `m:tip`.
//! Values are fixed locators / ids / meta — never event payloads.

use kutha_common::{Event, EventId};
use std::io;
use std::path::Path;

/// On-disk frame locator: segment seq, byte offset, frame length (big-endian in Rocks).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Locator {
    pub seq: u32,
    pub offset: u64,
    pub frame_len: u32,
}

/// Open RocksDB handle over a store directory. Dropping it does not drop the log.
pub struct SegmentIndex {
    _private: (),
}

impl SegmentIndex {
    /// Index the committed prefix of a segmented store. Rejects legacy jsonl dirs.
    pub fn build(_store_dir: impl AsRef<Path>, _rocks_dir: impl AsRef<Path>) -> io::Result<Self> {
        unimplemented!("ROCK-01 RED: segmented Rocks locator index")
    }

    /// Open if `m:tip` matches the log tip; otherwise destroy `rocks_dir` and rebuild.
    pub fn open_or_rebuild(
        store_dir: impl AsRef<Path>,
        rocks_dir: impl AsRef<Path>,
    ) -> io::Result<Self> {
        Self::build(store_dir, rocks_dir)
    }

    /// Index only events after the recorded tip when the log only grew.
    pub fn catch_up(&self) -> io::Result<usize> {
        unimplemented!("ROCK-01 RED: segmented Rocks locator index")
    }

    pub fn locate(&self, _id: EventId) -> io::Result<Option<Locator>> {
        unimplemented!("ROCK-01 RED: segmented Rocks locator index")
    }

    pub fn get_event(&self, _id: EventId) -> io::Result<Option<Event>> {
        unimplemented!("ROCK-01 RED: segmented Rocks locator index")
    }

    pub fn by_delivery_key(&self, _key: &str) -> io::Result<Option<EventId>> {
        unimplemented!("ROCK-01 RED: segmented Rocks locator index")
    }
}
