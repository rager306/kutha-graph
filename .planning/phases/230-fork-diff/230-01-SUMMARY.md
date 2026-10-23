---
phase: 230-fork-diff
plan: 01
subsystem: runtime-query
tags: [fork, diff_folds, FoldDiff]
requires: []
provides: ["diff_folds keyed/ranked/sorted fact-set query"]
affects: [230-02]
actuals: {tokens: 5802, tasks: 3, commits: 3}
plan_head_before: e566a2a5d7cc8793d55deaad9503ef45febb54c7
tech-stack: {added: [], patterns: [BTreeMap rank/sort, seq excluded from identity]}
key-files:
  created: [crates/kutha-runtime/tests/b4_fork_diff.rs]
  modified: [crates/kutha-runtime/src/fold_query.rs, crates/kutha-runtime/src/lib.rs, crates/kutha-runtime/src/fold.rs]
key-decisions:
  - "Fact Default so FactChange can Default (object is module-private)"
  - "apply_diff stays identity; totality RED for 02"
requirements-completed: [D-01, D-02, D-03]
duration: 29min
completed: 2026-10-23
status: complete
---
# Phase 230 Plan 01: fork diff
**Keyed/ranked/sorted `diff_folds` turns fold `a` into `b`; `seq` never participates.**
## Bench PRE
`taskset -c 0 env KUTHA_BENCH_N=10000 cargo bench -p kutha-runtime --bench baseline --offline` alone after waiting out `/root/prime-agent` cargo. Load before `2.88 3.79 4.09 3/1133 3460903`. **ingest_plain.n=97567248 ns**, **as_of.n=206975 ns**. After `3.61 3.82 4.08`. Exit 0, `validated yes`. Prime-agent not touched. Leaked `tmp/kutha-bench-plain-*` removed.
## RED (empty stub, before real `diff_folds`)
`cargo test -p kutha-runtime --offline --test b4_fork_diff -- --exact fork_diff_exact_on_counterfactual_fixture` exit **101**:
```
thread 'fork_diff_exact_on_counterfactual_fixture' (3479724) panicked at crates/kutha-runtime/tests/b4_fork_diff.rs:249:5:
post-cut B-C must be added
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 6 filtered out
```
Stub suite: 1 passed / 6 failed. **Passed:** `diff_folds_aa_is_empty` (empty stub is empty). **Failed:** mirror "distinct folds must produce a non-empty mirror pair"; exact "post-cut B-C must be added"; vs-fork `left: []` vs four post-cut mints; apply_diff identity ≠ live; equal-keys `left: 0 right: 2`; checkpoint "empty fork vs checkpointed live must add facts".
## GREEN
`fork_diff_exact` / `aa_empty` / `mirror` exit 0. Canonical sort; fingerprint/`fork_at`/`facts` untouched. Full `b4_fork_diff`: 6 passed, 1 failed.
## Plan 02 still RED
`apply_diff_reproduces_other_side_modulo_seq` exit 101 (identity helper by design).
## Deviations
Rule 3: `#[derive(Default)]` on `Fact` so `FactChange` can Default (`object` is module-private). rustfmt reordered fold_query imports; fingerprint body unchanged.
## Self-Check: PASSED
Files present: `fold_query.rs`, `lib.rs`, `fold.rs`, `b4_fork_diff.rs`, this SUMMARY. Commits `30afc37`, `a3358ab`, plus this feat commit.
