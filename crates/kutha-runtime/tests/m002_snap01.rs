//! M002 S02 / SNAP-01, OPEN-01, CRASH-01 oracles (Phase 50).
//! D-08: RED records current failures; FF5/FF6/M010–M017 assertions stay in their files.

use kutha_common::{Event, EventId, Op};
use kutha_runtime::{store, PersistedQuantumOutcome, Runtime, Snapshot};
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

const T_OLD: u64 = 2015;
const T_NEW: u64 = 2021;
const VF_FIRST: u64 = 2010;
const VF_SECOND: u64 = 2020;

fn persist_dir(label: &str) -> PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("tmp")
        .join(format!("p50-store-{label}-{nanos}"));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
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

fn seed_two_outcomes() -> Runtime {
    let mut rt = seed_runtime();
    let article = rt.intern("art-12-speed");
    let in_force = rt.intern("inForceAs");
    let limit_50 = rt.intern("50-kmh");
    rt.emit(Op::Assert {
        subject: article,
        relation: in_force,
        object: limit_50,
        valid_from: VF_FIRST,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    rt
}

fn log_ids(rt: &Runtime) -> Vec<EventId> {
    rt.log()
        .iter()
        .filter(|e| !matches!(e.op, Op::Define { .. }))
        .map(|e| e.id)
        .collect()
}

fn read_jsonl_events(path: &Path) -> Vec<Event> {
    let text = fs::read_to_string(path).unwrap();
    text.lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str::<Event>(l).expect("jsonl Event"))
        .collect()
}

fn write_jsonl_events(path: &Path, events: &[Event]) {
    let mut out = String::new();
    for e in events {
        out.push_str(&serde_json::to_string(e).unwrap());
        out.push('\n');
    }
    fs::write(path, out).unwrap();
}

fn copy_store(src: &Path, dst: &Path) {
    let _ = fs::remove_dir_all(dst);
    fs::create_dir_all(dst).unwrap();
    for ent in fs::read_dir(src).unwrap() {
        let ent = ent.unwrap();
        if ent.file_type().unwrap().is_file() {
            fs::copy(ent.path(), dst.join(ent.file_name())).unwrap();
        }
    }
}

fn outcome_lines(dir: &Path) -> Vec<String> {
    fs::read_to_string(dir.join(store::OUTCOMES_REL))
        .unwrap()
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.to_string())
        .collect()
}

