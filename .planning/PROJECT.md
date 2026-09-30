# Kutha

## What This Is

Kutha is a research-stage, self-contained **hybrid AI-native temporal graph engine** (repo `kutha-graph`). The append-only event log is the product source of truth; the graph is a deterministic fold; CSR/HNSW/views are droppable leases. The primary wedge is **legal / normative temporal agents**: assert, invalidate, and query a norm **AS OF a date** with replayable lineage, without treating an LLM as legal authority.

Two orthogonal planes: **product** (`crates/kutha-common`, `crates/kutha-runtime`) owns temporal graph truth; **harness** (`scripts/kutha_gov`, `.kutha/`) owns trajectory honesty, freeze, and lifecycle non-collapse. Governor green ≠ ADR Accepted ≠ capability.

## Core Value

A developer can prove **legal point-in-time** on a named statute-shaped fixture (`as_of(2015) ≠ as_of(2021)`), keep governor CI **honest**, and advance **one Active Slice at a time** — not “ship the honeycomb.”

## Business Context

- **Customer**: Builders of legal/normative temporal agents (research wedge); not a paying GTM motion yet
- **Revenue model**: None (research engine, `0.0.0`, unpublished)
- **Success metric**: FF5 stays green; `uv run kutha-gov ci` fail-closed and honest; one Active Slice — **not** “implement ADR-010–093”
- **Strategy notes**: `STRATEGY.md`; idea stack STCA (ADR-002) → vision (ADR-001) → D1–D10 (ADR-000) → honeycomb map (ADR-010–093, all Proposed)

## Requirements

### Validated

Capabilities already in the P0 spike (not honeycomb Accepted; not a legal corpus pack):

- ✓ Named `GraphFold::as_of` / `live_at` (no implicit “now”) and FF5 statute PIT — existing (`crates/kutha-runtime/tests/ff5_legal_pit.rs`)
- ✓ FF6 fail-closed unknown product relation — existing (`allow.rs`, `tests/ff6_allowlist.rs`)
- ✓ Log, fold, emit→idle quantum, receipt, snapshot + WAL-cousin, CSR lease, leapfrog intersect, `Materializer`, `fork_at` — existing
- ✓ Typed `Op::{Assert,Retract,Correct,Behavior,Define}` (not STCA-guide §5 JSON merge-patch) — existing
- ✓ M010 semantic recovery without snapshot; live intern appends `Op::Define` — existing
- ✓ M011 S01–S03: portable `claim_id`, unknown-claim fail-closed, replay `caused_by`, thin P→Q `derivation_eligible_at` — existing
- ✓ Harness H0–H4 in; H2/H4 tenant AS OF dogfood (not ADR-090 ontology) — existing
- ✓ Rust 2021 product crates + Python 3.13 harness via **uv** — existing

GSD overlay v0.01 (Phases 1–3 verification-first):

- ✓ FIT-01…05 fitness suite falsifiable and green — v0.01
- ✓ GOV-01/02, PLANE-01…03, FREEZE-01, MAP-01 governor honesty + planes + freeze + map — v0.01
- ✓ GOV-03, NEXT-01, NEXT-02 lease-gate negative proof under Active Slice None — v0.01

GSD overlay v0.02 (Phases 4–8 — M011 semantic core close):

- ✓ CORR-01/02 partial correction with residual intervals; whole-version Correct unchanged — v0.02
- ✓ OUT-01/02 persisted quantum outcomes Zero/Partial/Full; crash needs explicit Resume — v0.02
- ✓ CSR-01/02 typed CSR lease preserves labels + multiplicity; untyped neighbor-set + FF5 green — v0.02
- ✓ PROV-01/02 provenance digest detects caused_by / rule_version without state-fingerprint change — v0.02
- ✓ FIX-01…03 end-to-end fixture: supports/conflict, justification admission, incremental≡reconstruct — v0.02
- ✓ GATE-01/02/03 named cargo + governor; lease-gated execute; freeze + honeycomb Proposed — v0.02

### Active

- [ ] Keep FF5/FF6/H2/H4/M010/M011 named fitness **green** (L_capability)
- [ ] Keep governor CI honest; three lifecycles uncollapsed; freeze until STATE names otherwise
- [ ] Start only the next delivery `.kutha/STATE.md` names — do **not** assume M002, legal pack, or honeycomb Accepted
- [ ] Optional formal `/gsd-audit-milestone` for v0.02 (closeout used override for stale verification digests)

### Out of Scope

