---
phase: 230-fork-diff
plan: 01
subsystem: runtime-query
tags: [fork, diff_folds, FoldDiff]
requires: []
provides: ["diff_folds keyed/ranked/sorted fact-set query"]
affects: [230-02]
actuals: {tokens: 0, tasks: 1, commits: 0}
tech-stack: {added: [], patterns: [canonical rank/sort, no seq in identity]}
key-files: {created: [], modified: []}
key-decisions: ["PRE recorded before crate edits; foreign cargo waited out"]
requirements-completed: []
duration: 0min
completed: 2026-10-23
status: in-progress
---
# Phase 230 Plan 01: fork diff (in progress)

## Bench PRE (KUTHA_BENCH_N=10000, taskset -c 0, host alone)

- **ingest_plain.n:** 97567248 ns
- **as_of.n:** 206975 ns
- **/proc/loadavg before:** `2.88 3.79 4.09 3/1133 3460903`
- **/proc/loadavg after:** `3.61 3.82 4.08 5/1152 3463799`
- **Foreign `/root/prime-agent` cargo:** present at first attempt (`cargo test -p pa-graph` / `pa-core`). Our contaminated bench was killed; waited until `NO_CARGO_COMPILE`. PRE rerun 2026-10-23T10:06:58Z–10:08:16Z, exit 0, `validated yes`. Prime-agent not touched. Leaked `tmp/kutha-bench-plain-*` removed after the run.
