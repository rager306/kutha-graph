---
phase: 92-derive-live-premise
verified: 2026-10-09T12:00:00Z
independent: true
verifier_model: composer-2.5-fast
status: passed
read_only_inputs:
  - .planning/phases/92-derive-live-premise/92-CONTEXT.md
  - crates/kutha-runtime/src/quantum.rs
  - crates/kutha-runtime/tests/m032_derive_live_premise.rs
commands:
  - { cmd: "cargo test -p kutha-runtime --test m032_derive_live_premise --offline", exit: 0 }
  - { cmd: "cargo test -p kutha-runtime --test facade_statute --offline", exit: 0 }
  - { cmd: "cargo test --workspace --offline --no-fail-fast", exit: 0 }
  - { cmd: "uv run kutha-gov ci", exit: 0 }
---

# Phase 92 — Independent Verification

Kill-claim: after retract of the latest subject fact, derive still mints Behavior with caused_by pointing at the invalidated Assert.

Workspace ci at close (0 HIGH). Quiet bench median within ≤10% vs M031 / Phase 80.

**Verdict:** passed.
