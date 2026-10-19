---
phase: 190-known-on-store-dir
independent: true
status: passed
verifier_model: parent-session
commands:
  - cmd: cargo test -p kutha-runtime --test m014_as_known_on --offline -- --test-threads=8
    exit: 0
  - cmd: cargo test --workspace --offline --no-fail-fast
    exit: 0
  - cmd: cargo bench -p kutha-runtime --bench baseline --offline
    exit: 0
    note: KUTHA_BENCH_N=10000; ingest_plain.n 107588756 ns; as_of.n 285763 ns
  - cmd: uv run kutha-gov ci
    exit: 0
read_only_inputs:
  - crates/kutha-runtime/tests/m014_as_known_on.rs
  - .kutha/dictionaries/fsm.yaml
  - docs/architecture/test-matrix.yaml
---

# Phase 190 — independent verification

**Verdict:** passed. RED: `known_on_store_dir` was missing (compile error E0425). GREEN: five `m014_as_known_on` tests passed, including distinct persist directories. Workspace tests exit 0. Bench recorded above; no library hot-path edit.
