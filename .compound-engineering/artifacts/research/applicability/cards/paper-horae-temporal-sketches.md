---
id: paper-horae-temporal-sketches
source: paper
axes: [Time, Query, Data]
usefulness: med
optimality: med
demand: med
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# Approximate graph-stream sketches for temporal range queries (Horae family)

Papers: [Horae: A Graph Stream Summarization Structure for Efficient Temporal Range Query](https://consensus.app/papers/details/20047a5fd7f956688bcc90e8cc4bbc4c/?utm_source=cursor) (Chen et al., 2022, ICDE, DOI: 10.1109/icde53745.2022.00254); [GRIT](https://consensus.app/papers/details/14e85b03745d59b28f18875d688f1962/?utm_source=cursor) (Hu et al., 2025, CIKM, DOI: 10.1145/3746252.3761105); [HIGGS](https://consensus.app/papers/details/1df556d901fe52baad75ebc55187be1b/?utm_source=cursor) (Zhao et al., 2024/2025, ICDE, DOI: 10.1109/icde65448.2025.00114); [PGSS-MDC](https://consensus.app/papers/details/500a30384e935f02abae75dc2d5e0ace/?utm_source=cursor) (Jia et al., 2023, World Wide Web, DOI: 10.1007/s11280-023-01165-z). Cousins: GSS / HourglassSketch / Sliding-ITeM; neural [Crane](https://arxiv.org/abs/2602.15360). Distinct from `paper-raphtory-lazy-temporal-views` (exact full history) and `paper-taris-incremental-icm` (exact incremental ICM).

## 1. Raw idea

Graph streams are unbounded edge sequences; storing the full adjacency is often impossible. Hash-compressed matrices (GSS, TCM) summarize topology but drop time, so they cannot answer “what existed in `[t1,t2]`.” Horae embeds a **time prefix** in a multi-layer sketch: any interval of length \(L\) decomposes into \(\leq 2\log L\) prefix-aligned sub-ranges (Binary Range Decomposition). GRIT flattens time into a `FlatIndex` + lazy updates to cut multilayer error. HIGGS is item-based bottom-up hierarchy (localize hash conflicts). PGSS-MDC uses fixed-size hierarchical counters for persistent past-range queries. All return **approximate** edge-weight / 1-hop answers with controllable error.

## 2. STCA applicability

Time/Query: a sketch is a **lossy lease** over the event log, same family as HNSW/NaviX — reversible only in the weak sense of “rebuild from log.” Do **not** make Horae SoT. Data: compressed matrices are not CSR/WCOJ materializations; they do not preserve identity, properties, or receipts. Distinct from Raphtory (lazy exact views over an in-RAM change log) and TARIS (exact time-respecting algorithms). Cyber/social stream dashboards can sit on a sketch pack; legal AS-OF cannot.

## 3. Quality / cost

Usefulness med: two–three orders of latency vs naive stream stores when you only need range-filtered topology counts. Optimality med for that job, **low** as a graph engine: errors accumulate; Crane shows frequent items starve rare ones under tight memory. Cost: optional `SketchView` pack with explicit error bounds; never a substitute for the log or for PIT Cypher.

## 4. Demand

Network/security/fraud teams want “edges in the last hour” without retaining the full stream. Engine demand: bounded approximate views. Product demand: they will try to skip the log — refuse.

## 5. Niche → effect

`no niche`
