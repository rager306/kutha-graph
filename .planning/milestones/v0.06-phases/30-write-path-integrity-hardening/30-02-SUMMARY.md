---
phase: 30-write-path-integrity-hardening
plan: 02
status: complete
requirements-completed: [HARD-03, HARD-04]
executor_model: composer-2.5
subsystem: kutha-runtime tests
commits: 2
plan_head_before: 205ca1ea199bc7769a599d6a5d3716810fe4dfba
plan_head_after: 241b63a2a86f8b398fb588643f41864397a38901
tags: [dual-mode, facade, bench]
---

# Phase 30 Plan 02: Dual-mode + facade triples Summary

**One-liner:** Eager/Lazy FF5 retract/correct oracle and facade `assert_eq!` live triples; bench within +10% of Phase 27 after-lease.

## D-07 rename table

| Old expect | New expect |
|------------|------------|

*(none — named FF5 / fold-index oracles untouched)*

## Bench vs Phase 27 (cap +10%)

| key | Phase 27 | Phase 30 | vs P27 |
|-----|--------:|---------:|-------:|
| ingest_plain.n | 120267790 | 126962516 | +5.6% |
| as_of.n | 511233 | 288999 | −43.5% |

Single run `tmp/p30-bench.txt`; gate file `tmp/p30-bench-check.txt`. No rerun needed (under cap).

## Commits

1. `8945cd1` — `test(30-02): dual-mode FF5 retract correct claim oracle`
2. `241b63a` — `feat(30-02): facade exact triples and Phase 27 bench delta`

## Deviations

None. Lazy matched Eager without `fold_query.rs` changes. Facade pins include derive-sourced duplicate `inForceAs` rows at 2021 after correct.

## Self-Check: PASSED
