//! Independent verifier kill-tests for M157 / phase 231 query IR.
//! Expectations from public `QueryIr` / `execute` / `try_hops_named`,
//! `as_of` / `live_at` / CSR leases, ADR-040/041, and roadmap B3.
//! Not executor tests as SoT. In-memory only.

use kutha_common::{rule_definition_hash, Op, TermId, TransactionTime, ValidTime};
use kutha_runtime::{execute, try_hops_named, QueryError, QueryIr, QueryRows, Runtime};
use std::collections::BTreeSet;
use std::panic::{catch_unwind, AssertUnwindSafe};

fn kill_case_count() -> u64 {
    match std::env::var("KUTHA_KILL_CASES") {
        Err(std::env::VarError::NotPresent) => 16,
        Ok(s) => {
            let n: u64 = s.parse().unwrap_or_else(|_| {
                panic!("KUTHA_KILL_CASES must be a positive integer, got {s:?}")
            });
            if n == 0 {
                panic!("KUTHA_KILL_CASES must be a positive integer, got {s:?}");
            }
            n
        }
        Err(e) => panic!("KUTHA_KILL_CASES must be a positive integer, got {e}"),
    }
}

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    fn bounded(&mut self, n: u64) -> u64 {
        if n == 0 {
            0
        } else {
            self.next() % n
        }
    }
}

fn shuffle<T>(items: &mut [T], rng: &mut Rng) {
    let n = items.len();
    if n < 2 {
        return;
    }
    for i in (1..n).rev() {
        let j = rng.bounded((i + 1) as u64) as usize;
        items.swap(i, j);
    }
}

fn tt_max() -> TransactionTime {
    TransactionTime::MAX
}

fn vt(y: u64) -> ValidTime {
    ValidTime::from(y)
}

fn tt(n: u64) -> TransactionTime {
    TransactionTime::from(n)
}

fn emit_edge(
    rt: &mut Runtime,
    s: TermId,
    r: TermId,
    o: TermId,
    valid_from: u64,
    valid_to: Option<u64>,
    key: Option<&str>,
) {
    rt.emit(Op::Assert {
        subject: s,
        relation: r,
        object: o,
        valid_from,
        valid_to,
        claim: None,
        delivery_key: key.map(str::to_string),
        polarity: None,
    })
    .expect("product Assert must admit");
}

fn emit_edge_result(
    rt: &mut Runtime,
    s: TermId,
    r: TermId,
    o: TermId,
    valid_from: u64,
    valid_to: Option<u64>,
    key: Option<&str>,
) -> Result<kutha_runtime::QuantumOutcome, kutha_runtime::RuntimeError> {
    rt.emit(Op::Assert {
        subject: s,
        relation: r,
        object: o,
        valid_from,
        valid_to,
        claim: None,
        delivery_key: key.map(str::to_string),
        polarity: None,
    })
}

/// Independent hop oracle: exactly-k unique ends over `live_at` triples.
/// Empty `rels` = any relation; len 1 = same rel every hop; len k = per hop.
fn hop_oracle(
    triples: &[(TermId, TermId, TermId)],
    start: TermId,
    rels: &[TermId],
    k: u32,
) -> Vec<TermId> {
    let mut frontier: BTreeSet<TermId> = BTreeSet::from([start]);
    for hop in 0..k {
        let mut next = BTreeSet::new();
        for &v in &frontier {
            for &(s, rel, o) in triples {
                if s != v {
                    continue;
                }
                let ok = match rels.len() {
                    0 => true,
                    1 => rel == rels[0],
                    _ => rel == rels[hop as usize],
                };
                if ok {
                    next.insert(o);
                }
            }
        }
        frontier = next;
    }
    frontier.into_iter().collect()
}

fn exec_hops(
    rt: &Runtime,
    start: TermId,
    rels: Vec<TermId>,
    k: u32,
    t: TransactionTime,
    v: ValidTime,
    limit: usize,
) -> Result<Vec<TermId>, QueryError> {
    match execute(
        rt,
        &QueryIr::Hops {
            start,
            rels,
            k,
            tt: t,
            vt: v,
            limit,
        },
    ) {
        Ok(QueryRows::Nodes(n)) => Ok(n),
        Ok(other) => panic!("Hops must return Nodes, got {other:?}"),
        Err(e) => Err(e),
    }
}

