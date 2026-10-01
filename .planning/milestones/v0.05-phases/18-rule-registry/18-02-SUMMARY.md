---
phase: 18-rule-registry
plan: 02
subsystem: runtime
tags: [rule-registry, retract, persist, RULE-01, RULE-02, RULE-03]

requires:
  - phase: 18-rule-registry
    provides: Op::RegisterRule, rule_definition_hash, rule_hash_live_at, user emit pin check
provides:
  - "Retract of a RegisterRule EventId versions the later registry cut"
  - "store::open hydrates rule-entries from the log"
  - "Product changelog for RULE-01..03"
affects: [18-03, emit, eligibility, persist]

actuals:
  tokens: 2295
  tasks: 2
  commits: 1
  plan_head_before: 8cf00abd09a2f33ba611a3d58df587f27fa9fa02
  plan_head_after: d84a21a054c28affb7a9bb953d8f582f53d85509

tech-stack:
  added: []
  patterns:
    - "UnknownFact accepts rule-entry EventId next to allow-entry; Correct stays Fact-only"
    - "hydrate_from_log rebuilds rule-entries so snapshot skip-serialize does not drop the registry"

key-files:
  created: []
  modified:
    - crates/kutha-runtime/src/fold.rs
    - crates/kutha-runtime/src/quantum.rs
    - crates/kutha-runtime/tests/m012_rule_registry.rs
    - CHANGELOG.md

key-decisions:
  - "Retract of RegisterRule is accepted as a live rule-entry EventId, not as a graph Fact"
  - "Opened runtimes accept hashed Behavior because hydrate rebuilds rule-entries from the log"

patterns-established:
  - "fork_at of the pre-Retract prefix still emits a hashed Behavior"

requirements-completed: [RULE-01, RULE-02, RULE-03]

coverage:
  - id: D1
    description: "Retract of RegisterRule drops later emit and eligibility; prefix fork still accepts the hash"
    requirement: RULE-01
    verification:
      - kind: unit
        ref: crates/kutha-runtime/tests/m012_rule_registry.rs#retract_register_rule_drops_eligibility_at_later_cut
        status: pass
    human_judgment: false
  - id: D2
    description: "persist then open reconstructs the registry so hashed Behavior still emits; free-string still fails closed"
    requirement: RULE-01
    verification:
      - kind: unit
        ref: crates/kutha-runtime/tests/m012_rule_registry.rs#persist_open_behavior_requires_hydrated_registry
        status: pass
    human_judgment: false
  - id: D4
    description: "ALL/FF6/FF5 and M012a LOG/REF/ING/DUR/TIME/HOT plus cargo test --workspace stay green"
    requirement: RULE-03
    verification:
      - kind: unit
        ref: cargo test --workspace --offline
        status: pass
    human_judgment: false

duration: 2min
completed: 2026-10-01
status: complete
---

# Phase 18 Plan 02: Versioned registry and persist/open Summary

**Retract of RegisterRule fails closed at a later cut; persist/open reconstructs the hashed registry from log facts**

## Performance

- **Duration:** 2 min
- **Started:** 2026-10-01T02:25:00Z
- **Completed:** 2026-10-01T02:26:52Z
- **Tasks:** 2
- **Files modified:** 4

## Accomplishments

- `UnknownFact` accepts a live rule-entry EventId; `apply` Retract sets `invalidated_at` on that entry.
- After Retract, hashed `emit(Behavior)` returns `UnknownRuleVersion` and `derivation_eligible_at` is false; `fork_at` of the pre-Retract prefix still emits.
- `store::open` hydrates rule-entries from the log; a three-character free-string pin still fails closed.

## Task Commits

1. **Task 1–2: Retract, persist/open, Product changelog** - `d84a21a` (feat)

**Plan metadata:** pending docs commit

_Note: kutha-changelog requires CHANGELOG.md in the same commit as crate paths, so both 18-02 tasks share one feat commit._

## Files Created/Modified

- `crates/kutha-runtime/src/quantum.rs` - Retract UnknownFact gate includes `has_rule_entry`
- `crates/kutha-runtime/src/fold.rs` - `has_rule_entry` used (no longer dead)
- `crates/kutha-runtime/tests/m012_rule_registry.rs` - retract and persist/open oracles
- `CHANGELOG.md` - dated Product entry for the rule registry

## Decisions Made

- Retract of a registry row is EventId-targeted like AllowRelation; Correct/CorrectInterval stay Fact-only.
- Snapshot JSON still serializes only `facts` and `next_seq`; the opened runtime trusts the log walk.

## Deviations from Plan

**1. [Process] Tasks 1 and 2 share one feat commit**
- **Found during:** Task 2 changelog
- **Issue:** Product changelog must ship with crate changes (docs-coupling / kutha-changelog).
- **Fix:** One `feat(18-02)` commit covering Retract, persist/open tests, and CHANGELOG.
- **Commit:** `d84a21a`

---

**Total deviations:** 1 process (commit batching)
**Impact on plan:** No behavior change. Governor observe names remain Plan 18-03.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Retract and persist/open oracles green; workspace tests green; Product changelog dated.
- Plan 18-03 registers RULE file names with the governor. Do not clear the S02 lease.

## Self-Check: PASSED

- FOUND: crates/kutha-runtime/src/fold.rs
- FOUND: crates/kutha-runtime/src/quantum.rs
- FOUND: crates/kutha-runtime/tests/m012_rule_registry.rs
- FOUND: CHANGELOG.md
- FOUND: d84a21a
