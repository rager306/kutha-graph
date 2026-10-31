# Intake draft — exempt the declared scratch root from `dogfood` glob_none

Status: **proposal, not applied**. No check, kind, ledger row, or test is changed here.
Owner approval + the intake steps below are required before code.

## Incident that motivates it (reproduced 2026-10-31)

A `uv run pytest` run was killed mid-flight. Pytest routes temp dirs into the repo scratch root
(`scripts/tests/conftest.py:8-10`: `tempfile.tempdir = <repo>/tmp`), so a killed run left
`tmp/kutha-selftest-tobudjha/tree/kutha-gov-selftest.egg-info/PKG-INFO`. The `dogfood` check
(`.kutha/dictionaries/checks.yaml:180`) runs a `glob_none` step with `glob: "**/*.egg-info/*"` from the repo
root and has no scratch exemption, so:

- the live tree reported a HIGH `build-artifact` finding;
- `scripts/tests/test_kutha_gov.py::HarnessTests::test_no_high_on_live_tree` failed;
- `scripts/tests/test_fsm.py::FsmTests::test_ci_quantum_scheduling_reaches_ok_with_skips` reached `fail`.

Deleting the leftovers made both tests green. The check itself is not wrong about source-tree
`*.egg-info`; it is wrong about the declared scratch root.

## Options

| Option | Change | Risk |
|---|---|---|
| A. Exclude the scratch root in the check | `glob_none` gains `exclude: ["tmp/**"]`; dogfood passes it | an `*.egg-info` under `tmp/` becomes invisible. Acceptable: `tmp/` is gitignored scratch, never repository content, and `tmp/` is already the declared home for one-shot output (`AGENTS.md` § Layout rules) |
| B. Fix the tests, not the check | product tests stop writing fixed relative `tmp/` dirs and clean up on panic | does not cover killed runs or `conftest.py`-routed tempfile; the harness suite would still pollute the root |
| C. Both (recommended) | A now + B as the product-side test fix (open-items index item 4) | two small changes, each independently verifiable |

## Intake rows (exact shape, for the eventual change)

1. `.kutha/dictionaries/invariants.yaml` — `I-dogfood.claim` becomes:
   `"Harness doc names H0–H5, is not an ADR, CutPort lists observe-built migrate-bin (debug/kutha-migrate), and the declared scratch root tmp/ is exempt from the egg-info scan"`
   (`source` unchanged; `disposition: check`; `check: dogfood` unchanged).
2. `.kutha/dictionaries/checks.yaml` — the `dogfood` `glob_none` step gains:
   ```yaml
   - kind: glob_none
     glob: "**/*.egg-info/*"
     exclude: ["tmp/**"]
     category: build-artifact
     message: "setuptools egg-info must not be in the tree: {path}"
   ```
3. `scripts/kutha_gov/kinds.py` — `_kind_glob_none` filters hits whose root-relative path matches any
   `exclude` pattern (pure path prefix/glob check, no filesystem walk change).
4. Selftest: the kind's derived mutation must still go HIGH. Add a test that creates
   `crates/probe.egg-info/PKG-INFO` → HIGH, and `tmp/probe.egg-info/PKG-INFO` → no finding.
5. `CHANGELOG.md` **Process** entry for the change; `docs/process/governor-intake.md` § Example intake rows
   gains the row above.

## Verification for the eventual change

```text
uv run kutha-gov precommit --check dogfood      # exit 0
uv run kutha-gov selftest --check dogfood       # exit 0, mutation still proves meaning
uv run pytest -q scripts/tests/test_kutha_gov.py::HarnessTests::test_no_high_on_live_tree
```

## Rollback

Revert the four edits together; the check returns to the full-tree scan. No data, log, or product impact.

## Why the check still means something with the exemption

The repository's own `tmp/` is gitignored and explicitly designated for one-shot scratch. A build
artifact there cannot ship. The source tree, `crates/`, `scripts/`, `docs/`, and every tracked path
remain fully scanned.
