//! Independent verifier kill-tests for M158 / phase 232 `Runtime::diff` glue.
//!
//! Derived from `fold_diff.rs` (`Runtime::diff` = `diff_folds` of the two
//! folds; identity ignores `seq`), `hydrate.rs` `fork_at` (interior of a
//! checkpointed prefix is `ForkCutUnavailable`), `query_ir.rs` (`execute` /
//! `prepare` Scan = sorted `live_at`; named hops fail closed), and ADR-061
//! (fact-set picture, not GED). In-memory only: no persist dirs.

use kutha_common::{Event, EventId, Op, TermId, TransactionTime, ValidTime};
use kutha_runtime::{
    FoldDiff, QueryError, QueryIr, QueryRows, Runtime, RuntimeError, diff_folds, execute, prepare,
    try_hops_named,
};
use serde_json::json;
use std::collections::BTreeMap;

fn tt_max() -> TransactionTime {
    TransactionTime::MAX
}

fn vt(y: u64) -> ValidTime {
    ValidTime::from(y)
}

fn scan_ir(vt_year: u64) -> QueryIr {
    QueryIr::Scan {
        tt: tt_max(),
        vt: vt(vt_year),
        limit: 0,
    }
}

fn triples(rows: QueryRows) -> Vec<(TermId, TermId, TermId)> {
    match rows {
        QueryRows::Triples(t) => t,
        other => panic!("expected Scan triples, got {other:?}"),
    }
}

fn scan(rt: &Runtime, vt_year: u64) -> Vec<(TermId, TermId, TermId)> {
    triples(execute(rt, &scan_ir(vt_year)).expect("Scan must succeed"))
}

fn sorted_live(rt: &Runtime, vt_year: u64) -> Vec<(TermId, TermId, TermId)> {
    let mut rows = rt.live_at(tt_max(), vt(vt_year));
    rows.sort_unstable();
    rows
}

fn emit_keyed(
    rt: &mut Runtime,
    subject: &str,
    relation: &str,
    object: &str,
    valid_from: u64,
    valid_to: Option<u64>,
    key: &str,
) -> EventId {
    let subject = rt.intern(subject);
    let relation = rt.intern(relation);
    let object = rt.intern(object);
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
    .expect("product Assert must admit")
    .receipt
    .event_ids[0]
}

fn assert_empty(diff: &FoldDiff, ctx: &str) {
    assert!(
        diff.added.is_empty() && diff.removed.is_empty() && diff.changed.is_empty(),
        "{ctx}: expected empty FoldDiff, got added={} removed={} changed={}",
        diff.added.len(),
        diff.removed.len(),
        diff.changed.len()
    );
}

fn assert_wrapper(a: &Runtime, b: &Runtime, ctx: &str) {
    let via_method = a.diff(b);
    let via_folds = diff_folds(a.fold(), b.fold());
    assert_eq!(
        via_method, via_folds,
        "{ctx}: Runtime::diff must equal diff_folds(self.fold(), other.fold()) and nothing else"
    );
    assert_eq!(
        a.fold().diff(b.fold()),
        via_folds,
        "{ctx}: GraphFold::diff must match the same script"
    );
}

fn fp(f: &kutha_runtime::Fact) -> String {
    format!(
        "{:?}|{}|{}|{}|{}|{:?}|{}|{:?}|{:?}|{:?}|{:?}",
        f.event_id,
        f.subject,
        f.relation,
        f.object(),
        f.valid_from,
        f.valid_to,
        f.ingested_at,
        f.invalidated_at,
        f.claim_id,
        f.delivery_key,
        f.polarity
    )
}

fn bag(rows: &[kutha_runtime::Fact]) -> BTreeMap<String, usize> {
    let mut m = BTreeMap::new();
    for f in rows {
        *m.entry(fp(f)).or_insert(0) += 1;
    }
    m
}

