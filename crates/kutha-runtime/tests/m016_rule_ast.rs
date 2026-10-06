//! RULE-01: canonical rule body hash is rule_version; non-canonical fail closed.

use kutha_common::{
    Op, RuleAst, RuleAtom, RuleTerm, blob_digest_matches, rule_body_hash, rule_definition_hash,
};
use kutha_runtime::{Runtime, RuntimeError};

#[test]
fn emit_rule_json_is_rejected_log_unchanged() {
    let mut rt = Runtime::default();
    let n = rt.log().len();
    let json = r#"{"name":"derive_pq"}"#;
    let err = rt.emit_rule(json, 2010, None).unwrap_err();
    assert!(
        matches!(err, RuntimeError::UnknownRuleVersion { ref pin } if pin == json),
        "{err:?}"
    );
    assert_eq!(n, rt.log().len());
    let err = rt.emit_rule(" derive_pq", 2010, None).unwrap_err();
    assert!(matches!(err, RuntimeError::UnknownRuleVersion { .. }));
    assert_eq!(n, rt.log().len());
}

#[test]
fn emit_rule_stores_canonical_named_pin_equals_hash() {
    let mut rt = Runtime::default();
    let pin = rule_definition_hash("derive_pq");
    assert_eq!(pin, RuleAst::named("derive_pq").unwrap().hash());
    rt.emit_rule("derive_pq", 2010, None).unwrap();
    let blob = rt
        .log()
        .as_slice()
        .iter()
        .rev()
        .find_map(|e| match &e.op {
            Op::StoreBlob { digest, bytes } => Some((digest.clone(), bytes.clone())),
            _ => None,
        })
        .expect("StoreBlob");
    assert_eq!(pin, blob.0);
    let ast = RuleAst::from_canonical(&blob.1).expect("canonical");
    assert_eq!(ast, RuleAst::named("derive_pq").unwrap());
    assert!(!blob_digest_matches(&pin, b"derive_pq"));
    assert!(blob_digest_matches(&pin, &blob.1));
    assert_eq!(pin, rule_body_hash(&blob.1));

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
    rt.emit(Op::Behavior {
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
fn emit_rule_ast_horn_pin_equals_body_hash() {
    let mut rt = Runtime::default();
    rt.emit_allow("derive_pq", 0, None).unwrap();
    let head = RuleAtom::new(
        "derive_pq",
        RuleTerm::var(0).unwrap(),
        RuleTerm::var(1).unwrap(),
    )
    .unwrap();
    let prem = RuleAtom::new(
        "relatedTo",
        RuleTerm::var(0).unwrap(),
        RuleTerm::constant("true").unwrap(),
    )
    .unwrap();
    let ast = RuleAst::horn(head, vec![prem]).unwrap();
    let pin = ast.hash();
    rt.emit_rule_ast(&ast, 2010, None).unwrap();
    let blob = rt
        .log()
        .as_slice()
        .iter()
        .rev()
        .find_map(|e| match &e.op {
            Op::StoreBlob { digest, bytes } => Some((digest.clone(), bytes.clone())),
            _ => None,
        })
        .expect("StoreBlob");
    assert_eq!(pin, blob.0);
    assert_eq!(ast, RuleAst::from_canonical(&blob.1).unwrap());

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
    rt.emit(Op::Behavior {
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
