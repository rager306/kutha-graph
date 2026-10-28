---
phase: 135-op-migrate-policy
independent: true
status: passed
verifier_model: parent-session
commands:
  - cmd: cargo test -p kutha-common --lib op_migrate --offline
    exit: 0
  - cmd: cargo test -p kutha-runtime --test m064_op_migrate --test m063_op_migrate --test m050_op_compat --offline
    exit: 0
  - cmd: cargo test --workspace --offline --no-fail-fast -- --test-threads=1
    exit: 0
  - cmd: uv run kutha-gov precommit
    exit: 0
  - cmd: uv run kutha-gov ci
    exit: 0
read_only_inputs:
  - .planning/phases/135-op-migrate-policy/135-CONTEXT.md
  - docs/process/op-compat.md
  - crates/kutha-common/src/op_migrate.rs
  - crates/kutha-runtime/tests/m064_op_migrate.rs
---

# Phase 135 — independent verification

**Verdict:** passed. PinPolicy expands to StoreBlob+Assert; serde still rejects legacy tags; `kutha-gov ci` 0 HIGH.
