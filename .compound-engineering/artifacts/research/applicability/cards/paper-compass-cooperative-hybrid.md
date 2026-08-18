---
id: paper-compass-cooperative-hybrid
source: paper
axes: [Query, Data]
usefulness: high
optimality: high
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# Compass: cooperative filtered search without a new hybrid index

Paper: [Compass: General Filtered Search across Vector and Structured Data](https://arxiv.org/abs/2510.27141) (Ye et al., 2025). Consensus: https://consensus.app/papers/details/bb11c209287157e0a3572c200a91b3a2/?utm_source=cursor

## 1. Raw idea

Do not invent a specialized hybrid index. Coordinate existing HNSW/IVF (vector) and B+-trees (relational) so candidate generation and predicate evaluation cooperate. Arbitrary conjunctions, disjunctions, and ranges; claims to beat NaviX while matching single-attribute specialized indexes when only one attribute is involved.

## 2. STCA applicability

Query: planner story complementary to NaviX (prefilter then kNN), ACORN (predicate subgraph), and DaMoN scan-vs-probe. Indexes stay access paths on the projection; the novelty is *execution*, not a fourth SoT.

## 3. Quality / cost

High: DBMS-compatible generality is what Kutha needs if Cypher filters are ad-hoc. Cost: arXiv; must not copy Compass into P0 — use it as the “no new index family” pole vs SIEVE’s many-indexes pole (queued).

## 4. Demand

Legal/science filters are multi-attribute and range-shaped. A specialized equality-only hybrid index will miss them.

## 5. Niche → effect

`no niche`
