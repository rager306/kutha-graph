# Roadmap: Kutha

## Overview

GSD overlay tracks the steel thread already in crates — not honeycomb waterfall. Harness delivery stays in `.kutha/STATE.md` / `.kutha/ROADMAP.md`.

v0.01–v0.07 are shipped. **v0.08** (harness M015) closes M014 audit residuals: honest `as_known_on` for closed VT intervals, fail-closed anchors, living docs + governor. Executable rules (F9/F16) move to **M016** / v0.09. Run protocol: `docs/process/gsd-auto-contract.md`.

## Id map

See `docs/process/gsd-auto-contract.md` § Id map. Do not copy the table onto this ROADMAP.

## Milestones

- ✅ **v0.01 GSD foundation** — Phases 1–3 (shipped 2026-09-29) — [archive](./milestones/v0.01-ROADMAP.md)
- ✅ **v0.02 Semantic core close** — Phases 4–8 (shipped 2026-09-30) — [archive](./milestones/v0.02-ROADMAP.md)
- ✅ **v0.03 Lean context + semantic governor** — Phases 9–11 (shipped 2026-09-30) — [archive](./milestones/v0.03-ROADMAP.md)
- ✅ **v0.04 Single-log SoT + stable references** — Phases 12–16 (shipped 2026-10-01) — [archive](./milestones/v0.04-ROADMAP.md)
- ✅ **v0.05 Dictionaries as facts** — Phases 17–21 (shipped 2026-10-01) — [archive](./milestones/v0.05-ROADMAP.md)
- ✅ **v0.06 Honest baseline and one write path** — Phases 22–30 (shipped 2026-10-02) — [archive](./milestones/v0.06-ROADMAP.md)
- ✅ **v0.07 Typed clocks and as-known-on** — Phases 31–33 (shipped 2026-10-02) — [archive](./milestones/v0.07-ROADMAP.md)
- 🚧 **v0.08 Known-on honesty and M014 residuals** — Phases 34–35 (harness M015)

## Phases

**v0.08 phases** (one phase per harness slice S01–S02; controller leases each slice before the phase runs):

- [x] **Phase 34: Known-on cut and anchor admission** — closed VT intervals survive `as_known_on`; non-monotone/duplicate anchors fail closed
- [ ] **Phase 35: Living docs and governor honesty** — docs match M014 ship + residual; ANK oracle required; budget min_hits restored when phases live

## Phase Details

### Phase 34: Known-on cut and anchor admission

**Goal**: `as_known_on` matches ADR-013 “known on D” for statutes with finite `valid_to`; anchors cannot lie about the past.
**Depends on**: M014 closed
**Requirements**: KNOWN-01, KNOWN-02, ANK-ADM-01, ANK-ADM-02
**Success Criteria**:
  1. RED oracle with FF5-shaped closed interval + anchors fails on HEAD before the fix and passes after.
  2. `as_of` / FF5 assertions unchanged (rename-only if any).
  3. Duplicate or backwards `TtAnchor` walls are rejected at admit with a named test.
  4. Persist/open rebuild agrees on anchors and known-on.

**Plans**: 2/2 plans executed max

Plans:
**Wave 1**
- [x] 34-01-PLAN.md — TT-only as_known_on (KNOWN-01/02); RED closed-VT oracle first

**Wave 2**
- [x] 34-02-PLAN.md — fail-closed TtAnchor admit (ANK-ADM-01/02)

### Phase 35: Living docs and governor honesty

**Goal**: Living documentation and governor match post-M014 reality and protect ANK.
**Depends on**: Phase 34
**Requirements**: DOC-01, GOV-01
**Success Criteria**:
  1. Living ROADMAP/PROJECT/semantic-gap/ADR-013 Evidence/p0-spike/legal-fixture-recon no longer claim M014 as future or F13 fully open.
  2. `true_on_d_differs_from_known_on_d` is observed by `kutha-gov ci`.
  3. With live phases present, artifact-budget empty-glob is HIGH again (`min_hits: 1`).
  4. `uv run kutha-gov ci` is 0 HIGH (controller at close).

**Plans**: 1–2 plans max

Plans:
- [ ] 35-01-PLAN.md — (written by planner)

## Progress

| Milestone | Phases | Status |
|-----------|--------|--------|
| v0.01–v0.07 | 1–33 | shipped |
| v0.08 | 34–35 | active (M015) |

**Next:** Phase 34 executor complete. Independent verifier writes `34-VERIFICATION.md` (D-05). Do not start Phase 35.