- Honeycomb ADR-010–093 as a delivery backlog — map only; Status Proposed ≠ Accepted
- Legal / science product packs, ADR-090 ontology, ADR-093 — freeze; FF5 is a fixture, not a corpus
- RocksDB crate, Cypher/GPML parser, HNSW, ADR-050 six dictionaries, ADR-080/081, Consensus Query 103+ — until `.kutha/STATE.md` names them (today: freeze until explicit **M002**)
- TypeScript as graph core; Python inside `kutha-runtime`; repo-root `ports/` / `adapters/` / `domain/`
- STCA-guide §5 JSON merge-patch / `object.created` as the product write surface
- Mandatory Neo4j / FalkorDB / Graphiti / cloud LLM for assert → invalidate → AS OF
- Promoting ADR cells to Accepted because governor is green
- Product SemVer bump (`0.0.0`), crates.io publish, GitHub Releases (repo policy; GSD overlay tags `v0.0x` are process-only)

<decisions>
## Foundation locks (ADR-000 D1–D10)

Narrative locks in `docs/ADR/ADR-000-kutha-hybrid-architecture-research.md`. **Not** ADR Status Accepted. **Not** GSD `locked: true`. Do not reopen in product code without an explicit honeycomb cell + STATE lease.

### Two-plane model (D1)

- **D1:** Control-plane SoT is the event log + behaviors. Data plane is pluggable reversible materializations (not SoT). Agent plane is meta-prompt + dictionaries (not an unconstrained LLM).

### Formal reactivity (D2)

- **D2:** `G(L) = foldl(apply_event, G0, L)`. Direct graph mutate is forbidden. Runtime quantum: emit → log+project → trigger B/R_B → cascade to idle. Identifiers: UUID v7.

### Agent control (D3)

- **D3:** Hard FSM as the sole control model is rejected. Versioned meta-prompt + dictionaries; LLM proposes; dictionaries + log own audited fact truth. (Six dictionary kinds are honeycomb ADR-050 — **frozen** until STATE names them. P0 uses a single FF6 relation allowlist.)

### Bi-temporal facts (D4)

- **D4:** Facts carry valid/transaction time; invalidation is typed, not silent overwrite; AS OF is native.

### Materializations as plugins (D5)

- **D5:** CSR/HNSW/temporal views are reversible leases. RVF is packaging, not hot SoT. (HNSW implementation frozen until STATE names it.)

### Core language (D6)

- **D6:** Graph core is Rust. TypeScript is not the materializer. (P0 spike has no Tokio/Rocks in workspace deps; Rocks remains freeze until M002 lease.)

### Load profiles (D7)

- **D7:** One product, three named load profiles (OLTP / temporal KG / agents). Not three engines.

### Antipattern controllers (D8)

- **D8:** Snapshots + lean events + WAL-cousin against replay tax, fat events, and single-writer myths. P0 WAL is a persist cousin, not Rocks.

### Testing (D9)

- **D9:** Strict replay and fork-and-diff are first-class. P0 has fold fingerprint + `caused_by` replay checks; full execution replay remains later.

### Positioning (D10)

- **D10:** Event-sourced bi-temporal graph engine on Rust; agents first-class and dict-governed; performance from pluggable materializations — not agents bolted onto storage.

## Ingest resolutions (approved 2026-09-29)

- **STCA paradigm:** ADR-002 wins. `docs/architecture/stca-guide.md` is **constraints / algorithms / isolation laws only**. Do not mint a second STCA lock.
- **Write operators:** ADR-010 D010-3 typed Assert/Retract/Correct (product `kutha_common::Op`) wins over stca-guide Algorithm 1 / §5 JSON merge-patch.
- **Honeycomb:** ADR-010–093 stay Proposed/map. Do not schedule one GSD phase per cell.

## GSD Phase 2 — Governor cycle (locked 2026-09-29)

Standing control loop for every GSD execute cycle starting Phase 2 (see `.planning/phases/02-honest-harness-and-freeze/02-CONTEXT.md`):

- **D-G1:** After each execute-wave and before phase-verify — required `uv run kutha-gov ci` + brief `explain trajectory` in SUMMARY.
- **D-G2:** HIGH stops the wave; WARN is recorded in VERIFICATION/STATE (never masked).
- **D-G3:** Improvements stay in harness YAML/docs and clarity of ADR/roadmap/trajectory; not a new milestone, not assumed M002, not freeze thaw / legal pack.

Claude-decided companions (same CONTEXT): **D-10** SUMMARY template (≤8-line trajectory excerpt); **D-11** WARN ledgered ≠ block (HIGH blocks); **D-12…D-14** PLANE/FREEZE/MAP path probes; **D-15** cargo smoke at tracer + pre-verify only.

## GSD Phase 3 — Lease gate (locked 2026-09-29)

See `.planning/phases/03-lease-gated-next-slice/03-CONTEXT.md`. While Active Slice is None, Phase 3 is **verification-only** (D-L1…D-L6): no new product slice, no legal-pack substitute, no assumed M002/honeycomb next; inherit Phase 2 governor cycle (D-L3). A mid-phase lease → hard stop and replan (D-L4).

### Claude's Discretion

Slice internals, test split vs `quantum.rs` size, and verification-plan shape — as long as freeze, planes, and typed Op hold. Do **not** discretionary-start M002, Cypher, HNSW, legal pack, or ADR-050.
</decisions>

