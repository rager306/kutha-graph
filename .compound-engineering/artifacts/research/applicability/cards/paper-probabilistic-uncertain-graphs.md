---
id: paper-probabilistic-uncertain-graphs
source: paper
axes: [Data, Query, Verify]
usefulness: high
optimality: med
demand: high
confidence: paper
layer5: "Legal/science: extracted facts and noisy links as possible worlds — probability is not DP noise and not TOKI invalidation"
status: closed
channels_failed: []
---

# Edge probability is a possible-world lease — not DP and not contradiction

Papers: [Khan & Tu](https://consensus.app/papers/details/aef7aaf8136c5bb98d8b33740628589c/?utm_source=cursor) (2015, VLDB tutorial) uncertain graphs: noisy measurements, inference, privacy manipulation; classical reachability/shortest-path become #P-complete under possible worlds [12][20]; [ProbTree](https://consensus.app/papers/details/356794e882c755c499b034f94f499f24/?utm_source=cursor) (Maniu et al., 2017, TODS) indexes a *succinct* set of worlds so ST-queries (shortest path) sample less and smaller [1][8]; subgraph search over uncertain graphs is #P-complete; filter (PIndex) then verify [2]; Monte-Carlo with partial-graph sampling and significance tests [3]; representative *deterministic* instance preserving expected degrees approximates clustering/shortest-path [19]; trigger-graph compilation for probabilistic DBs avoids materializing full lineage [15]; Banerjee survey of mining uncertain graphs [14]. Distinct from `paper-graph-differential-privacy` (query-time noise, not edge existence), `paper-toki-contradiction-ops` (typed invalidation of a fact, not p(e)), `paper-hypergraph-higher-order` (n-ary structure, not probability), `ultra-kg-foundation-reasoner` (neural scores, not possible worlds). GPU path sampling and probabilistic hypergraphs stay queued.

## 1. Raw idea

An edge (or fact) exists with probability `p`. Semantics = **possible worlds**: each world is a certain graph; the query answer is a distribution [1][12]. Exact evaluation is #P-hard, so you sample, index worlds (ProbTree), or emit a representative certain graph [19]. Confidence is a *annotation on the fold*, not a second truth.

## 2. STCA applicability

Data: `p` is a property of an event or a derived edge lease — the log still records *what was asserted*, with an optional confidence payload. Query: probabilistic MATCH is a planner choice (sample / bound / representative world), not leapfrog. Verify: explanations of *why this probability* (causality/blame on uncertain attributes) [18] are receipts over worlds, not quantum receipts of the log. Time: do not treat `p` as valid-time. Security: privacy-as-uncertainty [12] is not DP Laplace on counts. LLM does not invent `p`.

## 3. Quality / cost

Usefulness high: extraction and sensors are noisy. Optimality med: sampling and ProbTree are real; exact possible-worlds is intractable. Cost: P0 = optional `confidence` property, queries ignore it; honeycomb = possible-world sampling pack or a representative-instance lease. Do not make a probabilistic DB the SoT.

## 4. Demand

A docket hop extracted at 0.6 should not MATCH as certain. Engine demand: annotate and *optionally* query under possible worlds; fail-closed defaults to “treat as certain only if p = 1.”

## 5. Niche → effect

Legal/science: extracted facts and noisy links as possible worlds — probability is not DP noise and not TOKI invalidation
