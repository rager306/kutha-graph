use crate::quantum::{Runtime, RuntimeError};
use crate::snapshot::Snapshot;
use crate::wal;
use kutha_common::Event;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

const TERMS_REL: &str = "terms.jsonl";

/// File-backed semantic log + snapshot lease. WAL file is durability cousin, not Rocks-as-SoT.
/// `terms.jsonl` retains authoritative term strings (ADR-011); `snapshot.json` remains a droppable lease.
pub fn persist(runtime: &Runtime, dir: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dir)?;
    wal::append_events(&dir.join("events.wal"), runtime.log().as_slice())?;
    let mut events = File::create(dir.join("events.jsonl"))?;
    for e in runtime.log().iter() {
        serde_json::to_writer(&mut events, e).map_err(json_err)?;
        events.write_all(b"\n")?;
    }
    write_terms(dir, runtime.dictionary().strings())?;
    let snap = runtime.snapshot();
    let mut sf = File::create(dir.join("snapshot.json"))?;
    serde_json::to_writer(&mut sf, &snap).map_err(json_err)?;
    Ok(())
}

pub fn open(dir: &Path) -> std::io::Result<Runtime> {
    let wal_path = dir.join("events.wal");
    let events_path = dir.join("events.jsonl");
    let snap_path = dir.join("snapshot.json");
    let terms_path = dir.join(TERMS_REL);
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
        return Ok(Runtime::from_snapshot(snap, events));
    }
    if terms_path.exists() {
        let strings = read_terms(&terms_path)?;
        return Runtime::from_dict_and_events(strings, events, Runtime::default().max_cascade)
            .map_err(runtime_err);
    }
    if events.is_empty() {
        return Ok(Runtime::default());
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        "missing snapshot.json and terms.jsonl (cannot restore intern meanings)",
    ))
}

fn write_terms(dir: &Path, strings: &[String]) -> std::io::Result<()> {
    let mut f = File::create(dir.join(TERMS_REL))?;
    for s in strings {
        serde_json::to_writer(&mut f, s).map_err(json_err)?;
        f.write_all(b"\n")?;
    }
    Ok(())
}

fn read_terms(path: &Path) -> std::io::Result<Vec<String>> {
    let f = File::open(path)?;
    let mut out = Vec::new();
    for line in BufReader::new(f).lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        out.push(serde_json::from_str(&line).map_err(json_err)?);
    }
    Ok(out)
}

fn json_err(e: serde_json::Error) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::InvalidData, e)
}

fn runtime_err(e: RuntimeError) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string())
}
