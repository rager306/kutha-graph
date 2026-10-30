---
phase: 254-m003a-differential-falsifier
plan: 01
subsystem: query
tags: [m003a, gpml, differential, ff5]
requires: [253-01]
provides:
  - "B3 render/parse parity"
  - "FF5 GPML AS OF equals typed facade"
affects: []
actuals: {tokens: 0, tasks: 5, commits: 0}
tech-stack: {added: [], patterns: [differential-falsifier]}
key-files:
  created: [crates/kutha-runtime/tests/m003a_gpml_differential.rs]
  modified: [CHANGELOG.md]
key-decisions:
  - "Unknown vocabulary and out-of-scope shapes fail closed."
  - "Repeated same-relation bounded hops use compact QueryIr len=1."
requirements-completed: [GPML-02]
coverage:
  - id: D1
    description: "Differential parser/IR parity and FF5 facade parity."
    requirement: GPML-02
    verification:
      - kind: manual
        ref: "crates/kutha-runtime/tests/m003a_gpml_differential.rs"
        status: pass
    human_judgment: false
---

# Phase 254 summary — M003a differential falsifier

Added a mixed repeated/per-hop chain test that fails closed at prepare with `RelsShape`, plus differential tests for the B3 layered workload (`parse(render(workload)) == typed IR` for one and two hops over every layer-0 start) and FF5 (`GPML AS OF` result equals typed facade at 2015 and 2020). Added typed TT/limit preservation and fail-closed tests for unknown terms/relations, unsupported bounded hops, and writes. Five differential tests pass; workspace 741 passed / 0 failed.
