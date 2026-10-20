---
phase: 198-glob-regex-derive
independent: true
status: passed
verifier_model: parent-session
commands:
  - cmd: uv run pytest scripts/tests/test_selftest.py -k test_synthetic_glob_regex_absent_derives -q
    exit: 0
  - cmd: uv run kutha-gov json --check glob-regex-absent-derive
    exit: 0
  - cmd: uv run kutha-gov selftest --check glob-regex-absent-derive
    exit: 0
  - cmd: uv run kutha-gov ci
    exit: 0
read_only_inputs:
  - scripts/kutha_gov/selftest.py
  - scripts/kutha_gov/kinds.py
  - scripts/tests/test_selftest.py
  - .kutha/dictionaries/checks.yaml
---

# Phase 198 — independent verification

**Verdict:** passed. RED: synthetic check with only `glob_regex_absent` was UNPROVEN. GREEN: DERIVABLE + `glob_regex_absent_mutation_target`; pytest OK; check+selftest OK; budget 91. `kutha-gov ci` run_high=0 (91 checks).
