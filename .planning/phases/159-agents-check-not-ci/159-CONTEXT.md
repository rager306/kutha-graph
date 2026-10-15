---
phase: 159-agents-check-not-ci
---

# Phase 159 — AGENTS --check not for ci

After M087, CLI rejects `ci --check` and harness names three surfaces, but
AGENTS Flags still groups `ci` with `--check` ("where noted"), so agents try
selective checks on the full FSM. Thin slice: AGENTS wording + agent-surface +
CLI reject oracle. Honeycomb stays Proposed.
