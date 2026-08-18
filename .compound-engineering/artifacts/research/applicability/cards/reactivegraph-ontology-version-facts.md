---
id: reactivegraph-ontology-version-facts
source: reactivegraph
axes: [Time, Space, Query, Verify]
usefulness: med
optimality: low
demand: med
confidence: code
layer5: no niche
status: closed
channels_failed: []
---

# OntologyVersion registry + fact supersede; Cypher shape-guard — not SPARQL/SHACL

Repo `/root/reactivegraph` (not CBM-indexed). Read: `crates/rg-domain/src/knowledge.rs`, `doc/SCHEMA.md`, `crates/rg-composition/src/cypher_guard.rs`, ADR-0020/0030. `OntologyVersion` newtype (semver string); `KnowledgeFact` lifecycle Active → Superseded | Invalidated; `SnapshotId` is the AS-OF cutoff; `ontology_versions` and `fact_revisions` tables in embedded redb. `cypher_guard`: lexer + reject CREATE/MERGE/DELETE/…; require MATCH+RETURN. ADR-0030: embedded ruvector as sole storage (supersedes PG+extension). Distinct from Kutha D1 (event log SoT) and from law-nexus YAML catalog (RG ontology is vendor-fact schema, not legal ladders).

## 1. Raw idea

ReactiveGraph versions the **knowledge ontology** separately from task/plan/execution graphs. Facts are content-hashed; a newer snapshot supersedes or invalidates. Schema bump is an integer on redb (`SCHEMA_VERSION` 1…5) plus an `ontology_versions` blob registry. Query surface is Cypher with a **syntactic read-only guard**, not SHACL and not SPARQL.

## 2. STCA applicability

Time: snapshot AS-OF + fact revision history is a fold picture; do **not** take redb/ruvector as Kutha SoT (ADR-0030 is the anti-pattern relative to D1). Space: `OntologyVersion` is the dict stamp. Query: cypher_guard is a cheap compile gate (AZ cousin: don’t let MCP write). Verify: Invalidated ≠ delete. SPARQL/SHACL: absent — if RG ever needs RDF export, OBDA/SHACL2SPARQL are packs on a view.

## 3. Quality / cost

Usefulness med: clean separation of knowledge vs runtime graphs (ADR-0020 two-table rule). Optimality low for Kutha kernel: sole-store ruvector + integer schema meta. Cost: borrow fact Active/Superseded/Invalidated + read-only query shape; reject sole-substrate lock.

## 4. Demand

Agent platforms want “versioned KG about the codebase.” Engine demand: ontology_version on facts. Not a SPARQL shop.

## 5. Niche → effect

`no niche`
