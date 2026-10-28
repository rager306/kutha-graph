---
phase: 19-admission-and-policy-meta-facts
plan: 01
subsystem: runtime
tags: [admission, policy-pin, event-log, ADM-01, ADM-02, ADM-03]

requires:
  - phase: 18-rule-registry
    provides: Op::RegisterRule log-native registry and hashed Behavior pins
provides:
  - "Op::PinPolicy and Op::RecordAdmission log facts that do not mint graph Facts"
  - "policy_version_hash and GraphFold::policy_hash_live_at / admission_status_at"
  - "record_justification Result that requires a live pin, invokes check_admission, and cites the pin"
affects: [19-02, 19-03, record_justification, admission]

actuals:
  tokens: 8383
  tasks: 3
  commits: 2
  plan_head_before: 91c7fd762b35eec41397a1442ead9f09a8dc05af
  plan_head_after: 1c6827fc8fd8668d4ebeaac698e0c618e2ec2e09

tech-stack:
  added: []
  patterns:
    - "PinPolicy mirrors RegisterRule: dedicated Op, skip-serialized fold entries, emit without QuantumOutcome sidecar"
    - "RecordAdmission is MetaOpRejected on public emit; only record_justification appends it"

key-files:
  created:
    - crates/kutha-runtime/tests/m012_admission_facts.rs
  modified:
    - crates/kutha-common/src/event.rs
    - crates/kutha-common/src/lib.rs
    - crates/kutha-runtime/src/fold.rs
    - crates/kutha-runtime/src/quantum.rs
    - crates/kutha-runtime/tests/m011_e2e_fixture.rs
    - crates/kutha-runtime/tests/m012a_log_native.rs
    - crates/kutha-runtime/tests/m012a_stable_refs.rs

key-decisions:
  - "PinPolicy and RecordAdmission are dedicated Ops, not Assert/Define/Behavior"
  - "policy_version_hash is SHA-256 of kutha-policy-def plus UTF-8 definition bytes; the Op does not store a precomputed hash"
  - "record_justification returns Ok(jid) after recording admitted=false so the status fact is the record"

patterns-established:
  - "Policy- and admission-entries skip-serialize; hydrate_from_log rebuilds them from the log walk"
  - "UnknownPolicyVersion fails closed and does not grow the log"

requirements-completed: [ADM-01, ADM-02, ADM-03]

coverage:
  - id: D1
    description: "RecordAdmission is a log fact queryable AS OF a cut; PinPolicy is not a graph Fact"
    requirement: ADM-01
    verification:
      - kind: unit
        ref: crates/kutha-runtime/tests/m012_admission_facts.rs#admission_status_queryable_as_of_cut
        status: pass
    human_judgment: false
  - id: D2
    description: "RecordAdmission.policy_version equals policy_version_hash of the live PinPolicy"
    requirement: ADM-02
    verification:
      - kind: unit
        ref: crates/kutha-runtime/tests/m012_admission_facts.rs#admission_cites_pinned_policy_version
        status: pass
    human_judgment: false
  - id: D3
    description: "record_justification invokes check_admission on the product path; stale sources record admitted=false"
    requirement: ADM-03
    verification:
      - kind: unit
        ref: crates/kutha-runtime/tests/m012_admission_facts.rs#record_justification_invokes_check_admission
        status: pass
    human_judgment: false
  - id: D4
    description: "Leased e2e and M012a LOG/REF fixtures emit PinPolicy; ALL/RULE/FF6/FF5 stay green"
    requirement: ADM-03
    verification:
      - kind: unit
        ref: crates/kutha-runtime/tests/m011_e2e_fixture.rs#e2e_justification_cites_sources_and_rejects_stale_admission
        status: pass
      - kind: unit
        ref: crates/kutha-runtime/tests/m012_allowlist_facts.rs#allow_relation_appends_as_log_fact
        status: pass
    human_judgment: false

duration: 7min
completed: 2026-10-01
status: complete
---

# Phase 19 Plan 01: Admission and policy meta-facts tracer Summary

**Log-native PinPolicy plus RecordAdmission; record_justification invokes check_admission and cites the live policy hash AS OF**

## Performance

- **Duration:** 7 min
- **Started:** 2026-10-01T02:47:29Z
- **Completed:** 2026-10-01T02:54:06Z
- **Tasks:** 3
- **Files modified:** 8

