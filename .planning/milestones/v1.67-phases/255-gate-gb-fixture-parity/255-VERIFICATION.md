---
phase: 255-gate-gb-fixture-parity
independent: true
status: passed
verifier_model: zai/glm-5.3-flash
commands:
  - cmd: cargo test -p kutha-runtime --test m174_gate_gb
    exit: 0
  - cmd: cargo test --workspace
    exit: 0
  - cmd: git diff -- crates/kutha-runtime/src docs/ADR/ .github/
    exit: 0
  - cmd: uv run kutha-gov precommit
    exit: 0
read_only_inputs:
  - crates/kutha-runtime/tests/m174_gate_gb.rs
  - crates/kutha-runtime/tests/fixtures/ecfr/
  - .compound-engineering/artifacts/research/2026-10-30-gate-gb-decision.md
  - .planning/phases/255-gate-gb-fixture-parity/255-01-SUMMARY.md
  - .planning/STATE.md
---

# Phase 255 verification — Gate GB fixture parity

Independent verifier PASSed the full fixture corpus: FF5 vt 2015/2020 (GPML == typed IR == `as_of` facade); B3 every layer-0 start at 1/2 hops; M167 eCFR 3 sections x 7 cuts GPML-vs-facade parity; unknown relation/term fail closed. Targeted test 2/2 passed; workspace 744 passed / 0 failed across 172 suites; precommit exit 0. No product/ADR/CI change.
