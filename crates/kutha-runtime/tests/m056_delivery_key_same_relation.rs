//! M056 S03 / Phase 125 — CE #26: keyed retry requires same-relation live support.
//!
//! Lookup is already `(relation, delivery_key)`, but liveness followed the claim
//! alone. Two allowed relations sharing claim+key must not let a live A short-
//! circuit a retracted B.

use kutha_common::{EventId, Op, ValidTime};
use kutha_runtime::Runtime;

fn emit_keyed_on(
    rt: &mut Runtime,
    relation: &str,
    key: &str,
    object: &str,
    claim: Option<EventId>,
) -> kutha_runtime::QuantumOutcome {
    let s = rt.intern("subj-m056");
    let r = rt.intern(relation);
    let o = rt.intern(object);
    rt.emit(Op::Assert {
        subject: s,
        relation: r,
        object: o,
        valid_from: 2010,
        valid_to: None,
        claim,
        delivery_key: Some(key.into()),
        polarity: None,
    })
    .unwrap()
}

#[test]
fn keyed_retry_after_retract_does_not_borrow_sibling_relation_live_key() {
    let mut rt = Runtime::default();
    rt.emit_allow("relA", 2010, None).unwrap();
    rt.emit_allow("relB", 2010, None).unwrap();

    let first_a = emit_keyed_on(&mut rt, "relA", "shared-pin", "ObjA", None);
    assert_eq!(first_a.events_in_quantum, 1);
    let claim: EventId = first_a.receipt.event_ids[0];

    let first_b = emit_keyed_on(&mut rt, "relB", "shared-pin", "ObjB", Some(claim));
    assert!(
        first_b.events_in_quantum >= 1,
        "second relation with shared claim+key must mint its own support"
    );
    let id_b = first_b.receipt.event_ids[0];

    rt.emit(Op::Retract { event_id: id_b }).unwrap();
    let subj = rt.intern("subj-m056");
    let rel_b = rt.intern("relB");
    let obj_b = rt.intern("ObjB");
    assert!(
        !rt.as_of(ValidTime::from(2010))
            .contains(&(subj, rel_b, obj_b)),
        "retract must drop B's live support"
    );

    let retry_b = emit_keyed_on(&mut rt, "relB", "shared-pin", "ObjB", Some(claim));
    assert!(
        retry_b.events_in_quantum >= 1,
        "retracted B must remint; live A with same claim+key must not short-circuit B"
    );
    assert_ne!(
        retry_b.receipt.event_ids[0], id_b,
        "remint must mint a new EventId for B"
    );
    assert!(
        rt.as_of(ValidTime::from(2010))
            .contains(&(subj, rel_b, obj_b)),
        "reminted B must be live again"
    );

    // A remains short-circuitable while live.
    let n = rt.log().len();
    let retry_a = emit_keyed_on(&mut rt, "relA", "shared-pin", "ObjA", None);
    assert_eq!(n, rt.log().len(), "live A retry must not append");
    assert_eq!(retry_a.events_in_quantum, 0);
}
