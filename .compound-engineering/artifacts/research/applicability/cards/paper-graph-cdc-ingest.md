---
id: paper-graph-cdc-ingest
source: paper
axes: [Time, Data, Composition]
usefulness: high
optimality: med
demand: high
confidence: paper
layer5: "Legal/enterprise: ingest CMS/docket OLTP via log CDC into Kutha events — dual-write is the failure mode; the foreign WAL is not Kutha SoT"
status: closed
channels_failed: []
---

# CDC turns a foreign WAL into events — it is not Kutha’s log and not Raphtory’s in-engine changelog

Papers: [DBLog](https://consensus.app/papers/details/a5098b497ea5575f8193967bfb528214/?utm_source=cursor) (Andreakis et al., 2020) watermarked CDC: interleave PK-range backfill with the source transaction log, no table lock; adopted by Debezium and Flink CDC; Netflix production [9]; [certified virtual cuts](https://consensus.app/papers/details/5c98978a88a953339d2502c2933fff1b/?utm_source=cursor) (Andreakis, 2026) formalizes DBLog as a snapshot-*equivalent* replay certificate, not a physical snapshot [2]; [Neptune CDC](https://consensus.app/papers/details/b2a9ea5f5c595da98f06a3059494e667/?utm_source=cursor) (Bebee et al., 2019) graph store publishes a change stream to Elasticsearch (and other purpose-built systems) [6]; [Incremental KG construction](https://consensus.app/papers/details/af5fa2e1bcb757dda7a49bce9b7a06c6/?utm_source=cursor) (Van Assche et al., 2026) RML + LDES for evolving sources; up to 4.4× faster construction vs full regen [19]. Dual-write / distributed tx called out as the failed pattern [7][13]. Distinct from `paper-raphtory-lazy-temporal-views` (in-engine change log + lazy views), `paper-activegraph-log-is-sot` (Kutha’s own append-only log), `paper-taris-incremental-icm` (incremental *algorithms* on a stream).

## 1. Raw idea

Three CDC nouns. (1) **Ingest**: read someone else’s transaction log (Debezium/DBLog), backfill without locking, stream inserts/updates/deletes as events [9][2]. (2) **Publish**: a graph store emits its own change stream to search/analytics [6]. (3) **KG regen**: map evolving tabular/XML sources incrementally instead of rebuilding the graph [19]. Generic Kafka-to-BigQuery papers are not this card. Dual-write (app writes SQL *and* graph) is the anti-pattern CDC exists to kill.

## 2. STCA applicability

Time/Data: Kutha’s event log remains SoT. CDC is a **pack** that *translates* a foreign WAL (or RML source) into Kutha events with provenance of LSN/offset. Composition: do not make Kafka/Debezium the quantum. Publish-CDC is the inverse pack: fold → changelog for HNSW/search leases (Neptune→ES). Distinct from Raphtory: that *is* a temporal engine over its changelog; here the changelog is *someone else’s*. Schema evolution on the source is a demand cousin of `daily-archive-schema-lifecycle`.

## 3. Quality / cost

Usefulness high: legal CMS and scientific catalogs will not dual-write. Optimality med: DBLog is the serious mechanism; most hits are vendor CDC blogs. Cost: P0 = file ingest (OxiXML); honeycomb = Debezium-class connector that appends events, with virtual-cut certificates for backfill; do not stream-process the fold itself as CDC.

## 4. Demand

Without CDC, every counsel system becomes a dual-write bug. Engine demand: event types `ForeignInsert/Update/Delete` with source LSN, not a second Postgres.

## 5. Niche → effect

Legal/enterprise: ingest CMS/docket OLTP via log CDC into Kutha events — dual-write is the failure mode; the foreign WAL is not Kutha SoT.
