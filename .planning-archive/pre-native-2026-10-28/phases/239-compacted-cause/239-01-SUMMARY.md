---
phase: 239-compacted-cause
plan: 01
status: complete
completed: 2026-10-27
---
# 239-01: compacted Horn caused_by must be a supporting premise

Helper next to `horn_behavior_reproducible`. Walk in `replay_check`. No Op, fingerprint, or version bump.

## RED

`cargo test -p kutha-runtime --offline --test b11_compacted_cause` **101**.

`nil_caused_by_in_cited_blob_fails_open` panicked at `nil caused_by in cited blob must fail open (got Ok)`. Honest Horn checkpoint, set compacted `behaviors[0].caused_by` to `EventId::nil()`, keep `behaviors_exact` and the fold, recompute StoreBlob + Checkpoint `blob_digest`, `write_test_segmented`. Today `store::open` returned Ok.

`swapped_caused_by_in_cited_blob_fails_open` panicked at `swapped caused_by in cited blob must fail open (got Ok)`. Same fixture; `caused_by` became another fold assert that is not a premise of this Horn.

## GREEN

`covering_horn_hits_in` next to `horn_behavior_reproducible` (same `search_all` + `windows_cover`). `compacted_horn_cause` maps missing blob / no covering hit to `ExecutionReplayFailed` and a covering hit whose `premise_ids` lack `caused_by` to `BrokenLineage`. `replay_check` walks `rebuilt.compacted_behaviors` after the log Behavior walk; does not use log `seen`. `horn_behavior_reproducible` for log rows is the covering-hit boolean. No new Op or `RuntimeError`.

Oracle: nil and swapped cited-blob rewrites fail `store::open` with `BrokenLineage`. Unmodified fixture still opens. `marked_horn_checkpoint_restores_behaviors` and `r3_first_cause_roundtrip_after_recheckpoint` stay green because the honest cause is a premise id of a covering hit.

`cargo test -p kutha-runtime --offline --test b11_compacted_cause` **0**. Then crate `--offline --no-fail-fast` **0**. `uv run kutha-gov precommit` **0**. rustfmt `--edition 2024` on `b11_compacted_cause.rs` only. Did not rustfmt `eval.rs` / `provenance.rs` (`mod` children).

## Isolation

Did not edit `VERSION`, Cargo/pyproject versions. One `target/`. No worktree.
