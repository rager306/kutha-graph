---
phase: 134-op-migrate-rule
independent: true
status: passed
verifier_model: parent-session
commands:
  - cmd: cargo test -p kutha-common --lib op_migrate --offline
    exit: 0
  - cmd: cargo test -p kutha-runtime --test m063_op_migrate --test m062_op_migrate --test m050_op_compat --offline
    exit: 0
  - cmd: cargo test --workspace --offline --no-fail-fast -- --test-threads=1
    exit: 0
  - cmd: uv run kutha-gov precommit
    exit: 0
  - cmd: uv run kutha-gov ci
    exit: 0
read_only_inputs:
  - .planning/phases/134-op-migrate-rule/134-CONTEXT.md
  - docs/process/op-compat.md
  - crates/kutha-common/src/op_migrate.rs
  - crates/kutha-runtime/tests/m063_op_migrate.rs
---

# Phase 134 — independent verification

**Verdict:** passed. RegisterRule expands to StoreBlob+Assert; serde still rejects legacy tags; `kutha-gov ci` 0 HIGH.
