---
phase: 223-observe-partial-incomplete
independent: true
status: passed
verifier_model: parent-session
commands:
  - cmd: uv run pytest scripts/tests/test_fsm.py -q -k 'cargo_partial or cargo_nonzero_keeps or cargo_zero_exit or cargo_incomplete or partial_incomplete'
    exit: 0
  - cmd: uv run kutha-gov selftest --check observe-partial-incomplete,observe-incomplete-honesty
    exit: 0
  - cmd: uv run kutha-gov precommit --check observe-partial-incomplete,docs-coupling,budget-covers-checks,budget-template-align
    exit: 0
  - cmd: uv run kutha-gov ci --jobs 1
    exit: 0
read_only_inputs:
  - scripts/kutha_gov/observe.py
  - scripts/tests/test_fsm.py
  - .kutha/dictionaries/checks.yaml
  - docs/architecture/test-matrix.yaml
---

# Phase 223 — independent verification

**Verdict:** passed. Partial ok + non-zero cargo exit collapses to observe-incomplete and retries; real FAILED stays per-name; budget 102; `kutha-gov ci --jobs 1` run_high=0.
