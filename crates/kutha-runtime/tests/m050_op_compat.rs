//! G4 / CE #14 — documented Op wire break (M050).
//! Legacy dictionary meta-ops fail deserialize; current kutha:* Asserts round-trip.

use kutha_common::{Event, EventId, Op, TermId};

fn legacy_allow_relation_op_json() -> &'static str {
    r#"{"AllowRelation":{"name":"relatedTo","valid_from":0,"valid_to":null}}"#
}

#[test]
fn legacy_allow_relation_op_fails_to_deserialize() {
    let err = serde_json::from_str::<Op>(legacy_allow_relation_op_json()).unwrap_err();
    let msg = err.to_string();
    assert!(
        msg.contains("AllowRelation") || msg.contains("unknown variant"),
        "expected unknown-variant style error, got {msg}"
    );
}

#[test]
fn legacy_allow_relation_event_fails_to_deserialize() {
    let id = EventId::nil();
    let raw = format!(
        r#"{{"id":"{id}","op":{op},"ingested_at":1,"object_ids":[]}}"#,
        op = legacy_allow_relation_op_json()
    );
    assert!(
        serde_json::from_str::<Event>(&raw).is_err(),
        "main-era AllowRelation Event must fail open (documented break)"
    );
}

#[test]
fn legacy_register_rule_op_fails_to_deserialize() {
    let raw = r#"{"RegisterRule":{"definition":"R","valid_from":0,"valid_to":null}}"#;
    assert!(serde_json::from_str::<Op>(raw).is_err());
}

#[test]
fn legacy_pin_policy_op_fails_to_deserialize() {
    let raw = r#"{"PinPolicy":{"definition":"P","valid_from":0,"valid_to":null}}"#;
    assert!(serde_json::from_str::<Op>(raw).is_err());
}

#[test]
fn legacy_record_admission_op_fails_to_deserialize() {
    let raw = r#"{"RecordAdmission":{"justification_id":"j","admitted":true,"policy_version":"p","valid_from":0,"valid_to":null}}"#;
    assert!(serde_json::from_str::<Op>(raw).is_err());
}

#[test]
fn legacy_record_action_op_fails_to_deserialize() {
    let raw = r#"{"RecordAction":{"justification_id":"j","target_claim":"00000000-0000-0000-0000-000000000000","source_claim_ids":[],"source_event_ids":[],"rule_version":"r","admitted":true,"policy_version":"p","valid_from":0,"valid_to":null}}"#;
    assert!(serde_json::from_str::<Op>(raw).is_err());
}

#[test]
fn modern_kutha_allows_assert_roundtrips() {
    let op = Op::Assert {
        subject: 1 as TermId,
        relation: 2 as TermId,
        object: 1 as TermId,
        valid_from: 0,
        valid_to: None,
        claim: None,
        delivery_key: Some("relatedTo".into()),
        polarity: None,
    };
    let event = Event {
        id: EventId::nil(),
        op,
        ingested_at: 1,
        object_ids: vec![1, 2],
    };
    let bytes = serde_json::to_string(&event).expect("serialize");
    let back: Event = serde_json::from_str(&bytes).expect("deserialize modern Assert");
    assert_eq!(event, back);
    assert!(
        !bytes.contains("AllowRelation"),
        "current writers must not emit removed variants"
    );
}
