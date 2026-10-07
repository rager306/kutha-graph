---
phase: 63-hygiene
verified: 2026-10-07T06:48:00Z
independent: true
verifier_model: composer-2.5-fast
status: passed
read_only_inputs:
  - .planning/phases/63-hygiene/63-CONTEXT.md
  - README.md
  - .kutha/ROADMAP.md
  - .kutha/STATE.md
  - .cursor/rules/ce-skills-ru.mdc
  - scripts/kutha_gov/rust_source.py
  - crates/kutha-runtime/src/eval.rs
commands:
  - { cmd: "cargo fmt --all -- --check", exit: 0 }
  - { cmd: "bash -lc \"rg -n 'def line_at' scripts/kutha_gov; test $? -eq 1\"", exit: 1 }
  - { cmd: "bash -lc \"rg -F '## M020' .kutha/ROADMAP.md\"", exit: 0 }
  - { cmd: "bash -lc \"rg -F 'Next lease on the wedge: M018 S02' .kutha/ROADMAP.md; test $? -eq 1\"", exit: 1 }
  - { cmd: "bash -lc \"rg -F '.compound-engineering/artifacts/handoffs/' .cursor/rules/ce-skills-ru.mdc\"", exit: 0 }
  - { cmd: "bash -lc \"rg -F 'M020' README.md\"", exit: 0 }
  - { cmd: "uv run kutha-gov precommit --check state-readme", exit: 0 }
cbm: list_projects kutha-graph; search_graph line_at (index stale); check_index_coverage cited paths metadata_changed/not_tracked
---

# Phase 63 Hygiene — Independent Verification

**Goal (CONTEXT):** fmt green, remove dead `line_at`, M020 pointers, `ce-handoff` path — no product behaviour.

| Check | Evidence |
|-------|----------|
| HYG-01 fmt | `cargo fmt --all -- --check` exit 0 |
| HYG-02 dead code | No `def line_at` in `rust_source.py` (file ends at `first_assert_macro_span`; CBM graph entry stale) |
| HYG-03 pointers | `.kutha/ROADMAP.md` line 189 `## M020`; no `Next lease on the wedge: M018 S02`; STATE Active Milestone **M020** S01; README delivery row cites M020; `.planning/ROADMAP.md` has no `VERIFICATION pending` |
| HYG-04 ce-skills-ru | Handoff override to `.compound-engineering/artifacts/handoffs/` present |
| Harness | `state-readme` precommit OK |

**Note:** Commands 2 and 4 recorded exit 1 because host shell expands `$?` before `bash -lc`; direct `rg` confirms zero matches for both patterns (intended kill-test outcome).

**Verdict:** PASS — phase boundary satisfied; no PLAN/SUMMARY read.
