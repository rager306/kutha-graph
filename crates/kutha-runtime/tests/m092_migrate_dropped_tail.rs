//! M092 S01 — CE #13: Op/Pin migrate reports surface dropped_tail for torn jsonl.

use kutha_runtime::store::{
    ensure_legacy_dir, migrate_op_wire, migrate_pin_cite, migrate_pin_wire, persist,
    OP_WIRE_MARKER_REL, PIN_CITE_MARKER_REL, PIN_WIRE_MARKER_REL,
};
use kutha_runtime::Runtime;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

fn tmp_dir(label: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("kutha-m092-{label}-{nanos}"));
    fs::create_dir_all(&dir).expect("mkdir");
    dir
}

fn append_torn_tail(dir: &std::path::Path) {
    let mut f = OpenOptions::new()
        .append(true)
        .open(dir.join("events.jsonl"))
        .expect("append");
    // Complete object without trailing newline — D-05 / CE #13 torn tail.
    write!(f, r#"{{"incomplete":true"#).expect("torn");
}

#[test]
fn migrate_op_wire_reports_dropped_jsonl_tail() {
    let dir = tmp_dir("op");
    ensure_legacy_dir(&dir).expect("legacy");
    let mut rt = Runtime::default();
    rt.assert("s", "relatedTo", "o", 2010, None).expect("assert");
    persist(&rt, &dir).expect("persist");
    // Fresh store has no op-wire marker; tear the jsonl then rewrite.
    let _ = fs::remove_file(dir.join(OP_WIRE_MARKER_REL));
    append_torn_tail(&dir);
    let report = migrate_op_wire(&dir).expect("migrate_op_wire");
    assert!(!report.already_migrated);
    assert_eq!(report.dropped_tail, 1, "torn jsonl tail must be counted");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn migrate_pin_wire_reports_dropped_jsonl_tail() {
    let dir = tmp_dir("pin-wire");
    ensure_legacy_dir(&dir).expect("legacy");
    let mut rt = Runtime::default();
    rt.assert("s", "relatedTo", "o", 2010, None).expect("assert");
    persist(&rt, &dir).expect("persist");
    // Pin-wire requires modern Events; stamp op-wire so pin path runs on jsonl.
    fs::write(dir.join(OP_WIRE_MARKER_REL), b"kutha-op-wire/1\n").expect("op marker");
    let _ = fs::remove_file(dir.join(PIN_WIRE_MARKER_REL));
    append_torn_tail(&dir);
    let report = migrate_pin_wire(&dir).expect("migrate_pin_wire");
    assert!(!report.already_migrated);
    assert_eq!(report.dropped_tail, 1);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn migrate_pin_cite_reports_dropped_jsonl_tail() {
    let dir = tmp_dir("pin-cite");
    ensure_legacy_dir(&dir).expect("legacy");
    let mut rt = Runtime::default();
    rt.assert("s", "relatedTo", "o", 2010, None).expect("assert");
    persist(&rt, &dir).expect("persist");
    fs::write(dir.join(OP_WIRE_MARKER_REL), b"kutha-op-wire/1\n").expect("op marker");
    fs::write(dir.join(PIN_WIRE_MARKER_REL), b"kutha-pin-wire/1\n").expect("wire marker");
    let _ = fs::remove_file(dir.join(PIN_CITE_MARKER_REL));
    append_torn_tail(&dir);
    let report = migrate_pin_cite(&dir).expect("migrate_pin_cite");
    assert!(!report.already_migrated);
    assert_eq!(report.dropped_tail, 1);
    let _ = fs::remove_dir_all(&dir);
}
