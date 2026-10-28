---
phase: 238-cited-lineage
plan: 01
status: complete
completed: 2026-10-26
---
# 238-01: harvest checkpoint behaviors only from a cited blob

Cite gate on `checkpoint_prefix_behaviors`. No Op, fingerprint, or version bump.

## RED

`cargo test -p kutha-runtime --offline --test b10_cited_lineage -- --exact ordinary_snapshot_blob_is_not_harvested` **101**.

`ordinary_snapshot_blob_is_not_harvested` panicked at `ordinary uncited Snapshot blob must not harvest Behavior 01a17832-826c-74f2-863b-157dbccbfaf0`. Digest-matching ordinary `StoreBlob` whose JSON is a Snapshot (`behaviors_exact` true, one Behavior never on the log), no `Op::Checkpoint` cite, seal via `write_test_segmented`, `checkpoint_sealed_prefix`, `store::open`. Today `snapshot().behaviors` contained that id.

## GREEN

First pass: collect every prefix `Op::Checkpoint` `blob_digest`. Second pass: copy `Snapshot.behaviors` from a `StoreBlob` only when that digest is cited and `behaviors_exact` is true. Prefix `Op::Behavior` events still copied. Did not change `tt_anchors_from_events`. No new Op or `RuntimeError`.

Oracle: forged id absent after open. Real second checkpoint still restores Horn rows (`r5_checkpoint_twice`, `r3_first_cause_roundtrip_after_recheckpoint`, b9 marked Horn restore) because the prior receipt cite is still on the log when the next harvest runs.

## Isolation

Did not edit `eval.rs`, `VERSION`, Cargo/pyproject versions. One `target/`. No worktree. No rustfmt on `fold.rs`.
