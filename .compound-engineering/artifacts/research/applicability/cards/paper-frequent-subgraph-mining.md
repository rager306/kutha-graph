---
id: paper-frequent-subgraph-mining
source: paper
axes: [Query, Data, Composition]
usefulness: high
optimality: med
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# Frequent subgraph mining discovers patterns — it does not MATCH a given one

Papers: [gSpan](https://consensus.app/papers/details/7c0ae3584e265cf7a389e8445ee59826/?utm_source=cursor) (Yan & Han, 2002) DFS-code lexicographic order; frequent connected subgraphs without candidate generation [1]; FFSM vertical search [12]; cgSpan closed subgraphs [5]; Subdue compression-interesting rather than all-frequent [6]; [GraMi](https://consensus.app/papers/details/41416bb508505f4988380b4343417972/?utm_source=cursor) (Elseidy et al., 2014) FSM in a *single large graph*, min instances to hit support, plus pattern (friend-of-friend) matching [10]; SPMiner neural approx for large motifs [14]; TipTap streaming k-vertex frequent [19]. Distinct from `paper-subgraph-iso-vs-homomorphism` (yes/no embed of a *given* pattern), `paper-keyword-search-graphs` (keywords → connecting tree), `paper-temporal-motifs` (small *timed* patterns), `paper-graph-summarization-quotient` (merge vertices). DP-gSpan and temporal-quadruple FSM stay queued [7][2].

## 1. Raw idea

You do not know the pattern. Mine subgraphs that occur ≥ support [1][10]. Discovery is not query.

## 2. STCA applicability

Query: FSM is a pack that *emits* candidate patterns; MATCH/WCOJ then runs on a chosen one. Data: the frequent-set is a droppable lease of the fold at a log offset. Composition: support vs exponential output is Cui-shaped; closed mining [5] and ranking [8][9] bound the dump. Time: streaming FSM [19] is a window over the log, cousin of RSP. Verify: a support count is not a how-polynomial. LLM does not invent frequent patterns as facts.

## 3. Quality / cost

Usefulness high: “what motifs keep appearing in this docket graph?” Optimality med: subgraph iso is NP; gSpan/GraMi are the classical poles. Cost: P0 = user-supplied MATCH; honeycomb = optional FSM pack with support and closedness. Do not treat a mined pattern as SoT.

## 4. Demand

Exploratory legal/science graphs arrive without a Cypher in mind. Engine demand: mine-then-compile, fail-closed to explicit MATCH.

## 5. Niche → effect

`no niche`
