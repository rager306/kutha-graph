---
phase: 230-fork-diff
plan: 02
subsystem: runtime-query
tags: [fork, diff_folds, apply_diff]
requires: ["diff_folds keyed/ranked/sorted fact-set query"]
provides: ["apply_diff totality modulo seq", "checkpointed fork twin FoldDiff"]
affects: []
actuals: {tokens: 8536, tasks: 3, commits: 4}
plan_head_before: 7aae862d781f4d674b8a0d061bbddf8ab0251264
plan_head_after: a92e68a8a2064abde41134c73c4c05a1ed864eb1
tech-stack: {added: [], patterns: [apply_diff 11-field identity, twin cut via tail EventId]}
key-files:
  created: [crates/kutha-runtime/src/fold_diff.rs]
  modified: [crates/kutha-runtime/src/fold.rs, crates/kutha-runtime/src/fold_query.rs, crates/kutha-runtime/src/lib.rs, crates/kutha-runtime/tests/b4_fork_diff.rs, docs/ADR/ADR-061-fork-and-diff.md, CHANGELOG.md]
key-decisions:
  - "apply_diff consumes only added/removed/changed plus rows of a"
  - "checkpointed n maps to twin via first tail EventId"
  - "diff lives in fold_diff.rs; fold_query.rs matches 791df68"
requirements-completed: [D-04, D-05, D-06]
duration: 34min
completed: 2026-10-23
status: complete
---
# Phase 230 Plan 02: fork diff totality
**`apply_diff` reconstructs the other fold modulo `seq`; checkpointed legal cuts match the twin.**
## From 01
RED empty stub then GREEN keyed `diff_folds` (`30afc37`, `a3358ab`, `7aae862`).
## GREEN
b4 7/7. Totality both directions, `as_of` 2011/2015/2019, `as_known_on` two walls, xorshift rank, CI residuals, multipath `EventId::nil()`. Interior `1..=c` is `ForkCutUnavailable`.
## Isolation
`FoldDiff` / `FactChange` / `diff_folds` / `GraphFold::diff` moved to `fold_diff.rs` (same `lib.rs` re-exports). `fold_query.rs` byte-identical to 791df68. `Fact` drops `Default`; `Fact::blank` is crate-private for `FactChange::default`. Identity/`canon_cmp` compare `Option<u64>` for `valid_to`/`invalidated_at` (`None` > any `Some`; not `u64::MAX`).
## Bench A/B
Replaces single-sample POST. BASE=791df68 vs HEAD, 9 alternating pairs, `taskset -c 0`, `KUTHA_BENCH_N=10000 <bin> --bench`. PRE (BASE median) ingest_plain.n=**120198466** as_of.n=**264152**. POST (HEAD median) **116451919** (ratio 0.969) / **267389** (ratio 1.012). Min 95486772 / 99172304 and 234027 / 251208. No >3× outliers. 1-min load 3.21–6.06. Verdict PASS.
## Gates
`cargo test -p kutha-runtime` and `--workspace --offline --no-fail-fast` 0; `uv run kutha-gov precommit` 0. No clippy in precommit. tmp empty after cleanup. No `kutha-gov ci` (verifier).
## Commits
1. `44a6702` test apply_diff totality
2. `647c7bf` test checkpointed twin
3. `a92e68a` docs SEM-m156 CHANGELOG
4. this wave: isolate `diff_folds` + A/B bench numbers
## Deviations
Rule 3: twin cut via tail EventId, not compacted `n`. Single-sample as_of +36.8% was order/host noise; A/B median ratio 1.012.
## Self-Check: PASSED
