//! M003a phase 254: differential falsifier.
//!
//! Proves GPML text compiles to the exact typed IR used by the B3 workload and
//! that `AS OF` through GPML matches the FF5 typed facade.

use kutha_common::{Op, TransactionTime, ValidTime};
use kutha_runtime::{
    GpmlError, QueryRows, Runtime, RuntimeBinder, execute, parse_gpml, prepare, try_hops_named,
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

fn layered_runtime(n: u64) -> (Runtime, Vec<String>) {
    let mut rt = Runtime::default();
    let mut names = Vec::new();
    let mut layers: [Vec<String>; 3] = [Vec::new(), Vec::new(), Vec::new()];
    for (layer, slot) in layers.iter_mut().enumerate() {
        slot.reserve(n as usize);
        for i in 0..n {
            let name = format!("L{layer}_{i}");
            slot.push(name.clone());
            rt.intern(&name);
        }
    }
    let relation = rt.intern("relatedTo");
    for layer in 0..2 {
        for i in 0..n {
            let src = rt
                .dictionary()
                .id(&layers[layer][i as usize])
                .expect("source");
            for f in 0..4u64 {
                let dst_idx = (i.wrapping_mul(4).wrapping_add(f)) % n;
                let dst = rt
                    .dictionary()
                    .id(&layers[layer + 1][dst_idx as usize])
                    .expect("dest");
                rt.emit(Op::Assert {
                    subject: src,
                    relation,
                    object: dst,
                    valid_from: 2010,
                    valid_to: None,
                    claim: None,
                    delivery_key: None,
                    polarity: None,
                })
                .expect("layered assert");
            }
        }
    }
    names.extend(layers[0].iter().cloned());
    (rt, names)
}

fn render_hop(start: &str, relation: &str, hops: u32, vt: u64, limit: usize) -> String {
    let hop = format!("[:{relation}*{hops}]");
    format!("MATCH (a:Node {{name: \"{start}\"}})-{hop}->(x) AS OF {vt} RETURN x LIMIT {limit}")
}

fn expect_hop_ir(rt: &Runtime, start: &str, relation: &str, hops: u32, vt: u64, limit: usize) {
    let text = render_hop(start, relation, hops, vt, limit);
    let parsed = parse_gpml(&text, &RuntimeBinder::new(rt)).expect("GPML parses");
    let expected = try_hops_named(
        rt,
        start,
        &[relation],
        hops,
        TransactionTime::MAX,
        ValidTime::from(vt),
        limit,
    )
    .expect("typed IR");
    assert_eq!(parsed, expected, "text={text}");
}

#[test]
fn b3_workload_render_parse_parity() {
    let (rt, starts) = layered_runtime(8);
    for start in &starts {
        for hops in [1u32, 2] {
            expect_hop_ir(&rt, start, "relatedTo", hops, 2010, 0);
        }
    }
    // Also prove one parsed B3 query executes to a non-empty deduplicated frontier.
    let text = render_hop("L0_0", "relatedTo", 2, 2010, 0);
    let q = parse_gpml(&text, &RuntimeBinder::new(&rt)).expect("B3 query");
    match execute(&rt, &q).expect("execute B3") {
        QueryRows::Nodes(nodes) => assert!(!nodes.is_empty()),
        QueryRows::Triples(_) => panic!("expected nodes"),
    }
}

#[test]
fn ff5_gpml_as_of_matches_typed_facade() {
    let rt = statute_runtime();
    for vt in [2015u64, 2020u64] {
        let text = render_hop("art-12-speed", "inForceAs", 1, vt, 0);
        let q = parse_gpml(&text, &RuntimeBinder::new(&rt)).expect("FF5 GPML");
        let got = execute(&rt, &q).expect("execute FF5 GPML");
        let facade_nodes = rt.as_of(ValidTime::from(vt));
        let article = rt.dictionary().id("art-12-speed").expect("article");
        let relation = rt.dictionary().id("inForceAs").expect("inForceAs");
        let mut expected: Vec<_> = facade_nodes
            .iter()
            .filter(|(s, r, _)| *s == article && *r == relation)
            .map(|(_, _, o)| *o)
            .collect();
        expected.sort_unstable();
        expected.dedup();
        match got {
            QueryRows::Nodes(nodes) => assert_eq!(nodes, expected, "vt={vt}"),
            QueryRows::Triples(_) => panic!("expected nodes"),
        }
    }
}

#[test]
fn unknown_vocabulary_and_out_of_scope_shapes_fail_closed() {
    let (rt, starts) = layered_runtime(2);
    let binder = RuntimeBinder::new(&rt);
    let unknown_relation = render_hop(&starts[0], "notAllowed", 1, 2010, 0);
    assert_eq!(
        parse_gpml(&unknown_relation, &binder),
        Err(GpmlError::UnknownRelation {
            name: "notAllowed".to_string()
        })
    );
    let unknown_term = render_hop("not-interned", "relatedTo", 1, 2010, 0);
    assert_eq!(
        parse_gpml(&unknown_term, &binder),
        Err(GpmlError::UnknownTerm {
            name: "not-interned".to_string()
        })
    );
    let variable_hop = format!(
        "MATCH (a:Node {{name: \"{}\"}})-[:relatedTo*1..9]->(x) AS OF 2010 RETURN x",
        starts[0]
    );
    assert!(matches!(
        parse_gpml(&variable_hop, &binder),
        Err(GpmlError::Unsupported { .. })
    ));
}

#[test]
fn typed_tt_and_limit_are_preserved() {
    let (rt, starts) = layered_runtime(2);
    let text = format!(
        "MATCH (a:Node {{name: \"{}\"}})-[:relatedTo*2]->(x) AS OF 2010 TT 77 RETURN x LIMIT 3",
        starts[0]
    );
    let q = parse_gpml(&text, &RuntimeBinder::new(&rt)).expect("parse");
    let expected = try_hops_named(
        &rt,
        &starts[0],
        &["relatedTo"],
        2,
        TransactionTime::from(77),
        ValidTime::from(2010),
        3,
    )
    .expect("typed IR");
    assert_eq!(q, expected);
}

#[test]
fn mixed_repeated_chain_fails_closed_at_prepare() {
    let (rt, starts) = layered_runtime(2);
    let text = format!(
        "MATCH (a:Node {{name: \"{}\"}})-[:relatedTo*2]->(b)-[:knownBy]->(x) AS OF 2010 RETURN x",
        starts[0]
    );
    let q = parse_gpml(&text, &RuntimeBinder::new(&rt)).expect("mixed chain parses");
    match prepare(&rt, &q) {
        Err(kutha_runtime::QueryError::RelsShape { rels, k }) => {
            assert_eq!((rels, k), (2, 3));
        }
        other => panic!("expected RelsShape, got {other:?}"),
    }
}
