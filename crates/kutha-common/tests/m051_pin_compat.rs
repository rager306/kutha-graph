//! G4 / CE #24 — pin scheme v1 stability vectors (M051).
//! Hardcoded digests; lazy UTF-8-domain pins for idents must not match.

use kutha_common::{RuleAst, rule_body_hash, rule_definition_hash};

/// Pin scheme v1: Named AST hash for `derive_pq`.
const PIN_V1_DERIVE_PQ: &str =
    "2dca8c1394bfd203753efd7c991030c23719b41b285aad78ef061103b316596d";

/// Main-era / pre–RULE-01: domain + raw UTF-8 of the same ident (must diverge).
const LEGACY_UTF8_DERIVE_PQ: &str =
    "7ce5d5ad5ca8d5148305d84d3eee3f1356540ad44da56fac9fdde748603436db";

/// Non-ident free string keeps domain + UTF-8 (fail-closed admit path).
const PIN_NONIDENT: &str =
    "ac715049d46ceaeae121295921ba657df849db9fee9fdb1203201e7b7ba7a689";

const NONIDENT: &str = "not a rule ident!!!";

#[test]
fn ident_pin_matches_hardcoded_v1_vector() {
    let pin = rule_definition_hash("derive_pq");
    assert_eq!(pin, PIN_V1_DERIVE_PQ);
    assert_eq!(pin, RuleAst::named("derive_pq").unwrap().hash());
}

#[test]
fn ident_pin_diverges_from_legacy_utf8_domain_vector() {
    let pin = rule_definition_hash("derive_pq");
    assert_ne!(pin, LEGACY_UTF8_DERIVE_PQ);
    assert_eq!(rule_body_hash(b"derive_pq"), LEGACY_UTF8_DERIVE_PQ);
    assert_ne!(pin, rule_body_hash(b"derive_pq"));
}

#[test]
fn nonident_pin_matches_hardcoded_domain_utf8_vector() {
    let pin = rule_definition_hash(NONIDENT);
    assert_eq!(pin, PIN_NONIDENT);
    assert_eq!(pin, rule_body_hash(NONIDENT.as_bytes()));
    assert!(RuleAst::named(NONIDENT).is_none());
}
