//! M013 S01 / Phase 22: std-only baseline bench (`harness = false`).
//! No criterion. Measures relatedTo (no inverse cascade). Validates answers.

use kutha_common::{EventId, Op, TermId};
use kutha_runtime::{store, FoldIndexMode, Runtime};
use std::collections::BTreeMap;
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
    // D-05: KUTHA_BENCH_SABOTAGE=drop_one must exit non-zero (live_count != planned).
    env::var("KUTHA_BENCH_SABOTAGE").ok().as_deref() == Some("drop_one")
}

fn emit_related(
    rt: &mut Runtime,
    subject: TermId,
    relation: TermId,
    object: TermId,
    delivery_key: Option<String>,
) -> EventId {
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

fn vm_rss_bytes() -> Option<u64> {
    let status = std::fs::read_to_string("/proc/self/status").ok()?;
    for line in status.lines() {
        if let Some(rest) = line.strip_prefix("VmRSS:") {
            let kb: u64 = rest.split_whitespace().next()?.parse().ok()?;
            return Some(kb.saturating_mul(1024));
        }
    }
    None
}

fn scratch_dir(tag: &str) -> PathBuf {
    let pid = process::id();
    // Resolve to repo-root tmp/ even when cargo runs with package cwd.
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("tmp");
    let _ = std::fs::create_dir_all(&root);
    let dir = root.join(format!("kutha-bench-{tag}-{pid}"));
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

fn size_suffix(factor: u64) -> &'static str {
    match factor {
        1 => "n",
        2 => "2n",
        4 => "4n",
        _ => unreachable!("factors are 1, 2, 4"),
    }
}

fn fill_plain(rt: &mut Runtime, planned: u64, drop_last: bool) -> (EventId, EventId, u64) {
    let relation = rt.intern("relatedTo");
    let object = rt.intern("o");
    let mut first = None;
    let mut last = None;
    let mut emitted = 0u64;
    for i in 0..planned {
        if drop_last && i + 1 == planned {
            break;
        }
        let s = rt.intern(&format!("s{i}"));
        let id = emit_related(rt, s, relation, object, None);
        if first.is_none() {
            first = Some(id);
        }
        last = Some(id);
        emitted += 1;
    }
    (
        first.expect("at least one assert when planned > 0"),
        last.expect("at least one assert when planned > 0"),
        emitted,
    )
}

fn fill_keyed(rt: &mut Runtime, planned: u64) -> EventId {
    let relation = rt.intern("relatedTo");
    let object = rt.intern("o");
    let mut first = None;
    for i in 0..planned {
        let s = rt.intern(&format!("ks{i}"));
        let id = emit_related(rt, s, relation, object, Some(format!("dk{i}")));
        if first.is_none() {
            first = Some(id);
        }
    }
    first.expect("planned > 0")
}

fn put(
    metrics: &mut BTreeMap<String, (String, String)>,
    key: &str,
    value: impl ToString,
    unit: &str,
) {
    metrics.insert(key.to_string(), (value.to_string(), unit.to_string()));
}

fn main() {
    let n = parse_n();
    let drop_one = sabotage_drop_one();
    let index_mode = FoldIndexMode::from_env();
    let mut metrics: BTreeMap<String, (String, String)> = BTreeMap::new();
    put(&mut metrics, "base_n", n, "1");
    put(
        &mut metrics,
        "fold_index_mode",
        match index_mode {
            FoldIndexMode::Eager => "eager",
            FoldIndexMode::Lazy => "lazy",
        },
        "1",
    );

    let rss_before = vm_rss_bytes();

    for &factor in &[1u64, 2, 4] {
        let planned = n.saturating_mul(factor);
        let suffix = size_suffix(factor);

        // Plain ingest + validation + as_of + persist/open/replay + late retract.
        let mut rt = Runtime::with_index_mode(32, index_mode);
        // Buffer hint: ~Define+Assert+QuantumOutcome per relatedTo (+ headroom).
        rt.reserve_ingest_indexes(
            planned.saturating_mul(4).saturating_add(16) as usize,
            planned.saturating_add(8) as usize,
        );
        let t0 = Instant::now();
        let (first_id, last_id, emitted) = fill_plain(&mut rt, planned, drop_one && factor == 1);
        let ingest_ns = t0.elapsed().as_nanos();
        put(
            &mut metrics,
            &format!("ingest_plain.{suffix}"),
            ingest_ns,
            "ns",
        );

        // Lazy may leave leases cold; warm before timed as_of so modes compare fairly on lookup.
        if index_mode == FoldIndexMode::Lazy {
            rt.ensure_fold_indexes();
        }
        let live = rt.fold().live_count(u64::MAX, 2010) as u64;
        if live != planned {
            eprintln!(
                "plain live_count {live} != planned {planned} (emitted {emitted}, factor {factor})"
            );
            process::exit(1);
        }

        let t1 = Instant::now();
        let _ = rt.fold().as_of(2010);
        put(
            &mut metrics,
            &format!("as_of.{suffix}"),
            t1.elapsed().as_nanos(),
            "ns",
        );

        let t_claim = Instant::now();
        let supported = rt.fold().claim_supported_at(first_id, u64::MAX, 2010);
        put(
            &mut metrics,
            &format!("claim_supported_at.{suffix}"),
            t_claim.elapsed().as_nanos(),
            "ns",
        );
        if !supported {
            eprintln!("claim_supported_at false for first plain assert");
            process::exit(1);
        }

        let dir = scratch_dir(&format!("plain-{suffix}"));
        let t2 = Instant::now();
        store::persist(&rt, &dir).expect("persist");
        put(
            &mut metrics,
            &format!("persist.{suffix}"),
            t2.elapsed().as_nanos(),
            "ns",
        );

        let mut append_extra = 0u64;
        if factor == 1 {
            append_extra = (n / 100).max(1);
            let mut persister = store::Persister::attach(&rt, &dir).expect("persister attach");
            let relation = rt.intern("relatedTo");
            let object = rt.intern("o");
            for i in planned..planned + append_extra {
                let s = rt.intern(&format!("sapp{i}"));
                emit_related(&mut rt, s, relation, object, None);
            }
            let t_ap = Instant::now();
            persister.append(&rt).expect("persist append");
            put(
                &mut metrics,
                "persist_append.n",
                t_ap.elapsed().as_nanos(),
                "ns",
            );
        }

        let t3 = Instant::now();
        let opened = store::open(&dir).expect("open");
        put(
            &mut metrics,
            &format!("open.{suffix}"),
            t3.elapsed().as_nanos(),
            "ns",
        );

        if opened.fold().fingerprint() != rt.fold().fingerprint() {
            eprintln!("fingerprint mismatch after open ({suffix})");
            process::exit(1);
        }

        let t4 = Instant::now();
        opened.replay_check().expect("replay_check");
        put(
            &mut metrics,
            &format!("replay_check.{suffix}"),
            t4.elapsed().as_nanos(),
            "ns",
        );

        if factor == 1 {
            let events = rt.log().len() as f64;
            let epa = events / (n as f64);
            put(&mut metrics, "events_per_assert", format!("{epa:.6}"), "1");
        }

        let t5 = Instant::now();
        rt.emit(Op::Retract { event_id: last_id })
            .expect("late retract");
        put(
            &mut metrics,
            &format!("retract_late.{suffix}"),
            t5.elapsed().as_nanos(),
            "ns",
        );
        let after = rt.fold().live_count(u64::MAX, 2010) as u64;
        if after != planned - 1 + append_extra {
            eprintln!(
                "after retract live_count {after} != {}",
                planned - 1 + append_extra
            );
            process::exit(1);
        }

        // Keyed ingest on a fresh runtime (unretracted graph for this measurement).
        let mut keyed = Runtime::with_index_mode(32, index_mode);
        keyed.reserve_ingest_indexes(
            planned.saturating_mul(4).saturating_add(16) as usize,
            planned.saturating_add(8) as usize,
        );
        let tk = Instant::now();
        let keyed_first = fill_keyed(&mut keyed, planned);
        put(
            &mut metrics,
            &format!("ingest_keyed.{suffix}"),
            tk.elapsed().as_nanos(),
            "ns",
        );
        if index_mode == FoldIndexMode::Lazy {
            keyed.ensure_fold_indexes();
        }
        let keyed_live = keyed.fold().live_count(u64::MAX, 2010) as u64;
        if keyed_live != planned {
            eprintln!("keyed live_count {keyed_live} != planned {planned}");
            process::exit(1);
        }
        if !keyed.fold().claim_supported_at(keyed_first, u64::MAX, 2010) {
            eprintln!("keyed claim_supported_at false");
            process::exit(1);
        }
    }

    match (rss_before, vm_rss_bytes()) {
        (Some(before), Some(after)) if after >= before => {
            let delta = after - before;
            // Approximate: last graph left retracted; use N*4 planned live peak ≈ factor sum.
            // Prefer bytes / live_count of a dedicated N-sized unretracted graph.
            let mut sample = Runtime::with_index_mode(32, index_mode);
            let _ = fill_plain(&mut sample, n, false);
            let live = sample.fold().live_count(u64::MAX, 2010) as u64;
            let bpf = if live > 0 { delta / live.max(1) } else { 0 };
            put(&mut metrics, "bytes_per_fact", bpf, "B");
        }
        _ => put(&mut metrics, "bytes_per_fact", "n/a", "n/a"),
    }

    let expected: [&str; 29] = [
        "as_of.2n",
        "as_of.4n",
        "as_of.n",
        "base_n",
        "bytes_per_fact",
        "claim_supported_at.2n",
        "claim_supported_at.4n",
        "claim_supported_at.n",
        "events_per_assert",
        "fold_index_mode",
        "ingest_keyed.2n",
        "ingest_keyed.4n",
        "ingest_keyed.n",
        "ingest_plain.2n",
        "ingest_plain.4n",
        "ingest_plain.n",
        "open.2n",
        "open.4n",
        "open.n",
        "persist.2n",
        "persist.4n",
        "persist.n",
        "persist_append.n",
        "replay_check.2n",
        "replay_check.4n",
        "replay_check.n",
        "retract_late.2n",
        "retract_late.4n",
        "retract_late.n",
    ];
    for key in expected {
        if !metrics.contains_key(key) {
            eprintln!("missing metric key {key}");
            process::exit(1);
        }
    }

    for (key, (value, unit)) in &metrics {
        println!("{key} {value} {unit}");
    }
    println!("validated yes");
}
