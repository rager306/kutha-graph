---
id: paper-subgraph-iso-vs-homomorphism
source: paper
axes: [Query, Data, Composition]
usefulness: high
optimality: high
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# Cypher MATCH is not SPARQL BGP — injectivity is a compiler flag, not a second engine

Papers: [TurboHOM++](https://consensus.app/papers/details/55e34796c4f35072a0de758ba89d96e9/?utm_source=cursor) (Kim et al., 2015, VLDB) — SPARQL matching is **graph homomorphism**; a subgraph-isomorphism solver becomes an RDF engine by dropping injectivity; TurboHOM++ beats specialized RDF engines by up to 5 orders on in-memory billion-triple graphs [8]; [TurboISO](https://consensus.app/papers/details/db281286eb6c591f9d5d20b58ee12517/?utm_source=cursor) (Han et al., 2013) candidate regions + neighborhood equivalence classes (NEC) so identical query vertices are combined not permuted [16]; [DAF](https://consensus.app/papers/details/c212c795330d5f91aa706a4c74f35183/?utm_source=cursor) (Han et al., 2019, SIGMOD) DAG dynamic programming + failing sets vs CFL/TurboISO spanning-tree weakness [3]; [strong simulation](https://consensus.app/papers/details/3b09db8dd16e57b1817f2f115228e63f/?utm_source=cursor) (Ma et al., 2011) cubic-time relaxation that keeps topology; isomorphism is NP-complete [9]; [VF2++](https://consensus.app/papers/details/160ba900103759e7b8312e5f7e69123a/?utm_source=cursor) (Jüttner & Madarasi, 2018, DOI: 10.1016/j.dam.2018.02.018) matching-order + cutting rules [1]; [HFrame homomorphism GNN](https://consensus.app/papers/details/6eec9537909655a096102956ea27ae49/?utm_source=cursor) (Guo et al., 2025) homomorphism ≠ injective iso [15]. Distinct from `paper-lftj-wcoj` / `paper-kuzu-factorized-wcoj` (AGM-optimal *joins*, typically homomorphism of CQs) and `paper-temporal-motifs` (time-respecting *mining*, not query semantics).

## 1. Raw idea

Two matching contracts. **Isomorphism** (classic VF2/TurboISO/DAF): injective mapping, no two query vertices share a data vertex; Cypher `MATCH` historically this. **Homomorphism** (SPARQL BGP): several query vertices may map to one data vertex; drop the injectivity check [8]. **Simulation / strong simulation**: polynomial relaxations; they return compact, topology-preserving matches instead of enumerating embeddings [9]. Backtracking literature (candidate regions, NEC, failing sets, DAG order) is *not* the WCOJ literature (leapfrog / generic join / AGM). An LLM-oracle VF3 is EnumP-complete to enumerate; robustness+output-polynomial together is as hard as P=NP [11] — keep ML off the matcher.

## 2. STCA applicability

Query 070: the compiler must emit an explicit **injectivity flag** (Cypher iso vs SPARQL homo vs optional `MATCH` uniqueness). Data: WCOJ on tries is the hot conjunctive path (ADR-000); a VF-style backtracker is a **lease operator** for small labeled patterns / induced iso / chemical-style graphs, not a second SoT. Composition: homomorphism of CQs *is* what LFTJ already computes; this card forbids silently selling “pattern matching” as one algorithm. Temporal motifs stay mining; this is *query* MATCH.

## 3. Quality / cost

Usefulness high: GQL/Cypher vs SPARQL export will disagree without the flag. Optimality high: TurboHOM++ shows the iso engine, de-injected, can beat RDF stores [8]; DAF/TurboISO are the backtracking state of the art [3][16]. Cost: P0 = leapfrog homomorphism (CQ) + optional injectivity filter on output; do not import VF3 as the Cypher runtime. Simulation as a pack when users want “shape” not embeddings [9].

## 4. Demand

Legal citation patterns and chemical/knowledge packs both break if iso and homo are confused (same statute cited twice vs two distinct hops). Engine demand: one matcher family, two semantics, documented in the compiler.

## 5. Niche → effect

`no niche`
