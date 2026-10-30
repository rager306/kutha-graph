//! M175 phase 258: policy compiler integration with typed and GPML queries.

use kutha_common::{Op, TransactionTime, ValidTime};
use kutha_runtime::{
    AuthorizedQuery, PathPolicy, QueryIr, QueryRows, Runtime, RuntimeBinder, SecurityError,
    compile_path_policy, execute, execute_authorized, parse_gpml, try_hops_named,
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

fn allowed_policy() -> PathPolicy {
    PathPolicy::closed().allow_relations(["inForceAs"])
}

#[test]
fn authorized_gpml_result_equals_direct_execution() {
    let rt = statute_runtime();
    let text = "MATCH (a:Statute {name: \"art-12-speed\"})-[:inForceAs]->(x) AS OF 2015 RETURN x";
    let q = parse_gpml(text, &RuntimeBinder::new(&rt)).expect("GPML parses");
    let direct = execute(&rt, &q).expect("direct execute");
    let authorized = execute_authorized(&rt, &q, &allowed_policy()).expect("authorized execute");
    assert_eq!(authorized, direct);
    let QueryRows::Nodes(nodes) = authorized else {
        panic!("expected nodes");
    };
    let limit_50 = rt.dictionary().id("50-kmh").expect("interned");
    assert_eq!(nodes, vec![limit_50]);
}

#[test]
fn denied_gpml_returns_policy_error_not_empty_rows() {
    let rt = statute_runtime();
    let text = "MATCH (a:Statute {name: \"art-12-speed\"})-[:inForceAs]->(x) AS OF 2015 RETURN x";
    let q = parse_gpml(text, &RuntimeBinder::new(&rt)).expect("GPML parses");
    let denied = PathPolicy::closed().allow_relations(["knows"]);
    assert_eq!(
        execute_authorized(&rt, &q, &denied),
        Err(SecurityError::PolicyDenied {
            reason: "relation inForceAs is not allowed".to_string(),
        })
    );
}

#[test]
fn authorized_typed_two_hop_preserves_query() {
    let rt = statute_runtime();
    let q = try_hops_named(
        &rt,
        "art-12-speed",
        &["inForceAs", "inForceAs"],
        2,
        TransactionTime::MAX,
        ValidTime::from(2015),
        3,
    )
    .expect("typed query");
    let compiled = compile_path_policy(&rt, &q, &allowed_policy()).expect("policy compiles");
    assert_eq!(compiled, AuthorizedQuery::Allowed(q.clone()));
    let authorized = execute_authorized(&rt, &q, &allowed_policy()).expect("authorized");
    let direct = execute(&rt, &q).expect("direct");
    assert_eq!(authorized, direct);
}

#[test]
fn closed_default_scan_and_unlabeled_are_denied() {
    let rt = statute_runtime();
    let scan = QueryIr::Scan {
        tt: TransactionTime::MAX,
        vt: ValidTime::from(2015),
        limit: 0,
    };
    assert!(matches!(
        execute_authorized(&rt, &scan, &PathPolicy::closed()),
        Err(SecurityError::PolicyDenied { .. })
    ));
    let unlabeled = try_hops_named(
        &rt,
        "art-12-speed",
        &[],
        1,
        TransactionTime::MAX,
        ValidTime::from(2015),
        0,
    )
    .expect("typed unlabeled");
    assert!(matches!(
        execute_authorized(&rt, &unlabeled, &allowed_policy()),
        Err(SecurityError::PolicyDenied { .. })
    ));
}
