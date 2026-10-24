---
phase: 232-fork-query
plan: 01
status: complete
completed: 2026-10-24
---
# 232-01: Runtime::diff and the fork-scan chain

Glue only. No Op, wire, fingerprint, Hops, Horn, planner, or version bump.

## RED

`cargo test -p kutha-runtime --offline --test b6_fork_query -- --exact runtime_diff_explains_scan_on_fork` **101**.

`error[E0599]: no method named diff found for struct kutha_runtime::Runtime` at five call sites in `tests/b6_fork_query.rs` (in-memory fork, checkpointed `fork_at(0)`, legal `n > c`).

## GREEN

`impl Runtime { pub fn diff(&self, other: &Runtime) -> FoldDiff }` in `fold_diff.rs` is `diff_folds(self.fold(), other.fold())`. `fold()` is the existing public accessor (`quantum.rs`). No new export: the method is on `Runtime`. `apply_diff` stays in the test.

Oracle `runtime_diff_explains_scan_on_fork`: ingest (A-B windowed + surviving D-E), `fork_at(cut)`, Retract then post-cut B-C. `fork.diff(&live)` equals `diff_folds`. Retract is `changed` (dead row stays); `removed` empty; Scan at 2011/2019 (same `QueryIr` both sides) matches sorted `as_of` and drops A-B on live at 2011. Checkpointed `fork_at(0)` and `n = c+1` match the copied twin; interior `1..=c` is `ForkCutUnavailable`.

`cargo test -p kutha-runtime --offline --test b6_fork_query` **0** (1/1, 0.77s). Then crate `--offline --no-fail-fast` and `uv run kutha-gov precommit` **0**. rustfmt `--edition 2024` on `fold_diff.rs` and the test only.

## Docs

CHANGELOG Product: fork then query. ADR-061 Evidence 2026-10-24 (glue shipped, GED open, Proposed). Honeycomb restage comment + evidence name; map/delivery/capability unchanged.

## Isolation

Did not edit `fold_query.rs`, `eval.rs`, `leapfrog.rs`, `fold.rs`, `VERSION`, Cargo/pyproject versions. One `target/`. No worktree. No bench A/B (hot files untouched).
