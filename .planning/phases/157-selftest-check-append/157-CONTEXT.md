---
phase: 157-selftest-check-append
---

# Phase 157 — selftest --check append oracle

After M085, json/precommit have repeated `--check` oracles; `selftest`
shares `_join_check_flags` but had no multi-select append pin. Thin slice:
pytest + agent-surface. Honeycomb stays Proposed.
