//! M012 S05: two-hop derivation eligibility (DER-01, DER-02).
//! Walks `caused_by`; not a MATCH compiler or provenance-polynomial evaluator.

use kutha_common::{rule_definition_hash, EventId, Op};
use kutha_runtime::Runtime;

fn register_rule_id(rt: &Runtime, definition: &str) -> EventId {
    rt.log()
        .iter()
        .find(|e| matches!(&e.op, Op::RegisterRule { definition: d, .. } if d == definition))
        .map(|e| e.id)
        .unwrap_or_else(|| panic!("RegisterRule {definition}"))
}

fn emit_two_hop_chain(rt: &mut Runtime) -> (EventId, EventId, EventId) {
    let p = rt.intern("P");
    let q = rt.intern("Q");
    let r = rt.intern("R");
    let rel = rt.intern("relatedTo");
    let true_ = rt.intern("true");

    rt.emit(Op::RegisterRule {
        definition: "derive_pq".into(),
        valid_from: 2010,
        valid_to: None,
    })
    .unwrap();
    rt.emit(Op::RegisterRule {
        definition: "derive_qr".into(),
        valid_from: 2010,
        valid_to: None,
    })
    .unwrap();

    let asserted = rt
        .emit(Op::Assert {
            subject: p,
            relation: rel,
            object: true_,
            valid_from: 2010,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap();
    let cause_p = asserted.receipt.event_ids[0];

    let hop1_out = rt
        .emit(Op::Behavior {
            name: "derive_pq".into(),
            caused_by: cause_p,
            rule_version: rule_definition_hash("derive_pq"),
            subject: q,
            relation: rel,
            object: true_,
            valid_from: 2010,
            valid_to: None,
        })
        .unwrap();
    let hop1 = hop1_out.receipt.event_ids[0];

    let hop2_out = rt
        .emit(Op::Behavior {
            name: "derive_qr".into(),
            caused_by: hop1,
            rule_version: rule_definition_hash("derive_qr"),
            subject: r,
            relation: rel,
            object: true_,
            valid_from: 2010,
            valid_to: None,
        })
        .unwrap();
    let hop2 = hop2_out.receipt.event_ids[0];
    (cause_p, hop1, hop2)
}

#[test]
fn two_hop_derivation_eligible_when_chain_live() {
    let mut rt = Runtime::default();
    let (_cause_p, hop1, hop2) = emit_two_hop_chain(&mut rt);

    assert!(
        rt.derivation_eligible_at(hop1, u64::MAX, 2017),
        "hop1 must be eligible while the root Assert is live"
    );
    assert!(
        rt.derivation_eligible_at(hop2, u64::MAX, 2017),
        "hop2 must be eligible while every hop is live (DER-02)"
    );
    assert!(
        rt.fold()
            .facts()
            .iter()
            .any(|f| f.claim_id == hop2 && f.is_live_at(u64::MAX, 2017)),
        "hop2 fact must be live at the named cut"
    );
}

#[test]
fn two_hop_ineligible_when_root_assert_retracted() {
    let mut rt = Runtime::default();
    let (cause_p, hop1, hop2) = emit_two_hop_chain(&mut rt);

    assert!(
        rt.derivation_eligible_at(hop2, u64::MAX, 2017),
        "hop2 must start eligible"
    );

    rt.emit(Op::Retract { event_id: cause_p }).unwrap();

    assert!(
        rt.fold()
            .facts()
            .iter()
            .any(|f| f.claim_id == hop2 && f.is_live_at(u64::MAX, 2017)),
        "hop2 fact may still be live after retracting the root Assert"
    );
    assert!(
        !rt.derivation_eligible_at(hop1, u64::MAX, 2017),
        "hop1 must lose eligibility when the root Assert is withdrawn"
    );
    assert!(
        !rt.derivation_eligible_at(hop2, u64::MAX, 2017),
        "hop2 eligibility must walk caused_by and be false after root retract (DER-01)"
    );
}

#[test]
fn two_hop_ineligible_when_parent_rule_retracted() {
    let mut rt = Runtime::default();
    let (_cause_p, hop1, hop2) = emit_two_hop_chain(&mut rt);
    let rule_pq = register_rule_id(&rt, "derive_pq");

    assert!(
        rt.derivation_eligible_at(hop2, u64::MAX, 2017),
        "hop2 must start eligible"
    );

    rt.emit(Op::Retract { event_id: rule_pq }).unwrap();

    assert!(
        rt.fold()
            .facts()
            .iter()
            .any(|f| f.claim_id == hop1 && f.is_live_at(u64::MAX, 2017)),
        "hop1 fact may still be live after retracting derive_pq"
    );
    assert!(
        rt.fold()
            .facts()
            .iter()
            .any(|f| f.claim_id == hop2 && f.is_live_at(u64::MAX, 2017)),
        "hop2 fact may still be live after retracting derive_pq"
    );
    assert!(
        !rt.derivation_eligible_at(hop1, u64::MAX, 2017),
        "hop1 must lose eligibility when its registry pin is retracted"
    );
    assert!(
        !rt.derivation_eligible_at(hop2, u64::MAX, 2017),
        "hop2 must walk the parent pin and be ineligible (DER-01)"
    );
}

fn inverse_knows_rows(rt: &Runtime) -> Vec<&kutha_common::Event> {
    rt.log()
        .iter()
        .filter(|e| matches!(&e.op, Op::Behavior { name, .. } if name == "inverse_knows"))
        .collect()
}

#[test]
fn inverse_knows_cascade_is_residual_spike() {
    let mut rt = Runtime::default();
    let alice = rt.intern("Alice");
    let bob = rt.intern("Bob");
    let knows = rt.intern("knows");
    let asserted = rt
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
        .unwrap();
    let cause = asserted.receipt.event_ids[0];
    let inverses = inverse_knows_rows(&rt);
    assert_eq!(1, inverses.len(), "knows Assert must mint inverse_knows");
    match &inverses[0].op {
        Op::Behavior {
            name,
            caused_by,
            rule_version,
            ..
        } => {
            assert_eq!("inverse_knows", name);
            assert_eq!(cause, *caused_by);
            assert!(
                rule_version.is_empty(),
                "residual cascade keeps an empty pin: {rule_version}"
            );
        }
        other => panic!("expected inverse_knows Behavior, got {other:?}"),
    }

    let p = rt.intern("P");
    let q = rt.intern("Q");
    let rel = rt.intern("relatedTo");
    let true_ = rt.intern("true");
    rt.emit(Op::RegisterRule {
        definition: "derive_pq".into(),
        valid_from: 2010,
        valid_to: None,
    })
    .unwrap();
    let related = rt
        .emit(Op::Assert {
            subject: p,
            relation: rel,
            object: true_,
            valid_from: 2010,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap();
    let related_id = related.receipt.event_ids[0];
    let n = rt.log().len();
    assert_eq!(
        1,
        inverse_knows_rows(&rt).len(),
        "relatedTo Assert must not mint inverse_knows (DER-03)"
    );

    let pin = rule_definition_hash("derive_pq");
    let derived = rt
        .emit(Op::Behavior {
            name: "derive_pq".into(),
            caused_by: related_id,
            rule_version: pin.clone(),
            subject: q,
            relation: rel,
            object: true_,
            valid_from: 2010,
            valid_to: None,
        })
        .unwrap();
    let hop = derived.receipt.event_ids[0];
    let new_behavior = rt
        .log()
        .iter()
        .find(|e| e.id == hop)
        .expect("hashed Behavior");
    match &new_behavior.op {
        Op::Behavior {
            name,
            rule_version,
            ..
        } => {
            assert_eq!("derive_pq", name);
            assert_ne!("inverse_knows", name);
            assert_eq!(pin, *rule_version);
        }
        other => panic!("expected hashed derive_pq, got {other:?}"),
    }
    assert!(
        rt.log().len() > n,
        "hashed Behavior emit must append a row"
    );
    assert_eq!(
        1,
        inverse_knows_rows(&rt).len(),
        "hashed follow-on must not append another inverse_knows"
    );
}
