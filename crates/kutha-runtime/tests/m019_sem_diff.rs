//! SEM-csr-fold-diff (DIFF-01/02): seeded logs vs live_at/as_of/as_known_on,
//! typed/untyped CSR at (tt,vt) cuts, persist+open rebuild
//! (legacy, segmented+rollover, migrate, export_jsonl). BehaviorFree and WithHorn.
//! Default ≥64 seeds; `KUTHA_SEEDS=<n>` may raise. Persist/tamper/export use
//! `persist_seed_count()` = max(12, n/5). No wall-clock oracle.

mod common;

use common::sem::{
    MixCoverage, MixStyle, PersistLayout, assert_csr_matches_fold,
    assert_export_preserves_event_order, assert_same_stamps_shrunk, assert_seed_budget,
    assert_views_stable, fail_shrunk, fold_stamps, ingest_mix, open_export_imported, persist_dir,
    persist_schedule, persist_seed_count, persist_with_layout, replay_check_or_shrink,
    replay_runtime, rm_dir, runtime_stamps, scan_mix_coverage, seeded_persist_runtime,
    seeded_runtime, seeds, vt_cuts_for,
};
use kutha_runtime::{FoldIndexMode, GraphFold, Runtime, store};

fn diff_one(seed: u32, style: MixStyle, case_base: usize) {
    let (rt, meta) = seeded_runtime(seed, style, FoldIndexMode::Eager);
    assert_views_stable(&rt, &meta, seed, case_base);
    let replayed = GraphFold::replay(rt.log().as_slice());
    if runtime_stamps(&rt) != fold_stamps(&replayed, &rt) {
        let events = rt.log().as_slice().to_vec();
        fail_shrunk(
            seed,
            case_base + 1,
            &events,
            |prefix| {
                let r = replay_runtime(&rt, prefix);
                let g = GraphFold::replay(prefix);
                runtime_stamps(&r) != fold_stamps(&g, &r)
            },
            "GraphFold::replay stamps must equal live fold",
        );
    }
    replay_check_or_shrink(&rt, seed, case_base + 2);
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

fn persist_roundtrip_one(
    seed: u32,
    style: MixStyle,
    layout: PersistLayout,
    case: usize,
    export: bool,
) {
    let (rt, meta) = seeded_persist_runtime(seed, style);
    assert_csr_matches_fold(
        &rt,
        u64::MAX,
        vt_cuts_for(meta.min_vf, meta.max_vf)[1],
        seed,
        case,
    );
    let dir = persist_dir("p");
    persist_with_layout(&rt, &dir, layout);
    let opened = store::open(&dir).unwrap_or_else(|e| {
        fail_shrunk(
            seed,
            case,
            rt.log().as_slice(),
            |prefix| common::sem::persist_stamps_disagree(&rt, prefix, layout, seed),
            &format!("persist+open failed: {e}"),
        );
    });
    assert_same_stamps_shrunk(
        &rt,
        &opened,
        seed,
        case,
        layout,
        "persist+open must match live stamps",
    );
    assert_csr_matches_fold(
        &opened,
        u64::MAX,
        vt_cuts_for(meta.min_vf, meta.max_vf)[1],
        seed,
        case + 1,
    );
    if export {
        let dest = persist_dir("exp");
        let imported = open_export_imported(&dir, &dest);
        assert_same_stamps_shrunk(
            &rt,
            &imported,
            seed,
            case + 2,
            layout,
            "export_jsonl-imported open must match live stamps",
        );
        assert_export_preserves_event_order(&dest, &rt, seed, case + 3);
        rm_dir(&dest);
    }
    rm_dir(&dir);
}

#[test]
fn csr_fold_diff_persist_rebuild_layouts() {
    assert_seed_budget();
    let mut saw_legacy = false;
    let mut saw_rollover = false;
    let mut saw_migrated = false;
    let mut saw_export = false;
    let mut saw_horn_seg = false;
    let mut saw_horn_mig = false;
    for (i, seed) in seeds().into_iter().take(persist_seed_count()).enumerate() {
        for (pass, case) in persist_schedule(i).into_iter().enumerate() {
            persist_roundtrip_one(
                seed,
                case.style,
                case.layout,
                i * 20 + pass * 4,
                case.export,
            );
            match case.layout {
                PersistLayout::Legacy => saw_legacy = true,
                PersistLayout::Rollover => saw_rollover = true,
                PersistLayout::Migrated => saw_migrated = true,
                PersistLayout::Segmented => {}
            }
            if case.export {
                saw_export = true;
            }
            if case.style == MixStyle::WithHorn && case.layout == PersistLayout::Segmented {
                saw_horn_seg = true;
            }
            if case.style == MixStyle::WithHorn && case.layout == PersistLayout::Migrated {
                saw_horn_mig = true;
            }
        }
    }
    assert!(
        saw_legacy && saw_rollover && saw_migrated && saw_export,
        "persist schedule must cover legacy, rollover, migrated, export-imported"
    );
    assert!(
        saw_horn_seg && saw_horn_mig,
        "persist schedule must put WithHorn through segmented and migrated"
    );
}

#[test]
fn persist_roundtrip_export_jsonl_imported() {
    assert_seed_budget();
    let mut n = 0usize;
    for (i, seed) in seeds().into_iter().take(persist_seed_count()).enumerate() {
        for (pass, case) in persist_schedule(i).into_iter().enumerate() {
            if !case.export {
                continue;
            }
            persist_roundtrip_one(
                seed,
                case.style,
                case.layout,
                1000 + i * 20 + pass * 4,
                true,
            );
            n += 1;
            if n >= 2 {
                break;
            }
        }
        if n >= 2 {
            break;
        }
    }
    assert!(n >= 1, "export-imported layout must run at least once");
}

#[test]
fn mix_generator_coverage_ops_and_inside_cuts() {
    assert_seed_budget();
    let mut free = MixCoverage::default();
    let mut horn = MixCoverage::default();
    for seed in seeds() {
        let mut rt = Runtime::with_index_mode(32, FoldIndexMode::Eager);
        let meta = ingest_mix(&mut rt, seed, MixStyle::BehaviorFree);
        let c = scan_mix_coverage(&rt, &meta, MixStyle::BehaviorFree);
        free.n += c.n;
        free.retract += c.retract;
        free.correct += c.correct;
        free.interval += c.interval;
        free.vt_inside += c.vt_inside;
        free.vt_total += c.vt_total;
        let mut rt = Runtime::with_index_mode(32, FoldIndexMode::Eager);
        let meta = ingest_mix(&mut rt, seed, MixStyle::WithHorn);
        let c = scan_mix_coverage(&rt, &meta, MixStyle::WithHorn);
        horn.n += c.n;
        horn.retract += c.retract;
        horn.correct += c.correct;
        horn.interval += c.interval;
        horn.horn += c.horn;
        horn.vt_inside += c.vt_inside;
        horn.vt_total += c.vt_total;
    }
    eprintln!(
        "SEM coverage BehaviorFree n={} Retract={}/{} Correct={}/{} CorrectInterval={}/{} VT-inside={}/{} ({:.0}%)",
        free.n,
        free.retract,
        free.n,
        free.correct,
        free.n,
        free.interval,
        free.n,
        free.vt_inside,
        free.vt_total,
        100.0 * free.vt_inside as f64 / free.vt_total.max(1) as f64
    );
    eprintln!(
        "SEM coverage WithHorn n={} Retract={}/{} Correct={}/{} CorrectInterval={}/{} Horn={}/{} VT-inside={}/{} ({:.0}%)",
        horn.n,
        horn.retract,
        horn.n,
        horn.correct,
        horn.n,
        horn.interval,
        horn.n,
        horn.horn,
        horn.n,
        horn.vt_inside,
        horn.vt_total,
        100.0 * horn.vt_inside as f64 / horn.vt_total.max(1) as f64
    );
    assert!(
        free.retract * 2 >= free.n,
        "Retract must appear in ≥50% BehaviorFree seeds"
    );
    assert!(
        free.correct * 2 >= free.n,
        "Correct must appear in ≥50% BehaviorFree seeds"
    );
    assert!(
        free.interval * 5 >= free.n * 2,
        "CorrectInterval must appear in ≥40% BehaviorFree seeds"
    );
    assert!(
        horn.horn * 2 >= horn.n,
        "Horn/inverse Behavior in ≥50% WithHorn seeds"
    );
    assert!(
        free.vt_inside * 5 >= free.vt_total * 2,
        "VT-inside cuts must be ≥40% (2009 is the sole explicit edge; two of four cuts are inside)"
    );
}
