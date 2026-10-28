---
phase: 159-agents-check-not-ci
independent: true
status: passed
verifier_model: parent-session
commands:
  - cmd: uv run pytest scripts/tests/test_agent_surface.py scripts/tests/test_multi_check.py -q --cache-clear
    exit: 0
  - cmd: uv run kutha-gov precommit --check agent-surface --check test-matrix --check docs-coupling --check changelog-planes --check cite-lease --check state-readme
    exit: 0
  - cmd: uv run kutha-gov ci --skip-cargo
    exit: 0
read_only_inputs:
  - .planning/phases/159-agents-check-not-ci/159-CONTEXT.md
  - AGENTS.md
  - scripts/tests/test_agent_surface.py
  - scripts/tests/test_multi_check.py
  - .kutha/dictionaries/checks.yaml
---

# Phase 159 — independent verification

**Verdict:** passed. AGENTS Flags exclude `--check` from `ci`;
agent-surface + CLI reject oracle green; `ci --skip-cargo` tip_high=0 after close
CHANGELOG lands with the process-plane diff.
