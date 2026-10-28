---
phase: 130-budget-template
independent: true
status: passed
verifier_model: parent-session
commands:
  - cmd: uv run pytest scripts/tests/test_kutha_gov.py -k BudgetTemplate -q
    exit: 0
  - cmd: uv run kutha-gov precommit
    exit: 0
read_only_inputs:
  - .planning/phases/130-budget-template/130-CONTEXT.md
  - .kutha/dictionaries/checks.yaml
  - .kutha/dictionaries/fsm.yaml
  - .env.example
---

# Phase 130 — independent verification

**Verdict:** passed. Stale `.env.example` budget is HIGH; live tree green at
budget 80.
