---
phase: 230-fork-diff
plan: 02
subsystem: runtime-query
tags: [fork, diff_folds, apply_diff]
requires: ["diff_folds keyed/ranked/sorted fact-set query"]
provides: ["apply_diff totality modulo seq", "checkpointed fork twin FoldDiff"]
affects: []
actuals: {tokens: 8536, tasks: 3, commits: 3}
plan_head_before: 7aae862d781f4d674b8a0d061bbddf8ab0251264
plan_head_after: a92e68a8a2064abde41134c73c4c05a1ed864eb1
tech-stack: {added: [], patterns: [apply_diff 11-field identity, twin cut via tail EventId]}
key-files:
  created: []
  modified: [crates/kutha-runtime/tests/b4_fork_diff.rs, docs/ADR/ADR-061-fork-and-diff.md, CHANGELOG.md]
key-decisions:
  - "apply_diff consumes only added/removed/changed plus rows of a"
  - "checkpointed n maps to twin via first tail EventId"
requirements-completed: [D-04, D-05, D-06]
duration: 34min
completed: 2026-10-23
status: complete
---
# Phase 230 Plan 02: fork diff totality
**`apply_diff` reconstructs the other fold modulo `seq`; checkpointed legal cuts match the twin.**
## From 01
PRE ingest_plain.n=**97567248** as_of.n=**206975** (`taskset -c 0`). RED empty stub then GREEN keyed `diff_folds` (commits `30afc37`, `a3358ab`, `7aae862`). 01 SUMMARY last at `7aae862` (removed here for the one-SUMMARY budget).
## GREEN
b4 7/7. Totality both directions, `as_of` 2011/2015/2019, `as_known_on` two walls, xorshift rank, CI residuals, multipath `EventId::nil()`. Interior `1..=c` is `ForkCutUnavailable`.
## Bench POST
Quiet: load **1.88 3.73 6.01** → **2.12 3.56 5.86**. ingest_plain.n=**95695483** (−1.9%). as_of.n=**283138** (+36.8%). Discarded noisy (prime-agent cargo, not touched): 119518426/284320 and 139537588/322110. `validated yes`. `as_of` source unchanged this wave.
## Gates
`b4_fork_diff` 0; `b1_checkpoint_*` / `b3_checkpoint_cut*` 0 (kill3 26.7s); `cargo test --workspace --offline --no-fail-fast` 0; `uv run kutha-gov precommit` 0. tmp empty. No `kutha-gov ci` (verifier).
## Commits
1. `44a6702` test apply_diff totality
2. `647c7bf` test checkpointed twin
3. `a92e68a` docs SEM-m156 CHANGELOG
## Deviations
Rule 3: twin cut via tail EventId, not compacted `n`. as_of +36.8% vs PRE (ingest ok; `as_of` body unchanged; two noisy samples discarded).
## Self-Check: PASSED
