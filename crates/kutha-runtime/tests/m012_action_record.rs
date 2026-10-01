//! M012 S04: thin Action record binds resolved args to admission and policy (ACT-01..02).
//! Not ADR-050's six dictionary kinds. Not n-ary derivation.

use kutha_common::{policy_version_hash, rule_definition_hash, EventId, Op};
use kutha_runtime::Runtime;

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
