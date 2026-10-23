//! Phase 228 / M154 S01 — sealed-prefix checkpoint receipt oracle (B1).
//! RED: `store::checkpoint_sealed_prefix` is missing (E0425). Do not treat
//! sealed-file deletion as the RED.

use kutha_common::{
    policy_version_hash, sha256_hex, Event, EventId, Op, RuleAst, RuleAtom, RuleTerm, ValidTime,
    WallTime,
};
use kutha_runtime::{store, GraphFold, Runtime, RuntimeError};
use std::fs;
use std::path::Path;

mod common;

struct LimitGuard;

impl Drop for LimitGuard {
    fn drop(&mut self) {
        store::reset_test_segment_limits();
    }
}

fn tiny_limits() -> LimitGuard {
    // One event per segment so the sealed prefix includes graph StoreBlobs.
    // Defines pack first; max_events=3 can seal intern rows only.
    store::set_test_segment_limits(Some(4096), Some(1));
    LimitGuard
}

fn persist_dir() -> common::PersistScratch {
    common::PersistScratch::new("b1-checkpoint")
}

fn sealed_len(dir: &Path) -> usize {
    manifest(dir)["sealed"]
        .as_array()
        .expect("sealed array")
        .len()
}

fn sealed_event_count(dir: &Path) -> usize {
    manifest(dir)["sealed"]
        .as_array()
        .expect("sealed array")
        .iter()
        .map(|s| s["count"].as_u64().expect("sealed count") as usize)
        .sum()
}

fn manifest(dir: &Path) -> serde_json::Value {
    serde_json::from_str(&fs::read_to_string(dir.join(store::MANIFEST_REL)).unwrap()).unwrap()
}

fn blob_payload(events: &[Event], digest: &str) -> Option<(EventId, Vec<u8>)> {
    events.iter().find_map(|e| match &e.op {
        Op::StoreBlob { digest: d, bytes } if d == digest => Some((e.id, bytes.clone())),
        _ => None,
    })
}

