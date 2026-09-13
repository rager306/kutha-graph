---
id: paper-hypergraph-higher-order
source: paper
axes: [Data, Query, Composition]
usefulness: high
optimality: med
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# N-ary facts need identity-preserving incidence records

Papers: [From Graphs to Hypergraphs](https://consensus.app/papers/details/10348d85263651f4b024ec3f1bc1d8fd/?utm_source=cursor) (Wang et al., 2024) — projecting a hypergraph to a pairwise graph has two irreducible loss patterns; recovering the lost hyperedges without extra signal is combinatorially impossible [14]; [HIF](https://consensus.app/papers/details/71366ba219765a119a22898dd1c38db8/?utm_source=cursor) (Coll et al., 2025) interchange format: undirected/directed hypergraphs and simplicial complexes; attributes on nodes, edges, *incidences* [2]; [high-order hypergraph walks](https://consensus.app/papers/details/00b720d012c85978874f81dd78ac84ec/?utm_source=cursor) (Aksoy et al., 2019) walks have length *and* width; graph metrics miss the structure [3]; [higher-order motifs](https://consensus.app/papers/details/8056af706c5f58dfac0190a50d539a8d/?utm_source=cursor) (Lotito et al., 2021) [4]; ACM survey [19]. Distinct from `paper-ocpm-multi-object-events` (one *event* related to many objects is a process log encoding) and from RDF triples / property *binary* edges.

## 1. Raw idea

A hyperedge groups participants in one interaction. A clique projection without hyperedge identity loses that grouping. An incidence star can preserve it: retain a distinct hyperedge node and its participant records. The [HIF specification](https://github.com/HIF-org/HIF-standard#structure) requires node and edge identifiers per incidence and allows direction and attributes. HIF also provides explicit records for isolated nodes and empty edges.

For an n-ary tuple, preserve roles or positions and every repeated participation; a plain membership set cannot encode those distinctions. Reconstruction is then direct: group incidences by hyperedge id and recover their roles, positions, and multiplicity. This losslessness claim is an encoding argument under those conditions, not a claim that HIF alone enforces every tuple constraint. Hypergraph walks and motifs still need semantics beyond VF2 on a clique projection [3][4].

## 2. STCA applicability

Data: a proposed n-ary encoding must distinguish the assertion event, claim/hyperedge identity, and participant incidences. OCPM motivates multi-object events; it does not prove the current Kutha event schema implements them. Composition: an incidence lease is rebuildable when the authoritative encoding retains the necessary records. Query 070 must join participants through the same hyperedge identity; adjacency in a lossy clique projection is insufficient evidence of one interaction. No hypergraph-native SoT is required.

## 3. Quality / cost

Usefulness high: legal multi-party acts and scientific co-authorship are n-ary. Optimality med: an incidence encoding must preserve identity, roles, and multiplicity through queries and retractions. Current P0 offers binary triples and `leapfrog_intersect`; this card does not claim a set-valued participant property or full LFTJ matcher exists. A future encoding/lease belongs to the existing honeycomb; HIF is an interchange, not storage [2].

## 4. Demand

Two filings with the same participants must remain distinct, and retracting one must preserve the other. A round trip through incidences must retain participant roles and repeated occurrences. Engine demand: an identity-preserving n-ary contract before relying on such queries or explanations.

## 5. Niche → effect

`no niche`
