//! M175 phase 257: closed-default C4 path-policy compiler tests.
//!
//! This is a bounded foundation; no grants, tenants, vacuum authority,
//! capabilities, WASM, full ABAC language, or ADR status change.

use kutha_common::{Op, TransactionTime, ValidTime};
use kutha_runtime::{
    AuthorizedQuery, PathPolicy, QueryIr, Runtime, SecurityError, compile_path_policy,
    execute_authorized, try_hops_named,
};

fn statute_runtime() -> Runtime {
    let mut rt = Runtime::default();
    let article = rt.intern("art-12-speed");
    let in_force = rt.intern("inForceAs");
    let limit_50 = rt.intern("50-kmh");
    let limit_30 = rt.intern("30-kmh");
    rt.emit(Op::Assert {
        subject: article,
        relation: in_force,
        object: limit_50,
        valid_from: 2010,
        valid_to: Some(2020),
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .expect("first assert");
    rt.emit(Op::Assert {
        subject: article,
        relation: in_force,
        object: limit_30,
        valid_from: 2020,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .expect("second assert");
    rt
}

fn hop(rt: &Runtime, rels: &[&str], k: u32) -> kutha_runtime::QueryIr {
    try_hops_named(
        rt,
        "art-12-speed",
        rels,
        k,
        TransactionTime::MAX,
        ValidTime::from(2015),
        0,
    )
    .expect("typed hop")
}

#[test]
fn labeled_allowed_hop_compiles_allowed() {
    let rt = statute_runtime();
    let q = hop(&rt, &["inForceAs"], 1);
    let policy = PathPolicy::closed().allow_relations(["inForceAs"]);
    assert_eq!(
        compile_path_policy(&rt, &q, &policy),
        Ok(AuthorizedQuery::Allowed(q))
    );
}

#[test]
fn denied_relation_compiles_denied_before_prepare() {
    let rt = statute_runtime();
    let q = hop(&rt, &["inForceAs"], 1);
    let policy = PathPolicy::closed().allow_relations(["knows"]);
    assert_eq!(
        compile_path_policy(&rt, &q, &policy),
        Ok(AuthorizedQuery::Denied {
            reason: "relation inForceAs is not allowed".to_string(),
        })
    );
}

#[test]
fn scan_denies_by_default_and_allows_explicitly() {
    let rt = statute_runtime();
    let q = kutha_runtime::QueryIr::Scan {
        tt: TransactionTime::MAX,
        vt: ValidTime::from(2015),
        limit: 0,
    };
    assert_eq!(
        compile_path_policy(&rt, &q, &PathPolicy::closed()),
        Ok(AuthorizedQuery::Denied {
            reason: "scan is denied by closed-default path policy".to_string(),
        })
    );
    assert_eq!(
        compile_path_policy(&rt, &q, &PathPolicy::closed().allow_scan()),
        Ok(AuthorizedQuery::Allowed(q))
    );
}

#[test]
fn unlabeled_and_zero_hop_walks_deny_by_default() {
    let rt = statute_runtime();
    let unlabeled = hop(&rt, &[], 1);
    let zero = hop(&rt, &["inForceAs"], 0);
    for q in [unlabeled, zero] {
        assert_eq!(
            compile_path_policy(
                &rt,
                &q,
                &PathPolicy::closed().allow_relations(["inForceAs"])
            ),
            Ok(AuthorizedQuery::Denied {
                reason: "unlabeled walk is denied by closed-default path policy".to_string(),
            })
        );
    }
}

#[test]
fn multi_hop_requires_every_relation() {
    let rt = statute_runtime();
    let allowed = hop(&rt, &["inForceAs", "inForceAs"], 2);
    let policy = PathPolicy::closed().allow_relations(["inForceAs"]);
    assert!(matches!(
        compile_path_policy(&rt, &allowed, &policy),
        Ok(AuthorizedQuery::Allowed(_))
    ));

    let mixed = hop(&rt, &["inForceAs", "knows"], 2);
    let policy = PathPolicy::closed().allow_relations(["inForceAs", "knows"]);
    assert!(matches!(
        compile_path_policy(&rt, &mixed, &policy),
        Ok(AuthorizedQuery::Allowed(_))
    ));

    // Construct a per-hop typed query whose second relation resolves in the
    // dictionary but is absent from the path-policy allowlist.
    let article = rt.dictionary().id("art-12-speed").expect("article");
    let in_force = rt.dictionary().id("inForceAs").expect("inForceAs");
    let denied_relation = rt.dictionary().id("50-kmh").expect("dictionary term");
    let denied_mixed = QueryIr::Hops {
        start: article,
        rels: vec![in_force, denied_relation],
        k: 2,
        tt: TransactionTime::MAX,
        vt: ValidTime::from(2015),
        limit: 0,
    };
    let policy = PathPolicy::closed().allow_relations(["inForceAs"]);
    assert_eq!(
        compile_path_policy(&rt, &denied_mixed, &policy),
        Ok(AuthorizedQuery::Denied {
            reason: "relation 50-kmh is not allowed".to_string(),
        })
    );
}

/// `QueryRejected` is the post-policy arm: the path policy allowed the query,
/// then the query engine refused it. Two rels under `k=3` is not the 0, 1, or k
/// shape, so `prepare` rejects it after authorization. Reverting the mapping to
/// `PolicyDenied` fails this test.
#[test]
fn post_policy_rels_shape_failure_is_query_rejected() {
    let rt = statute_runtime();
    let start = rt.dictionary().id("art-12-speed").expect("article");
    let in_force = rt.dictionary().id("inForceAs").expect("inForceAs");
    let q = QueryIr::Hops {
        start,
        rels: vec![in_force, in_force],
        k: 3,
        tt: TransactionTime::MAX,
        vt: ValidTime::from(2015),
        limit: 0,
    };
    let policy = PathPolicy::closed().allow_relations(["inForceAs"]);
    assert_eq!(
        compile_path_policy(&rt, &q, &policy),
        Ok(AuthorizedQuery::Allowed(q.clone()))
    );
    let err = execute_authorized(&rt, &q, &policy).expect_err("rels shape must be rejected");
    assert!(
        matches!(&err, SecurityError::QueryRejected { reason } if reason.contains("invalid relation shape")),
        "post-policy shape failure must be QueryRejected; got {err:?}"
    );
}

/// The other mapped arm: the path policy inspects relations only, so an
/// unknown start term passes it and is refused by `prepare`.
#[test]
fn post_policy_unknown_term_is_query_rejected() {
    let rt = statute_runtime();
    let in_force = rt.dictionary().id("inForceAs").expect("inForceAs");
    let q = QueryIr::Hops {
        start: kutha_common::TermId::MAX,
        rels: vec![in_force],
        k: 1,
        tt: TransactionTime::MAX,
        vt: ValidTime::from(2015),
        limit: 0,
    };
    let policy = PathPolicy::closed().allow_relations(["inForceAs"]);
    assert_eq!(
        compile_path_policy(&rt, &q, &policy),
        Ok(AuthorizedQuery::Allowed(q.clone()))
    );
    let err = execute_authorized(&rt, &q, &policy).expect_err("unknown start must be rejected");
    assert!(
        matches!(&err, SecurityError::QueryRejected { reason } if reason.contains("unknown term")),
        "post-policy unknown term must be QueryRejected; got {err:?}"
    );
}
