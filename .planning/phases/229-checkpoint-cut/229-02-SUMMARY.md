---
phase: 229-checkpoint-cut
plan: 02
subsystem: runtime-store
tags: [checkpoint, cut, Horn, migrate, ForkCutUnavailable]
requires: [{phase: 229-checkpoint-cut, provides: "fallible fork_at Result + b3 fork REDs GREEN"}]
provides: ["Horn cascade twin parity after cut", "migrate InvalidData on Checkpoint"]
affects: []
actuals: {tokens: 4215, tasks: 4, commits: 11}
plan_head_before: c30e5e932cc66df0fd1cc80445e86dc04bf2f30e
plan_head_after: 44962da8b1169ea695027951824b75601faf39eb
tech-stack: {added: [], patterns: [Snapshot.behaviors exact Horn lineage including duplicate EventId paths]}
key-files: {created: [], modified: [crates/kutha-runtime/src/fold.rs, crates/kutha-runtime/src/pipeline.rs, crates/kutha-runtime/src/snapshot.rs, crates/kutha-runtime/src/eval.rs, CHANGELOG.md]}
key-decisions:
  - "Kill-fix r1: Snapshot.behaviors exact prefix Behavior records (option a)"
  - "Kill-fix r2: keep same-id multi-cause Behavior rows; rebuild indexes each minting row like append"
requirements-completed: [D-03, D-04, D-05]
duration: 240min
completed: 2026-10-23
status: complete
---
# Phase 229 Plan 02: checkpoint cut (Horn + migrate)
**Exact prefix Behavior records (including two minting paths that share EventId) in the checkpoint blob; Horn cascade after a cut matches T and P; migrate refuses Checkpoint.**
## Twin is P, not live T
kill2 `persist_twins` persist+copy then `store::open` both. Oracle is persist+reopen (P), not a never-reopened Runtime (T). Throwaway `b3_three_arm_probe` (deleted, not committed): T=never-reopened; P=`store::open` no checkpoint; C=checkpoint+open. Command: `cargo test -p kutha-runtime --offline --test b3_three_arm_probe -- --nocapture`.
## Three-arm BEFORE (HEAD 740051c)
| Scenario | T A-E | P A-E | C A-E | T==P | P==C | T==C |
| 3hop retract A-B | 0 | 0 | 2 | true | false | false |
| 3hop retract D-E | 0 | 2 | 0 | false | false | true |
| 2hop retract A-B | 0 | 0 | 0 | true | true | true |
C snapshot.behaviors n=6 vs T/P log Behaviors n=7: dropped second A-E (`e33cf753`, caused_by D-E); kept caused_by B-E. 2-hop first differs at 3-hop (two paths to A-E).
## Outcome
**(ii) on retract A-B** (P=T, C diverges). Retract D-E: C=T but P diverges (pre-existing reopen skip). C≠P on both 3-hop retracts. Not (i): do not treat A-B as reopen-only. No separate reopen RED: rebuild alignment made P=T.
## Root cause
`Event::stable_derived` omits `caused_by`. `checkpoint_prefix_behaviors` HashSet-by-id dropped the second path. `rebuild_log_indexes` `note_live_behavior` skipped duplicate `live_behaviors` ids so `derived_by_cause` kept only the first cause. Live T `append_indexed` pushed both.
## Fix
No id-dedup in `checkpoint_prefix_behaviors`. `index_live_behavior_event` matches `append_indexed`. AFTER probe: all three scenarios T==P==C true (fp too). Kill-tests not edited.
## Bench PRE vs round-2 POST
PRE ingest_plain.n **101894268 ns**, as_of.n **247421 ns**. POST 2026-10-23T06:11:28Z `taskset -c 0` load **4.93 7.13 6.78**: **97135182 ns** (−4.7%), **229128 ns** (−7.4%). Unpinned noisy samples discarded (prime-agent rustc).
## GREEN
`b3_checkpoint_cut_kill2 --test-threads=1` **0** (16/16). kill 11/11. `b3_checkpoint_cut`, `b1_checkpoint_kill`/`kill2`/`kill3`, `b1_checkpoint_receipt`, `m002_commit01`, `m110_rebuild_live_derived` **0**. `cargo test --workspace --offline --no-fail-fast` **0** (m019_dur_a WouldBlock once under parallel, serial 19/19 then workspace 0). `uv run kutha-gov precommit` **0**.
## Deviations
Rule 1: rebuild indexes every minting row (also P=T on retract D-E). `.planning/STATE.md` / `229-VERIFICATION.md` not edited. Probe deleted.
## Task Commits
`97c984a` exact behaviors · `770150e` migrate · `44962da` multi-path Behavior rows · kill-tests `7f6d3d8`/`555c5a0` (verifier)
## Self-Check: PASSED
