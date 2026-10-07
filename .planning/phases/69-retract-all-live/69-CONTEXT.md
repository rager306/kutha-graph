# Phase 69: Retract all live residuals — Context

**Milestone:** v0.15 / harness M021 S02
**Requirements:** RETRACT-01
**Depends on:** Phase 68
**Run rules:** `docs/process/gsd-auto-contract.md`
**Gathered:** 2026-10-07
**Status:** Ready for planning
**Mode:** Controller + owner (GSD auto)

## Phase Boundary

Fix M020 residual: Retract by CorrectInterval (or any multi-row) event id must invalidate every live Fact sharing that `event_id`, not only the first live index. RED oracle first. Bench gate ≤10%. Honeycomb stays Proposed.

## Locked decisions

- **D-RETRACT-ALL-LIVE:** `Op::Retract { event_id }` invalidates all live facts with that event id. `fact_index_by_event_id` may stay “prefer first live” for Correct / CorrectInterval targeting one residual. Do not change Horn algebra or C01/TR-02.
- **D-SCOPE:** Product fold only; no legal pack; no fifth clock.

## Verifier kill-tests

- RED-then-green: after CorrectInterval that leaves ≥2 live residuals under one event id, one Retract clears all of them at the post-retract cut.
- Assert-only Retract still clears the single live row.
- Named Phase 66 pins stay green; bench ≤10% on `ingest_plain.n` / `as_of.n`.

## Harness

Active Milestone M021, Active Slice **S02**.
