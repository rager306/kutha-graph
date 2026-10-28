---
phase: 174-define-keep-first
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
  - .planning/phases/174-define-keep-first/174-CONTEXT.md
  - crates/kutha-common/src/intern.rs
---

# Phase 174 — independent verification

**Verdict:** passed. A repeated dictionary name keeps the first TermId. RED
resolved Alice to 2; GREEN resolves it to 0. Workspace tests exit 0. Bench
`ingest_plain.n` 120671104 ns, `as_of.n` 290190 ns. `kutha-gov ci` tip_high=0.
