---
phase: 203-yaml-needles-live
independent: true
status: passed
verifier_model: parent-session
commands:
  - cmd: uv run kutha-gov json --check yaml-needles-live
    exit: 0
  - cmd: uv run kutha-gov selftest --check yaml-needles-live
    exit: 0
  - cmd: uv run pytest scripts/tests/test_kutha_gov.py -k yaml_needles_live
    exit: 0
  - cmd: uv run kutha-gov ci
    exit: 0
read_only_inputs:
  - .kutha/dictionaries/checks.yaml
  - .kutha/dictionaries/fsm.yaml
  - scripts/kutha_gov/kinds.py
  - docs/architecture/test-matrix.yaml
---

# Phase 203 — independent verification

**Verdict:** passed. RED: first `yaml-needles-live` glob → `AGENTS.md` yields HIGH (missing `fn <name>`). GREEN: crates glob; selftest `derived-by-own-mutation` (2 steps); budget 95; `kutha-gov ci` run_high=0 (95 checks).
