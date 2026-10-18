# Phase 175 Context — unindex cause reverse pointer

**Milestone:** v0.98 (M104-S01)
**Status:** ready (discuss skipped — lease names the slice)

## Decision

`unindex_behavior` walks every `derived_by_cause` value. Keep `cause_of_behavior: EventId → EventId` when indexing a Behavior, and on unindex remove from that one bucket (then drop empty). Premise-rel buckets stay a separate residual. Count cause-bucket visits for the oracle.

## Oracle

Correct one derived `knownBy` after D live inverse Behaviors. Cause-bucket examines stay constant as D grows (32 vs 128). Today examines grow with D.

## Out of scope

Premise-rel reverse index, single-combination DFS, policy-pin election, honeycomb thaw.
