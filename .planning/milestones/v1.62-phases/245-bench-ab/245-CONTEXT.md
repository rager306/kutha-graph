---
phase: "245"
name: "bench-ab"
created: 2026-10-29
---

# Phase 245: Bench A/B — Context

## Decisions

1. Old = `cdf1942` (2026-10-01 baseline note commit); HEAD = current tree.
2. Use a separate worktree and `CARGO_TARGET_DIR` for the old commit so targets do not mix.
3. Interleave Old/HEAD at least twice, `KUTHA_BENCH_N=10000`, same idle host; record medians, not a single run.
4. Compare only keys present in both (old baseline predates `persist_append.n`).
5. This is not B3 and not a competitor gate.

## Discretion Areas

Number of pairs (2 or 3) and whether to use `taskset`; state the method either way.

## Deferred Ideas

Competitor baseline and HNSW/planner only after a controlled number.
