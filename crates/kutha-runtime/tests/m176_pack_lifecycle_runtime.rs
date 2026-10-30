//! M176 phase 260: replay-derived C1 lifecycle lease and graph isolation.

use kutha_common::Op;
use kutha_runtime::{PackLifecycleState, Runtime, derive_pack_lifecycle, store};

fn build_runtime() -> Runtime {
    let mut rt = Runtime::default();
    rt.emit_pack_install("core", "Core Pack", "1.0.0")
        .expect("install");
    rt.emit_pack_activate("core", "1.0.0").expect("activate");
    rt
}

#[test]
fn lifecycle_state_is_replay_derived_and_graph_neutral() {
    let mut rt = build_runtime();
    let fp_before = rt.fold().fingerprint();
    let facts_before = rt.fold().facts().len();

    let lifecycle = rt.pack_lifecycle();
    assert_eq!(lifecycle.len(), 1);
    assert_eq!(lifecycle[0].pack_id, "core");
    assert_eq!(lifecycle[0].state, PackLifecycleState::Active);
    assert_eq!(lifecycle[0].version, "1.0.0");
    assert_eq!(derive_pack_lifecycle(rt.log().as_slice()), lifecycle);

    rt.emit_pack_retire("core", "1.0.0").expect("retire");
    let lifecycle = rt.pack_lifecycle();
    assert_eq!(lifecycle[0].state, PackLifecycleState::Retired);
    assert_eq!(rt.fold().facts().len(), facts_before);
    assert_eq!(rt.fold().fingerprint(), fp_before);
}

#[test]
fn lifecycle_state_survives_persist_open() {
    let rt = build_runtime();
    let dir = std::path::PathBuf::from("tmp/m176-lifecycle-persist");
    std::fs::remove_dir_all(&dir).ok();
    rt.persist(&dir).expect("persist");
    let opened = Runtime::open(&dir).expect("open");
    assert_eq!(opened.pack_lifecycle(), rt.pack_lifecycle());
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn lifecycle_fork_cut_preserves_prefix_state() {
    let mut rt = build_runtime();
    let before_fork = rt.pack_lifecycle();
    let fork = rt.fork_at(rt.log().len()).expect("fork at log end");
    assert_eq!(fork.pack_lifecycle(), before_fork);

    rt.emit_pack_retire("core", "1.0.0").expect("retire");
    let cut = rt.log().len() - 1;
    let fork = rt.fork_at(cut).expect("fork before retire");
    assert_eq!(
        fork.pack_lifecycle().last().map(|p| p.state),
        Some(PackLifecycleState::Active)
    );
}

#[test]
fn lifecycle_query_ignores_product_graph_events() {
    let mut rt = build_runtime();
    let article = rt.intern("article");
    let relation = rt.intern("relatedTo");
    let object = rt.intern("value");
    rt.emit(Op::Assert {
        subject: article,
        relation,
        object,
        valid_from: 2010,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .expect("product assert");
    let lifecycle = rt.pack_lifecycle();
    assert_eq!(lifecycle.len(), 1);
    assert_eq!(lifecycle[0].state, PackLifecycleState::Active);
}

#[test]
fn lifecycle_state_survives_checkpoint_sealed_prefix() {
    struct Reset;
    impl Drop for Reset {
        fn drop(&mut self) {
            store::reset_test_segment_limits();
        }
    }
    let _reset = Reset;
    store::set_test_segment_limits(Some(4096), Some(2));

    let mut rt = build_runtime();
    let dir = std::path::PathBuf::from("tmp/m176-lifecycle-checkpoint");
    std::fs::remove_dir_all(&dir).ok();
    rt.persist(&dir).expect("persist lifecycle prefix");
    // Force the Pack* events into a sealed prefix: append product events and
    // persist again, so the checkpoint boundary includes the lifecycle events.
    let article = rt.intern("article");
    let relation = rt.intern("relatedTo");
    let object = rt.intern("value");
    for _ in 0..6 {
        rt.emit(Op::Assert {
            subject: article,
            relation,
            object,
            valid_from: 2010,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .expect("product assert");
    }
    rt.persist(&dir)
        .expect("persist with sealed lifecycle prefix");
    store::checkpoint_sealed_prefix(&dir).expect("checkpoint");
    let opened = Runtime::open(&dir).expect("open");
    assert_eq!(
        opened.pack_lifecycle().last().map(|p| p.state),
        Some(PackLifecycleState::Active)
    );
    std::fs::remove_dir_all(&dir).ok();
}
