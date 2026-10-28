---
phase: 231-hop-workload
plan: 01+02
status: complete
completed: 2026-10-23
---
# 231: B3-lite hop workload

No Op/wire/fingerprint. `eval.rs` / `fold_query.rs` / `leapfrog.rs` / `csr.rs` / `quantum.rs` / `fold.rs` / `baseline.rs` bodies untouched. CBM `query_ir.rs` / `hop_k.rs` `not_tracked`; `baseline.rs` `metadata_match`. No `index_repository`.

## 231-01 query IR
RED `19fb9e2`: stub `execute` → empty triples; six named tests **101**. GREEN `9b9f460` Scan + named lookup; `d1ba802` hop walk (CSR leases, PIT, shuffle). Controller amend `f453b1e`: `QueryError::RelsShape { rels, k }` (not `UnknownRelation` for a bad `rels` length); oracles `b5_hop_per_hop_rels_order` (wrong order → no rows) and `b5_hop_limit_truncates_sorted_prefix` (`limit==0` unlimited). `try_hops_named` stays re-exported from `lib.rs`. Fix round: `prepare` snapshots Scan/CSR at `(tt, vt)`; `Prepared::run` walks; `execute` = prepare+run. `PreparedHops::run_from` reuses one lease for many starts. Cut mismatch unrepresentable (`run` takes no `QueryIr`). Oracles `b5_hop_prepare_run_equals_execute`, `b5_hop_prepared_cut_is_snapshot`. `b5_hop_workload` **0** (10/10); kill **14/14**.

## 231-02 hop_k + A/B
`[[bench]] hop_k` (`7c668fe`): lease once outside the timed walk. Metrics `hop2.walk.{n,2n,4n}` (walk only) and `csr_lease.{n,2n,4n}`. Default `KUTHA_HOP_N=256`. Old `hop2.*` (interpreter+n×CSR, 4n/2n **4.06–4.51**) superseded. Band (4 runs, `taskset -c 0`, load 16–19 after 15 min wait on foreign cargo): hop2.walk.n/calib **0.005–0.013**; 2n/n **1.56–3.01**; 4n/2n **1.36–2.56** (expect ~2); csr_lease.n/calib **0.010–0.020**. hop2 has no BASE twin. A/B vs `46b7114` unchanged (hot-path files untouched).

A/B vs `46b7114`, 9 alternating pairs, shared `CARGO_TARGET_DIR`, direct binaries, `KUTHA_BENCH_N=10000 taskset -c 0`. BASE worktree removed after copy (`tmp/ab/`). PRE ingest **117180497** / as_of **275864**. POST ingest **120307962** (1.027); as_of **296743** (1.076); dropped HEAD as_of pairs 3–4 (>3× median) → **265565** (0.963). Load 2.58–5.27. **PASS** ≤1.10.

## Docs
ADR-040/041 Evidence 2026-10-23 (CSR lease consumed; leapfrog still off query path; Proposed). SEM-m157-* rows including prepare/run + snapshot. CHANGELOG Product: hop workload. honeycomb restage.

## Flake
`m019_dur_a::persist_migrate_race_*` WouldBlock: `remove_dir_all` while `Persister` held `kutha.lock`, plus process-global `set_test_segment_limits` racing lock oracles. Test-only: `PersistScratch` (Persister drops before dir Drop) + file-local `Mutex` on lock oracles / tiny_limits. 5× default threads + 1× `--test-threads=1` exit **0**.

## Commits
1. `19fb9e2` test RED stub
2. `9b9f460` feat Scan
3. `d1ba802` feat hop walk
4. `f453b1e` fix RelsShape + oracles
5. `7c668fe` feat hop_k
6. docs + SUMMARY merge
7. this: prepare/run + hop_k walk band + m019 isolation
