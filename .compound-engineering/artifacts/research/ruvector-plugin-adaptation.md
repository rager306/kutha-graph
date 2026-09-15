---
title: "RuVector → Kutha plugin adaptation strategy"
date: "2026-08-17"
last_updated: "2026-09-13"
status: research
literature_bound: closed
notes:
  - "CE docs_root research note. Cards in applicability/cards/ remain SoT. This file maps named adapter crates onto already-open honeycomb cells. It is not a matrix expansion, not a delivery lease, and not permission to implement HNSW/Cypher/Rocks."
  - "Vendor crate trees live outside this git checkout. ADRs must cite this note and existing cards, never machine-local absolute paths."
---

# RuVector → Kutha: adapt as packs, not as a second engine

## Diagnosis

RuVector is a **rich monorepo** (Rust crates + npm + examples + research): real HNSW/ANN, RVF COW runtime, WitnessChain, GNN *primitives*, sparse+dense hybrid retrieval, property-graph stores in places.

It is also **marketing-heavy and stub-laden at facades**: Cypher `QueryExecutor` returns empty success; GraphRAG fabricates documents/paths; some PostgreSQL/hybrid SQL paths return mock scores; mutations that `Ok(())` without effect. Prior audit classified surfaces as REAL / REAL-DISCONNECTED / PARTIAL / STUB-* / NO-OP-SUCCESS (see stub capability map §4).

**Therefore:** Kutha must not vendor “RuVector the product.” Kutha **borrows proven crates behind STCA Ports**, with a **truthfulness gate** (REAL + wired + non-empty happy path + failure path). Everything else stays out of the core dependency closure.

This matches daily-archive ADR-040’s three-tier *intent*, remapped to Kutha locks:

| daily-archive tier | Role there | Kutha remapping |
|--------------------|------------|-----------------|
| Samyama | Knowledge SoT | **Event log = SoT**; Samyama-class CSR/HNSW = **materialization packs** |
| RuVector | Agent brain | **Optional intelligence packs** (ADR-052 class) — never mandatory |
| RVF | Experience / portable | **ADR-091 packaging** — never hot SoT |

## Adaptation principles

1. **Slice, don’t swallow** — depend on named crates/modules with pinned revision + capability matrix row, not the monorepo root.
2. **Facade distrust** — public marketing APIs are guilty until classified REAL with executable tests in *our* harness.
3. **Fail closed** — STUB-EMPTY / FIXED / FABRICATED / NO-OP-SUCCESS must error or stay unplugged; never silent empty success in Kutha CI.
4. **Ports first** — Kutha owns traits (`HnswAccessMethod`, `RvfSeal`, `WitnessVerify`, `RerankPort`); RuVector code is an *adapter*, swappable for Samyama/native later.
5. **Reversible packs** — unload/rollback side effects (D5); RVF seal does not become live adjacency.
6. **Truthfulness CI** — each borrowed surface gets: non-empty happy path, unsupported→error, reload/rollback where writes exist (stub map §10).

## CE grounding (2026-09-13)

Compound Engineering `docs_root` for this repo is `.compound-engineering/artifacts`. Honeycomb ADRs stay under `docs/ADR/`. Applicability **cards** are research SoT (163, bound closed). This note is the in-repo home for *adapter crate identifiers*.

Rules for citing RuVector from an ADR:

1. Cite a closed card when the noun already has one (`ruvector-hnsw`, `ruvector-hnsw-delete-repair`, `paper-acorn-predicate-subgraph`, `paper-navix-filtered-hnsw`, `paper-constant-size-evidence`, `ruvector-cypher-empty-success`, …).
2. Cite **this file** for crate-level adapter targets that have no dedicated card.
3. Do **not** mint new matrix cards for cousin crates (research-boundary: no Consensus Query 103+; vendor scouts already closed).
4. Do **not** write `/root/...` vendor paths into ADRs. Those paths are machine-local and fail CE claim validation on another clone.
5. Named adapter ≠ implemented pack. `.kutha/STATE.md` still freezes HNSW, Cypher, and M002. Proposed mapping does not authorize a crate dependency in `kutha-runtime`.

### Named adapter crates (identifiers, not in-tree paths)

Five crates are mapped as **Proposed** adapter targets. A sixth is a cousin of an existing noun and is **not** a new decision.

| Crate identifier | Pack | Honeycomb | Existing card (SoT) | Role |
|------------------|------|-----------|----------------------|------|
| `ruvector-temporal-tensor` | P-Temporal-Tensor | ADR-012 D012-5 | (none — cousin of snapshot/tier leases) | Diachronic embedding buffers as droppable tiers |
| `ruvector-retrieval-receipt` | P-Retrieval-Receipt | ADR-014 D014-5 / ADR-071 | `paper-constant-size-evidence` | Read-path Merkle over a result set; not the write quantum receipt |
| `ruvector-hnsw-repair` | P-HNSW | ADR-042 D042-2 | `ruvector-hnsw-delete-repair` | TombstoneOnly / BatchRepair / EagerRepair |
| `ruvector-acorn` | P-HNSW | ADR-042 D042-3 | `paper-acorn-predicate-subgraph` | Predicate-subgraph walk; NaviX remains the prefilter alternative |
| `ruvector-temporal-coherence` | P-Agent-Memory | ADR-052 D052-4 | (none — cousin of enrichment retrieve leases) | Composite memory *score*, never fact validity |
| `ruvector-proof-gate` | — | ADR-014 D014-2 cousin | `paper-query-admission-control` | Per-query admission gate ≠ Cui envelope *V*. **Not** a D014-n. |

