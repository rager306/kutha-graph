//! M064 S01 — explicit PinPolicy migrate; open/serde stay fail-open (m050).

use kutha_common::{
    policy_version_hash, remap_legacy_events, remap_legacy_ops, EventId, Op, TermDictionary,
    KUTHA_POLICY,
};

#[test]
fn pin_policy_op_remaps_to_store_blob_and_kutha_policy() {
    let mut dict = TermDictionary::default();
    let raw = r#"{"PinPolicy":{"definition":"allow-all","valid_from":2010,"valid_to":null}}"#;
    let value: serde_json::Value = serde_json::from_str(raw).expect("json");
    let ops = remap_legacy_ops(&value, |s| dict.intern(s)).expect("remap");
    assert_eq!(ops.len(), 2);
    let digest = policy_version_hash("allow-all");
    assert!(matches!(&ops[0], Op::StoreBlob { digest: d, bytes } if d == &digest && bytes == b"allow-all"));
    match &ops[1] {
        Op::Assert {
            relation,
            delivery_key,
            ..
        } => {
            assert_eq!(dict.lookup(*relation), Some(KUTHA_POLICY));
            assert_eq!(delivery_key.as_deref(), Some(digest.as_str()));
        }
        other => panic!("expected Assert, got {other:?}"),
    }
}

#[test]
fn pin_policy_event_keeps_id_on_assert() {
    let mut dict = TermDictionary::default();
    let id = EventId::nil();
    let raw = format!(
        r#"{{"id":"{id}","op":{{"PinPolicy":{{"definition":"allow-all","valid_from":2010,"valid_to":null}}}},"ingested_at":11,"object_ids":[]}}"#
    );
    let value: serde_json::Value = serde_json::from_str(&raw).expect("json");
    let events = remap_legacy_events(&value, |s| dict.intern(s)).expect("remap events");
    assert_eq!(events.len(), 2);
    assert!(matches!(events[0].op, Op::StoreBlob { .. }));
    assert_ne!(events[0].id, id);
    assert_eq!(events[1].id, id);
    assert!(matches!(events[1].op, Op::Assert { .. }));
    assert_eq!(events[1].ingested_at, 11);
}

#[test]
fn serde_open_path_still_rejects_pin_policy() {
    let raw = r#"{"PinPolicy":{"definition":"allow-all","valid_from":2010,"valid_to":null}}"#;
    assert!(
        serde_json::from_str::<Op>(raw).is_err(),
        "documented break: no serde alias"
    );
}
