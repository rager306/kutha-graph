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

# Pattern matching needs separate node, relationship, and result contracts

Papers: [TurboHOM++](https://consensus.app/papers/details/55e34796c4f35072a0de758ba89d96e9/?utm_source=cursor) (Kim et al., 2015, VLDB) — SPARQL matching is **graph homomorphism**; a subgraph-isomorphism solver becomes an RDF engine by dropping injectivity; TurboHOM++ beats specialized RDF engines by up to 5 orders on in-memory billion-triple graphs (8); [TurboISO](https://consensus.app/papers/details/db281286eb6c591f9d5d20b58ee12517/?utm_source=cursor) (Han et al., 2013) candidate regions + neighborhood equivalence classes (NEC) so identical query vertices are combined not permuted (16); [DAF](https://consensus.app/papers/details/c212c795330d5f91aa706a4c74f35183/?utm_source=cursor) (Han et al., 2019, SIGMOD) DAG dynamic programming + failing sets vs CFL/TurboISO spanning-tree weakness (3); [strong simulation](https://consensus.app/papers/details/3b09db8dd16e57b1817f2f115228e63f/?utm_source=cursor) (Ma et al., 2011) cubic-time relaxation that keeps topology; isomorphism is NP-complete (9); [VF2++](https://consensus.app/papers/details/160ba900103759e7b8312e5f7e69123a/?utm_source=cursor) (Jüttner & Madarasi, 2018, DOI: 10.1016/j.dam.2018.02.018) matching-order + cutting rules (1); [HFrame homomorphism GNN](https://consensus.app/papers/details/6eec9537909655a096102956ea27ae49/?utm_source=cursor) (Guo et al., 2025) homomorphism ≠ injective iso (15). Distinct from `paper-lftj-wcoj` / `paper-kuzu-factorized-wcoj` (AGM-optimal *joins*, typically homomorphism of CQs) and `paper-temporal-motifs` (time-respecting *mining*, not query semantics).

## 1. Raw idea

**Subgraph isomorphism** requires an injective vertex mapping; **homomorphism**, as in SPARQL BGP matching, can map different query vertices to one data vertex (8). Cypher's default relationship uniqueness is a separate rule: relationships cannot be reused within the same graph pattern match, while nodes can repeat. Cypher 25 also exposes `REPEATABLE ELEMENTS`; version and match mode therefore matter. See the [official matching rules](https://neo4j.com/docs/cypher-manual/current/patterns/repeatable-node-and-relationship-paths/). A cycle `A→B→A` using two distinct relationships distinguishes a trail from a path requiring all vertices to be distinct.

**Simulation / strong simulation** are polynomial relaxations returning compact matches (9). Candidate regions, NEC, and failing sets belong to backtracking; leapfrog/generic joins use a different execution strategy. Neither algorithm family alone specifies result multiplicity or path identity.

## 2. STCA applicability

Query 070: the proposed compiler contract separates vertex injectivity, relationship uniqueness and its scope, walk/trail/simple restrictions, path selection (such as shortest), endpoints versus path enumeration, and set/bag output. These are independent choices, not one “Cypher iso / SPARQL homo” flag. Data: LFTJ is the theoretical conjunctive-join baseline; extra equality/inequality and uniqueness predicates must survive planning. A VF-style backtracker can serve small labeled or induced-isomorphism patterns as an optional operator. Temporal motifs remain mining.

## 3. Quality / cost

Usefulness high: prevents incompatible matching contracts from being reported as the same answer. Optimality high reflects the referenced algorithms (8)(3)(16), not an implemented Kutha matcher. Cost: current P0 supplies `leapfrog_intersect` over sorted rows, not variable-ordered LFTJ, a Cypher parser, or an injectivity filter over complete matches. Those remain proposed work under existing ADRs; simulation can be an optional pack (9).

## 4. Demand

Legal citation patterns and chemical/knowledge packs differ on repeated nodes, repeated relationships, and duplicate result rows. Engine demand: explicit semantic contracts retained through compilation and execution.

## 5. Niche → effect

`no niche`
