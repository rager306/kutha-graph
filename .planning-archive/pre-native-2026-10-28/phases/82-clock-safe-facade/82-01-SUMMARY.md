# Phase 82-01 — Clock-safe facade summary

**Slice:** M025 S02 / GSD Phase 82  
**Status:** implemented

## Delivered

- `Runtime` / `GraphFold` `as_of` → `ValidTime`; `as_known_on` → `WallTime`.
- Call sites wrapped with `ValidTime::from` / `WallTime::from`.
- trybuild: `ui_clock_facade` + four `tests/ui/*` cases.
- ADR-013 Clarification Phase 82; honeycomb evidence.

## Deferred

- `norm_view` (S03), demo bin (S04); `live_at`/`known_at` still `impl Into`.
