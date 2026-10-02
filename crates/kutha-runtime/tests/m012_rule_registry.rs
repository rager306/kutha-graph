//! M012 S02: rule registry as log facts (RULE-01..03).
//! Not ADR-050's six dictionary kinds.

use kutha_common::{rule_definition_hash, Op, TransactionTime, ValidTime};
use kutha_runtime::{store, Runtime, RuntimeError};

#[test]
fn register_rule_pins_behavior_to_definition_hash() {
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
    let n = rt.log().len();

    rt.emit_rule("derive_pq", 2010, None).unwrap();

    assert_eq!(n + 2, rt.log().len(), "StoreBlob then kutha:rule Assert");
    let last = rt.log().as_slice().last().expect("appended");
    let pin = rule_definition_hash("derive_pq");
    assert!(
        matches!(
            &last.op,
            Op::Assert {
                delivery_key: Some(k),
                valid_from: 2010,
                valid_to: None,
                ..
            } if k == &pin
        ),
        "{:?}",
        last.op
    );
    assert_eq!(
        0,
        rt.fold()
            .live_count(TransactionTime::MAX, ValidTime::from(2010))
    );
    assert_eq!(0, rt.fold().as_of(ValidTime::from(2010)).len());
    assert_eq!(64, pin.len());
    assert!(
        pin.chars().all(|c| matches!(c, '0'..='9' | 'a'..='f')),
        "pin must be lowercase hex: {pin}"
    );
    assert!(rt.fold().rule_hash_live_at(&pin, u64::MAX, 2010));

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
            rule_version: pin,
            subject: q,
            relation: rel,
            object: true_,
            valid_from: 2010,
            valid_to: None,
        })
        .unwrap();
    let claim_q = derived.receipt.event_ids[0];
    assert!(
        rt.fold()
            .facts()
            .iter()
            .any(|f| f.claim_id == claim_q && f.is_live_at(u64::MAX, 2017)),
        "facts() must hold a live Q claim"
    );
    assert!(
        rt.derivation_eligible_at(claim_q, TransactionTime::MAX, ValidTime::from(2017)),
        "hashed derive_pq must be eligible at the named cut"
    );
}

#[test]
fn unknown_or_mismatched_rule_version_fails_closed() {
    let mut rt = Runtime::default();
    let p = rt.intern("P");
    let q = rt.intern("Q");
    let rel = rt.intern("relatedTo");
    let true_ = rt.intern("true");
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
    let other = rule_definition_hash("other-def");

    let n = rt.log().len();
    let err = rt
        .emit(Op::Behavior {
            name: "derive_pq".into(),
            caused_by: cause,
            rule_version: other.clone(),
            subject: q,
            relation: rel,
            object: true_,
            valid_from: 2010,
            valid_to: None,
        })
        .unwrap_err();
    assert!(
        matches!(err, RuntimeError::UnknownRuleVersion { ref pin } if pin == &other),
        "{err:?}"
    );
    assert_eq!(n, rt.log().len(), "fail-closed: log must not grow");

    rt.emit_rule("derive_pq", 2010, None).unwrap();
    let n = rt.log().len();
    let err = rt
        .emit(Op::Behavior {
            name: "derive_pq".into(),
            caused_by: cause,
            rule_version: other.clone(),
            subject: q,
            relation: rel,
            object: true_,
            valid_from: 2010,
            valid_to: None,
        })
        .unwrap_err();
    assert!(
        matches!(err, RuntimeError::UnknownRuleVersion { ref pin } if pin == &other),
        "{err:?}"
    );
    assert_eq!(
        n,
        rt.log().len(),
        "fail-closed: mismatched pin must not append"
    );
}

#[test]
fn free_string_rule_version_does_not_append() {
    let mut rt = Runtime::default();
    let p = rt.intern("P");
    let q = rt.intern("Q");
    let rel = rt.intern("relatedTo");
    let true_ = rt.intern("true");
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

    for expected in ["abc", ""] {
        let n = rt.log().len();
        let err = rt
            .emit(Op::Behavior {
                name: "derive_pq".into(),
                caused_by: cause,
                rule_version: expected.into(),
                subject: q,
                relation: rel,
                object: true_,
                valid_from: 2010,
                valid_to: None,
            })
            .unwrap_err();
        assert!(
            matches!(err, RuntimeError::UnknownRuleVersion { ref pin } if pin == expected),
            "{err:?}"
        );
        assert_eq!(
            n,
            rt.log().len(),
            "fail-closed: free-string pin must not append"
        );
    }

    let n = rt.log().len();
    let err = rt.emit_rule(String::new(), 2010, None).unwrap_err();
    assert!(
        matches!(err, RuntimeError::UnknownRuleVersion { ref pin } if pin.is_empty()),
        "{err:?}"
    );
    assert_eq!(
        n,
        rt.log().len(),
        "fail-closed: empty definition must not append"
    );
}

