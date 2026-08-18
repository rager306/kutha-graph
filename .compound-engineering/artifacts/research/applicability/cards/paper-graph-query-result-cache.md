---
id: paper-graph-query-result-cache
source: paper
axes: [Query, Composition, Time]
usefulness: high
optimality: med
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# Cached answers are a lease of a query — not IVM and not a KV TTL sidecar

Papers: [Graph-Aware, Workload-Adaptive SPARQL Query Caching](https://consensus.app/papers/details/022e132f9bc9543aae7b5614758705ae/?utm_source=cursor) (Papailiou et al., 2015, SIGMOD) canonical labelling of SPARQL graphs (incl. isomorphs); DP planner can use cached results as indexes; up to 100× average latency [1]; [eBay graph stores with application-level result caches](https://consensus.app/papers/details/5b877d91d569540495c67d02338e489f/?utm_source=cursor) (Nguyen et al., 2024) cache *final* results; writes delete impacted entries; strong consistency; production p95/p99 [3]; [one-hop sub-query result caches](https://consensus.app/papers/details/de7586309c805184b1bd9c59ef8f1db7/?utm_source=cursor) (Nguyen et al., 2024) cache immutable vertex-id sets per one-hop; p95 ≥2×, with rewrite ≥2.33× [4]; invalidation by analysing SPARQL graph patterns [7]; [KGraph](https://consensus.app/papers/details/dd1177e3a64156a3a2503cdff6ee3908/?utm_source=cursor) (Gao et al., 2025, ICDE) memoization of concurrent graph queries, partition-local, 4.2× vs CGQ systems [11]. Distinct from `paper-dbsp-ivm` / `paper-graphflow-delta-generic-join` (recompute *views* incrementally), `pogocache-ttl-kv-cache` (generic TTL KV, not query-shaped).

## 1. Raw idea

Three cache grains. (1) **Whole-query** results keyed by canonical SPARQL/Cypher graph [1][3]. (2) **Subquery / one-hop** result sets reused inside a plan [2][4]. (3) **Cross-query memo** of repeated graph computations under concurrency [11]. Invalidation: writes compute impacted keys (eBay) or pattern-analyse what an update can touch [7]. GRaCe *relaxes* the match to raise hit rate [6] — a different contract, not P0. HTTP-layer SPARQL caches [8] are protocol, not engine.

## 2. STCA applicability

Query 070: a cache entry is a **lease of a compiled query at a log offset** (AS-OF). Time: invalidation is “newer than this event id”, not TTL. Composition: do not put PogoCache in front of Cypher. IVM maintains a *named view* continuously; this card memos *ad-hoc* MATCH/SPARQL. Verify: a cache hit must carry the offset it was true at, or AS-OF lies.

## 3. Quality / cost

Usefulness high: repeated legal/citation hops. Optimality med: SIGMOD 2015 + eBay production are real; GNN/SECF prefetch is extra. Cost: P0 = no cache; honeycomb = canonical-query key + event-id invalidation + optional one-hop vertex-id cache; skip relaxed-semantics caches unless a pack.

## 4. Demand

p95 of interactive Cypher will not wait for leapfrog every time. Engine demand: cache keyed by (compiled plan, log offset), fail-closed on write.

## 5. Niche → effect

`no niche`
