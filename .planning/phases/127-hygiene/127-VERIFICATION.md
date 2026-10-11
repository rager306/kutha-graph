---
phase: 127-hygiene
independent: true
status: passed
verifier_model: parent-session
commands:
  - cmd: uv run kutha-gov precommit --check verification-uv-only
    exit: 0
  - cmd: uv run kutha-gov precommit --check statute-fixture-single-lang
    exit: 0
  - cmd: uv run kutha-gov selftest --check verification-uv-only
    exit: 0
  - cmd: uv run kutha-gov selftest --check statute-fixture-single-lang
    exit: 0
  - cmd: cargo test -p kutha-runtime --offline --test m025_demo_fixtures
    exit: 0
  - cmd: cargo test --workspace --offline --no-fail-fast
    exit: 0
  - cmd: KUTHA_GOV_BUDGET=78 uv run kutha-gov ci
    exit: 0
read_only_inputs:
  - .planning/phases/127-hygiene/127-CONTEXT.md
  - .planning/milestones/v0.13-phases/58-governor-smp/58-VERIFICATION.md
  - crates/kutha-runtime/fixtures/statute/punkt_15_1.txt
  - crates/kutha-runtime/tests/m025_demo_fixtures.rs
  - .kutha/dictionaries/checks.yaml
---

# Phase 127 — independent verification

**Verdict:** passed. #35 uv-only VERIFICATION + #36 single-language statute
fixture. Workspace cargo + full ci 0 HIGH at close (budget 78).
