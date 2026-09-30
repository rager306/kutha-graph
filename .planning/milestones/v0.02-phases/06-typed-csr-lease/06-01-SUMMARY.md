---
phase: 06-typed-csr-lease
plan: 01
subsystem: runtime-csr
tags: [csr, typed-lease, adr-040, adr-041, m011-s06]

requires:
  - phase: 05-persisted-quantum-outcome
    provides: Runtime emit/admit, GraphFold live Facts, untyped CsrLease, FF5 oracles
provides:
  - TypedEdge / TypedCsrLease / typed_csr_lease_at (labels + support multiplicity)
  - Named CSR-01 and CSR-02 integration oracles in m011_typed_csr.rs
affects: [06-02 GATE-01 needle registration]

actuals:
  tokens: 3919
  tasks: 3
  commits: 4

tech-stack:
  added: []
  patterns:
    - Dual lease from one GraphFold cut (typed edges vs untyped neighbor-set)
    - One TypedEdge per live Fact; sort by (relation, object, claim_id, fact_seq); no dedup

key-files:
  created:
    - crates/kutha-runtime/tests/m011_typed_csr.rs
    - .planning/phases/06-typed-csr-lease/06-01-SUMMARY.md
  modified:
    - crates/kutha-runtime/src/csr.rs
    - crates/kutha-runtime/src/quantum.rs
    - crates/kutha-runtime/src/lib.rs
    - CHANGELOG.md

key-decisions:
  - "D-T1: TypedCsrLease separate from CsrLease; untyped from_fold/dedup untouched"
  - "D-T2: TypedEdge includes fact_seq; one edge per live Fact; no support collapse"
  - "D-T3: typed_csr_lease_at additive twin; materializer/leapfrog stay on untyped"
  - "D-T4: Exact named oracles for CSR-01 and thin CSR-02"
  - "D-T7: Fold Assert/Retract/Correct semantics and quantum_outcomes sidecar unchanged"
  - "D-T6: GATE-01 fsm/check/bridge needles deferred to 06-02"

patterns-established:
  - "Parallel typed CSR lease rebuilt from the same TT×VT cut as untyped CsrLease"
  - "Support multiplicity via claim_id + fact_seq on TypedEdge"

requirements-completed: [CSR-01, CSR-02]

coverage:
  - id: D1
    description: Typed CSR lease preserves relation labels and support multiplicity
    requirement: CSR-01
    verification:
      - kind: integration
        ref: crates/kutha-runtime/tests/m011_typed_csr.rs#typed_csr_preserves_relation_labels_and_support_multiplicity
        status: pass
    human_judgment: false
  - id: D2
    description: Untyped neighbor-set and FF5 path still hold
    requirement: CSR-02
    verification:
      - kind: integration
        ref: crates/kutha-runtime/tests/m011_typed_csr.rs#untyped_csr_neighbor_set_and_ff5_still_hold
        status: pass
      - kind: unit
        ref: crates/kutha-runtime/src/quantum.rs#quantum::tests::csr_drop_rebuild_and_seek
        status: pass
      - kind: integration
        ref: crates/kutha-runtime/tests/ff5_legal_pit.rs#ff5_as_of_t1_differs_from_as_of_t2_on_statute_log
        status: pass
    human_judgment: false

plan_head_before: f3bce629c39d4cd87931170df6fbee7c48c373df
plan_head_after: c86345a49c35bd0895588dbc9d7e2eb97d1604c2
duration: 4min
completed: 2026-09-30
status: complete
---

# Phase 06 Plan 01: Typed CSR lease Summary

**Parallel `TypedCsrLease` projects one labeled edge per live Fact (labels + support multiplicity) while untyped `CsrLease` object-only dedup and FF5 stay green.**

## Performance

- **Duration:** ~4 min
- **Started:** 2026-09-30T01:43:01Z
- **Completed:** 2026-09-30T01:47:00Z
- **Tasks:** 3/3
- **Files modified:** 5 (+ SUMMARY)

## Accomplishments

- Added `TypedEdge` (`relation`, `object`, `claim_id`, `fact_seq`) and `TypedCsrLease::from_fold` / `edges_out` beside unchanged `CsrLease` (D-T1, D-T2).
- Added `Runtime::typed_csr_lease_at` twin of `csr_lease_at` (D-T3).
- Named oracles `typed_csr_preserves_relation_labels_and_support_multiplicity` (CSR-01) and `untyped_csr_neighbor_set_and_ff5_still_hold` (CSR-02).
- Product CHANGELOG dated 2026-09-30; wave-close `kutha-gov ci` HIGH 0 (D-T5). GATE-01 needles remain on 06-02 (D-T6).

## Task Commits

1. **Task 1: End-to-end typed CSR preserves labels and support multiplicity** - `9a4d866` (feat)
2. **Task 2: Untyped neighbor-set and FF5 path still hold** - `f408082` (test)
3. **Task 3: Product changelog, cargo, and D-T5 wave-close trajectory** - `e644b4c` (docs)

## Files Created/Modified

- `crates/kutha-runtime/src/csr.rs` — TypedEdge + TypedCsrLease (untyped path frozen)
- `crates/kutha-runtime/src/quantum.rs` — `typed_csr_lease_at`
- `crates/kutha-runtime/src/lib.rs` — pub use TypedCsrLease, TypedEdge
- `crates/kutha-runtime/tests/m011_typed_csr.rs` — CSR-01 / CSR-02 oracles
- `CHANGELOG.md` — 2026-09-30 Product entry

## Decisions Made

Followed D-T1…D-T4, D-T7 from CONTEXT: separate typed lease; fact_seq on edges; no untyped rewrite; GATE-01 deferred to 06-02 (D-T6).

## Deviations from Plan

### Auto-fixed Issues

None - plan executed as written for product code.

**Note:** Plan verify used `cargo test --lib csr_drop_rebuild_and_seek -- --exact`, which matches zero tests (nested as `quantum::tests::csr_drop_rebuild_and_seek`). Verified with the full module path instead (Rule 3 — blocking verify filter).

## Trajectory (D-10 / D-T5)

1. **Commands:** `uv run kutha-gov ci`; `uv run kutha-gov explain trajectory`.
2. **Outcome:** `ci` exit 0; **HIGH 0**, **LOW 0** (29 checks); explain exit 0.
3. **Explain paraphrase:** Check `trajectory` confirms Active Milestone/Slice pointers exist on ROADMAP; authority none — harness does not accept ADRs or claim product readiness.
4. **Honesty:** Green governor is not ADR Accepted and not L_capability.

## Threat Flags

None beyond the plan register (T-06-01…T-06-05 mitigated by one-edge-per-Fact typed builder + frozen untyped path).

## Known Stubs

None.

## Self-Check: PASSED

- FOUND: crates/kutha-runtime/src/csr.rs (TypedEdge, TypedCsrLease)
- FOUND: crates/kutha-runtime/src/quantum.rs (typed_csr_lease_at)
- FOUND: crates/kutha-runtime/tests/m011_typed_csr.rs (both named tests)
- FOUND: CHANGELOG.md (2026-09-30 TypedCsrLease + both fn names)
- FOUND: commits 9a4d866, f408082, e644b4c
- FOUND: .kutha/STATE.md Active Slice S06 (unchanged)
- SKIPPED: .planning/STATE.md / ROADMAP.md updates (dispatch forbid)
