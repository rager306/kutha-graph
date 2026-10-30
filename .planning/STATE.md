---
gsd_state_version: "1.0"
milestone: v1.70
status: planning
stopped_at: Phase 261 ready to execute
last_updated: "2026-10-30T18:32:22.223Z"
last_activity: 2026-10-31
last_activity_desc: Milestone v1.70 started; C1b manifest/operator registry planned
state_head: d9f7ed3e66e27e93360bff4543aea72953c9d39b
progress:
  total_phases: 2
  completed_phases: 0
  total_plans: 2
  completed_plans: 0
milestone_name: C1b pack manifest and operator registry
current_phase: 261
---

Total Phases: 0

# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-10-29)

**Core value:** Prove legal point-in-time on a named statute-shaped fixture while keeping governor CI honest and advancing one Active Slice at a time.
**Current focus:** v1.70 C1b pack manifest and operator registry (M177-S01), phase 261.

## Current Position

Phase: 261 — C1b pack manifest and operator registry
Plan: 261-01
Status: Ready to execute
Last activity: 2026-10-31 — Milestone v1.70 started; C1b planned

## Accumulated Context

### Decisions

- 2026-10-29: Prime suspect is `cec5b64` / `LiveFactFilter::Product` per-fact dictionary check.
- 2026-10-30: M169 and M170 closed the `as_of` causes; M171 identified `199807d` as the ingest index commit and accepted the per-assert map write cost.
- 2026-10-30: `KUTHA_FOLD_INDEX_MODE=lazy|thin` is the bounded write-thin opt-out; Eager remains default.
- 2026-10-30: B3 showed no gap; B4/B5 are skipped. M003a leases the read-only GPML/Cypher subset parser; HNSW/planner and legal pack stay frozen.
- 2026-10-30: M003a binder uses intern/allowlist now; pack-dictionary binder is a later small lease (M003b after C2).
- 2026-10-30: C1 foundation is bounded to typed lifecycle events and a replay-derived lease; no artifact loading, operator registration, dictionary facets, lease rebuild, WASM, or marketplace.
- 2026-10-30: Gate GB closed; M175 leases the bounded C4 foundation only — closed-default path-policy compiler before prepare. Grants, tenants, vacuum, capabilities, WASM, and full ABAC remain out of scope.

### Pending Todos

None.

### Blockers/Concerns

B3 environment is up: FalkorDB `kutha-b3-falkor` (127.0.0.1:6381) and Samyama `kutha-b3-samyama` (127.0.0.1:6380/8081), both smoke-tested; artifact `.compound-engineering/artifacts/research/2026-10-30-b3-environment.md`. FalkorDB loaded the 768-node/2048-edge shape and returns 4096 for the 2-hop count (internal 5.0 ms, not calibrated). Samyama `RETURN 1` works; `UNWIND ... CREATE` is rejected, but comma `CREATE` and single-edge `CREATE` are accepted; a batch load form is still needed. No competitor ratio and no B4/B5 decision. Do not start HNSW/planner/Cypher before phase 252. Bisect builds must not share a target dir; use a detached worktree. The M171 five-sample table is in `docs/architecture/benchmark-baseline.md`; the earlier n=2 outlier is retained only as a caveat.

## Deferred Items

| Category | Item | Status | Deferred At | Milestone |
|----------|------|--------|-------------|-----------|
| *(none)* | | | | |

## Session Continuity

Last session: 2026-10-29
Stopped at: Phase 260 complete — all phases complete
Resume file: None

## Operator Next Steps

- Execute phase 261 with the plan below.
