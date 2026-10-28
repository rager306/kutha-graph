---
phase: 125-delivery-key-same-relation
plan: 01
---

# Phase 125 Plan 01 — Summary

## What shipped

- `has_live_keyed_support(claim, relation, key)` — relation-scoped liveness.
- `delivery_key_retry` passes the Assert relation into the check.
- Oracle `keyed_retry_after_retract_does_not_borrow_sibling_relation_live_key`.
- Matrix `SEM-m056-delivery-key-same-relation` killed; SEM-keyed-retry-live updated.

## Residuals

- #28 → M056-S04; #35/#36 → M056-S05. Honeycomb Proposed.
