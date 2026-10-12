//! M065 S01 — explicit RecordAdmission migrate; open/serde stay fail-open (m050).

use kutha_common::{
    remap_legacy_events, remap_legacy_ops, EventId, Op, SupportPolarity, TermDictionary,
    KUTHA_ADMISSION,
};

#[test]
fn record_admission_op_remaps_to_kutha_admission_assert() {
    let mut dict = TermDictionary::default();
    let raw = r#"{"RecordAdmission":{"justification_id":"j","admitted":true,"policy_version":"p","valid_from":2017,"valid_to":null}}"#;
    let value: serde_json::Value = serde_json::from_str(raw).expect("json");
    let ops = remap_legacy_ops(&value, |s| dict.intern(s)).expect("remap");
    assert_eq!(ops.len(), 1);
    match &ops[0] {
        Op::Assert {
            subject,
            relation,
            object,
            delivery_key,
            polarity,
            valid_from,
            ..
        } => {
            assert_eq!(dict.lookup(*subject), Some("j"));
            assert_eq!(dict.lookup(*relation), Some(KUTHA_ADMISSION));
            assert_eq!(dict.lookup(*object), Some("true"));
            assert_eq!(delivery_key.as_deref(), Some("p"));
            assert_eq!(*polarity, Some(SupportPolarity::Positive));
            assert_eq!(*valid_from, 2017);
        }
        other => panic!("expected Assert, got {other:?}"),
    }
}

#[test]
fn record_admission_event_keeps_id() {
    let mut dict = TermDictionary::default();
    let id = EventId::nil();
    let raw = format!(
        r#"{{"id":"{id}","op":{{"RecordAdmission":{{"justification_id":"j","admitted":false,"policy_version":"pin","valid_from":0,"valid_to":null}}}},"ingested_at":9,"object_ids":[]}}"#
    );
    let value: serde_json::Value = serde_json::from_str(&raw).expect("json");
    let events = remap_legacy_events(&value, |s| dict.intern(s)).expect("remap events");
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].id, id);
    assert_eq!(events[0].ingested_at, 9);
    match &events[0].op {
        Op::Assert {
            object,
            polarity,
            delivery_key,
            ..
        } => {
            assert_eq!(dict.lookup(*object), Some("false"));
            assert_eq!(*polarity, Some(SupportPolarity::Negative));
            assert_eq!(delivery_key.as_deref(), Some("pin"));
        }
        other => panic!("expected Assert, got {other:?}"),
    }
}

#[test]
fn serde_open_path_still_rejects_record_admission() {
    let raw = r#"{"RecordAdmission":{"justification_id":"j","admitted":true,"policy_version":"p","valid_from":0,"valid_to":null}}"#;
    assert!(
        serde_json::from_str::<Op>(raw).is_err(),
        "documented break: no serde alias"
    );
}
