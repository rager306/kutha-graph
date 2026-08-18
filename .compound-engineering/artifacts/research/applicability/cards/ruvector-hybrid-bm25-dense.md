---
id: ruvector-hybrid-bm25-dense
source: ruvector
axes: [Query, Data]
usefulness: high
optimality: med
demand: high
confidence: observed
layer5: no niche
status: closed
channels_failed: []
---

# Hybrid BM25 + dense + RRF retrieve

Evidence: prior REAL classification of `ruvector-hybrid` crate in adaptation note; CBM shows hybrid search on nervous-system integration. Not re-audited line-by-line this wave → `observed` not `code` for the RRF crate itself.

## 1. Raw idea

Fuse lexical and vector scores (RRF) instead of vector-only RAG.

## 2. STCA applicability

Query: hybrid planner (ADR-043/071). Data: two materializations, one log.

## 3. Quality / cost

Useful pack. Cost: PostgreSQL/hybrid SQL mocks exist in the monorepo — those stay `claim` if promoted later.

## 4. Demand

Agent and legal retrieval both fail on embedding-only or keyword-only.

## 5. Niche → effect

`no niche`
