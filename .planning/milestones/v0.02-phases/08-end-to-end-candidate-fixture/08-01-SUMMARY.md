---
phase: 08-end-to-end-candidate-fixture
plan: 01
subsystem: runtime-e2e-fixture
tags: [justifications, conflict-report, m011-s08, check_admission, csr-lease]

requires:
  - phase: 07-provenance-and-rule-version-check
    provides: Behavior.rule_version, derivation_eligible_at, persist/open outcomes sidecar, CorrectInterval residuals, CSR drop-rebuild
provides:
  - JUSTIFICATIONS_REL justifications.jsonl persist/open twin of outcomes
  - record_justification / check_admission fail-closed AdmissionDenied
  - conflict_report_at positive_supports and negative_supports without a winner
  - Named FIX-01 / FIX-02 / FIX-03 oracles on one builder
affects: [08-02 GATE-01 needle registration]

actuals:
  tokens: 9180
  tasks: 3
  commits: 3

tech-stack:
  added: []
  patterns:
    - Outcomes JSONL sidecar clone for authoritative justifications (not a lease)
    - Current-picture fact_seq liveness at the row VT so stale cites cannot renew

key-files:
  created:
    - crates/kutha-runtime/tests/m011_e2e_fixture.rs
    - .planning/phases/08-end-to-end-candidate-fixture/08-01-SUMMARY.md
  modified:
    - crates/kutha-runtime/src/quantum.rs
    - crates/kutha-runtime/src/store.rs
    - crates/kutha-runtime/src/lib.rs
    - CHANGELOG.md

key-decisions:
  - "D-F1: one shared builder plus three named oracles in m011_e2e_fixture.rs"
  - "D-F2: justifications.jsonl beside the log; check_admission fail-closed on dead source_fact_seqs"
  - "D-F3: conflict_report_at reports both sides; no winner"
  - "D-F4: incremental fold matches replay, open-without-snapshot, and CSR drop-rebuild"
  - "D-F6: GATE-01 fsm/check/bridge needles deferred to 08-02"
  - "D-F7: fold arms, derivation_eligible_at, outcomes emit path, CSR from_fold, replay_check / provenance_fingerprint mix unchanged"

patterns-established:
  - "Justification buffer is explicit record_justification; emit still auto-pushes outcomes only"
  - "Stale admission keys on source_fact_seqs live at current TT × row VT, not claim_id alone"

requirements-completed: [FIX-01, FIX-02, FIX-03]

coverage:
  - id: D1
    description: Independent supports and conflict report at named cuts; residuals of a keep P; last positive-P empty; Q history live
    requirement: FIX-01
    verification:
      - kind: integration
        ref: crates/kutha-runtime/tests/m011_e2e_fixture.rs#e2e_fixture_supports_and_conflict_at_named_cuts
        status: pass
      - kind: integration
        ref: crates/kutha-runtime/tests/m011_claim_supports.rs#derived_q_loses_eligibility_when_last_premise_support_withdrawn
        status: pass
      - kind: integration
        ref: crates/kutha-runtime/tests/m011_partial_correction.rs#interval_patch_leaves_vt_2012_and_2021_residuals
        status: pass
    human_judgment: false
  - id: D2
    description: Justification cites survive snapshot delete; stale source_fact_seqs cannot renew; new row after reevaluation
    requirement: FIX-02
    verification:
      - kind: integration
        ref: crates/kutha-runtime/tests/m011_e2e_fixture.rs#e2e_justification_cites_sources_and_rejects_stale_admission
        status: pass
    human_judgment: false
  - id: D3
    description: Incremental Runtime matches GraphFold replay, open-without-snapshot, and typed/untyped CSR drop-rebuild
    requirement: FIX-03
    verification:
      - kind: integration
        ref: crates/kutha-runtime/tests/m011_e2e_fixture.rs#e2e_incremental_matches_reconstruct_after_discarding_leases
        status: pass
      - kind: integration
        ref: crates/kutha-runtime/tests/m011_typed_csr.rs#untyped_csr_neighbor_set_and_ff5_still_hold
        status: pass
      - kind: integration
        ref: crates/kutha-runtime/tests/m011_quantum_outcome.rs#budgets_0_1_2_distinguish_zero_partial_full_after_persist_open
        status: pass
    human_judgment: false
  - id: D4
    description: Wave-close cargo green and kutha-gov ci HIGH-free with D-10 Trajectory
    verification:
      - kind: other
        ref: cargo test --workspace --offline
        status: pass
      - kind: other
        ref: uv run kutha-gov ci
        status: pass
      - kind: other
        ref: uv run kutha-gov explain trajectory
        status: pass
    human_judgment: false

plan_head_before: 0126b32a2cb6d20b9251c26538665881a62fdf3d
plan_head_after: 6ebf2128bec3ed08f812958b851d84c3d37a0b05
duration: 7min
completed: 2026-09-30
status: complete
---

# Phase 8 Plan 01: End-to-end candidate fixture Summary

**Durable `justifications.jsonl` plus `conflict_report_at` make one S08 fixture distinguish history, evidence, and stale admission at named TT×VT cuts without rewriting fold, eligibility, CSR, or provenance.**

## Performance

- **Duration:** ~7 min
- **Started:** 2026-09-30T04:08:57Z
- **Completed:** 2026-09-30T04:16:00Z
- **Tasks:** 3/3
- **Files modified:** 5 (+ SUMMARY)

## Accomplishments

