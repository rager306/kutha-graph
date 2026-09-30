//! M012a S03 / ING-01: identical Assert retry under one delivery key does not mint a second support.

use kutha_common::Op;
use kutha_runtime::Runtime;

#[test]
fn identical_assert_same_delivery_key_does_not_mint_second_support() {
    let mut rt = Runtime::default();
    let s = rt.intern("S");
    let rel = rt.intern("relatedTo");
    let o = rt.intern("O");
    let key = "ingest-retry-1".to_string();

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
    let original = first.receipt.event_ids[0];
    let n = rt.log().len();
    let claim_id = rt
        .fold()
        .facts()
        .iter()
        .find(|f| f.event_id == original)
        .expect("minted support")
        .claim_id;
    assert_eq!(1, rt.fold().live_support_count(claim_id, u64::MAX, 2017));
    assert_eq!(
        1,
        rt.fold()
            .facts()
            .iter()
            .filter(|f| f.event_id == original)
            .count()
    );

    let retry = rt
        .emit(Op::Assert {
            subject: s,
            relation: rel,
            object: o,
            valid_from: 2010,
            valid_to: None,
            claim: None,
            delivery_key: Some(key),
            polarity: None,
        })
        .unwrap();
    assert_eq!(n, rt.log().len(), "retry must not append");
    assert_eq!(0, retry.events_in_quantum);
    assert_eq!(retry.receipt.event_ids[0], original);
    assert_eq!(1, rt.fold().live_support_count(claim_id, u64::MAX, 2017));
    assert_eq!(
        1,
        rt.fold()
            .facts()
            .iter()
            .filter(|f| f.event_id == original)
            .count()
    );
    rt.replay_check().unwrap();

    let unkeyed = rt
        .emit(Op::Assert {
            subject: s,
            relation: rel,
            object: o,
            valid_from: 2010,
            valid_to: None,
            claim: Some(claim_id),
            delivery_key: None,
            polarity: None,
        })
        .unwrap();
    assert_ne!(unkeyed.receipt.event_ids[0], original);
    assert_eq!(2, rt.fold().live_support_count(claim_id, u64::MAX, 2017));
    rt.replay_check().unwrap();
}
