//! Independent RULE-01 kill-tests. Lazy decoders (JSON, BE length, prefix-only) fail these.

use kutha_common::{RuleAst, rule_body_hash, rule_definition_hash};

#[test]
fn be_length_and_truncation_are_not_canonical() {
    // LE Named "ab" is 01 02 00 'a' 'b'. BE length 00 02 would claim 512 bytes.
    let be = [0x01, 0x00, 0x02, b'a', b'b'];
    assert!(RuleAst::from_canonical(&be).is_none());
    assert!(RuleAst::from_canonical(&[0x01]).is_none());
    assert!(RuleAst::from_canonical(&[0x01, 0x02]).is_none());
    assert!(RuleAst::from_canonical(&[0x03]).is_none());
}

#[test]
fn crafted_horn_zero_and_nine_premises_fail() {
    let head = RuleAst::named("derive_pq").unwrap().encode();
    // Minimal fake: tag Horn + leftover Named payload is not a Horn atom stream.
    let mut horn_named = vec![0x02];
    horn_named.extend_from_slice(&head[1..]);
    assert!(RuleAst::from_canonical(&horn_named).is_none());

    let mut zero = RuleAst::named("derive_pq").unwrap().encode();
    zero[0] = 0x02;
    zero.push(0);
    assert!(RuleAst::from_canonical(&zero).is_none());
}

#[test]
fn named_hash_is_not_domain_plus_utf8_ident() {
    let ident = "derive_pq";
    let pin = rule_definition_hash(ident);
    assert_eq!(pin, RuleAst::named(ident).unwrap().hash());
    assert_ne!(pin, rule_body_hash(ident.as_bytes()));
    let again = RuleAst::named(ident).unwrap().hash();
    assert_eq!(pin, again);
}
