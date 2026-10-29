# Kutha

## What This Is

Kutha is a research-stage, self-contained **hybrid AI-native temporal graph engine** (repo `kutha-graph`). The append-only event log is the product source of truth; the graph is a deterministic fold; CSR/HNSW/views are droppable leases. The primary wedge is **legal / normative temporal agents**: assert, invalidate, and query a norm **AS OF a date** with replayable lineage, without treating an LLM as legal authority.

Two orthogonal planes: **product** (`crates/kutha-common`, `crates/kutha-runtime`) owns temporal graph truth; **harness** (`scripts/kutha_gov`, `.kutha/`) owns trajectory honesty, freeze, and lifecycle non-collapse. Governor green ≠ ADR Accepted ≠ capability.

## Core Value

A developer can prove **legal point-in-time** on a named statute-shaped fixture (`as_of(2015) ≠ as_of(2021)`), keep governor CI **honest**, and advance **one Active Slice at a time** — not “ship the honeycomb.”

## Current Milestone: v1.63 as_of regression — closed (M169)

**Goal:** Locate the commit behind the 3.32× `as_of` gap and fix it or record an explicit tradeoff with a same-host bench.

**Lease:** `.kutha/STATE.md` is `M169-S01`; phases 247–248. No thaw.

**Last shipped:** v1.60 Operational honesty (M166, 2026-10-28); v1.59 compacted Horn cause (M165, 2026-10-27). Honeycomb stays Proposed.

## Business Context

- **Customer**: Builders of legal/normative temporal agents (research wedge); not a paying GTM motion yet
- **Revenue model**: None (research engine, release id `0.0.01`, not on crates.io)
- **Success metric**: FF5 stays green; `uv run kutha-gov ci` fail-closed and honest; one Active Slice — **not** “implement ADR-010–093”
- **Strategy notes**: `STRATEGY.md`; idea stack STCA (ADR-002) → vision (ADR-001) → D1–D10 (ADR-000) → honeycomb map (ADR-010–093, all Proposed)

## Requirements

### Validated

Capabilities already in the P0 spike (not honeycomb Accepted; not a legal corpus pack):

- ✓ Named `GraphFold::as_of` / `live_at` (no implicit “now”) and FF5 statute PIT — existing (`crates/kutha-runtime/tests/ff5_legal_pit.rs`)
- ✓ FF6 fail-closed unknown product relation — existing (`allow.rs`, `tests/ff6_allowlist.rs`)
- ✓ Log, fold, emit→idle quantum, receipt, snapshot + WAL-cousin, CSR lease, leapfrog intersect, `Materializer`, `fork_at` — existing. Also at HEAD: `manifest.json` + `KUTHASEG` (`store::persist_until` / `segment::persist_log`), `Persister` segmented append, `store::migrate` / `store::export_jsonl`, `store::checkpoint_sealed_prefix` (library API only, no operator CLI, not vacuum), optional unwired `rocks_index::SegmentIndex`.
- ✓ Typed `Op::{Assert,Retract,Correct,CorrectInterval,Behavior,Define,QuantumOutcome,JustificationCite,StoreBlob,TtAnchor,Checkpoint}` in `crates/kutha-common/src/event.rs` (not STCA-guide §5 JSON merge-patch) — existing
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

GSD overlay v0.07 (Phases 31–33 — M014 typed clocks):

- ✓ `ValidTime` / `TransactionTime` / `WallTime` newtypes; `Op::TtAnchor`; `as_known_on` greatest-anchor lookup — v0.07 (`event.rs`, `m014_as_known_on.rs`)
- ✓ Phase 34 residual: TT-only `known_at` / `is_known_at`; fail-closed duplicate or backwards `TtAnchor` walls — M015 S01 (`m015_known_on.rs`, `m015_anchor_admit.rs`)

GSD overlay v0.09 (Phases 36–40 — M016 executable rules):

- ✓ `RuleAst` canonical encoding hashed into `rule_version`; evaluator `live_horns` is lease-first via `GraphFold.horns_by_premise` (`eval.rs`; M017 S04; scan fallback `live_horns_scan`); thin `deny` policy body eval (`admission.rs`); execution replay `horn_behavior_reproducible` (`eval.rs`, called from `provenance.rs` `replay_check`); FIXG-01 C01 sunset inexpressible as Horn (`fixg01_c01_sunset_inexpressible_as_horn`) — v0.09

### Shipped

