---
phase: 48-oracle-tightening
plan: 01
subsystem: testing
tags: [oracle, I-F15, I-F16, GATE-01, BENCH-controller]
requires:
  - phase: 47-horn-determinism-closure
    provides: horn_correct_* killed
provides:
  - ORACLE-if15/if16 killed
  - GATE-01 on BENCH-controller
affects: [test-matrix, checks, fsm, honeycomb]
actuals:
  tokens: 3353
  tasks: 3
  commits: 3
  plan_head_before: ea4ef4ea2227fe29594b6ed0f0aea997427e8c73
  plan_head_after: e093d1cf8c24629eef14d7402e919d1d6574d5f5
tech-stack:
  added: []
  patterns: [document BENCH-controller; no new Check class]
key-files:
  created: []
  modified: [docs/architecture/test-matrix.yaml, .kutha/dictionaries/checks.yaml, .kutha/dictionaries/fsm.yaml, .kutha/dictionaries/honeycomb.yaml, CHANGELOG.md]
key-decisions:
  - "I-F15 stays extra_relatedto_horn_visits_constant_in_n; no extra facts() walk"
  - "GATE-01 is gsd-auto-contract bench gate plus BENCH-controller; observe_cargo stays workspace test"
requirements-completed: [ORACLE-01, GATE-01]
coverage:
  - {id: D1, description: I-F15 visit bound pin, requirement: ORACLE-01, verification: [{kind: unit, ref: crates/kutha-runtime/tests/m017_ingest_scaling.rs#extra_relatedto_horn_visits_constant_in_n, status: pass}], human_judgment: false}
  - {id: D2, description: I-F16 horn_correct pins, requirement: ORACLE-01, verification: [{kind: unit, ref: crates/kutha-runtime/tests/m019_sem_meta.rs#horn_correct_derived_not_live_after_premise_correct, status: pass}], human_judgment: false}
  - {id: D3, description: GATE-01 recorded as BENCH-controller, requirement: GATE-01, verification: [{kind: other, ref: "uv run kutha-gov ci (0 HIGH; extra_relatedto and horn_correct observed)", status: pass}], human_judgment: false}
duration: 101min
completed: 2026-10-06
status: complete
---
# Phase 48 Plan 01: Oracle tightening Summary

**I-F15 visit-bound and I-F16 horn_correct pins recorded; GATE-01 is BENCH-controller, honeycomb Proposed**

## Task Commits
1. `26ed928` docs(48-01): pin I-F15 visit-bound oracle
2. `f12750d` docs(48-01): pin I-F16 horn_correct oracles
3. `e093d1c` docs(48-01): record GATE-01 BENCH-controller planes

## Deviations
- Rule 3: invariants.yaml claim text with checks.yaml (docs-coupling ledger).
- Rule 3: repaired `47-VERIFICATION.md` `read_only_inputs` (verification-independent).
- Rule 1: `emit_allow("derive_pq")` in `m016_rule_ast.rs` so workspace observe is green after CASC-01.

## D-SCOPE
Honeycomb map **Proposed**. `.kutha/STATE.md` untouched. No `48-VERIFICATION.md`. DIFF-01 not reopened. FF5 unchanged. `kutha-gov ci` 0 HIGH (1 LOW trajectory).

## Self-Check: PASSED
