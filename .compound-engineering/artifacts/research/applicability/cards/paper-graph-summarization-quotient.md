---
id: paper-graph-summarization-quotient
source: paper
axes: [Data, Query, Composition]
usefulness: high
optimality: high
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# A summary is supernodes + corrections — not k² and not an OLAP cuboid

Papers: lossless summarization = **summary graph** (supernodes/superedges) plus **correction edges** that restore the original exactly [2][7][12]; [MoSSo](https://consensus.app/papers/details/a6215a2a69a954ac8c6fe5d7db10869b/?utm_source=cursor) (Ko et al., 2020, KDD) incremental on fully dynamic streams, sub-millisecond per update [7]; [SWeG](https://consensus.app/papers/details/2050ec761c1b5dc3a0795d5be6128ee3/?utm_source=cursor) (Shin et al., 2019) parallel/MapReduce, tens of billions of edges, combinable with other compressors [12]; [quotient/bisimulation survey](https://consensus.app/papers/details/2da4168a7db95b8ea3fdfa14dfdec034/?utm_source=cursor) (Scherp et al., 2023) structural summaries as equivalence classes; queries on the quotient = queries on the original when features are preserved [3]; lossy summaries with reconstruction/cut-norm error answer adjacency/degree/triangles on supernodes [5]; SSumM sparsifies the summary under MDL [17]; IBA-OTC lossless KG triples with ±/* corrections [1]. Distinct from `paper-k2tree-succinct-graph` (encode the *same* adjacency, no supernodes), `paper-graph-olap-cube` (dimensional roll-up, not neighborhood merge), `paper-horae-temporal-sketches` (stream sketches), `paper-semi-external-graph` (placement, not merge). Personalized/supervised OT summaries stay queued.

## 1. Raw idea

Merge similar neighborhoods into supernodes; record the exceptions [7][12]. Lossless: original reconstructs. Lossy: queries run on the small graph with a bound [5][8]. The summary is a **droppable lease**; the log still has every edge.

## 2. STCA applicability

Data: supernode partition is a reversible materialization (040), like CSR — drop and rebuild. Query: some MATCH/analytics compile onto the quotient [3]; correction edges are the fail-closed path back to the fold. Composition: size budget k bits (SSumM) is Cui-shaped [17]. Time: MoSSo updates the lease as the stream moves; do not treat the summary as transaction-time. Agent: LLM does not pick merges. Verify: corrections are the why of a reconstructed edge.

## 3. Quality / cost

Usefulness high: visualization and RAM-fit analytics on huge folds. Optimality high: MoSSo/SWeG/quotients are measured. Cost: P0 = none (query the fold); honeycomb = optional lossless summary lease + corrections. Do not make supernodes the SoT. Do not confuse with OLAP GROUP BY.

## 4. Demand

A 40 B-edge fold will not paint. Engine demand: a reconstructible supergraph lease, fail-closed to “answer on the fold.”

## 5. Niche → effect

`no niche`
