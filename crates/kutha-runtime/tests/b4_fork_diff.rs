//! Phase 230 / M156 S01 — fact-set fork diff (b4).
//! `diff_folds` is a query; no Op. Plan 02 owns totality / equal-key / checkpoint GREEN.

use kutha_common::{
    Event, EventId, Op, TermDictionary, TermId, ValidTime, WallTime,
};
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

/// Eleven-field identity (every Fact field except fold-local `seq`).
fn same_except_seq(a: &Fact, b: &Fact) -> bool {
    a.subject == b.subject
        && a.relation == b.relation
        && a.object() == b.object()
        && a.valid_from == b.valid_from
        && a.valid_to == b.valid_to
        && a.ingested_at == b.ingested_at
        && a.invalidated_at == b.invalidated_at
        && a.claim_id == b.claim_id
        && a.event_id == b.event_id
        && a.delivery_key == b.delivery_key
        && a.polarity == b.polarity
}

/// Reconstruct `b` from `a`'s rows plus `diff_folds(a,b)`. Does not read `b`.
fn apply_diff(rows_a: &[Fact], diff: &FoldDiff) -> Vec<Fact> {
    let mut out = rows_a.to_vec();
    for rem in &diff.removed {
        let i = out
            .iter()
            .position(|f| same_except_seq(f, rem))
            .unwrap_or_else(|| panic!("removed row not in a (diff hole)"));
        out.remove(i);
    }
    for ch in &diff.changed {
        let i = out
            .iter()
            .position(|f| same_except_seq(f, &ch.before))
            .unwrap_or_else(|| panic!("changed.before not in a (diff hole)"));
        out[i] = ch.after.clone();
    }
    out.extend(diff.added.iter().cloned());
    out
}

fn normalize(rows: &[Fact]) -> Vec<Fact> {
    let mut v = rows.to_vec();
    for f in &mut v {
        f.seq = 0;
    }
    v.sort_by(canon_cmp);
    v
}

fn xorshift32(state: &mut u32) -> u32 {
    let mut x = *state;
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
    *state = x;
    x
}

fn shuffle_facts(rows: &mut [Fact], seed: u32) {
    let mut s = seed | 1;
    for i in (1..rows.len()).rev() {
        let j = (xorshift32(&mut s) as usize) % (i + 1);
        rows.swap(i, j);
    }
}

fn product_rel(dict: Option<&TermDictionary>, rel: TermId) -> bool {
    match dict {
        None => true,
        Some(d) => match d.lookup(rel) {
            Some(name) => !name.starts_with("kutha:"),
            None => true,
        },
    }
}

fn product_as_of_from_rows(
    rows: &[Fact],
    dict: Option<&TermDictionary>,
    vt: ValidTime,
) -> Vec<(TermId, TermId, TermId)> {
    let mut out: Vec<(TermId, TermId, TermId)> = rows
        .iter()
        .filter(|f| f.is_live_at(u64::MAX, vt) && product_rel(dict, f.relation))
        .map(|f| (f.subject, f.relation, f.object()))
        .collect();
    out.sort_unstable();
    out
}

fn product_known_from_rows(
    rows: &[Fact],
    dict: Option<&TermDictionary>,
    tt: u64,
) -> Vec<(TermId, TermId, TermId)> {
    let mut out: Vec<(TermId, TermId, TermId)> = rows
        .iter()
        .filter(|f| f.is_known_at(tt) && product_rel(dict, f.relation))
        .map(|f| (f.subject, f.relation, f.object()))
        .collect();
    out.sort_unstable();
    out
}

fn sorted_triples(mut t: Vec<(TermId, TermId, TermId)>) -> Vec<(TermId, TermId, TermId)> {
    t.sort_unstable();
    t
}

fn assert_clock_oracles(applied: &[Fact], other: &GraphFold, dict: Option<&TermDictionary>) {
    for year in [2011_u64, 2015, 2019] {
        let vt = ValidTime::from(year);
        assert_eq!(
            product_as_of_from_rows(applied, dict, vt),
            sorted_triples(other.as_of(vt)),
            "as_of {year} from applied rows must match the other fold"
        );
    }
    for wall_u in [2014_u64, 2020] {
        let wall = WallTime::from(wall_u);
        let rebuilt = other
            .knowledge_tt_at(wall)
            .map(|tt| product_known_from_rows(applied, dict, tt));
        let expect = other.as_known_on(wall).map(sorted_triples);
        assert_eq!(
            rebuilt, expect,
            "as_known_on {wall_u} from applied rows must match the other fold"
        );
    }
}

fn assert_mirror(ab: &FoldDiff, ba: &FoldDiff) {
    assert_eq!(ab.added, ba.removed);
    assert_eq!(ab.removed, ba.added);
    assert_eq!(ab.changed.len(), ba.changed.len());
    for (c1, c2) in ab.changed.iter().zip(ba.changed.iter()) {
        assert_eq!(c1.before, c2.after);
        assert_eq!(c1.after, c2.before);
    }
}

fn assert_totality(a: &GraphFold, b: &GraphFold, dict: Option<&TermDictionary>) {
    let aa = diff_folds(a, a);
    assert!(
        aa.added.is_empty() && aa.removed.is_empty() && aa.changed.is_empty(),
        "diff(a,a) must be empty"
    );
    let diff = diff_folds(a, b);
    let ba = diff_folds(b, a);
    assert_mirror(&diff, &ba);
    let got = apply_diff(a.facts(), &diff);
    assert_eq!(
        normalize(&got),
        normalize(b.facts()),
        "normalize(apply_diff(a, diff_folds(a,b))) must equal normalize(b) modulo seq"
    );
    let mut shuffled = a.facts().to_vec();
    shuffle_facts(&mut shuffled, 0xC0FF_EE42);
    assert_eq!(
        normalize(&shuffled),
        normalize(a.facts()),
        "canonical rank must not depend on insertion order"
    );
    assert_clock_oracles(&got, b, dict);
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
    let mut fx = counterfactual();
    fx.live.emit_tt_anchor(2014).unwrap();
    fx.live.emit_tt_anchor(2020).unwrap();
    let fork = fx.live.fork_at(fx.cut).unwrap();
    let dict = Some(fx.live.dictionary());
    assert_totality(fork.fold(), fx.live.fold(), dict);
    assert_totality(fx.live.fold(), fork.fold(), dict);

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
    let multi = GraphFold::replay(&[e1.clone(), e2.clone()]);
    let empty = GraphFold::default();
    assert_totality(&empty, &multi, None);
    assert_totality(&multi, &empty, None);
    let multi_rev = GraphFold::replay(&[e2, e1]);
    assert_totality(&empty, &multi_rev, None);
    assert_eq!(
        normalize(multi.facts()),
        normalize(multi_rev.facts()),
        "equal-key rank must not depend on replay insertion order"
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
    let multi = GraphFold::replay(&[e1.clone(), e2.clone()]);
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
    let multi_rev = GraphFold::replay(&[e2, e1]);
    assert_eq!(
        normalize(&diff_folds(&empty, &multi).added),
        normalize(&diff_folds(&empty, &multi_rev).added),
        "multipath heads rank independently of replay order"
    );
    let mut shuffled = multi.facts().to_vec();
    shuffle_facts(&mut shuffled, 0xA5A5_5A5A);
    assert_eq!(normalize(&shuffled), normalize(multi.facts()));

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
    assert_totality(fork.fold(), fx.live.fold(), Some(fx.live.dictionary()));
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
