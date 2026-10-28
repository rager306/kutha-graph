# Phase 45 Context: Doc truth round 2

**Milestone:** v0.11 / harness M018 S01
**Requirements:** DOC-06, DOC-07, REG-01, LANG-01, HYG-01
**Depends on:** none (v0.10 shipped)
**Source:** 2026-10-03 review at `d230ab2`: five read-only audits, CBM + GitNexus, `rg` arbiter
**Run rules:** `docs/process/gsd-auto-contract.md`

## Locked decisions

- **D-01 (no code):** nothing under `crates/` changes. Behaviour findings (Horn vs knowledge cut, derived ids, `Correct`, cascade admit, fail-open open) belong to Phases 46–48; here they are only stated truthfully.
- **D-02 (evidence over prose):** each corrected claim cites the symbol or test: `store.rs` `replace_via_tmp` / `replace_snapshot` / `replace_events_wal`; `load_events_d04` (jsonl SoT, covering `KUTHAWAL`); `provenance.rs` `check_tt_anchors` in `replay_check`; `eval.rs` `live_horns` via `horns_by_premise`; `search_prems(..., u64::MAX)` still TT-max; sidecars `write_terms` / `write_outcomes` / `write_justifications` still `File::create` (SNAP-01, M002).
- **D-03 (C01 stays open):** no option is chosen here; Phase 46 decides.
- **D-04 (dated clarifications):** ADR bodies get a dated Clarification or Evidence line. ADR-000 translation keeps meaning, structure and D1–D10 numbering; no decision text changes.
- **D-05 (one register):** audit ids F01–F14 of 2026-10-02 become `A-F01`–`A-F14` wherever they are cited outside CHANGELOG history; F1–F16 of `semantic-gap-review.md` keep their names. CHANGELOG history is not rewritten; one preface line explains the prefix.
- **D-06 (one freeze table):** `.kutha/STATE.md` Freeze is the table; `PROJECT.md` cites it and carries no copy.
- **D-07 (hygiene):** `.gitignore` gains scratch and vendored trees only (`crates/**/tmp/`, `.gsd/`, `.claude/skills/`, `.cursor/skills/gsd-*/`, plugin installs `.cursor/gsd-*`, `.cursor/agents/`, `.cursor/hooks*`); nothing tracked is untracked; curated CE handoffs stay trackable.
- **D-08:** Verifier ≠ executor. Only the controller edits `.kutha/STATE.md`, `.planning/STATE.md`, `README.md` lease row and `CHANGELOG.md`. Do not write `45-VERIFICATION.md`.

## Owned file groups (disjoint)

- **A (ADR):** `docs/ADR/ADR-010, 012, 013, 014, 041, 050, 060`, `docs/ADR/README.md`, `.kutha/dictionaries/honeycomb.yaml` (restage lines).
- **B (architecture and planning):** `docs/architecture/{semantic-gap-review,p0-spike-inventory,semantic-contract-validation}.md`, `.planning/PROJECT.md`, `.kutha/ROADMAP.md` (closed-milestone wording only), `docs/process/gsd-auto-contract.md` (Closes column), applicability README and README card count, `.gitignore`.
- **C (English spine):** `docs/ADR/ADR-000-*.md` (translation plus D6 note), `docs/architecture/stca-guide.md`.
- **Controller:** `CHANGELOG.md`, STATE files, `README.md` lease row.

## Verifier kill-tests

- `rg 'File::create' docs/ADR/ADR-012* docs/ADR/ADR-014* docs/ADR/ADR-060*` shows only sidecar sentences; none says `snapshot.json` is truncated in place.
- `rg 'scheduled M017 S02|target S02' docs/architecture/semantic-gap-review.md` returns nothing.
- `rg '\bF(0[1-9]|1[0-4])\b' .planning/REQUIREMENTS.md docs/process/gsd-auto-contract.md` returns no unprefixed audit id.
- `rg '[А-Яа-я]' docs/ADR/ADR-000*.md docs/architecture/stca-guide.md` returns nothing.
- ADR-010 D010-4 and ADR-000 D6 name `KUTHAWAL` covering lease and keep Rocks as M002.
- ADR-013 has a dated paragraph: `as_of` is support-live, derived facts are not retracted with premises; eligibility is `derivation_eligible_at`.
- `git diff --stat -- crates/` is empty; `uv run kutha-gov ci` is green.

## Non-goals

Choosing C01 or the Horn rule; any code; renaming CBM projects; deleting history; thawing any freeze item; SNAP-01.

## Harness

Active Milestone M018, Active Slice **S01**.