fn exec_scan(
    rt: &Runtime,
    t: TransactionTime,
    v: ValidTime,
    limit: usize,
) -> Result<Vec<(TermId, TermId, TermId)>, QueryError> {
    match execute(
        rt,
        &QueryIr::Scan {
            tt: t,
            vt: v,
            limit,
        },
    ) {
        Ok(QueryRows::Triples(rows)) => Ok(rows),
        Ok(other) => panic!("Scan must return Triples, got {other:?}"),
        Err(e) => Err(e),
    }
}

fn sorted_live(rt: &Runtime, t: TransactionTime, v: ValidTime) -> Vec<(TermId, TermId, TermId)> {
    let mut rows = rt.live_at(t, v);
    rows.sort_unstable();
    rows
}

fn sorted_as_of(rt: &Runtime, v: ValidTime) -> Vec<(TermId, TermId, TermId)> {
    let mut rows = rt.as_of(v);
    rows.sort_unstable();
    rows
}

struct Snap {
    log: usize,
    dict: usize,
    facts: usize,
    defines: usize,
}

fn snap(rt: &Runtime) -> Snap {
    Snap {
        log: rt.log().len(),
        dict: rt.dictionary().len(),
        facts: rt.fold().facts().len(),
        defines: rt
            .log()
            .iter()
            .filter(|e| matches!(e.op, Op::Define { .. }))
            .count(),
    }
}

fn assert_frozen(before: &Snap, rt: &Runtime, ctx: &str) {
    let after = snap(rt);
    assert_eq!(before.log, after.log, "{ctx}: log grew");
    assert_eq!(before.dict, after.dict, "{ctx}: dictionary grew");
    assert_eq!(before.facts, after.facts, "{ctx}: fold facts grew");
    assert_eq!(before.defines, after.defines, "{ctx}: Define appended");
}

fn ingested_at(rt: &Runtime, id: kutha_common::EventId) -> u64 {
    rt.log()
        .as_slice()
        .iter()
        .find(|e| e.id == id)
        .unwrap_or_else(|| panic!("event {id} missing from log"))
        .ingested_at
}

fn last_retract_tt(rt: &Runtime, retracted: kutha_common::EventId) -> u64 {
    rt.log()
        .as_slice()
        .iter()
        .rev()
        .find(|e| matches!(&e.op, Op::Retract { event_id } if *event_id == retracted))
        .unwrap_or_else(|| panic!("Retract of {retracted} missing"))
        .ingested_at
}

fn ff5_statute() -> Runtime {
    let mut rt = Runtime::default();
    let article = rt.intern("art-12-speed");
    let in_force = rt.intern("inForceAs");
    let limit_50 = rt.intern("50-kmh");
    let limit_30 = rt.intern("30-kmh");
    emit_edge(&mut rt, article, in_force, limit_50, 2010, Some(2020), None);
    emit_edge(&mut rt, article, in_force, limit_30, 2020, None, None);
    rt
}

#[test]
fn kill_scan_equals_sorted_as_of_bag_and_keeps_multiplicity() {
    let rt = ff5_statute();
    for year in [2009, 2010, 2015, 2019, 2020, 2021] {
        let v = vt(year);
        let got = exec_scan(&rt, tt_max(), v, 0).expect("Scan");
        assert_eq!(
            got,
            sorted_as_of(&rt, v),
            "Scan(MAX,{year}) must equal sorted as_of"
        );
        assert_eq!(
            got,
            sorted_live(&rt, tt_max(), v),
            "Scan(MAX,{year}) must equal sorted live_at"
        );
    }

    let mut bag = Runtime::default();
    let a = bag.intern("dup-s");
    let r = bag.intern("relatedTo");
    let o = bag.intern("dup-o");
    emit_edge(&mut bag, a, r, o, 2010, None, Some("k1"));
    emit_edge(&mut bag, a, r, o, 2010, None, Some("k2"));
    let scan = exec_scan(&bag, tt_max(), vt(2010), 0).expect("Scan bag");
    let live = sorted_live(&bag, tt_max(), vt(2010));
    assert_eq!(scan, live, "Scan bag must keep live_at multiplicity");
    let copies = scan.iter().filter(|t| **t == (a, r, o)).count();
    assert!(
        copies >= 2,
        "two keyed Asserts of the same triple must stay a bag, got {copies}"
    );
}

