---
phase: 228-checkpoint-receipt
plan: 02
subsystem: runtime-store
tags: [checkpoint, sidecar, StoreBlob, kill2, bench]
requires: [{phase: 228-checkpoint-receipt, provides: "Op::Checkpoint wire + compile-only stub"}]
provides: ["prefix QO/JCite keep + empty-fold Checkpoint + GREEN kill except pre-existing StoreBlob digest"]
affects: []
actuals: {tokens: 4885, tasks: 3, commits: 12}
plan_head_before: 47a85e47853b1f71a0905c69188ed556d10723cd
plan_head_after: e2c5ce0d48f3adef063503414bb9672ab1f2d22f
tech-stack: {added: [], patterns: [kept prefix meta ops; Checkpoint only on empty fold]}
key-files:
  created: []
  modified: [crates/kutha-runtime/src/store.rs, crates/kutha-runtime/src/fold.rs, crates/kutha-runtime/src/provenance.rs]
key-decisions:
  - "Keep prefix StoreBlob/QuantumOutcome/JustificationCite; drop old Checkpoint; empty-fold receipt placement"
requirements-completed: [D-01, D-02, D-03, D-04, D-05, D-06]
duration: 75min
completed: 2026-10-22
status: complete
---
# Phase 228 Plan 02: checkpoint receipt Summary

**Sealed-prefix checkpoint keeps sidecar ops, fail-closes misplaced receipts, and leaves pre-existing StoreBlob digest-on-open as a twin-proven residual.**

## Kill2 HIGH (second independent verifier)

1. **R3 sidecars** (`r3_prefix_quantum_justification_admission_action_vs_twin`): `justification_records` 0 vs 1, `outcome_records` 0 vs 22. Root: rewrite kept only `StoreBlob`. Fix (`a683170`): copy prefix `StoreBlob` / `QuantumOutcome` / `JustificationCite` verbatim (same ids/order). `dropped_count` = prefix length minus those kinds. `Op::Checkpoint` is not copied (R5 folds the old receipt into the new Snapshot; one receipt).
2+3. **R4 placement** (`r4_duplicate_receipt_fails_closed`, `r4_receipt_moved_after_tail_fails_closed`): second or tail-moved `Checkpoint` opened Ok. Root: apply always hydrated. Fix (`e2c5ce0`): hydrate only when `facts` empty and `next_seq == 0`; `replay_check` requires only Define/kept-meta before the receipt (`ReplayDivergence`). R5 still passes.
4. **R4 truncated kept StoreBlob**: PRE-EXISTING. Plain twin (no checkpoint) with the same body truncate + `write_test_segmented` also `open` Ok / `replay_check` Ok. Not a checkpoint regression; kill-test left untouched. Only kill2 failure.

## Residual

`horn_behavior_reproducible` still fail-opens on a missing nonempty pin blob. Truncated `StoreBlob` bytes are not checked on `open` (admit-time `blob_digest_matches` only). CBM: src `metadata_changed`, receipt test `not_tracked`.

## Bench / verify

PRE vs kill2-fix POST (2026-10-22T18:03:00Z load 6.82 5.85 4.36): ingest_plain.n **144924496 → 102767611 ns (−29.1%)**; as_of.n **394795 → 215071 ns (−45.5%, eager)**. `b1_checkpoint_kill` 12/12 exit 0. `b1_checkpoint_kill2` 8/9 (only finding 4). receipt, m002 exit 0. workspace 101 = that one test. `uv run kutha-gov precommit` 0. Kill-tests and `228-VERIFICATION.md` not edited.

## Deviations

Sidecar keep and empty-fold placement closed this reopen. Finding 4 documented, not widened. `.planning/STATE.md` untouched.

## Task Commits

`a683170` keep QO/JCite · `e2c5ce0` empty-fold Checkpoint · this: CHANGELOG/SUMMARY

## Self-Check: PASSED
