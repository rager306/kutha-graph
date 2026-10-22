---
phase: 228-checkpoint-receipt
plan: 02
subsystem: runtime-store
tags: [checkpoint, fingerprint, PIT, StoreBlob, kill, bench]
requires: [{phase: 228-checkpoint-receipt, provides: "Op::Checkpoint wire + compile-only stub"}]
provides: ["checkpoint encoded persist + receipt verify + GREEN kill suite"]
affects: []
actuals: {tokens: 8000, tasks: 6, commits: 8}
plan_head_before: 47a85e47853b1f71a0905c69188ed556d10723cd
plan_head_after: pending
tech-stack: {added: [], patterns: [encoded_log after checkpoint; Checkpoint receipt verify]}
key-files:
  created: []
  modified: [crates/kutha-runtime/src/store.rs, crates/kutha-runtime/src/fold.rs, crates/kutha-runtime/src/provenance.rs]
key-decisions:
  - "Persist encoded_log after checkpoint; verify fingerprint_hex/fact_count/blob digest on apply and replay_check"
requirements-completed: [D-01, D-02, D-03, D-04, D-05, D-06]
duration: 50min
completed: 2026-10-22
status: complete
---
# Phase 228 Plan 02: checkpoint receipt rewrite Summary

**Sealed-prefix checkpoint persists encoded_log and fail-closes on a lying Checkpoint receipt.**

## Kill HIGH (independent verifier)

R2 `r2_open_after_checkpoint_with_admission_in_active_tail`: `open` → `snapshot log_offset is ahead of the committed prefix`. Root: rewrite wrote graph-only log (Defines dropped). `committed_prefix` builds its dict from Defines, so tail `kutha:admission`/`kutha:action` (Typed, no QO) classified NeedsOutcome and were truncated; snapshot `log_offset` still counted them. R3 `r3_persister_append_after_checkpoint`: `open_for_append` → `segment event count disagrees with runtime encoded log`. Same source: disk length ≠ `encoded_log` (Defines + graph). Fix (`525a737`): persist `encoded_log(&rt)` after building the rewritten Runtime.

R4 `r4_tamper_fingerprint_hex_fails_closed`: append `"00"` to `fingerprint_hex`; `open` Ok, `replay_check` Ok. Root: apply hydrated the blob without checking receipt fields. Fix (`bd5df2f`): `checkpoint_snapshot_if_receipt_ok` on apply (no-op on mismatch) and `replay_check` (typed `ReplayDivergence`). Fingerprint contract unchanged.

## Residual

`horn_behavior_reproducible` still fail-opens on a missing rule blob when the pin is nonempty. No kill-test reaches a tail Horn Behavior after checkpoint (prefix Behavior is compacted into Snapshot facts).

## Bench / verify

PRE vs kill-fix POST: ingest_plain.n **144924496 → 110385750 ns (−23.8%)**; as_of.n **394795 → 264182 ns (−33.1%, eager)**. `b1_checkpoint_kill` 12/12, `b1_checkpoint_receipt`, `m002_commit01`, workspace `--no-fail-fast`, `uv run kutha-gov precommit` all exit 0. Kill-tests not edited. `228-VERIFICATION.md` not edited.

## Deviations

**3. Snapshot blobs skip** — resolved earlier (`052cf8f` prefix StoreBlob copy). **R2/R3 encoded persist** and **R4 receipt verify** closed in this reopen. `.planning/STATE.md` untouched.

## Task Commits

`0a014eb` rewrite · `41d1f33` forged-meta · `49fef86` docs/POST · `052cf8f` StoreBlob copy · `525a737` encoded persist (R2+R3) · `bd5df2f` fingerprint_hex (R4) · this: CHANGELOG/SUMMARY

## Self-Check: PASSED
