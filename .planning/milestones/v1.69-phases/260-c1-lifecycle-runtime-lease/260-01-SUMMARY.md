---
phase: 260-c1-lifecycle-runtime-lease
plan: 01
subsystem: pack-lifecycle
tags: [c1, pack-lifecycle, replay, lease]
requires: [259-01]
provides:
  - "replay-derived droppable PackLifecycle lease"
  - "Runtime lifecycle query"
affects: []
actuals: {tokens: 0, tasks: 4, commits: 0}
tech-stack: {added: [], patterns: [derived-lease, graph-neutrality]}
key-files:
  created: [crates/kutha-runtime/src/pack_lifecycle.rs, crates/kutha-runtime/tests/m176_pack_lifecycle_runtime.rs]
  modified: [crates/kutha-runtime/src/lib.rs, crates/kutha-runtime/src/pipeline.rs]
key-decisions:
  - "Lifecycle state derives only from admitted D021 events."
  - "Lifecycle lease does not affect graph facts or fingerprint."
requirements-completed: [C1-02]
coverage:
  - id: D1
    description: "Replay-derived lifecycle runtime lease."
    requirement: C1-02
    verification:
      - kind: manual
        ref: "crates/kutha-runtime/tests/m176_pack_lifecycle_runtime.rs"
        status: pass
    human_judgment: false
---

# Phase 260 summary — C1 lifecycle runtime lease

Added `pack_lifecycle.rs` and `Runtime::pack_lifecycle()`. State derives deterministically from admitted D021 events in first-install order, with installed/active/retired states and event ids. Tests cover graph neutrality, persist/open parity, fork-at prefix cuts, and product graph event isolation. Runtime tests 5/5, including checkpoint_sealed_prefix survival; workspace 762 passed / 0 failed.
