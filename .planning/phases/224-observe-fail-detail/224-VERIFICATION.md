---
phase: 224-observe-fail-detail
independent: true
status: passed
verifier_model: parent-session
commands:
  - cmd: uv run pytest scripts/tests/test_fsm.py -q -k 'ObserveFailDetail'
    exit: 0
  - cmd: uv run kutha-gov selftest --check observe-fail-detail
    exit: 0
  - cmd: uv run kutha-gov precommit --check observe-fail-detail,docs-coupling,budget-covers-checks,budget-template-align
    exit: 0
  - cmd: uv run kutha-gov ci --jobs 1
    exit: 0
read_only_inputs:
  - scripts/kutha_gov/observe.py
  - scripts/tests/test_fsm.py
  - .kutha/dictionaries/checks.yaml
  - docs/architecture/test-matrix.yaml
---

# Phase 224 — independent verification

**Verdict:** passed. observe-fail includes elapsed_ms and truncated output tail; budget 103; `kutha-gov ci --jobs 1` run_high=0.
