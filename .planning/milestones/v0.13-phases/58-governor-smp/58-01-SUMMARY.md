# Phase 58 Summary (executor)

**Slice:** M019 S03 / PAR-01. Harness only. No `crates/` edit. No VERIFICATION (independent verifier).

## Commits

- `35b0a0d` `test(58-01): add RED pytest for governor --jobs parity`
- `6211c89` `feat(58-02): parallelize governor checks and selftest with --jobs`
- `836f6e5` `fix(58-03): isolate two-root jobs pytest from stdout races`

## RED

`uv run pytest scripts/tests/test_jobs.py -q` (`tmp/p58-red-pytest.log`): 10 failed, 1 passed. Failures: no `resolve_jobs`; `--help` lacks `--jobs`; argparse treats `--jobs N` as positional command (`invalid choice: '1'`). One vacuous pass: `--jobs 0` already exited 2 as unrecognized.

## Design

Process pool (`spawn`) for `run_checks` / `run_selftest`; merge by dictionary id. `--jobs 1` serial oracle (`eval_check` → `check-error`). N≥2: worker exceptions → HIGH `worker-error`. FSM `overlap_with` starts cargo/rocks on a thread when jobs>1; HIGH from checks/selftest still skips recording observe (serial skip). Scratch under repo `tmp/`; selftest copy skips `.compound-engineering` and `.planning/milestones`. Serial selftest before vs after skip: 58 derived / 4 explicit / 2 skip, rows identical (`tmp/p58-selftest-before.out` vs `tmp/p58-selftest-after-jobs1.out`). `--jobs 1 ci` vs pre-edit baseline: stdout equal (minus `phase_ms` / fold `runs=`); JSONL (op, subject, relation, object) equal.

## Timing (warm, 3 runs, mean s)

| cmd | jobs 1 | 2 | 4 | default | vs jobs 1 |
|-----|--------|---|---|---------|-----------|
| ci | 39.7 | 31.4 | 20.0 | 18.4 | **53.6% faster** |
| selftest | 19.5 | 28.8 | 14.9 | 14.6 | 25.0% faster |
| run_checks (`json`) | 2.89 | 2.26 | 2.00 | 2.02 | 30.2% faster |
| precommit | 2.56 | 2.37 | 2.04 | 2.29 | 10.4% faster |

Cold `ci` (fresh `CARGO_TARGET_DIR=tmp/p58-cold-target`, timeout 420, then deleted): exit 0, wall **167.5s**, `observe_cargo=140150` `run_selftest=20757` `run_checks=1873`.

`ci` met the 40% goal via cargo overlap. `selftest --jobs 2` is slower (per-check `copytree`). `precommit` is spawn-bound.

## Residuals

- Per-check selftest copies: jobs=2 > jobs=1 wall.
- `.env` `KUTHA_GOV_CARGO_TIMEOUT_SEC=180` still overrides rocks 420.
- CBM stale / `parallel.py` not_tracked (no `index_repository`).
- Isolation pytest must not share a patched stdout across threads.
- Kill before `finally` can leave `tmp/kutha-selftest-*`.

## Gates

- `uv run kutha-gov py`: PYTHON_TOOLING_GREEN
- `uv run kutha-gov precommit`: rc=0
- `selftest --jobs 1` and default: rc=0, identical 58/4/2 rows
- `ci --jobs 1` golden vs baseline; `ci` default rc=0 (~18s warm)
- Full pytest: 171 passed, 9 subtests, 102s (`tmp/p58-pytest.log`)
