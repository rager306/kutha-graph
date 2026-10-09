//! M032 S01 / Phase 92 — derive() must not chain from retracted premises
//! (CE review P1 #4). Match retract/correct: filter `invalidated_at.is_none()`.

use kutha_common::Op;
use kutha_runtime::{Runtime, RuntimeError};

#[test]
fn derive_after_sole_premise_retract_fails_closed() {
    let mut rt = Runtime::default();
    let s = "s-m032-sole";
    rt.assert(s, "relatedTo", "o1", 2010, None).unwrap();
    rt.retract(s, 2010).unwrap();

    match rt.derive("statute-fold", s, 2011) {
        Err(RuntimeError::UnknownFact { .. }) => {}
        Ok(out) => panic!("must not mint Behavior from a dead premise; got {out:?}"),
        Err(e) => panic!("expected UnknownFact after sole retract; got {e}"),
    }
    assert!(
        !rt.log()
            .iter()
            .any(|e| matches!(&e.op, Op::Behavior { .. })),
        "failed derive must append no Behavior"
    );
}

#[test]
fn derive_skips_invalidated_latest_uses_prior_live() {
    let mut rt = Runtime::default();
    let s = "s-m032-skip";
    let early = rt.assert(s, "relatedTo", "o-early", 2010, None).unwrap();
    let early_id = early.receipt.event_ids[0];
    let late = rt.assert(s, "relatedTo", "o-late", 2011, None).unwrap();
    let late_id = late.receipt.event_ids[0];
    rt.retract(s, 2011).unwrap();

    rt.derive("statute-fold", s, 2011)
        .expect("prior live premise must still allow derive");

    let o_early = rt.intern("o-early");
    let behavior = rt
        .log()
        .as_slice()
        .iter()
        .rev()
        .find(|e| matches!(&e.op, Op::Behavior { name, .. } if name == "statute-fold"))
        .expect("derive must append Behavior");
    match &behavior.op {
        Op::Behavior { caused_by, object, .. } => {
            assert_eq!(
                caused_by, &early_id,
                "caused_by must be the live early Assert, not retracted late"
            );
            assert_ne!(caused_by, &late_id, "must not cite retracted late Assert");
            assert_eq!(object, &o_early, "Behavior object must come from live premise");
        }
        other => panic!("expected Behavior, got {other:?}"),
    }
}
