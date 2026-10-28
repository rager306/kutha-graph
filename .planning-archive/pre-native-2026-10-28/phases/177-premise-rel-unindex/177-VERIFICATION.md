---
phase: 177-premise-rel-unindex
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
  - crates/kutha-runtime/tests/m105_audit_residuals.rs
---

# Phase 177 — independent verification

**Verdict:** passed. Unindex of one Behavior examines only that behavior's
premise relations. A non-behavior id does not scan premise buckets
(`unindex_premise_examines_only_that_behaviors_rels`). Workspace tests exit 0.
Bench `ingest_plain.n` 96637743 ns, `as_of.n` 217945 ns. `kutha-gov ci`
tip_high=0 run_high=0.
