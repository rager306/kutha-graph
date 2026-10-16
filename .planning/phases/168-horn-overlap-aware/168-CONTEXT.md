---
phase: 168-horn-overlap-aware
---

# Phase 168 — Horn overlap-aware premise discovery

CE whole-repo finding #4: multi-premise Horn discovery point-gated at trigger
`valid_from` missed later-starting overlapping premises (order-dependent).

**Design (controller):** overlap-aware candidates whose VT overlaps the trigger
window + existing `conjunct_vt`, not retry-at-candidate-starts. Bounds residual
single-combination-DFS separately. Honeycomb stays Proposed.
