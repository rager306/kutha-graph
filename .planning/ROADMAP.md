# Roadmap: Kutha

## Overview

GSD overlay tracks the steel thread already in crates — not honeycomb waterfall. Harness delivery stays in `.kutha/STATE.md` / `.kutha/ROADMAP.md`.

v0.01–v0.13 shipped. **v0.14 Audit correction** (harness M020) is active.

## Id map

See `docs/process/gsd-auto-contract.md` § Id map. Do not copy the table onto this ROADMAP.

## Milestones

- ✅ **v0.01–v0.13** — shipped — see `./milestones/` where archived
- ✅ **v0.11 Wedge unblock** — Phases 45–48 (harness M018) — shipped 2026-10-06
- ✅ **v0.12 Log durability protocol** — Phases 49–55 (harness M002) — shipped 2026-10-04
- ✅ **v0.13 Post-M002 coherence** — Phases 56–62 (harness M019) — shipped 2026-10-05
- 🔄 **v0.14 Audit correction** — Phases 63–67 (harness M020) — active

## Phases

- [x] **Phase 63: Hygiene** — fmt, dead `line_at`, stale pointers, `ce-skills-ru` (S01, no product behaviour)
  - **Goal:** `cargo fmt --check` green; remove unused `rust_source.line_at`; fix stale Next-action / planning pointers; land or drop `ce-skills-ru` handoff override; honeycomb stays Proposed
  - **Plans:** 1/1 executed; independent `63-VERIFICATION.md` passed
  - [x] 63-01-PLAN.md — fmt, delete line_at, land ce-skills-ru
- [x] **Phase 64: Harness fail-closed** — silent-pass paths become findings (S02)
  - **Goal:** nameless dictionary row, missing cite, bad JSONL, `min_phase` without NN-, empty heading glob fail closed; RED selftest first; honeycomb stays Proposed
  - **Plans:** 1/1 executed; independent `64-VERIFICATION.md` passed
  - [x] 64-01-PLAN.md — fail-closed dict id, cite path, JSONL fold, min_phase dirname, empty heading glob
- [ ] **Phase 65: Persist atomic rewrite** — tmp+fsync+rename; failpoint; bench (S03)
  - **Goal:** crash mid-`persist_log` leaves openable store or clean failure; named oracle; bench ≤10%; honeycomb stays Proposed
  - **Plans:** 0/1
- [ ] **Phase 66: Horn and fold residuals** — CorrectInterval, live_triples, Retract gate, oracle stub (S04)
  - **Goal:** residual re-derive; co-live-safe triple lease; MetaOpRejected on reserved Retract; real fingerprint assert; pins; bench ≤10%; honeycomb stays Proposed
  - **Plans:** 0/1
- [ ] **Phase 67: Process contract** — lease/bench/verify rules; M020 close (S05)
  - **Goal:** lease = STATE+CHANGELOG; product close records bench; re-verify after VERIFICATION edits; M020 closed; honeycomb stays Proposed
  - **Plans:** 0/1

## Progress

| Milestone | Phases | Status |
|-----------|--------|--------|
| v0.01–v0.13 | 1–62 | shipped |
| v0.14 | 63–67 | active |

**Next:** Lease M020 S03 / Phase 65 (persist atomic rewrite).
