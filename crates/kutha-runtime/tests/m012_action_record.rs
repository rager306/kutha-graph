//! M012 S04: thin Action record binds resolved args to admission and policy (ACT-01..02).
//! Not ADR-050's six dictionary kinds. Not n-ary derivation.

use kutha_common::{policy_version_hash, rule_definition_hash, EventId, Op};
use kutha_runtime::{store, Runtime, RuntimeError};

fn leased_pin_and_derive() -> (Runtime, EventId, EventId) {
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
fn action_binds_args_to_admission_and_policy() {
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
    let log_before = rt.log().len();
    let facts_before = rt.fold().facts().len();

    let action_id = rt.record_action(&jid, u64::MAX, 2017);
    assert!(
        action_id.is_ok(),
        "record_action must bind after justification: {action_id:?}"
    );
    let action_id = action_id.unwrap();
    assert_eq!(log_before + 1, rt.log().len());
    let last = rt.log().as_slice().last().expect("RecordAction");
    assert_eq!(action_id, last.id);
    assert!(
        matches!(
            &last.op,
            Op::RecordAction {
                justification_id,
                target_claim,
                admitted: true,
                ..
            } if justification_id == &jid && *target_claim == claim_q
        ),
        "{:?}",
        last.op
    );
    assert_eq!(facts_before, rt.fold().facts().len());

    let rec = rt.action_record_at(&jid, u64::MAX, 2017);
    assert!(rec.is_some(), "action_record_at at live cut");
    let rec = rec.unwrap();
    assert_eq!(jid, rec.justification_id);
    assert_eq!(claim_q, rec.target_claim);
    assert!(rec.source_event_ids.contains(&cause));
    assert!(rec.source_claim_ids.contains(&cause));
    assert_eq!(rule_definition_hash("derive_pq"), rec.rule_version);
    assert!(rec.admitted);
    assert_eq!(policy_version_hash("leased-policy"), rec.policy_version);
    assert_eq!(Some(true), rt.admission_status_at(&jid, u64::MAX, 2017));
    assert_eq!(None, rt.action_record_at(&jid, 0, 2017));
}

#[test]
fn action_cites_admission_pin_and_rejects_public_emit() {
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
    rt.record_action(&jid, u64::MAX, 2017)
        .expect("record_action");

    let policy = policy_version_hash("leased-policy");
    let admission_pin = rt.log().as_slice().iter().rev().find_map(|e| match &e.op {
        Op::RecordAdmission {
            justification_id,
            policy_version,
            ..
        } if justification_id == &jid => Some(policy_version.clone()),
        _ => None,
    });
    let action_pin = rt.log().as_slice().iter().rev().find_map(|e| match &e.op {
        Op::RecordAction {
            justification_id,
            policy_version,
            ..
        } if justification_id == &jid => Some(policy_version.clone()),
        _ => None,
    });
    assert_eq!(Some(policy.clone()), admission_pin);
    assert_eq!(Some(policy), action_pin);

    let n = rt.log().len();
    let err = rt
        .emit(Op::RecordAction {
            justification_id: jid.clone(),
            target_claim: claim_q,
            source_claim_ids: vec![cause],
            source_event_ids: vec![cause],
            rule_version: rule_definition_hash("derive_pq"),
            admitted: true,
            policy_version: "caller-supplied".into(),
            valid_from: 2017,
            valid_to: None,
        })
        .unwrap_err();
    assert!(matches!(err, RuntimeError::MetaOpRejected), "{err:?}");
    assert_eq!(n, rt.log().len());

    let err = rt
        .record_action("j:never-recorded", u64::MAX, 2017)
        .unwrap_err();
    assert!(
        matches!(err, RuntimeError::UnknownAdmission { ref justification_id } if justification_id == "j:never-recorded"),
        "{err:?}"
    );
    assert_eq!(n, rt.log().len());

    let admission_id = rt
        .log()
        .as_slice()
        .iter()
        .rev()
        .find(|e| {
            matches!(
                &e.op,
                Op::RecordAdmission {
                    justification_id,
                    ..
                } if justification_id == &jid
            )
        })
        .expect("RecordAdmission")
        .id;
    rt.emit(Op::Retract {
        event_id: admission_id,
    })
    .unwrap();
    let n = rt.log().len();
    let err = rt.record_action(&jid, u64::MAX, 2017).unwrap_err();
    assert!(
        matches!(err, RuntimeError::UnknownAdmission { ref justification_id } if justification_id == &jid),
        "{err:?}"
    );
    assert_eq!(n, rt.log().len());
}

#[test]
fn action_and_admission_as_of_prior_cut_after_policy_change() {
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
    let action_id = rt
        .record_action(&jid, u64::MAX, 2017)
        .expect("record_action");
    let prior_tt = rt
        .log()
        .iter()
        .find(|e| e.id == action_id)
        .expect("RecordAction")
        .ingested_at;
    let first = policy_version_hash("leased-policy");
    let rec = rt
        .action_record_at(&jid, prior_tt, 2017)
        .expect("action at prior cut");
    assert_eq!(first, rec.policy_version);
    assert_eq!(Some(true), rt.admission_status_at(&jid, prior_tt, 2017));

    rt.emit(Op::PinPolicy {
        definition: "later-policy".into(),
        valid_from: 2010,
        valid_to: None,
    })
    .unwrap();
    assert_eq!(
        Some(policy_version_hash("later-policy")),
        rt.live_policy_pin_at(u64::MAX, 2017)
    );
    let rec = rt
        .action_record_at(&jid, prior_tt, 2017)
        .expect("action still at prior cut");
    assert_eq!(first, rec.policy_version);
    assert!(rec.admitted);
    assert_eq!(Some(true), rt.admission_status_at(&jid, prior_tt, 2017));
    let tip = rt
        .action_record_at(&jid, u64::MAX, 2017)
        .expect("action at tip");
    assert_eq!(first, tip.policy_version);
}

#[test]
fn persist_open_reconstructs_action_record() {
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
    rt.record_action(&jid, u64::MAX, 2017)
        .expect("record_action");
    let first = policy_version_hash("leased-policy");

    let dir = std::env::temp_dir().join(format!(
        "kutha-m012-s04-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    store::persist(&rt, &dir).unwrap();
    let mut opened = store::open(&dir).unwrap();
    let rec = opened
        .action_record_at(&jid, u64::MAX, 2017)
        .expect("opened action_record_at");
    assert_eq!(first, rec.policy_version);
    assert!(rec.admitted);
    assert_eq!(claim_q, rec.target_claim);
    assert_eq!(Some(true), opened.admission_status_at(&jid, u64::MAX, 2017));
    assert!(
        opened.log().iter().any(
            |e| matches!(&e.op, Op::RecordAction { justification_id, .. } if justification_id == &jid)
        ),
        "opened log must contain RecordAction"
    );
    let n = opened.log().len();
    let err = opened
        .emit(Op::RecordAction {
            justification_id: jid.clone(),
            target_claim: claim_q,
            source_claim_ids: vec![cause],
            source_event_ids: vec![cause],
            rule_version: rule_definition_hash("derive_pq"),
            admitted: true,
            policy_version: "caller-supplied".into(),
            valid_from: 2017,
            valid_to: None,
        })
        .unwrap_err();
    assert!(matches!(err, RuntimeError::MetaOpRejected), "{err:?}");
    assert_eq!(n, opened.log().len());
    let _ = std::fs::remove_dir_all(&dir);
}
