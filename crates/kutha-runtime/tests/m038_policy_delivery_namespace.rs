//! M038 S01 / Phase 98 — delivery-key retry namespaced by relation
//! (CE review P2 #29). Admission Asserts share the policy pin string as
//! delivery_key; a global last-wins map then makes identical emit_policy
//! re-pin resolve to the admission row and spuriously DeliveryKeyConflict.

use kutha_common::{Op, TransactionTime, ValidTime, policy_version_hash, rule_definition_hash};
use kutha_runtime::{Runtime, RuntimeError};

fn leased_derive() -> (Runtime, kutha_common::EventId, kutha_common::EventId) {
    let mut rt = Runtime::default();
    let p = rt.intern("P-m038");
    let q = rt.intern("Q-m038");
    let rel = rt.intern("relatedTo");
    let true_ = rt.intern("true");
    rt.emit_rule("derive_pq", 2010, None).unwrap();
    rt.emit_policy("leased-policy", 2010, None).unwrap();
    let cause = rt
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
        .unwrap()
        .receipt
        .event_ids[0];
    let claim_q = rt
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
        .unwrap()
        .receipt
        .event_ids[0];
    (rt, claim_q, cause)
}

/// Kill: identical emit_policy after justification must not DeliveryKeyConflict.
#[test]
fn emit_policy_repin_after_justification_is_idempotent() {
    let (mut rt, claim_q, cause) = leased_derive();
    let pin = policy_version_hash("leased-policy");
    assert_eq!(
        Some(pin.clone()),
        rt.live_policy_pin_at(TransactionTime::MAX, ValidTime::from(2017))
    );

    let jid = rt
        .record_justification(
            claim_q,
            vec![cause],
            vec![cause],
            rule_definition_hash("derive_pq"),
            TransactionTime::MAX,
            ValidTime::from(2017),
        )
        .expect("record_justification");
    assert!(
        rt.admission_status_at(&jid, TransactionTime::MAX, ValidTime::from(2017))
            .is_some(),
        "admission recorded under live policy pin"
    );

    match rt.emit_policy("leased-policy", 2010, None) {
        Ok(out) => {
            assert_eq!(
                0, out.events_in_quantum,
                "identical policy Assert re-pin must short-circuit (ING-01)"
            );
        }
        Err(RuntimeError::DeliveryKeyConflict { key }) => {
            panic!(
                "policy re-pin must not conflict with admission sharing pin {pin}; got conflict key={key}"
            );
        }
        Err(e) => panic!("unexpected error on policy re-pin: {e}"),
    }
    assert_eq!(
        Some(pin),
        rt.live_policy_pin_at(TransactionTime::MAX, ValidTime::from(2017)),
        "policy pin still live after re-pin"
    );
}

/// Regression: bare identical emit_policy Assert still short-circuits (StoreBlob may append).
#[test]
fn emit_policy_repin_without_justification_still_idempotent() {
    let mut rt = Runtime::default();
    rt.emit_policy("solo-policy", 2010, None).unwrap();
    let out = rt.emit_policy("solo-policy", 2010, None).unwrap();
    assert_eq!(
        0, out.events_in_quantum,
        "identical policy Assert must short-circuit without justification"
    );
    assert_eq!(
        Some(policy_version_hash("solo-policy")),
        rt.live_policy_pin_at(TransactionTime::MAX, ValidTime::from(2017))
    );
}

/// Same delivery_key string on different relations must not collide.
#[test]
fn same_delivery_key_different_relation_do_not_conflict() {
    let mut rt = Runtime::default();
    let key = "shared-key-m038".to_string();
    let a = rt.intern("A-m038");
    let b = rt.intern("B-m038");
    let r1 = rt.intern("relatedTo");
    let r2 = rt.intern("knows");
    rt.emit(Op::Assert {
        subject: a,
        relation: r1,
        object: b,
        valid_from: 2010,
        valid_to: None,
        claim: None,
        delivery_key: Some(key.clone()),
        polarity: None,
    })
    .unwrap();
    // Different relation, same key — must append (not conflict / not steal retry).
    let out = rt
        .emit(Op::Assert {
            subject: a,
            relation: r2,
            object: b,
            valid_from: 2010,
            valid_to: None,
            claim: None,
            delivery_key: Some(key),
            polarity: None,
        })
        .unwrap();
    assert!(
        out.events_in_quantum > 0,
        "same key on different relation must mint a new Assert"
    );
}
