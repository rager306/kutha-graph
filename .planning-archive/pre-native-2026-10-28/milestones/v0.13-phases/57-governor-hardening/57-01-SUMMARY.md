# Phase 57 Summary (executor)

**Slice:** M019 S02 / GOV-04–06. Harness only. No `crates/` edit.

## Commits

- `2af9ef3` `feat(57-01): parse Cargo freeze deps with cargo_dep_constraint`
- `13de517` `feat(57-02): enrol M002 oracles and strict artifact-budget`
- `5382a5d` `feat(57-03): classify selftest and derive own-rule mutations`
- `ea25c98` `fix(57-03): format mutation target and narrow freeze rules for ty`
- `7e27fbe` `fix(57-04): hermetic pytest ci and cache rust_test_asserts index`

## RED (GOV-04)

Old `glob_absent` on FreezeDepTests (`tmp/p57-gov04-red.log`): 6 failed, 12 passed (package rename, assignment without spaces, same-line `# optional = true`, other-key optional, table form, workspace table). Comment-only passed.

## Cause (pytest `test_ci_green_on_this_tree`)

`tmp/p57-pytest-full.log`: 59 min, FAIL. `phase_ms`: `run_checks=112833` `run_selftest=1004839` `observe_cargo=180111` `observe_cargo=0`. First HIGH: `cargo test timed out`; `TimeoutExpired` returns empty stdout so every `required` name is `observe-missing` (65 HIGH). Second observe skipped (`_skip_heavy_observe` after HIGH). Not name mismatch or `observe_rocks` taint. Pytest env: `CARGO_TARGET_DIR` unset, `KUTHA_GOV_CARGO_TIMEOUT_SEC=180` from `.env` (overrides rocks 420). After warm `--no-run` (workspace 2s, rocks 75s) standalone `ci` `observe_cargo=35076` `observe_rocks=4706` — suite fits 180s when compiled; pytest hit the cap after 16 min selftest / cold or contended cargo.

## Fixes (57-04)

- `test_ci_green_on_this_tree` stubs PATH `cargo` + `KUTHA_TENANT_BIN` from `fsm.yaml` required lists. Oracles unchanged. Live cargo = standalone `ci`.
- `rust_test_asserts` process-local fn-name index keyed by root + path mtime/size. Scratch copy cannot reuse another root. Pytest: mutated scratch stays HIGH.

## Timings

- Selftest before: `run_selftest=1004839` (~16.7 min) in pytest ci; honeycomb-ledger ~14 min.
- Selftest after: `uv run kutha-gov selftest` 20s (`tmp/p57-selftest.log`); ci `run_selftest=22177`.
- Standalone ci after warm: 66s, rc=0 (`tmp/p57-ci.log`). `run_checks=3028` (was 113s).

## Residuals

- `yaml_map_list` flow-style `[ADR-000]` still no `- item` mutations.
- `KUTHA_GOV_CARGO_TIMEOUT_SEC=180` still overrides `observe_rocks` 420 (warm rocks 4.7s here).

## Gates

- `uv run kutha-gov py`: PYTHON_TOOLING_GREEN
- Targeted pytest (cache, hermetic ci, freeze, observe): 46 passed in 27.6s
- Cheap pytest (`-k` not live ci/quantum): 157 passed, 2 deselected, 55s
- `uv run kutha-gov selftest`: rc=0, 20s
- `uv run kutha-gov ci`: rc=0, 66s, cargo=ok, all required names observed
- Full `pytest scripts/tests -q`: 159 passed in 106s (`tmp/p57-pytest-final.log`)

## Open

- Independent verifier kill-mutations (D-08). Controller owns STATE/README/ROADMAP.
