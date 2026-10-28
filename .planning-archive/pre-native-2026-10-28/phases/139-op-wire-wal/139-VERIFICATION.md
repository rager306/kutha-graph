---
phase: 139-op-wire-wal
independent: true
status: passed
verifier_model: parent-session
commands:
  - cmd: cargo test -p kutha-runtime --lib wal --offline
    exit: 0
  - cmd: cargo test -p kutha-runtime --test m068_op_wire_wal --test m067_op_wire_migrate --offline
    exit: 0
  - cmd: cargo test --workspace --offline --no-fail-fast -- --test-threads=1
    exit: 0
  - cmd: uv run kutha-gov precommit
    exit: 0
  - cmd: uv run kutha-gov ci
    exit: 0
read_only_inputs:
  - .planning/phases/139-op-wire-wal/139-CONTEXT.md
  - docs/process/op-compat.md
  - crates/kutha-runtime/src/store.rs
  - crates/kutha-runtime/src/wal.rs
  - crates/kutha-runtime/tests/m068_op_wire_wal.rs
---

# Phase 139 — independent verification

**Verdict:** passed. WAL-only rewrite stamps marker; open fails on legacy frames until migrate; idempotent remigrate.
