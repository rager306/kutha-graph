---
title: RuVector → Kutha plugin adaptation strategy
date: 2026-08-17
status: research
sources:
  - /root/ruvector-blas/docs/audit/ruvector-graph-stub-capability-map.md (2026-07-15)
  - /root/.gsd/projects/e9f95f84e8b9/explorations/ruvector-deep-audit-2026-07-10.md
  - /root/.gsd/projects/e9f95f84e8b9/explorations/codebase-memory-ruvector-arch-2026-07-10.md
  - /root/daily-archive/doc/adr/ADR-040-technology-stack-lock-samyama-ruvector-rvf.md
  - ADR-000 D5/R8/P3–P4; ADR-001; ADR-090/093 RVF boundaries
  - codebase-memory project: root-vendor-source-ruvector (~531k nodes)
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

## Plugin map (proposed packs)

| Pack (working name) | Borrow from RuVector (REAL-leaning) | Explicitly do **not** borrow | Kutha cell / phase |
|---------------------|--------------------------------------|------------------------------|--------------------|
| **P-HNSW** | `rvf-index` / core HNSW build+search; postgres `hnsw_am` as *reference*, not dependency | Mock `ruvector_hybrid_search`, fixed-score facades | ADR-042 · P2 |
| **P-RVF-Seal** | `rvf-runtime` CowEngine, freeze/fork, segment layout; WitnessChain generate/verify | RVF as primary store; npm fallback VectorDB stubs | ADR-091 · P3 |
| **P-Witness-Bridge** | Witness receipts ↔ Quantum Receipt digests (event segment + lease IDs + dict@version) | Fabricated GraphRAG “reasoning paths” | ADR-014/091 · P1–P3 |
| **P-Hybrid-Retrieve** | `ruvector-hybrid` BM25+dense+RRF **crate** (REAL per audit) | PostgreSQL hybrid SQL mock; GraphRAG context retrieval stub | ADR-071 · P2 |
| **P-GNN-Rerank** | `ruvector-gnn` layer/attention primitives; optional message_pass | Graph-hybrid GNN facade with fixed `[0.7,0.2,0.1]` / zero embeddings | ADR-052 · P2–P4 |
| **P-Agent-Memory** (optional) | Narrow agent-memory / SONA patterns *as content behind dictionaries* | MemoryMiddleware as D3 control plane; whole “agent brain” product | ADR-050/052 · later |
| **Never as Kutha query SoT** | — | Cypher QueryExecutor empty path; SET/DELETE no-ops; federated empty success; npm FALLBACK-STUB backends | Rejected |

## Adoption ladder

1. **Inventory pin** — freeze vendor revision; import stub-map statuses for candidate symbols only.
2. **Probe memo (≤ few days each)** — HNSW fence vs Samyama; RVF seal schema vs Witness fields; hybrid crate vs our ports — *before* in-tree code (prior ideate idea pattern).
3. **Adapter spike** — one pack, one Port, truthfulness tests green.
4. **Honeycomb ADR** — open ADR-091 / 042 / 052 cell with borrow/contrast table.
5. **Vertical use** — legal/science RVF export only after P0–P1 physics (ADR-090/093 already say packaging ≠ storage).

## Anti-goals

- Binding Kutha core Cargo workspace to full `/vendor-source/ruvector`.
- Claiming “Cypher via RuVector” or “GraphRAG included.”
- Using utilization % marketing (e.g. “22/92 wired”) as readiness for Kutha.
- Replacing event-log SoT or dict-first control with RuVector memory middleware.

## Traceability

- Stub/REAL matrix: `ruvector-blas/docs/audit/ruvector-graph-stub-capability-map.md`
- Capability ROI (Postgres-era): `.gsd/.../ruvector-deep-audit-2026-07-10.md`
- Architecture snapshot: `.gsd/.../codebase-memory-ruvector-arch-2026-07-10.md`
- Live index: codebase-memory `root-vendor-source-ruvector`
- Product remapping precedent: daily-archive ADR-040 (Samyama/RuVector/RVF tiers)
