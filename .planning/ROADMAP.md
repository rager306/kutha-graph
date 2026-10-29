# Roadmap: Kutha

## Overview

v1.60 Operational honesty closed on 2026-10-28. Milestone **v1.61 Legal PIT probe** (M167) **closed** on 2026-10-29: the wedge mechanism is proved on a small real eCFR fixture as test data, not a legal pack. No thaw, no next milestone leased.

## Id map

See `docs/process/gsd-auto-contract.md` § Id map. Do not restate the table here.

## Phases

- [x] **Phase 243: eCFR fixture** - fetch and commit 3 sections × 3 edition dates with source and public-domain status (completed 2026-10-29)
- [x] **Phase 244: PIT probe** - `as_of` / `as_known_on` / `norm_view` at edition boundaries, replay divergence 0 (completed 2026-10-29)

## Phase Details

### Phase 243: eCFR fixture
**Goal**: A small public-domain eCFR fixture with three dated editions per section is committed with a reproducible fetch script and licence note.
**Depends on**: Nothing
**Requirements**: LP-01
**Success Criteria**:
  1. 3 sections × 3 dates are present as text extracted from eCFR XML.
  2. `fixtures/ecfr/README.md` records URLs, dates, and public-domain status.
  3. The fetch script is committed and does not run in CI.
**Plans**: 1/1 plans complete

### Phase 244: PIT probe
**Goal**: Build cuts from the fixture and prove the three temporal surfaces at edition boundaries.
**Depends on**: Phase 243
**Requirements**: LP-02
**Success Criteria**:
  1. `as_of` differs at a boundary where the text changed; unchanged text stays equal.
  2. `as_known_on` distinguishes publication/known-on from force where the fixture has an anchor.
  3. `replay_check` and a checkpoint round-trip diverge 0.
  4. No `Op` or core schema change is needed for the fixture.
**Plans**: 1/1 plans complete

## Progress

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 243. eCFR fixture | 1/1 | Complete    | 2026-10-29 |
| 244. PIT probe | 1/1 | Complete    | 2026-10-29 |
