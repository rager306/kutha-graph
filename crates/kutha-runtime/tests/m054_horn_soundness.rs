//! M054 S01 / Phase 117 — Horn soundness (CE whole-repo #4 #3 #2 #5).

use kutha_common::{Op, RuleAst, RuleAtom, RuleTerm, TransactionTime, ValidTime, WallTime};
use kutha_runtime::{NormView, Runtime};

fn two_prem_horn() -> RuleAst {
    let v = |i| RuleTerm::var(i).unwrap();
    let atom = |rel: &str, s, o| RuleAtom::new(rel, s, o).unwrap();
    RuleAst::horn(
        atom("relatedTo", v(0), v(2)),
        vec![atom("relatedTo", v(0), v(1)), atom("relatedTo", v(1), v(2))],
    )
    .unwrap()
}

fn live_triple(
    rt: &Runtime,
    s: kutha_common::TermId,
    r: kutha_common::TermId,
    o: kutha_common::TermId,
    vt: u64,
) -> bool {
    rt.fold()
        .facts()
        .iter()
        .any(|f| f.subject == s && f.relation == r && f.object() == o && f.is_live_at(u64::MAX, vt))
}

fn live_known_by(rt: &Runtime, s: kutha_common::TermId, o: kutha_common::TermId, vt: u64) -> bool {
    let kb = rt.dictionary().id("knownBy").expect("knownBy");
    live_triple(rt, s, kb, o, vt)
}

fn live_fact_vt(
    rt: &Runtime,
    s: kutha_common::TermId,
    r: kutha_common::TermId,
    o: kutha_common::TermId,
) -> Option<(u64, Option<u64>)> {
    rt.fold().facts().iter().find_map(|f| {
        if f.subject == s && f.relation == r && f.object() == o && f.invalidated_at.is_none() {
            Some((f.valid_from, f.valid_to))
        } else {
            None
        }
    })
}