fn assert_mirror(a: &Runtime, b: &Runtime, ctx: &str) {
    let ab = a.diff(b);
    let ba = b.diff(a);
    assert_eq!(
        bag(&ab.added),
        bag(&ba.removed),
        "{ctx}: added vs reverse removed"
    );
    assert_eq!(
        bag(&ab.removed),
        bag(&ba.added),
        "{ctx}: removed vs reverse added"
    );
    let mut ab_ch = BTreeMap::new();
    for c in &ab.changed {
        *ab_ch
            .entry(format!("{}=>{}", fp(&c.before), fp(&c.after)))
            .or_insert(0) += 1;
    }
    let mut ba_ch = BTreeMap::new();
    for c in &ba.changed {
        *ba_ch
            .entry(format!("{}=>{}", fp(&c.after), fp(&c.before)))
            .or_insert(0) += 1;
    }
    assert_eq!(ab_ch, ba_ch, "{ctx}: changed not swapped");
}

fn bump_seqs(rt: &Runtime, delta: u64) -> Runtime {
    let mut snap = rt.snapshot();
    let mut v = serde_json::to_value(&snap.fold).expect("serialize fold");
    let facts = v
        .get_mut("facts")
        .and_then(|f| f.as_array_mut())
        .expect("facts array");
    for (i, f) in facts.iter_mut().enumerate() {
        let seq = f.get("seq").and_then(|s| s.as_u64()).unwrap_or(0);
        f["seq"] = json!(seq.saturating_add(delta).saturating_add(i as u64 * 13));
    }
    snap.fold = serde_json::from_value(v).expect("deserialize bumped fold");
    Runtime::from_snapshot(snap, rt.log().as_slice().to_vec())
}

fn rewrite_invalidated_at(rt: &Runtime, at: Option<u64>) -> Runtime {
    let mut snap = rt.snapshot();
    let mut v = serde_json::to_value(&snap.fold).expect("serialize fold");
    let facts = v
        .get_mut("facts")
        .and_then(|f| f.as_array_mut())
        .expect("facts array");
    assert!(!facts.is_empty(), "need a row to rewrite invalidated_at");
    facts[0]["invalidated_at"] = json!(at);
    snap.fold = serde_json::from_value(v).expect("deserialize rewritten fold");
    Runtime::from_snapshot(snap, rt.log().as_slice().to_vec())
}

fn with_checkpoint_in_log(rt: &Runtime) -> (Runtime, usize) {
    let mut events = rt.log().as_slice().to_vec();
    let first_assert = events
        .iter()
        .position(|e| matches!(e.op, Op::Assert { .. }))
        .expect("need an Assert so the fold is nonempty at the receipt");
    assert!(
        first_assert + 2 < events.len(),
        "need events after the insert so a legal cut c+1 exists, len={}",
        events.len()
    );
    let insert_at = first_assert + 1;
    let ckpt = Event::new(
        Op::Checkpoint {
            first_event_id: EventId::nil(),
            last_event_id: EventId::nil(),
            dropped_count: 0,
            fact_count: 0,
            fingerprint_hex: String::new(),
            blob_digest: String::new(),
        },
        0u64,
    );
    events.insert(insert_at, ckpt);
    let rebuilt =
        Runtime::from_dict_and_events(rt.dictionary().strings().to_vec(), events, rt.max_cascade)
            .expect("rebuild with injected Checkpoint");
    let c = rebuilt
        .log()
        .as_slice()
        .iter()
        .position(|e| matches!(e.op, Op::Checkpoint { .. }))
        .expect("injected Checkpoint");
    assert_eq!(c, insert_at);
    (rebuilt, c)
}

fn statute_pair() -> (Runtime, EventId) {
    let mut rt = Runtime::default();
    let id = emit_keyed(
        &mut rt,
        "art-12-speed",
        "inForceAs",
        "50-kmh",
        2010,
        None,
        "statute-50",
    );
    emit_keyed(
        &mut rt,
        "art-12-speed",
        "inForceAs",
        "30-kmh",
        2010,
        Some(2020),
        "statute-30",
    );
    emit_keyed(
        &mut rt,
        "art-13-limit",
        "inForceAs",
        "90-kmh",
        2010,
        None,
        "statute-90",
    );
    (rt, id)
}

