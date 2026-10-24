//! Phase 232 / M158 S01 — fork then query (b6).
//! `Runtime::diff` is `diff_folds` of two folds. Scan is the live product bag.

use kutha_common::{Event, EventId, Op, TermId, TransactionTime, ValidTime};
use kutha_runtime::{
    Fact, FactChange, FoldDiff, QueryIr, QueryRows, Runtime, RuntimeError, diff_folds, execute,
    store,
};
use std::cmp::Ordering;
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

/// Compacted `c+1` is the first tail EventId; the twin cut is that id's offset.
fn twin_prefix_len_for_checkpointed(ckp: &Runtime, twin: &Runtime) -> usize {
    let events = ckp.log().as_slice();
    let c = first_checkpoint_index(events);
    match events.get(c + 1) {
        Some(first_tail) => twin
            .log()
            .as_slice()
            .iter()
            .position(|e| e.id == first_tail.id)
            .expect("first tail event must exist on the copied twin log"),
        None => twin.log().len(),
    }
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

fn scan(vt: u64) -> QueryIr {
    QueryIr::Scan {
        tt: TransactionTime::MAX,
        vt: ValidTime::from(vt),
        limit: 0,
    }
}

fn triples(rows: QueryRows) -> Vec<(TermId, TermId, TermId)> {
    match rows {
        QueryRows::Triples(v) => v,
        other => panic!("expected Triples, got {other:?}"),
    }
}

fn sorted_as_of(rt: &Runtime, vt: u64) -> Vec<(TermId, TermId, TermId)> {
    let mut t = rt.as_of(ValidTime::from(vt));
    t.sort_unstable();
    t
}

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

fn opt_u64_key(v: Option<u64>) -> u64 {
    v.unwrap_or(u64::MAX)
}

fn canon_cmp(a: &Fact, b: &Fact) -> Ordering {
    a.event_id
        .cmp(&b.event_id)
        .then(a.valid_from.cmp(&b.valid_from))
        .then(opt_u64_key(a.valid_to).cmp(&opt_u64_key(b.valid_to)))
        .then(a.object().cmp(&b.object()))
        .then(a.relation.cmp(&b.relation))
        .then(a.subject.cmp(&b.subject))
        .then(a.claim_id.cmp(&b.claim_id))
        .then(a.ingested_at.cmp(&b.ingested_at))
        .then(opt_u64_key(a.invalidated_at).cmp(&opt_u64_key(b.invalidated_at)))
}

fn normalize(rows: &[Fact]) -> Vec<Fact> {
    let mut v = rows.to_vec();
    for f in &mut v {
        f.seq = 0;
    }
    v.sort_by(canon_cmp);
    v
}

fn normalize_diff(diff: &FoldDiff) -> FoldDiff {
    let mut changed: Vec<FactChange> = diff
        .changed
        .iter()
        .map(|c| {
            let mut before = c.before.clone();
            let mut after = c.after.clone();
            before.seq = 0;
            after.seq = 0;
            FactChange { before, after }
        })
        .collect();
    changed.sort_by(|x, y| {
        let c = canon_cmp(&x.after, &y.after);
        if c != Ordering::Equal {
            c
        } else {
            canon_cmp(&x.before, &y.before)
        }
    });
    FoldDiff {
        added: normalize(&diff.added),
        removed: normalize(&diff.removed),
        changed,
    }
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

#[test]
fn runtime_diff_explains_scan_on_fork() {
    let mut live = Runtime::default();
    let rel = live.intern("relatedTo");
    let a = live.intern("A");
    let b = live.intern("B");
    let c = live.intern("C");
    let d = live.intern("D");
    let e = live.intern("E");
    let ab1 = emit_keyed(&mut live, a, rel, b, 2010, Some(2015), "ab-1");
    let _ab2 = emit_keyed(&mut live, a, rel, b, 2015, None, "ab-2");
    let _keep = emit_keyed(&mut live, d, rel, e, 2010, None, "de-1");
    let cut = live.log().len();
    live.emit(Op::Retract { event_id: ab1 }).unwrap();
    let _bc = emit_keyed(&mut live, b, rel, c, 2010, None, "bc-1");

    let fork = live.fork_at(cut).unwrap();
    let got = fork.diff(&live);
    let expect = diff_folds(fork.fold(), live.fold());
    assert_eq!(
        got, expect,
        "Runtime::diff must equal diff_folds of the two folds"
    );
    assert!(
        !got.changed.is_empty(),
        "Retract must show as changed (dead row stays in facts())"
    );
    assert!(
        !got.added.is_empty(),
        "post-cut Assert must be added (not the Scan delta)"
    );
    assert!(
        got.removed.is_empty(),
        "Retract does not drop the facts() row"
    );
    assert!(
        got.changed.iter().any(|ch| {
            ch.before.event_id == ab1
                && ch.before.invalidated_at.is_none()
                && ch.after.event_id == ab1
                && ch.after.invalidated_at.is_some()
        }),
        "retracted A-B must be changed"
    );

    let rebuilt = apply_diff(fork.fold().facts(), &got);
    assert_eq!(
        normalize(&rebuilt),
        normalize(live.fold().facts()),
        "apply_diff must reconstruct live facts modulo seq"
    );

    let dead = (a, rel, b);
    let keep = (d, rel, e);
    let extra = (b, rel, c);
    let mut nonempty = false;
    for vt in [2011_u64, 2019] {
        let q = scan(vt);
        let fork_bag = triples(execute(&fork, &q).unwrap());
        let live_bag = triples(execute(&live, &q).unwrap());
        assert_eq!(
            fork_bag,
            sorted_as_of(&fork, vt),
            "Scan on fork must match live product triples at {vt}"
        );
        assert_eq!(
            live_bag,
            sorted_as_of(&live, vt),
            "Scan on live must match live product triples at {vt}"
        );
        nonempty |= !fork_bag.is_empty() || !live_bag.is_empty();
        assert!(
            fork_bag.contains(&keep) && live_bag.contains(&keep),
            "surviving pre-cut D-E must stay in Scan at {vt}"
        );
        if vt == 2011 {
            assert!(fork_bag.contains(&dead), "fork Scan at 2011 still has A-B");
            assert!(
                !live_bag.contains(&dead),
                "live Scan drops the retracted A-B even though removed is empty"
            );
            assert!(
                !fork_bag.contains(&extra) && live_bag.contains(&extra),
                "post-cut B-C is Scan-live on live only"
            );
        }
    }
    assert!(nonempty, "Scan bags must be nonempty on at least one side");

    let _g = tiny_limits();
    let plain_dir = common::PersistScratch::new("b6-fork-q-plain");
    let ckpt_dir = common::PersistScratch::new("b6-fork-q-ckpt");
    let mut rt = Runtime::default();
    let rel = rt.intern("relatedTo");
    let subj = rt.intern("A");
    for i in 0..8 {
        let o = rt.intern(&format!("o{i}"));
        emit_keyed(&mut rt, subj, rel, o, 2010, Some(2020), &format!("dk-{i}"));
    }
    rt.emit_tt_anchor(2014).unwrap();
    store::persist(&rt, &plain_dir).unwrap();
    copy_store(&plain_dir, &ckpt_dir);
    store::checkpoint_sealed_prefix(&ckpt_dir).unwrap();
    let plain = store::open(&plain_dir).unwrap();
    let ckpt = store::open(&ckpt_dir).unwrap();
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
    }

    let ckpt0 = ckpt.fork_at(0).unwrap();
    let plain0 = plain.fork_at(0).unwrap();
    let d0_ckpt = ckpt0.diff(&ckpt);
    let d0_plain = plain0.diff(&plain);
    assert_eq!(d0_ckpt, diff_folds(ckpt0.fold(), ckpt.fold()));
    assert!(
        !d0_ckpt.added.is_empty(),
        "empty fork vs checkpointed live must add facts"
    );
    assert_eq!(
        normalize_diff(&d0_ckpt),
        normalize_diff(&d0_plain),
        "checkpointed fork_at(0) diff must match the un-checkpointed twin"
    );

    let n = c + 1;
    let ckpt_fork = ckpt
        .fork_at(n)
        .unwrap_or_else(|e| panic!("legal checkpointed fork_at({n}) must succeed: {e}"));
    let plain_n = twin_prefix_len_for_checkpointed(&ckpt, &plain);
    let plain_fork = plain
        .fork_at(plain_n)
        .unwrap_or_else(|e| panic!("twin fork_at({plain_n}) must succeed: {e}"));
    let d_ckpt = ckpt_fork.diff(&ckpt);
    let d_plain = plain_fork.diff(&plain);
    assert_eq!(d_ckpt, diff_folds(ckpt_fork.fold(), ckpt.fold()));
    assert_eq!(
        normalize_diff(&d_ckpt),
        normalize_diff(&d_plain),
        "checkpointed fork_at({n}) must diff like twin fork_at({plain_n})"
    );
}
