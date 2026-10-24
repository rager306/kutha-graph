# Phase 233 — missing Horn blob fails closed (M159 S01)

Source: `eval.rs` `horn_behavior_reproducible`. The comment says missing blob fails closed (RULE-04). The branch `blobs.get` miss returns `!rule_version.is_empty()`, so a nonempty pin with no blob is success. `replay_check` (`provenance.rs`) then accepts the Behavior.

## Locked decisions

1. On a missing blob, return `false`. Empty pin stays fail-closed. A blob that is present and is not Horn still returns true (named body skip). A Horn blob that does not match still returns false. Do not add an Op or a wire change.
2. Do not edit `fold_query.rs`, `leapfrog.rs`, or the fingerprint. Release id stays `0.0.01`.
3. If a current green test depended on the fail-open, fix the fixture (put the blob back, or empty the pin). Do not weaken `replay_check`.

## RED

`crates/kutha-runtime/tests/b7_horn_blob.rs`, test `missing_rule_blob_fails_replay`. Build a runtime with one Horn Behavior whose pin is nonempty, drop that blob from the fold, call `replay_check`, expect `ExecutionReplayFailed`. A twin with the blob present still returns Ok. The RED fails while the miss branch returns true.

## Gates

`cargo test -p kutha-runtime --offline --test b7_horn_blob`, then `--offline --no-fail-fast` for the crate. `uv run kutha-gov precommit`. No workspace build, no second target, no version bump. Bench not required unless a hot path file changes; `eval.rs` is not the ingest/`as_of` bench body. If `fold.rs` or `fold_query.rs` changes, stop.

## Stop

The fix needs a new Op, or a present Horn blob that matches starts failing replay on the ordinary emit path.
