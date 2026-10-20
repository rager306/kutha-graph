---
phase: 196-fixed-tmp-persist-dir
independent: true
status: passed
verifier_model: parent-session
commands:
  - cmd: uv run kutha-gov json --check fixed-tmp-persist-dir
    exit: 0
  - cmd: uv run kutha-gov selftest --check fixed-tmp-persist-dir
    exit: 0
  - cmd: cargo test -p kutha-runtime --test m026_generators --offline -- --test-threads=8
    exit: 0
  - cmd: cargo test --workspace --offline --no-fail-fast
    exit: 0
  - cmd: cargo bench -p kutha-runtime --bench baseline --offline
    exit: 0
    note: KUTHA_BENCH_N=10000; ingest_plain.n 123995257 ns; as_of.n 319631 ns
  - cmd: uv run kutha-gov ci
    exit: 0
read_only_inputs:
  - crates/kutha-runtime/examples/statute_as_of.rs
  - crates/kutha-runtime/tests/m026_generators.rs
  - .kutha/dictionaries/checks.yaml
  - docs/architecture/test-matrix.yaml
---

# Phase 196 — independent verification

**Verdict:** passed. RED: `fixed-tmp-persist-dir` HIGH on example + m026 `Path::new("tmp/…)`. GREEN: check exit 0; selftest explicit-mutation OK; m026 nine tests green. Workspace tests exit 0. Bench recorded above.
