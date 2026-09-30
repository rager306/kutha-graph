//! M011 S07 provenance digest; ADR-060 obligation 2.
//! State fold fingerprint stays Facts-only; lineage mix is a separate surface.

use kutha_common::{EventId, Op};
use kutha_runtime::Runtime;

/// Two valid priors plus a Behavior caused_by the first, name `derive_pq`, pin `r1`.
fn baseline_derive_pq_r1() -> (Runtime, EventId, EventId) {
    let mut rt = Runtime::default();
    let p = rt.intern("P");
    let q = rt.intern("Q");
    let rel = rt.intern("relatedTo");
    let true_ = rt.intern("true");

    let a1 = rt
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
    let a1_id = a1.receipt.event_ids[0];

    let a2 = rt
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
    let a2_id = a2.receipt.event_ids[0];
    assert_ne!(a1_id, a2_id);

    rt.emit(Op::Behavior {
        name: "derive_pq".into(),
        caused_by: a1_id,
        rule_version: "r1".into(),
        subject: q,
        relation: rel,
        object: true_,
        valid_from: 2010,
        valid_to: None,
    })
    .unwrap();

    (rt, a1_id, a2_id)
}

fn rebuild_cloned(rt: &Runtime, events: Vec<kutha_common::Event>) -> Runtime {
    Runtime::from_dict_and_events(
        rt.dictionary().strings().to_vec(),
        events,
        rt.max_cascade,
    )
    .unwrap()
}

#[test]
fn provenance_detects_caused_by_swap_when_state_fingerprint_matches() {
    let (rt, _a1, a2) = baseline_derive_pq_r1();
    let mut swapped = rt.log().as_slice().to_vec();
    for e in &mut swapped {
        if let Op::Behavior { caused_by, .. } = &mut e.op {
            *caused_by = a2;
        }
    }
    let rt2 = rebuild_cloned(&rt, swapped);

    assert_eq!(
        rt.fold().fingerprint(),
        rt2.fold().fingerprint(),
        "valid-prior caused_by swap must not move the state fingerprint"
    );
    assert_ne!(
        rt.provenance_fingerprint(),
        rt2.provenance_fingerprint(),
        "lineage digest must move when caused_by changes"
    );
    rt.replay_check()
        .expect("baseline replay_check must stay Ok");
    rt2.replay_check()
        .expect("swapped valid prior must still replay_check Ok");
}

#[test]
fn provenance_detects_rule_version_change_when_state_fingerprint_matches() {
    let (rt, _a1, _a2) = baseline_derive_pq_r1();
    let mut pinned = rt.log().as_slice().to_vec();
    for e in &mut pinned {
        if let Op::Behavior { rule_version, .. } = &mut e.op {
            *rule_version = "r2".into();
        }
    }
    let rt2 = rebuild_cloned(&rt, pinned);

    assert_eq!(
        rt.fold().fingerprint(),
        rt2.fold().fingerprint(),
        "rule_version-only change must not move the state fingerprint"
    );
    assert_ne!(
        rt.provenance_fingerprint(),
        rt2.provenance_fingerprint(),
        "lineage digest must move when rule_version changes"
    );
    rt.replay_check()
        .expect("baseline replay_check must stay Ok");
    rt2.replay_check()
        .expect("r2 clone must still replay_check Ok");
}

/// Not a GATE observe name (RESEARCH Q6). Legacy WAL/JSONL rows omit the field.
#[test]
fn behavior_without_rule_version_field_deserializes() {
    use kutha_common::Event;
    let json = r#"{
        "id": "00000000-0000-0000-0000-000000000001",
        "op": {
            "Behavior": {
                "name": "derive_pq",
                "caused_by": "00000000-0000-0000-0000-000000000002",
                "subject": 0,
                "relation": 1,
                "object": 2,
                "valid_from": 0,
                "valid_to": null
            }
        },
        "ingested_at": 0,
        "object_ids": [0, 1, 2]
    }"#;
    let e: Event = serde_json::from_str(json).expect("missing rule_version must decode");
    match e.op {
        Op::Behavior { rule_version, .. } => {
            assert_eq!(rule_version, "", "serde default must be empty string")
        }
        other => panic!("expected Behavior, got {other:?}"),
    }
}
