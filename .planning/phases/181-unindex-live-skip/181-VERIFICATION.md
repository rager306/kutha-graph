---
phase: 181-unindex-live-skip
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
  - crates/kutha-runtime/src/pipeline.rs
  - crates/kutha-runtime/tests/m107_unindex_live_scan.rs
---

# Phase 181 — independent verification

**Verdict:** passed. RED examine counts were `[32, 128]`. GREEN is `[0, 0]`.
Workspace tests exit 0. Bench `ingest_plain.n` 87971452 ns, `as_of.n` 240138 ns.
`kutha-gov ci` tip_high=0 run_high=0.
