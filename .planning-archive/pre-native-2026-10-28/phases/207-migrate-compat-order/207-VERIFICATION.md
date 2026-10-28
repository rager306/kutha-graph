---
phase: 207-migrate-compat-order
independent: true
status: passed
verifier_model: parent-session
commands:
  - cmd: cargo test -p kutha-runtime --test m133_migrate_compat_order --offline
    exit: 0
  - cmd: cargo test -p kutha-runtime --test m076_migrate_compat --offline
    exit: 0
  - cmd: uv run kutha-gov ci
    exit: 0
read_only_inputs:
  - crates/kutha-runtime/src/store.rs
  - crates/kutha-runtime/tests/m133_migrate_compat_order.rs
  - docs/architecture/test-matrix.yaml
---

# Phase 207 — independent verification

**Verdict:** passed. RED: swapped `migrate_pins` before `migrate_op_wire` fails oracle. GREEN: source order op-wire then pins; `m076_migrate_compat` still green; `kutha-gov ci` run_high=0 (97 checks).
