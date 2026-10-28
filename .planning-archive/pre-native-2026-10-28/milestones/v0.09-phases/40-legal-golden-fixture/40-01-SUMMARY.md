---
phase: 40-legal-golden-fixture
plan: 01
subsystem: time
tags: [FIXG-01, ADR-013, sunset]
requires:
  - phase: 39-policy-and-replay
    provides: RULE-03/04 closed
provides:
  - C01 fail-closed dated ADR-013 Evidence
affects: [docs/ADR, kutha-runtime tests]
actuals:
  tokens: 2144
  tasks: 1
  commits: 1
plan_head_before: 052b5a92864092d80705c24878272025d56e4509
plan_head_after: f0a5c61148d0b4b27e18d5cddc961c53b49d6b68
tech-stack:
  added: []
  patterns: [fail-closed golden fixture verdict]
key-files:
  created:
    - crates/kutha-runtime/tests/m016_golden_fixture.rs
  modified:
    - docs/ADR/ADR-013-bitemporal-facts-invalidation.md
    - CHANGELOG.md
key-decisions:
  - "D-03: C01 sunset inexpressible as Horn; stop before five clocks"
requirements-completed: [FIXG-01]
duration: 8min
completed: 2026-10-02
status: complete
---

# Phase 40 Plan 01: Legal golden fixture Summary

C01 sunset cannot be a Horn atom; ADR-013 Evidence records a dated fail-closed verdict. No fifth clock.

## Deviations from Plan

None — fail-closed path was the locked D-03.

## Self-Check: PASSED
