# Phase 98 CONTEXT — policy delivery-key namespace

**Source:** CE full-branch review P2 #29.  
**Lease:** M038 S01 / GSD v0.32.

## Goal

`by_delivery_key` retry is a global last-wins map on the delivery-key string alone. After `record_justification` pins an admission Assert with `delivery_key = policy_version`, an identical `emit_policy` re-pin resolves to the admission row, compares across relations, and returns `DeliveryKeyConflict` instead of idempotent retry. Namespace the ingest lease by `(relation, delivery_key)` so policy / admission / action / product keys do not collide. Keep admission `delivery_key` as the policy pin (ADR-050/051).

## Oracles

- `emit_policy_repin_after_justification_is_idempotent`
- `emit_policy_repin_without_justification_still_idempotent`
- `same_delivery_key_different_relation_do_not_conflict`
