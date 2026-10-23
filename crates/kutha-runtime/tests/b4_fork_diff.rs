//! Phase 230 / M156 S01 — fact-set fork diff (b4).
//! `diff_folds` is a query; no Op. Plan 02 owns totality / equal-key / checkpoint GREEN.

use kutha_common::{Event, EventId, Op, TermId};
use kutha_runtime::{
    diff_folds, store, Fact, FactChange, FoldDiff, GraphFold, Runtime, RuntimeError,
};
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

mod common;

struct LimitGuard;

impl Drop for LimitGuard {
    fn drop(&mut self) {
        store::reset_test_segment_limits();
    }
}

fn tiny_limits() -> LimitGuard {
    store::set_test_segment_limits(Some(4096), Some(1));
    LimitGuard
}

fn copy_store(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).unwrap();
    for ent in fs::read_dir(src).unwrap() {
        let ent = ent.unwrap();
        let name = ent.file_name();
        if name == "kutha.lock" {
            continue;
        }
        let from = ent.path();
        let to = dst.join(&name);
        if from.is_dir() {
            copy_store(&from, &to);
        } else {
            fs::copy(&from, &to).unwrap();
        }
    }
}

fn first_checkpoint_index(events: &[Event]) -> usize {
    events
        .iter()
        .position(|e| matches!(e.op, Op::Checkpoint { .. }))
        .expect("Op::Checkpoint on current log")
}

fn emit_keyed(
    rt: &mut Runtime,
    subject: TermId,
    relation: TermId,
    object: TermId,
    valid_from: u64,
    valid_to: Option<u64>,
    key: &str,
) -> EventId {
    rt.emit(Op::Assert {
        subject,
        relation,
        object,
        valid_from,
        valid_to,
        claim: None,
        delivery_key: Some(key.into()),
        polarity: None,
    })
    .unwrap()
    .receipt
    .event_ids[0]
}

struct Counterfactual {
    live: Runtime,
    cut: usize,
    ab1: EventId,
    ab2: EventId,
    bc: EventId,
    ci: EventId,
}

fn counterfactual() -> Counterfactual {
    let mut live = Runtime::default();
    let rel = live.intern("relatedTo");
    let a = live.intern("A");
    let b = live.intern("B");
    let c = live.intern("C");
    let ab1 = emit_keyed(&mut live, a, rel, b, 2010, Some(2015), "ab-1");
    let ab2 = emit_keyed(&mut live, a, rel, b, 2015, None, "ab-2");
    let cut = live.log().len();
    let bc = emit_keyed(&mut live, b, rel, c, 2010, None, "bc-1");
    live.emit(Op::Retract { event_id: ab1 }).unwrap();
    live.emit(Op::CorrectInterval {
        event_id: ab2,
        object: c,
        patch_from: 2018,
        patch_to: Some(2020),
    })
    .unwrap();
    let ci = live
        .log()
        .iter()
        .filter_map(|e| match &e.op {
            Op::CorrectInterval { .. } => Some(e.id),
            _ => None,
        })
        .last()
        .expect("CorrectInterval event in log");
    Counterfactual {
        live,
        cut,
        ab1,
        ab2,
        bc,
        ci,
    }
}

fn vt_key(to: Option<u64>) -> u64 {
    to.unwrap_or(u64::MAX)
}

fn polarity_rank(p: Option<kutha_common::SupportPolarity>) -> u8 {
    match p {
        None => 0,
        Some(kutha_common::SupportPolarity::Positive) => 1,
        Some(kutha_common::SupportPolarity::Negative) => 2,
    }
}

fn delivery_key_ord(a: &Option<String>, b: &Option<String>) -> Ordering {
    match (a, b) {
        (None, None) => Ordering::Equal,
        (None, Some(_)) => Ordering::Less,
        (Some(_), None) => Ordering::Greater,
        (Some(x), Some(y)) => x.as_bytes().cmp(y.as_bytes()),
    }
}

fn canon_cmp(a: &Fact, b: &Fact) -> Ordering {
    a.event_id
        .cmp(&b.event_id)
        .then(a.valid_from.cmp(&b.valid_from))
        .then(vt_key(a.valid_to).cmp(&vt_key(b.valid_to)))
        .then(a.object().cmp(&b.object()))
        .then(a.relation.cmp(&b.relation))
        .then(a.subject.cmp(&b.subject))
        .then(a.claim_id.cmp(&b.claim_id))
        .then(delivery_key_ord(&a.delivery_key, &b.delivery_key))
        .then(polarity_rank(a.polarity).cmp(&polarity_rank(b.polarity)))
        .then(a.ingested_at.cmp(&b.ingested_at))
        .then(vt_key(a.invalidated_at).cmp(&vt_key(b.invalidated_at)))
}

