# Roadmap: Kutha

## Overview

GSD overlay tracks the steel thread already in crates — not honeycomb waterfall. Harness delivery stays in `.kutha/STATE.md` / `.kutha/ROADMAP.md`. Do not plan ADR-010–093 as sequential GSD phases.

v0.01–v0.04 are shipped. **v0.04** delivered M012a (single-log SoT + stable references): log-native outcomes/justifications, EventId retract/cites, idempotent ingest, verify-on-open + atomic persist + stable Define, declared time scale, fold-internal hot indexes. Product gaps F4/F5 wait for M012; Rocks durability waits for M002.

## Milestones

- ✅ **v0.01 GSD foundation** — Phases 1–3 (shipped 2026-09-29) — [archive](./milestones/v0.01-ROADMAP.md)
- ✅ **v0.02 Semantic core close** — Phases 4–8 (shipped 2026-09-30) — [archive](./milestones/v0.02-ROADMAP.md)
- ✅ **v0.03 Lean context + semantic governor** — Phases 9–11 (shipped 2026-09-30) — [archive](./milestones/v0.03-ROADMAP.md)
- ✅ **v0.04 Single-log SoT + stable references** — Phases 12–16 (shipped 2026-10-01) — [archive](./milestones/v0.04-ROADMAP.md)

## Phases

<details>
<summary>✅ v0.01 GSD foundation (Phases 1–3) — SHIPPED 2026-09-29</summary>

- [x] Phase 1: Legal PIT fitness (3/3 plans) — completed 2026-09-29
- [x] Phase 2: Honest harness and freeze (3/3 plans) — completed 2026-09-29
- [x] Phase 3: Lease-gated next slice (3/3 plans) — completed 2026-09-29

Full detail: [milestones/v0.01-ROADMAP.md](./milestones/v0.01-ROADMAP.md)

</details>

<details>
<summary>✅ v0.02 Semantic core close (Phases 4–8) — SHIPPED 2026-09-30</summary>

- [x] Phase 4–8 — see [milestones/v0.02-ROADMAP.md](./milestones/v0.02-ROADMAP.md)

</details>

<details>
<summary>✅ v0.03 Lean context + semantic governor (Phases 9–11) — SHIPPED 2026-09-30</summary>

- [x] Phase 9–11 — see [milestones/v0.03-ROADMAP.md](./milestones/v0.03-ROADMAP.md)

</details>

<details>
<summary>✅ v0.04 Single-log SoT + stable references (Phases 12–16) — SHIPPED 2026-10-01</summary>

- [x] Phase 12: Log-native SoT (3/3 plans) — completed 2026-10-01
- [x] Phase 13: Stable references (3/3 plans) — completed 2026-10-01
- [x] Phase 14: Idempotent ingest (3/3 plans) — completed 2026-10-01
- [x] Phase 15: Verify, persist, and time scale (3/3 plans) — completed 2026-10-01
- [x] Phase 16: Fold-internal hot indexes (3/3 plans) — completed 2026-10-01

Full detail: [milestones/v0.04-ROADMAP.md](./milestones/v0.04-ROADMAP.md) · requirements: [milestones/v0.04-REQUIREMENTS.md](./milestones/v0.04-REQUIREMENTS.md) · phases: [milestones/v0.04-phases/](./milestones/v0.04-phases/)

</details>

## Next

No milestone is active in GSD. Next harness candidate: **M012** (dictionaries as facts — F4/F5), which needs an explicit Active Slice lease in `.kutha/STATE.md` before `/gsd-new-milestone`. M002 Rocks stays frozen until STATE names it.
