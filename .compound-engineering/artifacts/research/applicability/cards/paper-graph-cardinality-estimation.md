---
id: paper-graph-cardinality-estimation
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

# Cardinality is the planner’s statistic — not the join algorithm and not a temporal sketch

Papers: [Join Cardinality Estimation with OmniSketches](https://consensus.app/papers/details/fac32d6bdda456b3bf7d9e4bd95f27f4/?utm_source=cursor) (Justen et al., 2025) count-min + K-minwise; multi-join without uniformity/independence; SSB-skew up to 1077× fewer intermediates vs DuckDB; JOB-light often loses witnesses on bushy FK graphs [1]; [cardinality estimation graphs](https://consensus.app/papers/details/56f57f7f60645d4ea81b4683ce5ef252/?utm_source=cursor) (Salihoglu et al., 2026, CACM) optimistic (uniformity) vs pessimistic (AGM/MOLP) estimators are paths in one CEG [6][7]; [COLOR](https://consensus.app/papers/details/ba68ec6397c8582aa77c4dd1667d5f61/?utm_source=cursor) (Deeds et al., 2024, VLDB) graph-coloring summaries; up to 10³× vs competitors [4]; [WanderJoin-style sampling for complex graph queries](https://consensus.app/papers/details/083b515071f25f1e9d167582478283fe/?utm_source=cursor) (Hu et al., 2024, TODS) nested operators (OR, difference); strongly consistent; zero-estimate trap on skew [3]; [JOB](https://consensus.app/papers/details/bc7c711b2aa35c39a94a532274d6a42b/?utm_source=cursor) (Leis et al., 2017, VLDBJ) industrial estimators routinely miss join-crossing correlation; *cardinality* dominates cost-model errors [18]. Distinct from `paper-adopt-adaptive-wcoj-orders` (RL *order*, assumes a cost), `paper-horae-temporal-sketches` (approximate *temporal range* on streams), `paper-lftj-wcoj` (execute the join). Closes queued OmniSketch skip-unless as a planner spike.

## 1. Raw idea

WCOJ bounds *worst-case* work; the planner still needs |Q| for *this* instance. Optimistic estimators walk a CEG of average degrees and often underestimate; pessimistic LP/AGM overestimate but have combinatorial CEG forms [6]. OmniSketch interpolates without independence [1]. Sampling (WanderJoin/FaSTest) is accurate until skew yields zeros [3][12]. Learned CardEst (GNN/transformers) is demand, not P0 — JOB already shows traditional histograms die on many-to-many graph joins [18][2].

## 2. STCA applicability

Query 043/070: cardinality is a **statistic lease** on the fold (sketches / CEG / color summary), rebuilt like CSR. Leapfrog still runs; the estimator only picks variable order and whether to materialize. Composition: AGM pessimistic bound *is* the WCOJ theory — use it as a ceiling, OmniSketch/COLOR as the typical-case. Do not train a neural estimator as SoT. Horae sketches answer approximate *temporal counts*; this card answers *join size* for planning.

## 3. Quality / cost

Usefulness high: bad |Q| is why Cypher plans explode. Optimality high: CEG unifies the literature; COLOR/OmniSketch are concrete. Cost: P0 = AGM/degree-sequence pessimistic + optional OmniSketch on hot labels; skip GNN CardEst until JOB-on-Kutha exists.

## 4. Demand

Legal multi-hop AS-OF and scientific motif counts both need an order. Engine demand: inject estimates into the leapfrog variable permutation, not a second optimizer product.

## 5. Niche → effect

`no niche`
