---
phase: 156-json-check-append
---

# Phase 156 — json --check append oracle

After M079, repeated `--check` works for precommit, but `kutha-gov json`
had no oracle that append selects both rows (same `_join_check_flags`).
Thin slice: pytest + agent-surface. Honeycomb stays Proposed.
