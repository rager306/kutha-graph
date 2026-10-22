---
phase: 228-checkpoint-receipt
plan: 01
subsystem: runtime-store
tags: [checkpoint, Op, fingerprint, bench]
requires: []
provides:
  - Op::Checkpoint wire + compile-only store::checkpoint_sealed_prefix
affects: [228-02]
actuals:
  tokens: 0
  tasks: 1
  commits: 0
plan_head_before: 0770da57864c378672fc8c762a25cbf7d301654d
plan_head_after: pending
tech-stack:
  added: []
  patterns: [SelfCommit meta-op, reject_forged_meta]
key-files:
  created: []
  modified: []
key-decisions:
  - "PRE recorded before any crates/ edit"
requirements-completed: [D-01, D-02, D-03, D-04, D-05]
duration: pending
completed: 2026-10-22
status: in-progress
---
# Phase 228 Plan 01: checkpoint receipt Op wire Summary

**PRE baseline recorded; RED and Op::Checkpoint still pending this plan.**

## Bench PRE (D-05)

Host: 12 cores; benches ran alone. Load start 3.43 3.13 2.99 (2026-10-22T15:41:46Z); end 4.93 3.87 3.28. `KUTHA_BENCH_N=10000 cargo bench -p kutha-runtime --bench baseline --offline` exit 0.

- `ingest_plain.n` **144924496 ns**
- `as_of.n` **394795 ns**
- also: ingest_plain.2n 238350985; ingest_plain.4n 449478328; as_of.2n 752982; as_of.4n 1277359; fold_index_mode eager

## RED

Pending task 2.

## Deviations from Plan

None yet.
