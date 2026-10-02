//! Independent RULE-01 admit kill-tests. Lazy "hash the JSON / utf8 name" paths fail these.

use kutha_common::{blob_digest_matches, rule_definition_hash, Op, RuleAst};
use kutha_runtime::{Runtime, RuntimeError};

#[test]
fn store_blob_json_with_named_pin_does_not_append() {
    let mut rt = Runtime::default();
    let n = rt.log().len();
    let pin = rule_definition_hash("derive_pq");
    let json = br#"{"Named":"derive_pq"}"#.to_vec();
    let err = rt
        .emit(Op::StoreBlob {
            digest: pin.clone(),
            bytes: json.clone(),
        })
        .unwrap_err();
    assert!(
        matches!(err, RuntimeError::UnknownRuleVersion { pin: ref p } if p == &pin),
        "{err:?}"
    );
    assert_eq!(n, rt.log().len());
    assert!(!blob_digest_matches(&pin, &json));
    assert!(!blob_digest_matches(&pin, b"derive_pq"));
    let canonical = RuleAst::named("derive_pq").unwrap().encode();
    assert!(blob_digest_matches(&pin, &canonical));
}
