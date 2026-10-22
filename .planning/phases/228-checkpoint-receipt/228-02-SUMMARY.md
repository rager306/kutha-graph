---
phase: 228-checkpoint-receipt
plan: 02
subsystem: runtime-store
tags: [checkpoint, fingerprint, PIT, bench]
requires:
  - phase: 228-checkpoint-receipt
    provides: Op::Checkpoint wire + compile-only stub
provides:
  - store::checkpoint_sealed_prefix sealed rewrite + GREEN exactness
affects: []
actuals:
  tokens: 5667
  tasks: 3
  commits: 3
plan_head_before: 47a85e47853b1f71a0905c69188ed556d10723cd
plan_head_after: pending
tech-stack:
  added: []
  patterns: [on-log Snapshot blob + Op::Checkpoint receipt]
key-files:
  created: []
  modified:
    - crates/kutha-runtime/src/store.rs
    - crates/kutha-runtime/src/fold.rs
    - crates/kutha-runtime/tests/b1_checkpoint_receipt.rs
key-decisions:
  - "Checkpoint hydrates Snapshot fold; tt_anchors union blob walls; blobs stay serde-skip"
requirements-completed: [D-01, D-02, D-03, D-04, D-05, D-06]
duration: 20min
completed: 2026-10-22
status: complete
---
# Phase 228 Plan 02: checkpoint receipt rewrite Summary

**Sealed prefix rewrite under kutha.lock is GREEN: fingerprint, PIT, open, replay_check, and forged-meta.**

## Bench POST vs PRE (D-05)
Host benches alone. POST start load 2.11 2.55 3.05 (2026-10-22T16:14:46Z); end 4.04 3.12 3.22. `KUTHA_BENCH_N=10000 cargo bench -p kutha-runtime --bench baseline --offline` exit 0. ingest_plain.n **144924496 → 128077580 ns (−11.6%)**; as_of.n **394795 → 273420 ns (−30.7%, eager)**. Neither regresses >10%.

## Verify
`checkpoint_sealed_prefix_preserves_fingerprint_and_pit` exit 0. `user_emit_checkpoint_is_meta_op_rejected` exit 0. `uv run kutha-gov precommit` exit 0. `cargo test --workspace --offline --no-fail-fast` exit 0 (trybuild needed chmod u+w on 444 rmeta; not a product change). No new governor check.

## Snapshot shape gap
`GraphFold.blobs` is `#[serde(skip)]`. Prefix `StoreBlob` bodies (rule AST) are not in the checkpoint Snapshot. Allow/rule/policy/admission/action **facts**, `next_seq`, `tt_anchors`, and intern `dict_strings` restore. Fixture has no prefix rule blobs; oracle GREEN without weakening fingerprint.

## Deviations
- CHANGELOG H2 started in task 1 (docs-coupling on `crates/**/*.rs`); task 3 filled POST numbers.
- `.planning/STATE.md` untouched (controller-owned). honeycomb.yaml restage; map stays Proposed.
- Live dir keeps one SUMMARY (`artifact-budget` max_files_per_dir 1). 228-01 PRE/RED stay in git (`249dff8` / `47a85e4`).

## Task Commits
1. `0a014eb` feat sealed-prefix rewrite
2. `41d1f33` test forged-meta oracle
3. this commit: docs, matrix, POST, SUMMARY

## Self-Check: PASSED
