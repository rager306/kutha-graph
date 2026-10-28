---
phase: 111-pin-compat-policy
plan: 01
---

# Phase 111 Plan 01 — Summary

## What shipped

- `docs/process/pin-compat.md` — documented rotation to pin scheme v1.
- `crates/kutha-common/tests/m051_pin_compat.rs` — three hardcoded stability vectors.
- Check `pin-compat-policy`; invariant `I-pin-compat-policy`; matrix `SEM-m051-pin-compat-policy`.
- FSM / `.env.example` budget 75→76.
- `rule_definition_hash` docstring cites pin scheme v1 (no algorithm change).

## Residuals

- UTF-8→v1 Behavior pin migrate deferred.
- G4 next: #26 (clock `Into` facade), then #27/#28/#31.
- #6 decision-gate unchanged.
