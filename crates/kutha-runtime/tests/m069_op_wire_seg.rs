//! M069 S01 — segmented Op-wire migrate; open stays fail-open until rewrite.

use kutha_common::{EventId, Op, KUTHA_ALLOWS};
use kutha_runtime::store::{
    migrate_op_wire, open, write_json_payload_segmented, OP_WIRE_MARKER_BODY, OP_WIRE_MARKER_REL,
    MANIFEST_REL,
};
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

fn tmp_dir(label: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("kutha-m069-{label}-{nanos}"));
    fs::create_dir_all(&dir).expect("mkdir");
    dir
}

#[test]
fn migrate_op_wire_rewrites_segmented_allow_relation() {
    let dir = tmp_dir("seg");
    let id = EventId::nil();
    let define_knows = br#"{"id":"00000000-0000-0000-0000-000000000010","op":{"Define":{"name":"knows"}},"ingested_at":1,"object_ids":[]}"#;
    let define_known_by = br#"{"id":"00000000-0000-0000-0000-000000000011","op":{"Define":{"name":"knownBy"}},"ingested_at":2,"object_ids":[]}"#;
    let legacy = format!(
        r#"{{"id":"{id}","op":{{"AllowRelation":{{"name":"segRel","valid_from":0,"valid_to":null}}}},"ingested_at":3,"object_ids":[]}}"#
    );
    let sealed: &[&[u8]] = &[define_knows, define_known_by, legacy.as_bytes()];
    write_json_payload_segmented(&dir, &[sealed], &[]).expect("segmented fixture");
    assert!(dir.join(MANIFEST_REL).exists());
    assert!(!dir.join("events.jsonl").exists());

    assert!(
        open(&dir).is_err(),
        "open must stay fail-open on sealed legacy meta-op frame"
    );

    let report = migrate_op_wire(&dir).expect("migrate_op_wire");
    assert!(!report.already_migrated);
    assert_eq!(report.remapped_lines, 1);
    assert!(report.events_written >= 3);
    assert!(dir.join("events.jsonl").exists());
    assert!(!dir.join(MANIFEST_REL).exists());
    assert_eq!(
        fs::read(dir.join(OP_WIRE_MARKER_REL)).expect("marker"),
        OP_WIRE_MARKER_BODY
    );

    let opened = open(&dir).expect("open after segmented migrate_op_wire");
    assert!(opened.log().as_slice().iter().any(|e| {
        matches!(
            &e.op,
            Op::Assert {
                delivery_key: Some(k),
                ..
            } if k == "segRel"
        ) && e.id == id
    }));
    assert!(opened.log().as_slice().iter().any(|e| {
        matches!(&e.op, Op::Define { name } if name == "segRel" || name == KUTHA_ALLOWS)
    }));

    let again = migrate_op_wire(&dir).expect("idempotent");
    assert!(again.already_migrated);

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn open_still_fails_on_unmigrated_segmented_record_action() {
    let dir = tmp_dir("failopen");
    let raw: &[u8] = br#"{"id":"00000000-0000-0000-0000-000000000000","op":{"RecordAction":{"justification_id":"j","target_claim":"00000000-0000-0000-0000-000000000001","admitted":true,"valid_from":0,"valid_to":null}},"ingested_at":1,"object_ids":[]}"#;
    write_json_payload_segmented(&dir, &[&[raw]], &[]).expect("seg");
    assert!(open(&dir).is_err());
    let _ = fs::remove_dir_all(&dir);
}
