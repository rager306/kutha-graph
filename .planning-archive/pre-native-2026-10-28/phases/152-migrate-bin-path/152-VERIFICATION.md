---
phase: 152-migrate-bin-path
independent: true
status: passed
verifier_model: parent-session
commands:
  - cmd: uv run pytest scripts/tests/test_agent_surface.py -q
    exit: 0
  - cmd: uv run kutha-gov precommit --check agent-surface --check agents-lean --check test-matrix --check docs-coupling
    exit: 0
  - cmd: uv run kutha-gov ci --skip-cargo
    exit: 0
read_only_inputs:
  - .planning/phases/152-migrate-bin-path/152-CONTEXT.md
  - AGENTS.md
  - .kutha/META.md
  - .kutha/dictionaries/checks.yaml
  - .kutha/dictionaries/invariants.yaml
---

# Phase 152 — independent verification

**Verdict:** passed. AGENTS prefers built `debug/kutha-migrate`; agent-surface
and docs-coupling green; ci tip_high=0 with cargo skipped.
