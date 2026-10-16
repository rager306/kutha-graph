//! M100 S01 / Phase 171 — Correct and CorrectInterval must unindex a Behavior
//! (CE whole-repo 2026-10-15 residual).

use kutha_common::Op;
use kutha_runtime::Runtime;

fn behavior_id(rt: &Runtime) -> kutha_common::EventId {
    rt.log()
        .iter()
        .filter(|e| matches!(e.op, Op::Behavior { .. }))
        .last()
        .expect("inverse_knows Behavior")
        .id
}

fn assert_knows(rt: &mut Runtime, subject: &str, object: &str) -> kutha_common::EventId {
    let s = rt.intern(subject);
    let o = rt.intern(object);
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
    .unwrap()
    .receipt
    .event_ids[0]
}

/// After Correct of the derived head, retracting its premise must not examine that head.
#[test]
fn correct_of_behavior_drops_retract_index() {
    let mut rt = Runtime::default();
    let premise = assert_knows(&mut rt, "Alice", "Bob");
    let head = behavior_id(&rt);
    rt.correct("Bob", "knownBy", "Carol", 2010).unwrap();
    rt.reset_retract_examine_count();
    rt.emit(Op::Retract { event_id: premise }).unwrap();
    let n = rt.retract_examine_count();
    assert_eq!(
        n, 0,
        "Correct of Behavior {head} must unindex it before a later retract; examined {n}"
    );
}

/// Same lease drop for a valid-time patch of the derived head.
#[test]
fn correct_interval_of_behavior_drops_retract_index() {
    let mut rt = Runtime::default();
    let premise = assert_knows(&mut rt, "Alice", "Bob");
    let head = behavior_id(&rt);
    let carol = rt.intern("Carol");
    rt.emit(Op::CorrectInterval {
        event_id: head,
        object: carol,
        patch_from: 2012,
        patch_to: Some(2018),
    })
    .unwrap();
    rt.reset_retract_examine_count();
    rt.emit(Op::Retract { event_id: premise }).unwrap();
    let n = rt.retract_examine_count();
    assert_eq!(
        n, 0,
        "CorrectInterval of Behavior {head} must unindex it; examined {n}"
    );
}
