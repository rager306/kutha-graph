---
phase: 236-cause-once
plan: 01
status: complete
completed: 2026-10-26
---
# 236-01: push a head once per cause

`derived_by_cause` stores a Behavior id once. No Op, fingerprint, or version bump.

## RED

`cargo test -p kutha-runtime --offline --lib derived_by_cause_lists_a_head_once` **101**.

`quantum::tests::derived_by_cause_lists_a_head_once` panicked at `reverse list must store the head once, got [00000000-0000-0000-0000-000000000091, 00000000-0000-0000-0000-000000000091]` (`left: 2`, `right: 1`). Same Behavior EventId indexed twice under one `caused_by` via `append_indexed`.

## GREEN

`record_derived_by_cause` pushes the head only if it is not already in that cause's vec. Both `append_indexed` and `index_live_behavior_event` call it. `cause_of_behavior`, `by_event_id`, fingerprint, and Op unchanged. M161 tests stayed green.

`cargo test -p kutha-runtime --offline --lib derived_by_cause_lists_a_head_once` **0**. Then crate `--lib` **0** (29 passed). `uv run kutha-gov precommit` **0**. rustfmt `--edition 2024` on `pipeline.rs` and `quantum_invariants.rs`. Did not edit `quantum.rs` (`mod tests`).

## Isolation

Did not edit `fold.rs`, `eval.rs`, `VERSION`, Cargo/pyproject versions. One `target/`. No worktree. No bench A/B.
