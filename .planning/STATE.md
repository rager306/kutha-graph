---
gsd_state_version: "1.0"
milestone: v1.64
status: Awaiting next milestone
stopped_at: Phase 250 complete — all phases complete
last_updated: "2026-10-30T08:25:07.232Z"
last_activity: 2026-10-30
last_activity_desc: Milestone v1.64 completed and archived
state_head: 163163ef1f898232a0a93ce3bee272172a255c8a
progress:
  total_phases: 2
  completed_phases: 2
  total_plans: 2
  completed_plans: 2
milestone_name: ingest regression
current_phase: null
---

# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-10-29)

**Core value:** Prove legal point-in-time on a named statute-shaped fixture while keeping governor CI honest and advancing one Active Slice at a time.
**Current focus:** v1.64 ingest regression (M171-S01), complete.

## Current Position

Phase: Milestone v1.64 complete
Plan: —
Status: Awaiting next milestone
Last activity: 2026-10-30 — Milestone v1.64 completed and archived

## Accumulated Context

### Decisions

- 2026-10-29: Prime suspect is `cec5b64` / `LiveFactFilter::Product` per-fact dictionary check.
- 2026-10-30: M169 and M170 closed the `as_of` causes; M171 identified `199807d` as the ingest index commit and accepted the per-assert map write cost.
- 2026-10-30: `KUTHA_FOLD_INDEX_MODE=lazy|thin` is the bounded write-thin opt-out; Eager remains default.

### Pending Todos

None.

### Blockers/Concerns

Bisect builds must not share a target dir; use a detached worktree. The M171 five-sample table is in `docs/architecture/benchmark-baseline.md`; the earlier n=2 outlier is retained only as a caveat.

## Deferred Items

| Category | Item | Status | Deferred At | Milestone |
|----------|------|--------|-------------|-----------|
| *(none)* | | | | |

## Session Continuity

Last session: 2026-10-29
Stopped at: Phase 250 complete — all phases complete
Resume file: None

## Operator Next Steps

- Start the next milestone with /gsd-new-milestone
