# Phase 234 — StoreBlob digest on open (M160 S01)

Admit checks `blob_digest_matches`. `replay_check` does not. A same-length rewrite of an ordinary blob still opens. Checkpoint snapshot bytes are already checked. A missing Horn blob is M159 and must stay `ExecutionReplayFailed`.

## Locked decisions

1. In `Runtime::replay_check`, after the Behavior walk, reject `Op::StoreBlob` when `!blob_digest_matches(digest, bytes)` with `UnknownRuleVersion { pin: digest }`. Do not make `apply` fallible. No new error variant. No new Op.
2. Do not check Horn blobs twice in a way that changes M159's error. Behavior walk first.
3. Release id stays `0.0.01`. Do not edit VERSION or Cargo version.
4. Migrate tests that `store::open` a log whose digest does not match the bytes (legacy pin key, bytes are a rule body) must not be "fixed" by skipping the check. Open the log only after the migrate rewrite, or build bytes that match the digest. The oracle of the migrate test stays.

## RED

`crates/kutha-runtime/tests/b8_blob_digest.rs`, test `tampered_storeblob_same_length_fails_open`. Persist an ordinary blob, rewrite one byte keeping the length through the existing test rewrite helper (CRC must still match), `store::open` is `Err(UnknownRuleVersion)`. The matching twin opens. The RED fails while open returns Ok.

## Gates

`cargo test -p kutha-runtime --offline --test b8_blob_digest`, then the crate `--offline --no-fail-fast`. `uv run kutha-gov precommit`. No workspace build, no second target, no version bump. No bench: `fold_query.rs` and ingest bodies stay untouched.

## Stop

The check has to run before the Behavior walk, or a matching ordinary blob starts failing open.