## Accomplishments

- `Op::PinPolicy` and `Op::RecordAdmission` append as skip-serialized log facts; `fold.facts()` stays empty for those ops.
- `policy_version_hash` is SHA-256 of `kutha-policy-def` plus UTF-8 bytes; `RecordAdmission` cites `live_policy_pin_at`.
- `record_justification` is `Result<String, RuntimeError>`: missing pin is `UnknownPolicyVersion` and does not append; after the cite it calls `check_admission` and records `admitted` from that Result.

## Task Commits

1. **Task 1–2: PinPolicy/RecordAdmission tracer plus cite/invoke/fail-closed** - `2a3183e` (feat)
2. **Task 3: Leased e2e and M012a helpers emit PinPolicy before record_justification** - `1c6827f` (test)

**Plan metadata:** pending docs commit

_Note: TDD_MODE was false; exhaustive `Op` matches make an isolated RED uncompilable. ADM-02/ADM-03 named tests landed with the tracer feat commit._

## Files Created/Modified

- `crates/kutha-common/src/event.rs` - `Op::PinPolicy`, `Op::RecordAdmission`, `policy_version_hash`, `new` / `digest_bytes` arms
- `crates/kutha-common/src/lib.rs` - export `policy_version_hash`
- `crates/kutha-runtime/src/fold.rs` - skip-serialized policy- and admission-entries; `policy_hash_live_at`, `live_policy_pin_at`, `admission_status_at`
- `crates/kutha-runtime/src/quantum.rs` - public emit of PinPolicy, `UnknownPolicyVersion`, `record_justification` Result, Runtime `admission_status_at`
- `crates/kutha-runtime/tests/m012_admission_facts.rs` - ADM-01/02/03 named oracles
- `crates/kutha-runtime/tests/m011_e2e_fixture.rs` - PinPolicy on `build_through_t1`; unwrap Result
- `crates/kutha-runtime/tests/m012a_log_native.rs` - PinPolicy then unwrap Result
- `crates/kutha-runtime/tests/m012a_stable_refs.rs` - PinPolicy then unwrap Result

## Decisions Made

- Dedicated `Op::PinPolicy` / `Op::RecordAdmission` rather than `Assert`/`Define`/`Behavior` so status and policy are log facts, not graph Facts.
- `emit(PinPolicy)` does not append a `QuantumOutcome` sidecar; `emit(RecordAdmission)` is `MetaOpRejected`.
- `record_justification` returns `Ok(jid)` when `admitted` is false; `check_admission` remains the fail-closed query.

## Deviations from Plan

### Auto-fixed Issues

None - plan executed as written except commit batching below.

### Commit batching

**1. [Process] Tasks 1 and 2 share one feat commit**
- **Found during:** Task 1 (tracer)
- **Issue:** Cite, invoke, and fail-closed tests share the same Ops and `record_justification` Result wiring.
- **Fix:** Landed all named ADM tests with the implementation in `2a3183e`; Task 3 fixtures are a separate test commit.
- **Files modified:** `crates/kutha-runtime/tests/m012_admission_facts.rs`
- **Commit:** `2a3183e`

**2. [Rule 3 - Blocking] Unwrap e2e_incremental record_justification**
- **Found during:** Task 3
- **Issue:** `e2e_incremental_matches_reconstruct_after_discarding_leases` also calls `record_justification` and would not compile.
- **Fix:** Unwrap the Result at that site; PinPolicy already comes from `build_through_t1`.
- **Files modified:** `crates/kutha-runtime/tests/m011_e2e_fixture.rs`
- **Commit:** `1c6827f`

---

**Total deviations:** 1 process (commit batching), 1 Rule 3 compile fix
**Impact on plan:** Necessary for compile and for keeping ADM tests with the tracer. No scope creep.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Retract of PinPolicy/RecordAdmission EventIds and persist/open reconstruction are Plan 19-02.
- Governor observe names are Plan 19-03. Do not clear the S03 lease.

## Self-Check: PASSED

- FOUND: crates/kutha-runtime/tests/m012_admission_facts.rs
- FOUND: 2a3183e
- FOUND: 1c6827f

---
*Phase: 19-admission-and-policy-meta-facts*
*Completed: 2026-10-01*
