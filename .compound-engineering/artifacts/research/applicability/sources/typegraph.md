# TypeGraph (nicia-ai/typegraph)

User-supplied 2026-08-18. TypeScript-first embedded property graph on SQLite/PostgreSQL (Zod + Drizzle). ADR-000 rejects TypeScript as Kutha graph core.

| item | value |
|------|--------|
| Repo | https://github.com/nicia-ai/typegraph |
| Disk / CBM | cloned `/tmp/user-url-scout/typegraph` for this scout; not vendored; not indexed |
| Default confidence | `code` only for files actually read |
| Read | README; `packages/typegraph/src/ontology/{types,core-meta-edges,closures}.ts`; `core/temporal.ts`; `query/compiler/temporal.ts`; `graph-extension/define-graph-extension.ts` |
| Closed card | `typegraph-typed-sql-kg` |
| Do not | revive TypeScript as the engine core; treat SQLite/Postgres rows as event-log SoT; vendor the monorepo |

Kutha mapping: transferable **typed dict + app-clock bitemporal SQL compile + validated agent graph-extensions**. Product cousin for teams that stay on Postgres. Vector search / graph-algorithms API queued skip unless distinct from HNSW and SciRS.
