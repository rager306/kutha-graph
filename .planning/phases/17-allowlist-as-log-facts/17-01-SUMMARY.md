---
phase: 17-allowlist-as-log-facts
plan: 01
subsystem: runtime
tags: [allowlist, event-log, admit, fold, ALL-01, ALL-02, ALL-03]

requires:
  - phase: 16-hot-indexes
    provides: in-memory fold + YAML/env FF6 seed Runtime.allowed
provides:
  - "Op::AllowRelation log facts that do not mint graph Facts"
  - "GraphFold skip-serialized allow-entries and relation_allowed_at"
  - "Runtime::admit fold-cut then YAML seed; unknown still fail-closed"
affects: [17-02, 17-03, admit, persist-open]

actuals:
  tokens: 3423
  tasks: 2
  commits: 2
  plan_head_before: 32850c1a3e9a605b894a53a727b5247d7cd5bc62
  plan_head_after: e980d6f6791047fd639fede4a6c4fe2fa64a018a

tech-stack:
  added: []
  patterns:
    - "Meta log ops skip-serialize in GraphFold and rebuild from the event log"
    - "admit consults fold live allow-entries before YAML/env seed"

key-files:
  created:
    - crates/kutha-runtime/tests/m012_allowlist_facts.rs
  modified:
    - crates/kutha-common/src/event.rs
    - crates/kutha-runtime/src/fold.rs
    - crates/kutha-runtime/src/quantum.rs

key-decisions:
  - "AllowRelation is a dedicated Op, not Assert/Define, so admit can bootstrap without gating itself"
  - "emit(AllowRelation) appends one log event without a QuantumOutcome sidecar so the tracer log-len contract holds"
  - "YAML seed remains the fallback so inForceAs admits on an empty-log Runtime (FF6)"

patterns-established:
  - "Allowlist rows are fold leases rebuilt by hydrate_from_log; fingerprint still hashes facts only"
  - "Unknown and empty AllowRelation names fail closed and do not append"

requirements-completed: [ALL-01, ALL-02, ALL-03]

coverage:
  - id: D1
    description: "Op::AllowRelation appends as a log fact and does not mint a graph Fact"
    requirement: ALL-01
    verification:
      - kind: unit
        ref: crates/kutha-runtime/tests/m012_allowlist_facts.rs#allow_relation_appends_as_log_fact
        status: pass
    human_judgment: false
  - id: D2
    description: "admit allows Assert of a YAML-absent name after a live AllowRelation fold entry"
    requirement: ALL-02
    verification:
      - kind: unit
        ref: crates/kutha-runtime/tests/m012_allowlist_facts.rs#allow_relation_appends_as_log_fact
        status: pass
    human_judgment: false
  - id: D3
    description: "Unknown relation and empty AllowRelation name fail closed; YAML-seeded inForceAs still admits; FF6/FF5 green"
    requirement: ALL-03
    verification:
      - kind: unit
        ref: crates/kutha-runtime/tests/m012_allowlist_facts.rs#empty_allow_relation_name_does_not_append
        status: pass
      - kind: unit
        ref: crates/kutha-runtime/tests/m012_allowlist_facts.rs#yaml_seed_still_admits_in_force_as
        status: pass
      - kind: unit
        ref: crates/kutha-runtime/tests/ff6_allowlist.rs#ff6_unknown_relation_does_not_append
        status: pass
      - kind: unit
        ref: crates/kutha-runtime/tests/ff5_legal_pit.rs#ff5_as_of_t1_differs_from_as_of_t2_on_statute_log
        status: pass
    human_judgment: false

duration: 3min
completed: 2026-10-01
status: complete
---

# Phase 17 Plan 01: Allowlist tracer Summary

**Op::AllowRelation is an appendable log fact; admit consults the fold cut for YAML-absent names; unknown still fails closed**

## Performance

- **Duration:** 3 min
- **Started:** 2026-10-01T01:43:27Z
- **Completed:** 2026-10-01T01:46:31Z
- **Tasks:** 2
- **Files modified:** 4

## Accomplishments

