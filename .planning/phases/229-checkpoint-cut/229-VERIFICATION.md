---
phase: 229-checkpoint-cut
independent: true
status: failed
verifier_model: Cursor Grok 4.6
commands:
  - cmd: cargo test -p kutha-runtime --offline --test b3_checkpoint_cut --test b3_checkpoint_cut_kill --test b3_checkpoint_cut_kill2 --test b1_checkpoint_kill --test b1_checkpoint_kill2 --test b1_checkpoint_kill3 --test b1_checkpoint_receipt -- --test-threads=1
    exit: 0
  - cmd: cargo test -p kutha-runtime --offline --test b3_checkpoint_cut_kill3 -- --test-threads=1
    exit: 101
  - cmd: cargo test --workspace --offline --no-fail-fast
    exit: 101
  - cmd: uv run kutha-gov ci --jobs 1
    exit: 1
read_only_inputs:
  - docs/process/gsd-auto-contract.md
  - crates/kutha-runtime/src/hydrate.rs
  - crates/kutha-runtime/src/store.rs
  - crates/kutha-runtime/src/fold.rs
  - crates/kutha-runtime/src/pipeline.rs
  - crates/kutha-runtime/src/eval.rs
  - crates/kutha-runtime/src/provenance.rs
  - crates/kutha-runtime/src/quantum.rs
  - crates/kutha-runtime/src/snapshot.rs
  - crates/kutha-runtime/tests/b3_checkpoint_cut.rs
  - crates/kutha-runtime/tests/b3_checkpoint_cut_kill2.rs
  - crates/kutha-runtime/tests/b1_checkpoint_kill3.rs
---

# Phase 229 — independent verification (round 3)

**Verdict:** failed. One HIGH R2: re-registering the same Horn after a cut doubles live derived `relatedTo(A,C)` on C (2) vs T (1). Round-1/2 kills 66/66 (3-hop HIGHs now pass). Kill3 7/8; property 128/128 seeds. Workspace 101 only `r3_reregister_same_horn_after_checkpoint`. `ci` `run_high=2` (`observe_cargo` timeout 360s; property ~618s). Kill3 committed `c8c5e96` before ci. CBM: cited src `metadata_changed`, kill3 `not_tracked` (no `index_repository`). No product edits. Normalisation: named live product triples / `as_of` / `as_known_on`; fingerprint only with no new UUID-v7 Assert/rule/Correct since persist.

**R1** pass: `fork_at(0)` empty; `1..=c` and `n>len` `ForkCutUnavailable {cut,log_len}`; legal `c<n` Ok vs twin; `rollback_user_cascade` after checkpoint restores pre-cascade; interior still Err after failed cascade.

**R2** fail: finding 1. Round-2 3-hop / diamond / mid-chain / tail re-assert / second rule / `open_for_append` / TT history / tamper `behaviors` fail-closed — pass. Round-3 tail multi-path same EventId, first-cause after re-checkpoint, fork then retract, 128-seed T/P/C (incl. persist+open and `open_for_append` / second checkpoint) — pass. P==T on those named answers.

**R3** pass: `migrate_compat` / `migrate_op_wire` / `kutha-migrate` `InvalidData` + English "Op::Checkpoint" "unchanged"; listing+bytes equal. Non-checkpointed and legacy AllowRelation still Ok.

**R4** pass: `git diff 8ef4ef1..HEAD -- crates/*/tests crates/*/src` — only allowed kill3 migrate observation → must Err; other tests mechanical `fork_at().unwrap()` / rustfmt. `rebuild_log_indexes` / `index_live_behavior_event` push every live minting row into `derived_by_cause` (same as `append_indexed`; live-gated, then unindex on Retract). No new Op, no serde alias.

**R5** pass: PersistScratch RAII; `ls tmp` clean after runs. ci timeout orphaned two kill3 PIDs and scratch (killed/cleaned). Tree clean except this report before its commit.

## Findings

1. **HIGH R2:** prefix transitivity + A-B + B-C, checkpoint, `emit_rule_ast(trans)` again, `Retract A-B`, re-Assert A-B (`rr-ab-re`). `live_count` A-C **C=2 T=1**. Compacted prefix `kutha:rule` Assert is off the rewritten log so `by_delivery_key` misses the pin; C mints a second live derived. Repro: `r3_reregister_same_horn_after_checkpoint` (no RNG seed).
2. **LOW:** vintage Snapshot JSON with `behaviors` removed and digest recomputed **opens**; after `Retract A-B` live A-C vintage=true twin=false (`r2_vintage_payload_without_behaviors`).
3. **LOW:** `ci` `run_high=2` from `observe_cargo` timeout 360s (required tests not run in that observe). Workspace already isolated the product fail to finding 1. `m019_dur_a` 19/19 under workspace parallel (no WouldBlock).
