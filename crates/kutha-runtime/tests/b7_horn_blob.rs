//! Phase 233 / M159 S01 — missing Horn blob fails closed (b7).
//! A nonempty pin with no blob must fail replay (RULE-04).

use kutha_common::{Event, Op};
use kutha_runtime::{Runtime, RuntimeError};

fn knows_inverse_runtime() -> Runtime {
    let mut rt = Runtime::default();
    let a = rt.intern("Alice");
    let b = rt.intern("Bob");
    let knows = rt.intern("knows");
    rt.emit(Op::Assert {
        subject: a,
        relation: knows,
        object: b,
        valid_from: 2010,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    rt
}

fn inverse_behavior(rt: &Runtime) -> (kutha_common::EventId, String) {
    rt.log()
        .iter()
        .find_map(|e| match &e.op {
            Op::Behavior {
                name, rule_version, ..
            } if name == "inverse_knows" => Some((e.id, rule_version.clone())),
            _ => None,
        })
        .expect("inverse_knows Behavior")
}

fn without_pin_blob(rt: &Runtime, pin: &str) -> Runtime {
    let events: Vec<Event> = rt
        .log()
        .as_slice()
        .iter()
        .filter(|e| !matches!(&e.op, Op::StoreBlob { digest, .. } if digest == pin))
        .cloned()
        .collect();
    Runtime::from_dict_and_events(rt.dictionary().strings().to_vec(), events, rt.max_cascade)
        .unwrap()
}

#[test]
fn missing_rule_blob_fails_replay() {
    let twin = knows_inverse_runtime();
    let (behavior_id, pin) = inverse_behavior(&twin);
    assert!(!pin.is_empty(), "Horn pin must be nonempty");
    twin.replay_check()
        .expect("twin with Horn blob present must replay_check Ok");

    let missing = without_pin_blob(&twin, &pin);
    let err = missing
        .replay_check()
        .expect_err("missing nonempty pin blob must fail closed");
    assert!(
        matches!(err, RuntimeError::ExecutionReplayFailed { event_id } if event_id == behavior_id),
        "expected ExecutionReplayFailed for {behavior_id}, got {err:?}"
    );
}
