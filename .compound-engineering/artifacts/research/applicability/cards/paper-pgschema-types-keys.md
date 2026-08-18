---
id: paper-pgschema-types-keys
source: paper
axes: [Data, Query, Verify, Space]
usefulness: high
optimality: high
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# PG-Schema / PG-Keys: typed dictionaries and locally verifiable constraints

Papers: [PG-Schema: Schemas for Property Graphs](https://consensus.app/papers/details/5d19ee4316cd554686dab6e91c458b76/?utm_source=cursor) (Bonifati et al., 2023, PACMMOD, DOI: 10.1145/3589778; arXiv:2211.10962); [PG-Keys](https://consensus.app/papers/details/d60f88ee9b02545faa63950a3bee6e22/?utm_source=cursor) (Angles et al., 2021, SIGMOD, DOI: 10.1145/3448016.3457561); [Common Foundations for SHACL, ShEx, and PG-Schema](https://consensus.app/papers/details/8a660dc171215943beeed8b65fe47ee7/?utm_source=cursor) (Ahmetaj et al., 2025, WWW, DOI: 10.1145/3696410.3714694). Cousins: [schema validation/evolution](https://consensus.app/papers/details/312978432b0b54bbb3609c1aee0cf42f/?utm_source=cursor) (Bonifati et al., 2019); [PG-HIVE](https://consensus.app/papers/details/27ccf5a0457455989c7aad8a9665c169/?utm_source=cursor) incremental schema discovery; ProGS shapes. Distinct from `paper-gql-rules-materialization` (derived edges) and `paper-semantic-layer-smq` (SQL IR).

## 1. Raw idea

GQL v1 and most PG engines have weak DDL. PG-Schema splits **PG-Types** (node/edge label+property combinations, multi-inheritance, abstract types, open vs closed, STRICT vs LOOSE graph types in Cypher ASCII-art) from **PG-Keys** (exclusive / mandatory / singleton keys on nodes, edges, or properties; foreign keys; participation). Types are **locally verifiable**; constraints may mention types, types may not mention constraints (unlike recursive SHACL shape propagation). Strict / loose / partial validation covers schema-first, flexible, and mixed development. LDBC PGSWG → ISO GQL DDL track. SHACL/ShEx/PG-Schema share a comparable constraint core with different graph models.

## 2. STCA applicability

Space/Data: this is the **dictionary** surface — compile Cypher/GQL against declared types, not against a silent bag of labels. Verify: schema validation is a pack over the fold, not a second SoT; violations are facts (repair papers delete nodes/edges/labels — Kutha should log the violation, not silently rewrite history). Query: schema-assisted pattern proposal (fraud-explorer example) is AZ’s projected-schema cousin for *structure*, not for NL. Distinct from GQL Rules (inflationary MERGE of derived edges) and SMQ (metrics IR → SQL). Evolution (graph rewriting) must be events.

## 3. Quality / cost

Usefulness high: users already demand schema compliance; types make MATCH compilation and UI explorers possible. Optimality high for the type fragment (tractable if each key is); SHACL-style recursive shapes are the complexity trap to avoid. Cost: adopt PG-Types/PG-Keys *vocabulary* as dict AST; do not vendor Neo4j constraint dialect; descriptive schema mining (PG-HIVE/DiscoPG) is a pack that **emits** dictionary events.

## 4. Demand

Every Cypher surface without types becomes `ruvector-cypher-empty-success`. Engine demand: dict-first compile. Standards demand: GQL DDL will look like this.

## 5. Niche → effect

`no niche`
