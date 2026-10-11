---
phase: 113-cascade-atomicity
independent: true
verifier_model: parent-session-fresh-pass
commands:
  - cmd: cargo test -p kutha-runtime --test m052_cascade_atomic --offline
    exit: 0
  - cmd: cargo test --workspace --offline --no-fail-fast
    exit: 0
  - cmd: export KUTHA_GOV_BUDGET=76; uv run kutha-gov ci
    exit: 0
    note: 0 HIGH / 1 LOW / 76 checks on a clean HEAD worktree (tmp/bench-{pre,post}-65.txt and -66.txt present; no egg-info). Dirty main checkout may HIGH on controller-uncommitted STATE/docs-coupling.
  - cmd: KUTHA_BENCH_N=10000 taskset -c 0 cargo bench -p kutha-runtime --bench baseline -- --quick
    exit: 0
    note: three quiet runs; median ingest_plain.n ≈111.4M as_of.n ≈294.1k
read_only_inputs:
  - .planning/phases/113-cascade-atomicity/113-CONTEXT.md
  - crates/kutha-runtime/src/pipeline.rs
  - crates/kutha-runtime/tests/m052_cascade_atomic.rs
---

# Phase 113 — independent verification

**Verdict:** passed. Later-hop `UnknownRelation` leaves log/fold/outcomes unchanged; inverse_knows still persists `QuantumOutcome`. Matrix `SEM-m052-cascade-atomic` killed; budget 76; workspace cargo + clean-tree `kutha-gov ci` 0 HIGH / 1 LOW / 76 checks.

## Kill claims checked

1. `ghostRel` Horn on `knownBy` → `emit(Assert knows)` returns `UnknownRelation` and does not grow log, fold facts, or outcomes.
2. Genesis inverse_knows cascade still writes `QuantumOutcome` and a live `knownBy` fact.
