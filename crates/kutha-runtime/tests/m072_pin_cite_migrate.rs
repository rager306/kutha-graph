//! M072 S01 — directory pin-cite migrate for JustificationCite rule_version.

use kutha_common::{
    legacy_ident_rule_pin, rule_definition_hash, Event, EventId, Op, OutcomeDisposition, RuleAst,
};
use kutha_runtime::store::{
    ensure_legacy_dir, migrate_pin_cite, open, persist, PIN_CITE_MARKER_BODY, PIN_CITE_MARKER_REL,
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
    let dir = std::env::temp_dir().join(format!("kutha-m072-{label}-{nanos}"));
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

fn commit_outcome(ids: &[EventId], ingested_at: u64) -> Event {
    Event {
        id: EventId::from_u128(ids[0].as_u128().wrapping_add(99)),
        op: Op::QuantumOutcome {
            quantum_id: "m072".into(),
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

fn seeded_store(label: &str) -> (PathBuf, EventId, u32, u32, u32) {
    let dir = tmp_dir(label);
    ensure_legacy_dir(&dir).expect("legacy");
    let mut rt = Runtime::default();
    let out = rt
        .assert("s-m072", "relatedTo", "o-m072", 2010, None)
        .expect("assert premise");
    let premise = out.receipt.event_ids[0];
    let subject = rt.intern("s-m072");
    let relation = rt.intern("relatedTo");
    let object = rt.intern("o-m072");
    persist(&rt, &dir).expect("persist");
    (dir, premise, subject, relation, object)
}

#[test]
fn migrate_pin_cite_rewrites_justification_legacy_utf8_pin() {
    let (dir, premise, subject, relation, object) = seeded_store("cite");
    let legacy = legacy_ident_rule_pin("derive_pq").expect("legacy");
    let behavior_id = EventId::from_u128(42);
    let cite_id = EventId::from_u128(43);
    let claim = premise;
    let behavior = Event {
        id: behavior_id,
        op: Op::Behavior {
            name: "derive_pq".into(),
            caused_by: premise,
            // Already v1 — name still evidences the ident for cite remap.
            rule_version: rule_definition_hash("derive_pq"),
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
            target_claim: claim,
            source_claim_ids: vec![claim],
            source_event_ids: vec![premise],
            rule_version: legacy.clone(),
            tt: 900,
            vt: 2010,
        },
        ingested_at: 901,
        object_ids: vec![],
    };
    let v1_blob = named_derive_pq_blob(899);
    append_jsonl(
        &dir,
        &[
            v1_blob,
            behavior,
            cite,
            commit_outcome(&[behavior_id, cite_id], 902),
        ],
    );

    let before = open(&dir).expect("open with legacy cite pin");
    assert!(before.log().as_slice().iter().any(|e| {
        matches!(
            &e.op,
            Op::JustificationCite { rule_version, .. } if rule_version == &legacy
        ) && e.id == cite_id
    }));

    let report = migrate_pin_cite(&dir).expect("migrate_pin_cite");
    assert!(!report.already_migrated);
    assert_eq!(report.remapped_lines, 1);
    assert_eq!(
        fs::read(dir.join(PIN_CITE_MARKER_REL)).expect("marker"),
        PIN_CITE_MARKER_BODY
    );

    let v1 = rule_definition_hash("derive_pq");
    let after = open(&dir).expect("open after pin-cite");
    assert!(after.log().as_slice().iter().any(|e| {
        matches!(
            &e.op,
            Op::JustificationCite { rule_version, .. } if rule_version == &v1
        ) && e.id == cite_id
    }));

    let again = migrate_pin_cite(&dir).expect("idempotent");
    assert!(again.already_migrated);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn migrate_pin_cite_leaves_orphan_legacy_cite_unchanged() {
    let (dir, premise, _s, _r, _o) = seeded_store("orphan");
    let legacy = legacy_ident_rule_pin("derive_pq").expect("legacy");
    let cite_id = EventId::from_u128(77);
    // No Behavior/Define evidencing derive_pq — cite cannot be remapped honestly.
    let cite = Event {
        id: cite_id,
        op: Op::JustificationCite {
            justification_id: cite_id.to_string(),
            target_claim: premise,
            source_claim_ids: vec![premise],
            source_event_ids: vec![premise],
            rule_version: legacy.clone(),
            tt: 50,
            vt: 2010,
        },
        ingested_at: 50,
        object_ids: vec![],
    };
    append_jsonl(&dir, &[cite, commit_outcome(&[cite_id], 51)]);
    let report = migrate_pin_cite(&dir).expect("migrate");
    assert_eq!(report.remapped_lines, 0);
    assert!(dir.join(PIN_CITE_MARKER_REL).exists());
    let opened = open(&dir).expect("open");
    assert!(opened.log().as_slice().iter().any(|e| {
        matches!(
            &e.op,
            Op::JustificationCite { rule_version, .. } if rule_version == &legacy
        )
    }));
    let _ = fs::remove_dir_all(&dir);
}
