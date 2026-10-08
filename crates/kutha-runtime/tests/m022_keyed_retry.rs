//! M022 S02 / Phase 72 — keyed Assert retry only while the prior support is live.

use kutha_common::Op;
use kutha_runtime::Runtime;

/// Non-cascade relation so receipt EventIds are Assert-only.
fn emit_keyed(rt: &mut Runtime, key: &str, object: &str) -> kutha_runtime::QuantumOutcome {
    rt.emit_allow("leasedRel", 2010, None).unwrap();
    let s = rt.intern("subj");
    let r = rt.intern("leasedRel");
    let o = rt.intern(object);
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
    .unwrap()
}

#[test]
fn keyed_assert_retry_while_live_short_circuits() {
    let mut rt = Runtime::default();
    let first = emit_keyed(&mut rt, "k1", "Bob");
    assert_eq!(first.events_in_quantum, 1, "no cascade on leasedRel");
    let retry = emit_keyed(&mut rt, "k1", "Bob");
    assert_eq!(
        retry.events_in_quantum, 0,
        "identical live keyed Assert must short-circuit"
    );
    assert_eq!(
        retry.receipt.event_ids, first.receipt.event_ids,
        "short-circuit returns the original Assert EventId"
    );
}

#[test]
fn keyed_assert_retry_after_retract_remints_live_support() {
    let mut rt = Runtime::default();
    let first = emit_keyed(&mut rt, "k-retry", "Bob");
    assert_eq!(first.events_in_quantum, 1);
    let id = first.receipt.event_ids[0];
    let rel = rt.intern("leasedRel");
    let subj = rt.intern("subj");
    let bob = rt.intern("Bob");
    let triple = (subj, rel, bob);
    assert!(rt.as_of(2010).contains(&triple));

    rt.emit(Op::Retract { event_id: id }).unwrap();
    assert!(
        !rt.as_of(2010).contains(&triple),
        "retract must drop the live support"
    );

    let remint = emit_keyed(&mut rt, "k-retry", "Bob");
    assert!(
        remint.events_in_quantum >= 1,
        "after retract, same delivery_key must remint (not events_in_quantum==0)"
    );
    assert!(
        rt.as_of(2010).contains(&triple),
        "reminted keyed Assert must be live"
    );
    assert_ne!(
        remint.receipt.event_ids[0], id,
        "remint must mint a new EventId"
    );
}
