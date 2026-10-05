# Phase 59 Summary (executor)

**Slice:** M019 S04 / TEST-01. Docs + governor only. No `crates/` edit. No named M010–M017 oracle change. No C01 / Horn / `as_of` decision.

## Commits

- `359fdca` `docs(59-01): record test-strategy prior-art verdicts for D-02(e)`
- `e8981e8` `docs(59-02): add Proposed ADR-023 and the oracle matrix map`
- `783dd91` `test(59-03): add RED pytest for the test-matrix governor check`
- `4ec971a` `feat(59-04): interpret the oracle matrix with kind test_matrix`

## RED

`scripts/tests/test_test_matrix.py`: 10/10 failed on `test-matrix check is not registered` before the YAML row/kind existed (CHANGELOG 2026-10-05). After `feat(59-04)` the same file is green (10 passed; live tree no HIGH).

## Matrix counts

65 rows: **killed 34**, **gap 17**, **blocked 6**, **spec-only 4**, **characterization 3**, **gate 1**. Gaps: S05 (11 durability A), S06 (3 durability B), S07 (3 semantic). Blocked: HORN-KC-01, C01, ASOF-ELIG-01, DET-02, TR-02, HORN-CORR-01. `unowned` rejected.

## Check design

New kind `test_matrix` (intake: `yaml_map_list` cannot express ROADMAP `M###-S##` headings, killed×`kill_mutation≠none`, or `feature:` → `observe_cargo` state). Check `test-matrix`. Product oracles reuse `_prove_named_test`. Selftest derived-by-own-mutation (rename first product oracle to `__selftest_missing_oracle__`). Classification **59 / 4 / 2**. Budget 65. Matrix not in `observe_cargo.required`. Green = map consistency, not “product tested”.

## Residuals

- CBM `scripts/kutha_gov` metadata_changed; new yaml/pytest not_tracked (no `index_repository`).
- Stale `target/` unit from `tmp/p58-head` baked `CARGO_MANIFEST_DIR`; `cargo clean -p kutha-runtime` then workspace tests green. Not a crate change.
- `test_ci_green_on_this_tree` still stubs cargo.

## Gates

- `kutha-gov py`: green
- `precommit`: rc=0 (includes `test-matrix`)
- `selftest`: EXIT 0; `test-matrix` derived; 65/65
- `ci`: 0 HIGH, 1 LOW, 65 checks; cargo=ok (~31 s warm)
- `pytest scripts/tests -q`: 181 passed in 101.83 s (`tmp/p59-pytest.log`)

## Open questions (not decided)

C01 / FIXG-01, HORN-KC-01, ASOF-ELIG-01, DET-02, TR-02, HORN-CORR-01 — stay `blocked`.
