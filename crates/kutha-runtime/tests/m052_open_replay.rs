//! M052 S04 / Phase 115 — snapshot-free Define/terms open must `replay_check`
//! (CE whole-repo P1 #11). Dropping `snapshot.json` must not admit a log that
//! snapshot-backed open rejects. Empty-pin inverse is the unreproducible
//! mutant (`ExecutionReplayFailed`); fingerprint stays unchanged (M016 S04).

use kutha_common::{Event, Op};
use kutha_runtime::{store, Runtime};
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

fn persist_dir(label: &str) -> PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("tmp")
        .join(format!(
            "m052-open-replay-{label}-{}-{}",
            std::process::id(),
            nanos
        ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    store::ensure_legacy_dir(&dir).unwrap();
    dir
}

fn seed_inverse() -> Runtime {
    let mut rt = Runtime::default();
    let a = rt.intern("A");
    let b = rt.intern("B");
    let knows = rt.intern("knows");
    rt.emit(Op::Assert {
        subject: a,
        relation: knows,
        object: b,
        valid_from: 0,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    assert!(
        rt.log().iter().any(|e| matches!(
            &e.op,
            Op::Behavior { name, .. } if name == "inverse_knows"
        )),
        "seed must persist a Horn-pinned inverse_knows Behavior"
    );
    rt.replay_check().unwrap();
    rt
}

fn persist_legacy(rt: &Runtime, dir: &Path) {
    store::persist(rt, dir).unwrap();
}

fn read_jsonl_events(path: &Path) -> Vec<Event> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
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

/// Empty-pin the inverse_knows Behavior on disk (RULE-04). WAL is dropped so
/// jsonl is the sole log; snapshot-backed open still sees `snapshot.json`.
fn tamper_empty_pin_inverse(dir: &Path) {
    let _ = fs::remove_file(dir.join("events.wal"));
    let jsonl = dir.join("events.jsonl");
    let mut events = read_jsonl_events(&jsonl);
    let mut pinned = 0usize;
    for e in &mut events {
        if let Op::Behavior {
            name, rule_version, ..
        } = &mut e.op
        {
            if name == "inverse_knows" {
                *rule_version = String::new();
                pinned += 1;
            }
        }
    }
    assert!(
        pinned > 0,
        "jsonl must contain inverse_knows Behavior to empty-pin"
    );
    write_jsonl_events(&jsonl, &events);
}

fn strip_define_ops(dir: &Path) {
    let jsonl = dir.join("events.jsonl");
    let events: Vec<Event> = read_jsonl_events(&jsonl)
        .into_iter()
        .filter(|e| !matches!(e.op, Op::Define { .. }))
        .collect();
    assert!(
        !events.is_empty(),
        "terms path still needs graph events after stripping Define"
    );
    write_jsonl_events(&jsonl, &events);
}

fn assert_open_rejects_replay(dir: &Path, context: &str) {
    match store::open(dir) {
        Err(err) => {
            assert_eq!(
                err.kind(),
                ErrorKind::InvalidData,
                "{context}: expected InvalidData, got {err}"
            );
            let msg = err.to_string();
            assert!(
                msg.contains("execution replay failed"),
                "{context}: expected ExecutionReplayFailed via replay_check, got {msg}"
            );
        }
        Ok(_) => panic!("{context}: open must reject the unreproducible log"),
    }
}

/// Control: snapshot-backed open already runs `replay_check` and rejects the
/// empty-pin inverse. The Define/terms kills below must not be weaker.
#[test]
fn snapshot_backed_open_rejects_empty_pin_inverse() {
    let rt = seed_inverse();
    let dir = persist_dir("snap");
    persist_legacy(&rt, &dir);
    tamper_empty_pin_inverse(&dir);
    assert!(dir.join("snapshot.json").exists());
    assert_open_rejects_replay(&dir, "snapshot-backed open");
    let _ = fs::remove_dir_all(&dir);
}

/// Kill: after dropping snapshot.json, Define-backed reconstruction must still
/// fail closed on the same empty-pin log.
#[test]
fn define_backed_open_without_snapshot_rejects_empty_pin_inverse() {
    let rt = seed_inverse();
    let dir = persist_dir("define");
    persist_legacy(&rt, &dir);
    tamper_empty_pin_inverse(&dir);
    assert_open_rejects_replay(&dir, "snapshot-backed control before dropping snapshot");
    fs::remove_file(dir.join("snapshot.json")).unwrap();
    assert!(
        !dir.join("snapshot.json").exists(),
        "Define path must reconstruct without snapshot.json"
    );
    assert_open_rejects_replay(&dir, "Define-backed open without snapshot");
    let _ = fs::remove_dir_all(&dir);
}

/// Kill: terms.jsonl reconstruction (no snapshot, no Op::Define) must reject
/// the same empty-pin log snapshot-backed open rejects.
#[test]
fn terms_backed_open_without_snapshot_rejects_empty_pin_inverse() {
    let rt = seed_inverse();
    let dir = persist_dir("terms");
    persist_legacy(&rt, &dir);
    tamper_empty_pin_inverse(&dir);
    assert_open_rejects_replay(&dir, "snapshot-backed control before dropping snapshot");
    fs::remove_file(dir.join("snapshot.json")).unwrap();
    assert!(
        dir.join("terms.jsonl").exists(),
        "terms path needs terms.jsonl"
    );
    strip_define_ops(&dir);
    assert_open_rejects_replay(&dir, "terms-backed open without snapshot");
    let _ = fs::remove_dir_all(&dir);
}
