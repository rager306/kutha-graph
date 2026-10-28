---
phase: 120-scaling-bench
milestone: v0.48
harness: M054
slice: S04
ce_findings: "whole-repo P2 #15 #16 #17 #19 #25 #27"
---

# Phase 120 — CONTEXT

## Kill-claim

Bench RSS/as_of can measure the wrong sample; append/retract/premise/pin
paths still scan when indexes should answer.

## Goal

Thin order: #15/#25 bench honesty first, then #16/#17/#19/#27 indexes.
Kill `SEM-gap-scaling-bench` (or split SEM-m054-scale-*).

## Out of scope

#6 poison. Governor (done). ID/structure S05. Honeycomb Proposed.
