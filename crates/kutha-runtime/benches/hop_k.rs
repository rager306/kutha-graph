//! Self-relative 2-hop / typed-CSR bench (M157 S01 / Phase 231).
//! Duplicate parse/put/print from `baseline.rs` (no shared module).
//! Does not change the 29 `baseline.rs` keys.

use kutha_common::{Op, TermId, TransactionTime, ValidTime};
use kutha_runtime::{QueryIr, QueryRows, Runtime, execute};
use std::collections::BTreeMap;
use std::env;
use std::hint::black_box;
use std::process;
use std::time::Instant;

fn parse_n() -> u64 {
    env::var("KUTHA_HOP_N")
        .ok()
        .and_then(|s| s.parse().ok())
        .filter(|n: &u64| *n > 0)
        .unwrap_or(256)
}

fn emit_related(rt: &mut Runtime, subject: TermId, relation: TermId, object: TermId) {
    rt.emit(Op::Assert {
        subject,
        relation,
        object,
        valid_from: 2010,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .expect("relatedTo Assert must admit");
}

fn put(
    metrics: &mut BTreeMap<String, (String, String)>,
    key: &str,
    value: impl ToString,
    unit: &str,
) {
    metrics.insert(key.to_string(), (value.to_string(), unit.to_string()));
}

fn size_suffix(factor: u64) -> &'static str {
    match factor {
        1 => "n",
        2 => "2n",
        4 => "4n",
        _ => unreachable!("factors are 1, 2, 4"),
    }
}

/// Tight add loop; `black_box` keeps the work from being folded away.
fn calibrate(iters: u64) -> u128 {
    let t0 = Instant::now();
    let mut acc = 0u64;
    for i in 0..iters {
        acc = acc.wrapping_add(black_box(i));
    }
    black_box(acc);
    t0.elapsed().as_nanos()
}

/// Three layers of `n` nodes, fan-out 4, `relatedTo`. Layer-0 starts returned.
fn fill_layered(rt: &mut Runtime, n: u64) -> (TermId, Vec<TermId>) {
    let relation = rt.intern("relatedTo");
    let mut layers: [Vec<TermId>; 3] = [Vec::new(), Vec::new(), Vec::new()];
    for (layer, slot) in layers.iter_mut().enumerate() {
        slot.reserve(n as usize);
        for i in 0..n {
            slot.push(rt.intern(&format!("L{layer}_{i}")));
        }
    }
    for layer in 0..2 {
        for i in 0..n {
            let src = layers[layer][i as usize];
            for f in 0..4u64 {
                let dst = (i.wrapping_mul(4).wrapping_add(f)) % n;
                emit_related(rt, src, relation, layers[layer + 1][dst as usize]);
            }
        }
    }
    (relation, layers[0].clone())
}

fn ratio(num: u128, den: u128) -> String {
    if den == 0 {
        "inf".to_string()
    } else {
        format!("{:.6}", num as f64 / den as f64)
    }
}

fn main() {
    let n = parse_n();
    let mut metrics: BTreeMap<String, (String, String)> = BTreeMap::new();
    put(&mut metrics, "base_n", n, "1");

    // One calibration loop for all sizes so hop2.{n,2n,4n} share a denominator.
    let calib_iters = 20_000_000u64;
    let calib_ns = calibrate(calib_iters);
    put(&mut metrics, "calib", calib_ns, "ns");
    put(&mut metrics, "calib_iters", calib_iters, "1");

    let mut hop_raw = [0u128; 3];
    let mut csr_raw = [0u128; 3];
    let tt = TransactionTime::MAX;
    let vt = ValidTime::from(2010);

    for (idx, &factor) in [1u64, 2, 4].iter().enumerate() {
        let planned = n.saturating_mul(factor);
        let suffix = size_suffix(factor);
        let mut rt = Runtime::default();
        let (rel, starts) = fill_layered(&mut rt, planned);

        let t_csr = Instant::now();
        let lease = rt.typed_csr_lease_at(tt, vt);
        csr_raw[idx] = t_csr.elapsed().as_nanos();
        black_box(lease);

        let t_hop = Instant::now();
        for start in &starts {
            let q = QueryIr::Hops {
                start: *start,
                rels: vec![rel],
                k: 2,
                tt,
                vt,
                limit: 0,
            };
            match execute(&rt, &q) {
                Ok(QueryRows::Nodes(nodes)) if !nodes.is_empty() => {
                    black_box(nodes);
                }
                other => {
                    eprintln!("hop2 empty or non-Nodes for start at factor {factor}: {other:?}");
                    process::exit(1);
                }
            }
        }
        hop_raw[idx] = t_hop.elapsed().as_nanos();

        put(
            &mut metrics,
            &format!("hop2.{suffix}"),
            ratio(hop_raw[idx], calib_ns),
            "calib",
        );
        put(
            &mut metrics,
            &format!("csr_lease.{suffix}"),
            ratio(csr_raw[idx], calib_ns),
            "calib",
        );
        put(
            &mut metrics,
            &format!("hop2.{suffix}_ns"),
            hop_raw[idx],
            "ns",
        );
        put(
            &mut metrics,
            &format!("csr_lease.{suffix}_ns"),
            csr_raw[idx],
            "ns",
        );
    }

    put(
        &mut metrics,
        "hop2.2n_over_n",
        ratio(hop_raw[1], hop_raw[0]),
        "1",
    );
    put(
        &mut metrics,
        "hop2.4n_over_2n",
        ratio(hop_raw[2], hop_raw[1]),
        "1",
    );
    put(
        &mut metrics,
        "csr_lease.2n_over_n",
        ratio(csr_raw[1], csr_raw[0]),
        "1",
    );
    put(
        &mut metrics,
        "csr_lease.4n_over_2n",
        ratio(csr_raw[2], csr_raw[1]),
        "1",
    );

    let expected = [
        "hop2.n",
        "hop2.2n",
        "hop2.4n",
        "csr_lease.n",
        "csr_lease.2n",
        "csr_lease.4n",
        "hop2.2n_over_n",
        "csr_lease.2n_over_n",
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
