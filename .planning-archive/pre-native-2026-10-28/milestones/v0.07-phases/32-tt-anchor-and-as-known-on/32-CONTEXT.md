# Phase 32 Context: tt_anchor and as_known_on

**Milestone:** v0.07 / harness M014 S02
**Requirements:** ANK-01, ANK-02, ANK-03
**Depends on:** Phase 31 (clock newtypes)
**Source:** `docs/architecture/semantic-gap-review.md` F13; `.kutha/ROADMAP.md` M014 S02
**Run rules:** `docs/process/gsd-auto-contract.md`

## Locked decisions

- **D-01:** `tt_anchor` facts on the event log map transaction time to wall time (`WallTime` / date axis).
- **D-02:** `as_known_on(date)` resolves through the greatest anchor at or before the date, then evaluates the fold at that transaction time.
- **D-03:** A named test separates “true on D” (`as_of` / valid-time) from “known on D” (`as_known_on`); FF5 stays green.
- **D-04 (non-goals):** Fixture field migration beyond what anchors need (Phase 33); retract-by-EventId; rules; M002; five-legal-clocks sufficiency (ADR-013 open).
- **D-05:** Verifier is a different model from the executor.
- **D-06:** CBM graph-first; GitNexus secondary; `rg` arbiter.

## Verifier kill-tests

- Without an anchor at or before D, `as_known_on(D)` is empty / explicit miss (documented).
- With anchors A1@W1 and A2@W2 (W1 < D ≤ W2), resolution uses A1’s tt.
- `as_of(D)` and `as_known_on(D)` disagree on a fixture where valid truth and knowledge diverge.

## Non-goals

See D-04. Do not implement FIX-01/FIX-02 here.

## Harness

Active Milestone M014, Active Slice **S02** while this phase runs.
