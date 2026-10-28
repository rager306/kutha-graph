---
phase: 241-ci-gate-truth
independent: true
status: passed
verifier_model: glm-5.3
commands:
  - cmd: 'uv run kutha-gov precommit'
    exit: 0
  - cmd: 'uv run python -c "import yaml,sys;d=yaml.safe_load(sys.stdin);print(len(d[''jobs''][''gate''][''steps'']))" < .github/workflows/ci.yml'
    exit: 0
  - cmd: 'grep -n "toolchain: 1.99.0" .github/workflows/ci.yml'
    exit: 0
  - cmd: 'grep -n "uv run pytest -q" .github/workflows/ci.yml'
    exit: 0
  - cmd: 'grep -n "kutha-gov ci --jobs 1" .github/workflows/ci.yml'
    exit: 0
  - cmd: 'git status --porcelain -- .github/workflows/ci.yml docs/process/repository.md .kutha/STATE.md'
    exit: 0
read_only_inputs:
  - .github/workflows/ci.yml
  - docs/process/repository.md
  - .kutha/STATE.md
---

# Phase 241 verification - CI gate truth

Independent verifier, fresh context. Only the three declared inputs were read; no PLAN or SUMMARY opened.

## Claims checked against `.github/workflows/ci.yml`

| Claim | Evidence | Verdict |
|-------|----------|---------|
| `dtolnay/rust-toolchain` step pins `with: toolchain: 1.99.0` | line 22, under the SHA-pinned step `683b588c...` | holds |
| The gate runs `uv run pytest -q` | step `harness pytest`, lines 32-33 | holds |
| The gate runs `uv run kutha-gov ci --jobs 1` | step `governor ci`, line 37 | holds |
| YAML parses | `yaml.safe_load` -> 8 steps under `jobs.gate` | holds |
| Governor dictionary plane still green | `uv run kutha-gov precommit` -> every check `OK`, exit 0 | holds |

`docs/process/repository.md` line 15 states the same three facts ("installs Rust 1.99.0, runs
`uv run pytest -q` and `uv run kutha-gov ci --jobs 1` after `cargo fetch --locked`") and matches the
workflow. That is a consistency check, not independent evidence.

`.kutha/STATE.md` leases M166-S01 with three REDs: CI 40/40 at action parse, pytest timeout honesty,
missing bench baseline. The working-tree change answers the CI RED only.

## Residuals - not claimed

1. **Remote green is unproven.** No push was made, so there is no `gh run list -w ci.yml` success to cite.
   Local file content is not a remote pass.
2. **The evidence is uncommitted.** `git status --porcelain` reports `M` for `ci.yml`,
   `docs/process/repository.md`, and `.kutha/STATE.md` at HEAD `ee9947d`. `git show HEAD:.github/workflows/ci.yml`
   still has neither the `with: toolchain` block nor the `harness pytest` step (last ci.yml commit
   `70cf9c1`, 2026-10-23). Until this is committed and pushed, the remote gate keeps the parse failure.
3. **Pin string only.** That `1.99.0` exists as a stable toolchain on the runner, and that it matches the
   workspace MSRV, was not checked.
4. **pytest was not run to completion here.** A `uv run pytest -q` attempt was started and stopped
   (exit 143) on the parent's instruction to avoid colliding with the benchmark; the parent owns that gate.
5. Benchmark pre/post numbers are outside this phase's scope.
