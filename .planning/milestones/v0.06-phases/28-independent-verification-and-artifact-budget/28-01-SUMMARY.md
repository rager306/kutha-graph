---
phase: 28-independent-verification-and-artifact-budget
plan: 01
subsystem: harness
tags: [governor, glob_max_lines, artifact-budget, verification-independent]
requires:
  - phase: 27-one-write-path-and-facade
    provides: v0.06 VERIFICATION files to conform
provides:
  - glob_max_lines kind
  - artifact-budget (PLAN/SUMMARY/VERIFICATION/CONTEXT)
  - verification-independent check
affects: [28-02]
actuals: { tokens: 15025, tasks: 3, commits: 3 }
plan_head_before: 8b6701aa9488f51bc4f78ee3e9dd7e1c7439c7fd
plan_head_after: 53660bbaf6a832f414fa253b6692fb593fba48a2
tech-stack: { added: [glob_max_lines], patterns: [YAML check after invariant; derived oversize mutation] }
key-files:
  created: []
  modified: [scripts/kutha_gov/kinds.py, scripts/kutha_gov/selftest.py, scripts/tests/test_kutha_gov.py, .kutha/META.md, .kutha/dictionaries/checks.yaml, .kutha/dictionaries/invariants.yaml, CHANGELOG.md]
key-decisions:
  - "min_phase filters .planning/phases/NN-* only; max_files_per_dir -1 unlimited"
  - "glob_absent needles are literal -PLAN.md and -SUMMARY.md"
requirements-completed: [PROC-01]
coverage:
  - { id: D-02, description: "glob_max_lines budgets from phase 22", requirement: PROC-01, verification: [{ kind: unit, ref: "GlobMaxLinesTests + selftest artifact-budget", status: pass }], human_judgment: false }
  - { id: D-05, description: "22-27 VERIFICATION independent + independence check", requirement: PROC-02, verification: [{ kind: unit, ref: "precommit verification-independent", status: pass }], human_judgment: false }
duration: 7min
completed: 2026-10-02
status: complete
---

# Phase 28 Plan 01: glob_max_lines, artifact budget, independent verification Summary

**Governor kind `glob_max_lines` plus `artifact-budget` and `verification-independent`; 22–27 VERIFICATION and 27-02-SUMMARY at v0.06 line budgets.**

## Task Commits

1. **glob_max_lines + PLAN artifact-budget** — `f94ec09` feat
2. **Conform 22-27 VERIFICATION and 27-02-SUMMARY** — `dd41a59` docs
3. **Remaining budgets + independence** — `53660bb` feat

## Deviations from Plan

- TDD_MODE false; plan named one commit per task. Tests written first; no separate `test(28-01)` RED commits.
- Task 3 `kutha-gov py` failed on unrelated dirty `scripts/tests/test_fsm.py`; re-ran against HEAD of that file (green). Not committed.
- 22-VERIFICATION already ≤60 with required keys; task 2 commit omitted it.
- PROC-02 not completed: 28-VERIFICATION is D-08 post-execute (independent model).

## TDD Gate Compliance

Plan `type: execute`, `workflow.tdd_mode: false`. Tasks 1 and 3 had `tdd="true"`; RED evidence on unknown-kind, then GREEN in the named feat commits.

## Self-Check: PASSED
