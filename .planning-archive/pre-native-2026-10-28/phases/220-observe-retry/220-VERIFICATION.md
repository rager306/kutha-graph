---
phase: 220-observe-retry
independent: true
status: passed
verifier_model: parent-session
commands:
  - cmd: uv run pytest scripts/tests/test_fsm.py::ObserveRetryTests -q
    exit: 0
  - cmd: uv run kutha-gov selftest --check observe-retry-incomplete
    exit: 0
  - cmd: uv run kutha-gov precommit --check observe-retry-incomplete,docs-coupling,budget-covers-checks
    exit: 0
  - cmd: uv run kutha-gov ci
    exit: 0
read_only_inputs:
  - scripts/kutha_gov/observe.py
  - scripts/tests/test_fsm.py
  - .kutha/dictionaries/checks.yaml
  - .env.example
  - docs/architecture/test-matrix.yaml
---

# Phase 220 — independent verification

**Verdict:** passed. Default retry recovers incomplete→ok; retries=0 keeps incomplete; selftest OK; budget 99; `kutha-gov ci` run_high=0.
