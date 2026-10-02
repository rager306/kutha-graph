---
phase: 38-rule-evaluator
plan: 02
subsystem: rules
tags: [Horn, n-ary, retract]
requires:
  - phase: 38-rule-evaluator
    provides: Horn eval and genesis inverse
provides:
  - n-ary Horn named oracle
  - premise retract drops Horn-derived support
affects: [kutha-runtime]
actuals:
  tokens: 792
  tasks: 1
  commits: 2
plan_head_before: 2b378207ff78a34db1c5641be84deea34d0676ac
plan_head_after: e2bdd760cb61a4f710e34fe6ed2cffb780faedfa
tech-stack:
  added: []
  patterns: [n-ary Horn unify; Horn-only derived Retract]
key-files:
  created:
    - crates/kutha-runtime/tests/m016_rule_eval.rs
  modified:
    - CHANGELOG.md
key-decisions:
  - "D-03: retract of a Horn premise Retracts caused_by derived Behaviors"
requirements-completed: [RULE-02]
duration: 4min
completed: 2026-10-02
status: complete
---

# Phase 38 Plan 02: n-ary Horn Summary

Two-premise `relatedTo` Horn fires only with both premises live; retracting one drops the derived support.

## Deviations from Plan

None — retract maintenance landed in 38-01 (Rule 1); this plan adds the n-ary oracle.
