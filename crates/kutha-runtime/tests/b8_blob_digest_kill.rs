//! Independent verifier kill-tests for M160 / phase 234.
//! Public surface: after the Behavior walk, `replay_check` / `store::open`
//! reject an ordinary StoreBlob whose bytes fail `blob_digest_matches`
//! (`UnknownRuleVersion { pin: digest }`). Matching hashes still open.
//! Apply stays infallible (`from_dict_and_events` Ok). No leftover tmp dirs.

use kutha_common::{
    Event, EventId, Op, RuleAst, blob_digest_matches, policy_version_hash, rule_body_hash,
    sha256_hex,
};
use kutha_runtime::{Runtime, RuntimeError, store};
use std::fs;
use std::path::Path;

mod common;

fn persist_dir(label: &str) -> common::PersistScratch {
    common::PersistScratch::new(&format!("b8-kill-{label}"))
}

fn rebuild(rt: &Runtime, events: Vec<Event>) -> Runtime {
    Runtime::from_dict_and_events(rt.dictionary().strings().to_vec(), events, rt.max_cascade)
        .expect("apply is infallible: mismatched StoreBlob must still hydrate")
}

fn export_events(dir: &Path) -> Vec<Event> {
    store::export_jsonl(dir).unwrap();
    let text = fs::read_to_string(dir.join("events.jsonl")).unwrap();
    text.lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str::<Event>(l).expect("jsonl Event"))
        .collect()
}

fn rewrite_on_disk_log(dir: &Path, events: &[Event]) {
    let n = events.len();
    let (sealed, active) = if n >= 2 {
        (vec![n - 1], 1)
    } else {
        (Vec::new(), n)
    };
    store::write_test_segmented(dir, events, &sealed, active).unwrap();
}

fn ordinary_blob_runtime() -> (Runtime, String, Vec<u8>) {
    let mut rt = Runtime::default();
    let bytes = b"ordinary-blob-payload-m160-kill".to_vec();
    let digest = sha256_hex(&bytes);
    rt.emit(Op::StoreBlob {
        digest: digest.clone(),
        bytes: bytes.clone(),
    })
    .unwrap();
    (rt, digest, bytes)
}

fn flip_last_byte(bytes: &mut [u8]) {
    let last = bytes.len() - 1;
    bytes[last] ^= 0x5A;
}

fn flip_store_blob_last_byte(events: &mut [Event], digest: &str, orig_len: usize, orig: &[u8]) {
    let mut found = false;
    for e in events {
        if let Op::StoreBlob {
            digest: d,
            bytes: b,
        } = &mut e.op
        {
            if d == digest && !b.is_empty() {
                flip_last_byte(b);
                assert_eq!(b.len(), orig_len, "same-length rewrite");
                assert_ne!(b.as_slice(), orig);
                assert!(!blob_digest_matches(d, b));
                found = true;
                break;
            }
        }
    }
    assert!(found, "ordinary StoreBlob to flip");
}

fn assert_open_unknown_rule_version(dir: &Path, digest: &str, ctx: &str) {
    match store::open(dir) {
        Err(err) => {
            assert_eq!(err.kind(), std::io::ErrorKind::InvalidData, "{ctx}");
            let expected = RuntimeError::UnknownRuleVersion {
                pin: digest.to_string(),
            }
            .to_string();
            assert_eq!(err.to_string(), expected, "{ctx}: got {err}");
        }
        Ok(_) => panic!("{ctx}: expected UnknownRuleVersion open Err, got Ok"),
    }
}

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

#[test]
fn kill_same_length_tamper_fails_open_matching_twin_ok() {
    let (rt, digest, bytes) = ordinary_blob_runtime();
    let orig_len = bytes.len();
    assert!(blob_digest_matches(&digest, &bytes));

    let twin = persist_dir("twin");
    store::persist(&rt, &twin).unwrap();
    store::open(&twin).expect("matching ordinary StoreBlob must open");
    rt.replay_check()
        .expect("matching ordinary StoreBlob must replay_check Ok");

    let mut mem = rt.log().as_slice().to_vec();
    flip_store_blob_last_byte(&mut mem, &digest, orig_len, &bytes);
    let tampered = rebuild(&rt, mem);
    let err = tampered
        .replay_check()
        .expect_err("same-length tamper must fail replay_check");
    assert!(
        matches!(err, RuntimeError::UnknownRuleVersion { ref pin } if pin == &digest),
        "expected UnknownRuleVersion pin={digest}, got {err:?}"
    );

    let bad = persist_dir("tamper");
    store::persist(&rt, &bad).unwrap();
    let mut on_disk = export_events(&bad);
    flip_store_blob_last_byte(&mut on_disk, &digest, orig_len, &bytes);
    rewrite_on_disk_log(&bad, &on_disk);
    assert_open_unknown_rule_version(&bad, &digest, "same-length tamper");
}

