---
phase: 199-yaml-needles-derive
independent: true
status: passed
verifier_model: parent-session
commands:
  - cmd: uv run pytest scripts/tests/test_selftest.py -k 'yaml_needles_in_glob_derives or allowed_kinds_are_derivable' -q
    exit: 0
  - cmd: uv run kutha-gov json --check yaml-needles-derive
    exit: 0
  - cmd: uv run kutha-gov selftest --check yaml-needles-derive
    exit: 0
  - cmd: uv run kutha-gov ci
    exit: 0
read_only_inputs:
  - scripts/kutha_gov/selftest.py
  - scripts/kutha_gov/kinds.py
  - scripts/tests/test_selftest.py
  - .kutha/dictionaries/checks.yaml
---

# Phase 199 — independent verification

**Verdict:** passed. RED: synthetic `yaml_needles_in_glob`-only check UNPROVEN; ALLOWED−DERIVABLE included `yaml_needles_in_glob`. GREEN: mutation target + DERIVABLE; pytest OK; check+selftest OK; budget 92. `kutha-gov ci` run_high=0 (92 checks).
