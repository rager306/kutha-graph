---
phase: 218-git-auto-base-behavioral
independent: true
status: passed
verifier_model: parent-session
commands:
  - cmd: uv run pytest scripts/tests/test_ci_summary_labels.py scripts/tests/test_gitdiff_auto.py -q
    exit: 0
  - cmd: uv run kutha-gov selftest --check git-auto-base
    exit: 0
  - cmd: uv run kutha-gov precommit --check git-auto-base,docs-coupling
    exit: 0
  - cmd: uv run kutha-gov ci
    exit: 0
read_only_inputs:
  - scripts/kutha_gov/gitdiff.py
  - scripts/kutha_gov/__main__.py
  - scripts/tests/test_ci_summary_labels.py
  - scripts/tests/test_gitdiff_auto.py
  - .kutha/dictionaries/checks.yaml
  - docs/architecture/test-matrix.yaml
---

# Phase 218 — independent verification

**Verdict:** passed. Legacy `run_high=` / `tip_high=` file_contains needles HIGH on label reword (substring also matches `this_run_high=`); M144 check stays OK on label reword; merge-base wiring + pytest GREEN; selftest OK; `kutha-gov ci` run_high=0 (97 checks).
