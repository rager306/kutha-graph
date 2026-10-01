//! M013 S01 / Phase 22: std-only baseline bench (`harness = false`).
//! No criterion. Measures relatedTo (no inverse cascade). Validates answers.

use kutha_common::Op;
use kutha_runtime::{store, Runtime};
use std::env;
use std::path::PathBuf;
use std::process;
use std::time::Instant;

fn parse_n() -> u64 {
    env::var("KUTHA_BENCH_N")
        .ok()
        .and_then(|s| s.parse().ok())
        .filter(|n: &u64| *n > 0)
        .unwrap_or(10_000)
}

fn sabotage_drop_one() -> bool {
    env::var("KUTHA_BENCH_SABOTAGE").ok().as_deref() == Some("drop_one")
}

fn emit_related(
    rt: &mut Runtime,
    subject: kutha_common::TermId,
    relation: kutha_common::TermId,
    object: kutha_common::TermId,
    delivery_key: Option<String>,
) -> kutha_common::EventId {
    rt.emit(Op::Assert {
        subject,
        relation,
        object,
        valid_from: 2010,
        valid_to: None,
        claim: None,
        delivery_key,
        polarity: None,
    })
    .expect("relatedTo Assert must admit")
    .receipt
    .event_ids[0]
}

fn main() {
    let n = parse_n();
    let drop_one = sabotage_drop_one();
    let planned = n;

    let mut rt = Runtime::default();
    let relation = rt.intern("relatedTo");
    let object = rt.intern("o");

    let t0 = Instant::now();
    let mut first_id = None;
    let mut emitted = 0u64;
    for i in 0..planned {
        if drop_one && i + 1 == planned {
            break;
        }
        let s = rt.intern(&format!("s{i}"));
        let id = emit_related(&mut rt, s, relation, object, None);
        if first_id.is_none() {
            first_id = Some(id);
        }
        emitted += 1;
    }
    let ingest_ns = t0.elapsed().as_nanos();

    let live = rt.fold().live_count(u64::MAX, 2010) as u64;
    if live != planned {
        eprintln!("live_count {live} != planned {planned} (emitted {emitted})");
        process::exit(1);
    }

    let t1 = Instant::now();
    let _triples = rt.fold().as_of(2010);
    let as_of_ns = t1.elapsed().as_nanos();

    let dir = scratch_dir("plain");
    let t2 = Instant::now();
    store::persist(&rt, &dir).expect("persist");
    let persist_ns = t2.elapsed().as_nanos();

    let t3 = Instant::now();
    let opened = store::open(&dir).expect("open");
    let open_ns = t3.elapsed().as_nanos();

    if opened.fold().fingerprint() != rt.fold().fingerprint() {
        eprintln!("fingerprint mismatch after open");
        process::exit(1);
    }

    let t4 = Instant::now();
    opened.replay_check().expect("replay_check");
    let replay_ns = t4.elapsed().as_nanos();

    let mut lines = vec![
        format!("as_of.n {as_of_ns} ns"),
        format!("base_n {n} 1"),
        format!("ingest_plain.n {ingest_ns} ns"),
        format!("open.n {open_ns} ns"),
        format!("persist.n {persist_ns} ns"),
        format!("replay_check.n {replay_ns} ns"),
    ];
    lines.sort();
    for line in &lines {
        println!("{line}");
    }
    println!("validated yes");
    let _ = first_id;
}

fn scratch_dir(tag: &str) -> PathBuf {
    let pid = process::id();
    let dir = PathBuf::from(format!("tmp/kutha-bench-{tag}-{pid}"));
    let _ = std::fs::remove_dir_all(&dir);
    dir
}
