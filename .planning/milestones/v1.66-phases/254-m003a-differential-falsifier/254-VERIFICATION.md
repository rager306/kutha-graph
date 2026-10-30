---
phase: 254-m003a-differential-falsifier
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
  - crates/kutha-runtime/tests/m003a_gpml_differential.rs
  - crates/kutha-runtime/tests/m003a_gpml.rs
  - crates/kutha-runtime/src/gpml.rs
  - crates/kutha-runtime/src/query_ir.rs
  - crates/kutha-runtime/dictionaries/relations.yaml
  - .planning/STATE.md
---

# Phase 254 verification — M003a differential falsifier

Independent verifier PASSed B3 render/parse parity for every layer-0 start (1 and 2 hops), FF5 `GPML AS OF` equality with the typed facade at 2015/2020, typed TT/limit preservation, and fail-closed unknown vocabulary/out-of-scope constructs. Workspace 741 passed / 0 failed; precommit exit 0; no QueryIr/ADR/Op/CI changes.

A post-verifier local test was added for mixed repeated/per-hop chains: it parses but fails closed at `prepare` with `RelsShape { rels: 2, k: 3 }`. This is recorded as hygiene, not an independent finding.
