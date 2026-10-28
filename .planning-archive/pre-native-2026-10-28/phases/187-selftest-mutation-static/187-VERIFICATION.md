---
phase: 187-selftest-mutation-static
independent: true
status: passed
verifier_model: parent-session
commands:
  - cmd: uv run pytest scripts/tests/test_m113_selftest_mutation_static.py scripts/tests/test_m112_selftest_substring.py -q
    exit: 0
  - cmd: cargo test --workspace --offline --no-fail-fast
    exit: 0
  - cmd: taskset -c 0 env KUTHA_BENCH_N=10000 cargo bench -p kutha-runtime --offline --bench baseline
    exit: 0
  - cmd: uv run kutha-gov ci
    exit: 0
read_only_inputs:
  - scripts/kutha_gov/dictionary.py
  - scripts/kutha_gov/selftest.py
  - scripts/tests/test_m113_selftest_mutation_static.py
  - .kutha/dictionaries/checks.yaml
  - docs/architecture/test-matrix.yaml
---

# Phase 187 — independent verification

**Verdict:** passed. RED: declared `needle_REMOVED` selftest loaded without LoadError. GREEN: LoadError alongside survivors; M112 runtime UNPROVEN still holds. Matrix `SEM-m113-selftest-mutation-static` killed.
