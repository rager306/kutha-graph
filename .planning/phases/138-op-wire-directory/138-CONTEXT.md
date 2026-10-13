---
phase: 138-op-wire-directory
---

# Phase 138 — Context

Five meta-op remappers exist. Operators need a directory rewrite: `store::migrate_op_wire` reads legacy `events.jsonl` line-wise, remaps via `rewrite_event_json_line`, drops stale WAL/snapshot, stamps `op-wire.v1`. Open stays fail-open. Segmented and WAL-only dirs deferred.
