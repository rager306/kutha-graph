# Phase 28 Context: Independent verification and artifact budget

**Milestone:** v0.06 / harness M013 S07
**Requirements:** PROC-01, PROC-02, PROC-03, PROC-04
**Depends on:** Phase 27
**Source of gaps:** `docs/architecture/semantic-gap-review.md` addendum R4, R5, R6
**Run rules:** `docs/process/gsd-auto-contract.md`

## Locked decisions

- **D-01:** Harness plane only (`scripts/kutha_gov`, `.kutha/dictionaries`, `docs/process`). No `crates/` change.
- **D-02:** The budget check is a YAML row in `checks.yaml` over planning files: per phase at most two PLAN files of 150 lines, one SUMMARY of 60, one VERIFICATION of 60, one CONTEXT of 80. `file_max_lines` takes one path, so add one rare kind `glob_max_lines` in `kinds.py` (with its selftest mutation target), or justify an existing kind. Fail closed on excess; unknown kind stays HIGH.
- **D-03:** Row order: invariant in `invariants.yaml` first, then the check (`docs/process/governor-intake.md`). The selftest mutation (an oversize file in the copy) must not be vacuous.
- **D-04:** Scope of the budget check: phases 22 onward only. Archived v0.01–v0.05 phases are history and are not rewritten.
- **D-05:** Independence is checked structurally, not by honor: every v0.06 `NN-VERIFICATION.md` carries frontmatter `independent: true`, `verifier_model`, `commands:` with exit statuses, and `read_only_inputs:` listing requirements and public API files (not PLAN or SUMMARY). The check fails if a VERIFICATION file lists a PLAN or SUMMARY path.
- **D-06:** One id map table (`docs/process/gsd-auto-contract.md` § Id map) is the single place; the governor cites it (a `cite` row), and `.planning/ROADMAP.md` and `.kutha/ROADMAP.md` link to it instead of re-stating it.
- **D-07:** `/gsd-audit-milestone` runs before archive (PROC-04); no override like the v0.02 closeout. Its findings feed the milestone archive. Tag and archive remain GSD tools.
- **D-08:** The verifier for this phase is not the author of Phase 22–27 verification files.

## Tool contract

CBM only where code is read (`scripts/kutha_gov/kinds.py`: `search_graph`, `get_code_snippet`, `check_index_coverage`). Governor: `uv run kutha-gov selftest`, `uv run kutha-gov precommit`, `uv run kutha-gov py`, `uv run pytest`.

## Verifier kill-tests

- Add a 200-line PLAN file in a scratch copy: the budget check fails. Remove it: passes.
- Add a VERIFICATION file listing a SUMMARY path under `read_only_inputs`: the independence check fails.
- `uv run kutha-gov selftest` reports every new check as proven (not vacuous).
- Phases 22–27 each have a conforming VERIFICATION file; `/gsd-audit-milestone` output exists and is read.

## Non-goals

Rewriting archived planning files, a new Python class, product code, tagging beyond the GSD archive.

## Harness

Active Milestone M013, Active Slice **S07** while this phase runs (controller leases it). Milestone close and archive follow this phase.
