//! RULE-01: canonical RuleAst encoding (M016 S02). Not an evaluator.

use kutha_common::{RuleAst, RuleAtom, RuleTerm, rule_body_hash};

#[test]
fn named_and_horn_round_trip() {
    let named = RuleAst::named("derive_pq").expect("ident");
    let bytes = named.encode();
    let parsed = RuleAst::from_canonical(&bytes).expect("canonical");
    assert_eq!(named, parsed);
    assert_eq!(bytes, parsed.encode());

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
    let horn = RuleAst::horn(head, vec![prem]).expect("horn");
    let hb = horn.encode();
    let hp = RuleAst::from_canonical(&hb).expect("horn canonical");
    assert_eq!(horn, hp);
    assert_eq!(hb, hp.encode());
}

#[test]
fn from_canonical_rejects_non_canonical() {
    let named = RuleAst::named("derive_pq").unwrap();
    let mut trailing = named.encode();
    trailing.push(0x00);
    assert!(RuleAst::from_canonical(&trailing).is_none());

    assert!(RuleAst::from_canonical(&[0x00]).is_none(), "unknown tag");
    assert!(
        RuleAst::from_canonical(b"derive_pq").is_none(),
        "raw utf8 name is not canonical"
    );
    assert!(
        RuleAst::from_canonical(br#"{"name":"derive_pq"}"#).is_none(),
        "JSON is not canonical"
    );
}

#[test]
fn body_hash_is_tagged_not_utf8_domain() {
    let ast = RuleAst::named("derive_pq").unwrap();
    let pin = ast.hash();
    assert_eq!(64, pin.len());
    assert!(pin.chars().all(|c| matches!(c, '0'..='9' | 'a'..='f')));
    assert_eq!(pin, rule_body_hash(&ast.encode()));
    assert_ne!(&ast.encode(), b"derive_pq");
}

#[test]
fn constructors_enforce_bounds() {
    assert!(RuleAst::named("").is_none());
    assert!(RuleAst::named(" derive_pq").is_none());
    assert!(RuleAst::named("{json}").is_none());
    assert!(RuleAst::named("1abc").is_none());
    let too_long = "a".to_string() + &"b".repeat(64);
    assert_eq!(65, too_long.len());
    assert!(RuleAst::named(&too_long).is_none());
    assert!(RuleTerm::var(16).is_none());
    let head = RuleAtom::new(
        "derive_pq",
        RuleTerm::var(0).unwrap(),
        RuleTerm::var(1).unwrap(),
    )
    .unwrap();
    assert!(RuleAst::horn(head.clone(), vec![]).is_none());
    let prem = RuleAtom::new(
        "relatedTo",
        RuleTerm::var(0).unwrap(),
        RuleTerm::constant("true").unwrap(),
    )
    .unwrap();
    let nine = vec![prem.clone(); 9];
    assert!(RuleAst::horn(head, nine).is_none());
}
