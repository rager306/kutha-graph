---
phase: 122-append-tip-rollback
plan: 01
---

# Phase 122 Plan 01 — Summary

## What shipped

- Post-write `Other`: restore durable memory cursor; leave crash-shaped suffix; flag `uncommitted_suffix`.
- Next `append` rewinds that suffix before collect/write (no duplicate EventIds).
- Committed rollover failure → fail-closed poison.
- Oracles: `m055_append_tip_rollback` (2). Matrix `SEM-gap-persist-poison` killed.
- m030 oracle updated for WalSync-visible tip + unique EventIds.

## Residuals

- Honeycomb Proposed.
- Advisory CE residuals (#26…) still unleased.
