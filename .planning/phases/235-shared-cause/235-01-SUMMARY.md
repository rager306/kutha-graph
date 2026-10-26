---
phase: 235-shared-cause
plan: 01
status: complete
completed: 2026-10-25
---
# 235-01: unindex every cause of a shared head

Vec of distinct causes. No Op, fingerprint, or version bump.

## RED

`cargo test -p kutha-runtime --offline --lib shared_head_unindex_clears_every_cause` **101**.

`quantum::tests::shared_head_unindex_clears_every_cause` panicked at `first cause still lists the retracted shared head`. Two Behavior rows, same EventId, different `caused_by`, both indexed via `append_indexed`; Retract cleared only the last-wins cause.

## GREEN

`cause_of_behavior: IngestMap<EventId, Vec<EventId>>`. `record_cause_of_behavior` pushes a cause only if it is not already in the vec (`append_indexed` and `index_live_behavior_event`). `unindex_behavior` walks every stored cause and retains the id out of each `derived_by_cause` list. Hydrate constructors stayed `IngestMap::default()`. `by_event_id`, fingerprint, and Op unchanged.

`cargo test -p kutha-runtime --offline --lib shared_head_unindex_clears_every_cause` **0**. Then crate `--lib` **0**. `uv run kutha-gov precommit` **0**. rustfmt `--edition 2024` on `pipeline.rs` and `quantum_invariants.rs`. Hand-formatted `quantum.rs` (`mod tests`).

## Isolation

Did not edit `fold.rs`, `eval.rs`, `VERSION`, Cargo/pyproject versions. One `target/`. No worktree. No bench A/B.
