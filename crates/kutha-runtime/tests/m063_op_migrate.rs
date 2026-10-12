//! M063 S01 — explicit RegisterRule migrate; open/serde stay fail-open (m050).

use kutha_common::{
    remap_legacy_events, remap_legacy_ops, EventId, Op, RuleAst, TermDictionary, KUTHA_RULE,
};

#[test]
fn register_rule_op_remaps_to_store_blob_and_kutha_rule() {
    let mut dict = TermDictionary::default();
    let raw = r#"{"RegisterRule":{"definition":"derive_pq","valid_from":2010,"valid_to":null}}"#;
    let value: serde_json::Value = serde_json::from_str(raw).expect("json");
    let ops = remap_legacy_ops(&value, |s| dict.intern(s)).expect("remap");
    assert_eq!(ops.len(), 2);
    let digest = RuleAst::named("derive_pq").unwrap().hash();
    assert!(matches!(&ops[0], Op::StoreBlob { digest: d, .. } if d == &digest));
    match &ops[1] {
        Op::Assert {
            relation,
            delivery_key,
            ..
        } => {
            assert_eq!(dict.lookup(*relation), Some(KUTHA_RULE));
            assert_eq!(delivery_key.as_deref(), Some(digest.as_str()));
        }
        other => panic!("expected Assert, got {other:?}"),
    }
}

#[test]
fn register_rule_event_keeps_id_on_assert() {
    let mut dict = TermDictionary::default();
    let id = EventId::nil();
    let raw = format!(
        r#"{{"id":"{id}","op":{{"RegisterRule":{{"definition":"derive_pq","valid_from":2010,"valid_to":null}}}},"ingested_at":9,"object_ids":[]}}"#
    );
    let value: serde_json::Value = serde_json::from_str(&raw).expect("json");
    let events = remap_legacy_events(&value, |s| dict.intern(s)).expect("remap events");
    assert_eq!(events.len(), 2);
    assert!(matches!(events[0].op, Op::StoreBlob { .. }));
    assert_ne!(events[0].id, id, "blob gets a fresh id");
    assert_eq!(events[1].id, id, "Assert keeps legacy event id");
    assert!(matches!(events[1].op, Op::Assert { .. }));
    assert_eq!(events[1].ingested_at, 9);
}

#[test]
fn serde_open_path_still_rejects_register_rule() {
    let raw = r#"{"RegisterRule":{"definition":"derive_pq","valid_from":2010,"valid_to":null}}"#;
    assert!(
        serde_json::from_str::<Op>(raw).is_err(),
        "documented break: no serde alias"
    );
}
