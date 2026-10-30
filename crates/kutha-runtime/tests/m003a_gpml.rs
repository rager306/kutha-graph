//! M003a phase 253: focused parser/binder tests for the read-only GPML subset.
//!
//! Differential render/parse parity and FF5 facade parity are phase 254.

use kutha_common::{Op, ValidTime};
use kutha_runtime::{GpmlError, QueryIr, Runtime, RuntimeBinder, execute, parse_gpml, prepare};

const T_OLD: u64 = 2015;
const T_NEW: u64 = 2020;

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
    .expect("first statute assert admits");
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
    .expect("second statute assert admits");
    rt
}

#[test]
fn parses_one_hop_to_query_ir_and_executes_as_of() {
    let rt = statute_runtime();
    let binder = RuntimeBinder::new(&rt);
    let q = parse_gpml(
        "MATCH (a:Statute {name: \"art-12-speed\"})-[:inForceAs]->(x) AS OF 2015 RETURN x",
        &binder,
    )
    .expect("one-hop GPML parses");
    let expected = try_named_expected(&rt, "art-12-speed", &["inForceAs"], 1, u64::MAX, T_OLD, 0);
    assert_eq!(q, expected);
    let rows = execute(&rt, &q).expect("execute");
    let nodes = match rows {
        kutha_runtime::QueryRows::Nodes(nodes) => nodes,
        kutha_runtime::QueryRows::Triples(_) => panic!("hops must return nodes"),
    };
    let limit_50 = rt.dictionary().id("50-kmh").expect("interned");
    assert_eq!(nodes, vec![limit_50]);
}

#[test]
fn parses_bounded_variable_length_as_fixed_upper_bound() {
    let rt = statute_runtime();
    let binder = RuntimeBinder::new(&rt);
    let q = parse_gpml(
        "MATCH (a:Statute {name: \"art-12-speed\"})-[:inForceAs*1..2]->(x) AS OF 2020 RETURN x LIMIT 7",
        &binder,
    )
    .expect("bounded GPML parses");
    let expected = try_named_expected(&rt, "art-12-speed", &["inForceAs"], 2, u64::MAX, T_NEW, 7);
    assert_eq!(q, expected);
}

#[test]
fn parses_single_named_node_as_zero_hop() {
    let rt = statute_runtime();
    let binder = RuntimeBinder::new(&rt);
    let q = parse_gpml(
        "MATCH (a:Statute {name: \"art-12-speed\"}) AS OF 2015 RETURN a LIMIT 1",
        &binder,
    )
    .expect("single node parses");
    let expected = try_named_expected(&rt, "art-12-speed", &[], 0, u64::MAX, T_OLD, 1);
    assert_eq!(q, expected);
}

#[test]
fn unknown_relation_and_term_fail_closed() {
    let rt = statute_runtime();
    let binder = RuntimeBinder::new(&rt);
    let err = parse_gpml(
        "MATCH (a:Statute {name: \"art-12-speed\"})-[:notAllowed]->(x) AS OF 2015 RETURN x",
        &binder,
    )
    .expect_err("unknown relation refused");
    assert_eq!(
        err,
        GpmlError::UnknownRelation {
            name: "notAllowed".to_string()
        }
    );

    let err = parse_gpml(
        "MATCH (a:Statute {name: \"not-interned\"})-[:inForceAs]->(x) AS OF 2015 RETURN x",
        &binder,
    )
    .expect_err("unknown term refused");
    assert_eq!(
        err,
        GpmlError::UnknownTerm {
            name: "not-interned".to_string()
        }
    );
}

#[test]
fn unsupported_constructs_fail_closed() {
    let rt = statute_runtime();
    let binder = RuntimeBinder::new(&rt);
    for q in [
        "MATCH (a:Statute {age: 3}) AS OF 2015 RETURN a",
        "MATCH (a:Statute {name: \"art-12-speed\"})-[:inForceAs]->(b:Statute {name: \"x\"}) AS OF 2015 RETURN b",
        "MATCH (a:Statute {name: \"art-12-speed\"})-[:inForceAs*0]->(x) AS OF 2015 RETURN x",
        "MATCH (a:Statute {name: \"art-12-speed\"})-[:inForceAs*1..9]->(x) AS OF 2015 RETURN x",
    ] {
        let err = parse_gpml(q, &binder).expect_err(q);
        assert!(
            matches!(err, GpmlError::Unsupported { .. }),
            "q={q}, err={err:?}"
        );
    }
    // Writes are outside the parser's accepted entry keyword and fail closed.
    assert!(matches!(
        parse_gpml("CREATE (a:Statute)", &binder),
        Err(GpmlError::Parse { .. })
    ));
}

#[test]
fn malformed_queries_return_parse_errors() {
    let rt = statute_runtime();
    let binder = RuntimeBinder::new(&rt);
    for q in [
        "MATCH (a:Statute {name: \"art-12-speed\"}) AS OF 2015 RETURN",
        "MATCH (a:Statute {name: \"art-12-speed\"})-[:inForceAs]-> AS OF 2015 RETURN x",
        "MATCH (a:Statute {name: \"art-12-speed\"}) AS OF twenty RETURN x",
    ] {
        assert!(
            matches!(parse_gpml(q, &binder), Err(GpmlError::Parse { .. })),
            "q={q}"
        );
    }
}

#[test]
fn typed_cut_is_not_stringly_and_prepare_matches_handwritten_ir() {
    let rt = statute_runtime();
    let binder = RuntimeBinder::new(&rt);
    let q = parse_gpml(
        "MATCH (a:Statute {name: \"art-12-speed\"})-[:inForceAs]->(x) AS OF 2015 TT 99 RETURN x",
        &binder,
    )
    .expect("GPML with TT parses");
    let QueryIr::Hops { tt, vt, .. } = q else {
        panic!("expected hops");
    };
    assert_eq!(tt, kutha_common::TransactionTime::from(99));
    assert_eq!(vt, ValidTime::from(2015));
    let prepared_parser = prepare(&rt, &q).expect("prepared");
    let handwritten = try_named_expected(&rt, "art-12-speed", &["inForceAs"], 1, 99, 2015, 0);
    let prepared_hand = prepare(&rt, &handwritten).expect("prepared");
    assert_eq!(format!("{prepared_parser:?}"), format!("{prepared_hand:?}"));
}

fn try_named_expected(
    rt: &Runtime,
    start: &str,
    rels: &[&str],
    k: u32,
    tt: u64,
    vt: u64,
    limit: usize,
) -> QueryIr {
    kutha_runtime::try_hops_named(
        rt,
        start,
        rels,
        k,
        kutha_common::TransactionTime::from(tt),
        ValidTime::from(vt),
        limit,
    )
    .expect("handwritten IR")
}
