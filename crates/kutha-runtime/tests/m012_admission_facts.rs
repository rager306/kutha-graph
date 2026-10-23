//! M012 S03: admission status and policy version as log meta-facts (ADM-01..03).
//! Not ADR-050's six dictionary kinds. Not a thin Action record.

use kutha_common::{
    EventId, KUTHA_ADMISSION, KUTHA_POLICY, Op, TransactionTime, ValidTime, policy_version_hash,
    rule_definition_hash,
};
use kutha_runtime::{Runtime, RuntimeError, store};

fn leased_pin_and_derive() -> (Runtime, EventId, EventId, EventId) {
    let mut rt = Runtime::default();
    assert_eq!(
        0,
        rt.fold()
            .live_count(TransactionTime::MAX, ValidTime::from(0))
    );

    let p = rt.intern("P");
    let q = rt.intern("Q");
    let rel = rt.intern("relatedTo");
    let true_ = rt.intern("true");

    rt.emit_rule("derive_pq", 2010, None).unwrap();
    let pinned = rt.emit_policy("leased-policy", 2010, None).unwrap();
    let pin_event = pinned.receipt.event_ids[0];

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
    (rt, claim_q, cause, pin_event)
}

#[test]
fn admission_status_queryable_as_of_cut() {
    let mut rt = Runtime::default();
    assert_eq!(
        0,
        rt.fold()
            .live_count(TransactionTime::MAX, ValidTime::from(0))
    );

    let p = rt.intern("P");
    let q = rt.intern("Q");
    let rel = rt.intern("relatedTo");
    let true_ = rt.intern("true");

    rt.emit_rule("derive_pq", 2010, None).unwrap();

    let n = rt.log().len();
    rt.emit_policy("leased-policy", 2010, None).unwrap();
    assert_eq!(n + 2, rt.log().len(), "StoreBlob then kutha:policy Assert");
    let last = rt.log().as_slice().last().expect("appended");
    let policy = policy_version_hash("leased-policy");
    assert!(
        matches!(
            &last.op,
            Op::Assert {
                delivery_key: Some(k),
                valid_from: 2010,
                valid_to: None,
                ..
            } if k == &policy
        ),
        "{:?}",
        last.op
    );
    assert_eq!(
        0,
        rt.fold()
            .live_count(TransactionTime::MAX, ValidTime::from(2010))
    );
    assert_eq!(64, policy.len());
    assert!(
        policy.chars().all(|c| matches!(c, '0'..='9' | 'a'..='f')),
        "policy must be lowercase hex: {policy}"
    );
    assert!(rt.policy_hash_live_at(&policy, TransactionTime::MAX, ValidTime::from(2010)));

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
    let facts_before = rt
        .fold()
        .live_count(TransactionTime::MAX, ValidTime::from(2017));
    let log_before = rt.log().len();

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
    assert_eq!(
        log_before + 2,
        rt.log().len(),
        "kutha:admission Assert then JustificationCite"
    );
    assert_eq!(
        facts_before,
        rt.fold()
            .live_count(TransactionTime::MAX, ValidTime::from(2017))
    );
    assert_eq!(
        Some(true),
        rt.admission_status_at(&jid, TransactionTime::MAX, ValidTime::from(2017))
    );
    assert_eq!(
        None,
        rt.admission_status_at(&jid, TransactionTime::from(0), ValidTime::from(2017))
    );
}

#[test]
fn admission_cites_pinned_policy_version() {
    let (mut rt, claim_q, cause, _) = leased_pin_and_derive();
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
    let policy = policy_version_hash("leased-policy");
    let cited = rt.log().as_slice().iter().rev().find_map(|e| match &e.op {
        Op::Assert {
            relation,
            delivery_key: Some(policy_version),
            ..
        } if e.id.to_string() == jid
            && rt.dictionary().lookup(*relation) == Some(KUTHA_ADMISSION) =>
        {
            Some(policy_version.clone())
        }
        _ => None,
    });
    assert_eq!(Some(policy.clone()), cited);
    assert!(rt.policy_hash_live_at(&policy, TransactionTime::MAX, ValidTime::from(2017)));
}

#[test]
fn record_justification_invokes_check_admission() {
    let (mut rt, claim_q, cause, _) = leased_pin_and_derive();
    rt.emit(Op::Retract { event_id: cause }).unwrap();
    let jid = rt
        .record_justification(
            claim_q,
            vec![cause],
            vec![cause],
            rule_definition_hash("derive_pq"),
            TransactionTime::MAX,
            ValidTime::from(2017),
        )
        .expect("status fact is recorded even when denied");
    assert_eq!(
        Some(false),
        rt.admission_status_at(&jid, TransactionTime::MAX, ValidTime::from(2017))
    );
}

