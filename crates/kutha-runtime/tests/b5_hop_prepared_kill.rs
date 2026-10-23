//! Independent verifier kill-tests for the phase-231 prepare/run split.
//! Snapshot vs live `execute`, `PreparedHops::run_from`, shape errors at
//! `prepare`. Oracle: exact-k unique ends over `live_at` triples.
//! Does not edit `b5_hop_workload_kill.rs`. In-memory only.

use kutha_common::{Op, TermId, TransactionTime, ValidTime};
use kutha_runtime::{
    execute, prepare, Prepared, PreparedHops, QueryError, QueryIr, QueryRows, Runtime,
};
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

fn tt_max() -> TransactionTime {
    TransactionTime::MAX
}

fn vt(y: u64) -> ValidTime {
    ValidTime::from(y)
}

fn emit_edge(rt: &mut Runtime, s: TermId, r: TermId, o: TermId, key: &str) {
    rt.emit(Op::Assert {
        subject: s,
        relation: r,
        object: o,
        valid_from: 2010,
        valid_to: None,
        claim: None,
        delivery_key: Some(key.into()),
        polarity: None,
    })
    .expect("product Assert must admit");
}

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

fn hops_ir(start: TermId, rels: Vec<TermId>, k: u32, limit: usize) -> QueryIr {
    QueryIr::Hops {
        start,
        rels,
        k,
        tt: tt_max(),
        vt: vt(2010),
        limit,
    }
}

fn nodes(rows: QueryRows) -> Vec<TermId> {
    match rows {
        QueryRows::Nodes(n) => n,
        other => panic!("expected Nodes, got {other:?}"),
    }
}

fn triples(rows: QueryRows) -> Vec<(TermId, TermId, TermId)> {
    match rows {
        QueryRows::Triples(t) => t,
        other => panic!("expected Triples, got {other:?}"),
    }
}

fn as_hops(p: Prepared) -> PreparedHops {
    match p {
        Prepared::Hops(h) => h,
        Prepared::Scan(_) => panic!("expected Prepared::Hops"),
    }
}

fn snap(rt: &Runtime) -> (usize, usize, usize) {
    (
        rt.log().len(),
        rt.dictionary().len(),
        rt.fold().facts().len(),
    )
}

#[test]
fn kill_prepared_hops_frozen_after_assert_and_retract() {
    let mut rt = Runtime::default();
    let rel = rt.intern("relatedTo");
    let a = rt.intern("ps-A");
    let b = rt.intern("ps-B");
    let c = rt.intern("ps-C");
    emit_edge(&mut rt, a, rel, b, "ab");
    let q = hops_ir(a, vec![rel], 1, 0);
    let prepared = prepare(&rt, &q).expect("prepare");
    let before = nodes(prepared.run());
    assert_eq!(before, vec![b]);
    assert_eq!(before, nodes(execute(&rt, &q).expect("execute at cut")));

    emit_edge(&mut rt, a, rel, c, "ac");
    assert_eq!(
        nodes(prepared.run()),
        vec![b],
        "Prepared taken before Assert must keep the old cut"
    );
    let mut live = nodes(execute(&rt, &q).expect("execute after assert"));
    live.sort();
    assert_eq!(live, {
        let mut v = vec![b, c];
        v.sort();
        v
    });
    let triples_now = rt.live_at(tt_max(), vt(2010));
    assert_eq!(
        live,
        hop_oracle(&triples_now, a, &[rel], 1),
        "live execute must match live_at oracle after Assert"
    );

    let ab = rt
        .fold()
        .facts()
        .iter()
        .find(|f| f.subject == a && f.object() == b && f.invalidated_at.is_none())
        .unwrap()
        .event_id;
    rt.emit(Op::Retract { event_id: ab }).unwrap();
    assert_eq!(
        nodes(prepared.run()),
        vec![b],
        "Prepared must still answer B after Retract"
    );
    assert_eq!(
        nodes(execute(&rt, &q).expect("execute after retract")),
        vec![c],
        "execute after Retract must see the new fold"
    );
}

#[test]
fn kill_prepared_scan_frozen_equals_execute_at_cut() {
    let mut rt = Runtime::default();
    let rel = rt.intern("relatedTo");
    let a = rt.intern("sc-A");
    let b = rt.intern("sc-B");
    let c = rt.intern("sc-C");
    emit_edge(&mut rt, a, rel, b, "ab");
    let q = QueryIr::Scan {
        tt: tt_max(),
        vt: vt(2010),
        limit: 0,
    };
    let prepared = prepare(&rt, &q).expect("prepare scan");
    let at_cut = triples(prepared.run());
    assert_eq!(at_cut, triples(execute(&rt, &q).expect("execute scan")));
    let mut expect = rt.live_at(tt_max(), vt(2010));
    expect.sort_unstable();
    assert_eq!(at_cut, expect);

    emit_edge(&mut rt, a, rel, c, "ac");
    assert_eq!(
        triples(prepared.run()),
        at_cut,
        "PreparedScan must not live-update"
    );
    let after = triples(execute(&rt, &q).expect("scan after"));
    assert_ne!(after, at_cut, "execute Scan must see the new triple");
    assert!(after.contains(&(a, rel, c)));
}

