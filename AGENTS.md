# AGENTS.md — Kutha / kutha-graph

Agent operating notes for this repository. Chat with the human is **Russian**; this file and all other docs stay **English**.

## Language policy

| Surface | Language |
|---------|----------|
| Chat with the user (questions, summaries, menus, clarifications) | **Russian** |
| Documents (ADR, README, plans, process notes, comments in docs) | **English** |
| Source code, identifiers, commit messages, PR bodies, CI logs | **English** |
| Inline code comments and docstrings | **English** |

Do not mix languages inside a single artifact. Russian commit/changelog/PR verbs route via `.cursor/rules/ce-skills-ru.mdc`. Do not patch plugin-cache skill descriptions.

## Two planes (do not collapse)

| Plane | Owns | Must not own |
|-------|------|----------------|
| **Product** (`crates/kutha-*`) | Temporal graph truth: event log = SoT | Roadmap ceremony |
| **Harness** (`scripts/kutha_gov`, `.kutha/`) | Trajectory honesty, freeze, lifecycle non-collapse | Architecture decisions, legal/product readiness |

Governor green ≠ ADR Accepted ≠ capability. Honeycomb **Proposed** ≠ delivery backlog. Literature cards ≠ shipping list. Session planning lives in `.planning/` (GSD overlay, not harness SoT) and must not grow a second living roadmap. `.planning/STATE.md` is GSD progress; `.kutha/STATE.md` is harness lease; `docs/ADR/` is architecture — do not collapse them.

## Product snapshot

- **Product:** Kutha — hybrid AI-native temporal graph engine (research stage).
- **Idea stack (top → down):** **STCA** (ADR-002) → vision (ADR-001) → locks D1–D10 (ADR-000) → honeycomb cells (ADR-010–093, all **Proposed**).
- **Formula:** event log = SoT; graph = deterministic fold; CSR/HNSW/views = droppable leases; LLM proposes; dictionaries + log own audited truth.
- **Wedge:** legal / normative temporal agents (norm **AS OF** a date). Science next; finance/clinical are receipt/hold riders until a pack exists.

P0 crate spike (not Accepted product): `docs/architecture/p0-spike-inventory.md`. Agents do not mint aggregator waves.

## Where to start

Read `.kutha/STATE.md` first; do not start work it does not name. Freeze is that file's Freeze section. GSD focus is `.planning/STATE.md` (does not thaw freeze or lease a slice). Repo layout: `README.md` § Layout.

## Artifacts and layout rules

- Compound Engineering `docs_root` is `.compound-engineering/artifacts` (`.compound-engineering/config.yaml`; `docs_root` in `config.local.yaml` is ignored). Plans, research, ideation, and handoffs live there, not under `docs/`. Captured learnings: `artifacts/solutions/<category>/` (YAML frontmatter `module`, `tags`, `problem_type`) — relevant context, not architecture SoT or a backlog.
- Durable CE outputs stay under `docs_root`, never `/tmp`, `$TMPDIR`, or `.tmp`. `ce-handoff` defaults to OS-evictable `/tmp`: write `artifacts/handoffs/<topic>.md` instead. One-shot scratch goes in repo-local gitignored `tmp/` (harness tests and `kutha-gov selftest` use it), not system `/tmp`; `.context/compound-engineering/` is gitignored scratch. Prefer `.planning/` for surviving session plans.
- Do **not** add repo-root `ports/` / `adapters/` / `domain/` (ADR-022). Do **not** put Python inside `kutha-runtime`.

## Commands

Product:

```text
cargo test --workspace
```

Harness (Python **3.13** via **uv** only — not system `python3`):

```text
uv run kutha-gov ci          # FSM quantum: relations → checks → observe → emit → tenant → fold
uv run kutha-gov selftest    # mutate a copy; unproven fails
uv run kutha-gov precommit   # dictionary checks only (no cargo, no JSONL)
uv run kutha-gov fsm         # print the process machine
uv run kutha-gov map         # compact L_map index
uv run kutha-gov py          # ruff + ty + pyrefly
uv run kutha-gov fold | list | explain …
uv run pytest
```

