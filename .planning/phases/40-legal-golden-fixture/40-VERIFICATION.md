---
independent: true
verifier_model: cursor-grok-4.6-xhigh
status: passed
score: 1/1
exit: pass
commands:
  - cmd: "cargo test --offline -p kutha-runtime --test m016_golden_fixture --test ff5_legal_pit"; exit: 0
  - cmd: "rg -n 'FIXG-01 / M016 S05' docs/ADR/ADR-013-bitemporal-facts-invalidation.md"; exit: 0
  - cmd: "rg -n 'struct (ApplicationTime|PromulgationTime|EditionTime)' crates/"; exit: 1
  - cmd: "git diff --exit-code -- .kutha/STATE.md"; exit: 0
read_only_inputs:
  - .planning/REQUIREMENTS.md
  - .planning/ROADMAP.md
  - .planning/phases/40-legal-golden-fixture/40-CONTEXT.md
  - crates/kutha-common/src/rule.rs
  - crates/kutha-runtime/tests/m016_golden_fixture.rs
  - docs/ADR/ADR-013-bitemporal-facts-invalidation.md
  - .kutha/STATE.md
---

# Phase 40 — independent verification

**ROADMAP SC:** FIXG-01. Kill-tests: C01 quote + Horn cannot hold calendar sunset; ADR-013 Evidence fail-closed 2026-10-02; no fifth clock newtype; FF5 green.

| REQ | Status | Evidence |
|-----|--------|----------|
| FIXG-01 | satisfied | fail-closed dated Evidence; `fixg01_c01_sunset_inexpressible_as_horn` |

**Score:** 1/1. Residual: five clocks remain a later pack (ADR-090), not core axes. No `.kutha/STATE.md` edit this verify.
