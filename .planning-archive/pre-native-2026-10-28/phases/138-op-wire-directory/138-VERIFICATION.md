---
phase: 138-op-wire-directory
independent: true
status: passed
verifier_model: parent-session
commands:
  - cmd: cargo test -p kutha-common --lib op_migrate --offline
    exit: 0
  - cmd: cargo test -p kutha-runtime --test m067_op_wire_migrate --offline
    exit: 0
  - cmd: cargo test --workspace --offline --no-fail-fast -- --test-threads=1
    exit: 0
  - cmd: uv run kutha-gov precommit
    exit: 0
  - cmd: uv run kutha-gov ci
    exit: 0
read_only_inputs:
  - .planning/phases/138-op-wire-directory/138-CONTEXT.md
  - docs/process/op-compat.md
  - crates/kutha-common/src/op_migrate.rs
  - crates/kutha-runtime/src/store.rs
  - crates/kutha-runtime/tests/m067_op_wire_migrate.rs
---

# Phase 138 — independent verification

**Verdict:** passed. Directory rewrite stamps marker; open fails until migrate; idempotent remigrate.
