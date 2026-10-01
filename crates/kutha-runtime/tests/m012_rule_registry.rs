//! M012 S02: rule registry as log facts (RULE-01..03).
//! Not ADR-050's six dictionary kinds.

use kutha_common::{rule_definition_hash, Op};
use kutha_runtime::{Runtime, RuntimeError};

#[test]
fn register_rule_pins_behavior_to_definition_hash() {
    let mut rt = Runtime::default();
    assert_eq!(0, rt.fold().facts().len());

    let p = rt.intern("P");
    let q = rt.intern("Q");
    let rel = rt.intern("relatedTo");
    let true_ = rt.intern("true");
    let n = rt.log().len();

    rt.emit(Op::RegisterRule {
        definition: "derive_pq".into(),
        valid_from: 2010,
        valid_to: None,
    })
    .unwrap();

    assert_eq!(n + 1, rt.log().len(), "RegisterRule appends one log event");
    let last = rt.log().as_slice().last().expect("appended");
    assert!(
        matches!(
            &last.op,
            Op::RegisterRule {
                definition,
                valid_from: 2010,
                valid_to: None
            } if definition == "derive_pq"
        ),
        "{:?}",
        last.op
    );
    assert_eq!(0, rt.fold().facts().len());
    assert_eq!(0, rt.graph_len());

    let pin = rule_definition_hash("derive_pq");
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
        rt.derivation_eligible_at(claim_q, u64::MAX, 2017),
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

    rt.emit(Op::RegisterRule {
        definition: "derive_pq".into(),
        valid_from: 2010,
        valid_to: None,
    })
    .unwrap();
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
    let err = rt
        .emit(Op::RegisterRule {
            definition: String::new(),
            valid_from: 2010,
            valid_to: None,
        })
        .unwrap_err();
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
