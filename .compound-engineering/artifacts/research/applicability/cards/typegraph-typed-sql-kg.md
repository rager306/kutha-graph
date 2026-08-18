---
id: typegraph-typed-sql-kg
source: typegraph
axes: [Query, Time, Data, Agent]
usefulness: high
optimality: med
demand: high
confidence: code
layer5: no niche
status: closed
channels_failed: []
---

# Typed PG-in-SQL is an application port — TypeScript is not the Kutha core

User URL: https://github.com/nicia-ai/typegraph (cloned `/tmp/user-url-scout/typegraph`, not CBM-indexed). TypeScript-first embedded KG on SQLite/PostgreSQL (Zod + Drizzle). Read: `packages/typegraph/src/ontology/types.ts` + `core-meta-edges.ts` — meta-edges `subClassOf` / `implies` / `inverseOf` / `disjointWith` with inference kinds (subsumption, constraint, …); `ontology/closures.ts` — Warshall transitive closure; `core/temporal.ts` — bitemporal `RecordedInstant` `r1:<16-digit revision>:<UTC>` so same-millisecond commits stay ordered; `query/compiler/temporal.ts` — valid-time SQL uses the **application** clock, not `NOW()`, pinned across set-ops; `graph-extension/define-graph-extension.ts` — agent JSON → frozen `GraphExtension`, then `store.evolve`. ADR-000 **rejects TypeScript as graph core**. Distinct from `paper-pgschema-types-keys` (ISO types/keys, not Zod-on-SQLite), `paper-engram-bitemporal-memory` / `paper-tgql-intervals` (memory/QL, not SQL compile), `paper-online-pg-schema-evolution` (SMO/F1 ALTER, not LLM JSON extensions), `graphiti-bitemporal-fact-edges` (Neo4j extract).

## 1. Raw idea

Keep the property graph **in the app’s existing SQL database**: `defineNode`/`defineEdge` with Zod, fluent traverse compiles to SQL, ontology meta-edges close under Warshall, bitemporal reads reconstruct valid-time and recorded-time, agents can propose a graph-extension document that evolves kinds at runtime without redeploy. Vector/hybrid search is a sidecar (pgvector / sqlite-vec). Not a distributed graph engine.

## 2. STCA applicability

Query/Data: the *transferable* noun is **typed dictionary + SQL-shaped compile of MATCH**, not Drizzle. Time: recorded instant = (monotonic revision, wall clock) is a useful encoding of transaction-time on a *foreign* SQL store; Kutha’s SoT remains the event log, not SQLite rows. Agent: graph-extension JSON is an **SMO candidate** — validate, then log `SchemaModify`; never let the LLM mutate tables as truth. Composition: TypeGraph is a **port/product cousin** for teams that will not leave Postgres; Kutha core stays Rust. Do not revive TS-as-core.

## 3. Quality / cost

Usefulness high: the ideas (app-clock AS-OF, pinned read instant, extension validation with JSON pointers) are concrete. Optimality med: Warshall on type names is fine for TBox size, not for the hop engine; SQL traverse ≠ WCOJ. Cost: steal the temporal-compile and extension-validation *contracts*; do not vendor the monorepo or make SQLite the fold.

## 4. Demand

App teams already have Postgres and want graph + audit time without a second database. Engine demand: dict-first compile + bitemporal reads; offer SQL *export/CDC in*, not SQL *as SoT*.

## 5. Niche → effect

`no niche`
