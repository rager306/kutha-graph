---
phase: 231-hop-workload
plan: 01+02
status: complete
completed: 2026-10-23
---
# 231: B3-lite hop workload

No Op/wire/fingerprint. `eval.rs` / `fold_query.rs` / `leapfrog.rs` / `csr.rs` / `quantum.rs` / `fold.rs` / `baseline.rs` bodies untouched. CBM `query_ir.rs` / `hop_k.rs` `not_tracked`; `baseline.rs` `metadata_match`. No `index_repository`.

## 231-01 query IR
RED `19fb9e2`: stub `execute` → empty triples; six named tests **101**. GREEN `9b9f460` Scan + named lookup; `d1ba802` hop walk (CSR leases, PIT, shuffle). Controller amend `f453b1e`: `QueryError::RelsShape { rels, k }` (not `UnknownRelation` for a bad `rels` length); oracles `b5_hop_per_hop_rels_order` (wrong order → no rows) and `b5_hop_limit_truncates_sorted_prefix` (`limit==0` unlimited). `try_hops_named` stays re-exported from `lib.rs`. `cargo test -p kutha-runtime --offline --test b5_hop_workload` **0** (8/8).

## 231-02 hop_k + A/B
`[[bench]] hop_k` (`7c668fe`): `hop2.{n,2n,4n}` and `csr_lease.{n,2n,4n}` vs a 20e6 add-loop; also 2n/n. Default `KUTHA_HOP_N=256`. Band (4 runs, `taskset -c 0`): hop2.n/calib **1.39–2.14**; 2n/n **2.73–2.88**; 4n/2n **4.06–4.51**; csr_lease.n/calib **0.010–0.031** (noisy). hop2 has no BASE twin.

A/B vs `46b7114`, 9 alternating pairs, shared `CARGO_TARGET_DIR`, direct binaries, `KUTHA_BENCH_N=10000 taskset -c 0`. BASE worktree removed after copy (`tmp/ab/`). PRE ingest **117180497** / as_of **275864**. POST ingest **120307962** (1.027); as_of **296743** (1.076); dropped HEAD as_of pairs 3–4 (>3× median) → **265565** (0.963). Load 2.58–5.27. **PASS** ≤1.10.

## Docs
ADR-040/041 Evidence 2026-10-23 (CSR lease consumed; leapfrog still off query path; Proposed). SEM-m157-* rows. CHANGELOG Product: hop workload. README / stca-guide one line (`try_hops_named` re-export). honeycomb restage.

## Commits
1. `19fb9e2` test RED stub
2. `9b9f460` feat Scan
3. `d1ba802` feat hop walk
4. `f453b1e` fix RelsShape + oracles
5. `7c668fe` feat hop_k
6. this: docs + SUMMARY merge
