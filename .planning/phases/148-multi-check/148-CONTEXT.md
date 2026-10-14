---
phase: 148-multi-check
---

# Phase 148 — multi-check CLI select

After M076 retro: `--check` took one id; repeating the flag kept only the last
(argparse). Agents dropped intended oracles. Thin slice: comma-separated
dictionary lookup via `_parse_check_ids` / `_select_checks` (no hardcoded check
names). Honeycomb stays Proposed.
