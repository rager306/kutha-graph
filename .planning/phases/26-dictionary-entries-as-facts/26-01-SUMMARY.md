---
phase: 26-dictionary-entries-as-facts
plan: 01
subsystem: testing
tags: [dictionary-facts, as_of, RED, D-08]
requires:
  - { phase: 25-genesis-log, provides: genesis AllowRelation + admit fold-only }
provides: [D-08 RED as_of oracles for policy/admission/allows]
affects: [26-02]
actuals: { tokens: 1244, tasks: 2, commits: 2 }
plan_head_before: 82f1f493c0d4dc5752401ee410dba672274984dd
plan_head_after: 2858675302ec86a3dc0fbcf75e1ca6be293e189d
tech-stack: { added: [], patterns: [RED as_of relation walk; copy leased_pin_and_derive] }
key-files: { created: [crates/kutha-runtime/tests/m013_dictionary_facts.rs], modified: [CHANGELOG.md] }
key-decisions:
  - "D-08 RED: as_of walks facts only; policy/admission/allows stay in side tables until 26-02"
patterns-established: ["26-02 greens these tests; never edit .kutha/STATE.md; no Op migration in 26-01"]
requirements-completed: [SUB-01, SUB-02, SUB-03, SUB-04]
coverage:
  - id: D1
    description: "D-08 RED admission+policy visible on as_of after later PinPolicy"
    requirement: SUB-01
    verification: [{ kind: unit, ref: "crates/kutha-runtime/tests/m013_dictionary_facts.rs#admission_and_policy_visible_on_as_of_after_later_pin", status: fail }]
    human_judgment: true
    rationale: "RED cargo FAILED is acceptance; 26-02 greens."
  - id: D2
    description: "D-08 RED genesis allow visible on as_of(0)"
    requirement: SUB-02
    verification: [{ kind: unit, ref: "crates/kutha-runtime/tests/m013_dictionary_facts.rs#genesis_allow_visible_on_as_of", status: fail }]
    human_judgment: true
    rationale: "RED cargo FAILED is acceptance; 26-02 greens."
duration: 7min
completed: 2026-10-01
status: complete
---

# Phase 26 Plan 01: RED dictionary entries as facts Summary

**Failing `m013_dictionary_facts` tests prove `as_of` does not see `kutha:policy`, `kutha:admission`, or genesis `kutha:allows` (fold side tables).**

## Performance

- **Duration:** 7 min · **Started:** 2026-10-01T15:54:00Z · **Completed:** 2026-10-01T16:01:29Z · **Tasks:** 2 · **Files:** 2

## Accomplishments

- `admission_and_policy_visible_on_as_of_after_later_pin` FAILED: no `kutha:policy` triple after later `PinPolicy` (cut A=19).
- `genesis_allow_visible_on_as_of` FAILED: `as_of(0)` has no `kutha:allows` relation.
- No Op migration. `.kutha/STATE.md` untouched. Evidence: `tmp/p26-red.txt`, `tmp/p26-red2.txt`.

## Task Commits

1. **RED admission+policy on as_of** — `2ba6250` (test)
2. **RED genesis allow on as_of** — `2858675` (test)

## Decisions / Deviations

None — plan executed as written. CBM: `fold.rs`/`quantum.rs` metadata_changed; new test not_tracked; no `index_repository`. Regression: `ff5_legal_pit` + `m012_admission_facts` + `m013_genesis_log` ok (full `cargo test` would include this RED file). Ready for 26-02.

## Self-Check: PASSED
