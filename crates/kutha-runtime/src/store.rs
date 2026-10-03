use crate::quantum::{
    cascade_limit, Justification, PersistedQuantumOutcome, Runtime, RuntimeError,
};
use crate::snapshot::Snapshot;
use crate::wal;
use kutha_common::{Event, EventId, Op};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

const TERMS_REL: &str = "terms.jsonl";
/// Outcomes sidecar lease. SoT is `Op::QuantumOutcome` on the event log (LOG-01 / D-02).
pub const OUTCOMES_REL: &str = "quantum_outcomes.jsonl";
/// Justifications sidecar lease. SoT is `Op::JustificationCite` on the event log (LOG-02 / D-02).
pub const JUSTIFICATIONS_REL: &str = "justifications.jsonl";

/// File-backed semantic log + snapshot lease. WAL file is durability cousin, not Rocks-as-SoT.
/// `terms.jsonl` is a derived intern picture. Durable term meanings ride `Op::Define` (persist prefix + live intern, M010 S02–S03).
pub fn persist(runtime: &Runtime, dir: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dir)?;
    let events = encoded_log(runtime);
    replace_events_wal(dir, &events, true)?;
    replace_events_jsonl(dir, &events)?;
    write_terms(dir, runtime.dictionary().strings())?;
    replace_snapshot(dir, &runtime.snapshot(), true)?;
    // Outcomes sidecar is a lease of log-native rows (D-05); never persist Typed-only buffers.
    write_outcomes(dir, &crate::hydrate::outcomes_from_events(runtime.log().iter()))?;
    // Justifications last so a crash cannot leave cites without matching events;
    // missing file on open is empty vec (cannot admit).
    write_justifications(dir, &runtime.justification_records())?;
    Ok(())
}

const EVENTS_JSONL: &str = "events.jsonl";
const EVENTS_JSONL_TMP: &str = "events.jsonl.tmp";
const EVENTS_WAL: &str = "events.wal";
const EVENTS_WAL_TMP: &str = "events.wal.tmp";
const SNAPSHOT_JSON: &str = "snapshot.json";
const SNAPSHOT_JSON_TMP: &str = "snapshot.json.tmp";

fn abort_before_rename() -> std::io::Error {
    std::io::Error::new(
        std::io::ErrorKind::Other,
        "injected fault: abort before rename",
    )
}

fn replace_via_tmp(
    dest: &Path,
    tmp: &Path,
    commit: bool,
    write: impl FnOnce(&mut File) -> std::io::Result<()>,
) -> std::io::Result<()> {
    {
        let mut f = File::create(tmp)?;
        write(&mut f)?;
        f.sync_all()?;
    }
    if !commit {
        return Err(abort_before_rename());
    }
    fs::rename(tmp, dest)?;
    Ok(())
}

/// Replace `events.jsonl` by writing a same-directory temp file, syncing, then rename.
/// `persist` returns Ok only after this rename (DUR-02).
fn replace_events_jsonl(dir: &Path, events: &[Event]) -> std::io::Result<()> {
    write_events_jsonl_temp(dir, events, true)
}

/// Test seam (DUR-02): write the temp sibling then return Err without renaming dest.
pub fn abort_events_jsonl_replace_before_rename(
    dir: &Path,
    events: &[Event],
) -> std::io::Result<()> {
    write_events_jsonl_temp(dir, events, false)
}

/// Test seam (REPL-02): WAL temp write, dest left byte-identical.
pub fn abort_wal_replace_before_rename(dir: &Path, runtime: &Runtime) -> std::io::Result<()> {
    replace_events_wal(dir, &encoded_log(runtime), false)
}

/// Test seam (REPL-02): snapshot temp write, dest left byte-identical.
pub fn abort_snapshot_replace_before_rename(dir: &Path, runtime: &Runtime) -> std::io::Result<()> {
    replace_snapshot(dir, &runtime.snapshot(), false)
}

fn write_events_jsonl_temp(dir: &Path, events: &[Event], commit: bool) -> std::io::Result<()> {
    replace_via_tmp(
        &dir.join(EVENTS_JSONL),
        &dir.join(EVENTS_JSONL_TMP),
        commit,
        |f| {
            for e in events {
                serde_json::to_writer(&mut *f, e).map_err(json_err)?;
                f.write_all(b"\n")?;
            }
            Ok(())
        },
    )
}

fn replace_events_wal(dir: &Path, events: &[Event], commit: bool) -> std::io::Result<()> {
    replace_via_tmp(
        &dir.join(EVENTS_WAL),
        &dir.join(EVENTS_WAL_TMP),
        commit,
        |f| wal::write_wal(f, events),
    )
}

fn replace_snapshot(dir: &Path, snap: &Snapshot, commit: bool) -> std::io::Result<()> {
    replace_via_tmp(
        &dir.join(SNAPSHOT_JSON),
        &dir.join(SNAPSHOT_JSON_TMP),
        commit,
        |f| {
            serde_json::to_writer(&mut *f, snap).map_err(json_err)?;
            Ok(())
        },
    )
}

fn read_jsonl_events(path: &Path) -> std::io::Result<Vec<Event>> {
    let f = File::open(path)?;
    let mut v = Vec::new();
    for line in BufReader::new(f).lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        v.push(serde_json::from_str(&line).map_err(json_err)?);
    }
    Ok(v)
}

