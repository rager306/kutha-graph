---
phase: 132-env-budget-floor
---

# Phase 132 — Context

M060 residual: live `.env` `KUTHA_GOV_BUDGET` below `fsm.yaml` defaults.budget truncated only on `ci`. Precommit runs the full dictionary, so stale env was invisible there.

Generic kind `env_int_at_least` (meta-prompt FSM): floor from YAML int or `{path, pattern}` capture; unset env passes; set-and-short → HIGH.
