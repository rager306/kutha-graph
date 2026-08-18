---
id: paper-pghive-schema-discovery
source: paper
axes: [Data, Query, Space]
usefulness: med
optimality: med
demand: med
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# Incremental schema discovery emits dictionary events (PG-HIVE / DiscoPG)

Papers: [PG-HIVE: Hybrid Incremental Schema Discovery for Property Graphs](https://consensus.app/papers/details/27ccf5a0457455989c7aad8a9665c169/?utm_source=cursor) (Sideri et al., 2025, arXiv:2512.01092, DOI: 10.48550/arxiv.2512.01092) [1]; demo [PG-HIVE EDBT](https://consensus.app/papers/details/0b100f91884f517c99fa292555bc01c1/?utm_source=cursor) [2]; [DiscoPG](https://consensus.app/papers/details/cb56a1d8d6ff5d7a9b293b497be847bc/?utm_source=cursor) (Bonifati et al., 2022, PVLDB) first PG schema-discovery system; GMM on labels+properties; static and dynamic [3]; [GMMSchema](https://consensus.app/papers/details/845dec9579485c5195debc53dcc175f8/?utm_source=cursor) hierarchical GMM [6]. Distinct from `paper-pgschema-types-keys` (prescriptive PG-Types/Keys) and `paper-gql-rules-materialization` (derived edges).

## 1. Raw idea

Schema-free property graphs are hard to query, integrate, and visualize. DiscoPG/GMMSchema **mine** latent node types from labels and properties via Gaussian mixture hierarchical clustering [3][6]. PG-HIVE adds LSH + property/label clustering, infers datatypes, constraints, cardinalities **even without labels**, and does it **incrementally** so new data does not recompute the whole schema (up to +65% node / +40% edge accuracy vs SOTA; ~1.95× faster) [1]. Output is a *descriptive* schema, not a write-time DDL.

## 2. STCA applicability

Space/Data: discovered types are **dictionary candidates**. They must land as log events (“propose type T for label set L”) that a human or TOKI operator accepts — not a silent rewrite of the fold. Query: a mined schema can drive AZ-style projected MATCH *after* it is sealed in dicts. Distinct from PG-Schema, which *declares* types; discovery is the on-ramp when the graph arrived schema-free (ingest packs, Graphiti dumps). Do not treat the clustering model as SoT.

## 3. Quality / cost

Usefulness med: every messy ingest needs a first schema guess. Optimality med: LSH+GMM is fine for a pack; not a hot-path join. Cost: run discovery as a batch/incremental pack; emit dict events; validate with PG-Keys afterwards.

## 4. Demand

“We dumped JSON into Neo4j, now what are the types?” Engine demand: dict proposals. Product demand: DiscoPG-class explorer — keep it a tool, not the catalog.

## 5. Niche → effect

`no niche`
