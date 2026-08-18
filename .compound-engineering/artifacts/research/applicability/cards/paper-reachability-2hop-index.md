---
id: paper-reachability-2hop-index
source: paper
axes: [Query, Data, Composition]
usefulness: high
optimality: high
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# Reachability is a labeled hop-index lease — not RPQ and not a journey

Papers: [Cohen et al.](https://consensus.app/papers/details/700007f77a5655529d0cf5461ac2b723/?utm_source=cursor) (2002) 2-hop covers: labels `Lout`/`Lin` so `u` reaches `v` iff the labels intersect; often smaller than explicit transitive closure [4]; [GRAIL](https://consensus.app/papers/details/c1538e3b9bbb510e88755960a8fba77d/?utm_source=cursor) (Yildirim et al., 2011/2010) randomized interval labeling, linear index, scales to millions [5][13]; [FERRARI](https://consensus.app/papers/details/591b040aff675c05afe54ad961fcd1d1/?utm_source=cursor) (Seufert et al., 2013) bounded-size approximate ranges + recursive fallback [9]; Hierarchical/Distribution labeling without materializing TC [1]; **label-constrained** 2-hop answers LCR in microseconds on billion-edge graphs, worst-case bound by index-entry size [2][3]; RLC index for Kleene-plus label concatenation (a Cypher `*` cousin with a *reachability* index, not an automaton interpreter) [7]; DLCR maintains 2-hop under edge updates [10]; SIGMOD tutorial: thirty years of compressing TC into GDBMSs [16]. Distinct from `paper-regular-path-queries` (language + automaton × graph), `paper-tvg-journeys-restless` (time-respecting walks), `paper-subgraph-iso-vs-homomorphism` (pattern MATCH), `paper-k2tree-succinct-graph` (adjacency compression, not TC). Temporal-bipartite reachability stays queued.

## 1. Raw idea

`MATCH (a)-[*]->(b)` as a boolean (or hop-bounded) **can I get there?** is not WCOJ and not “run BFS every time.” Store per-vertex labels so the answer is an intersection or an interval test [4][5]. Edge labels become a constraint set on the same index (LCR) [2]. The structure is a **droppable lease**: rebuild from the fold when the log moves [10].

## 2. STCA applicability

Query: boolean/k-hop reachability is an access path the planner may pick instead of expanding `*`. Data: 2-hop/interval labels are a materialization plugin (040), reversible, not SoT. Composition: FERRARI’s space budget is a Cui-shaped tradeoff (index size vs probes) [9]. Time: untimed reachability; as-of means rebuild or version the labels from the log, do not put clocks inside GRAIL. Agent: LLM does not search the DAG. Verify: a miss is a why-not on the compiled hop test.

## 3. Quality / cost

Usefulness high: every Cypher `*` that is only “exists a path.” Optimality high: microsecond LCR vs BFS [2]; GRAIL is the scalable interval pole [5]. Cost: P0 = online BFS/DFS with hop cap; honeycomb = GRAIL/2-hop lease + LCR when relationship types matter. Do not materialize full TC. GPU path-sampling stays skipped.

## 4. Demand

Counsel asks “is this entity reachable from that statute node under {cites, amends}?” Engine demand: a labeled reachability lease, not a full RPQ engine on every boolean.

## 5. Niche → effect

`no niche`
