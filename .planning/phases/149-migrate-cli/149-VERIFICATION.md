---
phase: 149-migrate-cli
independent: true
status: passed
verifier_model: parent-session
commands:
  - cmd: cargo test -p kutha-runtime --offline --test m078_migrate_cli
    exit: 0
  - cmd: uv run kutha-gov precommit --check op-compat-policy,pin-compat-policy,test-matrix,agents-lean
    exit: 0
  - cmd: uv run kutha-gov ci --skip-cargo
    exit: 0
read_only_inputs:
  - .planning/phases/149-migrate-cli/149-CONTEXT.md
  - crates/kutha-runtime/src/bin/kutha-migrate.rs
  - docs/process/op-compat.md
---

# Phase 149 — independent verification

**Verdict:** passed. `kutha-migrate` runs Op-wire then pins via `migrate_compat`;
missing dir exits 2; policy checks and ci tip green.
