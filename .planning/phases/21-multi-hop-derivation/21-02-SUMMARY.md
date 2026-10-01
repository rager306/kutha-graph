---
phase: 21-multi-hop-derivation
plan: 02
subsystem: runtime
tags: [derivation, inverse_knows, persist, DER-03]

requires:
  - phase: 21-multi-hop-derivation
    provides: caused_by walk and two-hop live/retract oracles
provides:
  - "Named residual spike guard inverse_knows_cascade_is_residual_spike"
  - "persist_open_reconstructs_two_hop_eligibility"
  - "Product changelog for two-hop walk and residual inverse_knows"
affects: [21-03, follow_ons, store::open]

actuals:
  tokens: 1899
  tasks: 2
  commits: 2
  plan_head_before: fe635a5b6d14cf0e87171b11b3aeb67ed589ed50
  plan_head_after: 23c95a75ac3b996fd68f836f4c2efa3bd7b3c36f

tech-stack:
  added: []
  patterns:
    - "inverse_knows stays knows-only with an empty pin; hashed Behavior is the leased follow-on"
    - "Two-hop eligibility is a log-fact picture; persist/open does not need a new Op"

key-files:
  created: []
  modified:
    - crates/kutha-runtime/src/quantum.rs
    - crates/kutha-runtime/tests/m012_multi_hop.rs
    - CHANGELOG.md

key-decisions:
  - "D-03 locked or: keep follow_ons as the residual automatic cascade; do not add a second interned-relation cascade"
  - "hydrate_from_log already reconstructs the walk; no new persist/open path"

patterns-established:
  - "Named residual guard documents inverse_knows so it cannot impersonate a hashed pin"
  - "Temp persist dirs use kutha-m012-s05 plus nanos and remove_dir_all"

requirements-completed: [DER-01, DER-02, DER-03]

coverage:
  - id: D1
    description: "knows Assert mints inverse_knows with an empty pin; relatedTo Assert does not"
    requirement: DER-03
    verification:
      - kind: unit
        ref: crates/kutha-runtime/tests/m012_multi_hop.rs#inverse_knows_cascade_is_residual_spike
        status: pass
    human_judgment: false
  - id: D2
    description: "persist then open reconstructs two-hop live eligibility and post-retract ineligibility"
    requirement: DER-01
    verification:
      - kind: unit
        ref: crates/kutha-runtime/tests/m012_multi_hop.rs#persist_open_reconstructs_two_hop_eligibility
        status: pass
    human_judgment: false
  - id: D3
    description: "Workspace cargo and prior ALL/RULE/ADM/ACT/FF5/FF6/M012a oracles stay green"
    requirement: DER-02
    verification:
      - kind: unit
        ref: cargo test --workspace --offline
        status: pass
    human_judgment: false

duration: 2min
completed: 2026-10-01
status: complete
---

# Phase 21 Plan 02: Residual cascade and persist/open Summary

**inverse_knows stays the residual automatic knows-only cascade with an empty pin; persist/open reconstructs two-hop eligibility from the log**

## Performance

- **Duration:** 2 min
- **Started:** 2026-10-01T04:06:06Z
- **Completed:** 2026-10-01T04:08:18Z
- **Tasks:** 2
- **Files modified:** 3

## Accomplishments

- Named DER-03 residual guard: `knows` Assert still mints `inverse_knows` with an empty pin; `relatedTo` Assert and hashed `derive_pq` do not (T-21-03).
- `follow_ons` rustdoc names that residual and that hashed Behavior chains are the leased follow-on.
- persist/open reconstructs hop1/hop2 live eligibility and post-retract hop2 ineligibility (T-21-04).
- Dated Product changelog; `0.0.0` unchanged.

## Task Commits

1. **Task 1: Named residual guard for inverse_knows** - `71957a9` (test)
2. **Task 2: persist/open reconstructs two-hop eligibility** - `23c95a7` (feat)

**Plan metadata:** (pending docs commit)

## Files Created/Modified

- `crates/kutha-runtime/src/quantum.rs` - `follow_ons` rustdoc (residual automatic cascade)
- `crates/kutha-runtime/tests/m012_multi_hop.rs` - DER-03 and persist/open oracles
- `CHANGELOG.md` - Product entry for the two-hop walk and residual guard

## Decisions Made

- Keep `follow_ons` as the residual cascade; do not add a second interned-relation cascade.
- No hydrate_from_log change: `store::open` already rebuilds fold state the walk needs.

## Deviations from Plan

None - plan executed exactly as written.

Task 1 was GREEN without changing cascade logic: the residual already existed; the task added the named guard and rustdoc.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 21-03 can register the three DER observe names. Do not clear the S05 lease.

## Self-Check: PASSED

---
*Phase: 21-multi-hop-derivation*
*Completed: 2026-10-01*
