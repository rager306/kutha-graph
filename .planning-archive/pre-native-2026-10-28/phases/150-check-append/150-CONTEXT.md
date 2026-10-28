---
phase: 150-check-append
---

# Phase 150 — repeatable --check flags

After M077, comma-lists work but repeating `--check a --check b` keeps only
the last argparse value. Thin slice: `action=append` then `_join_check_flags`
into existing `_parse_check_ids`. Honeycomb stays Proposed.
