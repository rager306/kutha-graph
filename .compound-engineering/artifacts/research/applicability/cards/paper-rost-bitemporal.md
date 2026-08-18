---
id: paper-rost-bitemporal
source: paper
axes: [Time, Query]
usefulness: high
optimality: med
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# Bitemporal property graphs + continuous event detection (Rost)

Paper: [Bitemporal Property Graphs to Organize Evolving Systems](https://arxiv.org/abs/2111.13499) (Rost et al., 2021). Consensus: https://consensus.app/papers/details/3b766971c8965f96b2f44e913df67685/?utm_source=cursor

## 1. Raw idea

VT×TT property graph plus a temporal query language and continuous event detection over evolving relationships (Oracle–Leipzig IoT/time-series cooperation).

## 2. STCA applicability

Time: same axes as TGMS/T-GQL, with *event detection* as a behavior over the fold. Query: temporal QL. Event detection is a pack, not SoT.

## 3. Quality / cost

Clear model; prototype DB. Cost: detecting events in the hot store vs emitting them as log events (Kutha prefers the latter).

## 4. Demand

“Tell me when the graph’s belief about X changed” without a GNN forecast pack.

## 5. Niche → effect

`no niche`
