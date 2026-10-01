---
phase: 21-multi-hop-derivation
plan: 01
subsystem: runtime
tags: [derivation, caused_by, multi-hop, DER-01, DER-02]

requires:
  - phase: 18-rule-registry
    provides: RegisterRule, rule_definition_hash, rule_hash_live_at, one-hop derivation_eligible_at
provides:
  - "derivation_eligible_at walks caused_by through ancestor Behaviors with a visited EventId set"
  - "Named oracles two_hop_derivation_eligible_when_chain_live and two_hop_ineligible_when_root_assert_retracted"
  - "Parent RegisterRule retract drops hop2 eligibility while hop2 fact may stay live"
affects: [21-02, 21-03, derivation_eligible_at, admission]

actuals:
  tokens: 1852
  tasks: 3
  commits: 3
  plan_head_before: 66517c6f1110359aa85c3429b3ba8ee7604e37e5
  plan_head_after: d7496a06d40f30208640b14f682658b04b4f0913

tech-stack:
  added: []
  patterns:
    - "Single-parent Behavior.caused_by stays EventId; eligibility walks the chain instead of widening the schema"
    - "Visited EventId set bounds the walk; already-seen id is ineligible (T-21-01)"

key-files:
  created:
    - crates/kutha-runtime/tests/m012_multi_hop.rs
  modified:
    - crates/kutha-runtime/src/quantum.rs

key-decisions:
  - "Keep caused_by as a single EventId; satisfy D-01/D-02 by walking ancestor Behaviors"
  - "Do not bound the walk with KUTHA_MAX_CASCADE; finite log plus visited is the bound"
  - "Public derivation_eligible_at(derived, tt, vt) signature is unchanged"

patterns-established:
  - "After the immediate premise is live, Assert ends the walk; Behavior recurses"
  - "Ancestor rule_hash_live_at participates so retracting derive_pq drops hop2"

requirements-completed: [DER-01, DER-02]

coverage:
  - id: D1
    description: "Two-hop hashed chain is eligible while every hop is live"
    requirement: DER-02
    verification:
      - kind: unit
        ref: crates/kutha-runtime/tests/m012_multi_hop.rs#two_hop_derivation_eligible_when_chain_live
        status: pass
    human_judgment: false
  - id: D2
    description: "After Retract of the root Assert, hop2 fact may stay live and hop2 eligibility is false"
    requirement: DER-01
    verification:
      - kind: unit
        ref: crates/kutha-runtime/tests/m012_multi_hop.rs#two_hop_ineligible_when_root_assert_retracted
        status: pass
    human_judgment: false
  - id: D3
    description: "Retracting the parent RegisterRule drops hop2 eligibility while hop2's own pin can remain live"
    requirement: DER-01
    verification:
      - kind: unit
        ref: crates/kutha-runtime/tests/m012_multi_hop.rs#two_hop_ineligible_when_parent_rule_retracted
        status: pass
    human_judgment: false
  - id: D4
    description: "One-hop premise withdrawal and ALL/RULE/ADM/ACT/FF5/FF6/M012a LOG/REF stay green"
    requirement: DER-01
    verification:
      - kind: unit
        ref: crates/kutha-runtime/tests/m011_claim_supports.rs#derived_q_loses_eligibility_when_last_premise_support_withdrawn
        status: pass
    human_judgment: false

duration: 2min
completed: 2026-10-01
status: complete
---

# Phase 21 Plan 01: Two-hop eligibility walk Summary

**derivation_eligible_at walks a single-parent caused_by chain so a two-hop hashed Behavior fixture is eligible while live and ineligible after the root Assert is retracted**

## Performance

- **Duration:** 2 min
- **Started:** 2026-10-01T04:02:16Z
- **Completed:** 2026-10-01T04:04:50Z
- **Tasks:** 3
- **Files modified:** 2

## Accomplishments

- `Runtime::derivation_eligible_at` walks ancestor Behaviors via `caused_by` with a visited `EventId` set (DER-01, D-01, T-21-01).
- Named two-hop live-chain oracle is green (DER-02, D-02).
- Retracting the root Assert leaves the hop2 fact live and drops hop2 eligibility (T-21-02).
- Retracting the parent `derive_pq` RegisterRule drops hop1 and hop2 eligibility while both facts may stay live.
- Public signature is unchanged; no MATCH compiler; no provenance-polynomial evaluator; `follow_ons` untouched.

## Task Commits

1. **Task 1 RED: End-to-end two-hop eligibility** - `f7d2d33` (test)
2. **Task 1 GREEN: walk caused_by** - `2e64c78` (feat)
3. **Task 2: Walk ancestor rule pins** - `d7496a0` (test)

**Plan metadata:** (pending docs commit)

_Note: TDD tasks may have multiple commits (test → feat → refactor)_

## Files Created/Modified

- `crates/kutha-runtime/src/quantum.rs` - `derivation_eligible_at_walk` with visited set; rustdoc names that this is not a provenance-polynomial evaluator
- `crates/kutha-runtime/tests/m012_multi_hop.rs` - DER-01/DER-02 named oracles

## Decisions Made

- Walk the existing single-parent `caused_by` chain instead of changing `Op::Behavior`.
- Insert `derived` into the visited set first; already-seen id returns false.
- Do not use `KUTHA_MAX_CASCADE` as a walk bound.

## Deviations from Plan

None - plan executed exactly as written.

Task 2's parent-rule oracle was GREEN without a second product edit: the tracer walk already requires each ancestor hop's `rule_hash_live_at`.

## TDD Gate Compliance

- RED: `f7d2d33` — `two_hop_ineligible_when_root_assert_retracted` failed because hop2 stayed eligible after root Retract. Evidence: `.planning/phases/21-multi-hop-derivation/21-01-tdd-red-evidence.json` (`RED_EVIDENCE_OK`).
- GREEN: `2e64c78` — both tracer names pass.
- `two_hop_derivation_eligible_when_chain_live` was already green on one-hop keys while the chain was live; the retract oracle was the intentional RED.

## Issues Encountered

- `cargo test --format tap` is unsupported on this toolchain; RED evidence TAP appendix was added for `tdd-red-evidence` classification without fabricating the failure.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 21-02 can add the DER-03 `inverse_knows` residual guard and persist/open reconstruction.
- Do not register fsm.yaml names until Plan 21-03. Do not clear the S05 lease.

## Self-Check: PASSED

---
*Phase: 21-multi-hop-derivation*
*Completed: 2026-10-01*
