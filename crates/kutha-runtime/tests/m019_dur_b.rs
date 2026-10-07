//! M019 S06 / Phase 61 durability residuals B (default features).
//! DUR-export-race: `export_jsonl` takes `kutha.lock` before reading (Phase 60).
//! D-07: FF5/FF6/M010–M017/`m002_*` assertions unchanged.

use kutha_common::{Event, Op};
use kutha_runtime::{Runtime, store};
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

const T_OLD: u64 = 2015;
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
        .join(format!("p61-dur-{label}-{nanos}"));
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

fn assert_writer_lock_error(err: &std::io::Error) {
    assert_eq!(
        err.kind(),
        ErrorKind::WouldBlock,
        "export against a live writer must fail closed with WouldBlock; got {err}"
    );
    let msg = err.to_string();
    assert!(
        msg.contains(store::STORE_LOCK_REL),
        "error must name the lock file {}; got {msg}",
        store::STORE_LOCK_REL
    );
}

fn read_jsonl_events(path: &Path) -> Vec<Event> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| serde_json::from_str(l).expect("jsonl event"))
        .collect()
}

/// Mutation: drop `acquire_store_write_lock` in `export_jsonl`.
#[test]
fn export_jsonl_while_persister_holds_lock_fails_closed() {
    let rt = seed_runtime();
    let dir = persist_dir("export-lock");
    store::persist(&rt, &dir).unwrap();
    assert!(
        !dir.join("events.jsonl").exists(),
        "segmented persist must not write jsonl"
    );
    let _p = store::Persister::attach(&rt, &dir).unwrap();
    let err = store::export_jsonl(&dir).expect_err("export must not run under a live writer");
    assert_writer_lock_error(&err);
    assert!(
        !dir.join("events.jsonl").exists(),
        "fail-closed export must write nothing"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn export_jsonl_after_writer_drops_equals_log() {
    let rt = seed_runtime();
    let dir = persist_dir("export-after");
    store::persist(&rt, &dir).unwrap();
    {
        let _p = store::Persister::attach(&rt, &dir).unwrap();
        store::export_jsonl(&dir).expect_err("writer still holds the lock");
        assert!(!dir.join("events.jsonl").exists());
    }
    store::export_jsonl(&dir).unwrap();
    let exported = read_jsonl_events(&dir.join("events.jsonl"));
    let want: Vec<_> = store::encoded_events(&rt).iter().map(|e| e.id).collect();
    let got: Vec<_> = exported.iter().map(|e| e.id).collect();
    assert_eq!(got, want);
    let _ = fs::remove_dir_all(&dir);
}

/// Export must take the lock before `load_log_events` so it never observes a
/// half-appended frame. Mutation: load then lock, or skip the lock.
#[test]
fn export_jsonl_takes_lock_before_reading() {
    let src = include_str!("../src/store.rs");
    let fn_start = src
        .find("pub fn export_jsonl")
        .expect("export_jsonl must exist");
    let body = &src[fn_start..];
    let lock = body
        .find("acquire_store_write_lock")
        .expect("export_jsonl must take the writer lock");
    let load = body
        .find("load_log_events")
        .expect("export_jsonl must read the log");
    assert!(
        lock < load,
        "export_jsonl must acquire kutha.lock before load_log_events"
    );
}

/// Characterization (ADR-023 D023-2): cheap persist copies only. Remaining
/// `bytes_per_fact` is whole-bench RSS (ingest + persist transients + a second
/// Runtime on open), not ingest live. Mutation: concatenate frames before create.
#[test]
fn write_segment_file_streams_frames() {
    let src = include_str!("../src/segment.rs");
    let fn_start = src
        .find("fn write_segment_file_commit")
        .expect("write_segment_file_commit must exist");
    let body = &src[fn_start..];
    let replace = body
        .find("replace_via_tmp")
        .expect("write_segment_file_commit must use replace_via_tmp");
    let write = body
        .find("f.write_all(&frame)")
        .expect("write_segment_file_commit must write each frame");
    assert!(
        replace < write,
        "must open tmp via replace_via_tmp before writing frames"
    );
    let prefix = &body[..write];
    assert!(
        !prefix.contains("frames.extend") && !prefix.contains("all_frames"),
        "must not concatenate frames before the write"
    );
    assert!(
        !prefix.contains("File::create(path)"),
        "must not truncate the final segment path in place"
    );
}

/// Characterization (ADR-023 D023-2): encoded_log is dropped before snapshot.
#[test]
fn persist_drops_encoded_log_before_snapshot() {
    let src = include_str!("../src/store.rs");
    let fn_start = src
        .find("fn persist_unlocked")
        .expect("persist_unlocked must exist");
    let body = &src[fn_start..];
    let drop_events = body
        .find("drop(events)")
        .expect("persist_unlocked must drop encoded_log");
    let snapshot = body
        .find("replace_snapshot")
        .expect("persist_unlocked must write the snapshot");
    assert!(
        drop_events < snapshot,
        "encoded_log must be dropped before snapshot clone"
    );
}