#[test]
fn kill_scan_honours_tt_not_only_as_of() {
    let mut rt = Runtime::default();
    let a = rt.intern("A");
    let rel = rt.intern("relatedTo");
    let b = rt.intern("B");
    let out = emit_edge_result(&mut rt, a, rel, b, 2010, None, None).unwrap();
    let eid = out.receipt.event_ids[0];
    let tt_assert = ingested_at(&rt, eid);
    rt.emit(Op::Retract { event_id: eid }).unwrap();
    let tt_retract = last_retract_tt(&rt, eid);

    let v = vt(2010);
    let before = exec_scan(&rt, tt(tt_retract.saturating_sub(1)), v, 0).expect("pre-retract");
    let after = exec_scan(&rt, tt(tt_retract), v, 0).expect("at-retract");
    let asof = exec_scan(&rt, tt_max(), v, 0).expect("as_of via Scan");
    assert!(
        before.contains(&(a, rel, b)),
        "Scan at tt before retract must still see the row (tt_assert={tt_assert})"
    );
    assert!(
        !after.contains(&(a, rel, b)),
        "Scan at retract tt must hide the retracted row"
    );
    assert_eq!(
        after, asof,
        "Scan(MAX) equals Scan(retract_tt) after retract"
    );
    assert_ne!(
        before, asof,
        "Scan must honour tt: pre-retract bag ≠ as_of after retract"
    );
}

#[test]
fn kill_hops_exact_k_set_matches_live_at_oracle() {
    let mut rt = Runtime::default();
    let rel = rt.intern("relatedTo");
    let a = rt.intern("A");
    let b = rt.intern("B");
    let c = rt.intern("C");
    let d = rt.intern("D");
    // chain A→B→C, branch A→D, diamond D→C, self-loop C→C, cycle C→B
    emit_edge(&mut rt, a, rel, b, 2010, None, None);
    emit_edge(&mut rt, b, rel, c, 2010, None, None);
    emit_edge(&mut rt, a, rel, d, 2010, None, None);
    emit_edge(&mut rt, d, rel, c, 2010, None, None);
    emit_edge(&mut rt, c, rel, c, 2010, None, None);
    emit_edge(&mut rt, c, rel, b, 2010, None, None);

    let triples = rt.live_at(tt_max(), vt(2010));
    for k in 0..=5 {
        let got = exec_hops(&rt, a, vec![rel], k, tt_max(), vt(2010), 0).expect("hops");
        let expect = hop_oracle(&triples, a, &[rel], k);
        assert_eq!(got, expect, "exact-{k} hops from A vs live_at oracle");
    }
    // k=0 is {start} even if start has no edges
    let z = rt.intern("Z-isolated");
    let z_got = exec_hops(&rt, z, vec![], 0, tt_max(), vt(2010), 0).expect("k=0");
    assert_eq!(z_got, vec![z]);

    // k larger than diameter from A without a cycle back: still matches oracle
    let got10 = exec_hops(&rt, a, vec![rel], 10, tt_max(), vt(2010), 0).expect("k=10");
    assert_eq!(got10, hop_oracle(&triples, a, &[rel], 10));
}

