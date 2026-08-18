---
id: paper-stkg-property-streams
source: paper
axes: [Time, Data, Query]
usefulness: high
optimality: med
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# A property stream is a time series on an entity — not a journey and not a GNN forecast

Papers: [stRDFS](https://consensus.app/papers/details/28e21a54d6135aca88b5de68c42ff18e/?utm_source=cursor) (Zhu et al., 2020) labels *properties* with spatiotemporal features without changing the RDF triple layout [19]; [SSTKG](https://consensus.app/papers/details/7666a478606757d6943cd291c4f9ff32/?utm_source=cursor) (Yang et al., 2024) a simple STKG for embedding dynamic facts for forecast/recommend [18]; industrial KG + IoT series: attach quality-prediction *semantics* to a KG node, then link-reason [5]. Distinct from `paper-tvg-journeys-restless` (topology changes over time), `paper-temporal-motifs` (small timed patterns), `paper-temporal-interval-index` (validity intervals, not numeric series), `paper-rdf-stream-processing` (standing SPARQL on triple windows), `paper-spatial-geosparql-graphs` (geometry predicates). STGNN forecasters (DynaSTy, DST-WDCN, FourierGNN, STAR, …) stay queued as GNN — they are not MATCH and not SoT [1][2][8][12][17].

## 1. Raw idea

An entity keeps a **sequence of measured values** (sensor, quote, lab reading) while the topology may be slow [19][5]. That is a property stream, not “the graph evolved” and not “forecast next hop.”

## 2. STCA applicability

Time: each sample is an event (or a packed series lease) on the log; valid-time of a *value* is not the valid-time of an edge. Data: the series store is a droppable materialization; the log owns the samples. Query: AS-OF a timestamp on a property, window aggregates — not journey algorithms. Composition: sampling rate vs retention is Cui-shaped. Verify: a forecast score is not a receipt. LLM does not interpolate missing ticks as truth.

## 3. Quality / cost

Usefulness high: dockets-as-counts, instruments, vitals. Optimality med: modeling (stRDFS) is the P0 noun; STGNN is a pack at best. Cost: P0 = events with numeric payloads; honeycomb = optional series lease + window ops. Do not make a GNN the fold.

## 4. Demand

“What was this meter at t, and which asset was it wired to?” is a property stream joined to the graph, not a TVG journey.

## 5. Niche → effect

`no niche`