#[test]
fn kill_run_from_reuse_equals_per_start_execute_and_oracle() {
    let mut rt = Runtime::default();
    let rel = rt.intern("relatedTo");
    let a = rt.intern("rf-A");
    let b = rt.intern("rf-B");
    let c = rt.intern("rf-C");
    let d = rt.intern("rf-D");
    emit_edge(&mut rt, a, rel, b, "ab");
    emit_edge(&mut rt, b, rel, c, "bc");
    emit_edge(&mut rt, a, rel, d, "ad");
    emit_edge(&mut rt, d, rel, c, "dc");
    let template = hops_ir(a, vec![rel], 2, 0);
    let hops = as_hops(prepare(&rt, &template).expect("prepare"));
    let live = rt.live_at(tt_max(), vt(2010));
    for start in [a, b, c, d] {
        let via_lease = nodes(hops.run_from(start).expect("run_from"));
        let via_exec = nodes(execute(&rt, &hops_ir(start, vec![rel], 2, 0)).expect("execute"));
        let via_oracle = hop_oracle(&live, start, &[rel], 2);
        assert_eq!(via_lease, via_exec, "run_from({start}) vs execute");
        assert_eq!(via_lease, via_oracle, "run_from({start}) vs live_at oracle");
    }
    assert_eq!(nodes(hops.run()), nodes(hops.run_from(a).unwrap()));
}

#[test]
fn kill_run_from_unknown_out_of_range_and_not_in_lease() {
    let mut rt = Runtime::default();
    let rel = rt.intern("relatedTo");
    let a = rt.intern("ur-A");
    let b = rt.intern("ur-B");
    let isolated = rt.intern("ur-isolated");
    emit_edge(&mut rt, a, rel, b, "ab");
    let hops = as_hops(prepare(&rt, &hops_ir(a, vec![rel], 1, 0)).expect("prepare"));

    match hops.run_from(TermId::MAX) {
        Err(QueryError::UnknownTerm { .. }) => {}
        other => panic!("out of range must be UnknownTerm, got {other:?}"),
    }
    let dict_len = rt.dictionary().len() as TermId;
    match hops.run_from(dict_len) {
        Err(QueryError::UnknownTerm { name }) => {
            assert_eq!(name, dict_len.to_string());
        }
        other => panic!("dict_len boundary must be UnknownTerm, got {other:?}"),
    }

    let isolated_got = nodes(hops.run_from(isolated).expect("interned, no edges"));
    assert!(
        isolated_got.is_empty(),
        "start in dict but not a CSR subject must yield empty ends, not an error: {isolated_got:?}"
    );
    assert_eq!(
        isolated_got,
        nodes(execute(&rt, &hops_ir(isolated, vec![rel], 1, 0)).unwrap())
    );

    let later = rt.intern("ur-after-prepare");
    match hops.run_from(later) {
        Err(QueryError::UnknownTerm { .. }) => {}
        other => panic!("interned-after-prepare must fail closed, got {other:?}"),
    }
    emit_edge(&mut rt, later, rel, b, "later");
    let live_later = nodes(execute(&rt, &hops_ir(later, vec![rel], 1, 0)).expect("live later"));
    assert_eq!(
        live_later,
        vec![b],
        "execute after intern+assert must see the new start"
    );

    let outcome = catch_unwind(AssertUnwindSafe(|| hops.run_from(TermId::MAX)));
    assert!(outcome.is_ok(), "run_from must not panic on TermId::MAX");
}

#[test]
fn kill_run_from_limit_is_sorted_prefix() {
    let mut rt = Runtime::default();
    let rel = rt.intern("relatedTo");
    let a = rt.intern("lm-A");
    let starts_b = rt.intern("lm-S");
    let mut objs = Vec::new();
    for name in ["lm-Z", "lm-M", "lm-B"] {
        let o = rt.intern(name);
        emit_edge(&mut rt, a, rel, o, name);
        emit_edge(&mut rt, starts_b, rel, o, &format!("s-{name}"));
        objs.push(o);
    }
    objs.sort();
    let hops = as_hops(prepare(&rt, &hops_ir(a, vec![rel], 1, 2)).expect("prepare limit 2"));
    let from_a = nodes(hops.run_from(a).expect("from a"));
    let from_s = nodes(hops.run_from(starts_b).expect("from s"));
    assert_eq!(
        from_a,
        objs[..2],
        "run_from uses prepared limit on sorted ends"
    );
    assert_eq!(from_s, objs[..2]);
    assert_eq!(
        from_s,
        nodes(execute(&rt, &hops_ir(starts_b, vec![rel], 1, 2)).unwrap())
    );
    let unlimited = as_hops(prepare(&rt, &hops_ir(a, vec![rel], 1, 0)).expect("limit 0"));
    assert_eq!(nodes(unlimited.run_from(a).unwrap()), objs);
}

