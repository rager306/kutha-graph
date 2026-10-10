---
phase: 110-op-compat-policy
milestone: v0.44
harness: M050
slice: S01
ce_finding: "P1 #14"
---

# Phase 110 — CONTEXT

## Kill-claim

Five dictionary meta-ops were removed with no serde backfill or format marker, so main-era dirs fail open without a recorded compat policy (CE review P1 #14).

## Goal

Controller policy: **documented break; serde backfill deferred**. Pin in `docs/process/op-compat.md`, CHANGELOG, characterization oracles (`m050_op_compat`), and governor check `op-compat-policy`. Budget 74→75.

## Oracles

- `legacy_allow_relation_op_fails_to_deserialize`
- `modern_kutha_allows_assert_roundtrips`
- check `op-compat-policy`

## Out of scope

Serde TermId remap / migrate implementation; #24 pin rotation; #6. Honeycomb Proposed.
