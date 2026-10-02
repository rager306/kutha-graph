# Phase 35 Context: Living docs and governor honesty

**Milestone:** v0.08 / harness M015 S02
**Requirements:** DOC-01, GOV-01
**Depends on:** Phase 34
**Source:** post-M014 audit stale-doc table; governor erosion notes
**Run rules:** `docs/process/gsd-auto-contract.md`

## Locked decisions

- **D-01:** Living docs (ROADMAP M013 CLOSED text, PROJECT idle→M015, semantic-gap F13, ADR-013 Evidence, p0-spike, legal-fixture-recon, CHANGELOG order, id-map-cite needles) match post-M014 + Phase 34 residual.
- **D-02:** `true_on_d_differs_from_known_on_d` in `observe_cargo.required`; honeycomb ADR-013 evidence lists it and M015 known-on oracles.
- **D-03:** With live `.planning/phases/` non-empty, artifact-budget PLAN/SUMMARY/VERIFICATION/CONTEXT globs use `min_hits: 1` again; keep `allow_empty` only if still needed for verification-independent when empty — prefer min_hits 1 now that phases are live.
- **D-04 (non-goals):** M016 rules; five clocks; intern/WAL product fixes; honeycomb Accepted.
- **D-05:** Verifier ≠ executor.
- **D-06:** CBM for any code cites; mostly docs this phase.

## Verifier kill-tests

- `rg` living files: no "F13 stays open for M014", no M013 LEASED heading, no PROJECT "Current Milestone: v0.07".
- `kutha-gov precommit --check` paths / FSM required list includes ANK test name.
- `uv run kutha-gov ci` 0 HIGH with live phases and min_hits 1.

## Non-goals

See D-04.

## Harness

Active Milestone M015, Active Slice **S02** while this phase runs.