#[test]
fn kill_truncated_length_fails_open_crc_rewritten() {
    // Truncate Event bytes, then rewrite segments so frame CRC is recomputed.
    // Chopping the sealed file in place would fail CRC before replay_check.
    let (rt, digest, bytes) = ordinary_blob_runtime();
    assert!(bytes.len() > 1, "need a nonempty suffix to drop");

    let dir = persist_dir("trunc");
    store::persist(&rt, &dir).unwrap();
    let mut events = export_events(&dir);
    let mut found = false;
    for e in &mut events {
        if let Op::StoreBlob {
            digest: d,
            bytes: b,
        } = &mut e.op
        {
            if d == &digest && b.len() == bytes.len() {
                b.pop();
                assert!(!b.is_empty(), "keep nonempty so this is length, not empty");
                assert!(b.len() < bytes.len());
                assert!(!blob_digest_matches(d, b));
                found = true;
                break;
            }
        }
    }
    assert!(found, "ordinary StoreBlob to truncate");
    rewrite_on_disk_log(&dir, &events);
    assert_open_unknown_rule_version(&dir, &digest, "truncated length with rewritten CRC");
}

#[test]
fn kill_missing_horn_wins_over_leading_bad_ordinary_blob() {
    let twin = knows_inverse_runtime();
    let (behavior_id, pin) = inverse_behavior(&twin);
    assert!(!pin.is_empty());
    twin.replay_check()
        .expect("twin with Horn blob present must replay_check Ok");

    let ordinary = b"ordinary-payload-m160-order".to_vec();
    let digest = sha256_hex(&ordinary);
    let mut bad_bytes = ordinary.clone();
    flip_last_byte(&mut bad_bytes);
    assert!(!blob_digest_matches(&digest, &bad_bytes));

    // Leading mismatched blob: a single-walk StoreBlob-first impl would
    // return UnknownRuleVersion. Behavior walk must still win.
    let mut events: Vec<Event> = vec![Event {
        id: EventId::from_u128(0xBAD0_B10B),
        op: Op::StoreBlob {
            digest,
            bytes: bad_bytes,
        },
        ingested_at: 0,
        object_ids: vec![],
    }];
    events.extend(
        twin.log()
            .as_slice()
            .iter()
            .filter(|e| !matches!(&e.op, Op::StoreBlob { digest, .. } if digest == &pin))
            .cloned(),
    );
    assert!(
        events
            .iter()
            .any(|e| matches!(&e.op, Op::Behavior { rule_version, .. } if rule_version == &pin)),
        "keep the nonempty Horn pin on the Behavior"
    );
    assert!(
        !events
            .iter()
            .any(|e| matches!(&e.op, Op::StoreBlob { digest, .. } if digest == &pin)),
        "Horn blob must be absent"
    );

    let mixed = rebuild(&twin, events);
    let err = mixed
        .replay_check()
        .expect_err("missing Horn plus bad ordinary blob must fail closed");
    assert!(
        matches!(err, RuntimeError::ExecutionReplayFailed { event_id } if event_id == behavior_id),
        "Behavior error must win; expected ExecutionReplayFailed for {behavior_id}, got {err:?}"
    );
}

#[test]
fn kill_policy_and_rule_body_hash_matching_blobs_open() {
    let mut rt = Runtime::default();
    let policy = "policy-body-m160-kill";
    let policy_pin = policy_version_hash(policy);
    assert_ne!(policy_pin, sha256_hex(policy.as_bytes()));
    rt.emit(Op::StoreBlob {
        digest: policy_pin,
        bytes: policy.as_bytes().to_vec(),
    })
    .unwrap();

    let ast = RuleAst::named("namedSkip").expect("named");
    let body = ast.encode();
    let rule_pin = rule_body_hash(&body);
    assert_ne!(rule_pin, sha256_hex(&body));
    rt.emit(Op::StoreBlob {
        digest: rule_pin,
        bytes: body,
    })
    .unwrap();

    rt.replay_check()
        .expect("policy and rule-body matching hashes must replay_check Ok");
    let dir = persist_dir("alt-hash");
    store::persist(&rt, &dir).unwrap();
    store::open(&dir).expect("policy and rule-body matching hashes must open");
}

#[test]
fn kill_digest_sha256_of_other_content_fails() {
    let bytes = b"content-a-m160-kill".to_vec();
    let other = b"content-b-m160-kill".to_vec();
    let digest = sha256_hex(&other);
    assert_ne!(digest, sha256_hex(&bytes));
    assert!(!blob_digest_matches(&digest, &bytes));

    let rt = Runtime::default();
    let mut events = rt.log().as_slice().to_vec();
    let next_tt = events
        .last()
        .map(|e| e.ingested_at.saturating_add(1))
        .unwrap_or(0);
    events.push(Event {
        id: EventId::from_u128(0xD165_A7ED),
        op: Op::StoreBlob {
            digest: digest.clone(),
            bytes,
        },
        ingested_at: next_tt,
        object_ids: vec![],
    });
    let mismatched = rebuild(&rt, events);
    let err = mismatched
        .replay_check()
        .expect_err("sha256 of different content must fail replay_check");
    assert!(
        matches!(err, RuntimeError::UnknownRuleVersion { ref pin } if pin == &digest),
        "expected UnknownRuleVersion pin={digest}, got {err:?}"
    );

    let bad = persist_dir("other-hash");
    store::persist(&mismatched, &bad).unwrap();
    assert_open_unknown_rule_version(&bad, &digest, "sha256 of different content");
}
