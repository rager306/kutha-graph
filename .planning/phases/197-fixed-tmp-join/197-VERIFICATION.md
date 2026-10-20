---
phase: 197-fixed-tmp-join
independent: true
status: passed
verifier_model: parent-session
commands:
  - cmd: uv run pytest scripts/tests/test_kutha_gov.py -k GlobRegexAbsent -q
    exit: 0
  - cmd: uv run kutha-gov json --check fixed-tmp-persist-dir
    exit: 0
  - cmd: uv run kutha-gov selftest --check fixed-tmp-persist-dir
    exit: 0
  - cmd: cargo test --workspace --offline --no-fail-fast
    exit: 0
  - cmd: cargo bench -p kutha-runtime --bench baseline --offline
    exit: 0
    note: KUTHA_BENCH_N=10000; ingest_plain.n 154187259 ns; as_of.n 486800 ns
  - cmd: uv run kutha-gov ci
    exit: 0
read_only_inputs:
  - crates/kutha-runtime/tests/common/sem/persistence.rs
  - scripts/kutha_gov/kinds.py
  - .kutha/dictionaries/checks.yaml
  - docs/architecture/test-matrix.yaml
---

# Phase 197 — independent verification

**Verdict:** passed. RED: `fixed-tmp-persist-dir` HIGH on sem `p62-sem` `.join("tmp").join("…")`. GREEN: kind `glob_regex_absent`; leaf folded into `format!("p62-sem-…")`; check exit 0; selftest explicit-mutation OK (2 units). Workspace tests exit 0. Bench recorded above. `kutha-gov ci` run_high=0.
