---
phase: 228-checkpoint-receipt
plan: 01
subsystem: runtime-store
tags: [checkpoint, Op, fingerprint, bench]
requires: []
provides:
  - Op::Checkpoint wire + compile-only store::checkpoint_sealed_prefix
affects: [228-02]
actuals:
  tokens: 4265
  tasks: 3
  commits: 3
plan_head_before: 0770da57864c378672fc8c762a25cbf7d301654d
plan_head_after: pending
tech-stack:
  added: []
  patterns: [SelfCommit meta-op, reject_forged_meta]
key-files:
  created: [crates/kutha-runtime/tests/b1_checkpoint_receipt.rs]
  modified: [crates/kutha-common/src/event.rs, crates/kutha-runtime/src/store.rs]
key-decisions:
  - "PRE before crates/; stub returns ErrorKind::Other; no serde alias or remap_legacy_ops"
requirements-completed: [D-01, D-02, D-03, D-04, D-05]
duration: 20min
completed: 2026-10-22
status: complete
---
# Phase 228 Plan 01: checkpoint receipt Op wire Summary

**PRE recorded; RED was E0425; Op::Checkpoint is SelfCommit; exactness is not GREEN.**

## Bench PRE (D-05)
Host 12 cores, benches alone. Load start 3.43 3.13 2.99 (2026-10-22T15:41:46Z); end 4.93 3.87 3.28. `KUTHA_BENCH_N=10000 cargo bench -p kutha-runtime --bench baseline --offline` exit 0. ingest_plain.n **144924496 ns**; as_of.n **394795 ns** (eager).

## RED (verbatim, before production code)
`cargo test -p kutha-runtime --offline --test b1_checkpoint_receipt -- --exact checkpoint_sealed_prefix_preserves_fingerprint_and_pit` exit **101**:
```
error[E0425]: cannot find function `checkpoint_sealed_prefix` in module `store`
  --> crates/kutha-runtime/tests/b1_checkpoint_receipt.rs:92:12
92 |     store::checkpoint_sealed_prefix(&dir).unwrap();
error: could not compile `kutha-runtime` (test "b1_checkpoint_receipt") due to 1 previous error
```
After stub: same command exit **101**, panic `Custom { kind: Other, error: "checkpoint_sealed_prefix: sealed prefix rewrite is not implemented" }`. Sealed-delete RED not used.

## Verify
every_op_variant_is_classified exit 0 (table 11). m002_commit01 17/17 exit 0. m019_sem_model 5/5 exit 0. kutha-common exit 0. kutha-runtime --lib 22/22 exit 0.

## Deviations from Plan
- [Rule 1] Genesis already owns delivery_key `inForceAs`; fixture intern()s it (emit_allow DeliveryKeyConflict).
- Accidental `git checkout` of pre-existing dirty `.compound-engineering/.../report.md` while backing out crate-wide rustfmt; WIP not recovered.
- `.planning/STATE.md` left untouched (controller-owned).

## Task Commits
1. `249dff8` docs PRE bench
2. `7f30fa4` test RED oracle
3. this commit: feat Op::Checkpoint + compile-only stub

## Self-Check: PASSED
