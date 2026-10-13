---
phase: 140-op-wire-seg
plan: 01
---

# Phase 140-01 — Summary

- `migrate_op_wire` rewrites segmented dirs via `segment::recover_json_values`.
- Demotes to legacy jsonl SoT; drops segments + manifest; stamps `op-wire.v1`.
- Open stays fail-open on sealed legacy meta-op frames.
- Matrix `SEM-m069-op-wire-seg` killed.