#[test]
fn kill_runtime_diff_equals_diff_folds_aa_empty_and_mirror() {
    let (mut rt, _) = statute_pair();
    let cut = rt.log().len();
    let fork = rt.fork_at(cut).expect("fork at tip");
    assert_wrapper(&rt, &fork, "tip vs self-fork");
    assert_empty(&rt.diff(&rt), "Runtime::diff(a,a) via the method");
    assert_empty(&fork.diff(&fork), "Runtime::diff(fork,fork)");
    assert_empty(
        &rt.diff(&fork),
        "fork_at(len) must match the parent picture",
    );
    assert_mirror(&rt, &fork, "tip vs self-fork");

    let extra = emit_keyed(
        &mut rt,
        "art-12-speed",
        "inForceAs",
        "70-kmh",
        2012,
        None,
        "statute-70",
    );
    assert_wrapper(&fork, &rt, "fork vs parent after extra mint");
    assert_mirror(&fork, &rt, "fork vs parent after extra mint");
    let d = fork.diff(&rt);
    assert!(
        d.added.iter().any(|f| f.event_id == extra),
        "post-cut mint must be added, not a live-only filter: {d:?}"
    );
    assert!(d.removed.is_empty(), "extra mint must not remove: {d:?}");
    let q = scan_ir(2012);
    let fork_scan = triples(execute(&fork, &q).unwrap());
    let parent_scan = triples(execute(&rt, &q).unwrap());
    assert_eq!(fork_scan, sorted_live(&fork, 2012));
    assert_eq!(parent_scan, sorted_live(&rt, 2012));
    assert_ne!(
        fork_scan, parent_scan,
        "Scan on both sides must see different live bags after a post-cut mint"
    );
    assert!(
        !fork_scan.is_empty() && !parent_scan.is_empty(),
        "both Scan bags must be nonempty"
    );
}

#[test]
fn kill_seq_only_empty_via_the_method() {
    let (rt, _) = statute_pair();
    let bumped = bump_seqs(&rt, 99);
    assert!(
        rt.fold()
            .facts()
            .iter()
            .zip(bumped.fold().facts())
            .any(|(a, b)| a.seq != b.seq),
        "fixture must actually differ in seq"
    );
    assert_wrapper(&rt, &bumped, "seq-only pair");
    assert_empty(
        &rt.diff(&bumped),
        "Runtime::diff must ignore seq (identity is event_id+window+rank)",
    );
    assert_empty(&bumped.diff(&rt), "seq-only reverse");

    let rewritten = rewrite_invalidated_at(&rt, Some(7));
    let d = rt.diff(&rewritten);
    assert!(
        d.added.is_empty() && d.removed.is_empty() && d.changed.len() == 1,
        "real field change must be changed, not hidden by seq: {d:?}"
    );
    assert!(d.changed[0].before.invalidated_at.is_none());
    assert_eq!(d.changed[0].after.invalidated_at, Some(7));
    assert_wrapper(&rt, &rewritten, "invalidated_at rewrite");
}

#[test]
fn kill_retract_is_changed_not_removed_and_scan_drops() {
    let (mut rt, id) = statute_pair();
    let cut = rt.log().len();
    let fork = rt.fork_at(cut).expect("fork before retract");
    rt.emit(Op::Retract { event_id: id }).expect("retract live");

    assert_wrapper(&fork, &rt, "fork vs retracted parent");
    assert_mirror(&fork, &rt, "fork vs retracted parent");
    let d = fork.diff(&rt);
    assert!(
        d.added.is_empty() && d.removed.is_empty(),
        "identity-stable Retract must not add/remove: {d:?}"
    );
    assert_eq!(d.changed.len(), 1, "Retract must be one changed row: {d:?}");
    assert_eq!(d.changed[0].before.event_id, id);
    assert!(d.changed[0].before.invalidated_at.is_none());
    assert_eq!(d.changed[0].after.event_id, id);
    assert!(d.changed[0].after.invalidated_at.is_some());
    assert!(
        rt.fold()
            .facts()
            .iter()
            .any(|f| f.event_id == id && f.invalidated_at.is_some()),
        "diff domain is facts() including the dead row"
    );

    let parent_scan = scan(&rt, 2010);
    let fork_scan = scan(&fork, 2010);
    assert_eq!(parent_scan, sorted_live(&rt, 2010));
    assert_eq!(fork_scan, sorted_live(&fork, 2010));
    let subj = rt.dictionary().id("art-12-speed").unwrap();
    let rel = rt.dictionary().id("inForceAs").unwrap();
    let obj = rt.dictionary().id("50-kmh").unwrap();
    assert!(
        !parent_scan.contains(&(subj, rel, obj)),
        "Scan is the live picture and must drop the retracted triple: {parent_scan:?}"
    );
    assert!(
        fork_scan.contains(&(subj, rel, obj)),
        "fork Scan must keep the triple (fork isolation, not a shared fold): {fork_scan:?}"
    );

    let tt_before =
        TransactionTime::from(d.changed[0].after.invalidated_at.unwrap().saturating_sub(1));
    let pit = QueryIr::Scan {
        tt: tt_before,
        vt: vt(2010),
        limit: 0,
    };
    let pit_rows = triples(execute(&rt, &pit).expect("PIT Scan"));
    assert!(
        pit_rows.contains(&(subj, rel, obj)),
        "Scan at tt before invalidated_at must still see the triple"
    );
}

