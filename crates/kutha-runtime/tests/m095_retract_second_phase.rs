//! M095 S01 / Phase 166 — second-phase retract scan must not walk all live_behaviors
//! (CE whole-repo 2026-10-15 #3).

use kutha_common::Op;
use kutha_runtime::Runtime;

/// Retracting an unrelated decoy must not examine O(D) live Horn heads of another relation.
#[test]
fn retract_second_phase_examines_constant_in_unrelated_live_d() {
    let mut counts = Vec::new();
    for &d in &[32u32, 128] {
        let mut rt = Runtime::default();
        for i in 0..d {
            let s = rt.intern(&format!("Alice-{i}"));
            let o = rt.intern(&format!("Bob-{i}"));
            let knows = rt.intern("knows");
            rt.emit(Op::Assert {
                subject: s,
                relation: knows,
                object: o,
                valid_from: 2010,
                valid_to: None,
                claim: None,
                delivery_key: None,
                polarity: None,
            })
            .unwrap();
        }
        let live_d = rt
            .log()
            .iter()
            .filter(|e| matches!(e.op, Op::Behavior { .. }))
            .count();
        assert!(
            live_d >= d as usize,
            "expected ≥{d} inverse_knows Behaviors, got {live_d}"
        );

        let decoy_s = rt.intern("decoy-s");
        let decoy_o = rt.intern("decoy-o");
        let decoy_rel = rt.intern("inForceAs");
        let decoy = rt
            .emit(Op::Assert {
                subject: decoy_s,
                relation: decoy_rel,
                object: decoy_o,
                valid_from: 2010,
                valid_to: None,
                claim: None,
                delivery_key: None,
                polarity: None,
            })
            .unwrap()
            .receipt
            .event_ids[0];

        rt.reset_retract_examine_count();
        rt.emit(Op::Retract { event_id: decoy }).unwrap();
        counts.push(rt.retract_examine_count());
    }
    assert!(
        counts[1] <= counts[0] + 8,
        "second-phase retract must not grow with unrelated live D: {counts:?}"
    );
    assert!(
        counts[1] < 32,
        "second-phase retract of unrelated decoy must stay bounded: {counts:?}"
    );
}

/// Non-trigger multi-premise retract must still drop the derived head after indexing.
#[test]
fn retract_non_trigger_still_drops_after_premise_rel_index() {
    use kutha_common::{RuleAst, RuleAtom, RuleTerm};

    let v = |i| RuleTerm::var(i).unwrap();
    let atom = |rel: &str, s, o| RuleAtom::new(rel, s, o).unwrap();
    let horn = RuleAst::horn(
        atom("relatedTo", v(0), v(2)),
        vec![atom("relatedTo", v(0), v(1)), atom("relatedTo", v(1), v(2))],
    )
    .unwrap();

    let mut rt = Runtime::default();
    rt.emit_rule_ast(&horn, 2010, None).unwrap();
    let a = rt.intern("A");
    let b = rt.intern("B");
    let c = rt.intern("C");
    let rel = rt.intern("relatedTo");

    let prem_ab = rt
        .emit(Op::Assert {
            subject: a,
            relation: rel,
            object: b,
            valid_from: 2010,
            valid_to: None,
            claim: None,
            delivery_key: None,
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
        delivery_key: None,
        polarity: None,
    })
    .unwrap();

    assert!(
        rt.fold().facts().iter().any(|f| {
            f.subject == a && f.relation == rel && f.object() == c && f.invalidated_at.is_none()
        }),
        "derived relatedTo(A,C) must be live before non-trigger retract"
    );

    rt.emit(Op::Retract { event_id: prem_ab }).unwrap();

    assert!(
        !rt.fold().facts().iter().any(|f| {
            f.subject == a && f.relation == rel && f.object() == c && f.invalidated_at.is_none()
        }),
        "retracting non-trigger premise AB must still drop derived AC"
    );
}
