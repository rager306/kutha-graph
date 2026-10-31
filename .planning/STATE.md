---
gsd_state_version: "1.0"
milestone: v1.80
status: In progress
stopped_at: Phases 275-276 planned
last_updated: "2026-10-31T23:59:00.000Z"
last_activity: 2026-10-31
last_activity_desc: Milestone v1.80 started under the M187 lease
state_head: d3a0798
progress:
  total_phases: 2
  completed_phases: 0
  total_plans: 2
  completed_plans: 0
milestone_name: Test honesty
current_phase: 275
---

# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-10-31)

**Core value:** Prove legal point-in-time on a named statute-shaped fixture while keeping governor CI honest and advancing one Active Slice at a time.
**Current focus:** v1.80 test honesty (M187-S01), phases 275-276.

## Current Position

Phase: 275 (planned) and 276 (planned, file-disjoint; both wave 1)
Plan: 275-01, 276-01
Status: Ready to execute
Last activity: 2026-10-31 — M187 leased, milestone v1.80 opened

## Accumulated Context

### Decisions

- 2026-10-31: M187 test honesty leased after the audit wave. Five defects: v1.73 vacuous enrichment tests; missing pack-level replay tests; v1.74 eCFR assert; fixed relative `tmp/` dirs; wall-clock scaling oracles. Plus the `QueryRejected` taxonomy (`gaps_found` in 257/258).
- 2026-10-31: Test-only milestone; one named error-taxonomy change in `security.rs`. No new `Op`, no log/graph change, no thaw.
- 2026-10-31: ADR-094 records the shipped two-crate workspace as a legal P0 variant (ADR-020 allows it); no refactor implied.
- 2026-10-31: Honeycomb capability backfill promoted ten cells to `named` on existing test evidence; `map` stays Proposed.
- 2026-10-31: M186 A/B2 medians corrected from the raw logs; gate metrics inside 10%, `as_of.2n`/`ingest_keyed.n` recorded as secondary.
- 2026-10-30: B3 showed no gap; B4/B5 skipped; GPML read-only parser leased (M003a).
- 2026-10-30: `KUTHA_FOLD_INDEX_MODE=lazy|thin` is the bounded write-thin opt-out; Eager remains default.

### Blockers

- None for v1.80. C2 dictionaries (M188/v1.81) needs its own lease after this one closes.

## Session Continuity

- Governor: `uv run kutha-gov py` is green; `precommit` green.
- Workspace at last full run: 832 passed / 0 failed (before v1.80; re-record at close).
- Harness pytest at last full run: 296 passed / 2 skipped.