#[test]
fn kill_per_hop_rels_shape_and_order() {
    let mut rt = Runtime::default();
    let knows = rt.intern("relatedTo");
    let owns = rt.intern("inForceAs");
    let a = rt.intern("ph-A");
    let b = rt.intern("ph-B");
    let c = rt.intern("ph-C");
    emit_edge(&mut rt, a, knows, b, 2010, None, None);
    emit_edge(&mut rt, b, owns, c, 2010, None, None);

    let triples = rt.live_at(tt_max(), vt(2010));
    let forward = exec_hops(&rt, a, vec![knows, owns], 2, tt_max(), vt(2010), 0).expect("per-hop");
    assert_eq!(forward, hop_oracle(&triples, a, &[knows, owns], 2));
    assert_eq!(forward, vec![c]);

    let swapped = exec_hops(&rt, a, vec![owns, knows], 2, tt_max(), vt(2010), 0).expect("swapped");
    assert!(
        swapped.is_empty(),
        "swapped per-hop rels must yield no ends"
    );

    let shape = execute(
        &rt,
        &QueryIr::Hops {
            start: a,
            rels: vec![knows, owns],
            k: 3,
            tt: tt_max(),
            vt: vt(2010),
            limit: 0,
        },
    );
    match shape {
        Err(QueryError::RelsShape { rels: 2, k: 3 }) => {}
        other => panic!("expected RelsShape 2 vs k=3, got {other:?}"),
    }

    let repeated = exec_hops(&rt, a, vec![knows], 2, tt_max(), vt(2010), 0).expect("repeat");
    let per_same =
        exec_hops(&rt, a, vec![knows, knows], 2, tt_max(), vt(2010), 0).expect("per same");
    assert_eq!(repeated, per_same, "len==1 and len==k with the same rel");
}

#[test]
fn kill_unlabeled_follows_any_relation() {
    let mut rt = Runtime::default();
    let r1 = rt.intern("relatedTo");
    let r2 = rt.intern("observed");
    let a = rt.intern("u-A");
    let b = rt.intern("u-B");
    let c = rt.intern("u-C");
    emit_edge(&mut rt, a, r1, b, 2010, None, None);
    emit_edge(&mut rt, a, r2, c, 2010, None, None);
    // parallel different relations onto the same object
    emit_edge(&mut rt, a, r2, b, 2010, None, None);

    let triples = rt.live_at(tt_max(), vt(2010));
    let any = exec_hops(&rt, a, vec![], 1, tt_max(), vt(2010), 0).expect("unlabeled");
    let mut expect: Vec<TermId> = hop_oracle(&triples, a, &[], 1);
    expect.sort();
    assert_eq!(any, expect);
    assert_eq!(any, {
        let mut v = vec![b, c];
        v.sort();
        v
    });
    let only_r1 = exec_hops(&rt, a, vec![r1], 1, tt_max(), vt(2010), 0).expect("r1");
    assert_eq!(only_r1, vec![b]);
}

#[test]
fn kill_clocks_valid_to_exclusive_and_retract_tt() {
    let mut rt = Runtime::default();
    let rel = rt.intern("relatedTo");
    let a = rt.intern("clk-A");
    let b = rt.intern("clk-B");
    let c = rt.intern("clk-C");
    emit_edge(&mut rt, a, rel, b, 2010, Some(2020), Some("ab"));
    emit_edge(&mut rt, b, rel, c, 2010, None, Some("bc"));

    let at_end = exec_hops(&rt, a, vec![rel], 1, tt_max(), vt(2020), 0).expect("vt=to");
    assert!(
        at_end.is_empty(),
        "valid_to is exclusive: vt==valid_to must not hop A→B, got {at_end:?}"
    );
    let before_end = exec_hops(&rt, a, vec![rel], 1, tt_max(), vt(2019), 0).expect("vt=to-1");
    assert_eq!(before_end, vec![b]);

    let ab = rt
        .fold()
        .facts()
        .iter()
        .find(|f| f.subject == a && f.object() == b && f.invalidated_at.is_none())
        .unwrap()
        .event_id;
    rt.emit(Op::Retract { event_id: ab }).unwrap();
    let retract_tt = last_retract_tt(&rt, ab);

    let later = exec_hops(&rt, a, vec![rel], 1, tt(retract_tt), vt(2015), 0).expect("later tt");
    assert!(later.is_empty(), "retracted edge invisible at retract tt");
    let earlier = exec_hops(
        &rt,
        a,
        vec![rel],
        1,
        tt(retract_tt.saturating_sub(1)),
        vt(2015),
        0,
    )
    .expect("earlier tt");
    assert_eq!(earlier, vec![b], "retracted edge visible at earlier tt");
}

