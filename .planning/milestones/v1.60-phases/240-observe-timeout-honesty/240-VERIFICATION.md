---
phase: 240-observe-timeout-honesty
independent: true
status: passed
verifier_model: glm-5.3
commands:
  - cmd: uv run pytest -q scripts/tests/test_fsm.py::ObserveBuildTests
    exit: 0
  - cmd: 'uv run python -c "<temp snippet: 127 retry/build, non-zero test still builds, partial-output retry>"'
    exit: 0
read_only_inputs:
  - scripts/kutha_gov/observe.py
  - scripts/tests/test_fsm.py
  - .kutha/STATE.md
---

# Phase 240 verification — observe timeout honesty (M166-S01)

Independent verifier. Read only `.kutha/STATE.md` (lease), `scripts/kutha_gov/observe.py`,
and `scripts/tests/test_fsm.py`. No PLAN or SUMMARY read. No repo file added or edited.

## Verdict

**PASS.** The requirement holds on the current working tree: a cargo timeout (`124`) or a
missing cargo binary (`127`) makes exactly one cargo call and skips the build, even when
`KUTHA_GOV_CARGO_OBSERVE_RETRIES` is set; a non-zero test status still builds; the
partial-output retry path is unchanged.

## Evidence in code

- `run_cargo_observation`: inside the attempt loop, `findings` is set, then
  `if code in {124, 127}: break` — the retry only happens through the `incomplete` branch
  below it, so a timeout or missing binary cannot consume a second attempt.
- The build gate is `if build is not None and code not in {124, 127}:`, so `124`/`127`
  spend no build wall clock. A non-zero test status (for example `1` with a `FAILED` line)
  is not in that set, so the build still runs, while `if code == 0: code = build_code`
  keeps the test failure as the recorded exit.
- Partial-output retry unchanged: `interpret_cargo_output` still collapses
  `cargo_exit != 0` + missing names + no `FAILED` into `observe-incomplete`, and the loop
  still retries on `incomplete` until `attempts` is spent, adding the LOW
  `observe-retried` breadcrumb only when a later attempt reaches `code == 0`.

## Attacks

| # | Attack | Result |
|---|--------|--------|
| 1 | `uv run pytest -q scripts/tests/test_fsm.py::ObserveBuildTests` | exit 0, `4 passed in 0.13s` — includes `test_observe_skips_build_after_timeout` and `test_observe_timeout_does_not_retry_with_retries_env` (1 call, code 124, `observe-error` under `KUTHA_GOV_CARGO_OBSERVE_RETRIES=2`) |
| 2 | Temp `uv run python -c` snippet: `FileNotFoundError` from `subprocess.run` with `KUTHA_GOV_CARGO_OBSERVE_RETRIES=2` | exit 0 — `code 127`, `1` call (`cargo test --offline`), no build call, findings `observe-error` + `observe-incomplete` |
| 3 | Temp snippet: `returncode=1` with `test foo ... FAILED`, `KUTHA_GOV_CARGO_OBSERVE_RETRIES=2` | exit 0 — `code 1`, `2` calls: `cargo test --offline` then `cargo build --offline`; findings `observe-fail` |
| 4 | Temp snippet: first attempt `101` with partial `ok` stream (`foo` ok, `bar` missing), second attempt `0` with both ok, retries env unset | exit 0 — `code 0`, `2` calls, `seen == ["foo", "bar"]`, exactly one LOW `observe-retried` |
| 5 | `git diff` of the two changed files | change is the single `if code in {124, 127}: break` guard plus the new retries-env test; no other behaviour touched |

`uv run pytest -q` (full harness suite) is **not** claimed here: the parent runs it
concurrently, and a second full run collides on the same workspace. The verifier's full-suite
run was stopped on the parent's instruction; that gate is parent-owned.

## Residual

- A `127` run reports both `observe-error` and `observe-incomplete` (fail-closed; cosmetic noise). Full pytest and `kutha-gov ci` stay parent-owned.
