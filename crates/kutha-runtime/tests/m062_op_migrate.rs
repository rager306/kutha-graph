//! M062 S01 — explicit AllowRelation migrate; open/serde stay fail-open (m050).

use kutha_common::{
    remap_legacy_event_value, remap_legacy_op_value, EventId, Op, TermDictionary, KUTHA_ALLOWS,
};

#[test]
fn allow_relation_op_remaps_to_kutha_allows_assert() {
    let mut dict = TermDictionary::default();
    let raw = r#"{"AllowRelation":{"name":"relatedTo","valid_from":0,"valid_to":null}}"#;
    let value: serde_json::Value = serde_json::from_str(raw).expect("json");
    let op = remap_legacy_op_value(&value, |s| dict.intern(s)).expect("remap");
    match op {
        Op::Assert {
            subject,
            relation,
            object,
            delivery_key,
            ..
        } => {
            assert_eq!(subject, object);
            assert_eq!(dict.lookup(relation), Some(KUTHA_ALLOWS));
            assert_eq!(delivery_key.as_deref(), Some("relatedTo"));
        }
        other => panic!("expected Assert, got {other:?}"),
    }
}

#[test]
fn allow_relation_event_remaps_preserving_id() {
    let mut dict = TermDictionary::default();
    let id = EventId::nil();
    let raw = format!(
        r#"{{"id":"{id}","op":{{"AllowRelation":{{"name":"relatedTo","valid_from":0,"valid_to":null}}}},"ingested_at":7,"object_ids":[]}}"#
    );
    let value: serde_json::Value = serde_json::from_str(&raw).expect("json");
    let event = remap_legacy_event_value(&value, |s| dict.intern(s)).expect("remap event");
    assert_eq!(event.id, id);
    assert_eq!(event.ingested_at, 7);
    assert!(
        matches!(event.op, Op::Assert { .. }),
        "expected Assert after migrate"
    );
    assert!(
        event.object_ids.contains(&dict.id("relatedTo").unwrap()),
        "object_ids recomputed from remapped op"
    );
}

#[test]
fn serde_open_path_still_rejects_allow_relation() {
    let raw = r#"{"AllowRelation":{"name":"relatedTo","valid_from":0,"valid_to":null}}"#;
    assert!(
        serde_json::from_str::<Op>(raw).is_err(),
        "documented break: no serde alias"
    );
}
