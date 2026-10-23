---
phase: 229-checkpoint-cut
plan: 01
subsystem: runtime-fork
tags: [fork_at, checkpoint, cut, ForkCutUnavailable]
requires: [{phase: 228-checkpoint-receipt, provides: "Op::Checkpoint sealed-prefix receipt"}]
provides: ["fallible fork_at + b3 REDs"]
affects: [229-02]
actuals: {tokens: 0, tasks: 1, commits: 0}
tech-stack: {added: [], patterns: [fork_at fail-closed at compacted cut]}
key-files:
  created: []
  modified: []
key-decisions: []
requirements-completed: [D-01]
duration: 0min
completed: 2026-10-23
status: in-progress
---
# Phase 229 Plan 01: checkpoint cut (fork_at)

**PRE recorded; REDs and fork_at Result land in later tasks.**

## Bench PRE

Waited 10×60s for `/root/prime-agent` `cargo test -p pa-core` (still looping). Ran bench anyway (not skipped). Host: 12-way, rustc 1.99.0.
- start 2026-10-23T02:57:57Z load **4.05 3.54 3.23**; concurrent `pa-core` pid 2367466
- end 2026-10-23T02:58:57Z load **5.32 4.09 3.44**; exit 0, `validated yes`
- **ingest_plain.n = 101894268 ns**
- **as_of.n = 247421 ns** (eager)
- also: ingest_plain.2n 183813797, ingest_plain.4n 414493410, as_of.2n 538864, as_of.4n 1595122
Cleaned `tmp/kutha-bench-plain-*`. crates/ untouched.

## RED (task 2)

(pending)

## old-to-new (task 3)

(pending)
