---
phase: 211-parallel-error-behavioral
independent: true
status: passed
verifier_model: parent-session
commands:
  - cmd: uv run pytest scripts/tests/test_jobs.py::JobsParityTests::test_eval_check_worker_maps_run_exception_to_check_error -q
    exit: 0
  - cmd: uv run kutha-gov precommit --check parallel-check-error-parity,docs-coupling
    exit: 0
  - cmd: uv run kutha-gov ci
    exit: 0
read_only_inputs:
  - scripts/kutha_gov/parallel.py
  - scripts/tests/test_jobs.py
  - .kutha/dictionaries/checks.yaml
  - docs/architecture/test-matrix.yaml
---

# Phase 211 — independent verification

**Verdict:** passed. RED: `return check.run(ctx)` in `_eval_check_worker` raises through the behavioral oracle. GREEN: docstring reworded without needle rot; `kutha-gov ci` run_high=0 (97 checks).
