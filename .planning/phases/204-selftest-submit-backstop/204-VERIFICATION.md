---
phase: 204-selftest-submit-backstop
independent: true
status: passed
verifier_model: parent-session
commands:
  - cmd: uv run pytest scripts/tests/test_selftest.py -k parallel_submit_failure
    exit: 0
  - cmd: uv run kutha-gov json --check selftest-submit-backstop
    exit: 0
  - cmd: uv run kutha-gov selftest --check selftest-submit-backstop
    exit: 0
  - cmd: uv run kutha-gov ci
    exit: 0
read_only_inputs:
  - scripts/kutha_gov/selftest.py
  - scripts/tests/test_selftest.py
  - .kutha/dictionaries/checks.yaml
  - docs/architecture/test-matrix.yaml
---

# Phase 204 — independent verification

**Verdict:** passed. RED: dict-comprehension submit → `RuntimeError: submit-after-first-success`. GREEN: submit one-by-one + harvest + leftover serial; pytest both ids OK; check `selftest-submit-backstop`; budget 96; `selftest.py` 981 ≤1000; `kutha-gov ci` run_high=0 (96 checks).
