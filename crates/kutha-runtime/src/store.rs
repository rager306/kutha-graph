use crate::quantum::Runtime;
use crate::snapshot::Snapshot;
use crate::wal;
use kutha_common::Event;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

/// File-backed semantic log + snapshot lease. WAL file is durability cousin, not Rocks-as-SoT.
pub fn persist(runtime: &Runtime, dir: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dir)?;
    wal::append_events(&dir.join("events.wal"), runtime.log().as_slice())?;
    let mut events = File::create(dir.join("events.jsonl"))?;
    for e in runtime.log().iter() {
        serde_json::to_writer(&mut events, e).map_err(json_err)?;
        events.write_all(b"\n")?;
    }
    let snap = runtime.snapshot();
    let mut sf = File::create(dir.join("snapshot.json"))?;
    serde_json::to_writer(&mut sf, &snap).map_err(json_err)?;
    Ok(())
}

pub fn open(dir: &Path) -> std::io::Result<Runtime> {
    let wal_path = dir.join("events.wal");
    let events_path = dir.join("events.jsonl");
    let snap_path = dir.join("snapshot.json");
    let events: Vec<Event> = if wal_path.exists() {
        wal::recover_events(&wal_path)?
    } else if events_path.exists() {
        let f = File::open(&events_path)?;
        let mut v = Vec::new();
        for line in BufReader::new(f).lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }
            v.push(serde_json::from_str(&line).map_err(json_err)?);
        }
        v
    } else {
        Vec::new()
    };
    if snap_path.exists() {
        let snap: Snapshot = serde_json::from_reader(File::open(&snap_path)?).map_err(json_err)?;
        Ok(Runtime::from_snapshot(snap, events))
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "missing snapshot.json (lease required to restore intern map)",
        ))
    }
}

fn json_err(e: serde_json::Error) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::InvalidData, e)
}
