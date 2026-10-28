---
phase: 117-horn-soundness
milestone: v0.48
harness: M054
slice: S01
ce_findings: "whole-repo P1 #4 #3 #2 #5"
---

# Phase 117 — CONTEXT

## Kill-claim

Horn fold can mint overlong VT heads, hide Correct re-fires from NormView,
keep dead multi-premise conclusions live, and drop uncovered VT residuals /
alternate proofs via overlap-only dedup.

## Goal

Fix order #4 → #3 → #2 → #5. RED oracles in `m054_horn_soundness.rs`.
Promote matrix `SEM-gap-horn-soundness` to killed rows (or split SEM-m054-*).

## Out of scope

Persist #6/#9/#10. Governor/scaling/ID. Honeycomb Proposed.