fn event_ids(events: &[Event]) -> Vec<EventId> {
    events.iter().map(|e| e.id).collect()
}

/// `candidate` covers `covered` when ids match in order and `candidate` may be longer (D-04).
fn ids_cover(candidate: &[Event], covered: &[Event]) -> bool {
    let a = event_ids(candidate);
    let b = event_ids(covered);
    a.len() >= b.len() && a[..b.len()] == b[..]
}

/// D-04: jsonl is SoT; WAL is a lease. Prefer WAL only when recover yields magic+v1
/// and recovered ids cover jsonl ids in order. Header-only / magic-only / empty /
/// short / recover-err + readable jsonl → jsonl. Both parse and neither covers the
/// other → InvalidData. No jsonl and invalid WAL → InvalidData.
fn load_events_d04(dir: &Path) -> std::io::Result<Vec<Event>> {
    let wal_path = dir.join(EVENTS_WAL);
    let jsonl_path = dir.join(EVENTS_JSONL);
    let jsonl = if jsonl_path.exists() {
        Some(read_jsonl_events(&jsonl_path)?)
    } else {
        None
    };
    let wal = if wal_path.exists() {
        Some(wal::recover_events(&wal_path))
    } else {
        None
    };
    match (wal, jsonl) {
        (Some(Ok(w)), Some(j)) => {
            if ids_cover(&w, &j) {
                Ok(w)
            } else if ids_cover(&j, &w) {
                Ok(j)
            } else {
                Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "WAL and events.jsonl ids disagree (neither sequence covers the other)",
                ))
            }
        }
        (Some(Ok(w)), None) => Ok(w),
        (Some(Err(_)), Some(j)) => Ok(j),
        (Some(Err(e)), None) => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("invalid WAL and no events.jsonl: {e}"),
        )),
        (None, Some(j)) => Ok(j),
        (None, None) => Ok(Vec::new()),
    }
}

pub fn open(dir: &Path) -> std::io::Result<Runtime> {
    let snap_path = dir.join(SNAPSHOT_JSON);
    let terms_path = dir.join(TERMS_REL);
    let events = load_events_d04(dir)?;
    let outcomes = load_outcomes(&dir.join(OUTCOMES_REL))?;
    let justifications = load_justifications(&dir.join(JUSTIFICATIONS_REL))?;
    if snap_path.exists() {
        let snap: Snapshot = serde_json::from_reader(File::open(&snap_path)?).map_err(json_err)?;
        let graph: Vec<Event> = events
            .into_iter()
            .filter(|e| !matches!(e.op, Op::Define { .. }))
            .collect();
        // Hydrate trap: compare snapshot-held walls before from_snapshot rebuilds from the log.
        if snap.fold.tt_anchors_pinned {
            Runtime::check_tt_anchors(&snap.fold.tt_anchors, &graph).map_err(runtime_err)?;
        }
        let rt = Runtime::from_snapshot(snap, graph);
        rt.replay_check().map_err(runtime_err)?;
        return finish_open(rt, outcomes, justifications);
    }
    let defined = defined_names(&events);
    if !defined.is_empty() {
        let rt =
            Runtime::from_dict_and_events(defined, events, cascade_limit()).map_err(runtime_err)?;
        return finish_open(rt, outcomes, justifications);
    }
    if terms_path.exists() {
        let strings = read_terms(&terms_path)?;
        let rt =
            Runtime::from_dict_and_events(strings, events, cascade_limit()).map_err(runtime_err)?;
        return finish_open(rt, outcomes, justifications);
    }
    if events.is_empty() {
        let rt = Runtime::default();
        return finish_open(rt, outcomes, justifications);
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        "missing snapshot.json, terms.jsonl, and Op::Define events (cannot restore intern meanings)",
    ))
}

fn finish_open(
    mut rt: Runtime,
    outcomes: Option<Vec<PersistedQuantumOutcome>>,
    justifications: Option<Vec<Justification>>,
) -> std::io::Result<Runtime> {
    rt.hydrate_from_log();
    if let Some(rows) = outcomes {
        if rows.as_slice() != rt.outcome_records() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "outcomes sidecar disagrees with log QuantumOutcome rows",
            ));
        }
    }
    if let Some(rows) = justifications {
        if rows != rt.justification_records() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "justifications sidecar disagrees with log JustificationCite rows",
            ));
        }
    }
    Ok(rt)
}

fn encoded_log(runtime: &Runtime) -> Vec<Event> {
    let mut out: Vec<Event> = runtime
        .dictionary()
        .strings()
        .iter()
        .map(|name| Event::stable_define(name.clone(), 0))
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

fn load_outcomes(path: &Path) -> std::io::Result<Option<Vec<PersistedQuantumOutcome>>> {
    if !path.exists() {
        return Ok(None);
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
    Ok(Some(out))
}

fn write_justifications(dir: &Path, rows: &[Justification]) -> std::io::Result<()> {
    let mut f = File::create(dir.join(JUSTIFICATIONS_REL))?;
    for row in rows {
        serde_json::to_writer(&mut f, row).map_err(json_err)?;
        f.write_all(b"\n")?;
    }
    Ok(())
}

fn load_justifications(path: &Path) -> std::io::Result<Option<Vec<Justification>>> {
    if !path.exists() {
        return Ok(None);
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
    Ok(Some(out))
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
