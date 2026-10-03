//! M002 S04 / APPEND-01 oracles (Phase 52).
//! D-09: RED records current failures; FF5/FF6/M010–M017 assertions stay in their files.

use kutha_common::{Event, EventId, Op};
use kutha_runtime::{store, Runtime, Snapshot};
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
        .join(format!("p52-store-{label}-{nanos}"));
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

fn snap_log_offset(dir: &Path) -> usize {
    let snap: Snapshot =
        serde_json::from_reader(fs::File::open(dir.join("snapshot.json")).unwrap()).unwrap();
    snap.log_offset
}

fn emit_one_more(rt: &mut Runtime, wall: u64) {
    rt.emit_tt_anchor(wall).unwrap();
}

#[test]
fn append_keeps_prefix_bytes_of_jsonl_and_wal() {
    let mut rt = seed_runtime();
    let dir = persist_dir("prefix");
    store::persist(&rt, &dir).unwrap();
    let jsonl_before = fs::read(dir.join("events.jsonl")).unwrap();
    let wal_before = fs::read(dir.join("events.wal")).unwrap();
    assert!(!jsonl_before.is_empty() && !wal_before.is_empty());

    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    emit_one_more(&mut rt, T_NEW);
    p.append(&rt).unwrap();

    let jsonl_after = fs::read(dir.join("events.jsonl")).unwrap();
    let wal_after = fs::read(dir.join("events.wal")).unwrap();
    assert!(
        jsonl_after.len() > jsonl_before.len(),
        "append must grow jsonl"
    );
    assert!(wal_after.len() > wal_before.len(), "append must grow wal");
    assert_eq!(
        &jsonl_after[..jsonl_before.len()],
        jsonl_before.as_slice(),
        "D-03: jsonl prefix bytes must be unchanged"
    );
    assert_eq!(
        &wal_after[..wal_before.len()],
        wal_before.as_slice(),
        "D-03: WAL prefix bytes must be unchanged"
    );

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn append_one_log_sync_per_quantum() {
    let mut rt = seed_runtime();
    let dir = persist_dir("one-sync");
    store::persist(&rt, &dir).unwrap();
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    emit_one_more(&mut rt, T_NEW);
    let before = store::persist_log_sync_count();
    p.append(&rt).unwrap();
    let delta = store::persist_log_sync_count().saturating_sub(before);
    assert_eq!(
        delta, 1,
        "D-07: non-checkpoint append quantum must sync the log file exactly once; count={delta}"
    );

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn append_nothing_new_does_no_io() {
    let rt = seed_runtime();
    let dir = persist_dir("noop");
    store::persist(&rt, &dir).unwrap();
    let jsonl_before = fs::read(dir.join("events.jsonl")).unwrap();
    let wal_before = fs::read(dir.join("events.wal")).unwrap();
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    let before = store::persist_log_sync_count();
    p.append(&rt).unwrap();
    let delta = store::persist_log_sync_count().saturating_sub(before);
    assert_eq!(delta, 0, "D-03: append with nothing new must not sync");
    assert_eq!(
        fs::read(dir.join("events.jsonl")).unwrap(),
        jsonl_before,
        "D-03: append with nothing new must not write jsonl"
    );
    assert_eq!(
        fs::read(dir.join("events.wal")).unwrap(),
        wal_before,
        "D-03: append with nothing new must not write WAL"
    );

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn append_torn_wal_tail_drops_only_tail() {
    let rt = seed_runtime();
    let dir = persist_dir("wal-torn");
    store::persist(&rt, &dir).unwrap();
    let expected_prefix = {
        let events = read_jsonl_events(&dir.join("events.jsonl"));
        assert!(events.len() >= 2, "seed persist must write several events");
        events[..events.len() - 1]
            .iter()
            .filter(|e| !matches!(e.op, Op::Define { .. }))
            .map(|e| e.id)
            .collect::<Vec<_>>()
    };
    fs::remove_file(dir.join("events.jsonl")).unwrap();
    fs::remove_file(dir.join("snapshot.json")).unwrap();
    let wal = dir.join("events.wal");
    let mut bytes = fs::read(&wal).unwrap();
    bytes.truncate(bytes.len().saturating_sub(3));
    fs::write(&wal, &bytes).unwrap();

    let opened = store::open(&dir).expect("D-05: torn WAL tail drops; prefix + terms open");
    assert_eq!(log_ids(&opened), expected_prefix);
    opened.replay_check().unwrap();

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn append_torn_jsonl_tail_without_newline_is_dropped() {
    let rt = seed_runtime();
    let dir = persist_dir("jsonl-torn");
    store::persist(&rt, &dir).unwrap();
    let jsonl_path = dir.join("events.jsonl");
    let events = read_jsonl_events(&jsonl_path);
    assert!(events.len() >= 2, "seed persist must write several events");
    let kept = &events[..events.len() - 1];
    let last = events.last().unwrap();
    let last_json = serde_json::to_string(last).unwrap();
    assert!(
        !last_json.is_empty(),
        "last jsonl record must be a complete JSON object"
    );
    let mut torn = String::new();
    for e in kept {
        torn.push_str(&serde_json::to_string(e).unwrap());
        torn.push('\n');
    }
    // Complete JSON object, no terminating newline — D-05 torn tail.
    torn.push_str(&last_json);
    fs::write(&jsonl_path, torn).unwrap();
    fs::remove_file(dir.join("events.wal")).unwrap();
    fs::remove_file(dir.join("snapshot.json")).unwrap();

    let opened = store::open(&dir).expect("D-05: torn jsonl tail without newline is dropped");
    let got: Vec<EventId> = log_ids(&opened);
    let expect: Vec<EventId> = kept
        .iter()
        .filter(|e| !matches!(e.op, Op::Define { .. }))
        .map(|e| e.id)
        .collect();
    assert_eq!(
        got, expect,
        "D-05: a final jsonl line without \\n must be dropped, not parsed"
    );
    opened.replay_check().unwrap();

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn complete_malformed_jsonl_line_still_fails() {
    let rt = seed_runtime();
    let dir = persist_dir("jsonl-bad");
    store::persist(&rt, &dir).unwrap();
    let jsonl_path = dir.join("events.jsonl");
    let mut events = read_jsonl_events(&jsonl_path);
    assert!(!events.is_empty());
    write_jsonl_events(&jsonl_path, &events);
    let mut text = fs::read_to_string(&jsonl_path).unwrap();
    text.push_str("this is not json\n");
    fs::write(&jsonl_path, text).unwrap();

    match store::open(&dir) {
        Ok(_) => panic!("D-05: a complete newline-terminated malformed jsonl line must fail"),
        Err(e) => assert_eq!(e.kind(), ErrorKind::InvalidData),
    }
    events.clear();

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn append_replay_parity_with_full_persist() {
    let mut rt = seed_runtime();
    let dir_append = persist_dir("parity-a");
    store::persist(&rt, &dir_append).unwrap();
    let mut p = store::Persister::attach(&rt, &dir_append).unwrap();
    emit_one_more(&mut rt, T_NEW);
    p.append(&rt).unwrap();
    p.close(&rt).unwrap();

    let dir_full = persist_dir("parity-b");
    store::persist(&rt, &dir_full).unwrap();

    let opened_a = store::open(&dir_append).unwrap();
    let opened_b = store::open(&dir_full).unwrap();
    assert_eq!(log_ids(&opened_a), log_ids(&opened_b));
    assert_eq!(
        opened_a.fold().fingerprint(),
        opened_b.fold().fingerprint(),
        "append then open must match full persist fingerprint"
    );
    assert_eq!(opened_a.as_of(VF_FIRST), opened_b.as_of(VF_FIRST));
    opened_a.replay_check().unwrap();
    opened_b.replay_check().unwrap();

    let _ = fs::remove_dir_all(&dir_append);
    let _ = fs::remove_dir_all(&dir_full);
}

#[test]
fn append_tail_guard_mismatch_fails_closed_without_writing() {
    let mut rt = seed_runtime();
    let dir = persist_dir("guard");
    store::persist(&rt, &dir).unwrap();
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    let jsonl_path = dir.join("events.jsonl");
    let wal_path = dir.join("events.wal");
    let jsonl_before = fs::read(&jsonl_path).unwrap();
    let wal_before = fs::read(&wal_path).unwrap();
    let mut torn_wal = wal_before.clone();
    torn_wal.push(0xFF);
    fs::write(&wal_path, &torn_wal).unwrap();
    emit_one_more(&mut rt, T_NEW);

    match p.append(&rt) {
        Ok(_) => panic!("D-04: tail-guard mismatch must return InvalidData"),
        Err(e) => assert_eq!(e.kind(), ErrorKind::InvalidData),
    }
    assert_eq!(
        fs::read(&jsonl_path).unwrap(),
        jsonl_before,
        "D-04: mismatch must write nothing to jsonl"
    );
    assert_eq!(
        fs::read(&wal_path).unwrap(),
        torn_wal,
        "D-04: mismatch must write nothing to WAL"
    );

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn append_crash_matrix_reopens_to_prefix() {
    let mut rt = seed_runtime();
    let ids_a = log_ids(&rt);
    let template = persist_dir("crash-a");
    store::persist(&rt, &template).unwrap();
    emit_one_more(&mut rt, T_NEW);
    let ids_b = log_ids(&rt);
    assert_ne!(ids_a, ids_b, "B must extend A");

    let append_steps = [
        store::AppendStep::Jsonl,
        store::AppendStep::WalWrite,
        store::AppendStep::WalSync,
    ];
    for step in append_steps {
        let dir = persist_dir(&format!("crash-{step:?}"));
        copy_store(&template, &dir);
        let opened_pre = store::open(&dir).unwrap();
        let mut p = store::Persister::attach(&opened_pre, &dir).unwrap();
        store::append_with_failpoint(&mut p, &rt, step)
            .expect_err("failpoint must inject I/O error after the named append step");
        match store::open(&dir) {
            Ok(opened) => {
                let ids = log_ids(&opened);
                assert!(
                    ids == ids_a || ids == ids_b,
                    "D-08: open after {step:?} must be A's or B's ids, got {ids:?}"
                );
                opened.replay_check().unwrap();
            }
            Err(e) => {
                assert_eq!(
                    e.kind(),
                    ErrorKind::InvalidData,
                    "D-08: open may fail closed InvalidData, not other kinds; {step:?} {e}"
                );
            }
        }
        let _ = fs::remove_dir_all(&dir);
    }

    let ckpt_steps = [
        store::AppendStep::CheckpointJsonl,
        store::AppendStep::CheckpointTerms,
        store::AppendStep::CheckpointSnapshot,
        store::AppendStep::CheckpointOutcomes,
        store::AppendStep::CheckpointJustifications,
    ];
    for step in ckpt_steps {
        let dir = persist_dir(&format!("ckpt-{step:?}"));
        copy_store(&template, &dir);
        let opened_pre = store::open(&dir).unwrap();
        let mut p = store::Persister::attach(&opened_pre, &dir).unwrap();
        p.append(&rt).unwrap();
        store::checkpoint_with_failpoint(&mut p, &rt, step)
            .expect_err("failpoint must inject I/O error after the named checkpoint step");
        match store::open(&dir) {
            Ok(opened) => {
                let ids = log_ids(&opened);
                assert!(
                    ids == ids_a || ids == ids_b,
                    "D-08: open after checkpoint {step:?} must be A's or B's ids, got {ids:?}"
                );
                opened.replay_check().unwrap();
                if ids == ids_b {
                    // WAL sync already committed B before checkpoint.
                }
            }
            Err(e) => {
                assert_eq!(
                    e.kind(),
                    ErrorKind::InvalidData,
                    "D-08: open may fail closed InvalidData; {step:?} {e}"
                );
            }
        }
        let _ = fs::remove_dir_all(&dir);
    }

    let _ = fs::remove_dir_all(&template);
}

#[test]
fn open_between_checkpoints_uses_stale_leases() {
    let mut rt = seed_runtime();
    let dir = persist_dir("stale-lease");
    store::persist(&rt, &dir).unwrap();
    let offset_a = snap_log_offset(&dir);
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    emit_one_more(&mut rt, T_NEW);
    p.append(&rt).unwrap();
    assert_eq!(
        snap_log_offset(&dir),
        offset_a,
        "D-06: a small append must not rewrite snapshot.json"
    );
    let expected = log_ids(&rt);
    let opened = store::open(&dir).expect("D-06: stale snapshot + log ahead tail-folds");
    assert_eq!(log_ids(&opened), expected);
    opened.replay_check().unwrap();

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn checkpoint_every_threshold_refreshes_snapshot() {
    let mut rt = seed_runtime();
    let dir = persist_dir("ckpt-every");
    store::persist(&rt, &dir).unwrap();
    let offset_a = snap_log_offset(&dir);
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    for i in 0..store::CHECKPOINT_EVERY {
        rt.emit_tt_anchor(T_NEW.saturating_add(i as u64)).unwrap();
    }
    p.append(&rt).unwrap();
    let offset_b = snap_log_offset(&dir);
    assert!(
        offset_b > offset_a,
        "D-06: append of CHECKPOINT_EVERY events must refresh snapshot; {offset_a} -> {offset_b}"
    );
    let opened = store::open(&dir).unwrap();
    assert_eq!(log_ids(&opened), log_ids(&rt));
    opened.replay_check().unwrap();

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn append_after_open_resumes() {
    let rt = seed_runtime();
    let dir = persist_dir("resume");
    store::persist(&rt, &dir).unwrap();
    drop(rt);
    let mut opened = store::open(&dir).unwrap();
    let mut p = store::Persister::attach(&opened, &dir).unwrap();
    emit_one_more(&mut opened, T_NEW);
    p.append(&opened).unwrap();
    p.close(&opened).unwrap();
    let expected = log_ids(&opened);
    let again = store::open(&dir).unwrap();
    assert_eq!(log_ids(&again), expected);
    again.replay_check().unwrap();

    let _ = fs::remove_dir_all(&dir);
}
