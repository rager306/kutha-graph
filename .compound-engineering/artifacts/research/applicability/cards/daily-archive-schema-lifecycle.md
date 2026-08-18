---
id: daily-archive-schema-lifecycle
source: daily-archive
axes: [Space, Time, Verify, Data]
usefulness: high
optimality: med
demand: high
confidence: code
layer5: "Scientific archive: versioned YAML schema + PaperRevision expressions; Samyama remains a lease"
status: closed
channels_failed: []
---

# Versioned schema manifest + schema-as-code compile; data healing ≠ schema healing

CBM `daily-archive` (coverage: `crates/kg-ontology/src/schema.rs`, `healing.rs` — `no_recorded_issue`). `NodeSchemaDef` trait: label, required/optional fields, `validate()`. ADR-044: Samyama is schemaless; schema is an **application-layer** contract — YAML `data/schema/versions/*.yaml`, `SchemaVersion` / `SchemaMigration` nodes, expand-contract, idempotent migrations; `CURRENT_SCHEMA_VERSION = 1` integer is insufficient. ADR-045 validator returns *all* violations (registry, required, D127/D134, types, unknown fields). ADR-058 PaperRevision = FRBR Expression under versionless Paper Work. `HealingOperation`: Correct/Merge/Split/Silence/Migrate/Rollback/RepairEdge + `ProvenanceEvent`. Distinct from `ruvector-hnsw-delete-repair` (ANN rewiring) and `paper-pg-constraint-repair` (PG-Constraint ILP deletes).

## 1. Raw idea

Two compiles: (1) Rust `NodeSchemaDef` as the running validator; (2) intended YAML manifests as the versioned SoT for *schema*, with in-graph `SchemaVersion` audit. Data-level `GraphHealingUseCase` already exists; ADR-044 says it does **not** cover missing indexes / orphaned labels / deprecated types. PaperRevision keeps v1 vs v5 structure from colliding under one arXiv work id.

## 2. STCA applicability

Space/Time: schema lifecycle is dictionary versioning (τOWL cousin, Cypher/PG not OWL). Data healing with provenance is the right *shape* for compensating events — still not silent ILP delete. ADR-093 already remaps Samyama from SoT to lease; this card extracts the **schema compiler + version registry** so Kutha does not copy `kg-ontology` as a second engine. SPARQL/SHACL unused here; Cypher + trait validate is the compile target.

## 3. Quality / cost

Usefulness high: 28 node types without a registry is how Category/`vid` bugs shipped. Optimality med: dual SoT risk (Rust trait vs YAML) until YAML wins. Cost: one dict manifest format for Kutha packs; compile to validators; log SchemaMigration; keep HealingOperation as event types.

## 4. Demand

Scientific ingest will add node types weekly. Engine demand: schema_version on every node (they already require it). Product: fail-closed import (D127) stays pack policy.

## 5. Niche → effect

Scientific archive: versioned YAML schema + PaperRevision expressions; Samyama remains a lease.
