---
phase: 121-id-structure
milestone: v0.48
harness: M054
slice: S05
ce_findings: "whole-repo P2 #24 #12 #20 #21"
---

# Phase 121 — CONTEXT

## Kill-claim

Derived EventIds can leave the documented UUID contract; SEM helper and
kinds.py are oversized; shrink corpus uses a local FNV instead of canonical SHA.

## Goal

#24 EventId UUID contract; #20 shrink SHA reuse; #12/#21 structural splits
without behavior change. Kill `SEM-gap-id-and-structure`.

## Out of scope

#6 poison. Honeycomb Proposed.
