---
phase: 180-open-index-lease
independent: true
status: passed
verifier_model: parent-session
commands:
  - cmd: cargo test --workspace --offline --no-fail-fast
    exit: 0
  - cmd: taskset -c 0 env KUTHA_BENCH_N=10000 cargo bench -p kutha-runtime --offline --bench baseline
    exit: 0
  - cmd: uv run kutha-gov ci
    exit: 0
read_only_inputs:
  - crates/kutha-runtime/src/store.rs
  - crates/kutha-runtime/src/fold.rs
  - crates/kutha-runtime/tests/m106_open_index_lease.rs
---

# Phase 180 — independent verification

**Verdict:** passed. RED was examine 28 vs 48 (mode flag without lease
warmth). GREEN matches the pre-persist scan. Workspace tests exit 0.
Bench `ingest_plain.n` 112513507 ns, `as_of.n` 284374 ns. `kutha-gov ci`
tip_high=0 run_high=0.
