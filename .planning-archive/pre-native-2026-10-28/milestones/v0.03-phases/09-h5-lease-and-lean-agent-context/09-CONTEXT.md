# Phase 9: H5 lease + lean agent context - Context

**Gathered:** 2026-09-30
**Status:** Ready for planning

<domain>
## Phase Boundary

Name the **H5** harness rung (semantic governor + lean agent context) consistently across harness docs, and slim `AGENTS.md` to durable operating rules. Volatile data (lease values, spike inventory, literature counts, research notes, repo tree, long subagent workflow) is **relocated** to the doc that owns it, never deleted. Stale lease citations are corrected.

**In scope:** LEASE-01, CTX-01, CTX-02, CTX-03, CTX-04.

**Out of this phase:** any `crates/` edit; new governor kinds or drift-guard checks (Phase 11); ADR amendments and `.kutha/ROADMAP.md` "Later milestones" reorder (Phase 10); leasing M012a/M002; promoting any ADR. Do not touch `.cursor/rules/*.mdc` (separate always-applied rule files) or the CE plugin cache.

</domain>

<decisions>
## Implementation Decisions

### H5 lease (user-directed; operator act)

- **D-H1:** The user asked for this work explicitly, so the harness lease is written by the operator in its own commit (`docs(process): lease H5 …`), not folded into a content commit. Edit `.kutha/STATE.md`: `**Phase:** H5`; Active Milestone stays `M011`, Active Slice stays `None`, all three lifecycles stay unchanged, freeze list unchanged. Rewrite "Next action" to say: H5 is a harness/docs-only rung; no `crates/` edits; product fixes wait for an explicit M012a lease. — **Reversibility:** reversible

- **D-H2:** `.kutha/ROADMAP.md` "Harness dogfood" gets `- [ ] **H5** — semantic governor + lean agent context (harness/docs only; checkbox flips at Phase 11 close)`. `docs/process/kutha-harness.md` adds an **H5** rung row/section (Now) and demotes H4 from "Now". `README.md` Status table names harness phase **H5** and stops citing a slice/milestone-progress phrase. The `h4-lease` check (needles `**Phase:** H4`, `- [x] **H4**`) is edited in place to `**Phase:** H5` plus `- [x] **H4**` and `- [ ] **H5**`-or-`- [x] **H5**` semantics via existing kinds; the `dogfood` check adds `**H5**`; `invariants.yaml` claim text for I-dogfood is updated to H0–H5. No new check ids; no new kinds. — **Reversibility:** reversible

### AGENTS.md diet

- **D-A1:** Target ≤ 110 lines (from 194). `AGENTS.md` keeps only rules an agent needs on every turn: language policy, two planes, product snapshot (4 lines: product, idea stack, formula, wedge), read-STATE-first pointer with one-sentence freeze rule, CE docs_root/handoff/artifact rules (condensed), commands, working conventions, Codex→Cursor `Task` mapping table, short CBM pointer, subagent rules pointer. — **Reversibility:** reversible

- **D-A2 (no lease values):** `AGENTS.md` must not contain `L_delivery=`, `L_map=`, `L_capability=` tokens, "M011 is active", "Harness H4 remains on", slice ids as current position, or card counts. It says: "Read `.kutha/STATE.md` first; do not start work it does not name." The freeze sentence points at the STATE "Freeze" section instead of enumerating dated milestone gates. — **Reversibility:** reversible

