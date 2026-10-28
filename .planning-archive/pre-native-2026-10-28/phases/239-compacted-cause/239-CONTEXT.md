# Phase 239 — compacted Horn cause must support the head (M165 S01)

After a checkpoint, Horn rows live in `Snapshot.behaviors`, not on the log. `replay_check` applies `BrokenLineage` only to log `Op::Behavior`. Fingerprint walks `fold.facts`, so a rewritten `caused_by` plus a recomputed blob digest still opens. Retract of the real premise then misses the head.

## Locked decisions

1. In `replay_check`, each compacted Horn behavior must have a `search_all` hit that covers the head window and whose `premise_ids` contain `caused_by`.
2. No such hit, or a missing rule blob: `ExecutionReplayFailed`. A hit exists but `caused_by` is not in it: `BrokenLineage`. Do not use the log `seen` set. An honest prefix assert is a restored fact.
3. Do not change `horn_behavior_reproducible` for log rows. Do not change the fingerprint. No new Op. Release id stays `0.0.01`.
4. Named / non-Horn compacted bodies stay on the existing skip path.

## RED

`crates/kutha-runtime/tests/b11_compacted_cause.rs`.

- `nil_caused_by_in_cited_blob_fails_open`: honest Horn checkpoint, set `behaviors[0].caused_by` to nil, keep `behaviors_exact` and the fold, recompute the blob digest and `Op::Checkpoint.blob_digest`. `store::open` is `Ok` today. It must be `Err`.
- `swapped_caused_by_in_cited_blob_fails_open`: same, but `caused_by` becomes another assert id that is in the fold and is not a premise of this Horn.

An unmodified checkpoint of the same fixture still opens.

## Gates

`cargo test -p kutha-runtime --offline --test b11_compacted_cause`, then `cargo test -p kutha-runtime --offline --no-fail-fast`. One target. No version bump. `uv run kutha-gov precommit`.

## Stop

An honest Horn checkpoint fails open, or the fix needs a new Op.
