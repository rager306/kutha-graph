---
name: codebase-memory
description: >-
  Kutha adapter for codebase-memory-mcp (structural code graph). Use when
  exploring architecture, finding callers/callees, tracing impact, checking
  index freshness, or before/after ce-work edits. Not a Compound Engineering
  replacement. GitNexus is secondary.
---

# Codebase Memory (Kutha)

Evidence plane for **code**, not honeycomb ADRs and not governor CI. Follow `.cursor/rules/code-graph-cbm.mdc`. Project name is usually `kutha-graph`.

Do **not** replace `ce-plan` / `ce-work` / `ce-debug` / `ce-code-review`. Do **not** put MCP steps in plans.

## Default tools (use these)

| Question | Tool |
|----------|------|
| Indexed? Fresh? | `list_projects`, `index_status` |
| Coverage of paths | `check_index_coverage` |
| Find symbol | `search_graph` (`query` or `name_pattern`) |
| Callers / callees | `trace_path` (inbound / outbound / both) |
| Read body | `get_code_snippet` after exact `qualified_name` |
| Diff blast radius | `detect_changes` |
| Literals / gap | `search_code` or Grep |

## Verify / Auditor extras

- `query_graph` — Cypher; always `LIMIT`; `graph="missed"` for indexer gaps
- `get_architecture` — compact overview; request `aspects` only when needed
- `get_graph_schema` — once per session if writing Cypher

## Do not call

| Tool | Why |
|------|-----|
| `delete_project` | Forbidden |
| `manage_adr` | Conflicts with `docs/ADR/` |
| `ingest_traces` | Opt-in only |
| `index_repository` | Parent only; missing project, stale coverage, or explicit rebuild |

## Evidence tiers

- **Scout:** few positive lookups; no absence / dead-code / exhaustive claims.
- **Verify (default):** task-directed traces, snippets for material claims, paginate `has_more`.
- **Auditor:** current generation, bounded scope, both call directions when material, disclosed gaps.

After candidate paths: `check_index_coverage` once with every cited path. For negative claims, pass `scopes` too. `indexed_no_recorded_gap` is not completeness.

## CE phases

- `ce-plan` → Scout
- `ce-work` → Verify before and after edits
- `ce-debug` → `trace_path` / `search_graph`
- `ce-code-review` → `detect_changes`

## Subagents

Parent queries graph first. Pass project, generation/freshness, symbols, paths, coverage notes, and source fallback. Cursor: Scout ≈ `explore` + composer; Verify/Auditor ≈ `generalPurpose` with a heavier model. Codex custom roles stay in `docs/process/codex-subagents.md`.

## GitNexus

Secondary. Do not dual-query. If GitNexus index is refreshed, use `--index-only` (or local `.gitnexusrc` skip of AGENTS/skills inject).

## Gotchas

1. `trace_path` needs an exact name — `search_graph` first.
2. Paginate `search_graph` (`has_more` / `offset`).
3. Stale CBM: `check_index_coverage` freshness `metadata_changed` or `not_tracked` → integrator `index_repository`.
4. `search_code` can see disk text the graph nodes have not caught up to.
