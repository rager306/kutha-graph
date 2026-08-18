---
name: Kutha
last_updated: 2026-08-17
---

# Kutha Strategy

## Target problem

Builders need a graph engine where four things hold **at once**: (1) agents and GenAI enrichment live *inside* the runtime, not bolted onto storage; (2) time and audit are first-class (bi-temporal facts, lineage, point-in-time, fork); (3) multi-hop / vector / analytics stay in the Samyama–FalkorDB performance class; (4) capability grows via reversible packs, not core rewrites.

No incumbent closes that set: state-first graph DBs are fast but weak on agents/history/fork; pure event-sourced graphs audit well but lose multi-hop latency; memory frameworks (Graphiti/Zep-class) own temporal agent memory but are not a DB and depend on an external graph; document stores with in-DB GenAI lack a native graph.

**White space (session 2026-08-15 + research):** one Rust engine that combines in-DB agents, event-log projections, bi-temporal semantics, reversible high-perf views, and portable cognitive packaging — quadrant *high temporal × high hybrid × in-DB agents*. Literature gaps that fit the same niche include write-time contradiction contracts, provenance coupled to time, and LLM-outside-trust-boundary temporal operators (not bigger RAG).

## Our approach

Build on **STCA** (Space × Time × Composition): vertical domain slices talk only through **Ports**; the sole SoT is an append-only event log; the graph is a deterministic fold plus Behaviors / Relation Behaviors; packs compose as Cui generalized items via **max-convolution** of budgets — not a hard-coded workflow engine.

On that core, Kutha ships reversible hot materializations (CSR/HNSW/temporal views), native bi-temporal fact semantics, and dict-first agents (LLM optional and outside the trust boundary for audited fact truth). STCA must still be proven as a **graph database** product: join/query optimality (e.g. LFTJ-class bounds on hot paths), MVP speed to a falsifiable spike, and a replay/fork-first test strategy — not manifesto-only architecture.

## Who it's for

**Primary wedge:** Builders of legal / normative temporal agents. Job: assert, invalidate, and query facts AS OF a date with replayable lineage — without treating the LLM as legal authority.

**Next wedges (not exclusive):** scientific / scholarly archive (revision-scoped evidence); regulated finance / compliance; horizontal agent runtime (shared temporal memory); enterprise knowledge + security plane. Dictionaries and packs change per vertical; the STCA runtime does not.

**Buyer constraint for later GTM:** ABAC/temporal policies, agent sandbox, multi-tenancy, immutable audit, CMK/air-gap — required before “enterprise ship,” sketched after core physics.

## Key metrics

**Status:** provisional engineering falsifiers from ADR-000 R9 / exit criteria — **not PRD-locked product KPIs**. Refine and replace in a PRD before treating them as go-to-market success.

Candidate signals (must be able to regress):

- **Replay integrity** — strict replay divergence on golden logs → 0 on held fixtures (verify suite).
- **Cascade / quantum bound** — max depth and wall time per emit→idle under fixed budgets (P0 harness).
- **Legal PIT probe** — norm/edition @ date correctness (+ latency once fixtures exist) for the legal pack.
- **Hot-path proximity** — multi-hop (later hybrid) vs Samyama/FalkorDB-class baselines; goal “close on projections,” not always beat.
- **Fork cost** — time/space for fork-and-diff on a fixed counterfactual workload.

Fit scores in ADR-000 (regulated ~9/10, hybrid ~8–9/10, Samyama-replacement ~5–6/10) are research hypotheses to re-score against probes — not marketing numbers.

## Tracks

Plain-language investment areas (map to STCA axes + research backlog):

### 1. Make time trustworthy (Time / P0–P1)

Ship the event log, lean events, snapshots, bi-temporal assert/invalidate, and one runtime quantum (emit → cascade → idle) with a receipt. Without this, Kutha is not an auditable engine.

### 2. Make the graph fast enough (Composition + Data / P1–P2)

Treat speed as **removable pictures** of the same log (CSR/sparse, later HNSW/hybrid query) — reversible packs, not a second database. Prove optimality/latency on graph workloads; keep MVP path: one hot projection before a zoo of indexes.

