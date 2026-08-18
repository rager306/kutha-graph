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

# An n-ary fact is a hyperedge — clique projection is a lie

Papers: [From Graphs to Hypergraphs](https://consensus.app/papers/details/10348d85263651f4b024ec3f1bc1d8fd/?utm_source=cursor) (Wang et al., 2024) — projecting a hypergraph to a pairwise graph has two irreducible loss patterns; recovering the lost hyperedges without extra signal is combinatorially impossible [14]; [HIF](https://consensus.app/papers/details/71366ba219765a119a22898dd1c38db8/?utm_source=cursor) (Coll et al., 2025) interchange format: undirected/directed hypergraphs and simplicial complexes; attributes on nodes, edges, *incidences* [2]; [high-order hypergraph walks](https://consensus.app/papers/details/00b720d012c85978874f81dd78ac84ec/?utm_source=cursor) (Aksoy et al., 2019) walks have length *and* width; graph metrics miss the structure [3]; [higher-order motifs](https://consensus.app/papers/details/8056af706c5f58dfac0190a50d539a8d/?utm_source=cursor) (Lotito et al., 2021) [4]; ACM survey [19]. Distinct from `paper-ocpm-multi-object-events` (one *event* related to many objects is a process log encoding) and from RDF triples / property *binary* edges.

## 1. Raw idea

A hyperedge is a set (or tuple) of vertices: committee, reaction, co-author list, statute-citing-three-cases. Pairwise graphs force a clique or a star; both destroy “this *one* interaction involved these *k* nodes” [14][3]. Incidence is a first-class record (node–hyperedge), not an inferred path [2]. Motifs and walks on hypergraphs are not VF2 on the clique projection [4]. Generative / GNN hypergraph papers are demand, not a second engine.

## 2. STCA applicability

Data: the event log already *can* record an n-ary fact (one event, many participants) — that is OCPM’s *process* noun. This card is the **query/data-model** noun: MATCH a hyperedge without exploding it into binary edges. Composition: a hypergraph CSR/incidence lease is reversible; clique-projected CSR is not (information-theoretically) [14]. Query 070: Cypher binary patterns on a projection will invent paths that were never a fact. Do not replace the property-graph hop engine with a hypergraph-native SoT.

## 3. Quality / cost

Usefulness high: legal multi-party acts and scientific co-authorship are n-ary. Optimality med: hypergraph WCOJ is not in this batch; pairwise LFTJ stays P0. Cost: P0 = binary edges + optional “set-valued participant” property; honeycomb = incidence lease + hyperedge id on events; HIF is an interchange, not storage [2].

## 4. Demand

Flattening a 5-party filing into ten pairwise edges makes why-not provenance lie. Engine demand: n-ary fact type in the dictionary, not a new database.

## 5. Niche → effect

`no niche`
