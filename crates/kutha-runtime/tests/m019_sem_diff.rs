//! SEM-csr-fold-diff (DIFF-01/02): seeded logs vs live_at/as_of/as_known_on,
//! typed/untyped CSR at random (tt,vt) cuts, persist+open rebuild
//! (legacy, segmented+rollover, migrate).
//! Default ≥64 seeds; `KUTHA_SEEDS=<n>` may raise. No wall-clock oracle.

mod common;

use common::sem::{
    MixStyle, assert_same_stamps, assert_seed_budget, assert_views_stable, fail_seed,
    open_export_imported, persist_dir, persist_legacy, persist_migrated, persist_seed_count,
    persist_segmented, persist_segmented_rollover, replay_runtime, rm_dir, runtime_stamps,
    seeded_runtime, seeds, shrink_events,
};
use kutha_runtime::{FoldIndexMode, GraphFold, store};

fn diff_one(seed: u32, style: MixStyle, case_base: usize) {
    let (rt, meta) = seeded_runtime(seed, style, FoldIndexMode::Eager);
    assert_views_stable(&rt, meta.last_wall, seed, case_base);
    let replayed = GraphFold::replay(rt.log().as_slice());
    if runtime_stamps(&rt) != common::sem::fold_stamps(&replayed, &rt) {
        let events = rt.log().as_slice().to_vec();
        let shrunk = shrink_events(&events, |prefix| {
            let r = replay_runtime(&rt, prefix);
            let g = GraphFold::replay(prefix);
            runtime_stamps(&r) != common::sem::fold_stamps(&g, &r)
        });
        eprintln!("shrunk ops={}", common::sem::brief_ops(&shrunk));
        fail_seed(
            seed,
            case_base + 1,
            &shrunk,
            "GraphFold::replay stamps must equal live fold",
        );
    }
    rt.replay_check()
        .unwrap_or_else(|e| panic!("replay_check seed={seed}: {e}"));
}

#[test]
fn csr_fold_diff_behavior_free_agrees() {
    assert_seed_budget();
    for (case, seed) in seeds().into_iter().enumerate() {
        diff_one(seed, MixStyle::BehaviorFree, case * 10);
    }
}

#[test]
fn csr_fold_diff_with_horn_agrees() {
    assert_seed_budget();
    for (case, seed) in seeds().into_iter().enumerate() {
        diff_one(seed, MixStyle::WithHorn, 10_000 + case * 10);
    }
}

fn persist_roundtrip_one(seed: u32, style: MixStyle) {
    let (rt, meta) = seeded_runtime(seed, style, FoldIndexMode::Eager);
    assert_views_stable(&rt, meta.last_wall, seed, 0);

    let d_leg = persist_dir("leg");
    persist_legacy(&rt, &d_leg);
    let opened = store::open(&d_leg).unwrap();
    assert_same_stamps(
        &rt,
        &opened,
        seed,
        1,
        "legacy persist+open must match live stamps",
    );
    assert_views_stable(&opened, meta.last_wall, seed, 2);
    rm_dir(&d_leg);

    let d_seg = persist_dir("seg");
    persist_segmented(&rt, &d_seg);
    let opened = store::open(&d_seg).unwrap();
    assert_same_stamps(
        &rt,
        &opened,
        seed,
        3,
        "segmented persist+open must match live stamps",
    );
    rm_dir(&d_seg);

    let d_roll = persist_dir("roll");
    let sealed = persist_segmented_rollover(&rt, &d_roll);
    assert!(sealed >= 2);
    let opened = store::open(&d_roll).unwrap();
    assert_same_stamps(
        &rt,
        &opened,
        seed,
        4,
        "rollover segmented persist+open must match live stamps",
    );
    rm_dir(&d_roll);

    let d_mig = persist_dir("mig");
    persist_migrated(&rt, &d_mig);
    let opened = store::open(&d_mig).unwrap();
    assert_same_stamps(
        &rt,
        &opened,
        seed,
        5,
        "migrate persist+open must match live stamps",
    );
    rm_dir(&d_mig);
}

#[test]
fn csr_fold_diff_persist_rebuild_layouts() {
    assert_seed_budget();
    for seed in seeds().into_iter().take(persist_seed_count()) {
        persist_roundtrip_one(seed, MixStyle::BehaviorFree);
    }
}

#[test]
fn persist_roundtrip_export_jsonl_imported() {
    assert_seed_budget();
    for seed in seeds().into_iter().take(persist_seed_count()) {
        let (rt, _) = seeded_runtime(seed, MixStyle::BehaviorFree, FoldIndexMode::Eager);
        let src = persist_dir("exp-src");
        persist_segmented(&rt, &src);
        let dest = persist_dir("exp-dst");
        let opened = open_export_imported(&src, &dest);
        assert_same_stamps(
            &rt,
            &opened,
            seed,
            6,
            "export_jsonl-imported open must match live stamps",
        );
        rm_dir(&src);
        rm_dir(&dest);
    }
}
