---
phase: 113-cascade-atomicity
milestone: v0.46
harness: M052
slice: S02
ce_finding: "whole-repo P1 #8"
---

# Phase 113 — CONTEXT

## Kill-claim

`commit_user_cascade` commits each hop before later `rule_follow_ons` errors,
leaving a live prefix without QuantumOutcome that reopen hides.

## Goal

Stage the bounded cascade; commit sequence + outcome together or roll back.
RED oracle first: unknown later-hop head → Err leaves fold/log unchanged.

## Oracles

- Named tests in `m052_cascade_atomic.rs` (TBD exact names in PLAN)
- Matrix row `SEM-m052-cascade-atomic`

## Out of scope

Horn VT intersection (#2), multi-premise witness (#3). Honeycomb Proposed.
