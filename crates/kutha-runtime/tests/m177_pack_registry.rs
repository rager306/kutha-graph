//! C1b: replay-derived operator registry parity and graph neutrality.

use kutha_runtime::{PackLifecycleState, PackManifest, Runtime, store};
use std::sync::atomic::{AtomicU64, Ordering};

mod common;

/// Repo-local `tmp/` scratch with pid + nanos + seq (`m019_dur_a` idiom), so two
/// concurrent runs of this binary never share a store directory.
fn scratch(label: &str) -> common::PersistScratch {
    static N: AtomicU64 = AtomicU64::new(0);
    let seq = N.fetch_add(1, Ordering::Relaxed);
    common::PersistScratch::new(&format!("m177-{label}-{seq}"))
}

fn build_runtime() -> Runtime {
    let mut rt = Runtime::default();
    let manifest = PackManifest::new(
        "core",
        "Core Pack",
        "1.0.0",
        vec!["normalize".into(), "cite".into()],
    );
    rt.emit_pack_install_with_manifest("core", "Core Pack", "1.0.0", Some(manifest))
        .expect("install");
    rt.emit_pack_activate("core", "1.0.0").expect("activate");
    rt
}

#[test]
fn registry_is_replay_derived_and_lifecycle_aware() {
    let mut rt = build_runtime();
    let rows = rt.pack_operators().expect("registry");
    assert_eq!(rows.len(), 2);
    assert!(
        rows.iter()
            .all(|row| row.state == PackLifecycleState::Active)
    );
    assert_eq!(rows[0].operator, "normalize");
    assert_eq!(rows[1].operator, "cite");

    rt.emit_pack_retire("core", "1.0.0").expect("retire");
    let rows = rt.pack_operators().expect("registry");
    assert!(
        rows.iter()
            .all(|row| row.state == PackLifecycleState::Retired)
    );
}

#[test]
fn registry_survives_persist_open() {
    let rt = build_runtime();
    let dir = scratch("registry-persist");
    rt.persist(&dir).expect("persist");
    let opened = Runtime::open(&dir).expect("open");
    assert_eq!(
        opened.pack_operators().expect("registry"),
        rt.pack_operators().expect("registry")
    );
}

#[test]
fn registry_fork_cut_preserves_prefix_operators() {
    let mut rt = build_runtime();
    let before = rt.pack_operators().expect("registry");
    let fork = rt.fork_at(rt.log().len()).expect("fork");
    assert_eq!(fork.pack_operators().expect("registry"), before);

    rt.emit_pack_retire("core", "1.0.0").expect("retire");
    let cut = rt.log().len() - 1;
    let fork = rt.fork_at(cut).expect("fork before retire");
    assert!(
        fork.pack_operators()
            .expect("registry")
            .iter()
            .all(|row| row.state == PackLifecycleState::Active)
    );
}

#[test]
fn registry_survives_checkpoint_sealed_prefix() {
    struct Reset;
    impl Drop for Reset {
        fn drop(&mut self) {
            store::reset_test_segment_limits();
        }
    }
    let _reset = Reset;
    store::set_test_segment_limits(Some(4096), Some(2));

    let mut rt = build_runtime();
    let dir = scratch("registry-checkpoint");
    rt.persist(&dir).expect("persist manifest prefix");
    let article = rt.intern("article");
    let relation = rt.intern("relatedTo");
    let object = rt.intern("value");
    for _ in 0..6 {
        rt.emit(kutha_common::Op::Assert {
            subject: article,
            relation,
            object,
            valid_from: 2010,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .expect("assert");
    }
    rt.persist(&dir).expect("persist sealed prefix");
    store::checkpoint_sealed_prefix(&dir).expect("checkpoint");
    let opened = Runtime::open(&dir).expect("open");
    let rows = opened.pack_operators().expect("registry");
    assert_eq!(rows.len(), 2);
    assert!(
        rows.iter()
            .all(|row| row.state == PackLifecycleState::Active)
    );
}

#[test]
fn registry_does_not_execute_or_change_graph() {
    let rt = build_runtime();
    let before = rt.fold().fingerprint();
    let facts_before = rt.fold().facts().len();
    let rows = rt.pack_operators().expect("registry");
    assert_eq!(rows.len(), 2);
    assert_eq!(rt.fold().fingerprint(), before);
    assert_eq!(rt.fold().facts().len(), facts_before);
}
