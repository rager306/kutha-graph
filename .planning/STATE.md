---
gsd_state_version: "1.0"
milestone: v1.65
milestone_name: B3 hot-path benchmark
status: planning
last_updated: "2026-10-30T08:40:07.360Z"
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
**Current focus:** v1.65 B3 hot-path benchmark (M172-S01), phase 251.

## Current Position

Phase: 251 — B3 workload and baseline
Plan: 251-01
Status: Executing
Last activity: 2026-10-30 — Milestone v1.65 started; B3 workload and baseline planned


## Accumulated Context

### Decisions

- 2026-10-29: Prime suspect is `cec5b64` / `LiveFactFilter::Product` per-fact dictionary check.
- 2026-10-30: M169 and M170 closed the `as_of` causes; M171 identified `199807d` as the ingest index commit and accepted the per-assert map write cost.
- 2026-10-30: `KUTHA_FOLD_INDEX_MODE=lazy|thin` is the bounded write-thin opt-out; Eager remains default.
- 2026-10-30: B3 is the next gate; HNSW/planner/Cypher stay frozen until B3 shows a real gap or the no-gap decision skips B4/B5.

### Pending Todos

None.

### Blockers/Concerns

B3 competitor environment may be unavailable; record the exact blocker rather than starting HNSW/planner/Cypher. Bisect builds must not share a target dir; use a detached worktree. The M171 five-sample table is in `docs/architecture/benchmark-baseline.md`; the earlier n=2 outlier is retained only as a caveat.

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
