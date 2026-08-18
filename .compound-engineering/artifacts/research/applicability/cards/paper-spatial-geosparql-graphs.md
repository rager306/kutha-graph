---
id: paper-spatial-geosparql-graphs
source: paper
axes: [Space, Query, Data, Time]
usefulness: high
optimality: med
demand: high
confidence: paper
layer5: "Legal/science: parcels, indoor graphs, topographic features as geometry on the fold — spatial is a predicate lease, not named-graph Space and not Geo-Raft"
status: closed
channels_failed: []
---

# Geometry is a predicate lease — not a second GIS SoT

Papers: [GraST](https://consensus.app/papers/details/2a7c6a249877549fa5d2c3251801d919/?utm_source=cursor) (Yue et al., 2025) splits geospatial-temporal semantic queries: lightweight nodes in a property graph, full geometry in a spatial engine, cross-index, 1–2 orders of magnitude [1]; [GeoSPARQL 1.1](https://consensus.app/papers/details/a10a5babbd745ec79b1e3dd7265b391e/?utm_source=cursor) (Car et al., 2022) is the LOD spatial surface (ontology + SPARQL spatial functions + RIF) [12]; [GeoExpand](https://consensus.app/papers/details/789d6edce7d8571a8d7a57d78afe78a5/?utm_source=cursor) (Sun et al., 2019) prunes Neo4j expansion with spatial bitmaps on vertices [13]; [FineGeoKG](https://consensus.app/papers/details/1f046aa035895944aab4f784873c7bee/?utm_source=cursor) (Wei et al., 2024) first-class *strong geospatial relations* (topology + direction) as quantified edges, not just WKT blobs [8]; LinkedGeoData as a **virtual** KG over OSM via Ontop/Sparqlify GeoSPARQL [4]; QSR over YAGO beats purely quantitative GeoSPARQL on large KGs [16]. Distinct from `paper-geo-raft-wan` (log replica, not geography), `paper-federated-sparql-query` (`SERVICE` an endpoint), `paper-named-graphs-rdf-dataset` (Space as graph name), `paper-temporal-interval-index` (time intervals, not DE-9IM). Dual PostGIS+Neo4j with 2PC [1] is **not** Kutha SoT.

## 1. Raw idea

A node can have a geometry. MATCH must filter by topology (within, touches, nearest) and sometimes by time. The expensive work is spatial index + prune of graph expansion [13], not storing shapefiles in the event log. GeoSPARQL is the *language* of those predicates [12]. Strong spatial relations can be facts on the fold [8]. A GIS RDBMS is a **lease** of WKT/GML, like CSR is a lease of adjacency [1][4].

## 2. STCA applicability

Space: geography is a *predicate on the fold*, not tenant Space and not `GRAPH` names. Query: compile GeoSPARQL/DE-9IM into hybrid retrieve (bitmap/R-tree prune, then hop). Time: spatiotemporal objects are valid-time + geometry; do not invent a fourth clock. Data: grid/geohash encodings (GeoSOT cousins) are interned spatial IDs, not a second SoT. Verify: spatial why-not is “this parcel was outside the buffer,” not an LLM apology. LLM stays off the geometry write path.

## 3. Quality / cost

Usefulness high: legal parcels and scientific sites are spatial. Optimality med: prune-on-expand and VKG compile are real; dual-commit graph+PostGIS is the failure mode. Cost: P0 = optional geometry property + bounding-box filter on MATCH; honeycomb = spatial bitmap/R-tree lease rebuilt from the log, GeoSPARQL as a compile dialect. Do not vendor Neo4j+PostGIS as the quantum. IndoorGML/CityGML/BIM stay queued unless a second indoor noun.

## 4. Demand

“Every matter whose site is within this floodplain, AS OF closing” is a spatial MATCH, not glob over shapefiles. Engine demand: geometry as typed property + droppable spatial index.

## 5. Niche → effect

Legal/science: parcels, indoor graphs, topographic features as geometry on the fold — spatial is a predicate lease, not named-graph Space and not Geo-Raft