- Cloned the outcomes persist/open path for `JUSTIFICATIONS_REL` (`justifications.jsonl`); missing file loads empty; attach on all four `store::open` returns; written last after events/outcomes (D-F2).
- Added `Justification`, `ConflictReport`, `RuntimeError::AdmissionDenied`, `record_justification`, `check_admission`, and `conflict_report_at` (report only, no winner) (D-F2, D-F3).
- Shared builder plus named oracles `e2e_fixture_supports_and_conflict_at_named_cuts`, `e2e_justification_cites_sources_and_rejects_stale_admission`, `e2e_incremental_matches_reconstruct_after_discarding_leases` (D-F1, D-F4).
- Product CHANGELOG dated 2026-09-30; wave-close `kutha-gov ci` HIGH 0 (D-F5). GATE-01 needles remain on 08-02 (D-F6). Fold/eligibility/outcomes/CSR/`replay_check`/`provenance_fingerprint` mix unchanged (D-F7).

## Task Commits

1. **Task 1: End-to-end supports and conflict at named cuts via sidecar and builder** - `9204618` (feat)
2. **Task 2: Stale admission denied and reconstruct agrees after lease discard** - `6ebf212` (test)
3. **Task 3: Product changelog, cargo, and D-F5 wave-close trajectory** - (docs metadata commit with this SUMMARY)

## Files Created/Modified

- `crates/kutha-runtime/src/quantum.rs` — Justification buffer, admission, conflict report; constructors init empty; `fork_at` starts empty
- `crates/kutha-runtime/src/store.rs` — `JUSTIFICATIONS_REL`, write after outcomes, load empty if missing
- `crates/kutha-runtime/src/lib.rs` — re-export `Justification`, `ConflictReport`
- `crates/kutha-runtime/tests/m011_e2e_fixture.rs` — builder + three named FIX oracles
- `CHANGELOG.md` — 2026-09-30 Product entry (`JUSTIFICATIONS_REL` + three fn names)

## Decisions Made

Followed D-F1…D-F4, D-F7: one builder; sidecar not Snapshot; report both polarities; reconstruct vs incremental; do not change fold/eligibility/CSR/provenance. GATE-01 deferred to 08-02 (D-F6). `check_admission` treats cited `source_fact_seqs` as live on the **current** transaction picture at the row's VT so CorrectInterval/Retract can stale a t1 row (FIX-02); historical `row.tt` still drives `derivation_eligible_at` and the recorded cut.

## Deviations from Plan

### Auto-fixed Issues

None — product code followed the plan.

**Note:** Task 2 has `tdd="true"` but the tracer already shipped `check_admission` / persist / CSR APIs, so a RED failure for FIX-02/FIX-03 was not achievable without stripping Task 1. The two named oracles were added and passed on first run. Plan type is `execute` (not `tdd`); `TDD_MODE` was false.

**Git:** `main` is protected; work landed on `gsd/phase-08-end-to-end-candidate-fixture` rather than committing on the default branch.

**Admission TT:** Pattern text used `is_live_at(row.tt, row.vt)`, which would never stale after CorrectInterval (invalidation TT is later than t1). Implementation uses `is_live_at(u64::MAX, row.vt)` so FIX-02 `stale_support` holds. Documented as composing D-F2 “cannot renew” with fold invalidation.

---

**Total deviations:** 0 auto-fixed (TDD RED skipped as noted; branch re-home for protected `main`; admission TT as above).
**Impact on plan:** No scope creep. All three locked FIX fn names exist and pass.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Trajectory (D-10 / D-F5)

1. **Commands:** `uv run kutha-gov ci`; `uv run kutha-gov explain trajectory`.
2. **Outcome:** `ci` exit 0; **HIGH 0**, **LOW 0** (31 checks); explain exit 0. `cargo test --workspace --offline` exit 0 (D-15).
3. **Explain paraphrase:** Check `trajectory` confirms Active Milestone/Slice pointers exist on ROADMAP; authority none — harness does not accept ADRs or claim product readiness.
4. **Honesty:** Green governor is not ADR Accepted and not L_capability.

Cited: D-F1 (shared builder + three oracles), D-F2 (durable cites, stale cannot renew), D-F3 (conflict report, no winner), D-F4 (incremental matches reconstruct after discarding snapshot and CSR), D-F7 (compose, do not rewrite fold/eligibility/outcomes/CSR/provenance). GATE-01 needle registration remains plan 08-02 (D-F6). Existing `observe_cargo.required` names still pass; this wave did not append the new e2e fns.

## Threat Flags

None beyond the plan register (T-08-01…T-08-05, T-08-SC mitigated by live `source_fact_seqs`, sidecar not Snapshot, report-only conflict, missing file empty, live Behavior `rule_version` pin, no new packages).

## Known Stubs

None.

## Next Phase Readiness

Ready for 08-02: register GATE-01 `m011-e2e` / `B-m011-e2e` plus FSM observe names; Process/Trajectory changelog; honeycomb ADR-013/011/012/040 evidence append with map still Proposed. Do not edit `.kutha/STATE.md`. Do not mark cells Accepted.

## Self-Check: PASSED

- FOUND: crates/kutha-runtime/src/store.rs (JUSTIFICATIONS_REL)
- FOUND: crates/kutha-runtime/src/quantum.rs (conflict_report_at, check_admission, record_justification)
- FOUND: crates/kutha-runtime/src/lib.rs (Justification, ConflictReport re-exports)
- FOUND: crates/kutha-runtime/tests/m011_e2e_fixture.rs (all three named tests)
- FOUND: CHANGELOG.md (2026-09-30 JUSTIFICATIONS_REL + three fn names)
- FOUND: commits 9204618, 6ebf212
- FOUND: .kutha/STATE.md Active Slice S08 (unchanged)
- FOUND: fsm.yaml has no e2e observe names (D-F6)

---
*Phase: 08-end-to-end-candidate-fixture*
*Completed: 2026-09-30*
