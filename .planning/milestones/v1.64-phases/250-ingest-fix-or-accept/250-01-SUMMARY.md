---
phase: 250-ingest-fix-or-accept
plan: 01
subsystem: benchmark
tags: [ingest, tradeoff, index]
requires: [249-01]
provides:
  - "accepted per-assert index write cost"
affects: []
actuals: {tokens: 0, tasks: 3, commits: 0}
tech-stack: {added: [], patterns: [accept-write-cost]}
key-files:
  created: []
  modified: [docs/architecture/benchmark-baseline.md, CHANGELOG.md]
key-decisions:
  - "Accept the per-assert append_indexed cost; do not patch product code."
  - "Use KUTHA_FOLD_INDEX_MODE=lazy|thin as the bounded opt-out; Eager remains default."
requirements-completed: [PF-06]
coverage:
  - id: D1
    description: "Explicit ingest tradeoff with cause, range, and opt-out."
    requirement: PF-06
    verification:
      - kind: manual
        ref: "docs/architecture/benchmark-baseline.md#2026-10-30-m171-ingest-bisect"
        status: pass
    human_judgment: false
---

# Phase 250 summary — ingest fix or accept

Accepted, not patched. `199807d` adds per-assert `by_event_id` / `by_delivery_key` maintenance and GraphFold indexes; the cost buys O(1) event/delivery lookup. The 5-sample table records parent -> bad +39.6% and old -> HEAD +18.5%. `027a2ce` already provides `FoldIndexMode::{Eager,Lazy}`; `KUTHA_FOLD_INDEX_MODE=lazy|thin` is the bounded write-thin opt-out. Eager remains default. No product code, Op, ADR, or CI change.
