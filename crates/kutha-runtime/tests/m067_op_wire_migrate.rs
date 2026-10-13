//! M067 S01 — directory Op-wire migrate; open stays fail-open until rewrite.

use kutha_common::{EventId, Op, KUTHA_ALLOWS};
use kutha_runtime::store::{
    ensure_legacy_dir, migrate_op_wire, open, persist, OP_WIRE_MARKER_BODY, OP_WIRE_MARKER_REL,
};
use kutha_runtime::Runtime;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

fn tmp_dir(label: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("kutha-m067-{label}-{nanos}"));
    fs::create_dir_all(&dir).expect("mkdir");
    dir
}

#[test]
fn migrate_op_wire_rewrites_allow_relation_jsonl_and_stamps_marker() {
    let dir = tmp_dir("allow");
    // Genesis persist on a legacy layout so open's from_dict_and_events has
    // knows/knownBy; then append one main-era meta-op line.
    ensure_legacy_dir(&dir).expect("legacy layout");
    let rt = Runtime::default();
    persist(&rt, &dir).expect("persist genesis");
    let id = EventId::nil();
    let legacy = format!(
        r#"{{"id":"{id}","op":{{"AllowRelation":{{"name":"extraRel","valid_from":0,"valid_to":null}}}},"ingested_at":999,"object_ids":[]}}"#
    );
    {
        let mut f = fs::OpenOptions::new()
            .append(true)
            .open(dir.join("events.jsonl"))
            .expect("append jsonl");
        writeln!(f, "{legacy}").expect("write legacy line");
    }

    assert!(
        open(&dir).is_err(),
        "open must stay fail-open before migrate_op_wire"
    );

    let report = migrate_op_wire(&dir).expect("migrate_op_wire");
    assert!(!report.already_migrated);
    assert_eq!(report.remapped_lines, 1);
    assert!(report.events_written > 1);

    let marker = fs::read(dir.join(OP_WIRE_MARKER_REL)).expect("marker");
    assert_eq!(marker, OP_WIRE_MARKER_BODY);

    let opened = open(&dir).expect("open after migrate_op_wire");
    assert!(
        opened.log().as_slice().iter().any(|e| {
            matches!(
                &e.op,
                Op::Define { name } if name == "extraRel" || name == KUTHA_ALLOWS
            )
        }),
        "rewritten log must Define remapped terms"
    );
    assert!(
        opened.log().as_slice().iter().any(|e| {
            matches!(
                &e.op,
                Op::Assert {
                    delivery_key: Some(k),
                    ..
                } if k == "extraRel"
            ) && e.id == id
        }),
        "rewritten log must keep AllowRelation id on kutha:allows Assert"
    );

    let again = migrate_op_wire(&dir).expect("idempotent");
    assert!(again.already_migrated);
    assert_eq!(again.remapped_lines, 0);

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn migrate_op_wire_rejects_segmented_dir() {
    let dir = tmp_dir("seg");
    fs::write(dir.join("manifest.json"), "{}\n").expect("manifest");
    fs::write(dir.join("events.jsonl"), "").expect("jsonl");
    let err = migrate_op_wire(&dir).expect_err("segmented");
    assert_eq!(err.kind(), std::io::ErrorKind::InvalidData);
    assert!(
        err.to_string().contains("legacy jsonl"),
        "{}",
        err
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn open_still_fails_on_unmigrated_record_action_line() {
    let dir = tmp_dir("failopen");
    let raw = r#"{"id":"00000000-0000-0000-0000-000000000000","op":{"RecordAction":{"justification_id":"j","target_claim":"00000000-0000-0000-0000-000000000001","admitted":true,"valid_from":0,"valid_to":null}},"ingested_at":1,"object_ids":[]}"#;
    fs::write(dir.join("events.jsonl"), format!("{raw}\n")).expect("write");
    assert!(open(&dir).is_err());
    let _ = fs::remove_dir_all(&dir);
}