---
phase: 109-skip-cargo-log-marker
milestone: v0.43
harness: M049
slice: S01
ce_finding: "P2 #38"
---

# Phase 109 — CONTEXT

## Kill-claim

`--skip-cargo` green runs append a pass row while `emit_tenant` returns without recording a skip, so fold tip `last_tenant` (and previously `last_cargo`) can stay a stale full-evidence value (CE review P2 #38).

## Goal

Record allowlisted `tenant=skipped` (with LOW `observe-skipped`) when emit_tenant is suppressed by skip_cargo / prior-high. Cargo tip already becomes `skipped` via M047 `_note_observe_skipped`. FSM budget 73→74. Pin fold tip in the oracle.

## Oracles

- `test_skip_cargo_does_not_call_cargo_or_tenant` (fold last_cargo/last_tenant)
- check `skip-cargo-log-marker`

## Out of scope

#6 decision-gate; G4 #14 compat policy. Honeycomb Proposed.
