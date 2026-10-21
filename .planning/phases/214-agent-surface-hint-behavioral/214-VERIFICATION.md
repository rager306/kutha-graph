---
phase: 214-agent-surface-hint-behavioral
independent: true
status: passed
verifier_model: parent-session
commands:
  - cmd: uv run pytest scripts/tests/test_agent_surface.py scripts/tests/test_multi_check.py::MultiCheckSelectTests::test_ci_rejects_check_flag scripts/tests/test_multi_check.py::MultiCheckSelectTests::test_non_ci_rejects_skip_cargo_flag -q
    exit: 0
  - cmd: uv run kutha-gov selftest --check agent-surface
    exit: 0
  - cmd: uv run kutha-gov precommit --check agent-surface,docs-coupling
    exit: 0
  - cmd: uv run kutha-gov ci
    exit: 0
read_only_inputs:
  - scripts/kutha_gov/__main__.py
  - scripts/tests/test_agent_surface.py
  - scripts/tests/test_multi_check.py
  - .kutha/dictionaries/checks.yaml
  - docs/architecture/test-matrix.yaml
---

# Phase 214 — independent verification

**Verdict:** passed. Legacy hint needles HIGH on hint reword; M140 check stays OK on reword; CLI reject pytest still GREEN (surface redirect, not exact `hint:` template); selftest OK; `kutha-gov ci` run_high=0 (97 checks).
