---
phase: 253-m003a-parser-subset
plan: 01
subsystem: query
tags: [m003a, gpml, parser, query-ir]
requires: []
provides:
  - "read-only GPML/Cypher subset parser"
  - "VocabBinder intern/allowlist implementation"
affects: [254-m003a-differential-falsifier]
actuals: {tokens: 0, tasks: 5, commits: 0}
tech-stack: {added: [], patterns: [hand-rolled-parser, fail-closed-vocabulary]}
key-files:
  created: [crates/kutha-runtime/src/gpml.rs, crates/kutha-runtime/tests/m003a_gpml.rs]
  modified: [crates/kutha-runtime/src/lib.rs]
key-decisions:
  - "Repeated same-relation variable length uses QueryIr compact len=1; per-hop relations remain per-hop."
  - "Runtime binder checks FF6 allowlist before dictionary id and never interns."
requirements-completed: [GPML-01]
coverage:
  - id: D1
    description: "Read-only parser subset to typed QueryIr."
    requirement: GPML-01
    verification:
      - kind: manual
        ref: "crates/kutha-runtime/tests/m003a_gpml.rs"
        status: pass
    human_judgment: false
---

# Phase 253 summary — M003a parser subset

Added `gpml.rs` lexer/parser for the read-only M003a subset: node/relationship patterns, fixed hops and bounded variable length, start `name` filter, typed `AS OF`, optional `TT`, `RETURN`, and `LIMIT`. Added `VocabBinder`; `RuntimeBinder` resolves terms through dictionary lookup and relations through FF6 allowlist + dictionary, never interning. Mapped output remains existing `QueryIr`; no writes, aggregation, path functions, planner, HNSW, or legal pack. Seven focused tests pass.
