//! M136 S01 / Phase 210 — N derived + N retracts must not grow retract_examine
//! quadratically (CE 2026-10-15 testing gap; full live_behaviors / same-rel
//! sibling scan would).

use kutha_common::Op;
use kutha_runtime::Runtime;

fn seed_knows_chain(n: u32) -> (Runtime, Vec<kutha_common::EventId>) {
    let mut rt = Runtime::default();
    let mut premises = Vec::with_capacity(n as usize);
    for i in 0..n {
        let s = rt.intern(&format!("Alice-{i}"));
        let o = rt.intern(&format!("Bob-{i}"));
        let knows = rt.intern("knows");
        let id = rt
            .emit(Op::Assert {
                subject: s,
                relation: knows,
                object: o,
                valid_from: 2010,
                valid_to: None,
                claim: None,
                delivery_key: None,
                polarity: None,
            })
            .unwrap()
            .receipt
            .event_ids[0];
        premises.push(id);
    }
    let derived = rt
        .log()
        .iter()
        .filter(|e| matches!(e.op, Op::Behavior { .. }))
        .count();
    assert!(
        derived >= n as usize,
        "expected ≥{n} inverse_knows Behaviors, got {derived}"
    );
    (rt, premises)
}

#[test]
fn retract_n_derived_examines_grow_near_linear_not_quadratic() {
    let mut totals = Vec::new();
    for &n in &[16u32, 64] {
        let (mut rt, premises) = seed_knows_chain(n);
        rt.reset_retract_examine_count();
        for prem in premises {
            rt.emit(Op::Retract { event_id: prem }).unwrap();
        }
        totals.push(rt.retract_examine_count());
    }
    let small = totals[0];
    let large = totals[1];
    // 64/16 = 4× inputs. Near-linear ≤ ~6×; quadratic ≈ 16×.
    assert!(
        large <= small.saturating_mul(6),
        "N retracts over N derived must stay near-linear in retract_examine: \
         n=16 → {small}, n=64 → {large} (ratio {})",
        large as f64 / small.max(1) as f64
    );
    assert!(
        large < small.saturating_mul(12),
        "must not approach quadratic growth: n=16 → {small}, n=64 → {large}"
    );
}
