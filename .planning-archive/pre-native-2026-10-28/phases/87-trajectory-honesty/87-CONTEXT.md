# Phase 87: Trajectory honesty — Context

**Milestone:** v0.21 / harness M027 S01  
**Requirements:** TRAJ-01  
**Run rules:** `docs/process/gsd-auto-contract.md`  
**Mode:** GSD auto

## Phase Boundary

Fail-closed trajectory: characterization-shaped `killed` rows and idle ROADMAP `ACTIVE`. No `crates/` change. Reuse existing kinds. Honeycomb stays Proposed.

## Locked decisions

- **D-CHAR-KILL:** `killed` + mirror/alias/honor-system/env-floor kill text → HIGH (`matrix-killed-characterization`).
- **D-IDLE-ACTIVE:** ROADMAP `— ACTIVE` forbidden while Active Milestone is None (`glob_absent` + `unless_file_contains`).
- **D-CITE-ROADMAP:** `cite-lease` compares ROADMAP ACTIVE headings to STATE.

## Verifier kill-tests

- `test_killed_with_characterization_shaped_kill_is_high` is RED on old kind, green after.
- `uv run kutha-gov precommit --check test-matrix` and `--check trajectory` exit 0.
