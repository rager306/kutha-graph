---
phase: 205-yaml-map-list-nonempty
independent: true
status: passed
verifier_model: parent-session
commands:
  - cmd: uv run pytest scripts/tests/test_kutha_gov.py -k rejects_empty_select
    exit: 0
  - cmd: uv run kutha-gov json --check yaml-map-list-nonempty
    exit: 0
  - cmd: uv run kutha-gov selftest --check yaml-map-list-nonempty
    exit: 0
  - cmd: uv run kutha-gov ci
    exit: 0
read_only_inputs:
  - scripts/kutha_gov/kinds.py
  - scripts/tests/test_kutha_gov.py
  - .kutha/dictionaries/checks.yaml
  - docs/architecture/test-matrix.yaml
---

# Phase 205 — independent verification

**Verdict:** passed. RED: empty `items: []` vacuous high=0. GREEN: `or not rows` guard → HIGH `not a non-empty list`; check `yaml-map-list-nonempty`; budget 97; `kutha-gov ci` run_high=0 (97 checks).
