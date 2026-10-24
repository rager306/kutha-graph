//! M078 S01 — kutha-migrate operator CLI runs migrate_compat.

use kutha_common::{
    legacy_ident_rule_pin, rule_definition_hash, Event, EventId, Op, OutcomeDisposition, RuleAst,
    KUTHA_ALLOWS,
};
use kutha_runtime::store::{
    ensure_legacy_dir, open, persist, OP_WIRE_MARKER_BODY, OP_WIRE_MARKER_REL,
    PIN_CITE_MARKER_BODY, PIN_CITE_MARKER_REL, PIN_WIRE_MARKER_BODY, PIN_WIRE_MARKER_REL,
};
use kutha_runtime::Runtime;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn tmp_dir(label: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("kutha-m078-{label}-{nanos}"));
    fs::create_dir_all(&dir).expect("mkdir");
    dir
}

fn append_jsonl_raw(dir: &std::path::Path, line: &str) {
    let mut f = fs::OpenOptions::new()
        .append(true)
        .open(dir.join("events.jsonl"))
        .expect("append");
    writeln!(f, "{line}").expect("nl");
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
            quantum_id: "m078".into(),
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

fn migrate_bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_kutha-migrate"))
}

#[test]
fn kutha_migrate_runs_compat_on_store_dir() {
    let dir = tmp_dir("cli");
    ensure_legacy_dir(&dir).expect("legacy");
    let mut rt = Runtime::default();
    let out = rt
        .assert("s-m078", "relatedTo", "o-m078", 2010, None)
        .expect("assert premise");
    let premise = out.receipt.event_ids[0];
    let subject = rt.intern("s-m078");
    let relation = rt.intern("relatedTo");
    let object = rt.intern("o-m078");
    persist(&rt, &dir).expect("persist");

    let allow_id = EventId::nil();
    append_jsonl_raw(
        &dir,
        &format!(
            r#"{{"id":"{allow_id}","op":{{"AllowRelation":{{"name":"extraRel","valid_from":0,"valid_to":null}}}},"ingested_at":999,"object_ids":[]}}"#
        ),
    );

    let legacy = legacy_ident_rule_pin("derive_pq").expect("legacy");
    let behavior_id = EventId::from_u128(42);
    let cite_id = EventId::from_u128(43);
    let [legacy_blob, v1_blob] = named_derive_pq_blobs(899);
    append_jsonl(
        &dir,
        &[
            legacy_blob,
            v1_blob,
            Event {
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
            },
            Event {
                id: cite_id,
                op: Op::JustificationCite {
                    justification_id: cite_id.to_string(),
                    target_claim: premise,
                    source_claim_ids: vec![premise],
                    source_event_ids: vec![premise],
                    rule_version: legacy,
                    tt: 900,
                    vt: 2010,
                },
                ingested_at: 901,
                object_ids: vec![],
            },
            commit_outcome(&[behavior_id, cite_id], 902),
        ],
    );

    assert!(open(&dir).is_err(), "open fail-open before CLI migrate");

    let output = Command::new(migrate_bin())
        .arg(&dir)
        .output()
        .expect("spawn kutha-migrate");
    assert!(
        output.status.success(),
        "kutha-migrate failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("migrate:"),
        "expected migrate: report line, got {stdout}"
    );
    assert!(stdout.contains("op_wire_already=0") || stdout.contains("op_wire_remapped="));

    assert_eq!(
        fs::read(dir.join(OP_WIRE_MARKER_REL)).expect("op-wire"),
        OP_WIRE_MARKER_BODY
    );
    assert_eq!(
        fs::read(dir.join(PIN_WIRE_MARKER_REL)).expect("pin-wire"),
        PIN_WIRE_MARKER_BODY
    );
    assert_eq!(
        fs::read(dir.join(PIN_CITE_MARKER_REL)).expect("pin-cite"),
        PIN_CITE_MARKER_BODY
    );

    let v1 = rule_definition_hash("derive_pq");
    let after = open(&dir).expect("open after CLI");
    assert!(after.log().as_slice().iter().any(|e| {
        matches!(
            &e.op,
            Op::Define { name } if name == "extraRel" || name == KUTHA_ALLOWS
        )
    }));
    assert!(after.log().as_slice().iter().any(|e| {
        matches!(
            &e.op,
            Op::Behavior { rule_version, name, .. }
                if name == "derive_pq" && rule_version == &v1
        ) && e.id == behavior_id
    }));

    let again = Command::new(migrate_bin())
        .arg(&dir)
        .output()
        .expect("spawn idempotent");
    assert!(again.status.success());
    let again_out = String::from_utf8_lossy(&again.stdout);
    assert!(
        again_out.contains("op_wire_already=1"),
        "idempotent report missing already=1: {again_out}"
    );

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn kutha_migrate_missing_dir_is_contract_error() {
    let missing = std::env::temp_dir().join(format!(
        "kutha-m078-missing-{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos()
    ));
    let output = Command::new(migrate_bin())
        .arg(&missing)
        .output()
        .expect("spawn");
    assert_eq!(output.status.code(), Some(2));
}