fn facts_sorted(facts: &[Fact]) -> bool {
    facts
        .windows(2)
        .all(|w| canon_cmp(&w[0], &w[1]) != Ordering::Greater)
}

fn changes_sorted(changes: &[FactChange]) -> bool {
    changes.windows(2).all(|w| {
        let c = canon_cmp(&w[0].after, &w[1].after);
        if c != Ordering::Equal {
            c != Ordering::Greater
        } else {
            canon_cmp(&w[0].before, &w[1].before) != Ordering::Greater
        }
    })
}

fn post_cut_mint_ids(live: &Runtime, cut: usize) -> BTreeSet<EventId> {
    live.log().as_slice()[cut..]
        .iter()
        .filter(|e| {
            matches!(
                e.op,
                Op::Assert { .. } | Op::CorrectInterval { .. } | Op::Behavior { .. }
            )
        })
        .map(|e| e.id)
        .collect()
}

fn apply_diff(rows_a: &[Fact], _diff: &FoldDiff) -> Vec<Fact> {
    rows_a.to_vec()
}

fn normalize(rows: &[Fact]) -> Vec<Fact> {
    let mut v = rows.to_vec();
    for f in &mut v {
        f.seq = 0;
    }
    v.sort_by(canon_cmp);
    v
}

#[test]
fn diff_folds_aa_is_empty() {
    let rt = Runtime::default();
    let rel = rt.fold();
    let d = diff_folds(rel, rel);
    assert!(d.added.is_empty());
    assert!(d.removed.is_empty());
    assert!(d.changed.is_empty());
    assert_eq!(rel.diff(rel), d);
}

#[test]
fn diff_folds_mirror_symmetry() {
    let fx = counterfactual();
    let fork = fx.live.fork_at(fx.cut).unwrap();
    let ab = diff_folds(fork.fold(), fx.live.fold());
    let ba = diff_folds(fx.live.fold(), fork.fold());
    assert_eq!(ab.added, ba.removed);
    assert_eq!(ab.removed, ba.added);
    assert_eq!(ab.changed.len(), ba.changed.len());
    for (c1, c2) in ab.changed.iter().zip(ba.changed.iter()) {
        assert_eq!(c1.before, c2.after);
        assert_eq!(c1.after, c2.before);
    }
    assert!(
        !ab.added.is_empty() || !ab.removed.is_empty() || !ab.changed.is_empty(),
        "distinct folds must produce a non-empty mirror pair"
    );
}

#[test]
fn fork_diff_exact_on_counterfactual_fixture() {
    let fx = counterfactual();
    let fork = fx.live.fork_at(fx.cut).unwrap();
    let diff = diff_folds(fork.fold(), fx.live.fold());
    assert!(facts_sorted(&diff.added), "added must be canonically sorted");
    assert!(facts_sorted(&diff.removed), "removed must be canonically sorted");
    assert!(
        changes_sorted(&diff.changed),
        "changed must be canonically sorted"
    );
    assert!(diff.removed.is_empty(), "losers stay; retract/CI do not drop rows");
    let mint = post_cut_mint_ids(&fx.live, fx.cut);
    assert!(
        mint.contains(&fx.bc) && mint.contains(&fx.ci),
        "post-cut Assert and CorrectInterval must mint"
    );
    assert!(
        diff.added.iter().any(|f| f.event_id == fx.bc),
        "post-cut B-C must be added"
    );
    assert!(
        diff.added.iter().any(|f| f.event_id == fx.ci),
        "CI residuals must be added"
    );
    assert!(
        diff.changed.iter().any(|c| {
            c.before.event_id == fx.ab1
                && c.before.valid_from == 2010
                && c.before.invalidated_at.is_none()
                && c.after.event_id == fx.ab1
                && c.after.invalidated_at.is_some()
        }),
        "retracted first A-B must be changed"
    );
    assert!(
        diff.changed.iter().any(|c| {
            c.before.event_id == fx.ab2
                && c.before.invalidated_at.is_none()
                && c.after.event_id == fx.ab2
                && c.after.invalidated_at.is_some()
        }),
        "CI-invalidated second A-B must be changed"
    );
    for c in &diff.changed {
        assert_eq!(c.before.event_id, c.after.event_id);
        assert_eq!(c.before.valid_from, c.after.valid_from);
        assert_eq!(c.before.valid_to, c.after.valid_to);
    }
}

#[test]
fn diff_fold_vs_fork_at_matches_post_cut_mints() {
    let fx = counterfactual();
    let fork = fx.live.fork_at(fx.cut).unwrap();
    let diff = diff_folds(fork.fold(), fx.live.fold());
    let mint = post_cut_mint_ids(&fx.live, fx.cut);
    let mut minted: Vec<Fact> = fx
        .live
        .fold()
        .facts()
        .iter()
        .filter(|f| mint.contains(&f.event_id))
        .cloned()
        .collect();
    minted.sort_by(canon_cmp);
    let mut added = diff.added.clone();
    added.sort_by(canon_cmp);
    assert_eq!(added, minted, "added must be exactly post-cut minted facts");
}

