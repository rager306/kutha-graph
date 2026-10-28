---
phase: 241-ci-gate-truth
plan: 01
subsystem: harness
tags: [ci, github-actions, pytest, toolchain]
requires:
  - phase: 240-observe-timeout-honesty
    provides: "full pytest green"
provides:
  - "pinned Rust 1.99.0 in CI"
  - "pytest and governor gates in CI"
affects: [242-benchmark-baseline]
actuals:
  tokens: 700
  tasks: 4
  commits: 0
tech-stack:
  added: []
  patterns: [ci-runs-both-gates]
key-files:
  created: []
  modified:
    - .github/workflows/ci.yml
    - docs/process/repository.md
    - CHANGELOG.md
key-decisions:
  - "toolchain input is required by the pinned dtolnay action; without it the workflow dies before any gate."
  - "pytest runs before the governor quantum so the cheap gate fails first."
requirements-completed: [OP-02]
coverage:
  - id: D1
    description: "CI installs Rust 1.99.0 and runs pytest plus the governor gate."
    requirement: OP-02
    verification:
      - kind: integration
        ref: ".github/workflows/ci.yml"
        status: pass
      - kind: unit
        ref: "docs/process/repository.md#CI"
        status: pass
    human_judgment: false
---

# Phase 241 summary — CI gate truth

- Added `with: toolchain: 1.99.0` to the pinned `dtolnay/rust-toolchain` step.
- Added `uv run pytest -q` before `uv run kutha-gov ci --jobs 1`.
- Updated `docs/process/repository.md` and CHANGELOG.
- Local: YAML parses, `precommit` 105 OK. Remote green remains a residual until a push is observed.
- No product code, no Op.
