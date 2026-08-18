---
id: paper-obda-ontology-compile
source: paper
axes: [Query, Space, Agent]
usefulness: high
optimality: high
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# Compile the ontology into SPARQL/SQL — virtual OBDA, not a second triple store

Papers: [Ontop](https://consensus.app/papers/details/2ea01ad7a4ba59729ec8b9430f9c307a/?utm_source=cursor) (Calvanese et al., 2016, Semantic Web, DOI: 10.3233/sw-160217) SPARQL over RDB via OWL2QL + R2RML, virtual rewrite, no materialized triples [1]; [Quest](https://consensus.app/papers/details/64dfcb1097365c028d7733bef585e820/?utm_source=cursor) same family [2]; [UltrawrapOBDA](https://consensus.app/papers/details/2cb204a574ee5610820c0a55a5d8ea2c/?utm_source=cursor) (Sequeda et al., 2014) rewrite *and* selective materialization; SQL recursion for transitivity [5]; [PerfectMap](https://consensus.app/papers/details/c1fc9c496bba5de6bb891483bfd4f9e2/?utm_source=cursor) mapping-inclusion optimizations [6]; [Statoil OBDA](https://consensus.app/papers/details/16b163e04f7e5a579322733dd051934d/?utm_source=cursor) (Kharlamov et al., 2017) industrial mappings [12]; [schema-agnostic SPARQL 1.1 rewriting](https://consensus.app/papers/details/1fbc7eeb15305d26b46ba37dbb87a949/?utm_source=cursor) ontology stored *with* data [18]. Distinct from `paper-az-projected-schema-cypher` (projected MATCH, not OWL2QL) and `paper-semantic-layer-smq` (metrics IR → SQL).

## 1. Raw idea

OBDA: user query on a TBox; mappings (R2RML) connect classes/properties to SQL; the engine **rewrites** SPARQL (+ OWL2QL entailments) into SQL against the existing DB. Virtual = no ETL triples [1]. Hybrid = compile some views, leave the rest virtual [5]. SWRL/RIF linear recursion can ride along into recursive SQL [11]. This *is* ontology compilation: TBox + mapping → query plan, not “run a reasoner then dump n-triples.”

## 2. STCA applicability

Query/Space: Kutha dictionaries compile to **Cypher/WCOJ plans** the same way Ontop compiles to SQL. RDF/SPARQL is an optional **export dialect** over a fold, not the SoT (event log stays). Agent: NL→SPARQL still needs the compiler-not-executor fence (AZ). Do not materialize a second OWL ABox as truth. Mappings are pack artifacts (versioned with the ontology, τOWL cousin).

## 3. Quality / cost

Usefulness high: the only scalable “ontology as API” story. Optimality high for OWL2QL fragment; full OWL is approximation/rewrite [19]. Cost: borrow the *architecture* (virtual rewrite + optional views); implement Cypher-side, not a mandatory SPARQL endpoint in P0.

## 4. Demand

Enterprises already have tables; they will not reload into Jena. Engine demand: dict→plan compile. SPARQL pack later for RDF partners.

## 5. Niche → effect

`no niche`
