---
phase: 139-op-wire-wal
---

# Phase 139 — Op wire WAL-only migrate

Process debt after M067: WAL-only legacy dirs still deferred. Thin slice:
`migrate_op_wire` reads framed WAL payloads as JSON Values (`recover_json_values`),
remaps via `rewrite_event_json_line`, materializes `events.jsonl`, drops WAL,
stamps `op-wire.v1`. `recover_events` fails open on CRC-good legacy meta-op
frames (no silent truncate). Segmented dirs stay deferred. Honeycomb Proposed.
