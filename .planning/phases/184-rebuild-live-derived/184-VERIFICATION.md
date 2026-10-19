---
phase: 184-rebuild-live-derived
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
  - crates/kutha-runtime/tests/m110_rebuild_live_derived.rs
---

# Phase 184 — independent verification

**Verdict:** passed. RED retract examine after persist+open was `1`. GREEN is `0`.
Workspace tests exit 0. Bench `ingest_plain.n` 127651002 ns, `as_of.n` 294209 ns.
`kutha-gov ci` tip_high=0 run_high=0 (one expected LOW: trajectory, no Active Slice).