`rvf-index` remains the core HNSW crate already covered by `ruvector-hnsw`.

## Plugin map (proposed packs)

| Pack (working name) | Borrow from RuVector (REAL-leaning) | Explicitly do **not** borrow | Kutha cell / phase |
|---------------------|--------------------------------------|------------------------------|--------------------|
| **P-HNSW** | `rvf-index` core HNSW; `ruvector-hnsw-repair` (TombstoneOnly, BatchRepair, EagerRepair); `ruvector-acorn` predicate-agnostic filtered graph | Mock `ruvector_hybrid_search`, fixed-score facades | ADR-042 · P2 |
| **P-RVF-Seal** | `rvf-runtime` CowEngine, freeze/fork, segment layout; WitnessChain generate/verify | RVF as primary store; npm fallback VectorDB stubs | ADR-091 · P3 |
| **P-Witness-Bridge** | Witness receipts ↔ Quantum Receipt digests (event segment + lease IDs + dict@version) | Fabricated GraphRAG “reasoning paths” | ADR-014/091 · P1–P3 |
| **P-Retrieval-Receipt** | `ruvector-retrieval-receipt` Merkle provenance commitments over query result sets (read-path tamper evidence) | Unverified query-engine scores as truth | ADR-014/071 · P2 |
| **P-Temporal-Tensor** | `ruvector-temporal-tensor` tiered quantization (8/7/5/3-bit), segment delta reuse, random-access frame decode | Uncompressed f32 storage for cold history | ADR-012 · P1–P2 |
| **P-Hybrid-Retrieve** | `ruvector-hybrid` BM25+dense+RRF **crate** (REAL per audit) | PostgreSQL hybrid SQL mock; GraphRAG context retrieval stub | ADR-071 · P2 |
| **P-GNN-Rerank** | `ruvector-gnn` layer/attention primitives; optional message_pass | Graph-hybrid GNN facade with fixed `[0.7,0.2,0.1]` / zero embeddings | ADR-052 · P2–P4 |
| **P-Agent-Memory** (optional) | `ruvector-temporal-coherence` (decay + graph coherence gate); narrow SONA patterns *as content behind dictionaries* | MemoryMiddleware as D3 control plane; whole “agent brain” product | ADR-050/052 · later |
| **Never as Kutha query SoT** | — | Cypher QueryExecutor empty path; SET/DELETE no-ops; federated empty success; npm FALLBACK-STUB backends | Rejected |

## Adoption ladder

Honeycomb cells 012/014/042/052/071/091 are already **Proposed**. Mapping a crate into those cells is L_map work. It does not start an adapter spike.

1. **Inventory pin** — freeze a vendor revision *when a delivery lease names the pack*; import stub-map statuses for candidate symbols only.
2. **Probe memo (≤ few days each)** — HNSW fence vs Samyama; RVF seal schema vs Witness fields; hybrid crate vs our ports — *before* in-tree code.
3. **Adapter spike** — one pack, one Port, truthfulness tests green — only after STATE names that pack (HNSW remains frozen until then).
4. **Accepted** — only when the cell is in the running engine (Nygard). Written mapping is not Accepted.
5. **Vertical use** — legal/science RVF export only after P0–P1 physics (ADR-090/093 already say packaging ≠ storage).

## Anti-goals

- Binding Kutha core Cargo workspace to full `/vendor-source/ruvector`.
- Claiming “Cypher via RuVector” or “GraphRAG included.”
- Using utilization % marketing (e.g. “22/92 wired”) as readiness for Kutha.
- Replacing event-log SoT or dict-first control with RuVector memory middleware.

## Traceability

In-repo:

- Cards: `.compound-engineering/artifacts/research/applicability/cards/` (`ruvector-hnsw`, `ruvector-hnsw-delete-repair`, `ruvector-hybrid-bm25-dense`, `ruvector-rvf-cow-seal`, `ruvector-cypher-empty-success`, `ruvector-gnn-facade`, `paper-acorn-predicate-subgraph`, `paper-navix-filtered-hnsw`, `paper-constant-size-evidence`, `paper-query-admission-control`)
- Dossier: `.compound-engineering/artifacts/research/applicability/sources/ruvector.md`
- Bound: `.compound-engineering/artifacts/research/applicability/research-boundary.md`

Machine-local (not architecture SoT; do not paste into ADRs):

- Vendor checkout crate names above; prior stub/REAL audits and GSD explorations
- Code graph project name `root-vendor-source-ruvector` when that index exists
- Neighbor remapping precedent: daily-archive technology-stack ADR (Samyama/RuVector/RVF tiers)
