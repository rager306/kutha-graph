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
Status: Blocked — measurement evidence is not yet sufficient
Last activity: 2026-10-30 — bisect candidate 199807d; independent verifier BLOCKED


## Accumulated Context

### Decisions

- 2026-10-29: Prime suspect is `cec5b64` / `LiveFactFilter::Product` per-fact dictionary check.
- 2026-10-30: M169 and M170 closed the `as_of` causes; the remaining M168 residual is `ingest_plain.n` +12.6% versus `cdf1942`.

### Pending Todos

None.

### Blockers/Concerns

- Phase 249 BLOCKED: bisect candidate `199807d` is the first index commit, but the confirming bench is n=2 with a 1.68x outlier; no `249-SUMMARY`/baseline artifact yet. Need >=5 interleaved N=10000 samples per commit (`cdf1942`, `c26bf35`, `199807d`, `929bbc6`) on an idle host, with both min and median; then record the table and CHANGELOG. 199807d's known cost is the per-assert `by_event_id`/`by_delivery_key` maintenance; phase 250 should accept that tradeoff unless a new measurement says otherwise.
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
