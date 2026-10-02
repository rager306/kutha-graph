# Phase 31 Context: Clock newtypes

**Milestone:** v0.07 / harness M014 S01
**Requirements:** CLK-01, CLK-02, CLK-03
**Depends on:** M013 closed
**Source:** `docs/architecture/semantic-gap-review.md` F13; `.kutha/ROADMAP.md` M014 S01
**Run rules:** `docs/process/gsd-auto-contract.md`

## Locked decisions

- **D-01:** Newtypes on the public temporal API: `Vt` (valid time), `Tt` (transaction time), and a wall-clock type. Bare `u64` aliases that compile interchangeably are forbidden on that API.
- **D-02:** A `compile_fail` doctest or trybuild proves a silent mix is rejected.
- **D-03:** Named oracles (FF5, FF6, M010–M013) stay green. Mechanical renames only; old-to-new table in SUMMARY. No assertion edits.
- **D-04 (non-goals):** `tt_anchor` / `as_known_on` (Phase 32); fixture migration beyond renames (Phase 33); retract-by-EventId; rules; M002; five-legal-clocks sufficiency (ADR-013 open).
- **D-05:** Verifier is a different model from the executor.
- **D-06:** Tooling: CBM graph-first on touched symbols; GitNexus secondary unless the user asks (outside M013 dual-tool contract). `rg` is arbiter.

## Verifier kill-tests

- A doctest/trybuild that mixes `Vt` and `Tt` without conversion fails to compile.
- `as_of`-shaped calls still type-check with `Vt`; FF5 green.
- Renaming only: no oracle assertion bodies changed (`git diff` review).

## Non-goals

See D-04.

## Harness

Active Milestone M014, Active Slice **S01** while this phase runs (controller leases it after M013 closes).
