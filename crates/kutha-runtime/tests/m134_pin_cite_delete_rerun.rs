//! M134 S01 / Phase 208 — after pin-cite.v1 is stamped, later evidence does not
//! rematerialize orphans until the operator deletes the marker (CE residual).

use kutha_common::{
    legacy_ident_rule_pin, rule_definition_hash, Event, EventId, Op, OutcomeDisposition, RuleAst,
};
use kutha_runtime::store::{
    ensure_legacy_dir, migrate_pin_cite, open, persist, PIN_CITE_MARKER_REL,
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
    let dir = std::env::temp_dir().join(format!("kutha-m134-{label}-{nanos}"));
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

fn named_derive_pq_blobs(ingested_at: u64) -> [Event; 2] {
    let ast = RuleAst::named("derive_pq").expect("named");
    let bytes = ast.encode();
    let mk = |id, digest: String| Event {
        id: EventId::from_u128(id),
        op: Op::StoreBlob {
            digest,
            bytes: bytes.clone(),
        },
        ingested_at,
        object_ids: vec![],
    };
    [
        mk(
            0xB10B_0001,
            legacy_ident_rule_pin("derive_pq").expect("legacy"),
        ),
        mk(0xB10B_0002, ast.hash()),
    ]
}

fn commit_outcome(ids: &[EventId], ingested_at: u64) -> Event {
    Event {
        id: EventId::from_u128(ids[0].as_u128().wrapping_add(99)),
        op: Op::QuantumOutcome {
            quantum_id: "m134".into(),
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

fn cite_rule_version(log: &[Event], cite_id: EventId) -> Option<String> {
    log.iter().find_map(|e| match (&e.id, &e.op) {
        (id, Op::JustificationCite { rule_version, .. }) if *id == cite_id => {
            Some(rule_version.clone())
        }
        _ => None,
    })
}

#[test]
fn migrate_pin_cite_requires_delete_marker_to_rematerialize_after_new_evidence() {
    let dir = tmp_dir("delete-rerun");
    ensure_legacy_dir(&dir).expect("legacy");
    let mut rt = Runtime::default();
    let out = rt
        .assert("s-m134", "relatedTo", "o-m134", 2010, None)
        .expect("assert premise");
    let premise = out.receipt.event_ids[0];
    let subject = rt.intern("s-m134");
    let relation = rt.intern("relatedTo");
    let object = rt.intern("o-m134");
    persist(&rt, &dir).expect("persist");

    let legacy = legacy_ident_rule_pin("derive_pq").expect("legacy");
    let cite_id = EventId::from_u128(134);
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

    let first = migrate_pin_cite(&dir).expect("orphan migrate stamps marker");
    assert!(!first.already_migrated);
    assert_eq!(first.remapped_lines, 0);
    assert!(dir.join(PIN_CITE_MARKER_REL).exists());
    assert_eq!(
        cite_rule_version(open(&dir).expect("open").log().as_slice(), cite_id).as_deref(),
        Some(legacy.as_str())
    );

    // Evidence arrives after the marker — must not apply on a blind rerun.
    let behavior_id = EventId::from_u128(135);
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
    let [legacy_blob, v1_blob] = named_derive_pq_blobs(899);
    append_jsonl(
        &dir,
        &[legacy_blob, v1_blob, behavior, commit_outcome(&[behavior_id], 901)],
    );

    let blocked = migrate_pin_cite(&dir).expect("marker blocks rematerialize");
    assert!(blocked.already_migrated);
    assert_eq!(blocked.remapped_lines, 0);
    assert_eq!(
        cite_rule_version(open(&dir).expect("open blocked").log().as_slice(), cite_id)
            .as_deref(),
        Some(legacy.as_str()),
        "orphan cite must stay legacy while pin-cite.v1 exists"
    );

    fs::remove_file(dir.join(PIN_CITE_MARKER_REL)).expect("delete pin-cite.v1");
    let rematerialized = migrate_pin_cite(&dir).expect("rerun after delete");
    assert!(!rematerialized.already_migrated);
    assert_eq!(rematerialized.remapped_lines, 1);
    let v1 = rule_definition_hash("derive_pq");
    assert_eq!(
        cite_rule_version(
            open(&dir).expect("open rematerialized").log().as_slice(),
            cite_id
        )
        .as_deref(),
        Some(v1.as_str())
    );

    let _ = fs::remove_dir_all(&dir);
}
