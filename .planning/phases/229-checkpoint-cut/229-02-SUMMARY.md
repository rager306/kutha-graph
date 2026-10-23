---
phase: 229-checkpoint-cut
plan: 02
subsystem: runtime-store
tags: [checkpoint, cut, Horn, migrate, ForkCutUnavailable]
requires: [{phase: 229-checkpoint-cut, provides: "fallible fork_at Result + b3 fork REDs GREEN"}]
provides: ["Horn cascade twin parity after cut", "migrate InvalidData on Checkpoint"]
affects: []
actuals: {tokens: 7379, tasks: 4, commits: 7}
plan_head_before: c30e5e932cc66df0fd1cc80445e86dc04bf2f30e
plan_head_after: 97c984adcaadda458708113c4879efb942fd10c6
tech-stack: {added: [], patterns: [Snapshot.behaviors exact Horn lineage; migrate refuse Checkpoint]}
key-files: {created: [], modified: [crates/kutha-runtime/src/snapshot.rs, crates/kutha-runtime/src/fold.rs, crates/kutha-runtime/src/pipeline.rs, crates/kutha-runtime/src/eval.rs, crates/kutha-runtime/src/store.rs, crates/kutha-runtime/src/hydrate.rs, crates/kutha-runtime/src/quantum.rs]}
key-decisions:
  - "Kill-fix: Snapshot.behaviors exact prefix Op::Behavior records (option a); rematch removed"
  - "kill3 r4: observation Ok-or-Err → library InvalidData + dir unchanged + CLI non-success"
requirements-completed: [D-03, D-04, D-05]
duration: 180min
completed: 2026-10-23
status: complete
---
# Phase 229 Plan 02: checkpoint cut (Horn + migrate)
**Exact prefix Behavior records in the checkpoint blob; Horn cascade after a cut matches the twin; migrate refuses Checkpoint.**
## RED (kill-tests, before this fix)
`b3_checkpoint_cut_kill` (7f6d3d8) exit 101. `r2_chain_two_hops_retract_first_premise`: `HIGH R2: chain A-D stayed live after retracting prefix A-B`. `r2_two_rules_two_premises_chain_and_no_live_premise`: `HIGH R2: retracting prefix A-B dropped inverse C-B of still-live B-C`. (229-01 author REDs: interior/past-end fork, Horn fingerprint, migrate Ok.)
## Root cause
Rematch of restored v8 heads cannot recover mint lineage (which premises/rule minted which head). Multi-hop A-D is caused by A-C not A-B, so retract A-B left A-D live. Inverse C-B of live B-C was indexed under A-B and over-retracted.
## Fix (option a)
`Snapshot.behaviors: Vec<Event>` (serde default, skip if empty; blob digest covers it). `checkpoint_prefix_behaviors` copies prefix `Op::Behavior` plus prior blob records (R5). `apply(Checkpoint)` sets `fold.compacted_behaviors`; `rebuild_log_indexes` / `log_event` index them without re-applying facts. Heuristic rematch deleted. No-Horn omits the field. Behaviors stay off the rewritten log (empty-fold Checkpoint). Fingerprint still walks facts. Kill-tests not edited.
## Bench PRE vs kill-fix POST
PRE ingest_plain.n **101894268 ns**, as_of.n **247421 ns**. POST 2026-10-23T04:55:33Z load **2.03 3.47 6.08**: **90623213 ns** (−11.1%), **234898 ns** (−5.1%). Waited; `/root/prime-agent` nextest not touched. Noisy compile/load passes discarded.
## GREEN
`b3_checkpoint_cut_kill --test-threads=1` **0** (11/11). `b3_checkpoint_cut`, `b1_checkpoint_kill`/`kill2`/`kill3`, `b1_checkpoint_receipt`, `m002_commit01`, `m110_rebuild_live_derived` **0**. `cargo test --workspace --offline --no-fail-fast` **0** (`m019_dur_a` 19/19 in that run, no serial retry). `uv run kutha-gov precommit` **0**.
## Deviations
Rule 1 (earlier): swallow `load_log_events` InvalidData for legacy migrate. `.planning/STATE.md` / `229-VERIFICATION.md` not edited. CBM: edited src `metadata_changed`; `rg` arbiter.
## Task Commits
`e844b54` rematch (superseded) · `770150e` migrate · `5168f45` docs · `7f6d3d8` kill-tests · `97c984a` exact behaviors
## Self-Check: PASSED
