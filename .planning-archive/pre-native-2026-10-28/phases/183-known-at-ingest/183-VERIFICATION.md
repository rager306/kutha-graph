---
phase: 183-known-at-ingest
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
  - crates/kutha-runtime/src/fold.rs
  - crates/kutha-runtime/src/fold_query.rs
  - crates/kutha-runtime/tests/m109_known_at_ingest_index.rs
---

# Phase 183 — independent verification

**Verdict:** passed. RED examine counts were `[74, 266]`. GREEN stays under the early-cohort bound and does not grow with late decoys.
Workspace tests exit 0. Bench `ingest_plain.n` 171143640 ns, `as_of.n` 413771 ns.
`kutha-gov ci` tip_high=0 run_high=0 (one expected LOW: trajectory, no Active Slice).
