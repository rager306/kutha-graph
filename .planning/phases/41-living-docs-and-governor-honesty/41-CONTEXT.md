# Phase 41 Context: Living docs and governor honesty

**Milestone:** v0.10 / harness M017 S01
**Requirements:** DOC-02, DOC-03, DOC-04, DOC-05, GOV-02
**Depends on:** none (v0.09 shipped)
**Source:** 2026-10-02 audit at `c702d71`: three subagents, CBM + GitNexus, `rg` arbiter
**Run rules:** `docs/process/gsd-auto-contract.md`

## Locked decisions

- **D-01 (no code):** nothing under `crates/` changes. Rustdoc fixes belong to Phase 42.
- **D-02 (evidence over prose):** every corrected claim cites the code symbol or test that proves it (`eval.rs` `live_horns` scans `fold.facts()`; `as_of` = `live_at(MAX, vt)`; `as_known_on` = greatest `TtAnchor` then TT-only `known_at`; `Op` / `Event.ingested_at` stay `u64`; `ValidTime` / `TransactionTime` / `WallTime` newtypes in `event.rs`).
- **D-03 (C01 stays open):** ADR-013 records the options (pack stamp, fifth axis, stop) and that none is chosen. D013-1 is not rewritten; Status stays Proposed.
- **D-04 (dated clarifications):** ADR bodies get a dated `Clarification` or Evidence line; no silent rewrite of 000/001/002.
- **D-05 (one freeze table):** `.kutha/STATE.md` Freeze is the table; README, ROADMAP and PROJECT cite it.
- **D-06 (governor):** GOV-02 is a YAML change only (`cite-lease` globs) per `.kutha/META.md`; no new Python class. Selftest must prove a stale PROJECT token fails.
- **D-07 (banners, not deletion):** stale historical docs get a one-line status banner. No file is deleted; closed bodies stay in `.kutha/ROADMAP.md`.
- **D-08:** Verifier ≠ executor. Never edit `.kutha/STATE.md` (controller only). Do not write `41-VERIFICATION.md`.

## Owned file groups (disjoint)

- **A (ADR + README):** `docs/ADR/ADR-000, 013, 014, 041, 050, 060, 070`, `README.md`.
- **B (planning + governor + architecture):** `.planning/PROJECT.md`, `.planning/WINDOWS.md`, `.planning/RETROSPECTIVE.md`, `.planning/milestones/v0.06-MILESTONE-AUDIT.md`, `.kutha/dictionaries/{checks,invariants}.yaml`, `docs/architecture/{semantic-gap-review,semantic-contract-validation,legal-fixture-recon,p0-spike-inventory}.md`, `.kutha/ROADMAP.md` (M016 Success wording and freeze pointer only), `CHANGELOG.md`.

## Verifier kill-tests

- `rg 'Active Milestone.*M015|v0\.08.*overlay' .planning/PROJECT.md` returns nothing; mutating PROJECT to `Active Milestone: M015` makes `kutha-gov` fail HIGH.
- `rg 'until: "STATE names M01[45]"' .kutha/dictionaries/invariants.yaml` returns nothing.
- `rg 'CSR lease' .kutha/ROADMAP.md` does not appear in the M016 Success line; ADR-041 says Horn runs over live facts.
- README contains `as_of`, `live_at` and `as_known_on` as three named cuts.
- ADR-013 contains a dated paragraph naming options and "no option chosen".
- `git diff --stat -- crates/` is empty.

## Non-goals

Choosing a C01 option; typed `Op` clocks; moving closed ROADMAP bodies; hygiene of untracked `.cursor/skills/gsd-*`, `.gsd/`, CE handoffs; any freeze item.

## Harness

Active Milestone M017, Active Slice **S01**.
