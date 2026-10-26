---
phase: 238-cited-lineage
independent: true
status: pass
verifier_model: Cursor Grok 4.6
commands:
  - cmd: cargo test -p kutha-runtime --offline --test b10_cited_lineage
    exit: 0
  - cmd: cargo test --workspace --offline --no-fail-fast
    exit: 0
  - cmd: uv run kutha-gov ci --jobs 1
    exit: 0
read_only_inputs:
  - crates/kutha-runtime/src/fold.rs
  - crates/kutha-runtime/src/store.rs
  - crates/kutha-runtime/src/lib.rs
  - crates/kutha-runtime/src/provenance.rs
  - crates/kutha-runtime/src/snapshot.rs
  - crates/kutha-runtime/src/hydrate.rs
  - crates/kutha-runtime/tests/b10_cited_lineage.rs
  - crates/kutha-runtime/tests/b9_vintage_checkpoint.rs
  - crates/kutha-common/src/event.rs
---

# Phase 238 — independent verification

**Verdict: PASS.** `checkpoint_prefix_behaviors` builds a cite set from every prefix `Op::Checkpoint` `blob_digest`, then copies `Snapshot.behaviors` from a `StoreBlob` only when that digest is in the set and `behaviors_exact` is true. Prefix `Op::Behavior` still clones. No new `Op` (`kutha-common` unchanged vs `9ff6482`). `tt_anchors_from_events` is byte-identical to `9ff6482`. CBM `kutha-graph`: `fold.rs` `metadata_changed`, `b10` `not_tracked`; source read. No `index_repository`.

## Attacks

| id | claim | result |
|----|--------|--------|
| 1 | ordinary Snapshot blob is not harvested | PASS `ordinary_snapshot_blob_is_not_harvested` (body unchanged vs `844fbbd`) |
| 2 | cited blob with `behaviors_exact` false contributes nothing | not independently proven: `checkpoint_prefix_behaviors` is `pub(crate)` and not re-exported; `checkpoint_sealed_prefix` calls `open_hydrated` first; `explicit_false_behaviors_exact_fails_open` rejects. Source has `if snap.behaviors_exact`. Not a pass. |
| 3 | cited Horn checkpoint restores behaviors | PASS existing `marked_horn_checkpoint_restores_behaviors` |
| 4 | prefix `Op::Behavior` with no Snapshot blob is copied | PASS `prefix_behavior_copied_without_snapshot_blob` |
| 5 | uncited Snapshot sibling of a cited checkpoint does not launder | PASS `uncited_snapshot_sibling_of_cited_checkpoint_is_not_harvested` |
| 6 | `tt_anchors_from_events` unwritten | PASS function diff vs `9ff6482` empty |

No launder hole. `checkpoint_kept_prefix_op` still keeps uncited `StoreBlob`; re-checkpoint harvest is digest-cited, not “any Snapshot”.

## Commands

- `cargo test -p kutha-runtime --offline --test b10_cited_lineage` **0** (3/3)
- `cargo test --workspace --offline --no-fail-fast` **0**
- `uv run kutha-gov ci --jobs 1` first **1** (`docs-coupling`); after kill-test changelog **0**, `run_high=0`

No production edit. `tmp/` empty. `VERSION` unchanged.
