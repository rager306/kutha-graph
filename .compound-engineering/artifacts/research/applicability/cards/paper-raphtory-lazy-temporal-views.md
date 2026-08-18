---
id: paper-raphtory-lazy-temporal-views
source: paper
axes: [Time, Data, Query]
usefulness: med
optimality: med
demand: med
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# Lazy temporal graph views over a change log (Raphtory)

Papers: [Raphtory: Streaming analysis of distributed temporal graphs](https://consensus.app/papers/details/d854b735d72e572e9b45275da9e9ef40/?utm_source=cursor) (Steer et al., 2020, Future Gener. Comput. Syst., DOI: 10.1016/j.future.2019.08.022); JOSS rewrite [Raphtory: The temporal graph engine for Rust and Python](https://arxiv.org/html/2306.16309) (Arnold et al., arXiv:2306.16309). Related: [TARIS](https://consensus.app/papers/details/4a65890232335b4ba4542b9352f776bd/?utm_source=cursor) (Bhoot et al., 2024) incremental time-respecting algorithms on streams; [streaming-graph taxonomy](https://consensus.app/papers/details/cd7e8b5ad493535b8198d9bf528bb485/?utm_source=cursor) (Besta et al., 2019/2021).

## 1. Raw idea

Raphtory maintains a **chronological log of structural and property changes**, then exposes many **lazy graph views** (time windows, predicate subgraphs, layers, deletion semantics) that materialize only on access — thousands of perspectives without cloning the graph. Streamed events insert into the model without batching (2020: full history in memory, distributed). Algorithms: temporal motifs, temporal reachability, sliding-window PageRank. Rust core + Python + GraphQL. Contrast family: snapshot vs event ML models (UTG); incremental ICM (TARIS) vs GRADOOP/Tink.

## 2. STCA applicability

Time/Data: a change log plus on-demand views is the same *shape* as Kutha log + reversible projections. Do **not** take Raphtory as SoT: 2020 design keeps **full history in RAM**; it is a temporal-network analytics engine, not an auditable event-sourced GDBMS with WCOJ/Cypher. Query: window/subgraph views are Query-axis filters, not join optimality. Distinct from already-closed T-GQL intervals and GRADOOP TPGM (batch dataflow, bitemporal collections).

## 3. Quality / cost

Usefulness med: lazy views are the right *picture* API (one log, many windows). Optimality med: Rust + lazy materialization; no AGM/WCOJ story; memory bound is the product. Cost to Kutha: borrow **view descriptors** (window, layer, subgraph) over the event log; reject in-memory-full-history as the storage contract. TARIS shows stream incremental temporal algos can beat GRADOOP by orders of magnitude — queued, not this card.

## 4. Demand

Network-science and streaming-ingest users need AS-OF windows without rebuilding graphs. Engine demand: cheap view handles on the log. Not a substitute for legal PIT + receipts.

## 5. Niche → effect

`no niche`
