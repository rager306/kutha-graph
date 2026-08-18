---
id: paper-rdf-term-dictionary
source: paper
axes: [Data, Query, Composition]
usefulness: high
optimality: high
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# The hot path runs on interned IDs — the dictionary is a reversible map, not the graph

Papers: [Efficient RDF dictionaries with B+ trees](https://consensus.app/papers/details/3539cac6702a539780ec225dc66ff3af/?utm_source=cursor) (Singh et al., 2018) ensemble of B+ trees + hashed keys; 126M DBPedia labels; dictionaries are the neglected SPARQL *end-to-end* component [1]; [compact in-memory Trie dictionary](https://consensus.app/papers/details/dfef93cd34e053b89e82bdc726d7aefe/?utm_source=cursor) (Bazoobandi et al., 2015) prefix sharing; numeric ID = memory address so decode needs no second structure; 50–59% less RAM than uncompressed; *dynamic/streaming* not static-only [5]; [Dcomp](https://consensus.app/papers/details/ffe7faba3b33550e8fb3e5cc64d45a19/?utm_source=cursor) (Martínez-Prieto et al., 2012) string-dictionary compression 22–64% space, 1–50μs ops [11]; [LiteMat](https://consensus.app/papers/details/e40d33f0a9375425b7d08b506b5f5dd7/?utm_source=cursor) (Curé et al., 2015) *structured* IDs encode RDFS `subClassOf`/`subPropertyOf` so entailment is bit-prefix, not rewrite/materialize [4][12]. Distinct from `paper-ontology-temporal-versioning` (vocab *evolution*), `paper-obda-ontology-compile` (mapping compile), `paper-named-graphs-rdf-dataset` (quad *scope*), `oxixml-xml-rdf-stack` (`oxixml-model` string intern is a *code cousin* of this noun), `helix-indexes-as-access-paths` (property indexes, not term IDs).

## 1. Raw idea

Every RDF/property-graph engine replaces IRIs/literals with integers before joins [1][20]. The dictionary is two maps: string→id (query ingest) and id→string (result emit). Tries win on shared prefixes; B+ ensembles keep height low; Dcomp compresses the maps themselves [5][11]. LiteMat goes further: the ID *is* a hierarchy code so `?x a :Person` includes subtypes without expanding the BGP [4]. Word-embedding “dictionaries” and bilingual lexicons are not this card [9][15].

## 2. STCA applicability

Honeycomb 050 + Data: **dictionary-first** is already ADR language — this card names the *engine* map. Composition: IDs live in CSR/HNSW/leases; strings live in a reversible dictionary (like a pack). Time: dictionary entries can be dated (IRI renamed) without rewriting the log’s integer payload if the map is bi-temporal — cousin of ontology versioning, not a merge. Query: SPARQL/Cypher parse hits the dictionary *before* leapfrog; LiteMat-style prefix IDs are an optional inference lease, not OWL materialization. `oxixml-model` intern is the ingest-library instance of this noun.

## 3. Quality / cost

Usefulness high: without intern, WCOJ thrashes on strings. Optimality high: 50%+ RAM vs naive maps [5]; LiteMat avoids rewrite [4]. Cost: P0 = interned `u64` + append-only string arena; honeycomb = compressed dictionary + optional hierarchy bits; do not store IRIs in CSR.

## 4. Demand

Legal IRIs and scientific DOIs are long; the hop engine must not copy them. Engine demand: dictionary as a first-class reversible materialization, same family as CSR.

## 5. Niche → effect

`no niche`
