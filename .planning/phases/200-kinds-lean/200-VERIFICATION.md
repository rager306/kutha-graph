---
phase: 200-kinds-lean
independent: true
status: passed
verifier_model: parent-session
commands:
  - cmd: uv run kutha-gov json --check kinds-lean
    exit: 0
  - cmd: uv run kutha-gov selftest --check kinds-lean
    exit: 0
  - cmd: uv run kutha-gov ci
    exit: 0
read_only_inputs:
  - scripts/kutha_gov/kinds.py
  - .kutha/dictionaries/checks.yaml
  - docs/architecture/test-matrix.yaml
---

# Phase 200 — independent verification

**Verdict:** passed. RED: `file_max_lines` max 2430 HIGH on kinds.py (2431 lines). GREEN: check `kinds-lean` max 2500; selftest derived-by-own-mutation OK; budget 93. `kutha-gov ci` run_high=0 (93 checks).
