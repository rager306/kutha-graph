---
phase: 139-op-wire-wal
plan: 01
---

# Phase 139-01 — Summary

- `migrate_op_wire` rewrites WAL-only legacy dirs via `recover_json_values`.
- `recover_events` returns InvalidData on CRC-good non-Event JSON (fail-open).
- Open stays fail-open until rewrite. Segmented dirs deferred.
- Matrix `SEM-m068-op-wire-wal` killed.
