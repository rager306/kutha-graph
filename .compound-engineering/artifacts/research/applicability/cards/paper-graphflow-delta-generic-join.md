---
id: paper-graphflow-delta-generic-join
source: paper
axes: [Query, Time, Composition]
usefulness: high
optimality: high
demand: med
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# Graphflow (2017): continuous subgraph queries via Delta Generic Join

Paper: [Graphflow: An Active Graph Database](https://doi.org/10.1145/3035918.3056445) (Kankanamge et al., 2017). Consensus: https://consensus.app/papers/details/d7784a432a42507eb4cda19dc0a1698d/?utm_source=cursor

## 1. Raw idea

Passive GDBMSs only answer one-shot subgraph queries. Graphflow is *active*: Cypher++ with subgraph-condition-action triggers. Generic Join for one-shot WCOJ; **Delta Generic Join** for continuous queries as the graph mutates.

## 2. STCA applicability

Query/Time: IVM of views from log deltas — same family as DBSP/FlowLog, but *subgraph-shaped* and WCOJ. Composition: behaviors fire on subgraph emergence, not on chat. Distinct from 2021 columnar paper (storage) and from ActiveGraph (log-as-SoT essay).

## 3. Quality / cost

High narrative fit for honeycomb “views as leases.” Cost: SIGMOD demo, not a production kernel; Kutha must attach deltas to the event log, not a mutable in-memory graph SoT.

## 4. Demand

“Tell me when this pattern appears” without polling Cypher.

## 5. Niche → effect

`no niche`
