---
phase: 150-check-append
independent: true
status: passed
verifier_model: parent-session
commands:
  - cmd: uv run pytest scripts/tests/test_multi_check.py scripts/tests/test_agent_surface.py -q
    exit: 0
  - cmd: uv run kutha-gov precommit --check agent-surface --check agents-lean --check test-matrix
    exit: 0
  - cmd: uv run kutha-gov ci --skip-cargo
    exit: 0
read_only_inputs:
  - .planning/phases/150-check-append/150-CONTEXT.md
  - scripts/kutha_gov/__main__.py
  - AGENTS.md
---

# Phase 150 — independent verification

**Verdict:** passed. Repeated `--check` flags select both dictionary rows;
comma form still works; ci tip ok.
