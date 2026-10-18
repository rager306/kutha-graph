---
phase: 178-op-wire-define
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
  - crates/kutha-common/src/op_migrate.rs
---

# Phase 178 — independent verification

**Verdict:** passed. Op-wire rewrite omits a second modern Define when the
name is already interned (`rewrite_skips_duplicate_modern_define`). Workspace
tests exit 0. Bench `ingest_plain.n` 96637743 ns, `as_of.n` 217945 ns.
`kutha-gov ci` tip_high=0 run_high=0.