- `Op::AllowRelation { name, valid_from, valid_to }` hashes tag `allow-relation`; `Event::new` keeps `object_ids` empty.
- `GraphFold` skip-serializes allow-entries; `relation_allowed_at` uses the same liveness shape as `Fact::is_live_at`; `hydrate_from_log` rebuilds entries from the log.
- `Runtime::admit` checks live fold entries at `next_tt.saturating_sub(1)` and the op `valid_from`, then YAML seed. Empty AllowRelation names return `UnknownRelation` without appending.

## Task Commits

1. **Task 1: End-to-end append AllowRelation then admit a YAML-absent name** - `e6a24cc` (feat)
2. **Task 2: YAML seed still admits inForceAs; empty AllowRelation name fails closed** - `e980d6f` (test)

**Plan metadata:** pending docs commit

_Note: TDD_MODE was false; tracer landed as a single feat commit with the named oracle, then a test commit for YAML/empty-name._

## Files Created/Modified

- `crates/kutha-common/src/event.rs` - `Op::AllowRelation` plus `new` / `digest_bytes` arms
- `crates/kutha-runtime/src/fold.rs` - skip-serialized allow-entries, `relation_allowed_at`, hydrate rebuild
- `crates/kutha-runtime/src/quantum.rs` - admit fold-then-YAML, public emit of AllowRelation, hydrate rebuild
- `crates/kutha-runtime/tests/m012_allowlist_facts.rs` - ALL-01/02/03 tracer plus YAML-seed and empty-name oracles

## Decisions Made

- Dedicated `Op::AllowRelation` rather than `Assert`/`Define` so bootstrap is not gated by the allowlist it writes.
- `emit(AllowRelation)` does not append a `QuantumOutcome` sidecar, matching the tracer contract that log length grows by 1.
- YAML/env seed stays as fallback so FF6 `inForceAs` still admits with no log allow-entry.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Missing Critical] Empty AllowRelation name rejected in the tracer emit path**
- **Found during:** Task 1 (admit wiring)
- **Issue:** T-17-02 requires empty names fail closed; waiting until Task 2 would leave emit open.
- **Fix:** `admit` returns `UnknownRelation` for empty `AllowRelation` names and does not append.
- **Files modified:** `crates/kutha-runtime/src/quantum.rs`
- **Verification:** Task 2 `empty_allow_relation_name_does_not_append`
- **Committed in:** `e6a24cc` (Task 1)

**2. [Rule 1 - Bug] AllowRelation emit must not append QuantumOutcome**
- **Found during:** Task 1 (tracer log-len contract)
- **Issue:** Generic `emit` always appends a quantum-outcome meta event, so log length would grow by 2.
- **Fix:** After admit, AllowRelation appends one event and returns without cascade/outcome.
- **Files modified:** `crates/kutha-runtime/src/quantum.rs`
- **Verification:** `allow_relation_appends_as_log_fact`
- **Committed in:** `e6a24cc` (Task 1)

---

**Total deviations:** 2 auto-fixed (1 missing critical, 1 bug)
**Impact on plan:** Required for the named tracer assertions and T-17-02. No scope creep. Retract-of-allow-entry UnknownFact gate remains Plan 17-02.

## Issues Encountered

None beyond the emit sidecar vs log-len mismatch, which is documented as deviation 2.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Tracer green: YAML-absent `leasedRel` admits from the fold cut; `notALegalRelation` and empty names fail closed; FF6/FF5 named tests pass.
- Plan 17-02 should version allow-entries via Retract EventId and persist/open reconstruction.
- Do not register fsm.yaml names until Plan 17-03. Do not clear the S01 lease.

## Self-Check: PASSED

- FOUND: crates/kutha-common/src/event.rs
- FOUND: crates/kutha-runtime/src/fold.rs
- FOUND: crates/kutha-runtime/src/quantum.rs
- FOUND: crates/kutha-runtime/tests/m012_allowlist_facts.rs
- FOUND: e6a24cc
- FOUND: e980d6f

---
*Phase: 17-allowlist-as-log-facts*
*Completed: 2026-10-01*
