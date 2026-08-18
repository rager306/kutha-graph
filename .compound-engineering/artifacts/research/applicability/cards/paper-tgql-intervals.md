---
id: paper-tgql-intervals
source: paper
axes: [Time, Query]
usefulness: high
optimality: med
demand: high
confidence: paper
layer5: "Legal PIT paths: friends-of-friends who co-located in an interval"
status: closed
channels_failed: []
---

# Validity-interval property graphs and temporal paths (T-GQL)

Paper: [T-GQL](https://doi.org/10.1007/s00778-021-00675-4) (Debrouvier et al., 2021). Consensus: https://consensus.app/papers/details/b1169135782753e1b9bfd3af55260be6/?utm_source=cursor

## 1. Raw idea

Nodes/edges carry attributes timestamped with validity intervals. Query language T-GQL plus algorithms for temporal path semantics (not just snapshot hops).

## 2. STCA applicability

Time+Query: AS OF and “when did this path exist.” Complements TGMS operators and temporal WCOJ BGP. Implementation on Neo4j is a *host*, not Kutha SoT.

## 3. Quality / cost

Mature VLDB model. Cost: interval semantics vs event-log fold; path algorithms can be expensive without CSR/WCOJ.

## 4. Demand

Legal/compliance questions are interval questions (“who could reach X while Y was in force”).

## 5. Niche → effect

Legal PIT paths: interval-respecting multi-hop, not latest-state RAG.
