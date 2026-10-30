//! M174 / Gate GB: GPML `AS OF` parity across the existing fixture corpus.
//!
//! Fixtures: FF5 statute, B3 layered workload, and M167 eCFR PIT sections.
//! Read-only verification; no parser or QueryIr semantics change.

use kutha_common::{Op, TermId, TransactionTime, ValidTime};
use kutha_runtime::{execute, parse_gpml, try_hops_named, GpmlError, Runtime, RuntimeBinder};

const T_OLD: u64 = 2015;
const T_NEW: u64 = 2020;

const SECTIONS: [(&str, &str, &str, &str); 3] = [
    (
        "304.1",
        include_str!("fixtures/ecfr/304.1_2015-12-18.txt"),
        include_str!("fixtures/ecfr/304.1_2017-01-23.txt"),
        include_str!("fixtures/ecfr/304.1_2017-03-14.txt"),
    ),
    (
        "304.2",
        include_str!("fixtures/ecfr/304.2_2015-12-18.txt"),
        include_str!("fixtures/ecfr/304.2_2017-01-23.txt"),
        include_str!("fixtures/ecfr/304.2_2017-03-14.txt"),
    ),
    (
        "304.10",
        include_str!("fixtures/ecfr/304.10_2015-12-18.txt"),
        include_str!("fixtures/ecfr/304.10_2017-01-23.txt"),
        include_str!("fixtures/ecfr/304.10_2017-03-14.txt"),
    ),
];

const DATES: [u64; 3] = [20151218, 20170123, 20170314];
const CUTS: [u64; 7] = [
    20151217, 20151218, 20170122, 20170123, 20170313, 20170314, 20180101,
];

fn facade_objects(
    rt: &Runtime,
    subject: &str,
    relation: &str,
    vt: u64,
) -> Vec<TermId> {
    let subject = rt.dictionary().id(subject).expect("subject interned");
    let relation = rt.dictionary().id(relation).expect("relation interned");
    let mut objects: Vec<TermId> = rt
        .as_of(ValidTime::from(vt))
        .iter()
        .filter(|(s, r, _)| *s == subject && *r == relation)
        .map(|(_, _, o)| *o)
        .collect();
    objects.sort_unstable();
    objects.dedup();
    objects
}

fn ff5_runtime() -> Runtime {
    let mut rt = Runtime::default();
    let article = rt.intern("art-12-speed");
    let in_force = rt.intern("inForceAs");
    let limit_50 = rt.intern("50-kmh");
    let limit_30 = rt.intern("30-kmh");
    for (object, from, to) in [
        (limit_50, 2010u64, Some(2020u64)),
        (limit_30, 2020, None),
    ] {
        rt.emit(Op::Assert {
            subject: article,
            relation: in_force,
            object,
            valid_from: from,
            valid_to: to,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .expect("FF5 assert admits");
    }
    rt
}

fn assert_ff5() {
    let rt = ff5_runtime();
    let binder = RuntimeBinder::new(&rt);
    for vt in [T_OLD, T_NEW] {
        let text = format!(
            "MATCH (a:Statute {{name: \"art-12-speed\"}})-[:inForceAs]->(x) AS OF {vt} RETURN x"
        );
        let parsed = parse_gpml(&text, &binder).expect("FF5 GPML parses");
        let typed = try_hops_named(
            &rt,
            "art-12-speed",
            &["inForceAs"],
            1,
            TransactionTime::MAX,
            ValidTime::from(vt),
            0,
        )
        .expect("FF5 typed IR");
        assert_eq!(parsed, typed, "FF5 vt={vt}");
        let got = execute(&rt, &parsed).expect("FF5 executes");
        let kutha_runtime::QueryRows::Nodes(nodes) = got else {
            panic!("FF5 must return nodes");
        };
        let expected = facade_objects(&rt, "art-12-speed", "inForceAs", vt);
        assert_eq!(nodes, expected, "FF5 vt={vt}");
    }
}

fn b3_runtime(n: u64) -> (Runtime, Vec<String>) {
    let mut rt = Runtime::default();
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
            let src = rt.dictionary().id(&layers[layer][i as usize]).expect("source");
            for f in 0..4u64 {
                let dst_idx = (i.wrapping_mul(4).wrapping_add(f)) % n;
                let dst = rt.dictionary().id(&layers[layer + 1][dst_idx as usize]).expect("dest");
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
                .expect("B3 assert admits");
            }
        }
    }
    (rt, layers[0].clone())
}

