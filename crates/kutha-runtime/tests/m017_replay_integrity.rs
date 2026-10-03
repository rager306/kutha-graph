//! M017 S02 / REPL-01..04 GREEN oracles. D-07: FF5/FF6/M010–M016 assertions stay in their own files.

use kutha_common::{Event, EventId, Op};
use kutha_runtime::{store, FoldIndexMode, PersistedQuantumOutcome, Runtime};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

const T_OLD: u64 = 2015;
const VF_FIRST: u64 = 2010;
const VF_SECOND: u64 = 2020;
const WAL_MAGIC: &[u8; 8] = b"KUTHAWAL";
const WAL_VERSION: u32 = 1;

/// Persist dirs under repo `tmp/` (gitignored), as 42-01 names `tmp/p42-store-*`.
fn persist_dir(label: &str) -> PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("tmp")
        .join(format!("p42-store-{label}-{nanos}"));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    store::ensure_legacy_dir(&dir).unwrap();
    dir
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

fn read_jsonl_events(path: &Path) -> Vec<Event> {
    let text = fs::read_to_string(path).unwrap();
    text.lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str::<Event>(l).expect("jsonl Event"))
        .collect()
}

fn jsonl_graph_ids(dir: &Path) -> Vec<EventId> {
    read_jsonl_events(&dir.join("events.jsonl"))
        .into_iter()
        .filter(|e| !matches!(e.op, Op::Define { .. }))
        .map(|e| e.id)
        .collect()
}

fn log_ids(rt: &Runtime) -> Vec<EventId> {
    rt.log().iter().map(|e| e.id).collect()
}

fn wal_header_only() -> Vec<u8> {
    let mut bytes = Vec::from(*WAL_MAGIC);
    bytes.extend_from_slice(&WAL_VERSION.to_le_bytes());
    assert_eq!(bytes.len(), 12, "D-04 header is magic + version u32");
    bytes
}

