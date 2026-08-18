---
id: paper-graph-pack-plugin-lifecycle
source: paper
axes: [Composition, Space, Data]
usefulness: high
optimality: med
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# A pack is a versioned module — not a WASM sandbox and not a second SoT

Papers: [DuckPGQ](https://consensus.app/papers/details/0b0861fc7ff05f0cae1579bdebd80068/?utm_source=cursor) (ten Wolde et al., 2023, VLDB) SQL/PGQ as a DuckDB *extension*: new functions, types, operators, optimizer rules, even parsers injected into the host plan [19]; [RDF KB index/store versioning lifecycle](https://consensus.app/papers/details/307e19cb16005a7b8a17b99b7e402e1f/?utm_source=cursor) (Bellini et al., 2015) ontology/index deletes are not black-box rebuilds; a lifecycle tool versions the store when concepts change [1][2]; [Living Databases](https://consensus.app/papers/details/c9aca8ad502c53e1977b194091452e2e/?utm_source=cursor) (Deshpande, 2026) unifies schema evolution, versioning, transformations, provenance, and dependent objects (views/models) under one primitive [9]. Distinct from `paper-wasm-udf-sandbox` (isolate untrusted *code*), `paper-iso-gql-gpml` (GQL *surface*, not how the module plugs in), `paper-graph-branch-fork` (named data refs), `oxify-dag-llm-orchestration` (workflow DAG, not engine packs). ChronoGraph remains a snapshot cousin (queued skip).

## 1. Raw idea

Two lifecycle nouns. (1) **Host extension**: a graph pack registers operators and types into a host engine without forking it [19]. (2) **Index/store versioning**: when a dictionary or ontology pack changes, materialized triples/indexes must be versioned or the delete cascades [1][2]. Generic “plugin marketplace” and frontend feature-flag papers are not this card. WASM/Rhai in OxiFY is *how a node runs*, not *how a pack is installed, activated, and rolled back*.

## 2. STCA applicability

Composition 021: a Kutha pack is a **versioned artifact** (operators + dictionary + leases) with install/activate/retire events on the log. Space: port isolation is cargo/workspace; this card is the *runtime lifecycle* of that port. Data: CSR/HNSW rebuilds are pack-versioned leases, not silent mutation of SoT. DuckPGQ’s “inject parser/optimizer” is the compile-hook shape; do not vendor DuckDB. Living Databases’ dependent-object evolution is the same noun as reversible materializations.

## 3. Quality / cost

Usefulness high: honeycomb 021 is empty without a pack version type. Optimality med: DuckPGQ is a real extension point; RDF lifecycle tools are project-specific. Cost: P0 = statically linked operators; honeycomb = pack manifest + activate event + lease rebuild; do not stand up a plugin marketplace.

## 4. Demand

Legal and scientific packs will ship on different cadences. Engine demand: activate/retire as events so a bad pack rolls back without rewriting the log.

## 5. Niche → effect

`no niche`
