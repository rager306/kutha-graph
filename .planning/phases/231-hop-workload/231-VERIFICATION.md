---
phase: 231-hop-workload
independent: true
status: pass-with-findings
verifier_model: Cursor Grok 4.6
commands:
  - cmd: cargo test -p kutha-runtime --test b5_hop_workload_kill --offline
    exit: 0
  - cmd: cargo test --workspace --offline --no-fail-fast
    exit: 0
  - cmd: uv run kutha-gov ci --jobs 1
    exit: 0
read_only_inputs:
  - crates/kutha-runtime/src/query_ir.rs
  - crates/kutha-runtime/src/lib.rs
  - crates/kutha-runtime/src/csr.rs
  - crates/kutha-runtime/src/quantum.rs
  - crates/kutha-runtime/src/fold.rs
  - crates/kutha-runtime/src/fold_query.rs
  - crates/kutha-runtime/benches/hop_k.rs
  - docs/ADR/ADR-040-materialization-plugin-protocol.md
  - docs/ADR/ADR-041-csr-graphblas-hot-path.md
  - .compound-engineering/artifacts/research/2026-10-22-long-horizon-direction-roadmap.md
---

# Phase 231 — independent verification

**Verdict: PASS-WITH-FINDINGS.** Public `QueryIr`/`execute`/`try_hops_named`; `as_of`/`live_at`/CSR leases; ADR-040/041; roadmap B3. CBM: `query_ir.rs`/`hop_k.rs` `not_tracked` — source read. No `index_repository`. No `crates/*/src` edit.

## Claims

| claim | result |
|-------|--------|
| Scan bag = sorted `as_of` (tt=MAX); multiplicity kept | PASS `kill_scan_equals_sorted_as_of_bag_and_keeps_multiplicity` |
| Scan honours tt (not only as_of) | PASS `kill_scan_honours_tt_not_only_as_of` |
| Hops = sorted exact-k set; rels 0/1/k | PASS live_at oracle + `kill_per_hop_rels_shape_and_order` |
| Both clocks; `valid_to` exclusive; retract TT | PASS `kill_clocks_valid_to_exclusive_and_retract_tt` |
| Assert-order deterministic; limit prefix; 0 unlimited | PASS shuffle / limit tests |
| fail-closed unknown; no intern/Define/log/fold growth | PASS `kill_fail_closed_unknown_names_no_intern` |
| k==0 `{start}`; no panic; Horn iff live; CI; dead | PASS k0 / weird / horn / CI / multiplicity |
| Seeded graphs vs independent `live_at` BFS | PASS `kill_property_random_graphs_vs_live_at_oracle` |

## Findings

1. **MED** (report-only): `execute(Hops)` rebuilds `typed_csr_lease_at`/`csr_lease_at` every call. `hop_k` times that per layer-0 start → hop2 = interpreter + n×CSR, not the walk. Fan-out 4 (walk O(1)/start); cost ≈ starts×edges = Θ(n²). hop2 4n/2n ~4.1–4.5 is that square, not quadratic BFS. Proposal: `execute_with_lease(rt, q, &TypedCsrLease)` and/or cache key `(tt,vt)`; bench one lease then walk. Calib is honest (`black_box` wrapping_add), not walk-shaped.
2. **LOW**: typed CSR keeps Fact multiplicity; IR returns a **set** of ends. Matches the leased set contract and ADR-040 untyped neighbor-set spike. Follow-up if MATCH/path-count (ADR-041 F9) is leased.
3. **LOW**: unlabeled hops use deduping `CsrLease`; typed keeps parallel edges then set-collapses. Oracle agrees.

## Commands

- `cargo test -p kutha-runtime --test b5_hop_workload_kill --offline` **0** (14/14)
- `cargo test --workspace --offline --no-fail-fast` **0** (`m019_dur_a` did not flake)
- `uv run kutha-gov ci --jobs 1` **0**, `run_high=0`

No `tmp/` scratch left.
