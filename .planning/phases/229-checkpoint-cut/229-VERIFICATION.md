---
phase: 229-checkpoint-cut
independent: true
status: failed
verifier_model: Cursor Grok 4.6
commands:
  - cmd: cargo test -p kutha-runtime --offline --test b3_checkpoint_cut -- --test-threads=1
    exit: 0
  - cmd: cargo test -p kutha-runtime --offline --test b3_checkpoint_cut_kill -- --test-threads=1
    exit: 101
  - cmd: cargo test --workspace --offline --no-fail-fast
    exit: 101
  - cmd: uv run kutha-gov ci --jobs 1
    exit: 1
read_only_inputs:
  - docs/process/gsd-auto-contract.md
  - crates/kutha-runtime/src/hydrate.rs
  - crates/kutha-runtime/src/quantum.rs
  - crates/kutha-runtime/src/store.rs
  - crates/kutha-runtime/src/pipeline.rs
  - crates/kutha-runtime/src/fold.rs
  - crates/kutha-runtime/src/bin/kutha-migrate.rs
  - crates/kutha-common/src/event.rs
  - crates/kutha-runtime/tests/b1_checkpoint_kill.rs
  - crates/kutha-runtime/tests/b1_checkpoint_kill3.rs
  - crates/kutha-runtime/tests/b3_checkpoint_cut.rs
  - crates/kutha-runtime/tests/common/mod.rs
---

# Phase 229 — independent verification

**Verdict:** failed. Two HIGH R2 Horn-cascade divergences vs the un-checkpointed twin. Kill tests 9/11. Author `b3_checkpoint_cut` 4/4. Workspace 101 (`b3_checkpoint_cut_kill` + parallel `m019_dur_a` lock flake). `ci` `run_high=1` (`docs-coupling` on pre-existing dirty `tests/common/sem/*`, not this verifier). Kill tests committed `7f6d3d8` before ci. CBM: cited src `metadata_changed`, kill file `not_tracked` (no `index_repository`). No product edits. Normalisation: named live product triples / `as_of` / `as_known_on`; fingerprint only when no new minted support ids.

**R1** pass: `fork_at(0)` empty; `1..=c` and `n>len` `ForkCutUnavailable {cut,log_len}`; legal `c<n` matches twin prefix+tail; interior without Checkpoint still Ok.
**R2** fail: see findings. Single-hop transitivity retract, tail premises, Correct/CI/re-assert/direct head, pin window, re-register, double checkpoint, no-Horn, open and `open_for_append` match the twin.
**R3** pass: `migrate_compat` / `migrate_op_wire` / `kutha-migrate` `InvalidData` + English "Op::Checkpoint" "unchanged"; recursive listing+bytes equal. Non-checkpointed and legacy AllowRelation still Ok.
**R4** pass: `git diff 8ef4ef1..HEAD -- crates/*/tests crates/*/src` — only allowed kill3 migrate observation → must Err; other tests mechanical `.unwrap()` / rustfmt. No new Op, no serde alias (`CheckpointCut` fails).
**R5** pass: `ls tmp` empty after kill tests (PersistScratch RAII).

## Findings

1. **HIGH R2 chain:** transitivity A-B, B-C, C-D in the sealed prefix; after checkpoint `emit(Retract A-B)` leaves live `relatedTo(A,D)` on the compacted store; twin un-derives it. Repro: `r2_chain_two_hops_retract_first_premise`.
2. **HIGH R2 two rules:** transitivity + inverse on `relatedTo`; retract prefix A-B drops inverse `relatedTo(C,B)` of still-live B-C; twin keeps C-B. Pre-retract named sets matched. Repro: `r2_two_rules_two_premises_chain_and_no_live_premise`.
3. **LOW:** workspace `m019_dur_a` 3× `WouldBlock` kutha.lock under parallel `--no-fail-fast`; serial `--test-threads=1` 19/19.
4. **LOW:** `ci` docs-coupling HIGH from already-dirty `tests/common/sem/{oracles,persistence,shrink}.rs` (left uncommitted).
