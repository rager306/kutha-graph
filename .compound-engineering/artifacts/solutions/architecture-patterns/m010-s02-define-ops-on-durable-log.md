---
module: kutha-runtime
tags: [m010, define, intern, store]
problem_type: capability-gap
date: 2026-09-15
---

# Persist term strings as Define events, keep snapshot log_offset honest

## Problem

S01 recovered meanings from `terms.jsonl`. Dropping that sidecar still failed. Putting `Op::Define` into the same `Vec<Event>` as the snapshot tail would double-apply graph events because `Snapshot.log_offset` counts the in-memory graph log, not the durable prefix.

## Pattern

On persist, prefix durable WAL/JSONL with `Op::Define` in `dictionary().strings()` order, then graph events. On open: if snapshot exists, strip Define before `from_snapshot`. If not, rebuild the dict from Define names. `terms.jsonl` stays a derived picture.

## Not this

Do not auto-append Define from `Runtime::intern` (inflates live `log.len()`). Not Rocks. Not ADR Accepted.
