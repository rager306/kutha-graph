---
phase: 228-checkpoint-receipt
independent: true
status: passed
verifier_model: Cursor Grok 4.6
commands:
  - cmd: cargo test -p kutha-runtime --offline --test b1_checkpoint_kill -- --test-threads=1
    exit: 0
  - cmd: cargo test -p kutha-runtime --offline --test b1_checkpoint_kill2 -- --test-threads=1
    exit: 0
  - cmd: cargo test -p kutha-runtime --offline --test b1_checkpoint_kill3 -- --test-threads=1
    exit: 0
  - cmd: cargo test --workspace --offline --no-fail-fast
    exit: 0
  - cmd: uv run kutha-gov ci --jobs 1
    exit: 0
read_only_inputs:
  - docs/process/gsd-auto-contract.md
  - docs/process/op-compat.md
  - crates/kutha-runtime/src/store.rs
  - crates/kutha-common/src/event.rs
  - crates/kutha-common/src/op_migrate.rs
  - crates/kutha-runtime/src/pipeline.rs
  - crates/kutha-runtime/src/commit.rs
  - crates/kutha-runtime/src/fold.rs
  - crates/kutha-runtime/src/provenance.rs
  - crates/kutha-runtime/src/admission.rs
  - crates/kutha-runtime/src/segment.rs
  - crates/kutha-runtime/src/bin/kutha-migrate.rs
  - crates/kutha-runtime/tests/b1_checkpoint_kill.rs
  - crates/kutha-runtime/tests/b1_checkpoint_kill2.rs
---

# Phase 228 — third independent verification

**Verdict:** passed. No HIGH. kill1 12/12. Pre-edit kill2 failed only `r4_truncated_kept_storeblob_fails_closed` (open Ok, replay_check Ok); converted to `r4_truncated_kept_storeblob_parity_with_plain_store` (same fixture; twin parity). Post-edit kill2 9/9, kill3 12/12. Workspace cargo 0. `ci` `run_high=0` `run_low=1`. Tests committed `3c0d060` before ci. CBM: cited src `metadata_changed`, kill tests `not_tracked` (no `index_repository`). No `chmod`. No product failpoint on `checkpoint_sealed_prefix`.

**R1** pass: receipt `dropped_count>0`; prefix product Assert leaves the log; StoreBlob/QO/Cite kept; empty-active and non-empty-active exact vs twin.
**R2** pass: open, `replay_check`, fingerprint (all facts incl. dead), `as_of`, `as_known_on` at last prefix TtAnchor wall (event dropped, wall restored).
**R3** pass: Persister compose; prefix CI-multi residual 50/80/50, Retract/Correct; windowed allowlist+pins+`kutha:*` TermIds; prefix Horn A-C un-derives after tail retract of A-B; fact seqs; ≥2 sealed rollovers after checkpoint.
**R4** pass: `emit(Checkpoint)` MetaOpRejected+SelfCommit; forged receipt/blob/drop/legacy/empty-sealed fail closed; duplicate/moved receipt fail closed; lock held → WouldBlock. AMENDED: kept-StoreBlob body tamper is parity with plain twin (both open Ok, replay_check Ok). `migrate_compat`/`kutha-migrate`: Ok, remapped=0, demotes segments to jsonl; fingerprint and Checkpoint receipt unchanged.
**R5** pass: twice; checkpoint-append-checkpoint vs twin; exactly one receipt.
**R6** pass: `git diff 0770da5..HEAD -- crates` as_of assert only rewrapped; classification table 10→11 Checkpoint SelfCommit; serde externally tagged, no alias; `remap_legacy_ops` UnknownVariant(Checkpoint). Byte-identical stores: receipt fields match; minted Checkpoint `Event.id` differs (UUID v7); `blob_digest` equal.
**R7** pass: simulated W1–W6 open OLD, NEW, or Err — never a mix.

## Findings

None HIGH.

1. **LOW residual:** log-wide StoreBlob digest verification on open is absent (pre-existing; twin parity). Checkpoint blob is verified via receipt; ordinary kept StoreBlob bodies are not.
2. **LOW:** `migrate_compat` on a checkpointed dir does not fail closed; it demotes sealed layout to jsonl (`already=false remapped=0`). Answers and the receipt stay. Matches `docs/process/op-compat.md` warning.

CBM `checkpoint_sealed_prefix` absent from index; `rg` arbiter.