**RuVector heritage (adapt as packs, not the monorepo):** borrow only truthfulness-gated surfaces — HNSW index crates, `ruvector-hybrid` BM25+dense fusion, RVF CowEngine + WitnessChain for sealed portable units, optional GNN *primitives* for rerank. Do **not** vendor Cypher/GraphRAG facades that return empty or fabricated success (prior stub audit 2026-07). Ports own the contract; RuVector is an adapter. Detail: `.compound-engineering/artifacts/research/ruvector-plugin-adaptation.md`.

### 3. Make modules and verticals plug in (Space / packs)

Cargo vertical slices + Ports; Legal and Scientific as packs on the same physics; finance and others later. Same dictionaries pattern across verticals. RuVector/RVF appear here only as **optional** intelligence and packaging packs (ADR-091/052), never as hot SoT.

### 4. Make agents governable and checkable (Verify + Agent / P1–P2)

Meta-prompt + dictionaries; Strict Replay and Fork-and-Diff as the default test strategy; security sandbox before enterprise agents; Regimes gated loop optional later.

### Research program (Honeycomb / P0–P3)

| Phase | In plain words | Typical cells |
|-------|----------------|---------------|
| **P0** | Prove the heartbeat: log + fold + behavior cascade + replay | ADR-010…012 |
| **P1** | Facts over time + first fast picture + agent dictionaries | ADR-013, 040, 050 |
| **P2** | Vectors/hybrid query, optional GenAI pack, security sketch | ADR-042/043, 052, 080 |
| **P3** | Vertical proof (legal norm@date), portable packs, benchmarks | ADR-090 (+093), 091, BENCHMARKS |
| **P4** | Distribution / learned indexes — only after single-node hybrid holds | Raft, GNN |

Open honeycomb bands (Time 010–, Space 020–, Composition 030–, Data 040–, Agent 050–, Verify 060–, Query 070–, Security 080–, Vertical 090–) deepen one facet without rewriting ADR-000 D1–D10.

## Not working on

Each item is a temptation with a concrete failure mode:

- **Mandatory Neo4j / FalkorDB / Graphiti / cloud LLM for core assert → invalidate → AS OF** — becomes a memory/orchestration shell; we lose ownership of temporal truth and CI that proves the core self-contained.
- **“Another graph DB” or “memory on Neo4j” positioning** — collapses into crowded GraphRAG/memory market; abandons the hybrid quadrant.
- **Pure state-first Samyama product with agents bolted on** — fast today, but history/fork/agent governance stay second-class; fit as Samyama replacement is only ~5–6/10 by design.
- **Pure ActiveGraph without hot projections** — excellent lineage, dead on Profile B/C multi-hop latency and storage bloat.
- **Hard FSM as the sole agent control model** — every vertical forks the runtime; rejected for versioned meta-prompt + temporal dictionaries (FSM may be *derived* for audit, not hard-coded).
- **Workflow-engine orchestration instead of Cui pack composition** — fights reversible packs and budgeted competition between materializers/agents.
- **TypeScript as the graph core** — fine for agent I/O/SDK; GC/event-loop choke Profile B hot graphs.
- **RVF (or portable packs) as primary hot storage** — packaging/transport layer only; hot path stays RocksDB + projections.
- **Whole RuVector monorepo as a dependency / “agent brain product”** — too large, marketing-heavy, many STUB/NO-OP facades; adapt only REAL crates behind Ports (see ruvector-plugin-adaptation research note).
- **External ETL + external agent orchestrator as target shape** — agents leave the runtime; contradicts “agents inside the engine.”
- **Early Raft multi-node or GNN learned indexes** — distract before P0/P1 single-node hybrid evidence; P4 only.
- **Pure hexagonal folder layout (`ports/` / `adapters/` repo-wide) as the primary structure** — low cohesion under requirement change; STCA requires vertical slices with hexagon *inside* the module.

## Marketing

**One-liner:** Event-sourced bi-temporal graph engine on Rust where agents are first-class, governed by meta-prompts and dictionaries, with performance from pluggable materializations — not agents bolted onto storage.

**Key message:** STCA first — Space (packs + ports) × Time (log = proof) × Composition (Cui budgets). Kutha = STCA + hot pictures of the log + dict-first agents. LLM proposes; log and dictionaries own audited truth. Wedge: legal temporal agents first; science and other regulated verticals on the same physics.
