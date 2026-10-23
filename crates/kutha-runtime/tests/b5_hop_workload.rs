//! B3-lite hop workload oracles (M157 S01). In-memory `Runtime::default()` only.

use kutha_common::{Op, TermId, TransactionTime, ValidTime};
use kutha_runtime::{QueryError, QueryIr, QueryRows, Runtime, execute, try_hops_named};
use std::collections::BTreeSet;

const T_OLD: u64 = 2015;
const T_NEW: u64 = 2021;
const VF_FIRST: u64 = 2010;
const VF_SECOND: u64 = 2020;

fn emit_edge(
    rt: &mut Runtime,
    s: TermId,
    r: TermId,
    o: TermId,
    valid_from: u64,
    valid_to: Option<u64>,
) -> kutha_common::EventId {
    rt.emit(Op::Assert {
        subject: s,
        relation: r,
        object: o,
        valid_from,
        valid_to,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap()
    .receipt
    .event_ids[0]
}

fn hop_rel(rels: &[TermId], k: u32, hop: u32) -> Option<TermId> {
    match rels.len() {
        0 => None,
        1 => Some(rels[0]),
        n if n == k as usize => Some(rels[hop as usize]),
        _ => panic!("test helper: rels/k mismatch"),
    }
}

/// Nested join over `live_at` triples; same set rule as hop end-nodes.
fn nested_join_ends(
    rt: &Runtime,
    start: TermId,
    rels: &[TermId],
    k: u32,
    tt: TransactionTime,
    vt: ValidTime,
) -> Vec<TermId> {
    let triples = rt.live_at(tt, vt);
    let mut frontier: BTreeSet<TermId> = BTreeSet::from([start]);
    for hop in 0..k {
        let want = hop_rel(rels, k, hop);
        let mut next = BTreeSet::new();
        for (s, r, o) in &triples {
            if frontier.contains(s) && want.map(|rel| *r == rel).unwrap_or(true) {
                next.insert(*o);
            }
        }
        frontier = next;
    }
    frontier.into_iter().collect()
}

fn nodes(rows: QueryRows) -> Vec<TermId> {
    match rows {
        QueryRows::Nodes(v) => v,
        other => panic!("expected Nodes, got {other:?}"),
    }
}

fn xorshift32(state: &mut u32) -> u32 {
    let mut x = *state;
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
    *state = x;
    x
}

fn shuffle<T>(rows: &mut [T], seed: u32) {
    let mut s = seed | 1;
    for i in (1..rows.len()).rev() {
        let j = (xorshift32(&mut s) as usize) % (i + 1);
        rows.swap(i, j);
    }
}

fn intern_chain(
    rt: &mut Runtime,
) -> (
    TermId,
    TermId,
    TermId,
    TermId,
    TermId,
    TermId,
    TermId,
    TermId,
) {
    (
        rt.intern("A"),
        rt.intern("B"),
        rt.intern("C"),
        rt.intern("D"),
        rt.intern("X"),
        rt.intern("Y"),
        rt.intern("Z"),
        rt.intern("relatedTo"),
    )
}

fn load_chain_branch_diamond(
    rt: &mut Runtime,
    a: TermId,
    b: TermId,
    c: TermId,
    d: TermId,
    x: TermId,
    y: TermId,
    z: TermId,
    rel: TermId,
) {
    emit_edge(rt, a, rel, b, VF_FIRST, None);
    emit_edge(rt, a, rel, b, VF_FIRST, None); // support multiplicity
    emit_edge(rt, b, rel, c, VF_FIRST, None); // chain A→B→C
    emit_edge(rt, a, rel, d, VF_FIRST, None); // branch
    emit_edge(rt, a, rel, x, VF_FIRST, None); // diamond left
    emit_edge(rt, x, rel, z, VF_FIRST, None);
    emit_edge(rt, a, rel, y, VF_FIRST, None); // diamond right
    emit_edge(rt, y, rel, z, VF_FIRST, None);
}

fn hops(start: TermId, rels: Vec<TermId>, k: u32, tt: TransactionTime, vt: ValidTime) -> QueryIr {
    QueryIr::Hops {
        start,
        rels,
        k,
        tt,
        vt,
        limit: 0,
    }
}

#[test]
fn b5_hop_scan_equals_as_of_ff5() {
    let mut rt = Runtime::default();
    let article = rt.intern("art-12-speed");
    let in_force = rt.intern("inForceAs");
    let limit_50 = rt.intern("50-kmh");
    let limit_30 = rt.intern("30-kmh");
    let statute = rt.intern("statute-x");
    let in_force_obj = rt.intern("in-force");

    emit_edge(
        &mut rt,
        article,
        in_force,
        limit_50,
        VF_FIRST,
        Some(VF_SECOND),
    );
    emit_edge(&mut rt, article, in_force, limit_30, VF_SECOND, None);
    emit_edge(&mut rt, statute, in_force, in_force_obj, VF_FIRST, None);

    for vt in [T_OLD, T_NEW] {
        let mut expected = rt.as_of(ValidTime::from(vt));
        expected.sort_unstable();
        assert!(
            !expected.is_empty(),
            "as_of({vt}) bag must be nonempty so an empty stub cannot pass"
        );
        let got = execute(
            &rt,
            &QueryIr::Scan {
                tt: TransactionTime::MAX,
                vt: ValidTime::from(vt),
                limit: 0,
            },
        )
        .unwrap();
        assert_eq!(
            got,
            QueryRows::Triples(expected),
            "Scan bag vs sorted as_of at {vt}"
        );
    }
}

#[test]
fn b5_hop_nested_join_oracle() {
    let mut rt = Runtime::default();
    let (a, b, c, d, x, y, z, rel) = intern_chain(&mut rt);
    load_chain_branch_diamond(&mut rt, a, b, c, d, x, y, z, rel);
    let tt = TransactionTime::MAX;
    let vt = ValidTime::from(T_OLD);
    let rels = vec![rel];

    let zero = nodes(execute(&rt, &hops(a, rels.clone(), 0, tt, vt)).unwrap());
    assert_eq!(zero, nested_join_ends(&rt, a, &rels, 0, tt, vt));
    assert_eq!(zero, vec![a]);

    let one = nodes(execute(&rt, &hops(a, rels.clone(), 1, tt, vt)).unwrap());
    let want_one = nested_join_ends(&rt, a, &rels, 1, tt, vt);
    assert_eq!(one, want_one);
    let mut expect_one = vec![b, d, x, y];
    expect_one.sort_unstable();
    assert_eq!(one, expect_one, "1-hop collapses support multiplicity");

    let two = nodes(execute(&rt, &hops(a, rels.clone(), 2, tt, vt)).unwrap());
    let want_two = nested_join_ends(&rt, a, &rels, 2, tt, vt);
    assert_eq!(two, want_two);
    let mut expect_two = vec![c, z];
    expect_two.sort_unstable();
    assert_eq!(two, expect_two, "2-hop chain+diamond is unique end nodes");
}

#[test]
fn b5_hop_unlabeled_equals_untyped_neighbors() {
    let mut rt = Runtime::default();
    let a = rt.intern("A");
    let b = rt.intern("B");
    let c = rt.intern("C");
    let related = rt.intern("relatedTo");
    let in_force = rt.intern("inForceAs");
    emit_edge(&mut rt, a, related, b, VF_FIRST, None);
    emit_edge(&mut rt, a, in_force, c, VF_FIRST, None);

    let tt = TransactionTime::MAX;
    let vt = ValidTime::from(T_OLD);
    let got = nodes(execute(&rt, &hops(a, vec![], 1, tt, vt)).unwrap());
    let mut neighbors: Vec<TermId> = rt.csr_lease_at(tt, vt).neighbors(a).to_vec();
    neighbors.sort_unstable();
    neighbors.dedup();
    assert_eq!(got, neighbors, "unlabeled 1-hop drops labels; neighbor set");
    assert_eq!(got, {
        let mut v = vec![b, c];
        v.sort_unstable();
        v
    });
}

#[test]
fn b5_hop_cuts_pit() {
    let mut rt = Runtime::default();
    let a = rt.intern("A");
    let b = rt.intern("B");
    let c = rt.intern("C");
    let rel = rt.intern("relatedTo");
    emit_edge(&mut rt, a, rel, b, VF_FIRST, None);
    let bc = emit_edge(&mut rt, b, rel, c, VF_FIRST, Some(VF_SECOND));
    let early_tt = TransactionTime::from(
        rt.log()
            .iter()
            .find(|e| e.id == bc)
            .expect("B-r-C event")
            .ingested_at,
    );
    let rels = vec![rel];

    let early_vt = nodes(
        execute(
            &rt,
            &hops(
                a,
                rels.clone(),
                2,
                TransactionTime::MAX,
                ValidTime::from(T_OLD),
            ),
        )
        .unwrap(),
    );
    let late_vt = nodes(
        execute(
            &rt,
            &hops(
                a,
                rels.clone(),
                2,
                TransactionTime::MAX,
                ValidTime::from(T_NEW),
            ),
        )
        .unwrap(),
    );
    assert_eq!(
        early_vt,
        vec![c],
        "2-hop at VT 2015 includes C (window open)"
    );
    assert!(
        late_vt.is_empty(),
        "2-hop at VT 2021 drops C (edition window-end)"
    );

    rt.emit(Op::Retract { event_id: bc }).unwrap();
    let after_retract = nodes(
        execute(
            &rt,
            &hops(
                a,
                rels.clone(),
                2,
                TransactionTime::MAX,
                ValidTime::from(T_OLD),
            ),
        )
        .unwrap(),
    );
    let before_retract =
        nodes(execute(&rt, &hops(a, rels, 2, early_tt, ValidTime::from(T_OLD))).unwrap());
    assert_eq!(
        before_retract,
        vec![c],
        "2-hop at TT of B-r-C still includes C"
    );
    assert!(
        after_retract.is_empty(),
        "2-hop at TT::MAX after Retract drops C"
    );
}

#[test]
fn b5_hop_fail_closed_unknown_relation() {
    let mut rt = Runtime::default();
    let log0 = rt.log().len();
    let fp0 = rt.fold().fingerprint();
    let dict0 = rt.dictionary().len();

    let ghost_start = try_hops_named(
        &rt,
        "ghost-start",
        &[],
        1,
        TransactionTime::MAX,
        ValidTime::from(T_OLD),
        0,
    )
    .unwrap_err();
    assert!(
        matches!(ghost_start, QueryError::UnknownTerm { ref name } if name == "ghost-start"),
        "{ghost_start:?}"
    );
    assert_eq!(ghost_start.to_string(), "unknown term ghost-start");
    assert_eq!(log0, rt.log().len(), "ghost start must not intern");
    assert_eq!(fp0, rt.fold().fingerprint());
    assert_eq!(dict0, rt.dictionary().len());
    assert!(rt.dictionary().id("ghost-start").is_none());

    let a = rt.intern("A");
    let related = rt.intern("relatedTo");
    let in_force = rt.intern("inForceAs");
    let log1 = rt.log().len();
    let fp1 = rt.fold().fingerprint();

    let ghost_rel = try_hops_named(
        &rt,
        "A",
        &["ghost-rel"],
        1,
        TransactionTime::MAX,
        ValidTime::from(T_OLD),
        0,
    )
    .unwrap_err();
    assert!(
        matches!(ghost_rel, QueryError::UnknownRelation { ref name } if name == "ghost-rel"),
        "{ghost_rel:?}"
    );
    assert_eq!(
        ghost_rel.to_string(),
        "unknown relation ghost-rel (not in allowlist)"
    );
    assert_eq!(log1, rt.log().len());
    assert_eq!(fp1, rt.fold().fingerprint());
    assert!(rt.dictionary().id("ghost-rel").is_none());

    let mismatch = try_hops_named(
        &rt,
        "A",
        &["relatedTo", "inForceAs"],
        1,
        TransactionTime::MAX,
        ValidTime::from(T_OLD),
        0,
    )
    .unwrap_err();
    assert!(
        matches!(mismatch, QueryError::RelsShape { rels: 2, k: 1 }),
        "{mismatch:?}"
    );
    assert_eq!(
        mismatch.to_string(),
        "rels length 2 is not 0, 1, or k=1 (unlabeled, repeated, or per-hop)"
    );
    assert_eq!(log1, rt.log().len());
    assert_eq!(fp1, rt.fold().fingerprint());

    let mismatch_exec = execute(
        &rt,
        &hops(
            a,
            vec![related, in_force],
            1,
            TransactionTime::MAX,
            ValidTime::from(T_OLD),
        ),
    )
    .unwrap_err();
    assert!(
        matches!(mismatch_exec, QueryError::RelsShape { rels: 2, k: 1 }),
        "{mismatch_exec:?}"
    );
    assert_eq!(log1, rt.log().len());
    assert_eq!(fp1, rt.fold().fingerprint());

    let oob = execute(
        &rt,
        &hops(
            TermId::MAX,
            vec![],
            1,
            TransactionTime::MAX,
            ValidTime::from(T_OLD),
        ),
    )
    .unwrap_err();
    assert!(matches!(oob, QueryError::UnknownTerm { .. }), "{oob:?}");
    assert_eq!(log1, rt.log().len());
    assert_eq!(fp1, rt.fold().fingerprint());
}

#[test]
fn b5_hop_shuffle_deterministic() {
    let names = ["A", "B", "C", "D", "E", "F"];
    let edges = [
        ("A", "B"),
        ("B", "C"),
        ("A", "D"),
        ("D", "E"),
        ("A", "F"),
        ("F", "C"),
    ];

    let mut answers: Vec<Vec<TermId>> = Vec::new();
    for seed in [1u32, 7, 99] {
        let mut rt = Runtime::default();
        for n in names {
            let _ = rt.intern(n);
        }
        let rel = rt.intern("relatedTo");
        let mut order = edges.to_vec();
        shuffle(&mut order, seed);
        for (s, o) in order {
            let sid = rt.dictionary().id(s).unwrap();
            let oid = rt.dictionary().id(o).unwrap();
            emit_edge(&mut rt, sid, rel, oid, VF_FIRST, None);
        }
        let a = rt.dictionary().id("A").unwrap();
        let tt = TransactionTime::MAX;
        let vt = ValidTime::from(T_OLD);
        let rels = vec![rel];
        let got = nodes(execute(&rt, &hops(a, rels.clone(), 2, tt, vt)).unwrap());
        let join = nested_join_ends(&rt, a, &rels, 2, tt, vt);
        assert_eq!(got, join);
        assert!(
            !got.is_empty(),
            "shuffle hop answer must be nonempty (empty stub must not pass)"
        );
        answers.push(got);
    }
    assert_eq!(answers[0], answers[1]);
    assert_eq!(answers[1], answers[2]);
}

#[test]
fn b5_hop_per_hop_rels_order() {
    let mut rt = Runtime::default();
    let a = rt.intern("A");
    let b = rt.intern("B");
    let c = rt.intern("C");
    let r1 = rt.intern("relatedTo");
    let r2 = rt.intern("inForceAs");
    emit_edge(&mut rt, a, r1, b, VF_FIRST, None);
    emit_edge(&mut rt, b, r2, c, VF_FIRST, None);

    let tt = TransactionTime::MAX;
    let vt = ValidTime::from(T_OLD);
    let right = try_hops_named(&rt, "A", &["relatedTo", "inForceAs"], 2, tt, vt, 0).unwrap();
    let wrong = try_hops_named(&rt, "A", &["inForceAs", "relatedTo"], 2, tt, vt, 0).unwrap();

    let got_right = nodes(execute(&rt, &right).unwrap());
    let join_right = nested_join_ends(&rt, a, &[r1, r2], 2, tt, vt);
    assert_eq!(got_right, join_right);
    assert_eq!(got_right, vec![c], "A-relatedTo-B-inForceAs-C");

    let got_wrong = nodes(execute(&rt, &wrong).unwrap());
    let join_wrong = nested_join_ends(&rt, a, &[r2, r1], 2, tt, vt);
    assert_eq!(got_wrong, join_wrong);
    assert!(
        got_wrong.is_empty(),
        "swapped per-hop rels must not walk the chain"
    );
}

#[test]
fn b5_hop_limit_truncates_sorted_prefix() {
    // limit == 0 is unlimited (QueryIr Scan/Hops); otherwise a prefix of the sorted rows.
    let mut rt = Runtime::default();
    let (a, b, c, d, x, y, z, rel) = intern_chain(&mut rt);
    load_chain_branch_diamond(&mut rt, a, b, c, d, x, y, z, rel);
    let tt = TransactionTime::MAX;
    let vt = ValidTime::from(T_OLD);

    let unlimited = nodes(execute(&rt, &hops(a, vec![rel], 1, tt, vt)).unwrap());
    let mut expect = vec![b, d, x, y];
    expect.sort_unstable();
    assert_eq!(unlimited, expect, "limit 0 keeps the full sorted node set");

    let limited = nodes(
        execute(
            &rt,
            &QueryIr::Hops {
                start: a,
                rels: vec![rel],
                k: 1,
                tt,
                vt,
                limit: 2,
            },
        )
        .unwrap(),
    );
    assert_eq!(
        limited,
        expect[..2],
        "limit 2 is the prefix of sorted unique ends"
    );

    let mut as_of_sorted = rt.as_of(vt);
    as_of_sorted.sort_unstable();
    assert!(
        as_of_sorted.len() > 2,
        "scan bag must be long enough to show truncation"
    );
    let scan_unlimited = execute(
        &rt,
        &QueryIr::Scan {
            tt: TransactionTime::MAX,
            vt,
            limit: 0,
        },
    )
    .unwrap();
    assert_eq!(scan_unlimited, QueryRows::Triples(as_of_sorted.clone()));
    let scan_limited = execute(
        &rt,
        &QueryIr::Scan {
            tt: TransactionTime::MAX,
            vt,
            limit: 2,
        },
    )
    .unwrap();
    assert_eq!(
        scan_limited,
        QueryRows::Triples(as_of_sorted[..2].to_vec()),
        "Scan limit is a prefix of the sorted bag"
    );
}
