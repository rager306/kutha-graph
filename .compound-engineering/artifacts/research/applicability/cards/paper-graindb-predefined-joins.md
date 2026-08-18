---
id: paper-graindb-predefined-joins
source: paper
axes: [Query, Data]
usefulness: med
optimality: med
demand: med
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# GRainDB: predefined graph joins inside DuckDB via RIDs and SIP

Paper: [Making RDBMSs Efficient on Graph Workloads Through Predefined Joins](https://doi.org/10.14778/3510397.3510400) (Jin et al., 2021). Consensus: https://consensus.app/papers/details/a67f254d3124563fb98f372a4c06f385/?utm_source=cursor

## 1. Raw idea

Native GDBMS joins are pointer-like (adjacency). GRainDB materializes joining RIDs and uses hash joins + sideways information passing so scans stay sequential — more robust than INLJ-only graph processors inside an RDBMS. Competitive with a SOTA GDBMS on large m-n joins without always paying a graph engine.

## 2. STCA applicability

Contrast: putting the Kutha graph *into* DuckDB as predefined joins revives “RDBMS as SoT.” Useful lesson: sequential scans + SIP beat naive pointer chasing; CSR leases already encode predefined joins. Do not adopt DuckDB as the graph core.

## 3. Quality / cost

Honest RDBMS-vs-GDBMS bake-off. Cost: value-based tables are not an event log; AS-OF/WCOJ/HNSW still sit elsewhere.

## 4. Demand

People will ask “why not Postgres/DuckDB + edges table?” This card is the answer: predefined joins help hops, they do not replace log-as-SoT.

## 5. Niche → effect

`no niche`
