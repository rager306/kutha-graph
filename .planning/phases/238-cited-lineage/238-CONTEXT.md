# Phase 238 — cited checkpoint lineage (M164 S01)

`checkpoint_prefix_behaviors` copies `behaviors` from every `StoreBlob` that deserializes as a `Snapshot`. `tt_anchors_from_events` reads a blob only when `Op::Checkpoint` cites its digest. An ordinary blob can be laundered into the next checkpoint.

## Locked decisions

1. Copy `Snapshot.behaviors` from a `StoreBlob` only when some `Op::Checkpoint` in the same prefix cites that digest. Mirror `tt_anchors_from_events`.
2. Still copy prefix `Op::Behavior` events. Those are log rows, not blob payload.
3. A cited blob with `behaviors_exact: false` contributes nothing. Open already rejects that blob; harvest must not be a second door.
4. No new Op. Release id stays `0.0.01`.

## RED

`crates/kutha-runtime/tests/b10_cited_lineage.rs`, test `ordinary_snapshot_blob_is_not_harvested`. Emit a digest-matching `StoreBlob` whose JSON is a Snapshot with `behaviors_exact: true` and one Behavior id that never occurred in the log. No `Op::Checkpoint` cites it. Seal a non-empty prefix and call `checkpoint_sealed_prefix`. After `store::open`, `snapshot().behaviors` does not contain that id. Today it does.

## Gates

`cargo test -p kutha-runtime --offline --test b10_cited_lineage`, then `cargo test -p kutha-runtime --offline --no-fail-fast`. One target. No version bump. `uv run kutha-gov precommit`.

## Stop

A real second checkpoint loses Horn rows that the first checkpoint stored, or the fix needs a new Op.
