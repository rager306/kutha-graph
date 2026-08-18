---
id: paper-named-graphs-rdf-dataset
source: paper
axes: [Space, Data, Query, Composition]
usefulness: high
optimality: med
demand: high
confidence: paper
layer5: "Legal/science packs: statute vs commentary vs docket (or paper vs review vs evidence) as named graphs in one fold — not three SoTs"
status: closed
channels_failed: []
---

# Named graphs are a Space noun — quads, not a second log and not tenant quotas

Papers: [Named graphs](https://consensus.app/papers/details/4857059311aa5ac6a7a1ff36166dddac/?utm_source=cursor) (Carroll et al., 2005, J. Web Semant., DOI: 10.1016/j.websem.2005.09.001) URI-named RDF graphs; SPARQL as the query language; provenance/trust layer [1]; WWW companion [Named graphs, provenance and trust](https://consensus.app/papers/details/f1d4aa68c822570fb1cb85ccdf8d9e38/?utm_source=cursor) (DOI: 10.1145/1060745.1060835) publishers sign graphs; consumers accept per task [10]; [N3 multiple dataset semantics](https://consensus.app/papers/details/038fdf2a4af451cb810c92816ffdad57/?utm_source=cursor) (Arndt et al., 2019) — there is **no** unique RDF-dataset semantics; authors must name which [2]; [GRAPH clause in federated SPARQL](https://consensus.app/papers/details/844ed19240e257a7b7577d35f920375a/?utm_source=cursor) (Chaves-Fraga et al., 2017) — GRAPH as a search-space prune is only 5–10% and named graphs are semantically ambiguous [7]; [RDF→PG mappings](https://consensus.app/papers/details/20782504baf05b6ebefb9f5360aa0ef5/?utm_source=cursor) (Angles et al., 2020, IEEE Access, DOI: 10.1109/access.2020.2993117) two mappings are information-preserving; PG subsumes RDF capacity [9]; [RDF-star + Named Graphs](https://consensus.app/papers/details/69bb3a6648c2544c999cd98f575ddbcf/?utm_source=cursor) (Rupp et al., 2022) statement-level vs graph-level meta [11]; SHACL-DS [4] is the *validation* cousin already closed as `paper-shacl-sparql-compile`. Distinct from `paper-graph-tenant-isolation` (quotas / noisy-neighbor / vector silo) and `paper-iso-gql-gpml` (query surface, not the quad noun).

## 1. Raw idea

An RDF dataset is a **default graph plus zero or more named graphs** (quads: s,p,o,g). Named graphs let you talk *about* a graph (sign it, provenance it, version it) without RDF reification [1][10]. Implementations never agreed on dataset semantics (union vs isolated vs graph-as-resource) [2]. SPARQL `GRAPH` is both a scope and a federation hint; using it as a named-graph prune barely helps and the IRI is ambiguous [7]. Property graphs can *encode* RDF without loss [9]; RDF-star is the *edge* meta-level, named graphs the *bundle* meta-level [11]. Anatomy FAIR stores already shard observations into named-graph instances [16] — a pack pattern, not a new SoT.

## 2. STCA applicability

Space 020/080: a named graph is a **slice id on the fold** (pack + dictionary), the same event log. Query 070: Cypher/GQL needs an explicit `GRAPH`/`USE` equivalent so compiled patterns do not silently union every pack. Composition: default-graph vs named-graph is a compile-time scope, not a second WAL. Do not treat SHACL-DS as this card — that compiles *shapes across* datasets; this names the **dataset**. Tenant isolation answers “does this tenant starve another?”; named graphs answer “which triples are even in scope?”

## 3. Quality / cost

Usefulness high: packs already *are* named graphs; without the noun they leak into a global CSR. Optimality med: SPARQL engines vary; GRAPH is not a magic index [7][8]. Cost: P0 = one default graph; honeycomb = graph IRI on events + lease rebuild per named graph; do not stand up Jena/Virtuoso as SoT. RDF→PG mapping is export, not ingest-as-truth [9].

## 4. Demand

Legal and scientific packs share vertices (statute cited by paper cited by docket). Engine demand: quad on the event, `GRAPH` in the compiler, union only when the query asks.

## 5. Niche → effect

Legal/science packs: statute vs commentary vs docket (or paper vs review vs evidence) as named graphs in one fold — not three SoTs.
