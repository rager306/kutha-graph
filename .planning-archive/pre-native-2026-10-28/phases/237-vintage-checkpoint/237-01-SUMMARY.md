---
phase: 237-vintage-checkpoint
plan: 01
status: complete
completed: 2026-10-26
---
# 237-01: require behaviors_exact on checkpoint blobs

Mark on `Snapshot`. No Op, fingerprint, or version bump.

## RED

`cargo test -p kutha-runtime --offline --test b9_vintage_checkpoint -- --exact unmarked_checkpoint_blob_fails_open` **101**.

`unmarked_checkpoint_blob_fails_open` panicked at `unmarked checkpoint blob must fail open (got Ok)`. Normal no-Horn checkpoint, strip `behaviors_exact` (absent today), recompute StoreBlob + Checkpoint `blob_digest`, `write_test_segmented`. `store::open` returned Ok.

## GREEN

`Snapshot.behaviors_exact: bool` with `#[serde(default)]` (missing = false), always serialized. `Runtime::snapshot` and `store.rs` checkpoint blob set it true. `checkpoint_snapshot_if_receipt_ok` returns `Err(())` when `!snap.behaviors_exact` after digest, fact_count, and fingerprint. Existing `ReplayDivergence` receipt error. No new Op or `RuntimeError` variant.

Oracle: unmarked blob (field stripped, digest rewritten) fails `store::open`. Twin that keeps the mark opens, including empty `behaviors`. Did not touch other checkpoint tests.

`cargo test -p kutha-runtime --offline --test b9_vintage_checkpoint` **0**. Then crate `--offline --no-fail-fast` **0**. `uv run kutha-gov precommit` **0**. rustfmt `--edition 2024` on `snapshot.rs`, `hydrate.rs`, `store.rs`, `fold.rs`, `b9_vintage_checkpoint.rs` (no `mod` children).

## Isolation

Did not edit `eval.rs`, `VERSION`, Cargo/pyproject versions. One `target/`. No worktree. No bench A/B. No other tests retouched.
