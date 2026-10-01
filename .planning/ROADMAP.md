# Roadmap: Kutha

## Overview

GSD overlay tracks the steel thread already in crates — not honeycomb waterfall. Harness delivery stays in `.kutha/STATE.md` / `.kutha/ROADMAP.md`. Do not plan ADR-010–093 as sequential GSD phases.

v0.01–v0.05 are shipped. **v0.05** delivered M012 (dictionaries as facts): allowlist as log facts, rule registry (`rule_version` = hash), admission/policy meta-facts, thin action record, multi-hop derivation eligibility. Honeycomb stays Proposed. ADR-050 six dictionaries and M002 Rocks stay frozen until STATE names them.

## Milestones

- ✅ **v0.01 GSD foundation** — Phases 1–3 (shipped 2026-09-29) — [archive](./milestones/v0.01-ROADMAP.md)
- ✅ **v0.02 Semantic core close** — Phases 4–8 (shipped 2026-09-30) — [archive](./milestones/v0.02-ROADMAP.md)
- ✅ **v0.03 Lean context + semantic governor** — Phases 9–11 (shipped 2026-09-30) — [archive](./milestones/v0.03-ROADMAP.md)
- ✅ **v0.04 Single-log SoT + stable references** — Phases 12–16 (shipped 2026-10-01) — [archive](./milestones/v0.04-ROADMAP.md)
- ✅ **v0.05 Dictionaries as facts** — Phases 17–21 (shipped 2026-10-01) — [archive](./milestones/v0.05-ROADMAP.md)

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

<details>
<summary>✅ v0.05 Dictionaries as facts (Phases 17–21) — SHIPPED 2026-10-01</summary>

- [x] Phase 17: Allowlist as log facts (3/3 plans) — completed 2026-10-01
- [x] Phase 18: Rule registry (3/3 plans) — completed 2026-10-01
- [x] Phase 19: Admission and policy meta-facts (3/3 plans) — completed 2026-10-01
- [x] Phase 20: Thin action record (3/3 plans) — completed 2026-10-01
- [x] Phase 21: Multi-hop derivation (3/3 plans) — completed 2026-10-01

Full detail: [milestones/v0.05-ROADMAP.md](./milestones/v0.05-ROADMAP.md) · requirements: [milestones/v0.05-REQUIREMENTS.md](./milestones/v0.05-REQUIREMENTS.md) · phases: [milestones/v0.05-phases/](./milestones/v0.05-phases/)

</details>

## Next

No milestone is active in GSD. Next harness candidate needs an explicit lease in `.kutha/STATE.md` before `/gsd-new-milestone` (e.g. benchmark baseline after M012a indexes, or M002 log durability). Freeze holds for Rocks/Cypher/HNSW/ADR-050 six dictionaries/legal pack until STATE names them.
