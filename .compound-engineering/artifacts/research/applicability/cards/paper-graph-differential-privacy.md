---
id: paper-graph-differential-privacy
source: paper
axes: [Security, Query, Time, Verify]
usefulness: high
optimality: med
demand: high
confidence: paper
layer5: "Legal/enterprise: publish degree histograms or AS-OF counts without identifying a party; DP is not path-ABAC and not tenant slices"
status: closed
channels_failed: []
---

# Edge-DP vs node-DP vs event-level — noise is a query lease, not a second SoT

Papers: [Analyzing Graphs with Node Differential Privacy](https://consensus.app/papers/details/c884306e9967563b8a24590e096ef935/?utm_source=cursor) (Kasiviswanathan et al., 2013, DOI: 10.1007/978-3-642-36594-2_26) — node-DP hides a vertex *and* all incident edges; project onto bounded-degree graphs [8]; [N2E](https://consensus.app/papers/details/899a0edd56865521841320ba17ed37f6/?utm_source=cursor) (Hu et al., 2025) reduces node-DP tasks to edge-DP with error on the *true* max degree, not a conservative cap [6]; [Fully Dynamic Algorithms for Graph Databases with Edge Differential Privacy](https://consensus.app/papers/details/7dc58aaf6cac54e296d144a891a49414/?utm_source=cursor) (Raskhodnikova et al., 2025, PACMMOD) — continual release under inserts *and* deletes; **event-level** (one update) vs **item-level** (one edge across time) [2]; [Differential privacy and SPARQL](https://consensus.app/papers/details/d24b8343202952bb966118b6fe3fa79f/?utm_source=cursor) (Buil-Aranda et al., 2023) counting queries over a large SPARQL class when the RDF graph carries structural metadata [13]; SoK [17]. Distinct from `paper-xacml4g-path-abac` (may this hop return?), `paper-graph-tenant-isolation` (slice / quota), `paper-wasm-udf-sandbox` (process isolation).

## 1. Raw idea

Two neighborhood relations. **Edge-DP**: neighboring graphs differ by one edge (hides a relationship). **Node-DP**: differ by one vertex and all its edges (hides a person); much higher sensitivity; classic fix is degree projection [8]; N2E clips with a private max-degree estimate so you do not divide ε by a worst-case bound [6]. On a *mutating* graph, privacy grain splits again: event-level (one log update) vs item-level (all updates of one edge) [2]. Publishing the whole graph under node-DP is still young [1]. SPARQL counts can be DP if joins are handled and schema metadata exists [13]. Clique-projection / GNN local-DP papers are a different product (learn on a noisy graph), not this card.

## 2. STCA applicability

Security 080: DP is a **query-time noise lease** on aggregates/histograms, not a rewrite of the event log. Time: event-level DP ≈ one event; item-level DP ≈ one identity across valid-time — do not confuse with AS-OF correctness. Query: only *counting / histogram / cut* class in P0; pattern MATCH stays exact inside the tenant slice. Verify: a DP answer is *not* a receipt of a hop; quantum receipts stay exact. Do not store Laplace-noised CSR as SoT.

## 3. Quality / cost

Usefulness high: STRATEGY already wants shareable analytics without doxing a node. Optimality med: node-DP is expensive; fully dynamic item-level is new [2]. Cost: P0 = no DP; honeycomb = ε-budget on compiled aggregate Cypher + event- vs item-level flag; do not privatize the log.

## 4. Demand

Counsel graphs and citation graphs will be asked for “how many” without “who.” Engine demand: DP operator on aggregates, not on leapfrog MATCH.

## 5. Niche → effect

Legal/enterprise: publish degree histograms or AS-OF counts without identifying a party; DP is not path-ABAC and not tenant slices.