#[test]
fn retract_register_rule_drops_eligibility_at_later_cut() {
    let mut rt = Runtime::default();
    let p = rt.intern("P");
    let q = rt.intern("Q");
    let rel = rt.intern("relatedTo");
    let true_ = rt.intern("true");
    let pin = rule_definition_hash("derive_pq");
    let registered = rt.emit_rule("derive_pq", 2010, None).unwrap();
    let rule_id = registered.receipt.event_ids[0];
    let n = rt.log().len();

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
            rule_version: pin.clone(),
            subject: q,
            relation: rel,
            object: true_,
            valid_from: 2010,
            valid_to: None,
        })
        .unwrap();
    let claim_q = derived.receipt.event_ids[0];
    assert!(rt.derivation_eligible_at(claim_q, TransactionTime::MAX, ValidTime::from(2017)));

    rt.emit(Op::Retract { event_id: rule_id }).unwrap();

    let after = rt.log().len();
    let err = rt
        .emit(Op::Behavior {
            name: "derive_pq".into(),
            caused_by: cause,
            rule_version: pin.clone(),
            subject: q,
            relation: rel,
            object: true_,
            valid_from: 2010,
            valid_to: None,
        })
        .unwrap_err();
    assert!(
        matches!(err, RuntimeError::UnknownRuleVersion { ref pin } if pin == &rule_definition_hash("derive_pq")),
        "{err:?}"
    );
    assert_eq!(after, rt.log().len(), "fail-closed: log must not grow");
    assert!(
        !rt.derivation_eligible_at(claim_q, TransactionTime::MAX, ValidTime::from(2017)),
        "retracted registry hash must drop eligibility"
    );

    let mut prefix = rt.fork_at(n);
    let p = prefix.intern("P");
    let q = prefix.intern("Q");
    let rel = prefix.intern("relatedTo");
    let true_ = prefix.intern("true");
    let asserted = prefix
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
    prefix
        .emit(Op::Behavior {
            name: "derive_pq".into(),
            caused_by: cause,
            rule_version: pin,
            subject: q,
            relation: rel,
            object: true_,
            valid_from: 2010,
            valid_to: None,
        })
        .unwrap();
}

#[test]
fn persist_open_behavior_requires_hydrated_registry() {
    let mut rt = Runtime::default();
    let p = rt.intern("P");
    let q = rt.intern("Q");
    let rel = rt.intern("relatedTo");
    let true_ = rt.intern("true");
    let pin = rule_definition_hash("derive_pq");
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
    rt.emit(Op::Behavior {
        name: "derive_pq".into(),
        caused_by: cause,
        rule_version: pin.clone(),
        subject: q,
        relation: rel,
        object: true_,
        valid_from: 2010,
        valid_to: None,
    })
    .unwrap();

    let dir = std::env::temp_dir().join(format!(
        "kutha-m012-s02-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    store::persist(&rt, &dir).unwrap();
    let mut opened = store::open(&dir).unwrap();
    assert!(
        opened.log().iter().any(|e| {
            matches!(
                &e.op,
                Op::Assert {
                    delivery_key: Some(k),
                    ..
                } if k == &pin
            )
        }),
        "opened log must contain RegisterRule"
    );
    let q = opened.intern("Q");
    let rel = opened.intern("relatedTo");
    let true_ = opened.intern("true");
    let p = opened.intern("P");
    let asserted = opened
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
    opened
        .emit(Op::Behavior {
            name: "derive_pq".into(),
            caused_by: cause,
            rule_version: pin,
            subject: q,
            relation: rel,
            object: true_,
            valid_from: 2010,
            valid_to: None,
        })
        .unwrap();

    let n = opened.log().len();
    let err = opened
        .emit(Op::Behavior {
            name: "derive_pq".into(),
            caused_by: cause,
            rule_version: "abc".into(),
            subject: q,
            relation: rel,
            object: true_,
            valid_from: 2010,
            valid_to: None,
        })
        .unwrap_err();
    assert!(
        matches!(err, RuntimeError::UnknownRuleVersion { ref pin } if pin == "abc"),
        "{err:?}"
    );
    assert_eq!(
        n,
        opened.log().len(),
        "fail-closed: free-string pin must not append"
    );
    let _ = std::fs::remove_dir_all(&dir);
}
