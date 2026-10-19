---
phase: 185-hot-path-dogfood
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
  - .kutha/dictionaries/checks.yaml
  - .kutha/dictionaries/invariants.yaml
  - scripts/tests/test_m111_hot_path_dogfood.py
---

# Phase 185 — independent verification

**Verdict:** passed. RED pytest KeyError/`assertIn` before check enrollment. GREEN pytest 2 passed; unindex neighbor oracles green. Selftest `hot-path-lease-honesty` OK (mutation must not leave `tt_by_ingest` as a substring).
Workspace tests exit 0. Bench `ingest_plain.n` 145510037 ns, `as_of.n` 281605 ns.
`kutha-gov ci` tip_high=0 run_high=0 (one expected LOW: trajectory, no Active Slice). Budget 87.
