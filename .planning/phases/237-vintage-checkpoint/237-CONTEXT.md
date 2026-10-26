# Phase 237 — vintage checkpoint lineage fails closed (M163 S01)

`Snapshot.behaviors` uses `serde(default)` and omits an empty vec. A blob from before that field is the same JSON as a no-Horn checkpoint, so open restores no Behavior rows.

## Locked decisions

1. Add `behaviors_exact: bool` on `Snapshot`. `#[serde(default)]` so a missing field is false. Always serialize it. Both constructors (`Runtime::snapshot`, the checkpoint blob in `store.rs`) set it true.
2. `checkpoint_snapshot_if_receipt_ok` returns `Err(())` when `behaviors_exact` is false, after the digest, fact_count, and fingerprint checks. That is the existing receipt error. No new `Op`, no new `RuntimeError` variant.
3. A marked blob with an empty `behaviors` list still opens. That is a real no-Horn checkpoint.
4. Release id stays `0.0.01`.

## RED

`crates/kutha-runtime/tests/b9_vintage_checkpoint.rs`, test `unmarked_checkpoint_blob_fails_open`. Build a normal checkpoint, strip `behaviors_exact` from the blob JSON, recompute the blob digest and the `Op::Checkpoint` digest so the digest check would pass, rewrite the log the way `b8_blob_digest` does. `store::open` is `Err` today it is `Ok`. A twin that keeps the mark opens.

## Gates

`cargo test -p kutha-runtime --offline --test b9_vintage_checkpoint`, then `cargo test -p kutha-runtime --offline --no-fail-fast`. One target. No version bump. `uv run kutha-gov precommit`.

## Stop

A checkpoint written by this slice fails its own open, or the fix needs a new Op.
