---
phase: 120-scaling-bench
plan: 01
---

# Phase 120 Plan 01 — Summary

## What shipped

- #15/#25 bench RSS + as_of validation (`baseline.rs`).
- #16/#17/#19/#27 hot-path indexes (derived_by_cause, facts_by_relation,
  append suffix, pin maps).
- Oracles: `m054_scaling_bench` (7). Matrix SEM-m054-scale-* killed.

## Residuals

- RSS still in-process. Lazy cold relation index falls back. #6 gate.
  Honeycomb Proposed.
