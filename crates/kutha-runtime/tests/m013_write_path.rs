//! M013 S06 / Phase 27: unknown-relation admit is shared by Assert and
//! justification/action constructors (pipeline receipt). Never edit named
//! m012_*/m012a_*/m011_*/ff5/ff6 expects. Never edit `.kutha/STATE.md`.

use kutha_common::{Op, KUTHA_ALLOWS};
use kutha_runtime::{Runtime, RuntimeError};

#[test]
fn unknown_relation_rejected_for_assert_justification_and_action() {
    let mut rt = Runtime::default();
    let ghost = rt.intern("notALegalRelation");

    let err = rt
        .assert("art-12-speed", "notALegalRelation", "50-kmh", 2010, None)
        .unwrap_err();
    assert!(
        matches!(err, RuntimeError::UnknownRelation { ref name } if name == "notALegalRelation"),
        "{err:?}"
    );

    rt.emit_policy("leased-policy", 2010, None).unwrap();
    let outcomes_before = rt.outcome_records().len();

    let dummy = kutha_common::EventId::nil();
    let _ = rt.record_justification(dummy, Vec::new(), Vec::new(), "", u64::MAX, 2010);
    let _ = rt.record_action("missing", u64::MAX, 2010);

    assert!(
        rt.outcome_records().len() > outcomes_before,
        "justification/action must share Assert admit/receipt (outcome_records grow)"
    );
    assert!(
        !rt.fold().facts().iter().any(|f| f.relation == ghost),
        "no new fact may carry interned notALegalRelation"
    );
}

#[test]
fn public_emit_assert_kutha_allows_is_refused() {
    let mut rt = Runtime::default();
    let rel = rt.intern(KUTHA_ALLOWS);
    let s = rt.intern("subject");
    let o = rt.intern("object");
    let n = rt.log().len();

    let err = rt
        .emit(Op::Assert {
            subject: s,
            relation: rel,
            object: o,
            valid_from: 2010,
            valid_to: None,
            claim: None,
            delivery_key: Some("forged-allow".into()),
            polarity: None,
        })
        .unwrap_err();
    assert!(matches!(err, RuntimeError::MetaOpRejected), "{err:?}");
    assert_eq!(n, rt.log().len(), "forged kutha:allows must not append");

    rt.emit_allow("leasedRel", 2010, None).unwrap();
}

#[test]
fn keyed_correct_preserves_delivery_key_and_retry() {
    let mut rt = Runtime::default();
    let s = rt.intern("S");
    let rel = rt.intern("relatedTo");
    let o = rt.intern("O");
    let o2 = rt.intern("O2");
    let key = "p30-h02-correct-key".to_string();

    let first = rt
        .emit(Op::Assert {
            subject: s,
            relation: rel,
            object: o,
            valid_from: 2010,
            valid_to: None,
            claim: None,
            delivery_key: Some(key.clone()),
            polarity: None,
        })
        .unwrap();
    let minting = first.receipt.event_ids[0];
    let claim_id = rt
        .fold()
        .facts()
        .iter()
        .find(|f| f.event_id == minting)
        .expect("minted support")
        .claim_id;

    rt.emit(Op::Correct {
        event_id: minting,
        object: o2,
        valid_from: 2010,
        valid_to: None,
    })
    .unwrap();

    let live_keyed = rt
        .fold()
        .facts()
        .iter()
        .filter(|f| f.is_live_at(u64::MAX, 2017) && f.delivery_key.as_deref() == Some(key.as_str()))
        .count();
    assert_eq!(1, live_keyed, "correct replacement must keep delivery_key");
    assert_eq!(1, rt.fold().live_support_count(claim_id, u64::MAX, 2017));

    let n = rt.log().len();
    let retry = rt
        .emit(Op::Assert {
            subject: s,
            relation: rel,
            object: o2,
            valid_from: 2010,
            valid_to: None,
            claim: None,
            delivery_key: Some(key),
            polarity: None,
        })
        .unwrap();
    assert_eq!(n, rt.log().len(), "keyed retry must not append");
    assert_eq!(0, retry.events_in_quantum);
    assert_eq!(1, rt.fold().live_support_count(claim_id, u64::MAX, 2017));
}

#[test]
fn keyed_correct_interval_preserves_delivery_key_and_retry() {
    let mut rt = Runtime::default();
    let s = rt.intern("a");
    let rel = rt.intern("relatedTo");
    let p = rt.intern("P");
    let p_prime = rt.intern("P-prime");
    let key = "p30-h02i-interval-key".to_string();

    let first = rt
        .emit(Op::Assert {
            subject: s,
            relation: rel,
            object: p,
            valid_from: 2010,
            valid_to: None,
            claim: None,
            delivery_key: Some(key.clone()),
            polarity: None,
        })
        .unwrap();
    let minting = first.receipt.event_ids[0];
    let claim_id = rt
        .fold()
        .facts()
        .iter()
        .find(|f| f.event_id == minting)
        .expect("minted support")
        .claim_id;

    rt.emit(Op::CorrectInterval {
        event_id: minting,
        object: p_prime,
        patch_from: 2015,
        patch_to: Some(2020),
    })
    .unwrap();

    let live_keyed = rt
        .fold()
        .facts()
        .iter()
        .filter(|f| f.is_live_at(u64::MAX, 2017) && f.delivery_key.as_deref() == Some(key.as_str()))
        .count();
    assert_eq!(1, live_keyed, "interval replacement rows must keep delivery_key");
    assert_eq!(1, rt.fold().live_support_count(claim_id, u64::MAX, 2017));

    let n = rt.log().len();
    let retry = rt
        .emit(Op::Assert {
            subject: s,
            relation: rel,
            object: p_prime,
            valid_from: 2010,
            valid_to: None,
            claim: None,
            delivery_key: Some(key),
            polarity: None,
        })
        .unwrap();
    assert_eq!(n, rt.log().len(), "keyed retry must not append");
    assert_eq!(0, retry.events_in_quantum);
    assert_eq!(1, rt.fold().live_support_count(claim_id, u64::MAX, 2017));
}
