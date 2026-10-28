---
phase: 134-op-migrate-rule
plan: 01
---

# Phase 134-01 — Summary

- `remap_legacy_ops` remaps RegisterRule to StoreBlob + kutha:rule Assert.
- Open/serde stay fail-open. PinPolicy+ deferred.
- Matrix `SEM-m063-op-migrate-rule` killed.
