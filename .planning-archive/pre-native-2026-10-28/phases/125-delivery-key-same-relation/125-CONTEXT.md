---
phase: 125-delivery-key-same-relation
---

# Phase 125 — Context

CE #26: `delivery_key_retry` looks up by `(relation, key)` but checked liveness
via claim-scoped `has_live_keyed_support`. Two allowed relations sharing
claim+key let a live A short-circuit a retracted B. Fix: pass relation into
the liveness check. Reuse `has_live_keyed_support` + `m022` Correct lineage
oracles. Honeycomb stays Proposed.
