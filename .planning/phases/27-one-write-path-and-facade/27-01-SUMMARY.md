---
phase: 27-one-write-path-and-facade
plan: 01
subsystem: runtime
tags: [red, facade, admit, d-08]
requires:
  - phase: 26-dictionary-as-facts
    provides: emit_policy, EventId admissions
provides:
  - D-08 RED facade compile-fail
  - D-08 RED justification/action skip receipt
affects: [27-02]
actuals: { tokens: 1576, tasks: 2, commits: 2 }
tech-stack: { added: [], patterns: [D-08 RED first] }
key-files:
  created: [crates/kutha-runtime/tests/facade_statute.rs, crates/kutha-runtime/examples/statute_as_of.rs, crates/kutha-runtime/tests/m013_write_path.rs]
  modified: [CHANGELOG.md]
key-decisions:
  - "Facade RED imports only Runtime/RuntimeError and drives D-03 names"
  - "Admit RED compiles; fails because outcome_records do not grow after justification"
patterns-established: ["Phase 27 RED artifacts only; no Runtime split"]
requirements-completed: []
coverage:
  - id: D1
    description: "facade_statute and statute_as_of fail to compile (D-03 missing)"
    requirement: PIPE-04
    verification: [{ kind: integration, ref: "cargo test -p kutha-runtime --test facade_statute", status: fail }]
    human_judgment: false
  - id: D2
    description: "unknown_relation_rejected_for_assert_justification_and_action FAILED"
    requirement: PIPE-03
    verification: [{ kind: integration, ref: "cargo test -p kutha-runtime --test m013_write_path unknown_relation_rejected_for_assert_justification_and_action -- --exact", status: fail }]
    human_judgment: false
duration: 6min
completed: 2026-10-01
status: complete
plan_head_before: 5b51e3fd9cf586d12011a2e4605a629f0cd70b14
plan_head_after: 53b944fb1b6aa5b02cd524a5d784e2cc124031a7
---

# Phase 27 Plan 01: RED facade and unknown-relation admit Summary

**D-08 RED: facade-only statute consumer does not compile; justification/action skip Assert admit/receipt.**

## Accomplishments
- `statute_as_of_via_facade` and example `statute_as_of` fail with `error[` (no Runtime `assert`/`as_of`/`persist`/`open`)
- `unknown_relation_rejected_for_assert_justification_and_action` compiles and FAILED: `outcome_records` did not grow
- Named oracles and `.kutha/STATE.md` untouched; PIPE-03/04 stay open until 27-02

## Deviations from Plan
- Used `Runtime::default()` because current `new` takes `max_cascade`, so RED targets D-03 names

## Commits
- `26e10ae` test(27-01): RED facade statute compile
- `53b944f` test(27-01): RED unknown-relation admit for justification and action

## Self-Check: PASSED