#[test]
fn kill_limit_is_sorted_prefix_zero_unlimited() {
    let mut rt = Runtime::default();
    let rel = rt.intern("relatedTo");
    let a = rt.intern("lim-A");
    let mut objs = Vec::new();
    for name in ["lim-Z", "lim-M", "lim-B"] {
        let o = rt.intern(name);
        emit_edge(&mut rt, a, rel, o, 2010, None, None);
        objs.push(o);
    }
    objs.sort();
    let all = exec_hops(&rt, a, vec![rel], 1, tt_max(), vt(2010), 0).expect("unlimited");
    assert_eq!(all, objs, "limit 0 is unlimited sorted set");
    let pref = exec_hops(&rt, a, vec![rel], 1, tt_max(), vt(2010), 2).expect("limit 2");
    assert_eq!(pref, objs[..2], "limit is a prefix of the sorted rows");
    let over = exec_hops(&rt, a, vec![rel], 1, tt_max(), vt(2010), 99).expect("oversize");
    assert_eq!(over, objs);

    let mut scan_all = rt.as_of(vt(2010));
    scan_all.sort_unstable();
    let scan_pref = exec_scan(&rt, tt_max(), vt(2010), 1).expect("scan limit");
    assert_eq!(scan_pref, scan_all[..1]);
    let scan_zero = exec_scan(&rt, tt_max(), vt(2010), 0).expect("scan 0");
    assert_eq!(scan_zero, scan_all);
}

#[test]
fn kill_fail_closed_unknown_names_no_intern() {
    let mut rt = Runtime::default();
    let start = rt.intern("fc-start");
    let rel = rt.intern("relatedTo");
    let o = rt.intern("fc-o");
    emit_edge(&mut rt, start, rel, o, 2010, None, None);
    let before = snap(&rt);

    match try_hops_named(
        &rt,
        "no-such-term-zzz",
        &["relatedTo"],
        1,
        tt_max(),
        vt(2010),
        0,
    ) {
        Err(QueryError::UnknownTerm { name }) => {
            assert_eq!(name, "no-such-term-zzz");
        }
        other => panic!("expected UnknownTerm, got {other:?}"),
    }
    assert_frozen(&before, &rt, "unknown start");

    match try_hops_named(
        &rt,
        "fc-start",
        &["no-such-rel-zzz"],
        1,
        tt_max(),
        vt(2010),
        0,
    ) {
        Err(QueryError::UnknownRelation { name }) => {
            assert_eq!(name, "no-such-rel-zzz");
        }
        other => panic!("expected UnknownRelation, got {other:?}"),
    }
    assert_frozen(&before, &rt, "unknown relation");

    match try_hops_named(
        &rt,
        "fc-start",
        &["relatedTo", "relatedTo"],
        3,
        tt_max(),
        vt(2010),
        0,
    ) {
        Err(QueryError::RelsShape { rels: 2, k: 3 }) => {}
        other => panic!("expected RelsShape, got {other:?}"),
    }
    assert_frozen(&before, &rt, "rels shape");

    let _ = exec_hops(&rt, start, vec![rel], 1, tt_max(), vt(2010), 0).expect("ok hops");
    assert_frozen(&before, &rt, "successful execute");
}

