---
phase: 135-op-migrate-policy
plan: 01
---

# Phase 135-01 — Summary

- `pin_policy_to_ops` remaps PinPolicy to StoreBlob + kutha:policy Assert.
- Open/serde stay fail-open. RecordAdmission/RecordAction deferred.
- Matrix `SEM-m064-op-migrate-policy` killed.
