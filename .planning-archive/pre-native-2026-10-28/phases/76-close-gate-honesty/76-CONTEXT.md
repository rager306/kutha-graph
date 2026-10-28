# Phase 76: Close-gate honesty — Context

**Milestone:** v0.17 / harness M023 S02  
**Requirements:** GATE-02  
**Depends on:** Phase 75  
**Mode:** Controller + owner (GSD auto)

## Locked

- Correct, do not rewrite history: 72-VERIFICATION and `v0.16-MILESTONE-AUDIT.md` get a dated correction note (verification ran only `m022_*`); CHANGELOG gets a new entry.
- `docs/process/gsd-auto-contract.md` close step requires a recorded full `cargo test --workspace --offline --no-fail-fast` and full `uv run kutha-gov ci`; `precommit` is not a product close gate. A VERIFICATION `commands:` list for a product phase includes the workspace run.
- Governor proof is a YAML row (`.kutha/META.md`, `docs/process/governor-intake.md`), not a new Python class; selftest proves it.
- No `crates/` change.

## Verifier kill-tests

- Selftest mutation removing the checklist line is caught.
- 72-VERIFICATION and v0.16 audit mention the regression and the fixing commit.
