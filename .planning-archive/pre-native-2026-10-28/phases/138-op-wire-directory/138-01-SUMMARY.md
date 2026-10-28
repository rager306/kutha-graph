---
phase: 138-op-wire-directory
plan: 01
---

# Phase 138-01 — Summary

- `migrate_op_wire` rewrites legacy jsonl and stamps `op-wire.v1`.
- Open stays fail-open. Segmented/WAL-only deferred.
- Matrix `SEM-m067-op-wire-dir` killed.