- [x] **v0.06 Honest baseline and one write path (M013 S01–S09):** shipped 2026-10-02 — archive `.planning/milestones/v0.06-*`, tag `v0.06`
- [x] **v0.07 Typed clocks and as-known-on (M014 S01–S03):** shipped 2026-10-02 — archive `.planning/milestones/v0.07-*`, tag `v0.07`
- [x] **v0.08 Known-on honesty (M015 S01–S02):** shipped 2026-10-02 — archive `.planning/milestones/v0.08-*`, tag `v0.08`
- [x] **v0.09 Test hardening + executable rules (M016 S01–S05):** shipped 2026-10-02 — archive `.planning/milestones/v0.09-*`, tag `v0.09`
- [x] **v0.10 Docs honesty and replay integrity (M017 S01–S04):** shipped 2026-10-03 — archive `.planning/milestones/v0.10-*`, tag `v0.10`
- [x] **v0.12 Log durability protocol (M002 S01–S07):** shipped 2026-10-04 — archive `.planning/milestones/v0.12-*`, tag `v0.12`
- [x] **v0.13 Post-M002 coherence (M019 S01–S07):** shipped 2026-10-05 — archive `.planning-archive/pre-native-2026-10-28/milestones/v0.13-*`, process tag `v0.13`
- [x] **v0.14–v0.17** — shipped 2026-10-07…08 (M020–M023); see `.planning/MILESTONES.md`
- [x] **v0.18 Fold identity and write semantics (M024 S01–S02):** shipped 2026-10-08 — audit `.planning/milestones/v0.18-MILESTONE-AUDIT.md`

### Standing

- [ ] Keep FF5/FF6/H2/H4/M010/M011 named fitness **green** (`L_capability=ff5-green` per `.kutha/STATE.md`)
- [ ] Keep governor CI honest; three lifecycles uncollapsed; thaw gates in `.kutha/STATE.md` § Thaw map
- [ ] Start only the slice `.kutha/STATE.md` names — do **not** assume a legal pack or honeycomb Accepted (2026-10-03: M002 thawed; remaining gates in `.kutha/STATE.md` § Thaw map)

### Out of Scope

- Honeycomb ADR-010–093 as a delivery backlog — map only; Status Proposed ≠ Accepted
- Legal / science product packs — FF5 is a fixture, not a corpus
- Thaw map items: `.kutha/STATE.md` § Thaw map only (do not copy the list here)
- TypeScript as graph core; Python inside `kutha-runtime`; repo-root `ports/` / `adapters/` / `domain/`
- STCA-guide §5 JSON merge-patch / `object.created` as the product write surface
- Mandatory Neo4j / FalkorDB / Graphiti / cloud LLM for assert → invalidate → AS OF
- Promoting ADR cells to Accepted because governor is green
- crates.io publish stays off. Release ids are `N.N.NN` (`0.0.01` now). GSD overlay tags `v0.0x` stay process-only and are not the product release id.

<decisions>
## Foundation locks (ADR-000 D1–D10)

Narrative locks in `docs/ADR/ADR-000-kutha-hybrid-architecture-research.md`. **Not** ADR Status Accepted. **Not** GSD `locked: true`. Do not reopen in product code without an explicit honeycomb cell + STATE lease.

### Two-plane model (D1)

- **D1:** Control-plane SoT is the event log + behaviors. Data plane is pluggable reversible materializations (not SoT). Agent plane is meta-prompt + dictionaries (not an unconstrained LLM).

### Formal reactivity (D2)

- **D2:** `G(L) = foldl(apply_event, G0, L)`. Direct graph mutate is forbidden. Runtime quantum: emit → log+project → trigger B/R_B → cascade to idle. Identifiers: UUID v7.

### Agent control (D3)

- **D3:** Hard FSM as the sole control model is rejected. Versioned meta-prompt + dictionaries; LLM proposes; dictionaries + log own audited fact truth. (Six dictionary kinds are honeycomb ADR-050 — **frozen** until SIX-01; see `.kutha/STATE.md` § Thaw map. P0 uses a single FF6 relation allowlist.)

### Bi-temporal facts (D4)

- **D4:** Facts carry valid/transaction time; invalidation is typed, not silent overwrite; AS OF is native.

### Materializations as plugins (D5)

- **D5:** CSR/HNSW/temporal views are reversible leases. RVF is packaging, not hot SoT. (HNSW implementation frozen until M005; see `.kutha/STATE.md` § Thaw map.)

### Core language (D6)

