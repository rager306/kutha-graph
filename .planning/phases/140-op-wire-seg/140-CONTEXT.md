---
phase: 140-op-wire-seg
---

# Phase 140 — Op wire segmented migrate

Process debt after M068: segmented dirs still rejected. Thin slice:
`migrate_op_wire` recovers sealed+active KUTHASEG payloads as JSON Values,
remaps via `rewrite_event_json_line`, materializes `events.jsonl`, drops
segments + manifest, stamps `op-wire.v1`. Open stays fail-open on sealed
legacy meta-op frames. Honeycomb Proposed.
