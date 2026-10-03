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
    let relation = rt.intern("relatedTo");
    let object = rt.intern("ckpt-obj");
    for i in 0..store::CHECKPOINT_EVERY {
        let s = rt.intern(&format!("p52-ckpt-{i}"));
        rt.emit(Op::Assert {
            subject: s,
            relation,
            object,
            valid_from: VF_FIRST,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap();
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

fn assert_dict_term_parity(src: &Runtime, opened: &Runtime) {
    assert_eq!(
        opened.dictionary().strings(),
        src.dictionary().strings(),
        "open must restore interned strings in intern order"
    );
    for s in src.dictionary().strings() {
        assert_eq!(
            opened.dictionary().id(s),
            src.dictionary().id(s),
            "TermId for {s} must match the source runtime"
        );
    }
}

fn last_jsonl_line_start(bytes: &[u8]) -> usize {
    assert!(
        bytes.last() == Some(&b'\n'),
        "jsonl must be newline-terminated"
    );
    let body = &bytes[..bytes.len() - 1];
    match body.iter().rposition(|b| *b == b'\n') {
        Some(i) => i + 1,
        None => 0,
    }
}

fn wal_last_frame_len(wal: &[u8], last: &Event) -> usize {
    let payload = serde_json::to_vec(last).unwrap();
    let frame_len = 8 + payload.len();
    assert!(wal.len() >= frame_len, "WAL shorter than last frame");
    let start = wal.len() - frame_len;
    let stored = u32::from_le_bytes(wal[start..start + 4].try_into().unwrap()) as usize;
    assert_eq!(stored, payload.len(), "WAL last frame length prefix");
    frame_len
}

fn assert_append_guard_writes_nothing(
    p: &mut store::Persister,
    rt: &Runtime,
    jsonl_path: &Path,
    wal_path: &Path,
    jsonl_before: &[u8],
    wal_before: &[u8],
) {
    match p.append(rt) {
        Ok(_) => panic!("D-04: tail-guard mismatch must return InvalidData"),
        Err(e) => assert_eq!(e.kind(), ErrorKind::InvalidData),
    }
    assert_eq!(
        fs::read(jsonl_path).unwrap(),
        jsonl_before,
        "D-04: mismatch must write nothing to jsonl"
    );
    assert_eq!(
        fs::read(wal_path).unwrap(),
        wal_before,
        "D-04: mismatch must write nothing to WAL"
    );
}

fn emit_gap_assert(rt: &mut Runtime, subject: &str, object: &str) {
    let s = rt.intern(subject);
    let o = rt.intern(object);
    let r = rt.intern("inForceAs");
    rt.emit(Op::Assert {
        subject: s,
        relation: r,
        object: o,
        valid_from: VF_FIRST,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
}

#[test]
fn intern_between_appends_survives_open_without_checkpoint() {
    let mut rt = seed_runtime();
    let dir = persist_dir("gap-a");
    store::persist(&rt, &dir).unwrap();
    let offset_a = snap_log_offset(&dir);
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    emit_one_more(&mut rt, T_NEW);
    p.append(&rt).unwrap();
    emit_gap_assert(&mut rt, "p52-gap-article", "p52-gap-80");
    p.append(&rt).unwrap();
    assert_eq!(
        snap_log_offset(&dir),
        offset_a,
        "GAP A: intern-append must not checkpoint"
    );
    let opened = store::open(&dir).expect("open after intern-append without checkpoint");
    assert_dict_term_parity(&rt, &opened);
    assert_eq!(opened.fold().fingerprint(), rt.fold().fingerprint());
    assert_eq!(opened.as_of(VF_FIRST), rt.as_of(VF_FIRST));
    opened.replay_check().unwrap();

    let dir_full = persist_dir("gap-a-full");
    store::persist(&rt, &dir_full).unwrap();
    let from_full = store::open(&dir_full).unwrap();
    assert_eq!(log_ids(&opened), log_ids(&from_full));
    assert_dict_term_parity(&from_full, &opened);

    let _ = fs::remove_dir_all(&dir);
    let _ = fs::remove_dir_all(&dir_full);
}

#[test]
fn intern_only_append_reattach_and_open() {
    let mut rt = seed_runtime();
    let dir = persist_dir("intern-only");
    store::persist(&rt, &dir).unwrap();
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    let t_a = rt.intern("p52-intern-only-a");
    let t_b = rt.intern("p52-intern-only-b");
    p.append(&rt)
        .expect("intern-only quantum must append Define events");
    drop(p);
    let mut p = store::Persister::attach(&rt, &dir)
        .expect("attach after intern-only append must not InvalidData");
    p.append(&rt).unwrap();
    emit_gap_assert(&mut rt, "p52-intern-only-c", "p52-intern-only-d");
    p.append(&rt).unwrap();
    let opened = store::open(&dir).unwrap();
    assert_eq!(opened.dictionary().id("p52-intern-only-a"), Some(t_a));
    assert_eq!(opened.dictionary().id("p52-intern-only-b"), Some(t_b));
    assert_dict_term_parity(&rt, &opened);
    assert_eq!(opened.fold().fingerprint(), rt.fold().fingerprint());
    assert_eq!(opened.as_of(VF_FIRST), rt.as_of(VF_FIRST));
    opened.replay_check().unwrap();

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn append_tail_guard_rejects_truncated_or_garbage_logs() {
    let rt = seed_runtime();
    let template = persist_dir("guard-len-a");
    store::persist(&rt, &template).unwrap();

    let cases: [(&str, Box<dyn Fn(&[u8], &[u8], &Event) -> (Vec<u8>, Vec<u8>)>); 6] = [
        (
            "jsonl-truncate-record",
            Box::new(|j, w, _| {
                let start = last_jsonl_line_start(j);
                (j[..start].to_vec(), w.to_vec())
            }),
        ),
        (
            "wal-truncate-record",
            Box::new(|j, w, last| {
                let fl = wal_last_frame_len(w, last);
                (j.to_vec(), w[..w.len() - fl].to_vec())
            }),
        ),
        (
            "jsonl-duplicate-last",
            Box::new(|j, w, _| {
                let start = last_jsonl_line_start(j);
                let mut out = j.to_vec();
                out.extend_from_slice(&j[start..]);
                (out, w.to_vec())
            }),
        ),
        (
            "wal-duplicate-last",
            Box::new(|j, w, last| {
                let fl = wal_last_frame_len(w, last);
                let mut out = w.to_vec();
                out.extend_from_slice(&w[w.len() - fl..]);
                (j.to_vec(), out)
            }),
        ),
        (
            "jsonl-garbage-bytes",
            Box::new(|j, w, _| {
                let mut out = j.to_vec();
                out.extend_from_slice(b"not-a-json-record\n");
                (out, w.to_vec())
            }),
        ),
        (
            "wal-garbage-bytes",
            Box::new(|j, w, _| {
                let mut out = w.to_vec();
                out.extend_from_slice(&[0xFF, 0x00, 0xAA, 0xBB]);
                (j.to_vec(), out)
            }),
        ),
    ];
    for (label, tamper) in cases {
        let dir = persist_dir(&format!("guard-len-{label}"));
        copy_store(&template, &dir);
        let jsonl_path = dir.join("events.jsonl");
        let wal_path = dir.join("events.wal");
        let mut opened = store::open(&dir).unwrap();
        let mut p = store::Persister::attach(&opened, &dir).unwrap();
        let jsonl_orig = fs::read(&jsonl_path).unwrap();
        let wal_orig = fs::read(&wal_path).unwrap();
        let last = read_jsonl_events(&jsonl_path).pop().expect("jsonl events");
        let (j2, w2) = tamper(&jsonl_orig, &wal_orig, &last);
        assert_ne!(
            (&j2[..], &w2[..]),
            (&jsonl_orig[..], &wal_orig[..]),
            "{label}: tamper must change a file"
        );
        fs::write(&jsonl_path, &j2).unwrap();
        fs::write(&wal_path, &w2).unwrap();
        emit_one_more(&mut opened, T_NEW);
        assert_append_guard_writes_nothing(&mut p, &opened, &jsonl_path, &wal_path, &j2, &w2);
        let _ = fs::remove_dir_all(&dir);
    }

    let _ = fs::remove_dir_all(&template);
}

#[test]
fn append_tail_guard_rejects_same_length_last_record_swap() {
    let mut rt = seed_runtime();
    let dir = persist_dir("guard-id");
    store::persist(&rt, &dir).unwrap();
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    let jsonl_path = dir.join("events.jsonl");
    let wal_path = dir.join("events.wal");
    let jsonl_orig = fs::read(&jsonl_path).unwrap();
    let wal_orig = fs::read(&wal_path).unwrap();
    let mut events = read_jsonl_events(&jsonl_path);
    let last = events.last_mut().expect("jsonl last");
    let old_json = serde_json::to_string(last).unwrap();
    last.id = EventId::from_u128(last.id.as_u128().wrapping_add(1));
    let new_json = serde_json::to_string(last).unwrap();
    assert_eq!(
        old_json.len(),
        new_json.len(),
        "swap must keep jsonl record length"
    );
    let start = last_jsonl_line_start(&jsonl_orig);
    let mut tampered = jsonl_orig[..start].to_vec();
    tampered.extend_from_slice(new_json.as_bytes());
    tampered.push(b'\n');
    assert_eq!(tampered.len(), jsonl_orig.len(), "same total jsonl length");
    fs::write(&jsonl_path, &tampered).unwrap();
    emit_one_more(&mut rt, T_NEW);
    assert_append_guard_writes_nothing(&mut p, &rt, &jsonl_path, &wal_path, &tampered, &wal_orig);

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn append_twice_then_noop_keeps_prefix_and_matches_persist() {
    let mut rt = seed_runtime();
    let dir = persist_dir("append-i");
    store::persist(&rt, &dir).unwrap();
    let jsonl0 = fs::read(dir.join("events.jsonl")).unwrap();
    let wal0 = fs::read(dir.join("events.wal")).unwrap();
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    emit_one_more(&mut rt, T_NEW);
    p.append(&rt).unwrap();
    let jsonl1 = fs::read(dir.join("events.jsonl")).unwrap();
    let wal1 = fs::read(dir.join("events.wal")).unwrap();
    assert!(jsonl1.starts_with(&jsonl0) && jsonl1.len() > jsonl0.len());
    assert!(wal1.starts_with(&wal0) && wal1.len() > wal0.len());
    emit_gap_assert(&mut rt, "p52-twice-a", "p52-twice-b");
    p.append(&rt).unwrap();
    let jsonl2 = fs::read(dir.join("events.jsonl")).unwrap();
    let wal2 = fs::read(dir.join("events.wal")).unwrap();
    assert!(jsonl2.starts_with(&jsonl1) && jsonl2.len() > jsonl1.len());
    assert!(wal2.starts_with(&wal1) && wal2.len() > wal1.len());
    p.append(&rt).unwrap();
    assert_eq!(fs::read(dir.join("events.jsonl")).unwrap(), jsonl2);
    assert_eq!(fs::read(dir.join("events.wal")).unwrap(), wal2);
    emit_one_more(&mut rt, T_NEW + 5);
    p.append(&rt).unwrap();
    let jsonl3 = fs::read(dir.join("events.jsonl")).unwrap();
    let wal3 = fs::read(dir.join("events.wal")).unwrap();
    assert!(jsonl3.starts_with(&jsonl2) && jsonl3.len() > jsonl2.len());
    assert!(wal3.starts_with(&wal2) && wal3.len() > wal2.len());

    let opened = store::open(&dir).unwrap();
    let dir_full = persist_dir("append-i-full");
    store::persist(&rt, &dir_full).unwrap();
    let from_full = store::open(&dir_full).unwrap();
    assert_eq!(log_ids(&opened), log_ids(&from_full));
    assert_dict_term_parity(&from_full, &opened);
    assert_eq!(opened.fold().fingerprint(), from_full.fold().fingerprint());
    assert_eq!(opened.as_of(VF_FIRST), from_full.as_of(VF_FIRST));
    opened.replay_check().unwrap();

    let _ = fs::remove_dir_all(&dir);
    let _ = fs::remove_dir_all(&dir_full);
}

#[test]
fn checkpoint_abort_before_snapshot_rename_keeps_previous_bytes() {
    let mut rt = seed_runtime();
    let dir = persist_dir("ckpt-abort");
    store::persist(&rt, &dir).unwrap();
    let snap = dir.join("snapshot.json");
    let terms = dir.join("terms.jsonl");
    let outcomes = dir.join(store::OUTCOMES_REL);
    let justifications = dir.join(store::JUSTIFICATIONS_REL);
    let snap_before = fs::read(&snap).unwrap();
    let terms_before = fs::read(&terms).unwrap();
    let outcomes_before = fs::read(&outcomes).unwrap();
    let just_before = fs::read(&justifications).unwrap();
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    emit_one_more(&mut rt, T_NEW);
    p.append(&rt).unwrap();
    store::abort_checkpoint_snapshot_before_rename(&mut p, &rt)
        .expect_err("abort before snapshot rename is not checkpoint success");
    assert_eq!(fs::read(&snap).unwrap(), snap_before);
    assert_eq!(fs::read(&terms).unwrap(), terms_before);
    assert_eq!(fs::read(&outcomes).unwrap(), outcomes_before);
    assert_eq!(fs::read(&justifications).unwrap(), just_before);

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn checkpoint_leaves_no_tmp_and_fsyncs_dir() {
    let mut rt = seed_runtime();
    let dir = persist_dir("ckpt-fsync");
    store::persist(&rt, &dir).unwrap();
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    emit_one_more(&mut rt, T_NEW);
    p.append(&rt).unwrap();
    let before = store::persist_dir_fsync_count();
    p.checkpoint(&rt).unwrap();
    let delta = store::persist_dir_fsync_count().saturating_sub(before);
    let tmp: Vec<_> = fs::read_dir(&dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.file_name())
        .filter(|n| n.to_string_lossy().ends_with(".tmp"))
        .collect();
    assert!(
        tmp.is_empty(),
        "checkpoint Ok must not leave .tmp siblings, found {tmp:?}"
    );
    assert_eq!(
        delta, 5,
        "D-06: one dir fsync per lease rename (snapshot, terms, outcomes, justifications) plus one at the end; count={delta}"
    );

    let _ = fs::remove_dir_all(&dir);
}
