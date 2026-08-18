---
id: helix-indexes-as-access-paths
source: helix
axes: [Data, Query]
usefulness: high
optimality: med
demand: high
confidence: spec
layer5: no niche
status: closed
channels_failed: []
---

# Indexes as access paths over node/edge properties

Evidence: HelixDB docs (Jina, 2026-08-17): https://docs.helix-db.com/database/helix-db/start-here/introduction — vector/text/secondary index is an access path over a property already on a node or edge; graph remains SoT; object storage as storage SoT + NVMe cache. No kernel tree → never `code`.

## 1. Raw idea

HNSW/FTS/BTree are not sibling databases; they are indexes on the same labeled property graph, including edges.

## 2. STCA applicability

Data: same as Kutha materializations-as-indexes. Contrast: Helix object-storage SoT vs Kutha event-log SoT.

## 3. Quality / cost

Strong conceptual fit. Cost: object-storage latency model may fight P0 in-process Rust core.

## 4. Demand

Stops the “graph DB + Pinecone + Elastic” sync tax.

## 5. Niche → effect

`no niche`
