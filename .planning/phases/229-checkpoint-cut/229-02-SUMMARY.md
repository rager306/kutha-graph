---
phase: 229-checkpoint-cut
plan: 02
subsystem: runtime-store
tags: [checkpoint, cut, Horn, migrate, ForkCutUnavailable]
requires: [{phase: 229-checkpoint-cut, provides: "fallible fork_at Result + b3 fork REDs GREEN"}]
provides: ["Horn cascade twin parity after cut", "migrate InvalidData on Checkpoint"]
affects: []
actuals: {tokens: 8267, tasks: 3, commits: 3}
plan_head_before: c30e5e932cc66df0fd1cc80445e86dc04bf2f30e
plan_head_after: 5168f4516154e61d07c0c002ce10805085d720f1
tech-stack: {added: [], patterns: [restored-fact Horn rematch; migrate refuse Checkpoint]}
key-files: {created: [], modified: [crates/kutha-runtime/src/pipeline.rs, crates/kutha-runtime/src/eval.rs, crates/kutha-runtime/src/store.rs]}
key-decisions:
  - "F1: rematch restored v8 heads vs kept Horn blobs; no Behavior re-append"
  - "kill3 r4: observation Ok-or-Err → library InvalidData + dir unchanged + CLI non-success"
requirements-completed: [D-03, D-04, D-05]
duration: 90min
completed: 2026-10-23
status: complete
---
# Phase 229 Plan 02: checkpoint cut (Horn + migrate)
**Horn un-derive after a cut matches the twin; migrate refuses Checkpoint with InvalidData.** Merged 229-01 (last hash `c30e5e9`); `git rm` that SUMMARY (artifact budget: one file).
## Bench PRE (229-01) vs POST
Waited; `/root/prime-agent` `cargo test -p pa-core` looping at PRE, gone at POST retry. 12-way, rustc 1.99.0. PRE 2026-10-23T02:57:57Z load **4.05 3.54 3.23**: ingest_plain.n **101894268 ns**, as_of.n **247421 ns**. POST 03:38:12Z load **2.99 4.52 4.39**: **100147178 ns** (−1.7%), **267138 ns** (+8.0%, eager). First POST pass 140268212 ns (load 5.41 + 19s bench compile) discarded as noise.
## RED (229-01, before production)
`b3_checkpoint_cut` exit 101. (1) interior FIRED: fork_at(1) hole. (2) past-end FIRED: fork_at(14) full log. (3) Horn FIRED (F1 not dropped): fingerprint mismatch after live `emit(Retract)` (`[230,115,…]` vs `[252,224,…]`). (4) migrate FIRED: Ok `already_migrated: false`.
## F1 root cause and fix
`rebuild_log_indexes` walked remaining live `Op::Behavior` only. Checkpoint drops those ops; restored facts still hold derived heads (`event_id` v8, no log row). `retract_derived` skipped when `log_event` was None. Fix: rematch live restored heads against kept StoreBlob Horn bodies; fill `derived_by_cause` / `live_behaviors` / premise maps; emit `Retract { event_id }` when the minting Behavior is gone. No Behavior ops appended. Fingerprint contract unchanged. Twin is the oracle. `m110` still green.
## kill3 r4 (one allowed assertion change)
Old: Ok-with-parity **or** Err both accepted (observation). New: library `Err` kind `InvalidData`, `payload_listing` unchanged; CLI non-success. Reason: D-04 fail-closed; demote was a hole. `load_log_events` on legacy `AllowRelation` is `InvalidData` (cannot contain Checkpoint) so the helper returns Ok and Op-wire remaps (keeps m076).
## old-to-new fork_at (D-02, 229-01)
hydrate.rs `-> Result`; pipeline.rs `expect("rollback prefix was log.len()…")`; quantum_invariants / examples / tests `unwrap()` (m017, m016, facade_statute, m053, m013×3, m012×3, m012a).
## GREEN / gates
Interior, past-end, Horn, migrate b3 **exit 0**. kill / kill2 / kill3 / receipt / m002_commit01 **0**. `cargo test --workspace --offline --no-fail-fast` **0**. `uv run kutha-gov precommit` **0**. SEM-m155-fork-cut / migrate-checkpoint / horn-cascade killed. ADRs **Proposed**.
## Deviations
Rule 1: swallow `load_log_events` InvalidData so legacy migrate still remaps. `.planning/STATE.md` not edited (executor hard rule). CBM: pipeline/eval/store `metadata_changed`; `rg` arbiter.
## Task Commits
`e844b54` Horn rematch · `770150e` migrate refuse · `5168f45` docs/bench POST
## Self-Check: PASSED
