---
phase: 228-checkpoint-receipt
independent: true
status: failed
verifier_model: Cursor Grok 4.6
commands:
  - cmd: cargo test -p kutha-runtime --offline --test b1_checkpoint_kill -- --test-threads=1
    exit: 101
  - cmd: cargo test --workspace --offline --no-fail-fast
    exit: 101
  - cmd: uv run kutha-gov ci --jobs 1
    exit: 1
read_only_inputs:
  - docs/process/gsd-auto-contract.md
  - crates/kutha-runtime/src/store.rs
  - crates/kutha-common/src/event.rs
  - crates/kutha-runtime/src/lib.rs
  - crates/kutha-runtime/src/pipeline.rs
  - crates/kutha-runtime/src/commit.rs
  - crates/kutha-runtime/src/fold.rs
  - crates/kutha-runtime/src/provenance.rs
  - crates/kutha-runtime/src/persist_append.rs
  - crates/kutha-runtime/src/segment.rs
  - crates/kutha-runtime/src/quantum.rs
  - crates/kutha-runtime/tests/m002_seg01.rs
  - crates/kutha-runtime/tests/common/mod.rs
---

# Phase 228 — independent verification

**Verdict:** failed. Three HIGH kills in `crates/kutha-runtime/tests/b1_checkpoint_kill.rs`. Happy-path R2 and R5 pass; twin R3 (identical suffix) passes. Workspace cargo failed only that target (9 pass / 3 fail). `ci` `run_high=1` is `docs-coupling` (uncommitted `crates/` test; CHANGELOG not edited). No `chmod` / `u+w target` noise. CBM: new test file `not_tracked` (no `index_repository`).

**R1** pass (graph-prefix fixture): `Op::Checkpoint` with `dropped_count>0`; prefix product Assert leaves the log; prefix `StoreBlob` ids kept; fold `event_id` preserved.
**R2** mixed: fingerprint, `replay_check`, `as_of` (2009–2025), `as_known_on` (before first TT anchor / between 2015–2021 / after), dead-fact count match. **Fails** when `kutha:admission` is in the active tail.
**R3** mixed: same suffix on checkpointed vs twin matches fingerprint and PIT (Retract/Correct of prefix facts, Horn `relatedTo(A,C)`, TT 2024). **Fails** `open_for_append` / `Persister::append`.
**R4** mixed: `emit(Checkpoint)` → `MetaOpRejected` + log unchanged; `SelfCommit`; wrong `blob_digest`, truncated blob, dropped receipt, legacy and empty-sealed fail closed and leave payload. **Fails**: wrong `fingerprint_hex` opens, `replay_check` Ok.
**R5** pass: checkpoint twice; checkpoint → same suffix → checkpoint vs uncheckpointed twin.
**R6** pass: `git diff 0770da5..HEAD` only adds Checkpoint exhaustive-match arms and classification table 10→11; no deleted/weakened asserts. Serde externally tagged `Checkpoint`, no `checkpoint` alias.

## Findings

1. **HIGH R2** `r2_open_after_checkpoint_with_admission_in_active_tail`: `store::open` after checkpoint → `InvalidData` `snapshot log_offset is ahead of the committed prefix`. Min: `set_test_segment_limits(Some(4096), Some(1))`; `emit_rule`+`emit_policy`+product Asserts; `record_justification`/`record_action` last; persist; `checkpoint_sealed_prefix`; open.
2. **HIGH R3** `r3_persister_append_after_checkpoint`: `open_for_append` → `segment event count disagrees with runtime encoded log (recover with full persist)`. Min: graph-prefix fixture; checkpoint; `open_for_append`.
3. **HIGH R4** `r4_tamper_fingerprint_hex_fails_closed`: `export_jsonl` + append `"00"` to `Op::Checkpoint.fingerprint_hex` + `write_test_segmented`; `store::open` Ok and `replay_check` Ok.