Pin: `.python-version`. New governor check or CI phase: YAML row — `.kutha/META.md` and `docs/process/governor-intake.md` (not a new Python class). Hook: `uvx pre-commit install --overwrite` (precommit, not full `ci`).

## Working conventions

1. Honor locked ADR-000 **D1–D10**. Do not revive: pure Samyama product, pure ActiveGraph without hot projections, hard FSM as sole agent control, TypeScript as graph core, RVF as primary storage, Graphiti/Dify/Hindsight as SoT.
2. STCA first. New product detail → honeycomb ADR-010+ (`docs/ADR/README.md`), never silent rewrites of 000/001/002. **Accepted** only when that cell is in the running engine.
3. Honeycomb is a **map**. One steel thread at a time. “Promote all” is forbidden. GSD tracks fitness/freeze/lease — not cell enumeration.
4. Prefer falsifiable spikes over generic “build a graph DB” advice.
5. Core stays self-contained Rust (no mandatory external graph DB / Graphiti runtime / LLM for temporal truth).
6. Harness is a **parallel STCA plane** (`docs/process/kutha-harness.md`). H0 = files + JSONL + dictionaries + FSM; H2 = typed triples on the Kutha log via `kutha-tenant`. Do **not** copy `stca-guide.md` §5 JSON merge-patch as the product write surface. Do not clone sprawling multi-hundred milestone GSD installs. Control loop → check: `docs/process/governor-intake.md`.
7. Three harness lifecycles stay orthogonal: **L_map** · **L_delivery** · **L_capability**. Bridges may cite; they may not copy state machines.
8. Intern map (ADR-011) ≠ agent dictionaries (ADR-050). Do not collapse them.
9. Humans start at `README.md`. Agents follow this file, then `.kutha/STATE.md` + `.planning/STATE.md`. Dated history: `CHANGELOG.md` via `.cursor/skills/kutha-changelog/SKILL.md`. Commits: **ce-commit** (Russian: коммит / закоммить). Do not bump `0.0.0`, tag, or publish GitHub Releases unless STATE/process explicitly allows. Git user-rule stays safety-only.

## Code graph (Cursor)

**codebase-memory-mcp (CBM)** is the structural code evidence plane. It is not product SoT, honeycomb, or a governor check. Tools: `.cursor/skills/codebase-memory/SKILL.md`. Always-on: `.cursor/rules/code-graph-cbm.mdc`.

Forbidden: `delete_project`, CBM `manage_adr` (`docs/ADR/` owns ADRs). Only the parent/integrator runs `index_repository` (missing/stale or user-asked). Graph coverage ≠ governor green ≠ review approval.

**GitNexus is secondary.** Do not dual-query with CBM. Reindex with `gitnexus analyze --index-only` or a local `.gitnexusrc` (`skipAgentsMd` + `skipSkills`) — never let analyze rewrite this file or `CLAUDE.md`.

Subagent workflow: `docs/process/codex-subagents.md`. Parent owns the steel thread, freeze, integration, and the final Russian reply. Pass STATE freeze into helpers. Parallelism does not lease a milestone.

Every `Task` that needs the graph must name **`codebase-memory-mcp`** (`list_projects` first; graph before Grep; `check_index_coverage` on cited paths). Children do not inherit Cursor rules. No MCP → read source; do not claim graph verification.

| Codex-style role | Cursor `Task` |
|------------------|---------------|
| codebase-memory-scout | `explore` (narrow) |
| codebase-memory (verify) | `generalPurpose` (heavier when span is large) |
| codebase-memory-auditor | `generalPurpose` (bounded audit) |
| implementation-worker | owned files only |
| correctness-reviewer | `code-reviewer` (diff review ≠ CBM auditor) |

Disjoint file ownership. Only the integrator touches shared lockfiles, `.kutha/events.jsonl`, tenant data, or graph indexes, and runs `uv run kutha-gov ci` when required.

CE remains for commit/PR/changelog/handoff skills and the closed literature store; it is not the primary planning SoT. `ce-setup` does not author `README.md`.
