---
phase: 191-quantum-known-on-store-dir
independent: true
status: passed
verifier_model: parent-session
commands:
  - cmd: cargo test -p kutha-runtime --lib --offline quantum_known_on -- --test-threads=8
    exit: 0
  - cmd: cargo test --workspace --offline --no-fail-fast
    exit: 0
  - cmd: cargo bench -p kutha-runtime --bench baseline --offline
    exit: 0
    note: KUTHA_BENCH_N=10000; ingest_plain.n 113396450 ns; as_of.n 267298 ns
  - cmd: uv run kutha-gov ci
    exit: 0
read_only_inputs:
  - crates/kutha-runtime/src/quantum_invariants.rs
  - .kutha/dictionaries/fsm.yaml
  - docs/architecture/test-matrix.yaml
---

# Phase 191 — independent verification

**Verdict:** passed. RED: `quantum_known_on_store_dir` missing (E0425). GREEN: distinct persist directories; `as_known_on_survives_persist_open_and_matches_fold` green. Workspace tests exit 0. Bench recorded above; no library hot-path edit.
