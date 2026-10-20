---
phase: 195-facade-statute-store-dir
independent: true
status: passed
verifier_model: parent-session
commands:
  - cmd: cargo test -p kutha-runtime --test facade_statute --offline -- --test-threads=8
    exit: 0
  - cmd: cargo test --workspace --offline --no-fail-fast
    exit: 0
  - cmd: cargo bench -p kutha-runtime --bench baseline --offline
    exit: 0
    note: KUTHA_BENCH_N=10000; ingest_plain.n 123783243 ns; as_of.n 263320 ns
  - cmd: uv run kutha-gov ci
    exit: 0
read_only_inputs:
  - crates/kutha-runtime/tests/facade_statute.rs
  - .kutha/dictionaries/fsm.yaml
  - docs/architecture/test-matrix.yaml
---

# Phase 195 — independent verification

**Verdict:** passed. RED: `facade_statute_store_dir` missing (E0425). GREEN: two `facade_statute` tests passed, including distinct persist directories. Workspace tests exit 0. Bench recorded above; no library hot-path edit.
