//! M012 S03: admission status and policy version as log meta-facts (ADM-01..03).
//! Not ADR-050's six dictionary kinds. Not a thin Action record.

use kutha_common::{policy_version_hash, rule_definition_hash, Op};
use kutha_runtime::{Runtime, RuntimeError};

fn leased_pin_and_derive() -> (Runtime, kutha_common::EventId, kutha_common::EventId) {
    let mut rt = Runtime::default();
    assert_eq!(0, rt.fold().facts().len());

    let p = rt.intern("P");
    let q = rt.intern("Q");
    let rel = rt.intern("relatedTo");
    let true_ = rt.intern("true");

    rt.emit(Op::RegisterRule {
        definition: "derive_pq".into(),
        valid_from: 2010,
        valid_to: None,
    })
    .unwrap();
    rt.emit(Op::PinPolicy {
        definition: "leased-policy".into(),
        valid_from: 2010,
        valid_to: None,
    })
    .unwrap();

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
    let claim_q = derived.receipt.event_ids[0];
    (rt, claim_q, cause)
}

#[test]
fn admission_status_queryable_as_of_cut() {
    let mut rt = Runtime::default();
    assert_eq!(0, rt.fold().facts().len());

    let p = rt.intern("P");
    let q = rt.intern("Q");
    let rel = rt.intern("relatedTo");
    let true_ = rt.intern("true");

    rt.emit(Op::RegisterRule {
        definition: "derive_pq".into(),
        valid_from: 2010,
        valid_to: None,
    })
    .unwrap();

    let n = rt.log().len();
    rt.emit(Op::PinPolicy {
        definition: "leased-policy".into(),
        valid_from: 2010,
        valid_to: None,
    })
    .unwrap();
    assert_eq!(n + 1, rt.log().len(), "PinPolicy appends one log event");
    let last = rt.log().as_slice().last().expect("appended");
    assert!(
        matches!(
            &last.op,
            Op::PinPolicy {
                definition,
                valid_from: 2010,
                valid_to: None
            } if definition == "leased-policy"
        ),
        "{:?}",
        last.op
    );
    assert_eq!(0, rt.fold().facts().len());

    let policy = policy_version_hash("leased-policy");
    assert_eq!(64, policy.len());
    assert!(
        policy.chars().all(|c| matches!(c, '0'..='9' | 'a'..='f')),
        "policy must be lowercase hex: {policy}"
    );
    assert!(rt.policy_hash_live_at(&policy, u64::MAX, 2010));

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
    let claim_q = derived.receipt.event_ids[0];
    let facts_before = rt.fold().facts().len();
    let log_before = rt.log().len();

    let jid = rt
        .record_justification(
            claim_q,
            vec![cause],
            vec![cause],
            rule_definition_hash("derive_pq"),
            u64::MAX,
            2017,
        )
        .expect("record_justification");
    assert_eq!(
        log_before + 2,
        rt.log().len(),
        "JustificationCite then RecordAdmission"
    );
    assert_eq!(facts_before, rt.fold().facts().len());
    assert_eq!(Some(true), rt.admission_status_at(&jid, u64::MAX, 2017));
    assert_eq!(None, rt.admission_status_at(&jid, 0, 2017));
}

#[test]
fn admission_cites_pinned_policy_version() {
    let (mut rt, claim_q, cause) = leased_pin_and_derive();
    let jid = rt
        .record_justification(
            claim_q,
            vec![cause],
            vec![cause],
            rule_definition_hash("derive_pq"),
            u64::MAX,
            2017,
        )
        .expect("record_justification");
    let policy = policy_version_hash("leased-policy");
    let cited = rt.log().as_slice().iter().rev().find_map(|e| match &e.op {
        Op::RecordAdmission {
            justification_id,
            policy_version,
            ..
        } if justification_id == &jid => Some(policy_version.clone()),
        _ => None,
    });
    assert_eq!(Some(policy.clone()), cited);
    assert!(rt.policy_hash_live_at(&policy, u64::MAX, 2017));
}

#[test]
fn record_justification_invokes_check_admission() {
    let (mut rt, claim_q, cause) = leased_pin_and_derive();
    rt.emit(Op::Retract { event_id: cause }).unwrap();
    let jid = rt
        .record_justification(
            claim_q,
            vec![cause],
            vec![cause],
            rule_definition_hash("derive_pq"),
            u64::MAX,
            2017,
        )
        .expect("status fact is recorded even when denied");
    assert_eq!(Some(false), rt.admission_status_at(&jid, u64::MAX, 2017));
}

#[test]
fn empty_or_missing_policy_pin_fails_closed() {
    let mut rt = Runtime::default();
    let n = rt.log().len();
    let err = rt
        .emit(Op::PinPolicy {
            definition: String::new(),
            valid_from: 2010,
            valid_to: None,
        })
        .unwrap_err();
    assert!(
        matches!(err, RuntimeError::UnknownPolicyVersion { ref pin } if pin.is_empty()),
        "{err:?}"
    );
    assert_eq!(n, rt.log().len());

    let p = rt.intern("P");
    let q = rt.intern("Q");
    let rel = rt.intern("relatedTo");
    let true_ = rt.intern("true");
    rt.emit(Op::RegisterRule {
        definition: "derive_pq".into(),
        valid_from: 2010,
        valid_to: None,
    })
    .unwrap();
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
    let claim_q = derived.receipt.event_ids[0];
    let n = rt.log().len();
    let err = rt
        .record_justification(
            claim_q,
            vec![cause],
            vec![cause],
            rule_definition_hash("derive_pq"),
            u64::MAX,
            2017,
        )
        .unwrap_err();
    assert!(
        matches!(err, RuntimeError::UnknownPolicyVersion { .. }),
        "{err:?}"
    );
    assert_eq!(n, rt.log().len());

    let n = rt.log().len();
    let err = rt
        .emit(Op::RecordAdmission {
            justification_id: "j".into(),
            admitted: true,
            policy_version: "x".into(),
            valid_from: 2010,
            valid_to: None,
        })
        .unwrap_err();
    assert!(matches!(err, RuntimeError::MetaOpRejected), "{err:?}");
    assert_eq!(n, rt.log().len());
}