#[test]
fn kill_rels_shape_errors_at_prepare_not_run() {
    let mut rt = Runtime::default();
    let r1 = rt.intern("relatedTo");
    let r2 = rt.intern("inForceAs");
    let a = rt.intern("sh-A");
    let b = rt.intern("sh-B");
    emit_edge(&mut rt, a, r1, b, "ab");
    let q = hops_ir(a, vec![r1, r2], 3, 0);
    match prepare(&rt, &q) {
        Err(QueryError::RelsShape { rels: 2, k: 3 }) => {}
        other => panic!("shape must fail at prepare, got {other:?}"),
    }
    match execute(&rt, &q) {
        Err(QueryError::RelsShape { rels: 2, k: 3 }) => {}
        other => panic!("execute is prepare+run, got {other:?}"),
    }
    let before = snap(&rt);
    let _ = prepare(&rt, &q);
    assert_eq!(
        before,
        snap(&rt),
        "failed prepare must not intern or append"
    );

    let ok = as_hops(prepare(&rt, &hops_ir(a, vec![r1], 1, 0)).unwrap());
    let _ = ok.run();
    assert_eq!(before, snap(&rt), "run must not grow log/dict/fold");
}

#[test]
fn kill_k0_run_from_and_scan_limit_snapshot() {
    let mut rt = Runtime::default();
    let rel = rt.intern("relatedTo");
    let a = rt.intern("k0-A");
    let b = rt.intern("k0-B");
    emit_edge(&mut rt, a, rel, b, "ab");
    let hops = as_hops(prepare(&rt, &hops_ir(a, vec![rel], 0, 0)).expect("k=0"));
    assert_eq!(nodes(hops.run()), vec![a]);
    assert_eq!(nodes(hops.run_from(b).unwrap()), vec![b]);
    match hops.run_from(TermId::MAX) {
        Err(QueryError::UnknownTerm { .. }) => {}
        other => panic!("k=0 out of range, got {other:?}"),
    }

    let scan_q = QueryIr::Scan {
        tt: tt_max(),
        vt: vt(2010),
        limit: 1,
    };
    let prepared = prepare(&rt, &scan_q).unwrap();
    let limited = triples(prepared.run());
    let mut all = rt.live_at(tt_max(), vt(2010));
    all.sort_unstable();
    assert_eq!(limited, all[..1]);
    assert_eq!(limited, triples(execute(&rt, &scan_q).unwrap()));
}

#[test]
fn kill_property_run_from_vs_execute_vs_oracle() {
    let cases = kill_case_count();
    let allowed = ["relatedTo", "inForceAs", "observed"];
    for seed in 1..=cases {
        let mut rng = Rng(0xA5A5_5A5A_F00D ^ seed.wrapping_mul(0x9E37));
        let mut rt = Runtime::default();
        let n_nodes = 3 + rng.bounded(4) as usize;
        let nodes_ids: Vec<TermId> = (0..n_nodes)
            .map(|i| rt.intern(&format!("pr{seed}-n{i}")))
            .collect();
        let rels: Vec<TermId> = allowed.iter().map(|n| rt.intern(n)).collect();
        for e in 0..(4 + rng.bounded(8) as usize) {
            let s = nodes_ids[rng.bounded(n_nodes as u64) as usize];
            let r = rels[rng.bounded(rels.len() as u64) as usize];
            let o = nodes_ids[rng.bounded(n_nodes as u64) as usize];
            let _ = rt.emit(Op::Assert {
                subject: s,
                relation: r,
                object: o,
                valid_from: 2010,
                valid_to: None,
                claim: None,
                delivery_key: Some(format!("pr{seed}-{e}")),
                polarity: None,
            });
        }
        let k = 1 + rng.bounded(3) as u32;
        let rel = vec![rels[rng.bounded(rels.len() as u64) as usize]];
        let limit = match rng.bounded(3) {
            0 => 0,
            1 => 1,
            _ => 2,
        };
        let hops =
            as_hops(prepare(&rt, &hops_ir(nodes_ids[0], rel.clone(), k, limit)).expect("prepare"));
        let live = rt.live_at(tt_max(), vt(2010));
        for &start in &nodes_ids {
            let mut expect = hop_oracle(&live, start, &rel, k);
            if limit > 0 && expect.len() > limit {
                expect.truncate(limit);
            }
            let got = nodes(hops.run_from(start).expect("run_from"));
            let via_ex = nodes(execute(&rt, &hops_ir(start, rel.clone(), k, limit)).unwrap());
            assert_eq!(got, expect, "seed={seed} start run_from vs oracle");
            assert_eq!(got, via_ex, "seed={seed} run_from vs execute");
        }
    }
}
