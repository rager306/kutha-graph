# Phase 173 Context — migrate verify drops segments

**Milestone:** v0.96 (M102-S01)
**Status:** ready (discuss skipped — lease names the slice)

## Decision

Failed `migrate_verify_reopen` already deletes `manifest.json` and `sync_dir`s. The generation-unique `seg-*.log` files from that `persist_log` stay on disk. After the manifest is gone, delete every `seg-*.log` (none are referenced) and then `sync_dir` once. Legacy `events.jsonl` and `events.wal` stay.

## Oracle

`migrate_with_verify_fault(FlipSealedByte)` leaves no `seg-*.log`, no manifest, and `open` still reads the legacy log.

## Out of scope

Single-combination DFS, unindex reverse pointer, policy-pin election, honeycomb thaw.
