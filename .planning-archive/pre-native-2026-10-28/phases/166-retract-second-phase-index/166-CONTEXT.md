---
phase: 166-retract-second-phase-index
---

# Phase 166 — retract second-phase premise-relation index

CE whole-repo finding #3: after the indexed direct-children phase, every
retract/correct still walks the full `live_behaviors` vector. Thin slice: index
live Behaviors by Horn body premise relation and examine only candidates.
Honeycomb stays Proposed.
