---
phase: 253-m003a-parser-subset
independent: true
status: passed
verifier_model: zai/glm-5.3-flash
commands:
  - cmd: cargo test -p kutha-runtime --test m003a_gpml --test m003a_gpml_differential
    exit: 0
  - cmd: cargo test --workspace
    exit: 0
  - cmd: uv run kutha-gov precommit
    exit: 0
  - cmd: git diff -- crates/kutha-runtime/src/query_ir.rs docs/ADR/ .github/
    exit: 0
read_only_inputs:
  - crates/kutha-runtime/src/gpml.rs
  - crates/kutha-runtime/tests/m003a_gpml.rs
  - crates/kutha-runtime/tests/m003a_gpml_differential.rs
  - crates/kutha-runtime/src/query_ir.rs
  - crates/kutha-runtime/src/allow.rs
  - crates/kutha-runtime/dictionaries/relations.yaml
  - .planning/STATE.md
---

# Phase 253 verification — M003a parser subset

Two independent verifier contexts returned PASS. They confirmed the parser is read-only M003a only, maps to existing `QueryIr::Hops`, uses dictionary + FF6 allowlist binding with zero intern calls, preserves typed TT/VT/limit, and fails closed on unknown vocabulary/out-of-scope constructs. Targeted tests 7+4 passed; workspace 741 passed / 0 failed; precommit exit 0. No QueryIr/ADR/Op/CI changes.

Post-verifier hygiene: removed a new clippy warning from the differential test and added one local mixed repeated/per-hop chain test (`RelsShape` at prepare), which passes.
