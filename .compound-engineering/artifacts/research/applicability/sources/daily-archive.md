# daily-archive dossier (Wave 17 neighbor scout)

User wrote “daily-arxive”; repo is `/root/daily-archive`. CBM: `daily-archive`. Kutha pack architecture already in ADR-093; this dossier is schema-lifecycle evidence.

| item | value |
|------|--------|
| Default confidence | `code` for files read; `spec` for ADRs |
| Read | `crates/kg-ontology/src/schema.rs`, `healing.rs`; ADR-044 schema lifecycle; ADR-045 validator; ADR-058 PaperRevision |
| Coverage | `schema.rs` / `healing.rs`: `no_recorded_issue`, `metadata_match` (best-effort) |
| Closed card | `daily-archive-schema-lifecycle` |
| Do not | treat Samyama as Kutha SoT; collapse GraphHealingUseCase with PG-Constraint ILP or RuVector HNSW rewiring |

Kutha mapping: versioned YAML schema + `NodeSchemaDef` compile + `SchemaVersion`/`SchemaMigration` events. PaperRevision = Expression under versionless Work. Healing provenance = compensating events, not silent mutate.
