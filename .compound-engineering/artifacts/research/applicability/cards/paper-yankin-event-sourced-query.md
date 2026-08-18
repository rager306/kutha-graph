---
id: paper-yankin-event-sourced-query
source: paper
axes: [Time, Query, Verify]
usefulness: high
optimality: high
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# Event-sourced query mechanisms and portable cost envelopes

Paper: [Foundational Abstractions for Core Entities and Query Mechanisms in Event-Sourced Systems](https://doi.org/10.31891/csit-2026-2-19) (Yankin et al., 2026). Consensus: https://consensus.app/papers/details/2a49227472315230b60c1e6ff873e091/?utm_source=cursor

## 1. Raw idea

Queries over event-sourced state are not “read the snapshot.” Four mechanism groups — reconstruction, temporal, cross-stream, retroactive replay — each with scope/cut rules, ordering/merge, correlation, and version-normalization. Cost envelopes in selected evidence size + amortized per-event work, including snapshot-shortened replay.

## 2. STCA applicability

Time/Query: names the Kutha query taxonomy over the log (D1/D2) without making a fold the SoT. Verify: comparable costs across implementations. Complements ActiveGraph (“log is the fold”) with *how expensive* each as-of / replay cut is.

## 3. Quality / cost

High conceptual fit; theoretical experiment on a synthesized banking log, not a graph engine. Cost: still need CSR/WCOJ on the chosen cut; distributed time uncertainty left as future work.

## 4. Demand

Without named mechanisms, “AS OF” and “replay from event k” are marketing synonyms. Planner and leases need envelopes, not vibes.

## 5. Niche → effect

`no niche`
