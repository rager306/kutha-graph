---
phase: 175-unindex-cause-pointer
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
  - .planning/phases/175-unindex-cause-pointer/175-CONTEXT.md
  - crates/kutha-runtime/src/pipeline.rs
  - crates/kutha-runtime/tests/m104_unindex_cause_pointer.rs
---

# Phase 175 — independent verification

**Verdict:** passed. Unindex of one Behavior examines a constant number of cause
buckets. RED was `[32, 128]`; GREEN stays bounded. Workspace tests exit 0.
Bench `ingest_plain.n` 112401628 ns, `as_of.n` 321088 ns. `kutha-gov ci`
tip_high=0.
