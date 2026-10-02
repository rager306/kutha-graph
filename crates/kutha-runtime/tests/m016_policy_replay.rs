//! RULE-03/04 oracles: deny policy body; leased-policy still admits; AS OF prior admission.

use kutha_common::{
    policy_version_hash, rule_definition_hash, EventId, Op, TransactionTime, ValidTime,
};
use kutha_runtime::{Runtime, RuntimeError};

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
fn deny_policy_fails_admission_leased_policy_admits() {
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
        .expect("leased-policy admits");
    rt.check_admission(&jid).expect("leased-policy check");
    assert_eq!(
        Some(true),
        rt.admission_status_at(&jid, TransactionTime::MAX, ValidTime::from(2017))
    );
    let first = policy_version_hash("leased-policy");
    assert_eq!(
        Some(first),
        rt.live_policy_pin_at(TransactionTime::MAX, ValidTime::from(2017))
    );

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
        .expect("deny still records status");
    assert_eq!(
        Some(false),
        rt.admission_status_at(&jid2, TransactionTime::MAX, ValidTime::from(2017))
    );
    assert_eq!(
        Some(true),
        rt.admission_status_at(&jid, TransactionTime::MAX, ValidTime::from(2017)),
        "prior admission AS OF stays true"
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
