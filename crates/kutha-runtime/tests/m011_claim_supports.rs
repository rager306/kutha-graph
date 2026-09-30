//! M011 S01: independent supports share a claim_id; retract one, the other remains.

use kutha_common::{EventId, Op};
use kutha_runtime::Runtime;

#[test]
fn retracting_one_support_leaves_claim_supported() {
    let mut rt = Runtime::default();
    let p = rt.intern("P");
    let rel = rt.intern("relatedTo");
    let true_ = rt.intern("true");

    let first = rt
        .emit(Op::Assert {
            subject: p,
            relation: rel,
            object: true_,
            valid_from: 2010,
            valid_to: None,
            claim: None,
        })
        .unwrap();
    let claim: EventId = first.receipt.event_ids[0];
    let seq_a = rt.fold().facts()[0].seq;
    assert_eq!(rt.fold().facts()[0].claim_id, claim);
    assert_eq!(1, rt.fold().live_support_count(claim, u64::MAX, 2017));

    rt.emit(Op::Assert {
        subject: p,
        relation: rel,
        object: true_,
        valid_from: 2010,
        valid_to: None,
        claim: Some(claim),
    })
    .unwrap();
    assert_eq!(2, rt.fold().live_support_count(claim, u64::MAX, 2017));
    assert!(rt.fold().claim_supported_at(claim, u64::MAX, 2017));

    rt.emit(Op::Retract { fact_seq: seq_a }).unwrap();
    assert_eq!(1, rt.fold().live_support_count(claim, u64::MAX, 2017));
    assert!(
        rt.fold().claim_supported_at(claim, u64::MAX, 2017),
        "independent support must survive withdrawal of the other"
    );
    rt.replay_check().unwrap();
}

#[test]
fn unknown_claim_does_not_append() {
    let mut rt = Runtime::default();
    let p = rt.intern("P");
    let rel = rt.intern("relatedTo");
    let true_ = rt.intern("true");
    let n = rt.log().len();
    let ghost = EventId::nil();
    let err = rt
        .emit(Op::Assert {
            subject: p,
            relation: rel,
            object: true_,
            valid_from: 2010,
            valid_to: None,
            claim: Some(ghost),
        })
        .unwrap_err();
    assert!(
        matches!(err, kutha_runtime::RuntimeError::UnknownClaim { claim } if claim == ghost),
        "{err:?}"
    );
    assert_eq!(n, rt.log().len(), "fail-closed: unknown claim must not append");
}

#[test]
fn replay_rejects_behavior_without_prior_cause() {
    use kutha_common::{Event, Op};

    let dict = vec![
        "knows".into(),
        "knownBy".into(),
        "A".into(),
        "B".into(),
    ];
    let ghost = EventId::nil();
    let events = vec![Event::new(
        Op::Behavior {
            name: "inverse_knows".into(),
            caused_by: ghost,
            rule_version: String::new(),
            subject: 2,
            relation: 1,
            object: 0,
            valid_from: 0,
            valid_to: None,
        },
        0,
    )];
    let rt = Runtime::from_dict_and_events(dict, events, 32).unwrap();
    let err = rt.replay_check().unwrap_err();
    assert!(
        matches!(err, kutha_runtime::RuntimeError::BrokenLineage { caused_by } if caused_by == ghost),
        "{err:?}"
    );
}

/// M011 S03: pinned rule P ⊢ Q; eligibility tracks premise claim supports, not a single support event.
#[test]
fn derived_q_loses_eligibility_when_last_premise_support_withdrawn() {
    let mut rt = Runtime::default();
    let p = rt.intern("P");
    let q = rt.intern("Q");
    let rel = rt.intern("relatedTo");
    let true_ = rt.intern("true");

    let first = rt
        .emit(Op::Assert {
            subject: p,
            relation: rel,
            object: true_,
            valid_from: 2010,
            valid_to: None,
            claim: None,
        })
        .unwrap();
    let claim_p: EventId = first.receipt.event_ids[0];
    let seq_a = rt.fold().facts()[0].seq;

    let second = rt
        .emit(Op::Assert {
            subject: p,
            relation: rel,
            object: true_,
            valid_from: 2010,
            valid_to: None,
            claim: Some(claim_p),
        })
        .unwrap();
    assert!(!second.receipt.event_ids.is_empty());
    let seq_b = rt
        .fold()
        .facts()
        .iter()
        .find(|f| f.claim_id == claim_p && f.seq != seq_a)
        .unwrap()
        .seq;

    let derived = rt
        .emit(Op::Behavior {
            name: "derive_pq".into(),
            caused_by: claim_p,
            rule_version: String::new(),
            subject: q,
            relation: rel,
            object: true_,
            valid_from: 2010,
            valid_to: None,
        })
        .unwrap();
    let claim_q: EventId = derived.receipt.event_ids[0];
    let tt_after_derive = rt.fold().facts().iter().find(|f| f.claim_id == claim_q).unwrap().ingested_at;

    assert!(
        rt.derivation_eligible_at(claim_q, tt_after_derive, 2017),
        "Q must be eligible while P still has live supports"
    );
    assert_eq!(2, rt.fold().live_support_count(claim_p, tt_after_derive, 2017));

    rt.emit(Op::Retract { fact_seq: seq_a }).unwrap();
    let tt_one_left = u64::MAX;
    assert!(
        rt.derivation_eligible_at(claim_q, tt_one_left, 2017),
        "withdrawing one support must not drop Q while another remains"
    );
    assert!(rt.fold().claim_supported_at(claim_p, tt_one_left, 2017));

    rt.emit(Op::Retract { fact_seq: seq_b }).unwrap();
    assert!(
        !rt.derivation_eligible_at(claim_q, u64::MAX, 2017),
        "last positive support gone: Q loses eligibility through this derivation"
    );
    assert!(
        rt.fold().claim_supported_at(claim_q, u64::MAX, 2017),
        "withdrawal of P must not erase historical Q fact"
    );
    assert!(
        rt.derivation_eligible_at(claim_q, tt_after_derive, 2017),
        "earlier TT cut must keep Q eligible"
    );
    rt.replay_check().unwrap();
}
