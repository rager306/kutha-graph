# Roadmap: Kutha

## Overview

GSD overlay tracks the steel thread already in crates — not honeycomb waterfall. Harness delivery stays in `.kutha/STATE.md` / `.kutha/ROADMAP.md`. Do not plan ADR-010–093 as sequential GSD phases.

v0.01–v0.06 are shipped. **v0.07** (harness M014) closes F13: typed clocks (`Vt`, `Tt`, wall), `tt_anchor` facts, and `as_known_on(date)`. Honeycomb stays Proposed. ADR-050 six dictionaries and M002 Rocks stay frozen until STATE names them. Run protocol: `docs/process/gsd-auto-contract.md`.

## Id map

See `docs/process/gsd-auto-contract.md` § Id map. Do not copy the table onto this ROADMAP.

## Milestones

- ✅ **v0.01 GSD foundation** — Phases 1–3 (shipped 2026-09-29) — [archive](./milestones/v0.01-ROADMAP.md)
- ✅ **v0.02 Semantic core close** — Phases 4–8 (shipped 2026-09-30) — [archive](./milestones/v0.02-ROADMAP.md)
- ✅ **v0.03 Lean context + semantic governor** — Phases 9–11 (shipped 2026-09-30) — [archive](./milestones/v0.03-ROADMAP.md)
- ✅ **v0.04 Single-log SoT + stable references** — Phases 12–16 (shipped 2026-10-01) — [archive](./milestones/v0.04-ROADMAP.md)
- ✅ **v0.05 Dictionaries as facts** — Phases 17–21 (shipped 2026-10-01) — [archive](./milestones/v0.05-ROADMAP.md)
- ✅ **v0.06 Honest baseline and one write path** — Phases 22–30 (shipped 2026-10-02) — [archive](./milestones/v0.06-ROADMAP.md)
- 🚧 **v0.07 Typed clocks and as-known-on** — Phases 31–33 (harness M014)

## Phases

**v0.07 phases** (one phase per harness slice S01–S03; the controller leases each slice before the phase runs):

- [x] **Phase 31: Clock newtypes** — `Vt`, `Tt`, and wall clock are distinct types; a compile-fail doctest rejects silent mixes
- [ ] **Phase 32: `tt_anchor` facts and `as_known_on`** — anchors are log facts; `as_known_on(date)` resolves through the greatest anchor at or before the date
- [ ] **Phase 33: Fixture migration** — FF5 and M011/M012 fixtures use explicit clocks with unchanged assertions

## Phase Details

### Phase 31: Clock newtypes

**Goal**: Valid time, transaction time, and wall time cannot be mixed silently.
**Depends on**: Nothing (M013 closed)
**Requirements**: CLK-01, CLK-02, CLK-03
**Success Criteria** (what must be TRUE):
  1. `Vt`, `Tt`, and a wall-clock type are separate newtypes (not bare `u64` aliases) on the public temporal API.
  2. A `compile_fail` doctest (or trybuild) proves a silent mix is rejected.
  3. Named oracles (FF5, FF6, M010–M013) stay green; mechanical renames only, recorded in SUMMARY.

**Plans**: 2/2 plans executed

Plans:
**Wave 1**
- [x] 31-01-PLAN.md — Clock newtypes, WallTime, compile_fail mix, as_of/live_at Into

**Wave 2**
- [x] 31-02-PLAN.md — CSR/justification cuts typed; TIME-01 rename-only; named oracles green

### Phase 32: tt_anchor facts and as_known_on

**Goal**: “As known on date D” is an anchor lookup, distinct from “true on D”.
**Depends on**: Phase 31
**Requirements**: ANK-01, ANK-02, ANK-03
**Success Criteria** (what must be TRUE):
  1. `tt_anchor` facts on the log map transaction time to wall time.
  2. `as_known_on(date)` resolves through the greatest anchor at or before the date.
  3. A named test separates “true on D” from “known on D”; FF5 stays green with explicit clocks.

**Plans**: 2/2 plans executed

Plans:
**Wave 1**
- [x] 32-01-PLAN.md — Op::TtAnchor log facts and as_known_on lookup

**Wave 2**
- [x] 32-02-PLAN.md — true-on-D vs known-on-D named test; FF5 green

### Phase 33: Fixture migration

**Goal**: Existing fixtures speak typed clocks without changing oracle assertions.
**Depends on**: Phase 32
**Requirements**: FIX-01, FIX-02
**Success Criteria** (what must be TRUE):
  1. FF5 and the M011/M012/M013 fixtures use explicit clock types.
  2. Oracle assertions are unchanged (D-07 style); renames only, tabled in SUMMARY.
  3. `uv run kutha-gov ci` is 0 HIGH (controller at close).

**Plans**: 1–2 plans max

Plans:
- [ ] 33-01-PLAN.md — (written by planner)

## Progress

| Milestone | Phases | Status |
|-----------|--------|--------|
| v0.01–v0.06 | 1–30 | shipped |
| v0.07 | 31–33 | active (M014) |

**Next:** Phase 32 plans written (32-01 Wave 1, 32-02 Wave 2). Execute 32-01; independent verifier writes `32-VERIFICATION.md` (D-05) after both plans. Do not start Phase 33.
