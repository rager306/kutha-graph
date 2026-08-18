---
id: graphiti-bitemporal-fact-edges
source: graphiti
axes: [Time, Agent, Query]
usefulness: med
optimality: low
demand: high
confidence: code
layer5: no niche
status: closed
channels_failed: []
---

# Graphiti contrast: LLM-extracted facts with valid/invalid/expired on edges

CBM `graphiti` (coverage: `graphiti_core/edges.py`, `graphiti_core/graphiti.py`, indexed_no_recorded_gap, best-effort). `EntityEdge` carries `valid_at` / `invalid_at` / `expired_at` / `reference_time` plus `fact_embedding`. `Graphiti.add_episode` LLM-extracts nodes/edges from episode text, then writes to Neo4j/Falkor/Neptune/Kuzu.

## 1. Raw idea

Zep-class temporal KG memory: episodes in, LLM extracts entities and RELATES_TO facts, edges get bi-temporal-ish stamps and embeddings, search mixes hybrid retrieve. Graph is the working memory, not an event-log fold.

## 2. STCA applicability

Time: invalidate-not-delete on *edges* is a useful *shape* (Engram/MemStrata/TOKI), but Graphiti’s SoT is the extracted graph in an external GDBMS — rejected for Kutha (D1/D2). Agent: LLM is on the write path as extractor (TGMS/compiler-not-executor anti-pattern unless receipts). Query: hybrid search over fact embeddings is demand, not a second store.

## 3. Quality / cost

Observed product-market fit for agent memory. Cost: extraction nondeterminism, no WCOJ/CSR core, driver zoo, sequential `add_episode`. Keep as **contrast card**, not a kernel to copy.

## 4. Demand

The market already expects “as-of facts + embeddings” for agents. Kutha must answer that demand from the log, not by becoming Graphiti.

## 5. Niche → effect

`no niche` — contrast, not a vertical pack.
