---
id: paper-shacl-sparql-compile
source: paper
axes: [Verify, Query, Data]
usefulness: high
optimality: med
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# SHACL compiles to SPARQL (or SAT); recursive shapes are the trap

Papers: [Wikidata constraints as SHACL/SPARQL](https://consensus.app/papers/details/5eee55207fb258db90e18f6f8c094857/?utm_source=cursor) (Ferranti et al., 2024, DOI: 10.3233/sw-243611) — SHACL-Core cannot express all 32 WD constraint types; SPARQL can [1]; [SHACL over SPARQL endpoints](https://consensus.app/papers/details/7220c4725a2c5164bbc84ac492e662c3/?utm_source=cursor) (Corman et al., 2019) non-recursive = one query; recursive = NP-hard, SPARQL + SAT [2]; [SHACL2SPARQL](https://consensus.app/papers/details/01b947aacda8524092d66b2f2fa52896/?utm_source=cursor) [13]; [Re-SHACL](https://consensus.app/papers/details/aff6b3101aa7571090add65f1e608915/?utm_source=cursor) targeted reasoning before validate [8]; [SHACL-DS](https://consensus.app/papers/details/e559eb3a44f8591f980ef6bc4e045433/?utm_source=cursor) dataset/named-graph validation [4]; [SPARQL property paths](https://consensus.app/papers/details/33bd6a44aec357e092f20fef09628c2e/?utm_source=cursor) eval OK, containment harder [3]; [counting semantics of 1.1 paths](https://consensus.app/papers/details/53a72c24c96459418bcb8da1e077d52c/?utm_source=cursor) (Arenas et al., 2012) counting can make paths intractable [16]. Distinct from `paper-pgschema-types-keys` (PG-Types vs SHACL as *schema languages*; local types vs recursive shapes) and `paper-pg-constraint-repair` (repair after fail).

## 1. Raw idea

SHACL-Core is a shapes graph; SHACL-SPARQL embeds SPARQL in constraints. Production trick: **compile shapes to SPARQL ASK/SELECT** against an endpoint [2][10]. Recursion+negation needs Magic-Sets-style restriction to the target neighbourhood [11] or a SAT side-solver [2]. Property paths are SPARQL’s variable-length MATCH; the 1.1 counting semantics is a footgun [16]. SHACL-DS extends validation across named graphs in a dataset [4].

## 2. STCA applicability

Verify: shapes are **dictionary constraints**. Compile them to deterministic queries over the fold (Cypher rewrite cousin of SHACL2SPARQL), log violations as facts. Do not put a recursive SHACL reasoner in the write path. Query: SPARQL property paths ≈ Cypher `*`; prefer non-counting path semantics. RDF-star/SPARQL-star is a manifestation dialect, not Kutha SoT. Distinct from PG-Schema: that paper warned recursive shape *propagation*; this card is the **compile algorithm** to queries.

## 3. Quality / cost

Usefulness high if anyone brings RDF/Wikidata. Optimality med: Core is tractable-ish; full recursive SHACL is not P0. Cost: Cypher-side PG-Keys first; SHACL pack only as compile-to-query over an RDF *view*. Never SPARQL counting-paths as default.

## 4. Demand

“Validate the KG with SHACL” is the RDF world’s Grafixer. Engine demand: violation events. SPARQL endpoint is optional packaging.

## 5. Niche → effect

`no niche`
