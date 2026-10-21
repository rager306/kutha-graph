//! M139 S01 / Phase 213 — multi-premise same-rel retract_examine must stay
//! near-linear via premise-fact index (not relation-bucket phase 2).

use kutha_common::{Op, RuleAst, RuleAtom, RuleTerm};
use kutha_runtime::Runtime;

fn two_prem_horn() -> RuleAst {
    let v = |i| RuleTerm::var(i).unwrap();
    let atom = |rel: &str, s, o| RuleAtom::new(rel, s, o).unwrap();
    RuleAst::horn(
        atom("relatedTo", v(0), v(2)),
        vec![atom("relatedTo", v(0), v(1)), atom("relatedTo", v(1), v(2))],
    )
    .unwrap()
}

fn seed_chains(n: u32) -> (Runtime, Vec<kutha_common::EventId>) {
    let mut rt = Runtime::default();
    rt.emit_rule_ast(&two_prem_horn(), 2000, None).unwrap();
    let rel = rt.intern("relatedTo");
    let mut abs = Vec::with_capacity(n as usize);
    for i in 0..n {
        let a = rt.intern(&format!("A-{i}"));
        let b = rt.intern(&format!("B-{i}"));
        let c = rt.intern(&format!("C-{i}"));
        let ab = rt
            .emit(Op::Assert {
                subject: a,
                relation: rel,
                object: b,
                valid_from: 2010,
                valid_to: None,
                claim: None,
                delivery_key: Some(format!("ab-{i}")),
                polarity: None,
            })
            .unwrap()
            .receipt
            .event_ids[0];
        rt.emit(Op::Assert {
            subject: b,
            relation: rel,
            object: c,
            valid_from: 2010,
            valid_to: None,
            claim: None,
            delivery_key: Some(format!("bc-{i}")),
            polarity: None,
        })
        .unwrap();
        abs.push(ab);
    }
    let derived = rt
        .log()
        .iter()
        .filter(|e| match &e.op {
            Op::Behavior { relation, .. } => *relation == rel,
            _ => false,
        })
        .count();
    assert!(
        derived >= n as usize,
        "expected ≥{n} AC Behaviors, got {derived}"
    );
    (rt, abs)
}

#[test]
fn retract_n_multiprem_examines_grow_near_linear_not_quadratic() {
    let mut totals = Vec::new();
    for &n in &[8u32, 32] {
        let (mut rt, abs) = seed_chains(n);
        rt.reset_retract_examine_count();
        for ab in abs {
            rt.emit(Op::Retract { event_id: ab }).unwrap();
        }
        totals.push(rt.retract_examine_count());
        eprintln!("n={n} retract_examine={}", totals.last().unwrap());
    }
    let small = totals[0];
    let large = totals[1];
    let ratio = large as f64 / small.max(1) as f64;
    eprintln!("ratio 32/8 = {ratio} (linear~4, quadratic~16)");
    assert!(
        large <= small.saturating_mul(6),
        "multi-premise N retracts must stay near-linear: n=8 → {small}, n=32 → {large} (ratio {ratio})"
    );
    assert!(
        large < small.saturating_mul(12),
        "must not approach quadratic growth: n=8 → {small}, n=32 → {large}"
    );
}