#[test]
fn kill_expired_rows_stay_in_diff_not_in_scan() {
    let mut rt = Runtime::default();
    let cut = rt.log().len();
    emit_keyed(
        &mut rt,
        "art-12-speed",
        "inForceAs",
        "50-kmh",
        2010,
        Some(2015),
        "expired-50",
    );
    let fork = rt.fork_at(cut).expect("fork before expired mint");
    assert_wrapper(&fork, &rt, "pre-mint vs expired");
    let d = fork.diff(&rt);
    let expired = d
        .added
        .iter()
        .find(|f| f.delivery_key.as_deref() == Some("expired-50"))
        .expect("expired product row must be in the fact-set added bag");
    assert_eq!(expired.valid_to, Some(2015));
    assert!(
        !expired.is_live_at(tt_max(), vt(2020)),
        "fixture row must be dead at Scan vt"
    );
    let subj = rt.dictionary().id("art-12-speed").unwrap();
    let rel = rt.dictionary().id("inForceAs").unwrap();
    let obj = rt.dictionary().id("50-kmh").unwrap();
    assert!(
        !scan(&rt, 2020).contains(&(subj, rel, obj)),
        "Scan at vt past valid_to must drop the expired triple"
    );
    assert_eq!(scan(&rt, 2010), sorted_live(&rt, 2010));
    assert!(
        scan(&rt, 2010).contains(&(subj, rel, obj)),
        "Scan inside the window must keep the triple"
    );
}

#[test]
fn kill_checkpoint_legal_cut_vs_interior_err() {
    let (rt, _) = statute_pair();
    match rt.fork_at(rt.log().len() + 1) {
        Err(RuntimeError::ForkCutUnavailable { cut, log_len }) => {
            assert_eq!(cut, rt.log().len() + 1);
            assert_eq!(log_len, rt.log().len());
        }
        Ok(_) => panic!("past-end cut must be ForkCutUnavailable, got Ok"),
        Err(e) => panic!("past-end cut must be ForkCutUnavailable, got {e:?}"),
    }

    let (ckpt, c) = with_checkpoint_in_log(&rt);
    let len = ckpt.log().len();
    assert!(c >= 1, "receipt must sit after at least one event, c={c}");
    assert!(c + 1 <= len, "need a legal tail cut, c={c} len={len}");

    for n in 1..=c {
        match ckpt.fork_at(n) {
            Err(RuntimeError::ForkCutUnavailable { cut, log_len }) => {
                assert_eq!(cut, n);
                assert_eq!(log_len, len);
            }
            Ok(_) => panic!("fork_at({n}) Ok — interior 1..={c} must never reach diff/Scan"),
            Err(e) => panic!("fork_at({n}) wrong error {e:?}"),
        }
    }

    let empty = ckpt
        .fork_at(0)
        .expect("n==0 stays Ok on a checkpointed log");
    assert!(empty.fold().facts().is_empty());
    let legal = ckpt.fork_at(c + 1).expect("legal cut n>c must be Ok");
    assert_wrapper(&legal, &ckpt, "legal checkpointed cut vs head");
    assert_mirror(&legal, &ckpt, "legal checkpointed cut vs head");
    let q = scan_ir(2010);
    let legal_scan = triples(execute(&legal, &q).unwrap());
    let head_scan = triples(execute(&ckpt, &q).unwrap());
    assert_eq!(legal_scan, sorted_live(&legal, 2010));
    assert_eq!(head_scan, sorted_live(&ckpt, 2010));
    let tip = ckpt.fork_at(len).expect("fork_at(len) legal");
    assert_empty(&ckpt.diff(&tip), "fork_at(len) matches head");
}

