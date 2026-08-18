---
id: paper-graph-edit-distance
source: paper
axes: [Query, Data, Verify]
usefulness: high
optimality: med
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# GED is the distance between two graphs — not iso-yes/no and not a tuple join

Papers: GED = min cost of node/edge insert/delete/substitute to turn G into H; NP-hard [5]; A* with tighter partial-mapping bounds (AStar-BMao) for *verification* under a threshold [2]; ILP FORI solves IAM instances to optimality [6]; FGWAlign OT/Fused Gromov-Wasserstein as a GED-equivalent alignment [14]; filter-and-verification similarity search [15][17]. GNN regressors (SimGNN, GraphSim, GEDGNN) predict a score, often *without* an edit path [16][1][11] — useful as a pack, not as SoT. Distinct from `paper-subgraph-iso-vs-homomorphism` (does H embed in G?), `paper-wcoj-similarity-joins` (similar *tuples* inside one query), `ultra-kg-foundation-reasoner` (scored missing links), `paper-graph-sampling-aqp` (one graph, smaller sample). LLM-generated GED programs [9] stay queued (LLM not the metric).

## 1. Raw idea

Two folds (or two named-graph slices) differ. **How much** work to turn one into the other [5]? The answer is a number *and*, if honest, an edit path [1].

## 2. STCA applicability

Query: GED is an access path “compare two compiled views,” not MATCH. Data: both operands are leases of the log at offsets (or named graphs). Verify: an edit path is a *diff* cousin of fork-and-diff — explain *what* changed, not why a MATCH failed. Time: compare AS-OF t1 vs t2 of the same identity. LLM does not invent the distance.

## 3. Quality / cost

Usefulness high: near-duplicate matters, ontology drift, “did this fold change.” Optimality med: exact NP-hard; A*/ILP for small graphs; GNN scores lack paths. Cost: P0 = iso or hash equality; honeycomb = threshold GED search. Do not replace Cypher with a similarity score.

## 4. Demand

“Show me graphs within edit-distance k of this pattern” is a library query. Engine demand: threshold verification + optional path, fail-closed to “too large / refuse.”

## 5. Niche → effect

`no niche`
