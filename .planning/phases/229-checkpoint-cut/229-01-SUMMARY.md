---
phase: 229-checkpoint-cut
plan: 01
subsystem: runtime-fork
tags: [fork_at, checkpoint, cut, ForkCutUnavailable]
requires: [{phase: 228-checkpoint-receipt, provides: "Op::Checkpoint sealed-prefix receipt"}]
provides: ["fallible fork_at Result + b3 fork REDs GREEN"]
affects: [229-02]
actuals: {tokens: 8416, tasks: 3, commits: 3}
plan_head_before: 8ef4ef1ac53404987bd88424b89adf52dddb0c3a
tech-stack: {added: [], patterns: [fork_at fail-closed at compacted cut]}
key-files: {created: [crates/kutha-runtime/tests/b3_checkpoint_cut.rs], modified: [crates/kutha-runtime/src/hydrate.rs, crates/kutha-runtime/src/quantum.rs, crates/kutha-runtime/src/pipeline.rs]}
key-decisions: ["ForkCutUnavailable {cut, log_len}; Display fork cut {cut} unavailable (log_len {log_len}); EventLog::prefix unchanged"]
requirements-completed: [D-01, D-02, D-03, D-04]
duration: 30min
completed: 2026-10-23
status: complete
---
# Phase 229 Plan 01: checkpoint cut (fork_at)
**`fork_at` is `Result<Self, ForkCutUnavailable>`; interior/past-end GREEN; Horn F1 and migrate stay RED for 02.**
## Bench PRE
Waited 10×60s; `/root/prime-agent` `cargo test -p pa-core` still looping. Bench not skipped. 12-way, rustc 1.99.0.
- 2026-10-23T02:57:57Z load **4.05 3.54 3.23** (pa-core 2367466) → 02:58:57Z **5.32 4.09 3.44** exit 0
- **ingest_plain.n = 101894268 ns**; **as_of.n = 247421 ns** (eager)
## RED (verbatim, before production)
`cargo test -p kutha-runtime --offline --test b3_checkpoint_cut` **exit 101**. `tmp/` empty.
1. **interior FIRED** exact exit 101: `assertion \`left != right\` failed: fork_at(1) interior of checkpointed prefix is a hole (Self, want Err)` `left: 0` `right: 0`.
2. **past-end FIRED**: `fork_at(14) currently returns the full log; want Err` `left: 13` `right: 13`.
3. **Horn FIRED (F1 not dropped)**: fingerprint mismatch after live `emit(Retract)` on both runtimes (left `[230, 115, …]`, right `[252, 224, …]`).
4. **migrate FIRED**: `migrate_compat must refuse Checkpoint: CompatMigrateReport { … already_migrated: false … }`.
## old-to-new (D-02)
| file | old | new |
| src/hydrate.rs | -> Self | -> Result<Self, RuntimeError> |
| src/pipeline.rs | self.fork_at(prefix) | .expect("rollback prefix was log.len() before the failed cascade") |
| src/quantum_invariants.rs | rt.fork_at(cut) | .unwrap() |
| examples/statute_as_of.rs | opened.fork_at(0) | .unwrap() |
| tests/m017_verify44_killtests.rs | rt.fork_at(rt.log().len()) | .unwrap() |
| tests/m016_known_on_hardening.rs | rt.fork_at(cut) | .unwrap() |
| tests/facade_statute.rs | opened.fork_at(0) | .unwrap() |
| tests/m053_ce_review_regressions.rs | rt.fork_at(rt.log().len()) | .unwrap() |
| tests/m013_fold_indexes.rs | opened.fork_at(len); parent.fork_at(n) | .unwrap() both |
| tests/m013_genesis_log.rs | rt.fork_at(cut) | .unwrap() |
| tests/m013_dictionary_facts.rs | rt.fork_at(cut_a); rt.fork_at(cut) | .unwrap() both |
| tests/m012_admission_facts.rs | rt.fork_at(n) | .unwrap() |
| tests/m012_rule_registry.rs | rt.fork_at(n) | .unwrap() |
| tests/m012_allowlist_facts.rs | rt.fork_at(n) | .unwrap() |
| tests/m012a_stable_refs.rs | rt.fork_at(rt.log().len()) | .unwrap() |
## GREEN / leftover RED (plan 02)
Interior exact **exit 0**; past-end **exit 0**. `cargo test -p kutha-runtime --offline --no-fail-fast` **101** only `b3_checkpoint_cut`: Horn + migrate still fail (plan 02). `.prefix(` call remains hydrate.rs only; `log.rs` untouched.
## Deviations
Rule 3: `expect_err` needs `Debug` on `Runtime` — match `Err` instead. rustfmt --edition 2024 wrap/imports on owned files only. `.planning/STATE.md` not edited (executor hard rule).
## Self-Check: PASSED
