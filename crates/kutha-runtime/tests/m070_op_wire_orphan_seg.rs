//! M070 S01 — segmented Op-wire demote removes orphan seg-*.log files.

use kutha_common::EventId;
use kutha_runtime::store::{
    migrate_op_wire, open, write_json_payload_segmented, MANIFEST_REL, OP_WIRE_MARKER_REL,
};
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

fn tmp_dir(label: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("kutha-m070-{label}-{nanos}"));
    fs::create_dir_all(&dir).expect("mkdir");
    dir
}

fn seg_logs(dir: &std::path::Path) -> Vec<String> {
    let mut names = Vec::new();
    for entry in fs::read_dir(dir).expect("read_dir") {
        let entry = entry.expect("entry");
        let name = entry.file_name();
        let s = name.to_string_lossy();
        if s.starts_with("seg-") && s.ends_with(".log") {
            names.push(s.into_owned());
        }
    }
    names.sort();
    names
}

#[test]
fn migrate_op_wire_removes_orphan_generation_unique_seg_files() {
    let dir = tmp_dir("orphan");
    let id = EventId::nil();
    let define_knows = br#"{"id":"00000000-0000-0000-0000-000000000010","op":{"Define":{"name":"knows"}},"ingested_at":1,"object_ids":[]}"#;
    let define_known_by = br#"{"id":"00000000-0000-0000-0000-000000000011","op":{"Define":{"name":"knownBy"}},"ingested_at":2,"object_ids":[]}"#;
    let legacy = format!(
        r#"{{"id":"{id}","op":{{"AllowRelation":{{"name":"orphanRel","valid_from":0,"valid_to":null}}}},"ingested_at":3,"object_ids":[]}}"#
    );
    let sealed: &[&[u8]] = &[define_knows, define_known_by, legacy.as_bytes()];
    write_json_payload_segmented(&dir, &[sealed], &[]).expect("segmented");

    // Crash/rollover leftover not named by the current manifest.
    let orphan = dir.join("seg-00000099-deadbeefcafebabe.log");
    fs::write(&orphan, b"orphan-not-in-manifest").expect("orphan");
    assert!(orphan.exists());
    assert!(
        open(&dir).is_err(),
        "open must stay fail-open before migrate"
    );

    let report = migrate_op_wire(&dir).expect("migrate_op_wire");
    assert!(!report.already_migrated);
    assert!(!dir.join(MANIFEST_REL).exists());
    assert!(dir.join("events.jsonl").exists());
    assert!(dir.join(OP_WIRE_MARKER_REL).exists());
    assert!(
        seg_logs(&dir).is_empty(),
        "demote must remove orphan seg-*.log, left {:?}",
        seg_logs(&dir)
    );
    assert!(!orphan.exists());

    open(&dir).expect("open after demote");
    let _ = fs::remove_dir_all(&dir);
}
