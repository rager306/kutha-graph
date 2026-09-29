use crate::quantum::{cascade_limit, PersistedQuantumOutcome, Runtime, RuntimeError};
use crate::snapshot::Snapshot;
use crate::wal;
use kutha_common::{Event, Op};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

const TERMS_REL: &str = "terms.jsonl";
/// Authoritative quantum outcome sidecar (not a droppable lease — D-O1 / ADR-010).
pub const OUTCOMES_REL: &str = "quantum_outcomes.jsonl";

/// File-backed semantic log + snapshot lease. WAL file is durability cousin, not Rocks-as-SoT.
/// `terms.jsonl` is a derived intern picture. Durable term meanings ride `Op::Define` (persist prefix + live intern, M010 S02–S03).
pub fn persist(runtime: &Runtime, dir: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dir)?;
    let events = encoded_log(runtime);
    wal::append_events(&dir.join("events.wal"), &events)?;
    let mut events_file = File::create(dir.join("events.jsonl"))?;
    for e in &events {
        serde_json::to_writer(&mut events_file, e).map_err(json_err)?;
        events_file.write_all(b"\n")?;
    }
    write_terms(dir, runtime.dictionary().strings())?;
    let snap = runtime.snapshot();
    let mut sf = File::create(dir.join("snapshot.json"))?;
    serde_json::to_writer(&mut sf, &snap).map_err(json_err)?;
    // Outcomes after events so mid-crash cannot orphan a Full row (RESEARCH pitfall 3).
    write_outcomes(dir, runtime.outcome_records())?;
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
    let outcomes = load_outcomes(&dir.join(OUTCOMES_REL))?;
    if snap_path.exists() {
        let snap: Snapshot = serde_json::from_reader(File::open(&snap_path)?).map_err(json_err)?;
        let graph: Vec<Event> = events
            .into_iter()
            .filter(|e| !matches!(e.op, Op::Define { .. }))
            .collect();
        let mut rt = Runtime::from_snapshot(snap, graph);
        rt.attach_outcomes(outcomes);
        return Ok(rt);
    }
    let defined = defined_names(&events);
    if !defined.is_empty() {
        let mut rt =
            Runtime::from_dict_and_events(defined, events, cascade_limit()).map_err(runtime_err)?;
        rt.attach_outcomes(outcomes);
        return Ok(rt);
    }
    if terms_path.exists() {
        let strings = read_terms(&terms_path)?;
        let mut rt =
            Runtime::from_dict_and_events(strings, events, cascade_limit()).map_err(runtime_err)?;
        rt.attach_outcomes(outcomes);
        return Ok(rt);
    }
    if events.is_empty() {
        let mut rt = Runtime::default();
        rt.attach_outcomes(outcomes);
        return Ok(rt);
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        "missing snapshot.json, terms.jsonl, and Op::Define events (cannot restore intern meanings)",
    ))
}

fn encoded_log(runtime: &Runtime) -> Vec<Event> {
    let mut out: Vec<Event> = runtime
        .dictionary()
        .strings()
        .iter()
        .map(|name| Event::new(Op::Define { name: name.clone() }, 0))
        .collect();
    for e in runtime.log().iter() {
        if !matches!(e.op, Op::Define { .. }) {
            out.push(e.clone());
        }
    }
    out
}

fn defined_names(events: &[Event]) -> Vec<String> {
    events
        .iter()
        .filter_map(|e| match &e.op {
            Op::Define { name } => Some(name.clone()),
            _ => None,
        })
        .collect()
}

fn write_terms(dir: &Path, strings: &[String]) -> std::io::Result<()> {
    let mut f = File::create(dir.join(TERMS_REL))?;
    for s in strings {
        serde_json::to_writer(&mut f, s).map_err(json_err)?;
        f.write_all(b"\n")?;
    }
    Ok(())
}

fn write_outcomes(dir: &Path, rows: &[PersistedQuantumOutcome]) -> std::io::Result<()> {
    let mut f = File::create(dir.join(OUTCOMES_REL))?;
    for row in rows {
        serde_json::to_writer(&mut f, row).map_err(json_err)?;
        f.write_all(b"\n")?;
    }
    Ok(())
}

fn load_outcomes(path: &Path) -> std::io::Result<Vec<PersistedQuantumOutcome>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
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
