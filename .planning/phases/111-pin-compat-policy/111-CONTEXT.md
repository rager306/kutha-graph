---
phase: 111-pin-compat-policy
milestone: v0.45
harness: M051
slice: S01
ce_finding: "P1 #24"
---

# Phase 111 — CONTEXT

## Kill-claim

`rule_definition_hash` for ident inputs moved from domain+UTF-8 to Named AST
digests without a versioned pin scheme or hardcoded stability vectors, so
stored pins and Hyrum consumers silently diverge (CE review P1 #24).

## Goal

Controller policy: **documented rotation to pin scheme v1; no silent remap**.
Pin in `docs/process/pin-compat.md`, CHANGELOG, hardcoded oracles
(`m051_pin_compat`), and governor check `pin-compat-policy`. Budget 75→76.

## Oracles

- `ident_pin_matches_hardcoded_v1_vector`
- `ident_pin_diverges_from_legacy_utf8_domain_vector`
- `nonident_pin_matches_hardcoded_domain_utf8_vector`
- check `pin-compat-policy`

## Out of scope

Persisted Behavior `rule_version` migrate; Op serde backfill; #26 clock facade;
#6. Honeycomb Proposed.
