---
phase: 112-triage-correction
milestone: v0.46
harness: M052
slice: S01
ce_finding: "whole-repo #1 demoted"
---

# Phase 112 — CONTEXT

## Kill-claim

Whole-repo report listed Op-decode #1 as P1/fix-first though `op-compat.md` and
`m050_op_compat` already pin a documented break; that misorders the apply queue.

## Goal

Controller/advisor correction: #1 is **not a defect**. Artifact
`advisor-correction.md`, STATE/ROADMAP/gsd-auto-contract cite corrected
fix-first order starting at #8. Budget unchanged (no new check required if
needles land in existing docs + artifact).

## Oracles

- `docs/process/op-compat.md` still says documented break / backfill deferred
- `.compound-engineering/artifacts/reviews/2026-10-11-ce-whole-repo/advisor-correction.md` present with demotion language
- `.kutha/STATE.md` Next action names #8 first (not #1 migrate as defect)

## Out of scope

Implementing Op migrate; product code. Honeycomb Proposed.
