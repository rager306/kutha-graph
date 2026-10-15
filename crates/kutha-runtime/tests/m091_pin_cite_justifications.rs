//! M091 S01 — CE #1: migrate_pin_cite must not brick reopen with a stale
//! justifications.jsonl sidecar (non-empty lease after pre-migrate persist).

use kutha_common::{
    legacy_ident_rule_pin, rule_definition_hash, Event, EventId, Op, OutcomeDisposition,
};
use kutha_runtime::store::{
    ensure_legacy_dir, migrate_pin_cite, open, persist, JUSTIFICATIONS_REL,
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
    let dir = std::env::temp_dir().join(format!("kutha-m091-{label}-{nanos}"));
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
            quantum_id: "m091".into(),
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
fn migrate_pin_cite_reopens_after_nonempty_justifications_sidecar() {
    let dir = tmp_dir("sidecar");
    ensure_legacy_dir(&dir).expect("legacy");
    let mut rt = Runtime::default();
    let out = rt
        .assert("s-m091", "relatedTo", "o-m091", 2010, None)
        .expect("assert premise");
    let premise = out.receipt.event_ids[0];
    let subject = rt.intern("s-m091");
    let relation = rt.intern("relatedTo");
    let object = rt.intern("o-m091");
    persist(&rt, &dir).expect("seed persist");

    let legacy = legacy_ident_rule_pin("derive_pq").expect("legacy");
    let behavior_id = EventId::from_u128(42);
    let cite_id = EventId::from_u128(43);
    let behavior = Event {
        id: behavior_id,
        op: Op::Behavior {
            name: "derive_pq".into(),
            caused_by: premise,
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

    let with_cites = open(&dir).expect("open with legacy cite (no sidecar yet)");
    persist(&with_cites, &dir).expect("persist writes justifications sidecar");
    let sidecar = dir.join(JUSTIFICATIONS_REL);
    let bytes = fs::read(&sidecar).expect("justifications.jsonl after persist");
    assert!(
        !bytes.is_empty(),
        "oracle requires non-empty justifications sidecar"
    );
    assert!(
        std::str::from_utf8(&bytes)
            .expect("utf8")
            .contains(legacy.as_str())
            || bytes.len() > 2,
        "sidecar should retain cite lease bytes"
    );

    let report = migrate_pin_cite(&dir).expect("migrate_pin_cite");
    assert!(!report.already_migrated);
    assert_eq!(report.remapped_lines, 1);
    assert!(
        !sidecar.exists(),
        "stale justifications.jsonl must be removed (regenerable lease)"
    );

    let after = open(&dir).expect("open after pin-cite with prior non-empty sidecar");
    let v1 = rule_definition_hash("derive_pq");
    assert!(after.log().as_slice().iter().any(|e| {
        matches!(
            &e.op,
            Op::JustificationCite { rule_version, .. } if rule_version == &v1
        ) && e.id == cite_id
    }));

    let _ = fs::remove_dir_all(&dir);
}
