---
phase: 229-checkpoint-cut
independent: true
status: passed
verifier_model: Cursor Grok 4.6
commands:
  - cmd: cargo test -p kutha-runtime --offline --test b3_checkpoint_cut --test b3_checkpoint_cut_kill --test b3_checkpoint_cut_kill2 --test b1_checkpoint_kill --test b1_checkpoint_kill2 --test b1_checkpoint_kill3 --test b1_checkpoint_receipt -- --test-threads=1
    exit: 0
  - cmd: cargo test -p kutha-runtime --offline --test b3_checkpoint_cut_kill3 -- --test-threads=1
    exit: 0
  - cmd: env KUTHA_KILL_CASES=128 cargo test -p kutha-runtime --offline --test b3_checkpoint_cut_kill3 r3_property_random_graphs_three_arm -- --exact --test-threads=1
    exit: 0
  - cmd: cargo test --workspace --offline --no-fail-fast
    exit: 0
  - cmd: uv run kutha-gov ci --jobs 1
    exit: 0
read_only_inputs:
  - docs/process/gsd-auto-contract.md
  - crates/kutha-runtime/src/pipeline.rs
  - crates/kutha-runtime/src/admission.rs
  - crates/kutha-runtime/src/hydrate.rs
  - crates/kutha-runtime/src/eval.rs
  - crates/kutha-runtime/src/justification.rs
  - crates/kutha-runtime/src/fold.rs
  - crates/kutha-runtime/src/quantum.rs
  - crates/kutha-runtime/tests/b3_checkpoint_cut.rs
  - crates/kutha-runtime/tests/b3_checkpoint_cut_kill2.rs
  - .env.example
---

# Phase 229 — independent verification (round 4)

**Verdict:** passed. Round-3 HIGH (same Horn re-register after cut, C=2 T=1) is gone: `r3_reregister_same_horn_after_checkpoint` and executor twins pass. Kill3 9/9 (default 16 cases, 45.2s). Deep `KUTHA_KILL_CASES=128` 128/128 in 236.1s (no HIGH). Workspace 0 in 104.1s. `ci --jobs 1` exit 0, `run_high=0` (219.0s). Tests+`.env.example` committed `3e0022a` before ci. CBM: cited src `metadata_changed`, kill3 `not_tracked` (no `index_repository`). No product edits. Normalisation: named live product triples / `as_of` / `as_known_on`; fingerprint only with no new UUID-v7 Assert/rule/Correct since persist.

**R1** pass: `fork_at(0)` empty; `1..=c` and `n>len` `ForkCutUnavailable`; legal `c<n` Ok vs twin; cascade rollback after checkpoint restores pre-cascade; interior still Err.

**R2** pass: C==T==P on live / as_of / as_known_on / derived; Horn 2-hop and 3-hop; tail multi-path; first-cause re-checkpoint; `open_for_append`; re-register Horn; `emit_allow` / `emit_policy` retry (`events_in_quantum=0`, live pin=1); dead prefix delivery_key remints one live row; admission/action user re-emit MetaOpRejected (no second live); retract-dict + re-record remints one. Tamper `behaviors` fail-closed.

**R3** pass: migrate `InvalidData` + English "Op::Checkpoint"; bytes unchanged; legacy/non-checkpointed still migrate.

**R4** pass: `git diff 8ef4ef1..HEAD` — only allowed b1_checkpoint_kill3 migrate must-Err; other tests mechanical `fork_at().unwrap()` / rustfmt / added kills. `rebuild_log_indexes` fills `by_delivery_key` from facts (`or_insert`) and `compacted_assert_by_id` for payload match. No new Op, no serde alias.

**R5** pass: PersistScratch RAII; `ls tmp` clean after runs. Tree clean except this report before its commit.

## Findings

1. **LOW:** vintage Snapshot JSON with `behaviors` removed and digest recomputed **opens**; after `Retract A-B` live A-C vintage=true twin=false (`r2_vintage_payload_without_behaviors`).
2. **Wall:** kill 18.3s; kill2 **24.9s** (slowest of those two, still <60s); kill3 default 45.2s; deep 128 236.1s; workspace 104.1s. `m019_dur_a` 19/19, no WouldBlock. `ci` `run_low=1` (trajectory).
