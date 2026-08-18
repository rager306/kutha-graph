---
id: paper-online-pg-schema-evolution
source: paper
axes: [Data, Time, Composition, Verify]
usefulness: high
optimality: med
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# Schema change is a logged rewrite of the dictionary — not discovery and not OWL versioning

Papers: [Tesseract](https://consensus.app/papers/details/35960972b32754d59da99762ae0f2a39/?utm_source=cursor) (Hu et al., 2022) online transactional schema evolution as **data-definition-as-modification** on snapshot isolation: the ALTER is a table-wide write the CC protocol already knows [1]; [F1 asynchronous schema change](https://consensus.app/papers/details/50f8df5fb7965381ae918bc805c59b90/?utm_source=cursor) (Rae et al., 2013, VLDB) servers may be one version apart; corruption-causing ALTERs become a *sequence* of safe steps [4]; [PG schema validation and evolution](https://consensus.app/papers/details/312978432b0b54bbb3609c1aee0cf42f/?utm_source=cursor) (Bonifati et al., 2019) descriptive→prescriptive schemas; evolution as **graph rewriting**; validation as homomorphism schema→instance [5]; [Orion / U-Schema](https://consensus.app/papers/details/0245cf5f83dd5e5fa2e28a307c980777/?utm_source=cursor) (Chillón et al., 2024, TKDE) SCO taxonomy across graph/document/columnar/relational; Alloy-checked [3][14]; [PRISM SMOs](https://consensus.app/papers/details/6315b72fad615acc88849e95f73c716a/?utm_source=cursor) (Curino et al., 2008) Schema Modification Operators + query rewrite + data migration [7]. Distinct from `paper-pghive-schema-discovery` (infer a schema that is already there), `paper-pgschema-types-keys` (local types/keys, not the migration protocol), `paper-ontology-temporal-versioning` (OWL/KGCL), `daily-archive-schema-lifecycle` (YAML manifest compile).

## 1. Raw idea

Property graphs are “schema-optional” until production. Then you need **online** ALTER: no downtime, old queries still compile, data migrates. Tesseract: treat DDL as a snapshot write [1]. F1: never jump more than one schema version; decompose dangerous changes [4]. Bonifati: rewrite the *graph schema*, validate by homomorphism [5]. Orion: a portable taxonomy of schema-change ops including relationship types and structural variation [3]. PRISM: SMOs so Wikipedia-scale histories are scripts, not tribal knowledge [7]. Expand/contract is the operational pattern; dual-write during migration is the CDC cousin.

## 2. STCA applicability

Data/Time: a schema change is an **event** on the same log (`SchemaModify`), with valid-time like grants. Query: compiler binds to a schema version (AS-OF the dictionary), not “whatever nodes exist now.” Composition: pack activate (previous card) *may* require a schema SMO; they are not the same noun. Verify: homomorphism validation is a receipt that the instance still fits. Do not let an LLM emit ALTER as SoT.

## 3. Quality / cost

Usefulness high: legal packs will add properties without a maintenance window. Optimality med: Tesseract/F1 are RDBMS; Bonifati/Orion are the graph-shaped theory. Cost: P0 = append-only dictionary (new labels never remove old); honeycomb = SMO events + expand/contract + query rewrite to the new schema version.

## 4. Demand

Without online evolution, every new statute field is a rebuild. Engine demand: schema version on compiled Cypher, fail-closed if the instance homomorphism fails.

## 5. Niche → effect

`no niche`
