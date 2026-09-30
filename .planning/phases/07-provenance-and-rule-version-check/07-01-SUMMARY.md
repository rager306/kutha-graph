---
phase: 07-provenance-and-rule-version-check
plan: 01
subsystem: runtime-provenance
tags: [provenance, rule_version, adr-060, m011-s07, replay_check]

requires:
  - phase: 06-typed-csr-lease
    provides: Runtime emit/admit, GraphFold fingerprint, from_dict_and_events, replay_check BrokenLineage
provides:
  - Op::Behavior.rule_version with serde default
  - Runtime::provenance_fingerprint / provenance_check / ProvenanceMismatch
  - Named PROV-01 and PROV-02 oracles in m011_provenance.rs
affects: [07-02 GATE-01 needle registration]

actuals:
  tokens: 2893
  tasks: 3
  commits: 3

tech-stack:
  added: []
  patterns:
    - Dual verification from one EventLog (state fingerprint vs lineage digest)
    - Clone Event vec then from_dict_and_events so Fact.claim_id stays event.id

key-files:
  created:
    - crates/kutha-runtime/tests/m011_provenance.rs
    - .planning/phases/07-provenance-and-rule-version-check/07-01-SUMMARY.md
  modified:
    - crates/kutha-common/src/event.rs
    - crates/kutha-runtime/src/quantum.rs
    - crates/kutha-runtime/tests/m011_claim_supports.rs
    - CHANGELOG.md

key-decisions:
  - "D-P1: PROV-01 swaps caused_by among valid priors; ghost BrokenLineage stays regression"
  - "D-P2: rule_version on Op::Behavior with serde default; fold still ignores lineage"
  - "D-P3: provenance_fingerprint / provenance_check beside unchanged replay_check"
  - "D-P4: Exact named oracles for PROV-01 and PROV-02"
  - "D-P7: Assert/Retract/Correct/CorrectInterval fold, typed/untyped CSR, outcomes sidecar unchanged"
  - "D-P6: GATE-01 fsm/check/bridge needles deferred to 07-02"

patterns-established:
  - "Lineage digest hashes Behavior rows only with kutha-prov-v1 and length-prefixed name/rule_version"
  - "Clone-then-rebuild fixtures keep Event.id so state fingerprint can stay equal"

requirements-completed: [PROV-01, PROV-02]

coverage:
  - id: D1
    description: Caused_by swap among valid priors moves provenance digest while state fingerprint matches
    requirement: PROV-01
    verification:
      - kind: integration
        ref: crates/kutha-runtime/tests/m011_provenance.rs#provenance_detects_caused_by_swap_when_state_fingerprint_matches
        status: pass
      - kind: integration
        ref: crates/kutha-runtime/tests/m011_claim_supports.rs#replay_rejects_behavior_without_prior_cause
        status: pass
    human_judgment: false
  - id: D2
    description: Rule-version-only change moves provenance digest while state fingerprint matches; no execution replay
    requirement: PROV-02
    verification:
      - kind: integration
        ref: crates/kutha-runtime/tests/m011_provenance.rs#provenance_detects_rule_version_change_when_state_fingerprint_matches
        status: pass
    human_judgment: false
  - id: D3
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

plan_head_before: d347c2d3bc299d8fcf151eabefcb39ade8d21c90
plan_head_after: 11aeda748a3ec4cccfcaf856a3d7eb57b585b501
duration: 4min
completed: 2026-09-30
status: complete
---

# Phase 07 Plan 01: Provenance and rule-version check Summary

**`Runtime::provenance_fingerprint` hashes Behavior lineage (`id`, `caused_by`, `name`, `rule_version`) so a valid-prior `caused_by` swap or an `r1`→`r2` pin move the digest while `GraphFold::fingerprint` and `replay_check` stay green.**

## Performance

- **Duration:** ~4 min
- **Started:** 2026-09-30T02:41:55Z
- **Completed:** 2026-09-30T02:45:35Z
- **Tasks:** 3/3
- **Files modified:** 5 (+ SUMMARY)

## Accomplishments

- Added `rule_version: String` on `Op::Behavior` with `#[serde(default)]`; `Event::digest_bytes` mixes the field with no extra tag (D-P2). Fold Behavior arm still projects subject/relation/object/VT only (D-P7).
- Added `RuntimeError::ProvenanceMismatch`, `provenance_fingerprint`, and `provenance_check` beside unchanged `replay_check` (D-P3). Mix is domain tag `kutha-prov-v1` plus length-prefixed name/rule_version.
- Named oracles `provenance_detects_caused_by_swap_when_state_fingerprint_matches` (PROV-01) and `provenance_detects_rule_version_change_when_state_fingerprint_matches` (PROV-02). Ghost prior still `BrokenLineage` (D-P1).
- Product CHANGELOG dated 2026-09-30; wave-close `kutha-gov ci` HIGH 0 (D-P5). GATE-01 needles remain on 07-02 (D-P6).