#[test]
fn sidecar_truncated_mid_line_opens_from_log() {
    let rt = seed_two_outcomes();
    let dir = persist_dir("sidecar-trunc");
    store::persist(&rt, &dir).unwrap();
    let expected = log_ids(&rt);
    let path = dir.join(store::OUTCOMES_REL);
    let lines = outcome_lines(&dir);
    assert!(
        !lines.is_empty(),
        "persist must write a non-trivial outcomes sidecar"
    );
    let last = lines.last().unwrap();
    assert!(last.len() > 8, "outcome row must be long enough to split");
    let mut torn = String::new();
    for line in &lines[..lines.len() - 1] {
        torn.push_str(line);
        torn.push('\n');
    }
    torn.push_str(&last[..last.len() / 2]);
    fs::write(&path, torn).unwrap();

    let opened = store::open(&dir).expect("D-04: truncated sidecar is absent; intact log opens");
    assert_eq!(log_ids(&opened), expected);
    opened.replay_check().unwrap();

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn sidecar_stale_prefix_opens_from_log() {
    let rt = seed_two_outcomes();
    let dir = persist_dir("sidecar-prefix");
    store::persist(&rt, &dir).unwrap();
    let expected = log_ids(&rt);
    let lines = outcome_lines(&dir);
    assert!(
        lines.len() >= 2,
        "two user emits must persist at least two outcome rows"
    );
    fs::write(dir.join(store::OUTCOMES_REL), format!("{}\n", lines[0])).unwrap();

    let opened =
        store::open(&dir).expect("D-04: proper-prefix sidecar is absent; intact log opens");
    assert_eq!(log_ids(&opened), expected);
    opened.replay_check().unwrap();

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn sidecar_unparseable_terms_is_absent() {
    let rt = seed_runtime();
    let dir = persist_dir("terms-garbage");
    store::persist(&rt, &dir).unwrap();
    fs::remove_file(dir.join("snapshot.json")).unwrap();
    fs::remove_file(dir.join("events.wal")).unwrap();
    let events: Vec<Event> = read_jsonl_events(&dir.join("events.jsonl"))
        .into_iter()
        .filter(|e| !matches!(e.op, Op::Define { .. }))
        .collect();
    assert!(
        !events.is_empty(),
        "stripped jsonl must keep graph events so intern restore is required"
    );
    write_jsonl_events(&dir.join("events.jsonl"), &events);
    fs::write(dir.join("terms.jsonl"), "this is not json\n").unwrap();

    match store::open(&dir) {
        Ok(_) => panic!("D-04: unparseable last-resort terms are absent; intern restore must fail"),
        Err(e) => {
            let msg = e.to_string();
            assert!(
                msg.contains("cannot restore intern meanings"),
                "unparseable terms must surface intern-meanings, not a serde parse error; got {msg}"
            );
        }
    }

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn sidecar_spoof_not_prefix_still_fails() {
    let rt = seed_runtime();
    let dir = persist_dir("sidecar-spoof");
    store::persist(&rt, &dir).unwrap();
    let spoof = PersistedQuantumOutcome {
        quantum_id: "p50-spoof".into(),
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
        Ok(_) => panic!("D-04: well-formed rows that are not a log prefix must fail InvalidData"),
        Err(e) => assert_eq!(e.kind(), ErrorKind::InvalidData),
    }

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn persist_leaves_no_tmp_files() {
    let rt = seed_runtime();
    let dir = persist_dir("no-tmp");
    let before = store::persist_dir_fsync_count();
    store::persist(&rt, &dir).unwrap();
    let delta = store::persist_dir_fsync_count().saturating_sub(before);
    let tmp: Vec<_> = fs::read_dir(&dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.file_name())
        .filter(|n| n.to_string_lossy().ends_with(".tmp"))
        .collect();
    assert!(
        tmp.is_empty(),
        "persist Ok must not leave .tmp siblings, found {tmp:?}"
    );
    assert!(
        delta >= 6,
        "D-02: persist must dir-fsync at least once per rename (6 files); count={delta}"
    );

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn log_ahead_of_snapshot_opens_with_tail_fold() {
    let mut rt = seed_runtime();
    let template = persist_dir("ahead-a");
    store::persist(&rt, &template).unwrap();
    rt.emit_tt_anchor(T_NEW).unwrap();
    let expected = log_ids(&rt);

    let dir = persist_dir("ahead-b");
    copy_store(&template, &dir);
    store::persist_with_failpoint(&rt, &dir, store::PersistStep::Jsonl)
        .expect_err("failpoint after jsonl is not persist success");

    let opened = store::open(&dir).expect("D-05: log anchors at/after snap.next_tt are tail");
    assert_eq!(log_ids(&opened), expected);
    opened.replay_check().unwrap();

    let _ = fs::remove_dir_all(&template);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn extra_anchor_inside_snapshot_range_fails() {
    let rt = seed_runtime();
    let dir = persist_dir("extra-in-range");
    store::persist(&rt, &dir).unwrap();
    let snap: Snapshot =
        serde_json::from_reader(fs::File::open(dir.join("snapshot.json")).unwrap()).unwrap();
    assert!(
        snap.next_tt > 0,
        "seed persist must pin a non-zero snapshot next_tt"
    );
    let jsonl_path = dir.join("events.jsonl");
    let mut events = read_jsonl_events(&jsonl_path);
    let Some(template) = events.iter().find(|e| matches!(e.op, Op::TtAnchor { .. })) else {
        panic!("seed jsonl must contain a TtAnchor");
    };
    assert!(
        template.ingested_at < snap.next_tt,
        "copied anchor ingested_at must sit inside the snapshot covered range"
    );
    let extra = Event {
        id: EventId::from_u128(0x50_01),
        op: Op::TtAnchor { wall: 1990 },
        ingested_at: template.ingested_at,
        object_ids: Vec::new(),
    };
    events.push(extra);
    write_jsonl_events(&jsonl_path, &events);

    match store::open(&dir) {
        Ok(_) => panic!("D-05: extra TtAnchor with ingested_at < snap.next_tt must fail"),
        Err(e) => assert_eq!(e.kind(), ErrorKind::InvalidData),
    }

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn wal_short_header_without_jsonl_is_err() {
    let rt = seed_runtime();
    let dir = persist_dir("wal-short");
    store::persist(&rt, &dir).unwrap();
    fs::remove_file(dir.join("events.jsonl")).unwrap();
    fs::remove_file(dir.join("snapshot.json")).unwrap();
    let _ = fs::remove_file(dir.join(store::OUTCOMES_REL));
    let _ = fs::remove_file(dir.join(store::JUSTIFICATIONS_REL));
    fs::write(dir.join("events.wal"), b"KUTH").unwrap();

    match store::open(&dir) {
        Ok(_) => panic!("D-06: WAL shorter than magic+version and no jsonl must not yield a log"),
        Err(e) => {
            assert_eq!(e.kind(), ErrorKind::InvalidData);
            let msg = e.to_string();
            assert!(
                msg.contains("truncated WAL header") || msg.contains("invalid WAL"),
                "short header without jsonl must fail closed; got {msg}"
            );
        }
    }

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn wal_read_io_error_not_masked() {
    let rt = seed_runtime();
    let dir = persist_dir("wal-io");
    store::persist(&rt, &dir).unwrap();
    let wal = dir.join("events.wal");
    fs::remove_file(&wal).unwrap();
    fs::create_dir(&wal).unwrap();

    match store::open(&dir) {
        Ok(_) => panic!("D-06: non-InvalidData WAL I/O must not fall back to jsonl"),
        Err(e) => assert_ne!(
            e.kind(),
            ErrorKind::InvalidData,
            "disk I/O error must propagate, not become InvalidData jsonl fallback; got {e}"
        ),
    }

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn crash_matrix_every_step_reopens_to_prefix() {
    let mut rt = seed_runtime();
    let ids_a = log_ids(&rt);
    let template = persist_dir("crash-a");
    store::persist(&rt, &template).unwrap();
    rt.emit_tt_anchor(T_NEW).unwrap();
    let ids_b = log_ids(&rt);
    assert_ne!(ids_a, ids_b, "B must extend A");
    assert!(!ids_a.is_empty() && !ids_b.is_empty());

    let steps = [
        store::PersistStep::Wal,
        store::PersistStep::Jsonl,
        store::PersistStep::Terms,
        store::PersistStep::Snapshot,
        store::PersistStep::Outcomes,
        store::PersistStep::Justifications,
    ];
    for step in steps {
        let dir = persist_dir(&format!("crash-{step:?}"));
        copy_store(&template, &dir);
        store::persist_with_failpoint(&rt, &dir, step)
            .expect_err("failpoint must inject I/O error after the named step");
        match store::open(&dir) {
            Ok(opened) => {
                let ids = log_ids(&opened);
                assert!(
                    ids == ids_a || ids == ids_b,
                    "D-07: open after {step:?} must be A's or B's ids, got {ids:?}"
                );
                opened.replay_check().unwrap();
                if !matches!(step, store::PersistStep::Wal) {
                    assert_eq!(
                        ids, ids_b,
                        "D-07: steps after jsonl (including jsonl) must reopen as B"
                    );
                }
            }
            Err(e) => {
                assert_eq!(
                    e.kind(),
                    ErrorKind::InvalidData,
                    "D-07: open may fail closed InvalidData, not other kinds; {step:?} {e}"
                );
                assert!(
                    matches!(step, store::PersistStep::Wal),
                    "D-07: steps after jsonl must be Ok(B), got InvalidData at {step:?}: {e}"
                );
            }
        }
        let _ = fs::remove_dir_all(&dir);
    }

    let _ = fs::remove_dir_all(&template);
}
