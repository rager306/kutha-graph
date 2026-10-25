//! Independent verifier kill-tests for M159 / phase 233.
//!
//! Public surface: `replay_check` → `ExecutionReplayFailed` when
//! `horn_behavior_reproducible` is false. In-memory only.

use kutha_common::{Event, EventId, Op, RuleAst};
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

fn inverse_behavior(rt: &Runtime) -> (EventId, String) {
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

fn rebuild(rt: &Runtime, events: Vec<Event>) -> Runtime {
    Runtime::from_dict_and_events(rt.dictionary().strings().to_vec(), events, rt.max_cascade)
        .unwrap()
}

fn without_pin_blob(rt: &Runtime, pin: &str) -> Runtime {
    let events: Vec<Event> = rt
        .log()
        .as_slice()
        .iter()
        .filter(|e| !matches!(&e.op, Op::StoreBlob { digest, .. } if digest == pin))
        .cloned()
        .collect();
    rebuild(rt, events)
}

fn with_empty_pin(rt: &Runtime, behavior_id: EventId) -> Runtime {
    let events: Vec<Event> = rt
        .log()
        .as_slice()
        .iter()
        .map(|e| {
            if e.id != behavior_id {
                return e.clone();
            }
            let mut e = e.clone();
            if let Op::Behavior { rule_version, .. } = &mut e.op {
                rule_version.clear();
            }
            e
        })
        .collect();
    rebuild(rt, events)
}

fn with_matching_named(
    rt: &Runtime,
    old_pin: &str,
    new_pin: &str,
    bytes: Vec<u8>,
    behavior_id: EventId,
) -> Runtime {
    let events: Vec<Event> = rt
        .log()
        .as_slice()
        .iter()
        .map(|e| {
            let mut e = e.clone();
            match &mut e.op {
                Op::StoreBlob {
                    digest,
                    bytes: slot,
                } if digest.as_str() == old_pin => {
                    *digest = new_pin.to_string();
                    *slot = bytes.clone();
                }
                Op::Behavior { rule_version, .. } if e.id == behavior_id => {
                    *rule_version = new_pin.to_string();
                }
                _ => {}
            }
            e
        })
        .collect();
    rebuild(rt, events)
}

fn with_pin_bytes(rt: &Runtime, pin: &str, bytes: Vec<u8>) -> Runtime {
    let events: Vec<Event> = rt
        .log()
        .as_slice()
        .iter()
        .map(|e| match &e.op {
            Op::StoreBlob { digest, .. } if digest == pin => {
                let mut e = e.clone();
                if let Op::StoreBlob { bytes: slot, .. } = &mut e.op {
                    *slot = bytes.clone();
                }
                e
            }
            _ => e.clone(),
        })
        .collect();
    rebuild(rt, events)
}

fn assert_execution_replay_failed(rt: &Runtime, behavior_id: EventId, ctx: &str) {
    let err = rt.replay_check().expect_err(ctx);
    assert!(
        matches!(err, RuntimeError::ExecutionReplayFailed { event_id } if event_id == behavior_id),
        "{ctx}: expected ExecutionReplayFailed for {behavior_id}, got {err:?}"
    );
}

#[test]
fn kill_missing_nonempty_pin_is_execution_replay_failed() {
    let twin = knows_inverse_runtime();
    let (behavior_id, pin) = inverse_behavior(&twin);
    assert!(!pin.is_empty(), "Horn pin must be nonempty");
    assert!(
        twin.log()
            .iter()
            .any(|e| matches!(&e.op, Op::StoreBlob { digest, .. } if digest == &pin)),
        "twin must carry the pin blob before the strip"
    );

    let missing = without_pin_blob(&twin, &pin);
    assert!(
        missing
            .log()
            .iter()
            .any(|e| matches!(&e.op, Op::Behavior { rule_version, .. } if rule_version == &pin)),
        "strip must keep the nonempty pin on the Behavior"
    );
    assert!(
        !missing
            .log()
            .iter()
            .any(|e| matches!(&e.op, Op::StoreBlob { digest, .. } if digest == &pin)),
        "strip must drop only the matching StoreBlob"
    );
    assert_execution_replay_failed(
        &missing,
        behavior_id,
        "missing nonempty pin blob must fail closed",
    );
}

#[test]
fn kill_empty_pin_is_execution_replay_failed() {
    let twin = knows_inverse_runtime();
    let (behavior_id, pin) = inverse_behavior(&twin);
    assert!(!pin.is_empty());

    let emptied = with_empty_pin(&twin, behavior_id);
    assert!(
        emptied.log().iter().any(|e| matches!(
            &e.op,
            Op::Behavior { rule_version, .. } if e.id == behavior_id && rule_version.is_empty()
        )),
        "Behavior pin must be the empty string"
    );
    assert!(
        emptied
            .log()
            .iter()
            .any(|e| matches!(&e.op, Op::StoreBlob { digest, .. } if digest == &pin)),
        "empty-pin kill keeps the old blob; lookup of \"\" must still miss"
    );
    assert!(
        !emptied
            .log()
            .iter()
            .any(|e| matches!(&e.op, Op::StoreBlob { digest, .. } if digest.is_empty())),
        "must not plant a blob at the empty digest"
    );
    assert_execution_replay_failed(&emptied, behavior_id, "empty pin must fail closed");
}

#[test]
fn kill_present_horn_match_is_ok() {
    let twin = knows_inverse_runtime();
    let (_id, pin) = inverse_behavior(&twin);
    assert!(!pin.is_empty());
    twin.replay_check()
        .expect("matching Horn blob must replay_check Ok");
}

#[test]
fn kill_present_non_horn_body_skips_ok() {
    let twin = knows_inverse_runtime();
    let (behavior_id, pin) = inverse_behavior(&twin);
    let ast = RuleAst::named("namedSkip").expect("named");
    let named = ast.encode();
    let named_pin = ast.hash();
    assert_ne!(named, {
        twin.log()
            .iter()
            .find_map(|e| match &e.op {
                Op::StoreBlob { digest, bytes } if digest == &pin => Some(bytes.clone()),
                _ => None,
            })
            .expect("Horn blob bytes")
    });

    // Digest and Behavior pin both become the Named hash so blob_digest_matches
    // holds; Named still skips Horn replay (M159). Do not keep the Horn pin on
    // Named bytes (that pair is UnknownRuleVersion after M160).
    let skipped = with_matching_named(&twin, &pin, &named_pin, named, behavior_id);
    skipped
        .replay_check()
        .expect("present Named body must skip (replay Ok)");

    let missing = without_pin_blob(&skipped, &named_pin);
    assert_execution_replay_failed(
        &missing,
        behavior_id,
        "the same pin without a blob must not ride the Named skip",
    );
}

#[test]
fn kill_garbage_bytes_not_rule_ast_fail_closed() {
    let twin = knows_inverse_runtime();
    let (behavior_id, pin) = inverse_behavior(&twin);
    // Exact-consume: a Named prefix plus a trailing byte is not a RuleAst.
    // A parser that ignored the remainder would skip (Named) instead of fail closed.
    let mut garbage = RuleAst::named("namedSkip").expect("named").encode();
    garbage.push(0xFF);
    let broken = with_pin_bytes(&twin, &pin, garbage);
    assert_execution_replay_failed(
        &broken,
        behavior_id,
        "bytes that are not a RuleAst must fail closed",
    );
}
