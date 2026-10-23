---
phase: 229-checkpoint-cut
plan: 01
subsystem: runtime-fork
tags: [fork_at, checkpoint, cut, ForkCutUnavailable]
requires: [{phase: 228-checkpoint-receipt, provides: "Op::Checkpoint sealed-prefix receipt"}]
provides: ["fallible fork_at + b3 REDs"]
affects: [229-02]
actuals: {tokens: 0, tasks: 2, commits: 0}
tech-stack: {added: [], patterns: [fork_at fail-closed at compacted cut]}
key-files:
  created: [crates/kutha-runtime/tests/b3_checkpoint_cut.rs]
  modified: []
key-decisions: []
requirements-completed: [D-01, D-03]
duration: 0min
completed: 2026-10-23
status: in-progress
---
# Phase 229 Plan 01: checkpoint cut (fork_at)

**PRE recorded; four b3 REDs fired against infallible fork_at (Horn F1 confirmed).**

## Bench PRE

Waited 10×60s for `/root/prime-agent` `cargo test -p pa-core` (still looping). Bench not skipped. 12-way, rustc 1.99.0.
- 2026-10-23T02:57:57Z load **4.05 3.54 3.23** concurrent pa-core 2367466 → 02:58:57Z **5.32 4.09 3.44** exit 0
- **ingest_plain.n = 101894268 ns**; **as_of.n = 247421 ns** (eager)

## RED (verbatim, before production)

`cargo test -p kutha-runtime --offline --test b3_checkpoint_cut` **exit 101**. PersistScratch left `tmp/` empty.
1. **interior FIRED** exact `--exact fork_at_interior_of_checkpointed_prefix_is_err` exit 101: `assertion \`left != right\` failed: fork_at(1) interior of checkpointed prefix is a hole (Self, want Err)` `left: 0` `right: 0` (infallible Self, empty fold, not Err).
2. **past-end FIRED**: `assertion \`left != right\` failed: fork_at(14) currently returns the full log; want Err` `left: 13` `right: 13`.
3. **Horn FIRED (F1 not dropped)**: `assertion \`left == right\` failed: Horn un-derive fingerprint vs twin after live emit` left `[230, 115, 119, 6, … 100, 106]` right `[252, 224, 93, 58, … 219, 156]`. emit(Retract) on both runtimes succeeded; fingerprints differ.
4. **migrate FIRED**: `migrate_compat must refuse Checkpoint: CompatMigrateReport { op_wire: OpWireMigrateReport { remapped_lines: 0, events_written: 40, already_migrated: false, dropped_tail: 0 }, pins: … }`.

## old-to-new (task 3)

(pending)
