# Phase 232 — fork then query (M158 S01)

Scout 2026-10-24. Discuss skipped. Decisions below are locked.

## Goal

One caller-facing step from a fork to a fact-set diff, proven against `execute(Scan)` on both sides. Not a new engine, not Hops, not Horn, not a planner.

## Locked decisions

1. `Runtime::diff(&self, other: &Runtime) -> FoldDiff` in `fold_diff.rs`, body `diff_folds(self.fold(), other.fold())`. `fold()` is the existing accessor. No new Op, no serde, no fingerprint change. Do not edit `fold_query.rs`, `eval.rs`, `leapfrog.rs`, `fold.rs`.
2. Diff domain stays `facts()` including dead rows. Scan domain stays live product triples. The test shows a Retract or CorrectInterval as `changed` and a Scan bag that drops the dead triple. Do not treat `added`/`removed` as the Scan delta.
3. Fork cut `n` is a log prefix, not a clock. The same `QueryIr` is run on both runtimes.
4. Checkpoint: one legal cut (`fork_at(0)` or `n > c`) diffs like the plain twin. An interior cut stays `ForkCutUnavailable`. Checkpoint stays `store::checkpoint_sealed_prefix`.
5. `apply_diff` stays a test helper. Do not put it in `src`.
6. Release id stays `0.0.01`. Do not edit `VERSION`, `Cargo.toml` version, or `pyproject.toml` version.

## RED

`crates/kutha-runtime/tests/b6_fork_query.rs`, test `runtime_diff_explains_scan_on_fork`. It must fail while `Runtime::diff` is missing, then pass. Bags must be nonempty.

## Gates

Filtered cargo only (`-p kutha-runtime --test b6_fork_query`, then `--offline --no-fail-fast` once). One `target/`. No worktree. `uv run kutha-gov precommit` at the end. Workspace `ci` is the verifier's job. Bench: `fold_query.rs` and ingest paths stay untouched, so no A/B unless a hot file changes; if one changes, stop and report.

## Stop

New Op, wire change, fingerprint change, Hops or Horn claim, version bump, or a design conflict with ADR-000 D1–D10.
