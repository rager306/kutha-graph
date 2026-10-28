---
phase: 172-horn-pin-overlap
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
  - .planning/phases/172-horn-pin-overlap/172-CONTEXT.md
  - crates/kutha-runtime/src/eval.rs
  - crates/kutha-runtime/tests/m101_horn_pin_overlap.rs
---

# Phase 172 — independent verification

**Verdict:** passed. A rule pin that starts after the trigger still fires on the
intersection. RED was no derived head; GREEN is valid-time `[2015, 2020)`.
Workspace tests exit 0. Bench `ingest_plain.n` 118123444 ns, `as_of.n` 301913 ns.
`kutha-gov ci` tip_high=0.