#[test]
fn wal_header_only_falls_back_to_jsonl() {
    let rt = seed_runtime();
    let dir = persist_dir("wal-hdr");
    store::persist(&rt, &dir).unwrap();
    let expected = jsonl_graph_ids(&dir);
    assert!(
        !expected.is_empty(),
        "seeded jsonl must keep graph events including TtAnchor"
    );
    fs::write(dir.join("events.wal"), wal_header_only()).unwrap();

    let opened = store::open(&dir).expect("D-04: header-only WAL + readable jsonl → open Ok");
    assert_eq!(
        log_ids(&opened),
        expected,
        "D-04: open log ids must equal jsonl graph ids, never an empty WAL recover"
    );

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn wal_valid_covering_preferred() {
    let rt = seed_runtime();
    let dir = persist_dir("wal-ok");
    store::persist(&rt, &dir).unwrap();
    let expected = jsonl_graph_ids(&dir);
    let opened = store::open(&dir).expect("honest persist then open");
    assert_eq!(
        log_ids(&opened),
        expected,
        "covering WAL must yield the same graph ids as jsonl"
    );
    opened.replay_check().unwrap();

    let _ = fs::remove_dir_all(&dir);
}

static FOLD_INDEX_ENV: Mutex<()> = Mutex::new(());

struct RestoreEnv {
    key: &'static str,
    prev: Option<String>,
}

impl Drop for RestoreEnv {
    fn drop(&mut self) {
        match &self.prev {
            Some(v) => std::env::set_var(self.key, v),
            None => std::env::remove_var(self.key),
        }
    }
}

fn set_env(key: &'static str, value: &str) -> RestoreEnv {
    let prev = std::env::var(key).ok();
    std::env::set_var(key, value);
    RestoreEnv { key, prev }
}

#[test]
fn wal_magic_only_falls_back_to_jsonl() {
    let rt = seed_runtime();
    let dir = persist_dir("wal-magic");
    store::persist(&rt, &dir).unwrap();
    let expected = jsonl_graph_ids(&dir);
    fs::write(dir.join("events.wal"), WAL_MAGIC.as_slice()).unwrap();

    let opened = store::open(&dir).expect("D-04: magic-only WAL + readable jsonl → open Ok");
    assert_eq!(
        log_ids(&opened),
        expected,
        "D-04: magic-only WAL must not hide intact jsonl"
    );

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn wal_jsonl_id_disagreement_fails_closed() {
    let rt = seed_runtime();
    let dir = persist_dir("id-diverge");
    store::persist(&rt, &dir).unwrap();
    let jsonl_path = dir.join("events.jsonl");
    let mut events = read_jsonl_events(&jsonl_path);
    let mut flipped = false;
    for e in &mut events {
        if matches!(e.op, Op::Define { .. }) {
            continue;
        }
        e.id = EventId::from_u128(0x42_01);
        flipped = true;
        break;
    }
    assert!(
        flipped,
        "jsonl must contain a graph event whose id can diverge from WAL"
    );
    let mut out = String::new();
    for e in &events {
        out.push_str(&serde_json::to_string(e).unwrap());
        out.push('\n');
    }
    fs::write(&jsonl_path, out).unwrap();

    match store::open(&dir) {
        Ok(_) => panic!("D-04: both parse and neither id sequence covers the other → open Err"),
        Err(_) => {}
    }

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn tampered_tt_anchor_wall_fails_replay_check() {
    let rt = seed_runtime();
    let dir = persist_dir("tt-tamper");
    store::persist(&rt, &dir).unwrap();
    fs::remove_file(dir.join("events.wal")).unwrap();
    let jsonl_path = dir.join("events.jsonl");
    let mut events = read_jsonl_events(&jsonl_path);
    let mut tampered = false;
    for e in &mut events {
        if let Op::TtAnchor { wall } = &mut e.op {
            *wall = wall.saturating_add(7);
            tampered = true;
        }
    }
    assert!(
        tampered,
        "hydrate trap: tamper persisted TtAnchor wall, not an in-memory fold"
    );
    let mut out = String::new();
    for e in &events {
        out.push_str(&serde_json::to_string(e).unwrap());
        out.push('\n');
    }
    fs::write(&jsonl_path, out).unwrap();

    match store::open(&dir) {
        Err(_) => {}
        Ok(opened) => {
            assert!(
                opened.replay_check().is_err(),
                "REPL-01: tampered persisted TtAnchor wall must fail replay_check"
            );
        }
    }

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn abort_snapshot_replace_keeps_previous_bytes() {
    let rt = seed_runtime();
    let dir = persist_dir("abort-snap");
    store::persist(&rt, &dir).unwrap();
    let dest = dir.join("snapshot.json");
    let before = fs::read(&dest).unwrap();
    assert!(!before.is_empty(), "persist must write snapshot.json");
    store::abort_snapshot_replace_before_rename(&dir, &rt)
        .expect_err("injected abort is not persist success");
    let after = fs::read(&dest).unwrap();
    assert!(
        after == before,
        "REPL-02: abort before snapshot replace must leave previous bytes (after {} vs before {})",
        after.len(),
        before.len()
    );

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn abort_wal_replace_keeps_previous_bytes() {
    let rt = seed_runtime();
    let dir = persist_dir("abort-wal");
    store::persist(&rt, &dir).unwrap();
    let dest = dir.join("events.wal");
    let before = fs::read(&dest).unwrap();
    assert!(!before.is_empty(), "persist must write events.wal");
    store::abort_wal_replace_before_rename(&dir, &rt)
        .expect_err("injected abort is not persist success");
    let after = fs::read(&dest).unwrap();
    assert!(
        after == before,
        "REPL-02: abort before WAL replace must leave previous bytes (after {} vs before {})",
        after.len(),
        before.len()
    );

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn mismatched_outcomes_sidecar_fails_open() {
    let rt = seed_runtime();
    let absent_dir = persist_dir("outcomes-absent");
    store::persist(&rt, &absent_dir).unwrap();
    let _ = fs::remove_file(absent_dir.join(store::OUTCOMES_REL));
    store::open(&absent_dir).expect("absent outcomes sidecar stays Ok (m012a discard)");
    let _ = fs::remove_dir_all(&absent_dir);

    let dir = persist_dir("outcomes-mismatch");
    store::persist(&rt, &dir).unwrap();
    let spoof = PersistedQuantumOutcome {
        quantum_id: "p42-spoof".into(),
        disposition: kutha_common::OutcomeDisposition::Full,
        aborted_on_budget: false,
        events_in_quantum: 99,
        event_ids: Vec::new(),
        receipt_digest_hex: "00".into(),
        resume_of: None,
    };
    fs::write(
        dir.join(store::OUTCOMES_REL),
        format!("{}\n", serde_json::to_string(&spoof).unwrap()),
    )
    .unwrap();

    match store::open(&dir) {
        Ok(_) => panic!("REPL-03: sidecar that disagrees with log records must fail open"),
        Err(_) => {}
    }

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn snapshot_index_mode_survives_env_flip() {
    let _lock = FOLD_INDEX_ENV.lock().unwrap();
    let mut rt = Runtime::with_index_mode(32, FoldIndexMode::Eager);
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
    assert_eq!(rt.index_mode(), FoldIndexMode::Eager);

    let dir = persist_dir("idx-mode");
    store::persist(&rt, &dir).unwrap();
    let fp = rt.fold().fingerprint();
    let known = rt.as_known_on(T_OLD).expect("anchor 2015");

    let _restore = set_env("KUTHA_FOLD_INDEX_MODE", "lazy");
    let opened = store::open(&dir).expect("open after env flip");
    assert_eq!(
        opened.index_mode(),
        FoldIndexMode::Eager,
        "REPL-04: snapshot pins FoldIndexMode; env must not win"
    );
    assert_eq!(opened.fold().fingerprint(), fp);
    assert_eq!(opened.as_known_on(T_OLD).expect("anchor rebuilt"), known);

    let _ = fs::remove_dir_all(&dir);
}

fn seed_anchor_tt(rt: &Runtime) -> u64 {
    rt.log()
        .iter()
        .find_map(|e| match &e.op {
            Op::TtAnchor { wall } if *wall == T_OLD => Some(e.ingested_at),
            _ => None,
        })
        .expect("seed emits TtAnchor at T_OLD")
}

#[test]
fn as_known_on_differs_from_live_at_max_on_closed_vt() {
    let mut rt = seed_runtime();
    let article = rt.intern("art-12-speed");
    let in_force = rt.intern("inForceAs");
    let limit_50 = rt.intern("50-kmh");
    let statute = (article, in_force, limit_50);
    let tt = seed_anchor_tt(&rt);
    let known = rt.as_known_on(T_OLD).expect("anchor 2015");
    let live_max = rt.live_at(tt, u64::MAX);
    assert_ne!(
        known, live_max,
        "KNOWN-03: closed VT stays visible on as_known_on, not live_at(tt, MAX)"
    );
    assert!(known.contains(&statute));
    assert!(!live_max.contains(&statute));
}

#[test]
fn known_at_equals_is_known_at_filter() {
    let rt = seed_runtime();
    let tt = seed_anchor_tt(&rt);
    let known = rt.fold().known_at(tt);
    let strings = rt.dictionary().strings();
    let brute: Vec<_> = rt
        .fold()
        .facts()
        .iter()
        .filter(|f| {
            f.is_known_at(tt)
                && strings
                    .get(f.relation as usize)
                    .map(|n| !n.starts_with("kutha:"))
                    .unwrap_or(true)
        })
        .map(|f| (f.subject, f.relation, f.object()))
        .collect();
    assert_eq!(
        known, brute,
        "KNOWN-03: known_at equals is_known_at + product keep"
    );
}
