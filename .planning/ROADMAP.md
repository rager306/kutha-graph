# Roadmap: Kutha

## Overview

GSD overlay tracks the steel thread already in crates — not honeycomb waterfall. Harness delivery stays in `.kutha/STATE.md` / `.kutha/ROADMAP.md`.

v0.01–v0.10, v0.12, v0.13 shipped. v0.11 (harness M018) Phase 45 shipped; Phase 46 leased 2026-10-06.

## Id map

See `docs/process/gsd-auto-contract.md` § Id map. Do not copy the table onto this ROADMAP.

## Milestones

- ✅ **v0.01–v0.10, v0.12, v0.13** — shipped — see `./milestones/`
- 🚧 **v0.11 Wedge unblock** — Phases 45–48 (harness M018) — Phase 45 shipped; Phase 46 leased 2026-10-06
- ✅ **v0.12 Log durability protocol** — Phases 49–55 (harness M002) — shipped 2026-10-04 — [archive](./milestones/v0.12-ROADMAP.md)
- ✅ **v0.13 Post-M002 coherence** — Phases 56–62 (harness M019) — shipped 2026-10-05 — [archive](./milestones/v0.13-ROADMAP.md)

## Phases

- [x] **Phase 45: Doc truth round 2** — ADR Evidence, F-register prefix, English spine, hygiene (S01, docs only)
- [ ] **Phase 46: C01 and knowledge-cut decision** — ADR-013 verdict; Horn vs `as_known_on`; `as_of` vs eligibility (S02, decision gate)
  - **Goal:** ADR-013 records C01 (a) pack stamp, Horn ingest-at-head, and `as_of` support-live; named pin + blocked matrix invariants; honeycomb stays Proposed
  - **Plans:** 1/1 executed (await independent verifier)
  - [x] 46-01-PLAN.md — named as_of support-live pin; ADR-013/matrix/honeycomb verdicts; CHANGELOG planes
- [ ] **Phase 47: Horn determinism and closure** — derived identity, Correct re-derives, cascade admit and order (S03)
- [ ] **Phase 48: Differential tests and oracle tightening** — I-F15/I-F16 and bench gate (S04); DIFF-01 moved to Phase 62 (v0.13)

## Progress

| Milestone | Phases | Status |
|-----------|--------|--------|
| v0.01–v0.10, v0.12, v0.13 | 1–44, 49–62 | shipped |
| v0.11 | 45–48 | Phase 45 shipped; Phase 46 in progress |

**Next:** `/gsd-verify-work` for Phase 46 (C01 pack stamp, Horn ingest-at-head, `as_of` support-live).
