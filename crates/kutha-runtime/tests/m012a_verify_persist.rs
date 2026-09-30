//! M012a S04 / DUR-01..03: verify-on-open, atomic events.jsonl, stable Define ids.

use kutha_common::{Event, Op};
use kutha_runtime::{store, Runtime};
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

fn uuid_like() -> String {
    format!(
        "{:x}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    )
}

fn temp_dir(label: &str) -> PathBuf {
    std::env::temp_dir().join(format!("kutha-m012a-s04-{label}-{}", uuid_like()))
}

fn seed_runtime() -> Runtime {
    let mut rt = Runtime::default();
    let a = rt.intern("Alice");
    let b = rt.intern("Bob");
    let rel = rt.intern("relatedTo");
    rt.emit(Op::Assert {
        subject: a,
        relation: rel,
        object: b,
        valid_from: 2017,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    rt
}

fn read_jsonl_events(path: &Path) -> Vec<Event> {
    let text = fs::read_to_string(path).unwrap();
    text.lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str::<Event>(l).expect("jsonl Event"))
        .collect()
}

#[test]
fn open_rejects_tampered_snapshot() {
    let rt = seed_runtime();
    let dir = temp_dir("tamper");
    store::persist(&rt, &dir).unwrap();

    let opened = store::open(&dir).expect("honest persist then open");
    assert_eq!(opened.fold().fingerprint(), rt.fold().fingerprint());
    opened.replay_check().unwrap();

    let snap_path = dir.join("snapshot.json");
    let mut snap: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&snap_path).unwrap()).unwrap();
    let facts = snap
        .get_mut("fold")
        .and_then(|f| f.get_mut("facts"))
        .and_then(|f| f.as_array_mut())
        .expect("fold.facts");
    assert!(
        !facts.is_empty(),
        "seeded snapshot must have facts to pop"
    );
    facts.pop();
    fs::write(&snap_path, serde_json::to_vec_pretty(&snap).unwrap()).unwrap();

    let err = match store::open(&dir) {
        Ok(_) => panic!("tampered snapshot must not yield a Runtime"),
        Err(e) => e,
    };
    assert_eq!(err.kind(), ErrorKind::InvalidData);
    let msg = err.to_string();
    assert!(
        msg.contains("ReplayDivergenceError"),
        "open error must name ReplayDivergenceError, got {msg}"
    );

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn persist_replaces_events_jsonl_atomically() {
    let rt = seed_runtime();
    let dir = temp_dir("atomic");
    store::persist(&rt, &dir).unwrap();

    let jsonl = dir.join("events.jsonl");
    let seeded = read_jsonl_events(&jsonl);
    assert!(
        !seeded.is_empty(),
        "events.jsonl must contain JSON Event objects"
    );
    let dest_before = fs::read(&jsonl).unwrap();

    fs::remove_file(dir.join("events.wal")).unwrap();
    let opened = store::open(&dir).unwrap();
    assert_eq!(opened.fold().fingerprint(), rt.fold().fingerprint());
    opened.replay_check().unwrap();

    // Crash-before-rename: temp sibling may exist; dest bytes stay the seeded log.
    store::abort_events_jsonl_replace_before_rename(&dir, &seeded)
        .expect_err("failed temp write is not persist success");
    let dest_after = fs::read(&jsonl).unwrap();
    assert_eq!(
        dest_after, dest_before,
        "failed temp write must leave the previous complete jsonl"
    );
    assert_eq!(read_jsonl_events(&jsonl), seeded);

    let _ = fs::remove_dir_all(&dir);
}
