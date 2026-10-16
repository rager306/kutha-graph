---
phase: 167-bench-rss-before-sample
---

# Phase 167 — bench rss_before at dedicated sample

CE whole-repo finding #2: `rss_before` preceded the factor loops while
`rss_after` / denominator used only the dedicated sample. Thin slice: move
`rss_before` to immediately before sample alloc + structural oracle.
Honeycomb stays Proposed.
