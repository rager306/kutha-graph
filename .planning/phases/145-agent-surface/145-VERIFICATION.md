---
phase: 145-agent-surface
independent: true
status: passed
verifier_model: parent-session
commands:
  - cmd: uv run pytest scripts/tests/test_agent_surface.py -q
    exit: 0
  - cmd: uv run kutha-gov precommit
    exit: 0
  - cmd: uv run kutha-gov ci
    exit: 0
read_only_inputs:
  - .planning/phases/145-agent-surface/145-CONTEXT.md
  - .kutha/META.md
  - .kutha/dictionaries/checks.yaml
  - docs/process/gsd-auto-contract.md
  - AGENTS.md
---

# Phase 145 — independent verification

**Verdict:** passed. CBM column is `kutha-graph`; AGENTS lists `--jobs` / `--skip-cargo` / `kutha-gov json` / `--check`; check `agent-surface` green at budget 84.
