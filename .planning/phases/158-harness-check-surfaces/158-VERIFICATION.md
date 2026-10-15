---
phase: 158-harness-check-surfaces
independent: true
status: passed
verifier_model: parent-session
commands:
  - cmd: uv run pytest scripts/tests/test_agent_surface.py -q --cache-clear
    exit: 0
  - cmd: uv run kutha-gov precommit --check dogfood --check agent-surface --check test-matrix --check docs-coupling
    exit: 0
  - cmd: uv run kutha-gov ci --skip-cargo
    exit: 0
read_only_inputs:
  - .planning/phases/158-harness-check-surfaces/158-CONTEXT.md
  - docs/process/kutha-harness.md
  - scripts/tests/test_agent_surface.py
  - .kutha/dictionaries/checks.yaml
---

# Phase 158 — independent verification

**Verdict:** passed. Harness prose names json/precommit/selftest `--check`;
dogfood + agent-surface green; ci tip_high=0 with cargo skipped after close
CHANGELOG lands with the process-plane diff.
