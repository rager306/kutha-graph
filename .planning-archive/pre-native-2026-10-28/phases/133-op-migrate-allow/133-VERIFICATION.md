---
phase: 133-op-migrate-allow
independent: true
status: passed
verifier_model: parent-session
commands:
  - cmd: cargo test -p kutha-common --lib op_migrate --offline
    exit: 0
  - cmd: cargo test -p kutha-runtime --test m062_op_migrate --test m050_op_compat --offline
    exit: 0
  - cmd: cargo test --workspace --offline --no-fail-fast
    exit: 0
  - cmd: uv run kutha-gov precommit
    exit: 0
  - cmd: uv run kutha-gov ci
    exit: 0
read_only_inputs:
  - .planning/phases/133-op-migrate-allow/133-CONTEXT.md
  - docs/process/op-compat.md
  - crates/kutha-common/src/op_migrate.rs
  - crates/kutha-runtime/tests/m062_op_migrate.rs
  - crates/kutha-runtime/tests/m050_op_compat.rs
---

# Phase 133 — independent verification

**Verdict:** passed. AllowRelation remaps via explicit migrate; Op serde still rejects legacy tags; `kutha-gov ci` 0 HIGH (h5_selftest 82/82).
