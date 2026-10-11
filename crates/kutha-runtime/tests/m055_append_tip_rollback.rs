//! M055 S01 / Phase 122 — CE whole-repo #6.
//! Post-write append failure must not leave a tip that same-object retry
//! duplicates. Decision: rollback physical tip (keep Other + lock); poison
//! only after a committed rollover.

use kutha_common::{EventId, Op};
use kutha_runtime::{Runtime, store};
use std::collections::HashSet;
use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;

fn persist_dir(label: &str) -> PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("tmp")
        .join(format!(
            "m055-append-tip-{label}-{}-{}",
            std::process::id(),
            nanos
        ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn seed_runtime() -> Runtime {
    let mut rt = Runtime::default();
    let s = rt.intern("m055-s");
    let r = rt.intern("relatedTo");
    let o = rt.intern("m055-o");
    rt.emit(Op::Assert {
        subject: s,
        relation: r,
        object: o,
        valid_from: 2010,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    rt.emit_tt_anchor(2015).unwrap();
    rt
}

/// Graph EventIds only — segmented persist encodes Defines via stable_define,
/// so intern-time Define ids in `Runtime::log` are not the on-disk sequence.
fn graph_ids(rt: &Runtime) -> Vec<EventId> {
    rt.log()
        .iter()
        .filter(|e| !matches!(e.op, Op::Define { .. }))
        .map(|e| e.id)
        .collect()
}

fn assert_unique_ids(ids: &[EventId]) {
    let mut seen = HashSet::new();
    for id in ids {
        assert!(
            seen.insert(*id),
            "EventId {id} duplicated in persisted log"
        );
    }
}

fn assert_exact_graph_after_retry(rt: &Runtime, dir: &std::path::Path) {
    let want = graph_ids(rt);
    let after = store::open(dir).unwrap();
    let got = graph_ids(&after);
    assert_unique_ids(&got);
    assert_eq!(
        got, want,
        "retry must land each graph EventId once (exact sequence), not length-only growth"
    );
}

/// #6 kill: WalSync failpoint + retry must not mint duplicate EventIds.
#[test]
fn segmented_wal_sync_failpoint_retry_keeps_exact_event_id_sequence() {
    let mut rt = seed_runtime();
    let dir = persist_dir("wal-sync");
    store::persist(&rt, &dir).unwrap();
    assert!(
        dir.join("manifest.json").is_file(),
        "empty-dir persist must be segmented"
    );

    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    rt.emit_tt_anchor(2021).unwrap();

    let err = store::append_with_failpoint(&mut p, &rt, store::AppendStep::WalSync)
        .expect_err("failpoint must inject");
    assert_eq!(err.kind(), ErrorKind::Other);

    // Same object stays usable (m030); tip must be clean for an exact retry.
    p.append(&rt).expect("retry append after tip rollback");
    drop(p);

    assert_exact_graph_after_retry(&rt, &dir);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn segmented_wal_write_failpoint_retry_keeps_exact_event_id_sequence() {
    let mut rt = seed_runtime();
    let dir = persist_dir("wal-write");
    store::persist(&rt, &dir).unwrap();
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    rt.emit_tt_anchor(2022).unwrap();

    store::append_with_failpoint(&mut p, &rt, store::AppendStep::WalWrite)
        .expect_err("failpoint must inject");
    p.append(&rt).expect("retry append");
    drop(p);

    assert_exact_graph_after_retry(&rt, &dir);
    let _ = fs::remove_dir_all(&dir);
}
