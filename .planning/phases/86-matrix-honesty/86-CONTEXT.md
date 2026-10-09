# Phase 86: Matrix honesty — Context

**Milestone:** v0.20 / harness M026 S02  
**Requirements:** MAT-01  
**Run rules:** `docs/process/gsd-auto-contract.md`  
**Mode:** GSD auto

## Phase Boundary

Re-label honor-system / tautology matrix rows or give real product mutants. No product fold change. Optional cargo-mutants is verifier evidence, not a gate.

## Locked decisions

- **D-HONOR:** SEM-seeded-det → characterization; decision rows keep killed with honest kill_mutation text.
- **D-TAUT:** Naive as_of alias, known_at mirror, seed budget → characterization rows.
- **D-MUTANTS:** cargo-mutants not a CI gate.

## Verifier kill-tests

- `uv run kutha-gov precommit --check test-matrix` exit 0.
- `seed_budget_is_env_floor_not_domain` green.
