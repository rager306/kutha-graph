# Phase 235 — every cause of a shared head (M161 S01)

`Event::stable_derived` ignores `caused_by`, so two Horn paths to the same head share an EventId. `derived_by_cause` pushes both. `cause_of_behavior` stores one `EventId`, so the second insert wins. `unindex_behavior` then removes the head from only that cause.

## Locked decisions

1. `cause_of_behavior` becomes `IngestMap<EventId, Vec<EventId>>`. Each distinct `caused_by` is pushed once. Both `append_indexed` and `index_live_behavior_event` use that.
2. `unindex_behavior` removes the id from `derived_by_cause` under every stored cause, then drops the vec.
3. Do not change `by_event_id`, fingerprint, or Op. Release id stays `0.0.01`.
4. A single-cause head still unindexes that one cause.

## RED

A `#[cfg(test)]` test in the runtime crate (the indexes are `pub(crate)`; integration tests cannot see them). Name: `shared_head_unindex_clears_every_cause`. Append two Behavior rows with the same EventId and different `caused_by`, then unindex that id. Both `derived_by_cause` lists must not contain it. Before the fix the first cause still lists it.

## Gates

`cargo test -p kutha-runtime --offline --lib shared_head_unindex_clears_every_cause`, then `cargo test -p kutha-runtime --offline --lib`. One target. No workspace build. No version bump. `uv run kutha-gov precommit`.

## Stop

The type change forces a new Op, or a single-cause retract stops clearing `derived_by_cause`.
