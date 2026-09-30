---
phase: 16-fold-internal-hot-indexes
plan: 01
subsystem: runtime-fold
tags: [kutha, rust, fold, as_of, claim_supported_at, HOT-01]

requires:
  - phase: 15-verify-persist-and-time-scale
    provides: persist/open plus YearCe fixture cuts
provides:
  - Fold-internal VT and claim maps for as_of / claim_supported_at
  - Examine-count oracle that a decoy-heavy cut does not walk every fact
  - Snapshot deserialize rebuilds maps from facts; maps are not fingerprint input
affects:
  - 16-02
  - 16-03

actuals:
  tokens: 3954
  tasks: 2
  commits: 3
  plan_head_before: b6ca79b972bf09af1ab59163b796593282e4e428
  plan_head_after: 1e18dc2b950402605f81e5ee9d276c4110af7191

tech-stack:
  added: []
  patterns:
    - BTreeMap valid_from plus HashMap claim_id are skip-serialized fold leases
    - Deserialize and apply rebuild or maintain maps; fingerprint hashes facts only
    - Pub examine counter for tests/*.rs (cfg(test) is off on the lib)

key-files:
  created:
    - crates/kutha-runtime/tests/m012a_hot_indexes.rs
  modified:
    - crates/kutha-runtime/src/fold.rs
    - CHANGELOG.md

key-decisions:
  - "as_of / live_at walk vt_by_from.range(..=vt); claim_supported_at looks up claim_facts"
  - "Hot maps skip-serialize and rebuild from facts; they are not a second SoT"
  - "Examine counts prove skip; no Instant/elapsed oracle"

patterns-established:
  - "Pattern: pub reset_hot_examine_count / hot_examine_count on GraphFold for integration tests"
  - "Pattern: live_facts_at is the indexed live iterator for CSR Plan 16-02"

requirements-completed: [HOT-01]

coverage:
  - id: D1
    description: as_of(2017) matches brute-force is_live_at and examines fewer than all facts on a decoy-heavy log
    requirement: HOT-01
    verification:
      - kind: unit
        ref: crates/kutha-runtime/tests/m012a_hot_indexes.rs#as_of_and_claim_supported_at_skip_non_overlapping_facts
        status: pass
    human_judgment: false
  - id: D2
    description: Retract updates liveness through the maps; answers stay equal to brute-force
    requirement: HOT-01
    verification:
      - kind: unit
        ref: crates/kutha-runtime/tests/m012a_hot_indexes.rs#hot_as_of_matches_brute_force_after_retract
        status: pass
    human_judgment: false
  - id: D3
    description: Snapshot fold JSON is facts plus next_seq; open rebuilds maps; fingerprint unchanged
    requirement: HOT-01
    verification:
      - kind: unit
        ref: crates/kutha-runtime/tests/m012a_hot_indexes.rs#snapshot_fold_rebuilds_hot_maps_from_facts
        status: pass
    human_judgment: false

duration: 9min
completed: 2026-10-01
status: complete
---

# Phase 16 Plan 01: Fold-internal hot as_of / claim_supported_at Summary

**Fold-internal VT and claim maps so as_of and claim_supported_at skip non-overlapping facts, matching a brute-force live filter**

## Performance

- **Duration:** 9 min
- **Started:** 2026-09-30T19:19:45Z
- **Completed:** 2026-09-30T19:28:35Z
- **Tasks:** 2
- **Files modified:** 3

## Accomplishments

- `GraphFold` indexes fact slots by `valid_from` and `claim_id`; `live_at` / `as_of` / `claim_supported_at` walk those maps.
- Named HOT-01 oracle: 128 YearCe-3000 decoys plus 3 live 2017 `relatedTo` rows; examine counts stay below `facts().len()`.
- Retract and snapshot open keep equivalence; maps are not fingerprint or snapshot JSON keys.

## Task Commits

1. **Task 1 RED:** `2968cf3` (test) add failing examine-count oracle
2. **Task 1 GREEN:** `2e41bc7` (feat) fold-internal VT/claim maps
3. **Task 2:** `1e18dc2` (test) retract + snapshot rebuild oracles and Product changelog

## TDD Gate Compliance

- RED: `test(16-01): add failing test for indexed as_of and claim_supported_at` — `RED_EVIDENCE_OK` (examined 131, facts 131)
- GREEN: `feat(16-01): fold-internal index for as_of and claim_supported_at`
- REFACTOR: omitted (no cleanup commit)

## Files Created/Modified

- `crates/kutha-runtime/src/fold.rs` — skip-serialized maps, indexed hot reads, examine counter
- `crates/kutha-runtime/tests/m012a_hot_indexes.rs` — HOT-01 plus retract/snapshot oracles
- `CHANGELOG.md` — Product entry for fold-internal hot as_of / claim_supported_at

## Decisions Made

- Indexes are droppable leases: serde skip, rebuild from facts, fingerprint stays fact-vector only.
- Skip is proven by examine counts, not wall-clock.
- `live_facts_at` is `pub(crate)` for Plan 16-02 CSR `from_fold`.

## Deviations from Plan

Task 2 retract/snapshot tests were green on the Task 1 maps (in-place invalidate + Deserialize rebuild). No extra fold behavior was required.

None - plan executed as written aside from that TDD overlap.

**Total deviations:** 0 auto-fixed
**Impact on plan:** None

## Issues Encountered

None

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- HOT-01 landed. Plan 16-02 can point CSR `from_fold` at `live_facts_at` and wire `csr_lease_at` through Materializer.
- Do not register fsm.yaml names until 16-03. Do not clear harness S06.

## Self-Check: PASSED

- FOUND: crates/kutha-runtime/src/fold.rs
- FOUND: crates/kutha-runtime/tests/m012a_hot_indexes.rs
- FOUND: CHANGELOG.md
- FOUND: 2968cf3, 2e41bc7, 1e18dc2

---
*Phase: 16-fold-internal-hot-indexes*
*Completed: 2026-10-01*