#[test]
fn kill_prepared_from_one_runtime_is_not_the_other_picture() {
    let (mut rt, id) = statute_pair();
    let cut = rt.log().len();
    let fork = rt.fork_at(cut).expect("fork");
    rt.emit(Op::Retract { event_id: id }).expect("retract");

    let q = scan_ir(2010);
    let prepared_parent = prepare(&rt, &q).expect("prepare parent");
    let prepared_fork = prepare(&fork, &q).expect("prepare fork");
    let parent_live = triples(execute(&rt, &q).unwrap());
    let fork_live = triples(execute(&fork, &q).unwrap());
    assert_eq!(triples(prepared_parent.run()), parent_live);
    assert_eq!(triples(prepared_fork.run()), fork_live);
    assert_ne!(
        triples(prepared_parent.run()),
        fork_live,
        "Prepared taken on the parent is that fold's snapshot, not the fork"
    );
    assert_ne!(
        triples(prepared_fork.run()),
        parent_live,
        "Prepared taken on the fork is not the parent's retracted picture"
    );
    // Misuse: there is no API to rebind a Prepared to another Runtime.
    // execute always prepare+run on the argument it is given. A caller who
    // keeps parent_prepared.run() while reading fork is holding the wrong picture.
}

#[test]
fn kill_unknown_relation_fail_closed_on_fork_scan_still_ok() {
    let (mut parent, _) = statute_pair();
    let cut = parent.log().len();
    let fork = parent.fork_at(cut).expect("fork");
    let start = fork
        .dictionary()
        .id("art-12-speed")
        .expect("start interned before fork");

    assert!(
        fork.dictionary().id("ghost-rel").is_none(),
        "ghost must be absent before the named hops call"
    );
    match try_hops_named(
        &fork,
        "art-12-speed",
        &["ghost-rel"],
        1,
        tt_max(),
        vt(2010),
        0,
    ) {
        Err(QueryError::UnknownRelation { name }) => assert_eq!(name, "ghost-rel"),
        other => panic!("unknown relation on the fork must fail closed, got {other:?}"),
    }
    assert!(
        fork.dictionary().id("ghost-rel").is_none(),
        "try_hops_named must not intern a miss"
    );

    let parent_only = parent.intern("parent-only-rel");
    assert!(
        fork.dictionary().id("parent-only-rel").is_none(),
        "fork dict must not observe a post-fork intern on the parent"
    );
    match try_hops_named(
        &fork,
        "art-12-speed",
        &["parent-only-rel"],
        1,
        tt_max(),
        vt(2010),
        0,
    ) {
        Err(QueryError::UnknownRelation { name }) => assert_eq!(name, "parent-only-rel"),
        other => panic!("parent-only relation on the fork must fail closed, got {other:?}"),
    }

    let stolen = QueryIr::Hops {
        start,
        rels: vec![parent_only],
        k: 1,
        tt: tt_max(),
        vt: vt(2010),
        limit: 0,
    };
    match execute(&fork, &stolen) {
        Err(QueryError::UnknownTerm { .. }) => {}
        other => {
            panic!("Hops TermId minted after fork must fail closed on the fork, got {other:?}")
        }
    }

    let fork_scan = scan(&fork, 2010);
    assert_eq!(fork_scan, sorted_live(&fork, 2010));
    assert!(
        !fork_scan.is_empty(),
        "Scan on the fork must still succeed and stay nonempty"
    );
    assert_wrapper(&fork, &parent, "fork vs parent after parent-only intern");
}