/// #4 — Correct-as-caused_by must stay derivation_eligible when replacement premise live.
#[test]
fn correct_premise_refire_is_derivation_eligible() {
    let mut rt = Runtime::default();
    let alice = rt.intern("Alice");
    let bob = rt.intern("Bob");
    let knows = rt.intern("knows");
    let kb = rt.dictionary().id("knownBy").expect("knownBy");

    let prem = rt
        .emit(Op::Assert {
            subject: alice,
            relation: knows,
            object: bob,
            valid_from: 2010,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap()
        .receipt
        .event_ids[0];

    let correct_id = rt
        .emit(Op::Correct {
            event_id: prem,
            object: bob,
            valid_from: 2010,
            valid_to: Some(2030),
        })
        .unwrap()
        .receipt
        .event_ids[0];
    rt.emit_tt_anchor(2017).unwrap();

    let inv = rt
        .log()
        .iter()
        .filter(|e| {
            matches!(
                &e.op,
                Op::Behavior {
                    caused_by,
                    relation,
                    ..
                } if caused_by == &correct_id && relation == &kb
            )
        })
        .last()
        .expect("Correct must re-fire inverse with caused_by=Correct")
        .id;

    assert!(
        live_known_by(&rt, bob, alice, 2017),
        "as_of path: inverse fold-live after Correct"
    );
    assert!(
        rt.derivation_eligible_at(inv, TransactionTime::MAX, ValidTime::from(2017)),
        "Correct-caused re-fire must be derivation_eligible when premise+pin live"
    );

    match rt.norm_view(ValidTime::from(2017), WallTime::from(2017)) {
        NormView::Ok(triples) => {
            assert!(
                triples.iter().any(|t| t.subject == bob
                    && t.relation == kb
                    && t.object == alice
                    && t.eligible),
                "norm_view must present eligible knownBy after Correct re-fire"
            );
        }
        other => panic!("expected Ok after Correct re-fire, got {other:?}"),
    }
}

/// #3 — Retracting the non-trigger premise must drop the multi-premise derived head.
#[test]
fn retract_non_trigger_premise_drops_derived() {
    let mut rt = Runtime::default();
    rt.emit_rule_ast(&two_prem_horn(), 2010, None).unwrap();
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
    let prem_bc = rt
        .emit(Op::Assert {
            subject: b,
            relation: rel,
            object: c,
            valid_from: 2010,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap()
        .receipt
        .event_ids[0];

    let derived = rt
        .log()
        .iter()
        .find(|e| {
            matches!(
                &e.op,
                Op::Behavior {
                    subject,
                    relation,
                    object,
                    caused_by,
                    ..
                } if subject == &a && relation == &rel && object == &c && caused_by == &prem_bc
            )
        })
        .expect("derived relatedTo(A,C) caused_by=trigger BC")
        .id;
    assert!(live_triple(&rt, a, rel, c, 2017));

    rt.emit(Op::Retract { event_id: prem_ab }).unwrap();

    assert!(
        !live_triple(&rt, a, rel, c, 2017),
        "retracting non-trigger premise AB must drop derived AC"
    );
    assert!(
        rt.fold()
            .facts()
            .iter()
            .any(|f| f.event_id == derived && f.invalidated_at.is_some()),
        "derived Behavior fact must be invalidated"
    );
}

/// #3 — Correct of a live premise must emit one Retract of the old head, not a
/// DET-02-duplicate copy when the cascade later re-validates that same head.
#[test]
fn correct_cascade_does_not_duplicate_derived_retract() {
    let mut rt = Runtime::default();
    let alice = rt.intern("Alice");
    let bob = rt.intern("Bob");
    let carol = rt.intern("Carol");
    let knows = rt.intern("knows");

    let prem = rt
        .emit(Op::Assert {
            subject: alice,
            relation: knows,
            object: bob,
            valid_from: 2011,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap()
        .receipt
        .event_ids[0];

    let old_inv = rt
        .log()
        .iter()
        .find(|e| {
            matches!(
                &e.op,
                Op::Behavior { caused_by, .. } if caused_by == &prem
            )
        })
        .expect("knows Assert must fire inverse_knows")
        .id;

    rt.emit(Op::Correct {
        event_id: prem,
        object: carol,
        valid_from: 2013,
        valid_to: None,
    })
    .unwrap();

    let retracts = rt
        .log()
        .iter()
        .filter(|e| matches!(&e.op, Op::Retract { event_id } if *event_id == old_inv))
        .count();
    assert_eq!(
        retracts, 1,
        "Correct cascade must retract the old inverse once (got {retracts})"
    );
}

/// #2 — Head VT must be the conjunction of premise windows (not the trigger alone).
#[test]
fn horn_head_vt_is_conjunction_of_premises() {
    let mut rt = Runtime::default();
    rt.emit_rule_ast(&two_prem_horn(), 2010, None).unwrap();
    let a = rt.intern("A");
    let b = rt.intern("B");
    let c = rt.intern("C");
    let rel = rt.intern("relatedTo");

    rt.emit(Op::Assert {
        subject: a,
        relation: rel,
        object: b,
        valid_from: 2010,
        valid_to: Some(2015),
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    rt.emit(Op::Assert {
        subject: b,
        relation: rel,
        object: c,
        valid_from: 2012,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();

    assert!(
        live_triple(&rt, a, rel, c, 2013),
        "intersection [2012,2015) is live at 2013"
    );
    assert!(
        !live_triple(&rt, a, rel, c, 2016),
        "must not inherit trigger open-ended VT past expired premise"
    );
    assert_eq!(
        live_fact_vt(&rt, a, rel, c),
        Some((2012, Some(2015))),
        "derived Behavior VT must be premise conjunction"
    );
}

/// #5 — Partial overlap must emit uncovered VT residual (full cover still suppresses).
#[test]
fn overlap_dedup_emits_uncovered_vt_residual() {
    let mut rt = Runtime::default();
    let alice = rt.intern("Alice");
    let bob = rt.intern("Bob");
    let knows = rt.intern("knows");
    let kb = rt.dictionary().id("knownBy").expect("knownBy");

    rt.emit(Op::Assert {
        subject: alice,
        relation: knows,
        object: bob,
        valid_from: 2010,
        valid_to: Some(2020),
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    assert!(live_known_by(&rt, bob, alice, 2012));

    rt.emit(Op::Assert {
        subject: alice,
        relation: knows,
        object: bob,
        valid_from: 2005,
        valid_to: Some(2015),
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();

    assert!(
        live_known_by(&rt, bob, alice, 2007),
        "uncovered residual [2005,2010) must mint inverse coverage"
    );
    assert!(
        live_known_by(&rt, bob, alice, 2012),
        "original window coverage remains"
    );

    let live_derived = rt
        .fold()
        .facts()
        .iter()
        .filter(|f| {
            f.subject == bob
                && f.relation == kb
                && f.object() == alice
                && f.invalidated_at.is_none()
        })
        .count();
    assert!(
        live_derived >= 2,
        "partial overlap must keep original plus uncovered residual (got {live_derived})"
    );
}
