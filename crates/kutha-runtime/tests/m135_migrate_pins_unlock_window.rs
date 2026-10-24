//! M135 S01 / Phase 209 — migrate_pins releases kutha.lock between wire and
//! cite (CE residual: unlock window untested; converge via markers).

use kutha_common::{
    legacy_ident_rule_pin, rule_definition_hash, Event, EventId, Op, OutcomeDisposition, RuleAst,
};
use kutha_runtime::store::{
    acquire_store_write_lock, ensure_legacy_dir, migrate_pin_cite, migrate_pin_wire, migrate_pins,
    open, persist, PIN_CITE_MARKER_REL, PIN_WIRE_MARKER_REL, STORE_LOCK_REL,
};
use kutha_runtime::Runtime;
use std::fs;
use std::io::{ErrorKind, Write};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

fn tmp_dir(label: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("kutha-m135-{label}-{nanos}"));
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
            quantum_id: "m135".into(),
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

fn fn_body<'a>(src: &'a str, name: &str) -> &'a str {
    let start = src
        .find(&format!("pub fn {name}("))
        .unwrap_or_else(|| panic!("missing {name}"));
    let rest = &src[start..];
    let end = rest[1..]
        .find("\npub fn ")
        .or_else(|| rest[1..].find("\nfn "))
        .map(|i| i + 1)
        .unwrap_or(rest.len());
    &rest[..end]
}

fn seeded_both(label: &str) -> (PathBuf, EventId, EventId) {
    let dir = tmp_dir(label);
    ensure_legacy_dir(&dir).expect("legacy");
    let mut rt = Runtime::default();
    let out = rt
        .assert("s-m135", "relatedTo", "o-m135", 2010, None)
        .expect("assert premise");
    let premise = out.receipt.event_ids[0];
    let subject = rt.intern("s-m135");
    let relation = rt.intern("relatedTo");
    let object = rt.intern("o-m135");
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
            rule_version: legacy,
            tt: 900,
            vt: 2010,
        },
        ingested_at: 901,
        object_ids: vec![],
    };
    let [legacy_blob, v1_blob] = named_derive_pq_blobs(899);
    append_jsonl(
        &dir,
        &[
            legacy_blob,
            v1_blob,
            behavior,
            cite,
            commit_outcome(&[behavior_id, cite_id], 902),
        ],
    );
    (dir, behavior_id, cite_id)
}

#[test]
fn migrate_pins_and_compat_do_not_take_parent_lock() {
    let src = include_str!("../src/store.rs");
    for name in ["migrate_pins", "migrate_compat"] {
        let body = fn_body(src, name);
        assert!(
            !body.contains("acquire_store_write_lock"),
            "{name} must not wrap steps under a parent lock (nested try_lock WouldBlock); \
             unlock between callees is intentional"
        );
    }
}

#[test]
fn migrate_pins_gap_blocks_peer_cite_then_converges() {
    let (dir, behavior_id, cite_id) = seeded_both("gap");
    let wire = migrate_pin_wire(&dir).expect("wire half of unlock window");
    assert!(!wire.already_migrated);
    assert!(dir.join(PIN_WIRE_MARKER_REL).exists());
    assert!(!dir.join(PIN_CITE_MARKER_REL).exists());

    // Peer occupying the unlock gap: cite must fail closed while lock is held.
    let held = acquire_store_write_lock(&dir).expect("hold gap lock");
    let blocked = migrate_pin_cite(&dir).expect_err("cite under held lock");
    assert_eq!(blocked.kind(), ErrorKind::WouldBlock);
    assert!(
        blocked.to_string().contains(STORE_LOCK_REL),
        "error must name the lock file; got {blocked}"
    );
    drop(held);

    let cite = migrate_pin_cite(&dir).expect("cite after gap");
    assert!(!cite.already_migrated);
    assert!(dir.join(PIN_CITE_MARKER_REL).exists());

    let facade = migrate_pins(&dir).expect("facade converges");
    assert!(facade.wire.already_migrated);
    assert!(facade.cite.already_migrated);

    let v1 = rule_definition_hash("derive_pq");
    let after = open(&dir).expect("open");
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

    let _ = fs::remove_dir_all(&dir);
}