## Task Commits

1. **Task 1: End-to-end caused_by swap detected while state fingerprint matches** - `972f47a` (feat)
2. **Task 2: Rule-version pin change detected while state fingerprint matches** - `d6c324f` (test)
3. **Task 3: Product changelog, cargo, and D-P5 wave-close trajectory** - `11aeda7` (docs)

## Files Created/Modified

- `crates/kutha-common/src/event.rs` — `rule_version` on `Op::Behavior`; `digest_bytes` mix
- `crates/kutha-runtime/src/quantum.rs` — `provenance_fingerprint` / `provenance_check` / `ProvenanceMismatch`; `follow_ons` empty pin
- `crates/kutha-runtime/tests/m011_claim_supports.rs` — empty `rule_version` on exhaustive Behavior ctors
- `crates/kutha-runtime/tests/m011_provenance.rs` — PROV-01 / PROV-02 oracles (+ optional serde helper)
- `CHANGELOG.md` — 2026-09-30 Product entry

## Decisions Made

Followed D-P1…D-P4, D-P7 from CONTEXT: valid-prior swap (not ghost) as PROV-01; `rule_version` on the log event; separate provenance surface; exact D-P4 names; fold/CSR/outcomes frozen. GATE-01 deferred to 07-02 (D-P6).

## Deviations from Plan

### Auto-fixed Issues

None — product code followed the plan.

**Note:** Task 2 has `tdd="true"` but the tracer already mixed `rule_version` into `provenance_fingerprint`, so a RED failure for PROV-02 was not achievable without breaking Task 1. The PROV-02 oracle was added and passed on first run. Plan type is `execute` (not `tdd`); `TDD_MODE` was false. Optional `behavior_without_rule_version_field_deserializes` is not a GATE observe name (RESEARCH Q6).

**Git:** `main` is protected; work landed on `gsd/phase-07-provenance-and-rule-version-check` rather than committing on the default branch.

---

**Total deviations:** 0 auto-fixed (TDD RED skipped as noted; branch re-home for protected `main`).
**Impact on plan:** No scope creep. PROV-01/PROV-02 oracles exist with the locked names.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Trajectory (D-10 / D-P5)

1. **Commands:** `uv run kutha-gov ci`; `uv run kutha-gov explain trajectory`.
2. **Outcome:** `ci` exit 0; **HIGH 0**, **LOW 0** (30 checks); explain exit 0. `cargo test --workspace --offline` exit 0 (D-15).
3. **Explain paraphrase:** Check `trajectory` confirms Active Milestone/Slice pointers exist on ROADMAP; authority none — harness does not accept ADRs or claim product readiness.
4. **Honesty:** Green governor is not ADR Accepted and not L_capability.

Cited: D-P1 (valid-prior swap + ghost regression), D-P2 (`rule_version` on Behavior), D-P3 (provenance surface ≠ `replay_check`), D-P4 (named oracles), D-P7 (fold/CSR/outcomes unchanged). GATE-01 needle registration remains plan 07-02 (D-P6). Existing `observe_cargo.required` names still pass; this wave did not append the new provenance fns.

## Threat Flags

None beyond the plan register (T-07-01…T-07-05 mitigated by lineage hash, clone-then-rebuild, serde default, unchanged `BrokenLineage`, length-prefixed mix).

## Known Stubs

None.

## Next Phase Readiness

Ready for 07-02: register GATE-01 `m011-provenance` / `B-m011-provenance` plus FSM observe names; Process/Trajectory changelog; honeycomb ADR-060 and ADR-011 evidence append with map still Proposed. Do not implement execution replay or S08.

## Self-Check: PASSED

- FOUND: crates/kutha-common/src/event.rs (rule_version)
- FOUND: crates/kutha-runtime/src/quantum.rs (provenance_fingerprint, replay_check unchanged)
- FOUND: crates/kutha-runtime/tests/m011_provenance.rs (both named tests)
- FOUND: CHANGELOG.md (2026-09-30 provenance_fingerprint + both fn names)
- FOUND: commits 972f47a, d6c324f, 11aeda7
- FOUND: .kutha/STATE.md Active Slice S07 (unchanged)
- FOUND: fsm.yaml has no provenance_detects observe names (D-P6)

---
*Phase: 07-provenance-and-rule-version-check*
*Completed: 2026-09-30*
