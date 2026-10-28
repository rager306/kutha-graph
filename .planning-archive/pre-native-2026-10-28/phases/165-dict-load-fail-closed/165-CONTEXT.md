---
phase: 165-dict-load-fail-closed
---

# Phase 165 — dictionary loader fail-closed

CE whole-repo findings #6–#8: checks.yaml loader silently skips scalar rows,
overwrites duplicate ids, and runs stepless checks as vacuous OK. Thin slice:
LoadError alongside survivors (CE #20 pattern) + oracles. Honeycomb stays Proposed.
