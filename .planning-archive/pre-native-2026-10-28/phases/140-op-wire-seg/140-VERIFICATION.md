---
phase: 140-op-wire-seg
independent: true
status: passed
verifier_model: parent-session
commands:
  - cmd: cargo test -p kutha-runtime --test m069_op_wire_seg --test m068_op_wire_wal --test m067_op_wire_migrate --offline
    exit: 0
  - cmd: cargo test --workspace --offline --no-fail-fast -- --test-threads=1
    exit: 0
  - cmd: uv run kutha-gov precommit
    exit: 0
  - cmd: uv run kutha-gov ci
    exit: 0
read_only_inputs:
  - .planning/phases/140-op-wire-seg/140-CONTEXT.md
  - docs/process/op-compat.md
  - crates/kutha-runtime/src/store.rs
  - crates/kutha-runtime/src/segment.rs
  - crates/kutha-runtime/tests/m069_op_wire_seg.rs
---

# Phase 140 — independent verification

**Verdict:** passed. Segmented rewrite demotes to jsonl + marker; open fails on sealed legacy frames until migrate; idempotent remigrate.
