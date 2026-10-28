---
phase: 107-observe-skipped-breadcrumb
milestone: v0.41
harness: M047
slice: S01
ce_finding: "P2 #39"
---

# Phase 107 — CONTEXT

## Kill-claim

When the first `observe_cargo` fails (or `skip_cargo` / prior HIGH suppresses heavy observe), later observe states leave no triage breadcrumb (CE review P2 #39).

## Goal

Append a LOW `observe-skipped` finding naming FSM state and cause. Keep the observation relation allowlisted as `cargo` (same as `_apply_cargo_pack`) so emit_log does not fail-closed. FSM budget 71→72 for the new check.

## Oracles

- `test_second_observe_cargo_notes_skip_after_prior_high`
- check `observe-skipped-breadcrumb`

## Out of scope

#48 skip_emit/changed_paths payload, #6 decision-gate. Honeycomb Proposed.
