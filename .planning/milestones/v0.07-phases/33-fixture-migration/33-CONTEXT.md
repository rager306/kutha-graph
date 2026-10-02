# Phase 33 Context: Fixture migration

**Milestone:** v0.07 / harness M014 S03
**Requirements:** FIX-01, FIX-02
**Depends on:** Phase 32
**Source:** F13; `.kutha/ROADMAP.md` M014 S03
**Run rules:** `docs/process/gsd-auto-contract.md`

## Locked decisions

- **D-01:** FF5 and M011/M012/M013 fixtures use explicit `ValidTime` / `TransactionTime` / `WallTime` constructors where literals touch the public clock API.
- **D-02:** Oracle assertion bodies are unchanged (rename-only); SUMMARY carries an old-to-new table.
- **D-03:** `uv run kutha-gov ci` is controller-owned at close (executor does not run it).
- **D-04 (non-goals):** New temporal features; retract-by-EventId; rules; M002; honeycomb promotion.
- **D-05:** Verifier ≠ executor.
- **D-06:** CBM graph-first; `rg` arbiter.

## Verifier kill-tests

- `git diff` on FF5/FF6/M011–M013 test files shows constructor wraps only — no `assert_eq!` / `assert!` body edits.
- Named oracles still green after wraps.
- Workspace tests green.

## Non-goals

See D-04. Do not invent new fixtures beyond wraps needed for typed clocks.

## Harness

Active Milestone M014, Active Slice **S03** while this phase runs.