#[test]
fn kill_no_panic_weird_input() {
    let mut rt = Runtime::default();
    let start = rt.intern("weird-s");
    let rel = rt.intern("relatedTo");
    emit_edge(&mut rt, start, rel, start, 2010, None, None); // self-loop

    let cases: Vec<QueryIr> = vec![
        QueryIr::Hops {
            start: TermId::MAX,
            rels: vec![],
            k: 1,
            tt: tt_max(),
            vt: vt(2010),
            limit: 0,
        },
        QueryIr::Hops {
            start,
            rels: vec![TermId::MAX],
            k: 1,
            tt: tt_max(),
            vt: vt(2010),
            limit: 0,
        },
        QueryIr::Hops {
            start,
            rels: vec![],
            k: 20_000,
            tt: tt_max(),
            vt: vt(2010),
            limit: 0,
        },
        QueryIr::Hops {
            start,
            rels: vec![],
            k: 1,
            tt: tt_max(),
            vt: vt(2010),
            limit: 0,
        },
        QueryIr::Scan {
            tt: tt(0),
            vt: vt(0),
            limit: 0,
        },
        QueryIr::Hops {
            start,
            rels: vec![rel, rel, rel],
            k: 1,
            tt: tt_max(),
            vt: vt(2010),
            limit: 0,
        },
    ];
    for q in cases {
        let outcome = catch_unwind(AssertUnwindSafe(|| execute(&rt, &q)));
        assert!(outcome.is_ok(), "execute panicked on {q:?}: {outcome:?}");
    }

    // empty product fold: interned start, k>0 → empty nodes, no panic
    let mut empty = Runtime::default();
    let ghost = empty.intern("only-interned");
    let got = exec_hops(&empty, ghost, vec![], 3, tt_max(), vt(2010), 0).expect("empty fold");
    assert!(got.is_empty());

    // inverted / empty valid windows must not panic; never live
    let mut win = Runtime::default();
    let s = win.intern("inv-s");
    let r = win.intern("relatedTo");
    let o = win.intern("inv-o");
    let _ = emit_edge_result(&mut win, s, r, o, 2020, Some(2010), Some("inverted"));
    let _ = emit_edge_result(&mut win, s, r, o, 2010, Some(2010), Some("empty-win"));
    let hops = catch_unwind(AssertUnwindSafe(|| {
        exec_hops(&win, s, vec![r], 1, tt_max(), vt(2015), 0)
    }));
    assert!(hops.is_ok(), "inverted window hops panicked");
    let live = win.live_at(tt_max(), vt(2015));
    let expect = hop_oracle(&live, s, &[r], 1);
    assert_eq!(hops.unwrap().unwrap(), expect);
}

