//! M071 S01 — directory pin-wire migrate for Behavior rule_version.

use kutha_common::{
    Event, EventId, Op, OutcomeDisposition, RuleAst, legacy_ident_rule_pin, rule_definition_hash,
};
use kutha_runtime::Runtime;
use kutha_runtime::store::{
    PIN_WIRE_MARKER_BODY, PIN_WIRE_MARKER_REL, ensure_legacy_dir, migrate_pin_wire, open, persist,
};
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

fn tmp_dir(label: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("kutha-m071-{label}-{nanos}"));
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

/// Matching v1 Named `derive_pq` body. Do not key a blob with the legacy pin:
/// `blob_digest_matches` rejects that pair, and production must not special-case it.
fn named_derive_pq_blob(ingested_at: u64) -> Event {
    let ast = RuleAst::named("derive_pq").expect("named");
    Event {
        id: EventId::from_u128(0xB10B_0002),
        op: Op::StoreBlob {
            digest: ast.hash(),
            bytes: ast.encode(),
        },
        ingested_at,
        object_ids: vec![],
    }
}

fn jsonl_events(dir: &std::path::Path) -> Vec<Event> {
    let text = fs::read_to_string(dir.join("events.jsonl")).expect("jsonl");
    text.lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(l).expect("event"))
        .collect()
}

fn commit_outcome(behavior_id: EventId, ingested_at: u64) -> Event {
    Event {
        id: EventId::from_u128(behavior_id.as_u128().wrapping_add(1)),
        op: Op::QuantumOutcome {
            quantum_id: "m071".into(),
            disposition: OutcomeDisposition::Full,
            aborted_on_budget: false,
            events_in_quantum: 1,
            event_ids: vec![behavior_id],
            receipt_digest_hex: "00".into(),
            resume_of: None,
        },
        ingested_at,
        object_ids: vec![],
    }
}

/// Genesis + one Assert premise so Behavior `caused_by` lineage verifies on open.
fn seeded_store(label: &str) -> (PathBuf, EventId, u32, u32, u32) {
    let dir = tmp_dir(label);
    ensure_legacy_dir(&dir).expect("legacy");
    let mut rt = Runtime::default();
    let out = rt
        .assert("s-m071", "relatedTo", "o-m071", 2010, None)
        .expect("assert premise");
    let premise = out.receipt.event_ids[0];
    let subject = rt.intern("s-m071");
    let relation = rt.intern("relatedTo");
    let object = rt.intern("o-m071");
    persist(&rt, &dir).expect("persist");
    (dir, premise, subject, relation, object)
}

#[test]
fn migrate_pin_wire_rewrites_behavior_legacy_utf8_pin() {
    let (dir, premise, subject, relation, object) = seeded_store("pin");

    let legacy = legacy_ident_rule_pin("derive_pq").expect("legacy pin");
    let id = EventId::from_u128(42);
    let behavior = Event {
        id,
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
        ingested_at: 999,
        object_ids: vec![subject, relation, object],
    };
    let v1_blob = named_derive_pq_blob(998);
    append_jsonl(&dir, &[v1_blob, behavior, commit_outcome(id, 1000)]);

    let before = jsonl_events(&dir);
    assert!(before.iter().any(|e| {
        matches!(
            &e.op,
            Op::Behavior { rule_version, .. } if rule_version == &legacy
        ) && e.id == id
    }));

    let report = migrate_pin_wire(&dir).expect("migrate_pin_wire");
    assert!(!report.already_migrated);
    assert_eq!(report.remapped_lines, 1);
    assert_eq!(
        fs::read(dir.join(PIN_WIRE_MARKER_REL)).expect("marker"),
        PIN_WIRE_MARKER_BODY
    );

    let v1 = rule_definition_hash("derive_pq");
    let after = open(&dir).expect("open after pin migrate");
    assert!(after.log().as_slice().iter().any(|e| {
        matches!(
            &e.op,
            Op::Behavior { rule_version, name, .. }
                if name == "derive_pq" && rule_version == &v1
        ) && e.id == id
    }));

    let again = migrate_pin_wire(&dir).expect("idempotent");
    assert!(again.already_migrated);

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn migrate_pin_wire_leaves_v1_pin_unchanged() {
    let (dir, premise, subject, relation, object) = seeded_store("v1");
    let v1 = rule_definition_hash("derive_pq");
    let id = EventId::from_u128(7);
    let behavior = Event {
        id,
        op: Op::Behavior {
            name: "derive_pq".into(),
            caused_by: premise,
            rule_version: v1.clone(),
            subject,
            relation,
            object,
            valid_from: 2010,
            valid_to: None,
        },
        ingested_at: 50,
        object_ids: vec![subject, relation, object],
    };
    let v1_blob = named_derive_pq_blob(49);
    append_jsonl(&dir, &[v1_blob, behavior, commit_outcome(id, 51)]);
    let report = migrate_pin_wire(&dir).expect("migrate");
    assert_eq!(report.remapped_lines, 0);
    assert!(dir.join(PIN_WIRE_MARKER_REL).exists());
    let opened = open(&dir).expect("open");
    assert!(
        opened
            .log()
            .as_slice()
            .iter()
            .any(|e| { matches!(&e.op, Op::Behavior { rule_version, .. } if rule_version == &v1) })
    );
    let _ = fs::remove_dir_all(&dir);
}