#[test]
fn apply_diff_reproduces_other_side_modulo_seq() {
    let fx = counterfactual();
    let fork = fx.live.fork_at(fx.cut).unwrap();
    let diff = diff_folds(fork.fold(), fx.live.fold());
    let got = apply_diff(fork.fold().facts(), &diff);
    assert_eq!(
        normalize(&got),
        normalize(fx.live.fold().facts()),
        "identity helper leaves the left side; totality stays RED until plan 02"
    );
}

#[test]
fn diff_handles_equal_keys_ci_residuals_and_multipath_heads() {
    let e1 = Event {
        id: EventId::nil(),
        op: Op::Assert {
            subject: 1,
            relation: 2,
            object: 10,
            valid_from: 2010,
            valid_to: Some(2020),
            claim: None,
            delivery_key: None,
            polarity: None,
        },
        ingested_at: 1,
        object_ids: vec![1, 2, 10],
    };
    let e2 = Event {
        id: EventId::nil(),
        op: Op::Assert {
            subject: 1,
            relation: 2,
            object: 11,
            valid_from: 2010,
            valid_to: Some(2020),
            claim: None,
            delivery_key: None,
            polarity: None,
        },
        ingested_at: 2,
        object_ids: vec![1, 2, 11],
    };
    let multi = GraphFold::replay(&[e1, e2]);
    let empty = GraphFold::default();
    let d = diff_folds(&empty, &multi);
    assert_eq!(
        d.added.len(),
        2,
        "equal (event_id, window) heads must rank as two added rows"
    );
    assert_eq!(d.added[0].event_id, EventId::nil());
    assert_eq!(d.added[1].event_id, EventId::nil());
    assert_ne!(d.added[0].object(), d.added[1].object());
    assert!(facts_sorted(&d.added));

    let fx = counterfactual();
    let fork = fx.live.fork_at(fx.cut).unwrap();
    let diff = diff_folds(fork.fold(), fx.live.fold());
    let ci_added: Vec<_> = diff
        .added
        .iter()
        .filter(|f| f.event_id == fx.ci)
        .cloned()
        .collect();
    assert!(
        ci_added.len() >= 2,
        "CI residuals share event_id; identity is not event_id alone"
    );
    let windows: BTreeSet<_> = ci_added
        .iter()
        .map(|f| (f.valid_from, vt_key(f.valid_to)))
        .collect();
    assert_eq!(windows.len(), ci_added.len(), "CI residuals have distinct windows");
}

#[test]
fn diff_of_checkpointed_fork_matches_plain_twin() {
    let _g = tiny_limits();
    let plain_dir = common::PersistScratch::new("b4-diff-ckpt-plain");
    let twin_dir = common::PersistScratch::new("b4-diff-ckpt-twin");
    let mut rt = Runtime::default();
    let rel = rt.intern("relatedTo");
    let a = rt.intern("A");
    let _b = rt.intern("B");
    for i in 0..8 {
        let o = rt.intern(&format!("o{i}"));
        emit_keyed(&mut rt, a, rel, o, 2010, Some(2020), &format!("dk-{i}"));
    }
    let extra = rt.intern("extra");
    emit_keyed(&mut rt, a, rel, extra, 2010, None, "dk-extra");
    store::persist(&rt, &plain_dir).unwrap();
    copy_store(&plain_dir, &twin_dir);
    store::checkpoint_sealed_prefix(&twin_dir).unwrap();
    let plain = store::open(&plain_dir).unwrap();
    let ckpt = store::open(&twin_dir).unwrap();
    let events = ckpt.log().as_slice();
    let c = first_checkpoint_index(events);
    let len = ckpt.log().len();
    for n in 1..=c {
        let err = match ckpt.fork_at(n) {
            Err(e) => e,
            Ok(_) => panic!("interior cut must be ForkCutUnavailable"),
        };
        assert!(
            matches!(
                err,
                RuntimeError::ForkCutUnavailable { cut, log_len } if cut == n && log_len == len
            ),
            "{err:?}"
        );
        assert_eq!(
            err.to_string(),
            format!("fork cut {n} unavailable (log_len {len})")
        );
    }
    let d_ckpt = diff_folds(ckpt.fork_at(0).unwrap().fold(), ckpt.fold());
    let d_plain = diff_folds(plain.fork_at(0).unwrap().fold(), plain.fold());
    assert!(
        !d_ckpt.added.is_empty(),
        "empty fork vs checkpointed live must add facts"
    );
    assert_eq!(
        normalize(&d_ckpt.added),
        normalize(&d_plain.added),
        "checkpointed fork_at(0) diff must match the un-checkpointed twin"
    );
}
