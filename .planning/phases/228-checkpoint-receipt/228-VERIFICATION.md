---
phase: 228-checkpoint-receipt
independent: true
status: failed
verifier_model: Cursor Grok 4.6
commands:
  - cmd: cargo test -p kutha-runtime --offline --test b1_checkpoint_kill -- --test-threads=1
    exit: 0
  - cmd: cargo test -p kutha-runtime --offline --test b1_checkpoint_kill2 -- --test-threads=1
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
  - crates/kutha-runtime/src/justification.rs
  - crates/kutha-runtime/src/hydrate.rs
  - crates/kutha-runtime/tests/b1_checkpoint_kill.rs
  - crates/kutha-runtime/tests/common/mod.rs
---

# Phase 228 — second independent verification

**Verdict:** failed. Four HIGH kills in `crates/kutha-runtime/tests/b1_checkpoint_kill2.rs` (5 pass / 4 fail). First-pass 12 kills in `b1_checkpoint_kill.rs` all pass (executor fixes for admission-tail open, Persister attach, `fingerprint_hex` hold). Workspace cargo failed only that target. `ci` `run_high=2` is `observe_cargo` (exit 101 + incomplete). `docs-coupling` 0. No `chmod` / `u+w target` noise. CBM: cited src `metadata_changed`, kill2 `not_tracked` (no `index_repository`). No failpoint on `checkpoint_sealed_prefix`; persist_log uses unique seg names.

**R1** pass: receipt with `dropped_count>0`; prefix product Assert leaves the log; prefix StoreBlobs kept; fold `event_id` preserved.
**R2** pass: open, `replay_check`, fingerprint (all facts incl. dead), `as_of` 2009–2025, `as_known_on` before/between/after TT anchors; admission in active tail opens.
**R3** mixed: Persister append + persist + reopen + further append vs twin matches query PIT (Retract/Correct/Horn `relatedTo(A,C)`/TT 2024). Horn tail Behavior `replay_check` Ok with kept rule blob. **Fails** prefix `QuantumOutcome`/`JustificationCite` sidecar/log vs twin. Admission/action fold rows match.
**R4** mixed: `emit(Checkpoint)` → `MetaOpRejected` + SelfCommit; wrong `fingerprint_hex`/`fact_count`/`blob_digest`, truncated checkpoint blob, dropped receipt, truncated sealed *file*, legacy/empty-sealed fail closed. **Fails**: duplicate receipt, receipt moved after tail, truncated *kept* StoreBlob bytes — `open` Ok and `replay_check` Ok.
**R5** pass: checkpoint twice; checkpoint → append → checkpoint vs twin.
**R6** pass: `git diff 0770da5..HEAD -- crates` adds Checkpoint exhaustive arms and classification table 10→11; existing as_of assert only rewrapped. Serde externally tagged `Checkpoint`, no alias.
**R7** pass: simulated W1–W6 (new segs+old manifest; leftover old segs; new log+old snapshot/sidecars; new terms; old outcomes; old justifications) open as OLD, NEW, or Err — never a mix.

## Findings

1. **HIGH R3** `r3_prefix_quantum_justification_admission_action_vs_twin`: after checkpoint, `justification_records()` 0 vs 1 and `outcome_records()` 0 vs 22. PIT/fingerprint/admission/action match. Min: `set_test_segment_limits(Some(4096), Some(1))`; seed with prefix cite+QO; persist; checkpoint; `open`; compare twin.
2. **HIGH R4** `r4_duplicate_receipt_fails_closed`: clone `Op::Checkpoint` and insert a second receipt; `write_test_segmented`; `open` Ok, `replay_check` Ok.
3. **HIGH R4** `r4_receipt_moved_after_tail_fails_closed`: move receipt to end of log; `open` Ok, `replay_check` Ok.
4. **HIGH R4** `r4_truncated_kept_storeblob_fails_closed`: truncate a kept (non-checkpoint) `StoreBlob` body; `open` Ok, `replay_check` Ok.
