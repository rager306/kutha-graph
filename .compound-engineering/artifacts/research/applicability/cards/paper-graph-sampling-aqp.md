---
id: paper-graph-sampling-aqp
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

# A sample is a representative subgraph lease — not a sparsifier and not CardEst

Papers: [Leskovec & Faloutsos](https://consensus.app/papers/details/db5a297c52ee55fc85c2bc514d6bcb1e/?utm_source=cursor) (2006, KDD) forest fire and random-walk beat uniform edges; ~15% samples match static *and* evolutionary patterns; scale-up laws for diameter etc. [1]; Rank Degree / deterministic exploration preserves properties better than Forest Fire on some graphs [2][8]; [KG aggregate AQP](https://consensus.app/papers/details/ca846b060e3a585081b3551be0f64322/?utm_source=cursor) (Wang et al., 2022, ICDE) semantic-aware random walk + unbiased COUNT/SUM (consistent AVG) with confidence intervals — approximate aggregates *without* full factoid MATCH [6]. Distinct from `paper-spectral-sparsification` (keep all vertices, reweight edges for Laplacian/cuts), `paper-graph-summarization-quotient` (merge vertices), `paper-graph-cardinality-estimation` (planner statistic for one query; WanderJoin-style CardEst [3] stays a cousin of that card), `paper-horae-temporal-sketches` (stream sketches). GPU random-walk engines stay queued.

## 1. Raw idea

The fold is too big to measure. Draw a **smaller graph** (or a walk sample) that preserves the property you care about [1], or estimate COUNT/SUM from a walk with an interval [6]. The sample is not the graph.

## 2. STCA applicability

Query: AQP is a planner choice — exact MATCH vs sample+bound. Data: the sample is a droppable lease of a subgraph (or of walk traces), rebuilt from the log. Composition: sample size vs error is Cui-shaped. Time: Leskovec’s evolutionary match means resampling as the log grows; do not freeze a sample as SoT. Verify: confidence intervals [6] are numeric receipts of approximation, not how-polynomials. LLM does not pick seeds as truth.

## 3. Quality / cost

Usefulness high: dashboards and “how big is this pattern.” Optimality med: forest fire/RW are classic; KG AQP is the query noun. Cost: P0 = exact scan; honeycomb = optional sample lease + interval. Do not answer legal MATCH from a 15% fire sample. CardEst sampling stays on the planner card.

## 4. Demand

“About how many matters share this motif?” should not wait for exact WCOJ. Engine demand: sample-and-bound as an access path, fail-closed to exact.

## 5. Niche → effect

`no niche`
