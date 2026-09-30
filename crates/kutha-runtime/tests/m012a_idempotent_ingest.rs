//! M012a S03 / ING-01: identical Assert retry under one delivery key does not mint a second support.

use kutha_common::{Op, SupportPolarity};
use kutha_runtime::{store, Runtime, RuntimeError};

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

#[test]
fn delivery_key_payload_mismatch_does_not_append() {
    let mut rt = Runtime::default();
    let s = rt.intern("S");
    let rel = rt.intern("relatedTo");
    let o = rt.intern("O");
    let other = rt.intern("O2");
    let key = "ingest-retry-mismatch".to_string();

    rt.emit(Op::Assert {
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
    let n = rt.log().len();

    let err = rt
        .emit(Op::Assert {
            subject: s,
            relation: rel,
            object: other,
            valid_from: 2010,
            valid_to: None,
            claim: None,
            delivery_key: Some(key.clone()),
            polarity: None,
        })
        .unwrap_err();
    assert!(
        matches!(err, RuntimeError::DeliveryKeyConflict { key: ref k } if k == &key),
        "{err:?}"
    );
    assert_eq!(n, rt.log().len(), "mismatch must not append");
}

#[test]
fn keyed_assert_retry_after_persist_open_does_not_mint() {
    let mut rt = Runtime::default();
    let s = rt.intern("S");
    let rel = rt.intern("relatedTo");
    let o = rt.intern("O");
    let key = "ingest-retry-persist".to_string();

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
    let claim_id = rt
        .fold()
        .facts()
        .iter()
        .find(|f| f.event_id == original)
        .expect("minted support")
        .claim_id;

    let dir = std::env::temp_dir().join(format!(
        "kutha-m012a-s03-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    store::persist(&rt, &dir).unwrap();
    let mut opened = store::open(&dir).unwrap();
    let s = opened.intern("S");
    let rel = opened.intern("relatedTo");
    let o = opened.intern("O");
    let n_open = opened.log().len();
    let retry = opened
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
    assert_eq!(n_open, opened.log().len(), "open retry must not append");
    assert_eq!(retry.receipt.event_ids[0], original);
    assert_eq!(
        1,
        opened.fold().live_support_count(claim_id, u64::MAX, 2017)
    );
    opened.replay_check().unwrap();
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn claim_id_distinct_from_support_slot() {
    let mut rt = Runtime::default();
    let s = rt.intern("S");
    let rel = rt.intern("relatedTo");
    let o = rt.intern("O");

    let first = rt
        .emit(Op::Assert {
            subject: s,
            relation: rel,
            object: o,
            valid_from: 2010,
            valid_to: None,
            claim: None,
            delivery_key: Some("key-a".into()),
            polarity: None,
        })
        .unwrap();
    let id_a = first.receipt.event_ids[0];
    let second = rt
        .emit(Op::Assert {
            subject: s,
            relation: rel,
            object: o,
            valid_from: 2010,
            valid_to: None,
            claim: Some(id_a),
            delivery_key: Some("key-b".into()),
            polarity: None,
        })
        .unwrap();
    let id_b = second.receipt.event_ids[0];
    assert_ne!(id_a, id_b);

    let fa = rt
        .fold()
        .facts()
        .iter()
        .find(|f| f.event_id == id_a)
        .expect("first support");
    let fb = rt
        .fold()
        .facts()
        .iter()
        .find(|f| f.event_id == id_b)
        .expect("second support");
    assert_eq!(fa.claim_id, fb.claim_id);
    assert_eq!(
        fa.claim_id, id_a,
        "opening support claim_id equals its EventId"
    );
    assert_ne!(
        fb.claim_id, id_b,
        "later support claim_id is not its minting EventId"
    );
    let claim_s = fa.claim_id.to_string();
    assert_ne!(fa.delivery_key.as_deref(), Some(claim_s.as_str()));
    assert_ne!(fb.delivery_key.as_deref(), Some(claim_s.as_str()));
    assert_eq!(2, rt.fold().live_support_count(fa.claim_id, u64::MAX, 2017));
    rt.replay_check().unwrap();
}

#[test]
fn conflict_report_at_partitions_by_stored_polarity() {
    let mut rt = Runtime::default();
    let s = rt.intern("S");
    let rel = rt.intern("relatedTo");
    let o = rt.intern("O");

    let first = rt
        .emit(Op::Assert {
            subject: s,
            relation: rel,
            object: o,
            valid_from: 2010,
            valid_to: None,
            claim: None,
            delivery_key: Some("pol-pos".into()),
            polarity: Some(SupportPolarity::Positive),
        })
        .unwrap();
    let claim = first.receipt.event_ids[0];
    rt.emit(Op::Assert {
        subject: s,
        relation: rel,
        object: o,
        valid_from: 2010,
        valid_to: None,
        claim: Some(claim),
        delivery_key: Some("pol-neg".into()),
        polarity: Some(SupportPolarity::Negative),
    })
    .unwrap();
    let report = rt.conflict_report_at(claim, u64::MAX, 2017);
    assert!(!report.positive_supports.is_empty());
    assert!(!report.negative_supports.is_empty());
    rt.replay_check().unwrap();
}