- **D-A3 (relocation map, "no fact lost"):**
  - P0 spike inventory (in-spike / not-in-spike lists) → new `docs/architecture/p0-spike-inventory.md` (no lease values; pointer from README "Where to read next")
  - Literature bound / matrix location / trap cluster / GTM notes → `.compound-engineering/artifacts/research/applicability/README.md` (append an "Agent notes" section; keep the "do not mint aggregator waves" rule as one line in AGENTS.md without the card count)
  - Leapfrog Triejoin note → `.compound-engineering/artifacts/research/notes/leapfrog-triejoin.md`
  - Repo layout tree → `README.md` Layout table (extend rows so nothing in the tree is lost); the two prohibitions ("no repo-root `ports/`/`adapters/`/`domain/`", "no Python in `kutha-runtime`") stay in AGENTS.md
  - Long "Codex subagents" prose → already owned by `docs/process/codex-subagents.md`; verify each removed bullet exists there, add any that is missing; keep the role→`Task` table in AGENTS.md (referenced by `.cursor/rules/code-graph-cbm.mdc` and `docs/process/codex-subagents.md`)
  - "Add a governor check by appending …" paragraph → already in `.kutha/META.md` / `docs/process/governor-intake.md`; verify, then replace by one pointer line
  - The executor writes a **diet ledger** table (old block → new home → verified) in the plan SUMMARY. — **Reversibility:** reversible

- **D-A4 (meaning preserved):** Working conventions 1–9, language policy, CE routing pointer, freeze guard, D1–D10 guard, planes table, CBM forbidden tools (`delete_project`, `manage_adr`, GitNexus secondary) keep their meaning. Condense wording only; do not drop a prohibition. — **Reversibility:** reversible

### Stale citations (CTX-04)

- **D-S1:** Replace every stale lease citation with a pointer to `.kutha/STATE.md`: `AGENTS.md` (removed by D-A2), `README.md` Status row ("M011 active (S03 …)"), and the header of `docs/architecture/semantic-contract-validation.md` ("`.kutha/STATE.md` names Phase H4 … `L_delivery=M011-S03-done`" → historical snapshot wording "as of 2026-09-13, M011 S03; live lease: `.kutha/STATE.md`"). Do not edit the body of that doc beyond the header. Also check `docs/process/kutha-harness.md` and `STRATEGY.md` for the same drift and fix what is found. — **Reversibility:** reversible

### Governor and process inheritance

- **D-G1:** Inherit v0.02 D-G1…D-G3: `uv run kutha-gov ci` (HIGH stops) + `explain trajectory` per wave; SUMMARY carries an ≤8-line Trajectory excerpt. Product cargo is exercised by `ci` (observe) but no crate changes are expected. — **Reversibility:** reversible
- **D-G2:** CHANGELOG entry for H5 lease + diet is authored in Phase 11 (SEM-08) via `kutha-changelog`; Phase 9 does not write CHANGELOG. `docs-coupling` only requires CHANGELOG for `crates/**` diffs — verify it stays green. — **Reversibility:** reversible
- **D-G3:** Freeze, Proposed honeycomb, three lifecycles, and Active Slice None are not touched. — **Reversibility:** reversible

### Claude's Discretion

Exact wording of the condensed `AGENTS.md`, new-file names beyond those listed, README table row order, and how the H5 rung is phrased in the harness doc — as long as D-A2 (no lease values), D-A3 (nothing lost), and D-H2 (existing kinds only) hold. Docs-only phase: no code-graph queries are required; do not claim graph verification.

</decisions>

<canonical_refs>
## Canonical References

- `.planning/ROADMAP.md` — Phase 9 goal and success criteria
- `.planning/REQUIREMENTS.md` — LEASE-01, CTX-01…CTX-04
- `.kutha/STATE.md`, `.kutha/ROADMAP.md`, `.kutha/META.md` — harness lease and rung list (lease is edited only per D-H1)
- `.kutha/dictionaries/checks.yaml` (`h4-lease`, `dogfood`, `state-readme`, `docs-entry`), `.kutha/dictionaries/invariants.yaml` (I-dogfood)
- `AGENTS.md`, `README.md`, `CLAUDE.md` (shim; must still contain `@AGENTS.md`)
- `docs/process/kutha-harness.md`, `docs/process/codex-subagents.md`, `docs/process/governor-intake.md`
- `docs/architecture/semantic-contract-validation.md` (header only)
- `.compound-engineering/artifacts/research/applicability/README.md`
- `.cursor/rules/code-graph-cbm.mdc` (read-only: references "Map Codex roles in `AGENTS.md`")

</canonical_refs>
