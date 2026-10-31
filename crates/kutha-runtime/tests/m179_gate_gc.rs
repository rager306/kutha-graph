//! Gate GC: a toy pack installs, answers GPML AS OF, retires, and replays
//! with zero core edits (ADR-021 D021-1 / direction proposal gate GC).
//!
//! The pack declares one operator and carries no code. Product facts are
//! written by the host, not by the pack. The GPML leg is deliberately
//! pack-independent: it proves that an installed or retired pack does not
//! disturb queries, **not** that pack state is on the query path. Gate GC's
//! "queries via Cypher AS OF" is therefore evidenced as neutrality, and the
//! pack surface itself is exercised only through lifecycle, manifest, and
//! registry.

use kutha_common::Op;
use kutha_runtime::{
    PackLifecycleState, PackManifest, QueryRows, Runtime, RuntimeBinder, execute, parse_gpml,
};
use std::sync::atomic::{AtomicU64, Ordering};

mod common;

/// Repo-local `tmp/` scratch with pid + nanos + seq (`m019_dur_a` idiom), so two
/// concurrent runs of this binary never share a store directory.
fn scratch(label: &str) -> common::PersistScratch {
    static N: AtomicU64 = AtomicU64::new(0);
    let seq = N.fetch_add(1, Ordering::Relaxed);
    common::PersistScratch::new(&format!("m179-{label}-{seq}"))
}

const T_OLD: u64 = 2015;
const T_NEW: u64 = 2020;

fn toy_pack(rt: &mut Runtime) {
    let manifest = PackManifest::new("toy", "Toy Pack", "1.0.0", vec!["normalize".into()]);
    rt.emit_pack_install_with_manifest("toy", "Toy Pack", "1.0.0", Some(manifest))
        .expect("install toy pack");
    rt.emit_pack_activate("toy", "1.0.0").expect("activate");
}

fn statute_facts(rt: &mut Runtime) {
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
    .expect("old limit");
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
    .expect("new limit");
}

fn as_of(rt: &Runtime, year: u64) -> Vec<kutha_common::TermId> {
    let binder = RuntimeBinder::new(rt);
    let query = format!(
        "MATCH (a:Statute {{name: \"art-12-speed\"}})-[:inForceAs]->(x) AS OF {year} RETURN x"
    );
    let ir = parse_gpml(&query, &binder).expect("GPML parses");
    match execute(rt, &ir).expect("execute") {
        QueryRows::Nodes(nodes) => nodes,
        QueryRows::Triples(_) => panic!("hops return nodes"),
    }
}

#[test]
fn toy_pack_installs_queries_retires_and_replays_without_core_edits() {
    let mut rt = Runtime::default();
    let facts_before_pack = rt.fold().facts().len();
    toy_pack(&mut rt);
    assert_eq!(rt.fold().facts().len(), facts_before_pack);
    assert_eq!(rt.pack_lifecycle()[0].state, PackLifecycleState::Active);
    assert_eq!(rt.pack_operators().expect("registry").len(), 1);

    statute_facts(&mut rt);
    let fingerprint = rt.fold().fingerprint();

    let old = as_of(&rt, T_OLD);
    let new = as_of(&rt, T_NEW);
    assert_eq!(old, vec![rt.dictionary().id("50-kmh").expect("interned")]);
    assert_eq!(new, vec![rt.dictionary().id("30-kmh").expect("interned")]);
    assert_ne!(old, new, "AS OF cuts must differ");

    rt.emit_pack_retire("toy", "1.0.0").expect("retire");
    assert_eq!(rt.pack_lifecycle()[0].state, PackLifecycleState::Retired);
    assert!(
        rt.pack_operators()
            .expect("registry")
            .iter()
            .all(|row| row.state == PackLifecycleState::Retired)
    );
    assert_eq!(
        rt.fold().fingerprint(),
        fingerprint,
        "retire is graph-neutral"
    );
    assert_eq!(as_of(&rt, T_OLD), old, "retire does not change AS OF");
    assert_eq!(as_of(&rt, T_NEW), new);

    let dir = scratch("gate-gc");
    rt.persist(&dir).expect("persist");
    let opened = Runtime::open(&dir).expect("open");
    assert_eq!(opened.fold().facts().len(), rt.fold().facts().len());
    assert_eq!(opened.fold().fingerprint(), fingerprint);
    assert_eq!(opened.pack_lifecycle(), rt.pack_lifecycle());
    assert_eq!(
        opened.pack_operators().expect("registry"),
        rt.pack_operators().expect("registry")
    );
    assert_eq!(as_of(&opened, T_OLD), old);
    assert_eq!(as_of(&opened, T_NEW), new);
}

#[test]
fn pack_retire_then_replay_is_parity_equal_to_reopen() {
    let mut rt = Runtime::default();
    toy_pack(&mut rt);
    statute_facts(&mut rt);
    let active = rt.pack_lifecycle();
    rt.emit_pack_retire("toy", "1.0.0").expect("retire");
    let retired = rt.pack_lifecycle();

    let dir = scratch("gate-gc-replay");
    rt.persist(&dir).expect("persist");
    let opened = Runtime::open(&dir).expect("open");
    assert_eq!(opened.pack_lifecycle(), retired);
    assert_ne!(opened.pack_lifecycle(), active);
    assert_eq!(opened.fold().fingerprint(), rt.fold().fingerprint());
    assert_eq!(opened.fold().facts(), rt.fold().facts());
}
