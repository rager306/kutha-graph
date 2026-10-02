---
independent: true
verifier_model: composer-2.5
status: passed
score: 4/4
exit: pass
commands:
  - cmd: "! rg -n 'F13 stays open for M014' docs/architecture docs/ADR"; exit: 0
  - cmd: "! rg -n '^## M013:.*LEASED' .kutha/ROADMAP.md"; exit: 0
  - cmd: "rg -n 'Current Milestone: v0.08' .planning/PROJECT.md"; exit: 0
  - cmd: "rg -n 'true_on_d_differs_from_known_on_d' .kutha/dictionaries/fsm.yaml"; exit: 0
  - cmd: "rg -c 'min_hits: 1' .kutha/dictionaries/checks.yaml | awk -F: '{s+=$2} END{exit (s>=4)?0:1}'"; exit: 0
  - cmd: "uv run kutha-gov precommit"; exit: 0
read_only_inputs:
  - .planning/PROJECT.md
  - .planning/ROADMAP.md
  - .planning/REQUIREMENTS.md
  - docs/ADR/ADR-013-bitemporal-facts-invalidation.md
  - docs/architecture/semantic-gap-review.md
  - .kutha/dictionaries/fsm.yaml
  - .kutha/dictionaries/checks.yaml
  - .kutha/dictionaries/honeycomb.yaml
  - .kutha/STATE.md
---

# Phase 35 — independent verification

**ROADMAP SC:** (1) Living docs no longer claim M014 future / F13 fully open for M014 / M013 LEASED. (2) `true_on_d_differs_from_known_on_d` in `observe_cargo.required`. (3) Live artifact-budget globs `min_hits: 1`. (4) Governor precommit 0 HIGH.

| Kill-test | Status | Evidence |
|-----------|--------|----------|
| DOC-01 living notes | satisfied | No `F13 stays open for M014` under `docs/`; no `## M013:…LEASED` in `.kutha/ROADMAP.md`; `.planning/PROJECT.md` line 13 `Current Milestone: v0.08`; ADR-013 / semantic-gap addendum ship F13 clock+anchor, five-clock Q deferred M016 |
| GOV-01 ANK observe | satisfied | `fsm.yaml` `observe_cargo.required` includes `true_on_d_differs_from_known_on_d` (line 93); honeycomb ADR-013 evidence lists same |
| GOV-01 min_hits | satisfied | `checks.yaml` `artifact-budget` four `glob_max_lines` steps at `min_hits: 1` (PLAN/SUMMARY/VERIFICATION/CONTEXT) |
| Precommit | satisfied | `uv run kutha-gov precommit` exit 0; `artifact-budget` and full matrix `high=0` |

**Score:** 4/4 kill-tests. D-05: verifier ≠ executor; `.kutha/STATE.md` not modified during verify. M016 not started.
