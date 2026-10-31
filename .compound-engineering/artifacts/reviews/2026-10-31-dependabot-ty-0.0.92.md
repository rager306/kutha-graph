# Dependabot `ty` 0.0.72 → 0.0.92 (PR #20) — review

- Date: 2026-10-31
- Repo: `/root/kutha-graph`, review base `work/main-line` @ `6be40f2`; PR head `26f055c` (`dependabot/uv/ty-0.0.92`, base `main`)
- Mode: read-only. No merge, close, comment, commit, or tag. No repository file edited; the one code fix below is verified on a throwaway worktree only.
- Lease: `.kutha/STATE.md` has `Active Milestone: None`. No product change was attempted or needed.

## Context

PR #20 is a Dependabot `uv`-ecosystem update: `chore(deps-dev): bump ty from 0.0.72 to 0.0.92`, opened 2026-10-30, `mergeable=MERGEABLE`, `mergeStateStatus=CLEAN`.

Two facts shape the review:

1. **`ty` is not checked by CI.** The governor's `ci` quantum never invokes `ty`. The only `ty` invocation is `cmd_py` (`scripts/kutha_gov/__main__.py:301-335`), reachable only via the `py` subcommand (`__main__.py:438-439`). No governor check kind shells out to `ruff`/`ty`/`pyrefly` (`grep` over `scripts/kutha_gov` returns only `cmd_py`), no `.kutha/META.md` row covers them, and `.pre-commit-config.yaml` has `ruff` hooks but no `ty` hook. So a green CI `gate` job proves the lockfile installs and `pytest` passes — it does **not** prove `ty` 0.0.92 stays green.
2. **The PR targets a stale base.** `main` is 938 commits behind `work/main-line`. The PR's tree is `main`'s tree plus the lockfile bump; the harness code that actually runs the `ty` gate lives on `work/main-line`. Merging into `main` does not deliver the bump to the working line. I therefore measured the version diff on the current line (`6be40f2`) and checked separately that the PR merges cleanly into it.

The exact governor invocation is `uv run ty check --error-on-warning` with `cwd` = repo root. Paths and flags come from `pyproject.toml`, not from the command line:

```toml
[tool.ty.environment]
python-version = "3.13"

[tool.ty.src]
include = ["scripts"]
exclude = ["scripts/**/__pycache__/**"]

[tool.ty.rules]
unresolved-attribute = "warn"
unresolved-import = "warn"
```

Because `--error-on-warning` is passed, every diagnostic — including `warn`-level rules — fails the gate.

## PR contents

`gh pr view 20 --json title,body,files,headRefName,state` (exit 0) and `gh pr diff 20` (exit 0, 56 diff lines):

| Item | Value |
|---|---|
| Files changed | `uv.lock` only (20 insertions, 20 deletions) |
| `pyproject.toml` | untouched |
| CI workflows | untouched |
| `uv.lock` `[[package]] ty` | `0.0.72` → `0.0.92`, new sdist/wheel URLs, hashes, sizes, upload times |
| Lockfile header | `revision = 3` → `revision = 5` |
| Dependabot metadata | `dependency-type: direct:development`, `update-type: version-update:semver-patch` |
| Other locked packages | unchanged (`ruff 0.16.3`, `pyrefly 1.2.0`, `pytest 9.1.1`, `pyyaml 6.0.3`) |

No `pyproject.toml` change is required: the dev group declares `ty>=0.0.56` (`pyproject.toml:18`), which already admits `0.0.92`. Lockfile-vs-pyproject consistency was confirmed, not assumed: `uv lock --check` on the PR worktree returned exit 0 ("Resolved 11 packages in 3ms") and left the worktree clean.

## Current pin