fn assert_b3() {
    let (rt, starts) = b3_runtime(8);
    let binder = RuntimeBinder::new(&rt);
    for start in &starts {
        for hops in [1u32, 2] {
            let text = format!(
                "MATCH (a:Node {{name: \"{start}\"}})-[:relatedTo*{hops}]->(x) AS OF 2010 RETURN x"
            );
            let parsed = parse_gpml(&text, &binder).expect("B3 GPML parses");
            let typed = try_hops_named(
                &rt,
                start,
                &["relatedTo"],
                hops,
                TransactionTime::MAX,
                ValidTime::from(2010),
                0,
            )
            .expect("B3 typed IR");
            assert_eq!(parsed, typed, "B3 start={start} hops={hops}");
            match execute(&rt, &parsed).expect("B3 executes") {
                kutha_runtime::QueryRows::Nodes(nodes) => assert!(!nodes.is_empty()),
                _ => panic!("B3 must return nodes"),
            }
        }
    }
}

fn ecfr_runtime() -> (Runtime, Vec<String>) {
    let mut rt = Runtime::default();
    let relation = rt.intern("inForceAs");
    let mut subjects = Vec::new();
    for (name, _, _, _) in SECTIONS.iter() {
        let subject = format!("1 CFR {name}");
        rt.intern(&subject);
        subjects.push(subject);
    }
    for (edition, date) in DATES.iter().enumerate() {
        for (section, subject) in subjects.iter().enumerate() {
            let subject_id = rt.dictionary().id(subject).expect("section interned");
            let object = rt.intern(SECTIONS[section].1);
            let valid_to = if edition + 1 < DATES.len() {
                Some(DATES[edition + 1])
            } else {
                None
            };
            rt.emit(Op::Assert {
                subject: subject_id,
                relation,
                object,
                valid_from: *date,
                valid_to,
                claim: None,
                delivery_key: None,
                polarity: None,
            })
            .expect("eCFR assert admits");
        }
        rt.emit_tt_anchor(*date).expect("eCFR anchor admits");
    }
    (rt, subjects)
}

fn assert_ecfr() {
    let (rt, subjects) = ecfr_runtime();
    let binder = RuntimeBinder::new(&rt);
    for (section, subject) in subjects.iter().enumerate() {
        for cut in CUTS {
            let text = format!(
                "MATCH (s:Section {{name: \"{subject}\"}})-[:inForceAs]->(x) AS OF {cut} RETURN x LIMIT 1"
            );
            let q = parse_gpml(&text, &binder).expect("eCFR GPML parses");
            let got = execute(&rt, &q).expect("eCFR executes");
            let kutha_runtime::QueryRows::Nodes(nodes) = got else {
                panic!("eCFR must return nodes");
            };
            let expected = facade_objects(&rt, subject, "inForceAs", cut);
            assert_eq!(nodes, expected, "eCFR section={section} cut={cut}");
        }
    }
}

#[test]
fn gate_gb_ff5_b3_ecfr_parity() {
    assert_ff5();
    assert_b3();
    assert_ecfr();
}

#[test]
fn gate_gb_unknown_vocabulary_fails_closed() {
    let rt = ff5_runtime();
    let binder = RuntimeBinder::new(&rt);
    assert_eq!(
        parse_gpml(
            "MATCH (a:Statute {name: \"art-12-speed\"})-[:notAllowed]->(x) AS OF 2015 RETURN x",
            &binder,
        ),
        Err(GpmlError::UnknownRelation {
            name: "notAllowed".to_string(),
        })
    );
    assert_eq!(
        parse_gpml(
            "MATCH (a:Statute {name: \"not-interned\"})-[:inForceAs]->(x) AS OF 2015 RETURN x",
            &binder,
        ),
        Err(GpmlError::UnknownTerm {
            name: "not-interned".to_string(),
        })
    );
}
