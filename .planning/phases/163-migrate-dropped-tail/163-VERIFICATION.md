---
phase: 163-migrate-dropped-tail
independent: true
status: passed
verifier_model: parent-session
commands:
  - cmd: cargo test -p kutha-runtime --test m092_migrate_dropped_tail -- --nocapture
    exit: 0
  - cmd: cargo test --workspace --offline --no-fail-fast
    exit: 0
  - cmd: taskset -c 0 env KUTHA_BENCH_N=10000 cargo bench -p kutha-runtime --bench baseline
    exit: 0
  - cmd: uv run kutha-gov precommit --check test-matrix --check docs-coupling --check changelog-planes --check cite-lease --check state-readme
    exit: 0
  - cmd: uv run kutha-gov ci
    exit: 0
read_only_inputs:
  - .planning/phases/163-migrate-dropped-tail/163-CONTEXT.md
  - crates/kutha-runtime/src/store.rs
  - crates/kutha-runtime/src/wal.rs
  - crates/kutha-runtime/src/segment.rs
  - crates/kutha-runtime/tests/m092_migrate_dropped_tail.rs
  - docs/architecture/test-matrix.yaml
---

# Phase 163 — independent verification

**Verdict:** passed. `dropped_tail` counted on torn jsonl for Op/Pin migrates;
workspace green; quiet bench `ingest_plain.n` ≈107.2M ns / `as_of.n` ≈300.3k ns
(`KUTHA_BENCH_N=10000`, `taskset -c 0`); full `ci` tip_high=0 after close.