- **D6:** Graph core is Rust. TypeScript is not the materializer. (No Tokio crate in the workspace. Optional `rocksdb` is behind `rocks-index` as an unwired locator, never the event store; thaw map: `.kutha/STATE.md` § Thaw map.)

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
- **D-G3:** Improvements stay in harness YAML/docs and clarity of ADR/roadmap/trajectory; not a new milestone, not freeze thaw / legal pack. (2026-10-03: M002 thawed at the owner's request; remaining items follow `.kutha/STATE.md` § Thaw map.)

Claude-decided companions (same CONTEXT): **D-10** SUMMARY template (≤8-line trajectory excerpt); **D-11** WARN ledgered ≠ block (HIGH blocks); **D-12…D-14** PLANE/FREEZE/MAP path probes; **D-15** cargo smoke at tracer + pre-verify only.

## GSD Phase 3 — Lease gate (locked 2026-09-29)

See `.planning/phases/03-lease-gated-next-slice/03-CONTEXT.md`. While Active Slice is None, Phase 3 is **verification-only** (D-L1…D-L6): no new product slice, no legal-pack substitute, no assumed M002/honeycomb next; inherit Phase 2 governor cycle (D-L3). A mid-phase lease → hard stop and replan (D-L4).

### Claude's Discretion

Slice internals, test split vs `quantum.rs` size, and verification-plan shape — as long as freeze, planes, and typed Op hold. Do **not** discretionary-start Cypher, HNSW, legal pack, or ADR-050 (2026-10-03: M002 thawed; remaining gates in `.kutha/STATE.md` § Thaw map).
</decisions>

<map>
## Honeycomb (Proposed — not v1 backlog)

Cells ADR-010–014, 020–022, 030–031, 040–043, 050–052, 060–062, 070–071, 080–081, 090–093 name coordinates. **Accepted** only when that cell is in the running engine. P0 spike covers a subset of 010/011/013/014/040/041/060/061 surfaces in crates; that does not Accept the ADRs.

Harness compact index: `.kutha/dictionaries/honeycomb.yaml` (`uv run kutha-gov map`).
</map>

## Current State

Last shipped: **v1.59 — Compacted Horn cause** (M165, 2026-10-27), closing the checkpoint/replay lineage arc M154–M165. Prior GSD milestones v0.01–v1.58 are archived in `.planning-archive/pre-native-2026-10-28/` and git history. Product release id is `0.0.01` (`VERSION`; Cargo/pyproject `0.0.1`); `publish = false`. Honeycomb cells stay Proposed. Harness phase H5. `.kutha/STATE.md` is the only lease and thaw map.

## Next Milestone Goals

None leased. The next milestone is an owner decision; `.kutha/STATE.md` must name it first. Start it with the GSD `new-milestone` workflow, not by hand-editing phases. Remaining thaw gates: `.kutha/STATE.md` § Thaw map.

## Context

Brownfield research repo. Codebase map: `.planning/codebase/ARCHITECTURE.md`, `STACK.md`, `CONCERNS.md`.

**Harness lease** (process intent, not GSD memory, not product SoT): `.kutha/STATE.md`

- Active Milestone, Active Slice and `L_delivery`: cite `.kutha/STATE.md` only (they change per slice)
- Phase, `L_map`, `L_capability`, and Thaw map: cite `.kutha/STATE.md` only
- Next: the slice named in STATE; one lease at a time. Thaw map holds. Green governor ≠ ADR Accepted ≠ L_capability.

GSD files live under `.planning/`. **Never overwrite** `.kutha/STATE.md` with GSD STATE.

Chat with the human is Russian; this file and all other docs stay English (`AGENTS.md`).

Literature bound is closed (164 files in `cards/`: 163 original matrix rows + M002 intake `paper-kernel-io-dirty-throttle-cloud-oltp.md`, not a matrix row). Do not mint aggregator waves.

## Constraints

- **Stack:** Rust 2021 product crates; Python ≥3.13 harness via **uv** only (not system `python3`) — self-contained core, no mandatory graph DB
- **Thaw map:** the list lives in `.kutha/STATE.md` § Thaw map (2026-10-03; nothing thaws merely by an M002 lease); do not copy it here
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
| Per-item thaw gates (Rocks Phase 51; Cypher M003; HNSW M005) | `.kutha/STATE.md` § Thaw map | — Pending (per-item; 2026-10-03 M002 thawed) |
| Refactor and legal-fixture reconnaissance before durability (M013 before M002) | Post-M012 review R1: riskiest assumption last | ✓ Done (v0.06; `.planning/MILESTONES.md`) |
| Independent verification and planning-artifact budget as gates | Post-M012 review R4/R5; Phase 28 `28-VERIFICATION.md` | ✓ Good (v0.06 PROC) |
| CBM and GitNexus used together on M013 refactor phases | Measured tool limits; `rg` is arbiter | ✓ Done (v0.06; `.planning/MILESTONES.md`) |
| License / rename (ADR-092) | Repository is Apache-2.0 (2026-10-24). Name stays Kutha. Pack SPDX still open. Cell stays Proposed. | ✓ License file; rename not done |

## Evolution

This document evolves at phase transitions and milestone boundaries.

**After each phase transition** (GSD transition workflow):
1. Requirements invalidated? → Move to Out of Scope with reason
2. Requirements validated? → Move to Validated with phase reference
3. New requirements emerged? → Add to Active
4. Decisions to log? → Add to Key Decisions
5. "What This Is" still accurate? → Update if drifted

**After each milestone** (GSD complete-milestone workflow):
1. Full review of all sections
2. Core Value check — still the right priority?
3. Audit Out of Scope — reasons still valid?
4. Update Context with current state

---
*Last updated: 2026-10-28 after GSD native reset*