<map>
## Honeycomb (Proposed — not v1 backlog)

Cells ADR-010–014, 020–022, 030–031, 040–043, 050–052, 060–062, 070–071, 080–081, 090–093 name coordinates. **Accepted** only when that cell is in the running engine. P0 spike covers a subset of 010/011/013/014/040/041/060/061 surfaces in crates; that does not Accept the ADRs.

Harness compact index: `.kutha/dictionaries/honeycomb.yaml` (`uv run kutha-gov map`).
</map>

## Current State

Shipped **GSD v0.01 — GSD foundation** (2026-09-29) and **GSD v0.02 — Semantic core close** (2026-09-30). M011 tail S04–S08 is in crates and harness GATE needles; one e2e fixture separates preserved history, current evidence, and allowed action. Honeycomb cells stay Proposed. Product stays `0.0.0`.

## Next Milestone Goals

Planning-only until `/gsd-new-milestone` defines requirements. Delivery starts only when `.kutha/STATE.md` names the next Active Slice or milestone — freeze holds (no Rocks/Cypher/HNSW/ADR-050/legal pack by default).

## Context

Brownfield research repo. Codebase map: `.planning/codebase/ARCHITECTURE.md`, `STACK.md`, `CONCERNS.md`.

**Harness lease** (process intent, not GSD memory, not product SoT): `.kutha/STATE.md`

- Active Milestone: **M011**
- Active Slice: **None**
- Phase: **H4**
- Lifecycles: `L_map=honeycomb-proposed` · `L_delivery=M011-S08-done` · `L_capability=ff5-green`
- Next: Freeze until STATE names M002+ (or another explicit lease). Do not start a legal pack. Green governor ≠ ADR Accepted ≠ L_capability.

GSD files live under `.planning/`. **Never overwrite** `.kutha/STATE.md` with GSD STATE.

Chat with the human is Russian; this file and all other docs stay English (`AGENTS.md`).

Literature bound is closed (163 cards). Do not mint aggregator waves.

## Constraints

- **Stack:** Rust 2021 product crates; Python ≥3.13 harness via **uv** only (not system `python3`) — self-contained core, no mandatory graph DB
- **Freeze (until explicit M002 lease in `.kutha/STATE.md`):** RocksDB crate, Cypher/GPML parser, HNSW, ADR-050 six dictionaries, ADR-080/081, full ADR-090/093 packs, Consensus Query 103+
- **Planes:** Product vs harness dictionaries stay distinct; no Python in `kutha-runtime`; hexagon inside a slice (ADR-022)
- **STCA SPEC:** Isolation laws and Algorithms 2–3 from stca-guide apply as constraints; Algorithm 1 merge-patch does **not**
- **Delivery:** One Active Slice at a time; STATE is a lease; process SoT is `.kutha/events.jsonl` (gitignored)
- **Language:** Docs/ADRs/commit messages English; user chat Russian

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Event log = SoT; graph = fold | D1/D2, ADR-001 | ✓ Good (P0) |
| STCA paradigm = ADR-002; stca-guide = constraints | Ingest conflict (approved) | ✓ Good |
| Typed Op, not §5 merge-patch | ADR-010 > SPEC; harness forbids copy | ✓ Good |
| Legal PIT fixture before legal pack | Wedge + freeze | ✓ Good (FF5) |
| Honeycomb is a map | Three lifecycles; H4 | ✓ Good |
| GSD STATE ≠ harness STATE | Plane split | ✓ Good |
| Phase 3 verification-only under Active Slice None | D-L1…D-L6; no crate delivery without lease | ✓ Good (v0.01) |
| One GSD phase per M011 slice; GATE-02 lease discipline | v0.02 steel thread | ✓ Good (v0.02) |
| Compose S08 (justifications + conflict_report) without rewriting fold/CSR/provenance | D-F1…D-F7 | ✓ Good (v0.02) |
| GSD overlay annotated tags (`v0.01`, `v0.02`) | Milestone archival; not product SemVer / GitHub Release | ⚠️ Revisit (process tags only) |
| M002/Rocks/Cypher/HNSW frozen | `.kutha/STATE.md` | — Pending (lease) |
| License / rename (ADR-092) | Undecided | — Pending |

## Evolution

This document evolves at phase transitions and milestone boundaries.

**After each phase transition** (via `/gsd-transition`):
1. Requirements invalidated? → Move to Out of Scope with reason
2. Requirements validated? → Move to Validated with phase reference
3. New requirements emerged? → Add to Active
4. Decisions to log? → Add to Key Decisions
5. "What This Is" still accurate? → Update if drifted

**After each milestone** (via `/gsd-complete-milestone`):
1. Full review of all sections
2. Core Value check — still the right priority?
3. Audit Out of Scope — reasons still valid?
4. Update Context with current state

---
*Last updated: 2026-09-30 after v0.02 Semantic core close*