#[test]
fn checkpoint_sealed_prefix_preserves_fingerprint_and_pit() {
    let _limits = tiny_limits();
    let dir = persist_dir();
    let mut rt = Runtime::default();
    rt.emit_allow("derive_pq", 0, None).unwrap();
    let head = RuleAtom::new(
        "relatedTo",
        RuleTerm::var(0).unwrap(),
        RuleTerm::constant("derived").unwrap(),
    )
    .unwrap();
    let prem = RuleAtom::new(
        "relatedTo",
        RuleTerm::var(0).unwrap(),
        RuleTerm::constant("true").unwrap(),
    )
    .unwrap();
    let ast = RuleAst::horn(head, vec![prem]).unwrap();
    let rule_pin = ast.hash();
    let rule_bytes = ast.encode();
    rt.emit_rule_ast(&ast, 2010, None).unwrap();
    let policy_bytes = b"admit-all".to_vec();
    let policy_pin = policy_version_hash("admit-all");
    rt.emit_policy("admit-all", 2010, None).unwrap();
    let plain_bytes = b"plain-checkpoint-blob".to_vec();
    let plain_digest = sha256_hex(&plain_bytes);
    rt.emit(Op::StoreBlob {
        digest: plain_digest.clone(),
        bytes: plain_bytes.clone(),
    })
    .unwrap();
    let p = rt.intern("P");
    let rel = rt.intern("relatedTo");
    let true_ = rt.intern("true");
    let derived = rt.intern("derived");
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
    let cause = asserted.receipt.event_ids[0];
    rt.emit(Op::Behavior {
        name: "derive_pq".into(),
        caused_by: cause,
        rule_version: rule_pin.clone(),
        subject: p,
        relation: rel,
        object: derived,
        valid_from: 2010,
        valid_to: None,
    })
    .unwrap();
    // Genesis already allows inForceAs (delivery_key); a second emit_allow conflicts.
    let relation = rt.intern("inForceAs");
    let mut live_assert = None;
    for i in 0..6 {
        let subject = rt.intern(&format!("art-{i}"));
        let object = rt.intern(&format!("obj-{i}"));
        let id = rt
            .emit(Op::Assert {
                subject,
                relation,
                object,
                valid_from: 2010,
                valid_to: Some(2020),
                claim: None,
                delivery_key: Some(format!("dk-{i}")),
                polarity: None,
            })
            .unwrap()
            .receipt
            .event_ids[0];
        if live_assert.is_none() {
            live_assert = Some(id);
        }
    }
    rt.emit(Op::Retract {
        event_id: live_assert.expect("one live Assert"),
    })
    .unwrap();
    rt.emit_tt_anchor(2015).unwrap();
    rt.emit_tt_anchor(2021).unwrap();
    rt.replay_check()
        .expect("fixture replay_check before persist");
    store::persist(&rt, &dir).unwrap();
    assert!(
        sealed_len(&dir) >= 2,
        "fixture must span at least two sealed segments"
    );

    let encoded = store::encoded_events(&rt);
    let prefix_len = sealed_event_count(&dir);
    assert!(
        prefix_len > 0 && prefix_len <= encoded.len(),
        "sealed prefix length {prefix_len} vs encoded {}",
        encoded.len()
    );
    let prefix = &encoded[..prefix_len];
    let rule_before = blob_payload(prefix, &rule_pin).expect("rule StoreBlob in sealed prefix");
    let policy_before =
        blob_payload(prefix, &policy_pin).expect("policy StoreBlob in sealed prefix");
    let plain_before =
        blob_payload(prefix, &plain_digest).expect("plain StoreBlob in sealed prefix");
    assert_eq!(rule_before.1, rule_bytes);
    assert_eq!(policy_before.1, policy_bytes);
    assert_eq!(plain_before.1, plain_bytes);
    let prefix_kept_count = prefix
        .iter()
        .filter(|e| {
            matches!(
                e.op,
                Op::StoreBlob { .. } | Op::QuantumOutcome { .. } | Op::JustificationCite { .. }
            )
        })
        .count();
    let dict_strings = rt.dictionary().strings().to_vec();
    let fact_count = rt.fold().facts().len() as u64;
    assert!(rt.fold().rule_hash_live_at(&rule_pin, u64::MAX, 2010));
    assert!(rt.fold().policy_hash_live_at(&policy_pin, u64::MAX, 2010));

    let fingerprint = rt.fold().fingerprint();
    let as_of_2009 = rt.as_of(ValidTime::from(2009));
    let as_of_2015 = rt.as_of(ValidTime::from(2015));
    let as_of_2025 = rt.as_of(ValidTime::from(2025));
    let known_2015 = rt.as_known_on(WallTime::from(2015));
    let known_2021 = rt.as_known_on(WallTime::from(2021));

    store::checkpoint_sealed_prefix(&dir).unwrap();

    let opened = store::open(&dir).expect("open after checkpoint");
    opened
        .replay_check()
        .expect("replay_check after checkpoint");
    assert_eq!(opened.fold().fingerprint(), fingerprint);
    assert_eq!(opened.as_of(ValidTime::from(2009)), as_of_2009);
    assert_eq!(opened.as_of(ValidTime::from(2015)), as_of_2015);
    assert_eq!(opened.as_of(ValidTime::from(2025)), as_of_2025);
    assert_eq!(opened.as_known_on(WallTime::from(2015)), known_2015);
    assert_eq!(opened.as_known_on(WallTime::from(2021)), known_2021);
    assert_eq!(
        GraphFold::replay_with_dict(opened.log().as_slice(), opened.dictionary()).fingerprint(),
        fingerprint
    );
    assert_eq!(opened.dictionary().strings(), dict_strings.as_slice());
    assert!(opened.fold().rule_hash_live_at(&rule_pin, u64::MAX, 2010));
    assert!(opened
        .fold()
        .policy_hash_live_at(&policy_pin, u64::MAX, 2010));
    let log = opened.log().as_slice();
    let rule_after = blob_payload(log, &rule_pin).expect("rule StoreBlob survives checkpoint");
    let policy_after =
        blob_payload(log, &policy_pin).expect("policy StoreBlob survives checkpoint");
    let plain_after =
        blob_payload(log, &plain_digest).expect("plain StoreBlob survives checkpoint");
    assert_eq!(rule_after, rule_before);
    assert_eq!(policy_after, policy_before);
    assert_eq!(plain_after, plain_before);
    let ckpt = log
        .iter()
        .find_map(|e| match &e.op {
            Op::Checkpoint {
                dropped_count,
                fact_count,
                ..
            } => Some((*dropped_count, *fact_count)),
            _ => None,
        })
        .expect("Op::Checkpoint on rewritten log");
    assert_eq!(ckpt.0, (prefix_len - prefix_kept_count) as u64);
    assert_eq!(ckpt.1, fact_count);
}

#[test]
fn user_emit_checkpoint_is_meta_op_rejected() {
    let mut rt = Runtime::default();
    let before = rt.log().len();
    let err = rt
        .emit(Op::Checkpoint {
            first_event_id: EventId::nil(),
            last_event_id: EventId::nil(),
            dropped_count: 0,
            fact_count: 0,
            fingerprint_hex: String::new(),
            blob_digest: String::new(),
        })
        .unwrap_err();
    assert!(matches!(err, RuntimeError::MetaOpRejected), "{err:?}");
    assert_eq!(rt.log().len(), before);
}
