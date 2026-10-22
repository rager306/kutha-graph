---
phase: 228-checkpoint-receipt
plan: 02
subsystem: runtime-store
tags: [checkpoint, fingerprint, PIT, StoreBlob, bench]
requires: [{phase: 228-checkpoint-receipt, provides: "Op::Checkpoint wire + compile-only stub"}]
provides: ["store::checkpoint_sealed_prefix + prefix StoreBlob copy + GREEN exactness"]
affects: []
actuals: {tokens: 5902, tasks: 4, commits: 4}
plan_head_before: 47a85e47853b1f71a0905c69188ed556d10723cd
plan_head_after: pending
tech-stack: {added: [], patterns: [on-log Snapshot blob + Op::Checkpoint; prefix StoreBlobs copied verbatim]}
key-files:
  created: []
  modified: [crates/kutha-runtime/src/store.rs, crates/kutha-runtime/tests/b1_checkpoint_receipt.rs]
key-decisions:
  - "Prefix StoreBlobs copied verbatim; blobs stay serde-skip; dropped_count = prefix_len - kept StoreBlobs"
requirements-completed: [D-01, D-02, D-03, D-04, D-05, D-06]
duration: 35min
completed: 2026-10-22
status: complete
---
# Phase 228 Plan 02: checkpoint receipt rewrite Summary

**Prefix StoreBlobs copied onto the rewritten log so rule/policy/plain bodies survive reopen.**

## RED (before fix)

Sealed prefix: Horn `emit_rule_ast`, `emit_policy` admit-all, Horn-derived `Behavior` (`P relatedTo derived`), sha256 `StoreBlob` `plain-checkpoint-blob`. After checkpoint, fingerprint / as_of 2009/2015/2025 / as_known_on 2015/2021 / `replay_check` **matched**. Kill was missing rule body:

```
thread 'checkpoint_sealed_prefix_preserves_fingerprint_and_pit' (1306917) panicked at crates/kutha-runtime/tests/b1_checkpoint_receipt.rs:218:51:
rule StoreBlob survives checkpoint
test checkpoint_sealed_prefix_preserves_fingerprint_and_pit ... FAILED
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.61s
```

`PIPE_EXIT:101`. Earlier mismatched m016 Horn (`derive_pq` head vs `relatedTo` Behavior) failed at unwrap: `execution replay failed for 01a16473-965c-7b93-b139-d0101e395379` (fixture bug, discarded).

## Fix

Copy prefix `Op::StoreBlob` events verbatim (same ids/order) before the checkpoint Snapshot blob + `Op::Checkpoint`. `dropped_count` = sealed prefix event count minus those kept StoreBlobs. `fact_count` = prefix fold fact count (dead facts included). Fingerprint unchanged. D-01 stays on-log.

## Coverage

dict/TermIds: Snapshot `dict_strings` + `write_terms`; test asserts dictionary strings equal. Allowlist: `kutha:allows` facts (genesis `inForceAs` + `emit_allow derive_pq`) in Snapshot. Pins: `kutha:rule`/`kutha:policy` facts; live-at after open; bodies via copied blobs. Admissions/actions: Assert facts in Snapshot; fixture does not emit them. TT anchors: fixture walls; Snapshot + blob union; `as_known_on`. `next_seq`: Snapshot JSON (not serde-skip). Defines intern order: dropped Defines; intern from `dict_strings`. Prefix `QuantumOutcome`/`JustificationCite` stay dropped (sidecars rebuilt from the new log).

## Bench / verify

PRE vs copy POST: ingest_plain.n **144924496 → 127349319 ns (−12.1%)**; as_of.n **394795 → 296063 ns (−25.0%, eager)**. Named tests, `m002_commit01`, workspace `--no-fail-fast`, `uv run kutha-gov precommit` all exit 0.

## Deviations

**3. [Rule 1 - Bug] Snapshot `blobs` serde-skip** — resolved (RED above; prefix StoreBlobs copied). CHANGELOG H2 started in task 1. `.planning/STATE.md` untouched. Live dir one SUMMARY.

## Task Commits

1. `0a014eb` feat rewrite 2. `41d1f33` forged-meta 3. `49fef86` docs/first POST 4. this commit: StoreBlob copy + oracle + docs

## Self-Check: PASSED
