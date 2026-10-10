---
phase: 95-replay-dict-bind
verified: 2026-10-10T03:20:00Z
independent: true
verifier_model: composer-2.5-fast
status: passed
read_only_inputs:
  - .planning/phases/95-replay-dict-bind/95-CONTEXT.md
  - crates/kutha-runtime/src/fold.rs
  - crates/kutha-runtime/src/provenance.rs
  - crates/kutha-runtime/tests/m035_replay_dict_bind.rs
commands:
  - { cmd: "cargo test -p kutha-runtime --test m035_replay_dict_bind --offline", exit: 0 }
  - { cmd: "cargo test -p kutha-runtime --test m016_known_on_hardening --offline", exit: 0 }
  - { cmd: "cargo test --workspace --offline --no-fail-fast", exit: 0 }
  - { cmd: "uv run kutha-gov ci", exit: 0 }
---

# Phase 95 — Independent Verification

Kill-claim: `replay_check` returns a dictionary-blind fold whose product `as_of` includes genesis `kutha:allows` while the live Runtime excludes them.

Workspace ci at close (0 HIGH). Quiet bench median within ≤10% vs M034 / Phase 80.

**Verdict:** passed.