#[test]
fn empty_or_missing_policy_pin_fails_closed() {
    let mut rt = Runtime::default();
    let n = rt.log().len();
    let err = rt.emit_policy(String::new(), 2010, None).unwrap_err();
    assert!(
        matches!(err, RuntimeError::UnknownPolicyVersion { ref pin } if pin.is_empty()),
        "{err:?}"
    );
    assert_eq!(n, rt.log().len());

    let p = rt.intern("P");
    let q = rt.intern("Q");
    let rel = rt.intern("relatedTo");
    let true_ = rt.intern("true");
    rt.emit_rule("derive_pq", 2010, None).unwrap();
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
            TransactionTime::MAX,
            ValidTime::from(2017),
        )
        .unwrap_err();
    assert!(
        matches!(err, RuntimeError::UnknownPolicyVersion { .. }),
        "{err:?}"
    );
    assert_eq!(n, rt.log().len());

    let admission = rt.intern("kutha:admission");
    let subj = rt.intern("public-admission");
    let n = rt.log().len();
    let err = rt
        .emit(Op::Assert {
            subject: subj,
            relation: admission,
            object: subj,
            valid_from: 2010,
            valid_to: None,
            claim: None,
            delivery_key: Some("x".into()),
            polarity: None,
        })
        .unwrap_err();
    assert!(matches!(err, RuntimeError::MetaOpRejected), "{err:?}");
    assert_eq!(n, rt.log().len());
}

#[test]
fn retract_admission_and_policy_version_at_later_cut() {
    let (mut rt, claim_q, cause, pin_event) = leased_pin_and_derive();
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
    let admission_id: EventId = jid.parse().expect("admission EventId");
    assert_eq!(
        Some(true),
        rt.admission_status_at(&jid, TransactionTime::MAX, ValidTime::from(2017))
    );
    let n = rt.log().len();

    rt.emit_retract_dict(admission_id).unwrap();
    assert_eq!(
        None,
        rt.admission_status_at(&jid, TransactionTime::MAX, ValidTime::from(2017))
    );
    let fork = rt.fork_at(n).unwrap();
    assert_eq!(
        Some(true),
        fork.admission_status_at(&jid, TransactionTime::MAX, ValidTime::from(2017))
    );

    rt.emit_retract_dict(pin_event).unwrap();
    let after = rt.log().len();
    let err = rt
        .record_justification(
            claim_q,
            vec![cause],
            vec![cause],
            rule_definition_hash("derive_pq"),
            TransactionTime::MAX,
            ValidTime::from(2017),
        )
        .unwrap_err();
    assert!(
        matches!(err, RuntimeError::UnknownPolicyVersion { .. }),
        "{err:?}"
    );
    assert_eq!(after, rt.log().len(), "fail-closed: log must not grow");
}

#[test]
fn persist_open_reconstructs_admission_and_policy() {
    let (mut rt, claim_q, cause, _) = leased_pin_and_derive();
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

    let dir = std::env::temp_dir().join(format!(
        "kutha-m012-s03-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    store::persist(&rt, &dir).unwrap();
    let mut opened = store::open(&dir).unwrap();
    assert_eq!(
        Some(true),
        opened.admission_status_at(&jid, TransactionTime::MAX, ValidTime::from(2017))
    );
    opened.check_admission(&jid).unwrap();
    assert!(
        opened.log().iter().any(|e| matches!(
            &e.op,
            Op::Assert {
                delivery_key: Some(k),
                relation,
                ..
            } if k == &policy_version_hash("leased-policy")
                && opened.dictionary().lookup(*relation) == Some(KUTHA_POLICY)
        )),
        "opened log must contain PinPolicy"
    );
    assert!(
        opened.log().iter().any(|e| {
            e.id.to_string() == jid
                && matches!(
                    &e.op,
                    Op::Assert { relation, .. }
                        if opened.dictionary().lookup(*relation) == Some(KUTHA_ADMISSION)
                )
        }),
        "opened log must contain RecordAdmission"
    );
    let n = opened.log().len();
    let err = opened.emit_policy(String::new(), 2010, None).unwrap_err();
    assert!(
        matches!(err, RuntimeError::UnknownPolicyVersion { ref pin } if pin.is_empty()),
        "{err:?}"
    );
    assert_eq!(n, opened.log().len());
    let _ = std::fs::remove_dir_all(&dir);
}
