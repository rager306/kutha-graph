# Phase 63: Hygiene — Context

**Milestone:** v0.14 / harness M020 S01
**Requirements:** HYG-01 fmt, HYG-02 dead `line_at`, HYG-03 stale pointers, HYG-04 `ce-skills-ru`
**Depends on:** none (first slice)
**Run rules:** `docs/process/gsd-auto-contract.md`
**Gathered:** 2026-10-07
**Status:** Ready for planning
**Mode:** Controller + owner (GSD auto; discuss skipped)

## Phase Boundary

No product behaviour change. Fix formatting debt from Phase 47, remove one dead harness helper, align stale pointers with M020, land the `ce-handoff` destination override already in the working tree. Do not touch `persist_log`, Horn, admission, or silent-pass harness kinds (Phases 64–66).

## Locked decisions

- **D-FMT:** run `cargo fmt --all` on the tree; `cargo fmt --all -- --check` must be green. No rustc behaviour change.
- **D-DEAD:** delete `scripts/kutha_gov/rust_source.line_at` (CBM + `rg`: zero callers). Do not prune `AdjacencyIter::at_end` / `Materializer::name` (pub API).
- **D-POINTERS:** `.kutha/STATE.md` Next action and `.kutha/ROADMAP.md` / `.planning/*` name M020; drop “Next lease: M018 S02” and “48-VERIFICATION pending” wording where still present.
- **D-CE-RU:** keep the `ce-handoff` → `.compound-engineering/artifacts/handoffs/` override in `.cursor/rules/ce-skills-ru.mdc` (matches AGENTS.md Artifacts).
- **D-SCOPE:** no ADR edits; honeycomb stays **Proposed**. Verifier ≠ executor. Controller owns `.kutha/STATE.md`.

## Deferred (not this phase)

Harness fail-closed → Phase 64. Persist atomic → 65. Horn/fold → 66. Process lease/bench wording polish beyond the already-amended id-map/lease line → 67.

## Verifier kill-tests

- `cargo fmt --all -- --check` exits 0.
- `rg -n 'def line_at' scripts/kutha_gov` returns no matches.
- `.kutha/ROADMAP.md` contains `## M020` and does not say “Next lease on the wedge: M018 S02”.
- `.planning/ROADMAP.md` Phase 48 line is not “VERIFICATION pending” (v0.11 archive wording ok if checked complete).
- `ce-skills-ru.mdc` mentions `.compound-engineering/artifacts/handoffs/`.

## Harness

Active Milestone M020, Active Slice **S01** (after lease commit).
