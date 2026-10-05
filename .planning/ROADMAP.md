# Roadmap: Kutha

## Overview

GSD overlay tracks the steel thread already in crates — not honeycomb waterfall. Harness delivery stays in `.kutha/STATE.md` / `.kutha/ROADMAP.md`.

v0.01–v0.10, v0.12 shipped. v0.11 (harness M018) is paused after Phase 45 at a decision gate. v0.13 (harness M019) is active, Phase 62 done.

## Id map

See `docs/process/gsd-auto-contract.md` § Id map. Do not copy the table onto this ROADMAP.

## Milestones

- ✅ **v0.01–v0.10, v0.12** — shipped — see `./milestones/`
- 🚧 **v0.11 Wedge unblock** — Phases 45–48 (harness M018) — Phase 45 shipped; paused at the Phase 46 decision gate
- ✅ **v0.12 Log durability protocol** — Phases 49–55 (harness M002) — shipped 2026-10-04 — [archive](./milestones/v0.12-ROADMAP.md)
- 🚧 **v0.13 Post-M002 coherence** — Phases 56–62 (harness M019) — active, Phase 62 done.

## Phases

- [x] **Phase 45: Doc truth round 2** — ADR Evidence, F-register prefix, English spine, hygiene (S01, docs only)
- [ ] **Phase 46: C01 and knowledge-cut decision** — ADR-013 verdict; Horn vs `as_known_on`; `as_of` vs eligibility (S02, decision gate)
- [ ] **Phase 47: Horn determinism and closure** — derived identity, Correct re-derives, cascade admit and order (S03)
- [ ] **Phase 48: Differential tests and oracle tightening** — I-F15/I-F16 and bench gate (S04); DIFF-01 moved to Phase 62
- [x] **Phase 56: Docs honesty round 3** — living-doc SoT pass, ROADMAP collapse, honeycomb comment honesty (M019 S01, docs/YAML only)
- [x] **Phase 57: Governor hardening** — `cargo_dep_constraint`, M002 oracles, artifact-budget, id-map-cite, selftest (M019 S02)
- [x] **Phase 58: Governor SMP** — process-pool checks/selftest; `--jobs 1` oracle (M019 S03)
- [x] **Phase 59: Test strategy and oracle matrix** — honeycomb cell 023 Proposed + `test-matrix.md` + governor row (M019 S04)
- [x] **Phase 60: Durability residuals A** — advisory lock, mixed group, missing durability oracles (M019 S05)
- [x] **Phase 61: Durability residuals B** — Rocks staleness, `bytes_per_fact`, dead-code fate, wiring decision (M019 S06)
- [x] **Phase 62: Semantic and logic test expansion** — DIFF-01 (moved from M018 S04), cascade, determinism, Correct characterization (M019 S07)

## Progress

| Milestone | Phases | Status |
|-----------|--------|--------|
| v0.01–v0.10, v0.12 | 1–44, 49–55 | shipped |
| v0.11 | 45–48 | Phase 45 shipped; paused at decision gate |
| v0.13 | 56–62 | Phase 62 shipped; awaiting milestone close |

**Next:** controller milestone close (v0.13 / M019). v0.11 Phase 46 still waits for the owner's C01 / Horn decision.
