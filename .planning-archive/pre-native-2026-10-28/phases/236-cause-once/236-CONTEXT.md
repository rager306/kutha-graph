# Phase 236 — one head per cause (M162 S01)

M161 made `cause_of_behavior` a vec of distinct causes. `derived_by_cause` still `push`es on every index of the same `(caused_by, event id)` in both `append_indexed` and `index_live_behavior_event`. Unindex `retain`s every copy, so retract ends clean. The live list is still wrong.

## Locked decisions

1. One helper pushes `event id` onto `derived_by_cause[caused_by]` only when it is not already there. Both call sites use it.
2. Do not change `cause_of_behavior`, `by_event_id`, fingerprint, or Op. Release id stays `0.0.01`.
3. Two different causes still each list the head once.

## RED

`#[cfg(test)]` test `derived_by_cause_lists_a_head_once` in the runtime crate. Append the same Behavior under the same cause twice. The reverse list length for that cause is 1, and it contains the head. Before the fix the length is 2.

## Gates

`cargo test -p kutha-runtime --offline --lib derived_by_cause_lists_a_head_once`, then `--lib`. One target. No workspace build. No version bump. `uv run kutha-gov precommit`.

## Stop

Dedup needs a new Op, or a two-cause head loses one cause.
