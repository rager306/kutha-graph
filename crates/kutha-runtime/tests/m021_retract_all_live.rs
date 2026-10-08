//! M021 S02 / Phase 69 — Retract(event_id) clears every live residual for that id.

use kutha_common::{ValidTime};
use kutha_common::Op;
use kutha_runtime::Runtime;

/// RED→GREEN: CorrectInterval leaves three live rows under the CI event id;
/// one Retract must clear all three VT windows.
#[test]
fn retract_correct_interval_event_id_clears_all_residuals() {
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
            valid_from: 2010,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap()
        .receipt
        .event_ids[0];

    let ci_out = rt
        .emit(Op::CorrectInterval {
            event_id: prem,
            object: carol,
            patch_from: 2015,
            patch_to: Some(2020),
        })
        .unwrap();
    let ci = rt
        .log()
        .iter()
        .filter_map(|e| match &e.op {
            Op::CorrectInterval { .. } => Some(e.id),
            _ => None,
        })
        .last()
        .expect("CorrectInterval event in log");
    assert!(
        ci_out.receipt.event_ids.contains(&ci),
        "CI event id must be in the quantum receipt"
    );

    let triple_bob = (alice, knows, bob);
    let triple_carol = (alice, knows, carol);
    assert!(
        rt.as_of(ValidTime::from(2012)).contains(&triple_bob),
        "prefix residual live before retract"
    );
    assert!(
        rt.as_of(ValidTime::from(2017)).contains(&triple_carol),
        "patch residual live before retract"
    );
    assert!(
        rt.as_of(ValidTime::from(2021)).contains(&triple_bob),
        "suffix residual live before retract"
    );

    let live_before = rt
        .fold()
        .facts()
        .iter()
        .filter(|f| f.event_id == ci && f.invalidated_at.is_none())
        .count();
    assert_eq!(live_before, 3, "CI mints three live residuals");

    rt.emit(Op::Retract { event_id: ci }).unwrap();

    let live_after = rt
        .fold()
        .facts()
        .iter()
        .filter(|f| f.event_id == ci && f.invalidated_at.is_none())
        .count();
    assert_eq!(live_after, 0, "Retract must invalidate all CI residuals");

    assert!(
        !rt.as_of(ValidTime::from(2012)).contains(&triple_bob),
        "Retract(CI id) must clear prefix residual"
    );
    assert!(
        !rt.as_of(ValidTime::from(2017)).contains(&triple_carol),
        "Retract(CI id) must clear patch residual"
    );
    assert!(
        !rt.as_of(ValidTime::from(2021)).contains(&triple_bob),
        "Retract(CI id) must clear suffix residual"
    );
}

#[test]
fn retract_assert_event_id_still_clears_single_row() {
    let mut rt = Runtime::default();
    let alice = rt.intern("Alice");
    let bob = rt.intern("Bob");
    let knows = rt.intern("knows");
    let id = rt
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
    let triple = (alice, knows, bob);
    assert!(rt.as_of(ValidTime::from(2010)).contains(&triple));
    rt.emit(Op::Retract { event_id: id }).unwrap();
    assert!(!rt.as_of(ValidTime::from(2010)).contains(&triple));
}
