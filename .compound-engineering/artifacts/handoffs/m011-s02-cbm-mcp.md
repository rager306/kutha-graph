---
artifact_contract: "ce-handoff/v1"
created_at: "2026-09-16T07:08:00Z"
title: "M011 S02 done; CBM MCP not in this agent catalog"
summary: "M010 closed and M011 S01–S02 shipped on feat/m010-s01-semantic-recovery; next product slice is leased P→Q. codebase-memory-mcp works as a binary but was missing from this Cursor agent session."
keywords: ["M011", "claim_id", "BrokenLineage", "codebase-memory-mcp", "handoff"]
cwd: "/root/kutha-graph"
resume_focus: "Lease M011 S03 for a thin P→Q derivation fixture, or confirm CBM MCP after a new agent chat. Do not start M002."
repository: "kutha-graph"
repo_root_sha: "1ee5942d3f12fafc1be6983f45d0713958047900"
branch: "feat/m010-s01-semantic-recovery"
head: "f7adfb8"
worktree_path: "/root/kutha-graph"
---

# Session handoff

## Objective

Debt-first CE after M010: close identity/replay gaps, keep meta-prompt-FSM, freeze M002. User later asked why `codebase-memory-mcp` was missing and to commit + handoff.

## Done (product / process)

| Wave | HEAD-relative | Notes |
|------|----------------|-------|
| M010 S03 + close | `d9bab6d` | live `intern` → `Op::Define`; `graph_len`; lease `M010-closed` then M011 |
| M011 S01 | `59031eb` | `Assert.claim` / `Fact.claim_id`; retract one support, other remains |
| M011 S02 | `4a34fbc` + `f7adfb8` | `UnknownClaim` fail-closed; `replay_check` `BrokenLineage`; ADR-060 honeycomb evidence |

Plans/solutions: `.compound-engineering/artifacts/plans/2026-09-16-13*` and `solutions/architecture-patterns/m010-*`, `m011-*`.

Lease now: `.kutha/STATE.md` — Active Milestone **M011**, Active Slice **None**, `L_delivery=M011-S02-done`. Phase H4. M002 freeze unchanged.

## Decisions

- Next steel thread after recovery is claim/support (ADR-011 / semantic-contract), not Rocks.
- P→Q derivation engine is a **later slice**, not S02.
- Do not promote honeycomb to Accepted.
- GitNexus is the in-session graph (plugin). CBM CLI works; agent catalog did not include user `~/.cursor/mcp.json` servers.

## CBM MCP (machine-local)

- Binary: `/root/.local/bin/codebase-memory-mcp` 0.10.8; `cli list_projects` includes `kutha-graph`.
- Handshake ~5s. Index cache: `/root/.cache/codebase-memory-mcp/kutha-graph.db`.
- This agent catalog only listed plugins (GitNexus, GitHub, Gmail). User MCP (CBM, jina, exa) was absent.
- `kutha-graph` lacked `mcp-approvals.json` (unlike `daily-archive`).
- Written (needs **new agent chat** / MCP reload to take effect):
  - repo `.cursor/mcp.json` (stdio CBM)
  - `/root/.cursor/projects/root-kutha-graph/mcp-approvals.json` (not in git)
  - `/root/.cursor/plugins/local/codebase-memory/` (not in git)
- Do not call `delete_project`. Reindex GitNexus after HEAD moves; CBM `index_repository` only if `list_projects` misses the repo.

## Unfinished

- M011 S03: thin P→Q fixture (needs Active Slice lease).
- Execution replay (ADR-060 obligation 3) still absent.
- Untracked leftover: `.claude/skills/`, `.compound-engineering/artifacts/handoffs/ce-code-review-h4-overlay/` — do not treat as SoT.

## Verify

- `uv run kutha-gov ci` was green after S02.
- `cargo test --workspace --offline` green after S01/S02.

## Next (one path)

1. New Cursor agent so CBM appears, or continue with GitNexus if still missing.
2. If product: lease **M011 S03** P→Q in STATE/ROADMAP, TDD, changelog, commit.
3. Do **not** start M002 / legal pack / Cypher / HNSW.

Skills: `ce-work` after a leased plan; `ce-commit` / `kutha-changelog` for ship; GitNexus MCP `query` / `detect_changes`.
