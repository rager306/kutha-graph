---
phase: 255-gate-gb-fixture-parity
plan: 01
subsystem: query
tags: [gate-gb, gpml, parity, fixtures]
requires: []
provides:
  - "Gate GB parity across FF5/B3/eCFR"
affects: [256-gate-gb-closure-decision]
actuals: {tokens: 0, tasks: 5, commits: 0}
tech-stack: {added: [], patterns: [fixture-corpus-parity]}
key-files:
  created: [crates/kutha-runtime/tests/m174_gate_gb.rs]
  modified: [CHANGELOG.md]
key-decisions:
  - "Gate GB uses the existing fixture corpus and typed facade as oracle."
  - "No parser/QueryIr/fold semantics change."
requirements-completed: [GB-01]
coverage:
  - id: D1
    description: "GPML AS OF parity across FF5/B3/eCFR."
    requirement: GB-01
    verification:
      - kind: manual
        ref: "crates/kutha-runtime/tests/m174_gate_gb.rs"
        status: pass
    human_judgment: false
---

# Phase 255 summary — Gate GB fixture parity

Added `m174_gate_gb.rs`: FF5 vt 2015/2020 GPML-vs-facade parity; B3 every layer-0 start at 1/2 hops; M167 eCFR 3 sections x 7 cuts GPML-vs-facade parity; unknown relation/term fail closed. Targeted test 2/2 passed. No parser/QueryIr/fold semantics change.
