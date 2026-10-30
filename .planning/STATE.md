---
gsd_state_version: "1.0"
milestone: v1.64
milestone_name: ingest regression
status: planning
last_updated: "2026-10-30T02:43:12.269Z"
last_activity: 2026-10-30
progress:
  total_phases: 2
  completed_phases: 0
  total_plans: 2
  completed_plans: 0
  percent: 0
---

# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-10-29)

**Core value:** Prove legal point-in-time on a named statute-shaped fixture while keeping governor CI honest and advancing one Active Slice at a time.
**Current focus:** v1.64 ingest regression (M171-S01), phase 249.

## Current Position

Phase: 249 — Ingest bisect
Plan: 249-01
Status: Executing
Last activity: 2026-10-30 — Milestone v1.64 started; phase 249 planned


## Accumulated Context

### Decisions

- 2026-10-29: Prime suspect is `cec5b64` / `LiveFactFilter::Product` per-fact dictionary check.
- 2026-10-30: M169 and M170 closed the `as_of` causes; the remaining M168 residual is `ingest_plain.n` +12.6% versus `cdf1942`.

### Pending Todos

None.

### Blockers/Concerns

Bisect builds must not share a target dir; use a detached worktree.

## Deferred Items

| Category | Item | Status | Deferred At | Milestone |
|----------|------|--------|-------------|-----------|
| *(none)* | | | | |

## Session Continuity

Last session: 2026-10-29
Stopped at: Phase 248 complete — all phases complete
Resume file: None

## Operator Next Steps

- Start the next milestone with /gsd-new-milestone
