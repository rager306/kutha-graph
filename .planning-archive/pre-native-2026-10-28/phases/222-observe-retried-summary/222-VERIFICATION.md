---
phase: 222-observe-retried-summary
independent: true
status: passed
verifier_model: parent-session
commands:
  - cmd: uv run pytest scripts/tests/test_ci_summary_labels.py -q
    exit: 0
  - cmd: uv run kutha-gov selftest --check observe-retried-summary
    exit: 0
  - cmd: uv run kutha-gov precommit --check observe-retried-summary,docs-coupling,budget-covers-checks
    exit: 0
  - cmd: uv run kutha-gov ci
    exit: 0
read_only_inputs:
  - scripts/kutha_gov/__main__.py
  - scripts/tests/test_ci_summary_labels.py
  - .kutha/dictionaries/checks.yaml
  - docs/architecture/test-matrix.yaml
---

# Phase 222 — independent verification

**Verdict:** passed. Harness line prints observe_retried=N; pytest GREEN; selftest OK; budget 101; `kutha-gov ci` run_high=0.
