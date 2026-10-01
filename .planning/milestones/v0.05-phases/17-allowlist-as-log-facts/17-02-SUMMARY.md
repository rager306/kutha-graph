---
phase: 17-allowlist-as-log-facts
plan: 02
subsystem: runtime
tags: [allowlist, retract, persist, admit, ALL-01, ALL-02, ALL-03]

requires:
  - phase: 17-allowlist-as-log-facts
    provides: Op::AllowRelation, relation_allowed_at, admit fold-then-YAML
provides:
  - "Retract of AllowRelation EventId versions the later admit cut"
  - "store::persist then open reconstructs YAML-absent admit from log facts"
  - "Product changelog for allowlist-as-log-facts"
affects: [17-03, store-open, fork_at]

actuals:
  tokens: 1750
  tasks: 2
  commits: 2
  plan_head_before: 34182d47fdb013b9a479939281ee5e62c08f339d
  plan_head_after: 9103bd28a213fcf01cd0b8db994b9d460339b88f

tech-stack:
  added: []
  patterns:
    - "Retract UnknownFact gate accepts allow-entry EventIds; Correct stays Fact-only"
    - "hydrate_from_log rebuilds skip-serialized allow-entries so snapshot JSON need not store them"

key-files:
  created: []
  modified:
    - crates/kutha-runtime/src/fold.rs
    - crates/kutha-runtime/src/quantum.rs
    - crates/kutha-runtime/tests/m012_allowlist_facts.rs
    - CHANGELOG.md

key-decisions:
  - "Retract of an allow-entry EventId is not UnknownFact; Correct/CorrectInterval stay Fact-only"
  - "Open reconstructs allow-entries from the log walk, not from snapshot JSON keys"

patterns-established:
  - "Versioned allowlist: live fold entry admits; retracted name fails unless YAML still lists it"
  - "fork_at of the pre-Retract prefix still admits the YAML-absent name"

requirements-completed: [ALL-01, ALL-02, ALL-03]

coverage:
  - id: D1
    description: "Retract of AllowRelation drops admit at a later cut; prefix fork still admits"
    requirement: ALL-01
    verification:
      - kind: unit
        ref: crates/kutha-runtime/tests/m012_allowlist_facts.rs#retract_allow_relation_drops_admit_at_later_cut
        status: pass
    human_judgment: false
  - id: D2
    description: "persist then open admits a YAML-absent name from hydrated log facts"
    requirement: ALL-02
    verification:
      - kind: unit
        ref: crates/kutha-runtime/tests/m012_allowlist_facts.rs#admit_consults_fold_allowlist_not_yaml_alone
        status: pass
    human_judgment: false
  - id: D3
    description: "FF6, FF5, and M012a LOG/REF/ING/DUR/TIME/HOT named tests stay green"
    requirement: ALL-03
    verification:
      - kind: unit
        ref: crates/kutha-runtime/tests/ff6_allowlist.rs#ff6_unknown_relation_does_not_append
        status: pass
      - kind: unit
        ref: cargo test --workspace --offline
        status: pass
    human_judgment: false

duration: 2min
completed: 2026-10-01
status: complete
---

# Phase 17 Plan 02: Versioned allowlist persist Summary

**Retract versions allowlist facts by EventId; persist/open reconstructs YAML-absent admit from the log**

## Performance

- **Duration:** 2 min
- **Started:** 2026-10-01T01:48:50Z
- **Completed:** 2026-10-01T01:50:50Z
- **Tasks:** 2
- **Files modified:** 4

## Accomplishments

- `emit(Retract)` of a live allow-entry EventId is accepted (not `UnknownFact`). Apply sets `invalidated_at`; later Assert of that YAML-absent name fails closed.
- `fork_at` of the pre-Retract prefix still admits.
- `store::open` hydrates allow-entries from the log so `leasedRel` admits after discarding RAM. Snapshot JSON keys stay `facts` + `next_seq`.
- Dated Product CHANGELOG entry; `0.0.0` unchanged.

## Task Commits

1. **Task 1: Retract AllowRelation drops admit at a later cut** - `7d0a9c1` (feat)
2. **Task 2: persist/open admits YAML-absent name from log facts** - `9103bd2` (feat)

**Plan metadata:** pending docs commit

## Files Created/Modified

- `crates/kutha-runtime/src/fold.rs` - `has_allow_entry` for the Retract gate
- `crates/kutha-runtime/src/quantum.rs` - Retract UnknownFact accepts allow-entry EventIds
- `crates/kutha-runtime/tests/m012_allowlist_facts.rs` - retract and persist/open oracles
- `CHANGELOG.md` - Product plane entry for allowlist-as-log-facts

## Decisions Made

- Retract of allow-entry EventId is admitted; Correct/CorrectInterval remain Fact-only.
- Allow-entries stay skip-serialized; hydrate_from_log is the reconstruction path after open.

## Deviations from Plan

None - plan executed as written. Retract invalidation in `apply` was already present from 17-01 hydrate consistency; this plan added the emit UnknownFact gate.

---

**Total deviations:** 0 auto-fixed
**Impact on plan:** None

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- ALL file oracles exist and are green. Plan 17-03 registers them with the governor.
- Do not clear the S01 lease. Honeycomb stays Proposed.

## Self-Check: PASSED

- FOUND: crates/kutha-runtime/src/fold.rs
- FOUND: crates/kutha-runtime/src/quantum.rs
- FOUND: crates/kutha-runtime/tests/m012_allowlist_facts.rs
- FOUND: CHANGELOG.md
- FOUND: 7d0a9c1
- FOUND: 9103bd2

---
*Phase: 17-allowlist-as-log-facts*
*Completed: 2026-10-01*
