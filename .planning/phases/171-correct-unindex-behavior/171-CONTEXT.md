# Phase 171 Context — Correct unindexes Behavior

**Milestone:** v0.94 (M100-S01)
**Status:** ready (discuss skipped — lease names the slice)

## Decision

`append_indexed` already calls `unindex_behavior` on `Op::Retract`. `Op::Correct` and `Op::CorrectInterval` invalidate that Behavior fact in the fold and must call the same unindex for the corrected `event_id`, so `live_behaviors`, `derived_by_cause`, and `behaviors_by_premise_rel` do not keep a dead head.

## Oracle

Retract the premise after Correct (and after CorrectInterval) of the derived `knownBy` head. `retract_examine_count` is 0. Today the dead head is still indexed, so the count is > 0.

## Out of scope

Single-combination DFS, rule-pin point gating, unindex reverse-pointer, honeycomb thaw.
