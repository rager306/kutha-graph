---
phase: 171-correct-unindex-behavior
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
  - .planning/phases/171-correct-unindex-behavior/171-CONTEXT.md
  - crates/kutha-runtime/src/pipeline.rs
  - crates/kutha-runtime/tests/m100_correct_unindex_behavior.rs
---

# Phase 171 — independent verification

**Verdict:** passed. Correct and CorrectInterval of a Behavior unindex it.
Oracles examined 0 after the fix (RED was 2). Workspace tests exit 0
(one parallel `kutha.lock` WouldBlock on `mixed_typed_dict_partial_group_is_visible`
passed alone and on the rerun). Bench `ingest_plain.n` 126935335 ns,
`as_of.n` 283091 ns. `kutha-gov ci` tip_high=0.
