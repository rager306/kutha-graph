---
phase: 113-cascade-atomicity
plan: 01
---

# Phase 113 Plan 01 — Summary

## What shipped

- `commit_user_cascade` rolls back via `fork_at` on later-hop `rule_follow_ons` / admit Err.
- Oracles `m052_cascade_atomic`: unknown head `ghostRel` leaves log/fold/outcomes unchanged; inverse_knows still writes `QuantumOutcome`.
- Matrix `SEM-m052-cascade-atomic` (owner `M052-S02`). Budget stays 76.

## Bench

Three `--quick` runs (`KUTHA_BENCH_N=10000`): median ingest ≈111.4M / as_of ≈294.1k vs Phase 111 ≈116.2M / ≈272.7k.

## Residuals

- Horn VT/lineage #2–#5 out of scope. Honeycomb Proposed.
