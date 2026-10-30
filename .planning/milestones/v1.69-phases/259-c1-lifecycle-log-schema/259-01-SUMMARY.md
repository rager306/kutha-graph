---
phase: 259-c1-lifecycle-log-schema
plan: 01
subsystem: event-schema
tags: [c1, pack-lifecycle, d021, log-schema]
requires: []
provides:
  - "typed PackInstall/PackActivate/PackRetire log events"
  - "fail-closed D021 lifecycle admission"
affects: [260-c1-lifecycle-runtime-lease]
actuals: {tokens: 0, tasks: 5, commits: 0}
tech-stack: {added: [], patterns: [lifecycle-meta-events, replayed-state-admission]}
key-files:
  created: [crates/kutha-runtime/tests/m176_pack_lifecycle_schema.rs]
  modified: [crates/kutha-common/src/event.rs, crates/kutha-runtime/src/admission.rs, crates/kutha-runtime/src/pipeline.rs, crates/kutha-runtime/src/commit.rs, crates/kutha-runtime/src/provenance.rs, crates/kutha-runtime/src/quantum.rs, crates/kutha-runtime/src/fold.rs]
key-decisions:
  - "Lifecycle ops are meta self-commit events with empty object_ids and no graph facts."
  - "Install/activate/retire transitions validate against replayed log state and fail closed."
requirements-completed: [C1-01]
coverage:
  - id: D1
    description: "Typed D021 lifecycle log schema and admission."
    requirement: C1-01
    verification:
      - kind: manual
        ref: "crates/kutha-runtime/tests/m176_pack_lifecycle_schema.rs"
        status: pass
    human_judgment: false
---

# Phase 259 summary — C1 lifecycle log schema

Added typed `Op::PackInstall` / `PackActivate` / `PackRetire` with pack id/name/version, deterministic digest bytes, empty object_ids, and lifecycle meta classification. Runtime exposes dictionary-mode `emit_pack_*` constructors. Admission replays existing lifecycle events and rejects duplicate installs, unknown installs, version mismatch, activate-after-activate, and retire-after-retire. Existing log/migration compatibility remains; graph facts/fingerprint unchanged. Schema tests 4/4; workspace 762 passed / 0 failed. It also rejects empty identifiers and user-forged Pack*; only dictionary-mode emit_pack_* is accepted.
