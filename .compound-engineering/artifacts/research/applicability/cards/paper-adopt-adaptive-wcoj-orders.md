---
id: paper-adopt-adaptive-wcoj-orders
source: paper
axes: [Query]
usefulness: high
optimality: med
demand: med
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# ADOPT: RL episodes over WCOJ attribute orders

Paper: [ADOPT](https://doi.org/10.14778/3611479.3611489) (Wang et al., 2023). Consensus: https://consensus.app/papers/details/04982d52565d5982a1b7c1e23a683239/?utm_source=cursor

Related: [SkinnerDB](https://doi.org/10.1145/3464389) — regret-bounded join-order RL *during the current query*, no stats. Consensus: https://consensus.app/papers/details/21d6733eb5615adc808016f93c41a683/?utm_source=cursor

Caveat: simple adaptive processing (join-algorithm switch + LIP) can match or beat RL QOs without training [Zhang et al., VLDBJ](https://consensus.app/papers/details/c6885b3df3c25d1094aa10947f6ab281/?utm_source=cursor).

## 1. Raw idea

WCOJ cost is dominated by *attribute* order, not relation join order. Estimates fail under skew. ADOPT tries orders in episodes, tracks processed input so work is not repeated, picks the next order with RL (explore vs exploit). Wins when queries are too complex for static optimizers.

## 2. STCA applicability

Query: runtime adaptation for Samyama leapfrog / Kuzu WCOJ when AS-OF cuts change cardinalities. Complements Graphflow intersection-cost (static/adaptive vertex order) with *episode RL*. Event log stays SoT; orders are a plan, not a stored graph.

## 3. Quality / cost

Useful when static cost models lie. Cost: not P0 — RL in the hot path; Zhang 2023 says dumb AQP may suffice. Do not train a join-order net as Kutha identity.

## 4. Demand

Legal/science BGPs with skew: a wrong leapfrog variable order is a latency cliff.

## 5. Niche → effect

`no niche`