#[test]
fn kill_horn_derived_heads_visible_iff_live() {
    let mut rt = Runtime::default();
    let a = rt.intern("horn-A");
    let b = rt.intern("horn-B");
    let knows = rt.dictionary().id("knows").expect("genesis knows");
    let known_by = rt.dictionary().id("knownBy").expect("genesis knownBy");
    let asserted = rt
        .emit(Op::Assert {
            subject: a,
            relation: knows,
            object: b,
            valid_from: 2010,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap();
    let cause = asserted.receipt.event_ids[0];

    let live = rt.live_at(tt_max(), vt(2010));
    assert!(
        live.iter()
            .any(|&(s, r, o)| s == b && r == known_by && o == a),
        "inverse_knows must mint a live derived fact B-knownBy-A"
    );
    let derived_hop = exec_hops(&rt, b, vec![known_by], 1, tt_max(), vt(2010), 0).expect("derived");
    assert_eq!(
        derived_hop,
        hop_oracle(&live, b, &[known_by], 1),
        "derived Horn head must be hop-visible while live"
    );
    assert_eq!(derived_hop, vec![a]);

    rt.emit(Op::Retract { event_id: cause }).unwrap();
    let retract_tt = last_retract_tt(&rt, cause);
    let after = rt.live_at(tt(retract_tt), vt(2010));
    let dead = exec_hops(&rt, b, vec![known_by], 1, tt(retract_tt), vt(2010), 0).expect("dead");
    assert_eq!(dead, hop_oracle(&after, b, &[known_by], 1));
    assert!(
        dead.is_empty(),
        "derived head must vanish from hops when no longer live"
    );
    let earlier = exec_hops(
        &rt,
        b,
        vec![known_by],
        1,
        tt(retract_tt.saturating_sub(1)),
        vt(2010),
        0,
    )
    .expect("earlier derived");
    assert_eq!(earlier, vec![a]);
}

#[test]
fn kill_correct_interval_residuals_match_oracle() {
    let mut rt = Runtime::default();
    let rel = rt.intern("relatedTo");
    let a = rt.intern("ci-A");
    let b = rt.intern("ci-B");
    let c = rt.intern("ci-C");
    let out = emit_edge_result(&mut rt, a, rel, b, 2010, None, Some("ab")).unwrap();
    let eid = out.receipt.event_ids[0];
    rt.emit(Op::CorrectInterval {
        event_id: eid,
        object: c,
        patch_from: 2018,
        patch_to: Some(2020),
    })
    .unwrap();

    for year in [2010, 2015, 2017, 2018, 2019, 2020, 2021] {
        let v = vt(year);
        let live = rt.live_at(tt_max(), v);
        let got = exec_hops(&rt, a, vec![rel], 1, tt_max(), v, 0).expect("ci hops");
        assert_eq!(
            got,
            hop_oracle(&live, a, &[rel], 1),
            "CorrectInterval residual at vt={year}"
        );
    }
}

#[test]
fn kill_shuffle_assert_order_deterministic() {
    let names = ["sh-A", "sh-B", "sh-C", "sh-D"];
    let edges = [
        (0usize, 1usize),
        (1, 2),
        (0, 2),
        (2, 3),
        (3, 1),
        (1, 1), // self-loop
    ];
    let mut orders: Vec<Vec<(usize, usize)>> = Vec::new();
    let mut rng = Rng(0xC0FFEE);
    for _ in 0..8 {
        let mut e = edges.to_vec();
        shuffle(&mut e, &mut rng);
        orders.push(e);
    }

    let mut answers: Vec<Vec<TermId>> = Vec::new();
    for order in &orders {
        let mut rt = Runtime::default();
        let rel = rt.intern("relatedTo");
        let ids: Vec<TermId> = names.iter().map(|n| rt.intern(n)).collect();
        for &(i, j) in order {
            emit_edge(&mut rt, ids[i], rel, ids[j], 2010, None, None);
        }
        let got = exec_hops(&rt, ids[0], vec![rel], 2, tt_max(), vt(2010), 0).expect("shuffle");
        let live = rt.live_at(tt_max(), vt(2010));
        assert_eq!(got, hop_oracle(&live, ids[0], &[rel], 2));
        answers.push(got);
    }
    for a in &answers {
        assert_eq!(a, &answers[0], "assert order must not change hop ends");
        assert!(
            !a.is_empty(),
            "fixture must stay nonempty so empty==empty cannot pass"
        );
    }
}

#[test]
fn kill_multiplicity_parallel_dead_and_synthetic_horn() {
    let mut rt = Runtime::default();
    let rel = rt.intern("relatedTo");
    let extra = rt.intern("observed");
    let a = rt.intern("m-A");
    let b = rt.intern("m-B");
    let d = rt.intern("m-D");
    emit_edge(&mut rt, a, rel, b, 2010, None, Some("m1"));
    emit_edge(&mut rt, a, rel, b, 2010, None, Some("m2"));
    emit_edge(&mut rt, a, extra, b, 2010, None, Some("par"));
    let hops = exec_hops(&rt, a, vec![rel], 1, tt_max(), vt(2010), 0).expect("set");
    assert_eq!(
        hops,
        vec![b],
        "duplicate + parallel collapse to a set of ends"
    );

    // synthetic derived Behavior as a new hop edge, then retract it
    rt.emit_rule("statute-fold", 2010, None).unwrap();
    let pin = rule_definition_hash("statute-fold");
    let cause = rt
        .fold()
        .facts()
        .iter()
        .find(|f| f.subject == a && f.object() == b && f.invalidated_at.is_none())
        .unwrap()
        .event_id;
    let derived = rt
        .emit(Op::Behavior {
            name: "statute-fold".into(),
            caused_by: cause,
            rule_version: pin,
            subject: b,
            relation: rel,
            object: d,
            valid_from: 2010,
            valid_to: None,
        })
        .unwrap();
    let der_id = derived.receipt.event_ids[0];
    let live = rt.live_at(tt_max(), vt(2010));
    let two = exec_hops(&rt, a, vec![rel], 2, tt_max(), vt(2010), 0).expect("via derived");
    assert_eq!(two, hop_oracle(&live, a, &[rel], 2));
    assert!(
        two.contains(&d),
        "derived Behavior head must be hop-visible"
    );

    rt.emit(Op::Retract { event_id: der_id }).unwrap();
    let live_dead = rt.live_at(tt_max(), vt(2010));
    let after = exec_hops(&rt, a, vec![rel], 2, tt_max(), vt(2010), 0).expect("dead derived");
    assert_eq!(after, hop_oracle(&live_dead, a, &[rel], 2));
    assert!(
        !after.contains(&d),
        "dead derived row must not be hop-visible"
    );
}

#[test]
fn kill_property_random_graphs_vs_live_at_oracle() {
    let cases = kill_case_count();
    for seed in 1..=cases {
        let mut rng = Rng(0x9E37_79B9_7F4A_7C15 ^ seed.wrapping_mul(0xD1B5_4A32_D192_ED03));
        let mut rt = Runtime::default();
        let n_nodes = 2 + rng.bounded(6) as usize;
        let allowed = [
            "relatedTo",
            "inForceAs",
            "observed",
            "runStatus",
            "processAllows",
        ];
        let n_rels = 1 + rng.bounded(allowed.len() as u64) as usize;
        let nodes: Vec<TermId> = (0..n_nodes)
            .map(|i| rt.intern(&format!("p{seed}-n{i}")))
            .collect();
        let rels: Vec<TermId> = allowed[..n_rels].iter().map(|n| rt.intern(n)).collect();
        let n_edges = rng.bounded(12) as usize;
        for e in 0..n_edges {
            let s = nodes[rng.bounded(n_nodes as u64) as usize];
            let r = rels[rng.bounded(n_rels as u64) as usize];
            let o = nodes[rng.bounded(n_nodes as u64) as usize];
            let vf = 2010 + rng.bounded(3) * 5;
            let vt_choice = rng.bounded(5);
            let valid_to = match vt_choice {
                0 => None,
                1 => Some(vf),
                2 => Some(vf + 5),
                3 => Some(vf.saturating_sub(5)), // inverted
                _ => Some(2025),
            };
            let _ = emit_edge_result(
                &mut rt,
                s,
                r,
                o,
                vf,
                valid_to,
                Some(&format!("e{seed}-{e}")),
            );
        }

        let k = rng.bounded(5) as u32;
        let start = nodes[rng.bounded(n_nodes as u64) as usize];
        let mode = rng.bounded(3);
        let q_rels: Vec<TermId> = match mode {
            0 => vec![],
            1 => vec![rels[rng.bounded(n_rels as u64) as usize]],
            _ if k == 0 => vec![],
            _ => (0..k)
                .map(|_| rels[rng.bounded(n_rels as u64) as usize])
                .collect(),
        };
        let year = 2008 + rng.bounded(20);
        let limit = match rng.bounded(4) {
            0 => 0,
            1 => 1,
            2 => 2,
            _ => 8,
        };
        let tcut = if rng.bounded(2) == 0 {
            tt_max()
        } else {
            let last = rt
                .log()
                .as_slice()
                .last()
                .map(|e| e.ingested_at)
                .unwrap_or(0);
            tt(rng.bounded(last.saturating_add(1)))
        };

        let live = rt.live_at(tcut, vt(year));
        let mut expect = hop_oracle(&live, start, &q_rels, k);
        if limit > 0 && expect.len() > limit {
            expect.truncate(limit);
        }
        let got = exec_hops(&rt, start, q_rels.clone(), k, tcut, vt(year), limit)
            .unwrap_or_else(|e| panic!("seed={seed} hops err {e:?}"));
        assert_eq!(
            got, expect,
            "seed={seed} k={k} mode={mode} year={year} limit={limit} start hops vs live_at oracle"
        );

        let mut scan_expect = sorted_live(&rt, tcut, vt(year));
        if limit > 0 && scan_expect.len() > limit {
            scan_expect.truncate(limit);
        }
        let scan = exec_scan(&rt, tcut, vt(year), limit).expect("scan");
        assert_eq!(scan, scan_expect, "seed={seed} Scan vs sorted live_at");
    }
}
