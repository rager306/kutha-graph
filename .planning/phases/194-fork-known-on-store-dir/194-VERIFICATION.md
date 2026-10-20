---
phase: 194-fork-known-on-store-dir
independent: true
status: passed
verifier_model: parent-session
commands:
  - cmd: cargo test -p kutha-runtime --test m016_known_on_hardening --offline -- --test-threads=8
    exit: 0
  - cmd: cargo test --workspace --offline --no-fail-fast
    exit: 0
  - cmd: cargo bench -p kutha-runtime --bench baseline --offline
    exit: 0
    note: KUTHA_BENCH_N=10000; ingest_plain.n 108170549 ns; as_of.n 246429 ns
  - cmd: uv run kutha-gov ci
    exit: 0
read_only_inputs:
  - crates/kutha-runtime/tests/m016_known_on_hardening.rs
  - .kutha/dictionaries/fsm.yaml
  - docs/architecture/test-matrix.yaml
---

# Phase 194 — independent verification

**Verdict:** passed. RED: `fork_known_on_store_dir` missing (E0425). GREEN: five `m016_known_on_hardening` tests passed, including distinct persist directories. Workspace tests exit 0. Bench recorded above; no library hot-path edit.
