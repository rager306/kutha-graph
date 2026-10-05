//! Independent RULE-03/04 kill-tests. Lazy "leased-policy still admits under deny" or empty-pin inverse pass fail these.

use kutha_common::{Event, EventId, Op, TransactionTime, ValidTime, rule_definition_hash};
use kutha_runtime::{GraphFold, Runtime, RuntimeError, Snapshot, store};
use std::fs;

fn leased_pin_and_derive() -> (Runtime, EventId, EventId) {
    let mut rt = Runtime::default();
    let p = rt.intern("P");
    let q = rt.intern("Q");
    let rel = rt.intern("relatedTo");
    let true_ = rt.intern("true");
    rt.emit_rule("derive_pq", 2010, None).unwrap();
    rt.emit_policy("leased-policy", 2010, None).unwrap();
    let asserted = rt
        .emit(Op::Assert {
            subject: p,
            relation: rel,
            object: true_,
            valid_from: 2010,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap();
    let cause = asserted.receipt.event_ids[0];
    let derived = rt
        .emit(Op::Behavior {
            name: "derive_pq".into(),
            caused_by: cause,
            rule_version: rule_definition_hash("derive_pq"),
            subject: q,
            relation: rel,
            object: true_,
            valid_from: 2010,
            valid_to: None,
        })
        .unwrap();
    (rt, derived.receipt.event_ids[0], cause)
}

#[test]
fn deny_body_fails_check_admission_leased_policy_admits() {
    let (mut rt, claim_q, cause) = leased_pin_and_derive();
    let jid = rt
        .record_justification(
            claim_q,
            vec![cause],
            vec![cause],
            rule_definition_hash("derive_pq"),
            TransactionTime::MAX,
            ValidTime::from(2017),
        )
        .unwrap();
    rt.check_admission(&jid).expect("leased-policy admits");
    rt.emit_policy("deny", 2010, None).unwrap();
    let jid2 = rt
        .record_justification(
            claim_q,
            vec![cause],
            vec![cause],
            rule_definition_hash("derive_pq"),
            TransactionTime::MAX,
            ValidTime::from(2017),
        )
        .unwrap();
    assert_eq!(
        Some(false),
        rt.admission_status_at(&jid2, TransactionTime::MAX, ValidTime::from(2017))
    );
    assert_eq!(
        Some(true),
        rt.admission_status_at(&jid, TransactionTime::MAX, ValidTime::from(2017))
    );
    let err = rt.check_admission(&jid2).unwrap_err();
    assert!(
        matches!(
            err,
            RuntimeError::AdmissionDenied {
                reason: "policy",
                ..
            }
        ),
        "{err:?}"
    );
}

#[test]
fn empty_pin_inverse_fails_replay_fingerprint_unchanged() {
    let mut rt = Runtime::default();
    let a = rt.intern("A");
    let b = rt.intern("B");
    let knows = rt.intern("knows");
    rt.emit(Op::Assert {
        subject: a,
        relation: knows,
        object: b,
        valid_from: 0,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    let expected = rt.fold().fingerprint();
    rt.replay_check().unwrap();

    let dir = std::env::temp_dir().join(format!(
        "kutha-m016-s04-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    store::ensure_legacy_dir(&dir).unwrap();
    store::persist(&rt, &dir).unwrap();
    let _ = fs::remove_file(dir.join("events.wal"));
    let jsonl = dir.join("events.jsonl");
    let mut lines = Vec::new();
    for line in fs::read_to_string(&jsonl).unwrap().lines() {
        let mut v: serde_json::Value = serde_json::from_str(line).unwrap();
        if v["op"]["Behavior"]["name"] == "inverse_knows" {
            v["op"]["Behavior"]["rule_version"] = serde_json::Value::String(String::new());
        }
        lines.push(serde_json::to_string(&v).unwrap());
    }
    fs::write(&jsonl, lines.join("\n") + "\n").unwrap();
    let events: Vec<Event> = lines
        .iter()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();
    assert_eq!(expected, GraphFold::replay(&events).fingerprint());
    let snap: Snapshot =
        serde_json::from_str(&fs::read_to_string(dir.join("snapshot.json")).unwrap()).unwrap();
    let opened = Runtime::from_snapshot(snap, events);
    let err = opened.replay_check().unwrap_err();
    assert!(
        matches!(err, RuntimeError::ExecutionReplayFailed { .. }),
        "{err:?}"
    );
    let _ = fs::remove_dir_all(dir);
}
