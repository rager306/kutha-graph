---
phase: 25-genesis-log
plan: 01
subsystem: testing
tags: [genesis, allowlist, RED]
requires: [{ phase: 24-fold-indexes-for-ingest, provides: persist/open + relation_allowed_at }]
provides: [RED oracles for YAML HashSet fallback]
affects: [25-02]
actuals: { tokens: 1338, tasks: 2, commits: 2 }
plan_head_before: c3d026748453b9a14118ea7458e1de9973d62770
plan_head_after: 2589134c04231179fa5f944ee7228e1aeb27065d
tech-stack: { added: [], patterns: [RED genesis oracles; env mutex poison recover] }
key-files: { created: [crates/kutha-runtime/tests/m013_genesis_log.rs], modified: [CHANGELOG.md] }
key-decisions:
  - "RED expects fail-closed; YAML HashSet / open YAML reload still admit or deny wrongly"
patterns-established: ["25-02 greens these tests; never edit .kutha/STATE.md"]
requirements-completed: [GEN-01, GEN-02, GEN-03]
coverage:
  - id: D1
    description: "D-07 RED retract seeded relatedTo"
    requirement: GEN-03
    verification: [{ kind: unit, ref: "crates/kutha-runtime/tests/m013_genesis_log.rs#retract_seeded_related_to_fails_closed_after", status: fail }]
    human_judgment: true
    rationale: "RED cargo FAILED is acceptance; 25-02 greens."
  - id: D2
    description: "D-03 RED open missing YAML still emits"
    requirement: GEN-01
    verification: [{ kind: unit, ref: "crates/kutha-runtime/tests/m013_genesis_log.rs#open_after_relations_path_missing_still_emits", status: fail }]
    human_judgment: true
    rationale: "RED emit Ok must FAIL today; 25-02 greens."
duration: 6min
completed: 2026-10-01
status: complete
---

# Phase 25 Plan 01: RED genesis log oracles Summary

**Failing `m013_genesis_log` tests prove YAML HashSet still admits after retract and that `open` reloads a missing `KUTHA_RELATIONS_PATH`.**

## Performance

- **Duration:** 6 min · **Started:** 2026-10-01T15:24:00Z · **Completed:** 2026-10-01T15:29:43Z · **Tasks:** 2 · **Files:** 2

## Accomplishments

- `retract_seeded_related_to_fails_closed_after` FAILED: later Assert Ok via YAML set (D-07).
- `open_after_relations_path_missing_still_emits` FAILED: `UnknownRelation { inForceAs }` after persist/open with missing YAML (D-03).
- No wiring. `.kutha/STATE.md` untouched. Evidence: `tmp/p25-red.txt`, `tmp/p25-red2.txt`.

## Task Commits

1. **RED retract seeded relatedTo** — `97c56a9` (test)
2. **RED YAML-missing open still emits** — `2589134` (test)

## Decisions / Deviations

Mutex + `into_inner` on `KUTHA_RELATIONS_PATH`. **[Rule 3]** EventId before Retract emit. **Total deviations:** 2. CBM `admit` stale; source still `allowed.contains`. No `index_repository`. Ready for 25-02. Do not start 26+.

## Self-Check: PASSED
