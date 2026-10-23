---
phase: 229-checkpoint-cut
independent: true
status: failed
verifier_model: Cursor Grok 4.6
commands:
  - cmd: cargo test -p kutha-runtime --offline --test b3_checkpoint_cut --test b3_checkpoint_cut_kill --test b1_checkpoint_kill --test b1_checkpoint_kill2 --test b1_checkpoint_kill3 -- --test-threads=1
    exit: 0
  - cmd: cargo test -p kutha-runtime --offline --test b3_checkpoint_cut_kill2 -- --test-threads=1
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
  - crates/kutha-runtime/src/quantum.rs
  - crates/kutha-runtime/src/snapshot.rs
  - crates/kutha-runtime/src/eval.rs
  - crates/kutha-runtime/src/bin/kutha-migrate.rs
  - crates/kutha-common/src/event.rs
  - crates/kutha-runtime/tests/b3_checkpoint_cut_kill.rs
  - crates/kutha-runtime/tests/b3_checkpoint_cut.rs
  - crates/kutha-runtime/tests/b1_checkpoint_kill.rs
  - crates/kutha-runtime/tests/b1_checkpoint_kill3.rs
  - crates/kutha-runtime/tests/common/mod.rs
  - .planning/phases/228-checkpoint-receipt/228-VERIFICATION.md
---

# Phase 229 — independent verification (round 2)

**Verdict:** failed. Two HIGH R2 3-hop Horn-cascade divergences vs the un-checkpointed twin. Round-1 kills 48/48. Kill2 11/16. Workspace 101 (only `b3_checkpoint_cut_kill2`). `ci` `run_high=2` (`observe_cargo` on kill2). Kill2 committed `555c5a0` before ci. CBM: cited src `metadata_changed`, kill2 `not_tracked` (no `index_repository`). No product edits. Normalisation: named live product triples / `as_of` / `as_known_on`; fingerprint after Retract of prefix facts (same support slots).

**R1** pass: `fork_at(0)` empty; `1..=c` and `n>len` `ForkCutUnavailable {cut,log_len}`; legal `c<n` Ok vs twin (kill1); after a live cascade interior/past-end still Err.
**R2** fail: see findings. Middle-hop retract, diamond, Correct/CI mid-chain, retract derived, tail re-assert, second rule after checkpoint + trigger, `open_for_append` middle, TT history including pre-checkpoint invalidation, tamper `behaviors` stale digest fail closed — match twin. Round-1 2-hop / two-rule HIGHs now pass.
**R3** pass: `migrate_compat` / `migrate_op_wire` / `kutha-migrate` `InvalidData` + English "Op::Checkpoint" "unchanged"; listing+bytes equal. Non-checkpointed and legacy AllowRelation still Ok.
**R4** pass: `git diff 8ef4ef1..HEAD -- crates/*/tests crates/*/src` — only allowed kill3 migrate observation → must Err; other tests mechanical `fork_at().unwrap()` / rustfmt. No new Op, no serde alias.
**R5** pass: `ls tmp` no new dirs after runs (PersistScratch RAII).

## Findings

1. **HIGH R2 first hop:** trans A-B, B-C, C-D, D-E in the sealed prefix; after checkpoint `emit(Retract A-B)` leaves two live `relatedTo(A,E)`; twin un-derives them (A-C/A-D also drop on both). Repro: `r2_three_hop_retract_first_premise`. Same hole: `r2_checkpoint_append_checkpoint_retract`, `r2_cascade_persist_reopen_then_another_retract` (fails at first cascade), `r2_fork_then_live_retract_matches_twin_fork`.
2. **HIGH R2 last hop:** same chain; `emit(Retract D-E)` un-derives `relatedTo(A,E)` on the compacted store; twin keeps two A-E (both drop D-E/B-E/C-E). Repro: `r2_three_hop_retract_end`.
3. **LOW:** vintage Snapshot JSON with `behaviors` removed and digest recomputed opens; after `Retract A-B` live A-C vintage=true twin=false. Receipt `fingerprint`/`fact_count` do not claim Horn lineage exactness.
4. **LOW:** `ci` `run_high=2` from `observe_cargo` on kill2 (tree otherwise clean; `docs-coupling` 0). `m019_dur_a` 19/19 under workspace parallel (no WouldBlock this pass).