| Where | Value |
|---|---|
| `pyproject.toml:18` | `ty>=0.0.56` (dev group) |
| `uv.lock` | `ty 0.0.72`, `ruff 0.16.3`, `pyrefly 1.2.0`; lockfile `version = 1`, `revision = 3` |
| `.python-version` | `3.13` (interpreter only — no tool pinning policy) |
| `[tool.uv]` | `package = true` only; no `required-version`, no `uv.toml` |
| CI uv | `.github/workflows/ci.yml:29` pins `astral-sh/setup-uv@07a047e6…` = tag **v10.3.2**, whose `version` input defaults to "" → "the version in `pyproject.toml` or `latest`". Neither exists here, so **CI installs whatever uv is newest at run time** (that commit's payload notes "update known checksums for 0.13.5") |
| Local uv | `uv 0.12.13`; Python `3.13.12` via uv |

`uv.lock` is the only thing CI consumes (`ci.yml:37` runs `uv sync --frozen`), so the pin is the lockfile, and the PR is a pure lockfile refresh.

## Diagnostics diff (commands + exit statuses)

### Method

Two independent reproductions, both on the same sources, so the result does not depend on how the tool is launched:

- **A — governor-equivalent, project environment.** Worktree `tmp/ty-review/wt-head` at `6be40f2` (current line). `uv sync --frozen` installs the pinned tools; `ty` 0.0.72 runs natively and 0.0.92 via `uv run --with ty==0.0.92` over the same environment (this is what makes `pyyaml`/`pytest` resolvable, as in the real gate).
- **B — `uvx`, no project environment.** `uvx ty@<version> check --error-on-warning` in the same worktree.

Both used the same `pyproject.toml` `[tool.ty.*]` config and the same flags as `cmd_py`. Version identity was confirmed with `ty --version` (`ty 0.0.72`, `ty 0.0.92`).

### Result

| Run | exit | diagnostics | new | silenced |
|---|---|---|---|---|
| A: `uv run --frozen ty check --error-on-warning` (0.0.72) | 1 | 4 | — | — |
| A: `uv run --frozen --with ty==0.0.92 ty check --error-on-warning` | 1 | 5 | 1 | 0 |
| B: `uvx ty@0.0.72 check --error-on-warning` | 1 | 4 | — | — |
| B: `uvx ty@0.0.92 check --error-on-warning` | 1 | 5 | 1 | 0 |

**Both methods agree: one new diagnostic, none silenced, and the four pre-existing ones are unchanged in rule, message, path, line, and column.**

The new diagnostic:

```
warning[deprecated]: The overload of `contextmanager` is deprecated
  --> scripts/kutha_gov/observe.py:94:2
   |
94 | @contextmanager
   |  ^^^^^^^^^^^^^^ Annotating the return type as `-> Iterator[Foo]` with `@contextmanager` is deprecated. Use `-> Generator[Foo]` instead.
```

The site (`scripts/kutha_gov/observe.py:94-95`):

```python
from collections.abc import Iterator, Mapping, Sequence   # line 10

@contextmanager
def _exclusive_cargo_lock(cwd: Path) -> Iterator[None]:   # line 95
```

`Iterator` is used nowhere else in the file, so the correction is local.

Pre-existing diagnostics, identical under both versions:

| Rule | Location |
|---|---|
| `error[call-non-callable]` | `scripts/tests/test_jobs.py:421:32` |
| `warning[unresolved-attribute]` | `scripts/tests/test_m113_selftest_mutation_static.py:68:15` |
| `error[invalid-argument-type]` | `scripts/tests/test_selftest.py:673:21` |
| `error[call-non-callable]` | `scripts/tests/test_selftest.py:845:32` |

### When the rule appeared

`uvx ty@<v> check --error-on-warning`, same worktree, only the version varying:

| ty | exit | diagnostics | `observe.py:94` present |
|---|---|---|---|
| 0.0.72 | 1 | 4 | no |
| 0.0.80 | 1 | 4 | no |
| 0.0.85 | 1 | 5 | yes |
| 0.0.90 | 1 | 5 | yes |
| 0.0.92 | 1 | 5 | yes |

The `deprecated` diagnostic for the `contextmanager` + `Iterator` overload landed between 0.0.81 and 0.0.85, so this PR is the first bump that exposes it to the repo.

### Verified fix

Applied to the scratch worktree only (`tmp/ty-review/wt-head/scripts/kutha_gov/observe.py`), two lines:

```diff
-from collections.abc import Iterator, Mapping, Sequence
+from collections.abc import Generator, Mapping, Sequence
-def _exclusive_cargo_lock(cwd: Path) -> Iterator[None]:
+def _exclusive_cargo_lock(cwd: Path) -> Generator[None]:
```

| Run after the fix | exit | diagnostics |
|---|---|---|
| `uv run --frozen --with ty==0.0.92 ty check --error-on-warning` | 1 | 4 |
| `uv run --frozen ty check --error-on-warning` (0.0.72) | 1 | 4 |

The two diagnostic sets are **identical** after the fix (compared element-wise: rule, message, path, line, column). The fix is therefore safe to land before or after the bump, and `Generator[None]` is valid on the repo's floor (`requires-python >= 3.13`, which gives `collections.abc.Generator` its PEP 696 defaults). The fixed file is clean under both other tools: `uv run --frozen ruff check scripts/kutha_gov/observe.py` exit 0 (`All checks passed!`), `uv run --frozen ruff format --check scripts/kutha_gov/observe.py` exit 0 (`1 file already formatted`).

### End-to-end gate state (context, not caused by this PR)

The `ty` step is only part of `kutha-gov py`. Running the whole subcommand with each lockfile:

| Lockfile | Command | exit | Per-tool |
|---|---|---|---|
| pinned (`ty 0.0.72`, lock `revision 3`) | `UV_FROZEN=1 uv run --frozen kutha-gov py` in `wt-head` | 1 | `FAIL ruff check`, `FAIL ruff format --check`, `FAIL ty check`, `FAIL pyrefly check` → `PYTHON_TOOLING_FAIL` |
| PR (`ty 0.0.92`, lock `revision 5`) | `uv sync --frozen` (exit 0), then `UV_FROZEN=1 uv run --frozen kutha-gov py` in `wt-pr` | 1 | same four `FAIL` → `PYTHON_TOOLING_FAIL` |

`UV_FROZEN=1` was set so the nested `uv run` calls inside `cmd_py` cannot rewrite any lockfile; `git status --porcelain` in both worktrees stayed empty.

The gate is **already red on `work/main-line` with the pinned lockfile**, so 0.0.92 does not flip a green gate to red: it takes `ty` from 4 to 5 diagnostics inside an already-failing subcommand. The other three tools' results are byte-identical between the two runs (same tool versions in both lockfiles):

- `ruff check`: 5 findings — `I001` ×2 (`scripts/tests/test_agent_surface.py:3:1`, `scripts/tests/test_multi_check.py:3:1`, import block not sorted), `RUF003` (`scripts/kutha_gov/dictionary.py:123:79`, EN DASH), `SIM117` (`scripts/tests/test_fsm.py:1066:9`), `RUF100` (`scripts/tests/test_m113_selftest_mutation_static.py:68:53`, unused `noqa: SLF001`).
- `ruff format --check`: 10 files would be reformatted — `scripts/kutha_gov/dictionary.py` (first at 138:32), `gitdiff.py`, `kinds.py`, `selftest.py`, `scripts/tests/test_jobs.py`, `test_kutha_gov.py`, `test_m054_id_structure.py`, `test_m114_cargo_observe_lock.py`, `test_multi_check.py`, `test_selftest.py`.
- `pyrefly check`: 2 errors (`_message` missing attribute; `_write_mini` argument type) — "2 errors (2 suppressed, 15 warnings not shown)".

This matters for the verdict: the `ty` gate currently cannot distinguish "green" from "known red", so nothing automated will notice a sixth diagnostic either.

## Verdict

**Merge after fixing `scripts/kutha_gov/observe.py:94-95`** — change `Iterator[None]` to `Generator[None]` and import `Generator` from `collections.abc` instead of `Iterator`.

Evidence for the verdict:

- The bump itself is sound: lockfile-only, `pyproject` specifier already admits `0.0.92`, `uv lock --check` exit 0, CI `gate` green on the PR head (`gh pr checks 20`: `gate` pass 5m34s, `CodeQL` pass, `analyze (rust)`/`analyze (python)` pass, `mergeStateStatus=CLEAN`), and `git merge-tree --write-tree work/main-line 26f055c…` exit 0 with no conflicting paths, so it applies to the active line as well as to `main`.
- The only behavioural delta is one new diagnostic. It is in governed harness source (`scripts/kutha_gov/observe.py`), not in tests, and `--error-on-warning` promotes it to a failure. Merging as-is would permanently add a fifth known-red item to the only `ty` gate the repo has, in the one place where the diagnostic signals real code modernization rather than a test fixture's typing.
- The fix is two lines, removes exactly the new diagnostic, is verified to leave the diagnostic set identical to today's under both 0.0.72 and 0.0.92, and is clean under `ruff check` and `ruff format --check`.

Order of landing does not matter: the fix is compatible with the currently pinned 0.0.72, so it can be a separate harness commit either side of the bump. Because the `py` gate is red on other findings too, this fix does not make `kutha-gov py` green — it keeps the `ty` count at its present value. I did not apply the fix: this review is read-only and `.kutha/STATE.md` names no lease.

## Residual risk

1. **`ty` is pre-1.0 and moves fast.** This PR skips 20 releases (0.0.73 → 0.0.92). One new rule class already appeared inside that window (`deprecated` for the `contextmanager`/`Iterator` overload, between 0.0.81 and 0.0.85). Expect further new diagnostics from future bumps, none of which CI will catch.
2. **No automated `ty` gate.** `kutha-gov py` is not in `ci`, not in `precommit`, and not a pre-commit hook. `pyproject.toml`/docs still describe it as the Python gate (`.planning/codebase/CONVENTIONS.md:52`, historical phase summaries record `PYTHON_TOOLING_GREEN`), so the gate is drifting silently. A new CI phase or hook needs a governor-intake row (`.kutha/META.md`, `docs/process/governor-intake.md`), not a Python change.
3. **Tool pinning policy is the lockfile only.** `.python-version` pins the interpreter (3.13), not tools; there is no `[tool.uv] required-version` and no `uv.toml`. Tools move only when Dependabot opens a PR, and nothing re-runs them between bumps.
4. **Lockfile `revision 3 → 5`.** The bump is more than the `ty` block: Dependabot's uv rewrote the lockfile header. Verified non-breaking: local `uv 0.12.13` reads it (`uv lock --check` exit 0, `uv sync --frozen` exit 0), and CI's uv is `latest`. But uv 0.12.13 writes `revision 3`, so any local `uv lock` will rewrite the header back, producing noise in an unrelated commit. No functional failure on the tools available here.
5. **CI's uv version is not reproducible.** `setup-uv@v10.3.2` with no `version`/`version-file` input resolves to `latest`. The lockfile revision a run can read therefore depends on the day it runs. Pinning uv (a `version:` input or `[tool.uv] required-version`) would remove that variable; that is a CI/config decision, outside this PR.
6. **Stale base.** The PR targets `main`, 938 commits behind `work/main-line`. Merging it into `main` leaves the working line on `ty 0.0.72` until the branches meet; the clean merge-tree result means the lock hunk can be re-applied there without conflict. Dependabot PRs #19 (`ruff 0.16.3 → 0.17.3`) and #11 (`pyrefly 1.2.0 → 1.4.0`) are open against the same stale base and would re-open the same red-gate questions.
7. **Unrelated but adjacent:** `kutha-gov py` is fully red today (ruff 5 findings, 10 files unformatted, ty 4, pyrefly 2). Whichever bump lands first should not be read as the cause; the fix list above is the pre-existing baseline, measured with the pinned lockfile at `6be40f2`.

## Scratch cleaned

**yes** — `git worktree remove --force` for `tmp/ty-review/wt-head` and `tmp/ty-review/wt-pr`, then `rm -rf tmp/ty-review`; `git worktree list` shows only the main checkout. `git status --porcelain` in `/root/kutha-graph` shows no tracked change; the only untracked entries are this report and another reviewer's `2026-10-31-rust-advisory-pack-stack.md` (not mine). `tmp/` still contains other agents' `ab-*.log` files — left untouched. No `/tmp`, `$TMPDIR`, or `.tmp` use; peak scratch 161 MB, well under the 300 MB cap.
