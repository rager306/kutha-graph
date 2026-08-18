---
id: paper-sparql-multi-query-opt
source: paper
axes: [Query, Composition]
usefulness: high
optimality: high
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# A batch shares common subpatterns — not a result cache and not admission

Papers: [SPARQL MQO](https://consensus.app/papers/details/510af3e4552a5c6da9ec7bf140484b83/?utm_source=cursor) (Le et al., 2012, ICDE) NP-hard; partition a batch, discover common SPARQL sub-structures, cost-compare candidate combined plans; **engine-agnostic** rewrite [1]; share common BGP evaluation to cut sequential time [3]; [SwarmGuide](https://consensus.app/papers/details/dd0b78289c435b029e812fcc8b98c5b1/?utm_source=cursor) / RPQ MQO globally optimize visual/batched regular-path queries by commonalities [10][16]; federated RDF MQO rewrites with SPARQL 1.1 sharing while costing data shipment [2] — federation stays on its card, sharing is this noun. Distinct from `paper-graph-query-result-cache` (memo of one compiled query at a log offset), `paper-query-admission-control` (admit/reject), `paper-graph-cardinality-estimation` (single-query stats), `paper-adopt-adaptive-wcoj-orders` / `paper-hybrid-wcoj-intersection-cost` (order/intersections *inside* one or continuous query; Graphflow’s combined *delta* plan is a cousin, not batch MQO), `paper-federated-sparql-query` (`SERVICE`). SHACL-driven rewrite and Lothbrok P2P stay queued.

## 1. Raw idea

Many queries arrive together (dashboard, agent fan-out, visual builder). They share BGPs or RPQ fragments. Compile **once** a shared subplan, then specialize [1][16]. Not “cache yesterday’s result” and not “throttle the queue.”

## 2. STCA applicability

Query: MQO is a planner pass over a *batch* of compiled Cypher/SPARQL, producing a shared operator DAG (031 pack scheduling cousin). Composition: common-subpattern detection is the honeycomb; cost model must see leases (CSR/HNSW) not just triples. Time: a shared plan is valid at one log offset; after append, invalidate like a result cache. Verify: sharing must not mix tenants (path-ABAC still rewrites each leaf). Agent: LLM may emit the batch; MQO is deterministic.

## 3. Quality / cost

Usefulness high: agents issue near-duplicate MATCH. Optimality high: Le 2012 is the SPARQL landmark; RPQ SwarmGuide extends the noun. Cost: P0 = sequential plans; honeycomb = common-BGP/RPQ share in the compiler. Do not make MQO a second SoT. Do not cite product unified engines as the algorithm.

## 4. Demand

A legal pack’s 250 similar matter queries should not redo the same hop 250 times. Engine demand: batch-aware compile with shared subplans, fail-closed to independent plans.

## 5. Niche → effect

`no niche`
