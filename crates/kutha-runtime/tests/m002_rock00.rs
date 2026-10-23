//! ROCK-00 oracles (Phase 51). Feature-gated; default workspace tests never compile RocksDB.

#![cfg(feature = "rocks-index-spike")]

use kutha_common::{Event, EventId, Op};
use kutha_runtime::Runtime;
use kutha_runtime::rocks_spike;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Seek};
use std::path::Path;

mod common;

const T_OLD: u64 = 2015;
const VF_FIRST: u64 = 2010;
const VF_SECOND: u64 = 2020;

fn persist_dir(label: &str) -> common::PersistScratch {
    let scratch = common::PersistScratch::new(&format!("p51-store-{label}"));
    kutha_runtime::store::ensure_legacy_dir(scratch.as_ref()).unwrap();
    scratch
}

fn seed_runtime() -> Runtime {
    let mut rt = Runtime::default();
    let article = rt.intern("art-12-speed");
    let in_force = rt.intern("inForceAs");
    let limit_50 = rt.intern("50-kmh");
    rt.emit(Op::Assert {
        subject: article,
        relation: in_force,
        object: limit_50,
        valid_from: VF_FIRST,
        valid_to: Some(VF_SECOND),
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    rt.emit_tt_anchor(T_OLD).unwrap();
    rt
}

fn jsonl_id_offsets(path: &Path) -> Vec<(EventId, u64)> {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut out = Vec::new();
    loop {
        let offset = reader.stream_position().unwrap();
        line.clear();
        if reader.read_line(&mut line).unwrap() == 0 {
            break;
        }
        if line.trim().is_empty() {
            continue;
        }
        let event: Event = serde_json::from_str(line.trim_end()).expect("jsonl Event");
        out.push((event.id, offset));
    }
    out
}

#[test]
fn rocks_roundtrips_event_id_to_offset() {
    let dir = persist_dir("roundtrip");
    let rt = seed_runtime();
    rt.persist(&dir).unwrap();
    let log_path = dir.join("events.jsonl");
    let rocks_dir = dir.join("rocks");
    let expected = jsonl_id_offsets(&log_path);
    assert!(
        !expected.is_empty(),
        "seeded persist must write events.jsonl"
    );

    let n = rocks_spike::build_index(&log_path, &rocks_dir).unwrap();
    assert_eq!(n, expected.len());

    for (id, offset) in &expected {
        let found = rocks_spike::lookup(&rocks_dir, *id)
            .unwrap()
            .expect("id from the log must resolve");
        assert_eq!(found, *offset);
        let loaded = rocks_spike::read_event_at(&log_path, found).unwrap();
        assert_eq!(loaded.id, *id);
    }

    let missing = EventId::nil();
    assert!(
        !expected.iter().any(|(id, _)| *id == missing),
        "nil id must not appear in the seeded log"
    );
    assert_eq!(rocks_spike::lookup(&rocks_dir, missing).unwrap(), None);
}

#[test]
fn rocks_dir_is_droppable() {
    let dir = persist_dir("droppable");
    let rt = seed_runtime();
    rt.persist(&dir).unwrap();
    let log_path = dir.join("events.jsonl");
    let rocks_dir = dir.join("rocks");
    let expected = jsonl_id_offsets(&log_path);

    rocks_spike::build_index(&log_path, &rocks_dir).unwrap();
    fs::remove_dir_all(&rocks_dir).unwrap();
    assert!(!rocks_dir.exists());

    let n = rocks_spike::build_index(&log_path, &rocks_dir).unwrap();
    assert_eq!(n, expected.len());
    for (id, offset) in expected {
        assert_eq!(rocks_spike::lookup(&rocks_dir, id).unwrap(), Some(offset));
    }
}

#[test]
fn malformed_log_line_fails_closed() {
    let dir = persist_dir("malformed");
    let rt = seed_runtime();
    rt.persist(&dir).unwrap();
    let log_path = dir.join("events.jsonl");
    let rocks_dir = dir.join("rocks");

    let mut log = fs::read_to_string(&log_path).unwrap();
    log.push_str("{ not an event\n");
    fs::write(&log_path, log).unwrap();

    let err = rocks_spike::build_index(&log_path, &rocks_dir)
        .expect_err("a malformed line must not be skipped");
    assert_eq!(err.kind(), std::io::ErrorKind::InvalidData);
}
