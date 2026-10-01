---
phase: 19-admission-and-policy-meta-facts
plan: 02
subsystem: runtime
tags: [admission, policy-pin, retract, persist, ADM-01, ADM-02, ADM-03]

requires:
  - phase: 19-admission-and-policy-meta-facts
    provides: Op::PinPolicy, Op::RecordAdmission, record_justification Result, hydrate rebuild
provides:
  - "Retract of RecordAdmission and PinPolicy EventIds versions later AS OF cuts"
  - "persist/open reconstructs admission_status_at and the cited policy pin"
  - "Dated Product CHANGELOG for ADM-01..03"
affects: [19-03, store::open, Retract]

actuals:
  tokens: 1964
  tasks: 2
  commits: 2
  plan_head_before: 95ba13726bee87428ed4ca34d2891710f857b30c
  plan_head_after: f6e529eb6e24b7f770112c83b2a961cc39db25e1

tech-stack:
  added: []
  patterns:
    - "UnknownFact accepts policy-entry and admission-entry EventIds; Correct/CorrectInterval stay Fact-only"
    - "hydrate rebuilds policy- and admission-entries so persist/open answers AS OF from the log"

key-files:
  created: []
  modified:
    - crates/kutha-runtime/src/quantum.rs
    - crates/kutha-runtime/tests/m012_admission_facts.rs
    - CHANGELOG.md

key-decisions:
  - "Retract of a live admission-entry or policy-entry EventId is not UnknownFact"
  - "Correct/CorrectInterval remain Fact-only"

patterns-established:
  - "Prefix fork_at before Retract keeps the prior admission cut"

requirements-completed: [ADM-01, ADM-02, ADM-03]

coverage:
  - id: D1
    description: "Retract of RecordAdmission drops later AS OF; prefix fork still returns Some(true)"
    requirement: ADM-01
    verification:
      - kind: unit
        ref: crates/kutha-runtime/tests/m012_admission_facts.rs#retract_admission_and_policy_version_at_later_cut
        status: pass
    human_judgment: false
  - id: D2
    description: "Retract of PinPolicy makes later record_justification fail closed"
    requirement: ADM-02
    verification:
      - kind: unit
        ref: crates/kutha-runtime/tests/m012_admission_facts.rs#retract_admission_and_policy_version_at_later_cut
        status: pass
    human_judgment: false
  - id: D3
    description: "persist/open reconstructs admission AS OF and the policy pin; empty PinPolicy still fails closed"
    requirement: ADM-01
    verification:
      - kind: unit
        ref: crates/kutha-runtime/tests/m012_admission_facts.rs#persist_open_reconstructs_admission_and_policy
        status: pass
    human_judgment: false
  - id: D4
    description: "ALL, RULE, FF6, FF5, and M012a LOG/REF/ING/DUR/TIME/HOT plus workspace stay green"
    requirement: ADM-03
    verification:
      - kind: unit
        ref: cargo test --workspace --offline
        status: pass
    human_judgment: false

duration: 2min
completed: 2026-10-01
status: complete
---

# Phase 19 Plan 02: Versioned admission/policy retract and persist Summary

**Retract versions later admission and policy cuts; persist/open reconstructs AS OF from hydrated log entries**

## Performance

- **Duration:** 2 min
- **Started:** 2026-10-01T02:56:04Z
- **Completed:** 2026-10-01T02:57:51Z
- **Tasks:** 2
- **Files modified:** 3

## Accomplishments

- `emit(Retract)` of a live `RecordAdmission` or `PinPolicy` EventId is accepted; apply sets `invalidated_at`.
- After retract of admission, `admission_status_at` at `u64::MAX` is `None`; `fork_at` before the retract returns `Some(true)`.
- After retract of the pin, later `record_justification` is `UnknownPolicyVersion` and does not grow the log.
- `store::open` answers the same AS OF; empty `PinPolicy` still fails closed. Product CHANGELOG dated 2026-10-01.

## Task Commits

1. **Task 1–2 tests: Retract gate plus persist/open oracle** - `ed92e87` (feat)
2. **Task 2 docs: Product changelog** - `f6e529e` (docs)

**Plan metadata:** pending docs commit

## Files Created/Modified

- `crates/kutha-runtime/src/quantum.rs` - UnknownFact accepts policy-entry and admission-entry EventIds
- `crates/kutha-runtime/tests/m012_admission_facts.rs` - retract and persist/open oracles
- `CHANGELOG.md` - dated Product entry for ADM-01..03

## Decisions Made

- Retract of policy/admission EventIds is not UnknownFact; Correct/CorrectInterval stay Fact-only.
- Snapshot JSON keys stay `facts` and `next_seq`; hydrate rebuilds the skip-serialized entries.

## Deviations from Plan

### Auto-fixed Issues

None.

### Commit batching

**1. [Process] Persist/open test landed with the retract feat commit**
- **Found during:** Task 1
- **Issue:** Persist oracle shares the leased helper and hydrate already rebuilt in 19-01.
- **Fix:** Both named tests committed in `ed92e87`; changelog is a separate docs commit.
- **Commit:** `ed92e87`

---

**Total deviations:** 1 process (commit batching)
**Impact on plan:** No behavior change. Workspace and named oracles green.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Governor observe names and bridges are Plan 19-03. Do not clear the S03 lease.

## Self-Check: PASSED

- FOUND: crates/kutha-runtime/tests/m012_admission_facts.rs
- FOUND: ed92e87
- FOUND: f6e529e

---
*Phase: 19-admission-and-policy-meta-facts*
*Completed: 2026-10-01*
