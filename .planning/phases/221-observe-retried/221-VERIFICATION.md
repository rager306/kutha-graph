---
phase: 221-observe-retried
independent: true
status: passed
verifier_model: parent-session
commands:
  - cmd: uv run pytest scripts/tests/test_fsm.py::ObserveRetriedTests scripts/tests/test_fsm.py::ObserveRetryTests -q
    exit: 0
  - cmd: uv run kutha-gov selftest --check observe-retried-breadcrumb,observe-incomplete-honesty
    exit: 0
  - cmd: uv run kutha-gov precommit --check observe-retried-breadcrumb,docs-coupling,budget-covers-checks
    exit: 0
  - cmd: uv run kutha-gov ci
    exit: 0
read_only_inputs:
  - scripts/kutha_gov/observe.py
  - scripts/kutha_gov/fsm.py
  - scripts/tests/test_fsm.py
  - .kutha/dictionaries/checks.yaml
  - docs/architecture/test-matrix.yaml
---

# Phase 221 — independent verification

**Verdict:** passed. LOW finding keeps cargo=ok; incomplete→ok emits observe-retried; selftest OK; budget 100; `kutha-gov ci` run_high=0.
