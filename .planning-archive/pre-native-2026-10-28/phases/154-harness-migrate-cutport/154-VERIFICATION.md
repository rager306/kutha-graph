---
phase: 154-harness-migrate-cutport
independent: true
status: passed
verifier_model: parent-session
commands:
  - cmd: uv run pytest scripts/tests/test_agent_surface.py -q
    exit: 0
  - cmd: uv run kutha-gov precommit --check dogfood --check agent-surface --check agents-lean --check test-matrix --check docs-coupling
    exit: 0
  - cmd: uv run kutha-gov ci --skip-cargo
    exit: 0
read_only_inputs:
  - .planning/phases/154-harness-migrate-cutport/154-CONTEXT.md
  - docs/process/kutha-harness.md
  - .kutha/dictionaries/checks.yaml
  - .kutha/dictionaries/invariants.yaml
---

# Phase 154 — independent verification

**Verdict:** passed. Harness CutPort names migrate-bin; dogfood and
agent-surface green; ci tip_high=0 with cargo skipped.
