---
phase: 239-compacted-cause
independent: true
status: pass
verifier_model: Cursor Grok 4.6
commands:
  - cmd: cargo test -p kutha-runtime --offline --test b11_compacted_cause
    exit: 0
  - cmd: cargo test --workspace --offline --no-fail-fast
    exit: 0
  - cmd: uv run kutha-gov ci --jobs 1
    exit: 0
read_only_inputs:
  - crates/kutha-runtime/src/eval.rs
  - crates/kutha-runtime/src/provenance.rs
  - crates/kutha-runtime/src/fold.rs
  - crates/kutha-runtime/src/hydrate.rs
  - crates/kutha-runtime/src/store.rs
  - crates/kutha-runtime/src/quantum.rs
  - crates/kutha-runtime/src/segment.rs
  - crates/kutha-runtime/src/snapshot.rs
  - crates/kutha-runtime/tests/b11_compacted_cause.rs
  - crates/kutha-runtime/tests/b9_vintage_checkpoint.rs
  - crates/kutha-common/src/event.rs
  - crates/kutha-common/src/rule.rs
---

# Phase 239 — independent verification

**Verdict: PASS.** `replay_check` walks `rebuilt.compacted_behaviors` via `compacted_horn_cause(&rebuilt)`. A Horn covering `search_all` hit must cover the head window and include `caused_by` in `premise_ids`. Empty hits or missing blob → `ExecutionReplayFailed`. Covering hit without that cause → `BrokenLineage`. Named / non-Horn skip. `horn_behavior_reproducible` still succeeds on `!hits.is_empty()` (no `caused_by` membership). `search_all` now takes `fold`; log behavior unchanged. No new `Op` (`kutha-common` empty vs `ed8d4c4`). Compacted path does not consult log `seen`. CBM `kutha-graph`: `eval.rs`/`provenance.rs` `metadata_changed`, `b11` `not_tracked`; source read. No `index_repository`.

## Attacks

| id | claim | result |
|----|--------|--------|
| 1 | nil/swapped cause is BrokenLineage | PASS existing tests; not digest mismatch |
| 2 | honest Horn checkpoint opens | PASS `honest_horn_checkpoint_opens` |
| 3 | missing cited rule blob | PASS `missing_kept_rule_blob_is_execution_replay_failed` (strip StoreBlob keyed by `rule_version`; not `UnknownRuleVersion`) |
| 4 | log `horn_behavior_reproducible` not tightened | PASS vs `ed8d4c4` |
| 5 | `behaviors_exact` false still fails open | PASS `explicit_false_behaviors_exact_fails_open` |
| 6 | check uses rebuilt, not `snapshot.json` | PASS `poison_log_stale_snapshot_json_still_fails_open` |

## Commands

- `cargo test -p kutha-runtime --offline --test b11_compacted_cause` **0** (5/5)
- `cargo test --workspace --offline --no-fail-fast` **0**
- `uv run kutha-gov ci --jobs 1` first **1** (`docs-coupling`); after kill-test changelog **0**, `run_high=0`

No production edit. `tmp/` empty. `VERSION` unchanged.
