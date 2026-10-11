---
phase: 118-persist-durability
milestone: v0.48
harness: M054
slice: S02
ce_findings: "whole-repo P1 #9 #10"
---

# Phase 118 — CONTEXT

## Kill-claim

Torn empty active segments accept appends that disappear on reopen; full
persist rewrites stable seg-N paths before the manifest swap so a crash
can strand an old generation.

## Goal

#9 heal torn empty active under lock before append. #10 generation-unique
segment names, fsync, then atomic manifest. **#6 poison stays decision-gate.**

## Out of scope

#6 without human lift. Horn. Governor/scaling/ID. Honeycomb Proposed.
