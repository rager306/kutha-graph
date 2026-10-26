---
phase: 237-vintage-checkpoint
independent: true
status: pass
verifier_model: Cursor Grok 4.6
commands:
  - cmd: cargo test -p kutha-runtime --offline --test b9_vintage_checkpoint
    exit: 0
  - cmd: cargo test --workspace --offline --no-fail-fast
    exit: 0
  - cmd: uv run kutha-gov ci --jobs 1
    exit: 0
---

# Phase 237 — independent verification

**Verdict: PASS.** `Snapshot.behaviors_exact` is `#[serde(default)]` (missing → false) and always serialized (no `skip_serializing_if`). `checkpoint_snapshot_if_receipt_ok` returns `Err(())` when it is false, after digest / `fact_count` / fingerprint. Both writers (`Runtime::snapshot`, checkpoint blob in `store.rs`) set `true`. A marked blob with empty `behaviors` opens. No new `Op` (`kutha-common` untouched at `1e1338b`). Source read; CBM `kutha-graph` `metadata_changed` on snapshot/fold/hydrate/store; `b9` `not_tracked`. No `index_repository`.

## Attacks

| id | claim | result |
|----|--------|--------|
| 1 | missing mark + recomputed digest fails `store::open` | PASS `unmarked_checkpoint_blob_fails_open` (unchanged) |
| 2 | explicit `false` + matching digest fails open | PASS `explicit_false_behaviors_exact_fails_open` |
| 3 | marked no-Horn (behaviors omitted) opens | PASS `marked_no_horn_checkpoint_opens` |
| 4 | marked Horn restores `behaviors` | PASS `marked_horn_checkpoint_restores_behaviors` |
| 5 | digest mismatch with mark true fails | PASS `digest_mismatch_with_mark_true_fails_open` |
| 6 | re-checkpoint cannot launder vintage | PASS `recheckpoint_does_not_launder_vintage` (open_hydrated first; Err; bytes unchanged except `kutha.lock`) |

No launder hole. `from_snapshot` does not check the mark; `finish_open` → `replay_check` still fails closed.

## Commands

- `cargo test -p kutha-runtime --offline --test b9_vintage_checkpoint` **0** (6/6)
- `cargo test --workspace --offline --no-fail-fast` **0**
- `uv run kutha-gov ci --jobs 1` first **1** (`docs-coupling`, dirty tests without CHANGELOG); after kill-test changelog **0**, `run_high=0`

No production edit. `tmp/` empty. `VERSION` unchanged.
