---
phase: 186-selftest-substring
independent: true
status: passed
verifier_model: parent-session
commands:
  - cmd: uv run pytest scripts/tests/test_m112_selftest_substring.py -q
    exit: 0
  - cmd: cargo test --workspace --offline --no-fail-fast
    exit: 0
  - cmd: taskset -c 0 env KUTHA_BENCH_N=10000 cargo bench -p kutha-runtime --offline --bench baseline
    exit: 0
  - cmd: uv run kutha-gov ci
    exit: 0
read_only_inputs:
  - scripts/kutha_gov/selftest.py
  - scripts/kutha_gov/kinds.py
  - scripts/tests/test_m112_selftest_substring.py
  - .kutha/dictionaries/checks.yaml
  - docs/architecture/test-matrix.yaml
---

# Phase 186 — independent verification

**Verdict:** passed. RED: substring-preserving `replace_regex` classified VACUOUS. GREEN: UNPROVEN before apply. Derived `yaml_map_list` require_fields swaps use `__selftest_rm_*__:` so the needle is not a substring. Check `selftest-substring-honesty` enrolled; budget 88. Matrix `SEM-m112-selftest-substring` killed.
