//! M075 S01 — wire-then-cite pin migrate sequence in one entry.

use kutha_common::{
    legacy_ident_rule_pin, rule_definition_hash, Event, EventId, Op, OutcomeDisposition,
};
use kutha_runtime::store::{
    ensure_legacy_dir, migrate_pins, open, persist, PIN_CITE_MARKER_BODY, PIN_CITE_MARKER_REL,
    PIN_WIRE_MARKER_BODY, PIN_WIRE_MARKER_REL,
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
    let dir = std::env::temp_dir().join(format!("kutha-m075-{label}-{nanos}"));
    fs::create_dir_all(&dir).expect("mkdir");
    dir
}

fn append_jsonl(dir: &std::path::Path, events: &[Event]) {
    let mut f = fs::OpenOptions::new()
        .append(true)
        .open(dir.join("events.jsonl"))
        .expect("append");
    for e in events {
        serde_json::to_writer(&mut f, e).expect("json");
        writeln!(f).expect("nl");
    }
}

fn commit_outcome(ids: &[EventId], ingested_at: u64) -> Event {
    Event {
        id: EventId::from_u128(ids[0].as_u128().wrapping_add(99)),
        op: Op::QuantumOutcome {
            quantum_id: "m075".into(),
            disposition: OutcomeDisposition::Full,
            aborted_on_budget: false,
            events_in_quantum: ids.len(),
            event_ids: ids.to_vec(),
            receipt_digest_hex: "00".into(),
            resume_of: None,
        },
        ingested_at,
        object_ids: vec![],
    }
}

#[test]
fn migrate_pins_rewrites_behavior_then_cite_in_one_call() {
    let dir = tmp_dir("both");
    ensure_legacy_dir(&dir).expect("legacy");
    let mut rt = Runtime::default();
    let out = rt
        .assert("s-m075", "relatedTo", "o-m075", 2010, None)
        .expect("assert premise");
    let premise = out.receipt.event_ids[0];
    let subject = rt.intern("s-m075");
    let relation = rt.intern("relatedTo");
    let object = rt.intern("o-m075");
    persist(&rt, &dir).expect("persist");

    let legacy = legacy_ident_rule_pin("derive_pq").expect("legacy");
    let behavior_id = EventId::from_u128(42);
    let cite_id = EventId::from_u128(43);
    let behavior = Event {
        id: behavior_id,
        op: Op::Behavior {
            name: "derive_pq".into(),
            caused_by: premise,
            rule_version: legacy.clone(),
            subject,
            relation,
            object,
            valid_from: 2010,
            valid_to: None,
        },
        ingested_at: 900,
        object_ids: vec![subject, relation, object],
    };
    let cite = Event {
        id: cite_id,
        op: Op::JustificationCite {
            justification_id: cite_id.to_string(),
            target_claim: premise,
            source_claim_ids: vec![premise],
            source_event_ids: vec![premise],
            rule_version: legacy.clone(),
            tt: 900,
            vt: 2010,
        },
        ingested_at: 901,
        object_ids: vec![],
    };
    append_jsonl(
        &dir,
        &[
            behavior,
            cite,
            commit_outcome(&[behavior_id, cite_id], 902),
        ],
    );

    let report = migrate_pins(&dir).expect("migrate_pins");
    assert!(!report.wire.already_migrated);
    assert!(!report.cite.already_migrated);
    assert_eq!(report.wire.remapped_lines, 1);
    assert!(report.cite.remapped_lines >= 1);
    assert_eq!(
        fs::read(dir.join(PIN_WIRE_MARKER_REL)).expect("wire marker"),
        PIN_WIRE_MARKER_BODY
    );
    assert_eq!(
        fs::read(dir.join(PIN_CITE_MARKER_REL)).expect("cite marker"),
        PIN_CITE_MARKER_BODY
    );

    let v1 = rule_definition_hash("derive_pq");
    let after = open(&dir).expect("open after migrate_pins");
    assert!(after.log().as_slice().iter().any(|e| {
        matches!(
            &e.op,
            Op::Behavior { rule_version, name, .. }
                if name == "derive_pq" && rule_version == &v1
        ) && e.id == behavior_id
    }));
    assert!(after.log().as_slice().iter().any(|e| {
        matches!(
            &e.op,
            Op::JustificationCite { rule_version, .. } if rule_version == &v1
        ) && e.id == cite_id
    }));

    let again = migrate_pins(&dir).expect("idempotent");
    assert!(again.wire.already_migrated);
    assert!(again.cite.already_migrated);

    let _ = fs::remove_dir_all(&dir);
}
