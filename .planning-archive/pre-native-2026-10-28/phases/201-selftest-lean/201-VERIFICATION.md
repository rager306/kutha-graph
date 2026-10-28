---
phase: 201-selftest-lean
independent: true
status: passed
verifier_model: parent-session
commands:
  - cmd: uv run kutha-gov json --check selftest-lean
    exit: 0
  - cmd: uv run kutha-gov selftest --check selftest-lean
    exit: 0
  - cmd: uv run kutha-gov ci
    exit: 0
read_only_inputs:
  - scripts/kutha_gov/selftest.py
  - .kutha/dictionaries/checks.yaml
  - docs/architecture/test-matrix.yaml
---

# Phase 201 — independent verification

**Verdict:** passed. RED: `file_max_lines` max 950 HIGH on selftest.py (951 lines). GREEN: check `selftest-lean` max 1000; selftest derived-by-own-mutation OK; budget 94. `kutha-gov ci` run_high=0 (94 checks).
