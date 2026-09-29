# AGENTS.md — Kutha / kutha-graph

Agent operating notes for this repository. Read this before honeycomb ADRs or the applicability matrix. Chat with the human is **Russian**; this file and all other docs stay **English**.

## Language policy

| Surface | Language |
|---------|----------|
| Chat with the user (questions, summaries, menus, clarifications) | **Russian** |
| Documents (ADR, README, plans, process notes, comments in docs) | **English** |
| Source code, identifiers, commit messages, PR bodies, CI logs | **English** |
| Inline code comments and docstrings | **English** |

Do not mix languages inside a single artifact.

## Primary workflow: GSD

Delivery and session planning run through **GSD** (`.cursor/skills/gsd-*`, `.cursor/gsd-core/`). Do not invent a parallel planning tree.

| Need | Command / location |
|------|-------------------|
| Read GSD position | `.planning/STATE.md` (GSD lease — not harness SoT) |
| Requirements / roadmap | `.planning/REQUIREMENTS.md`, `.planning/ROADMAP.md` |
| Project brief | `.planning/PROJECT.md` |
| Codebase map | `.planning/codebase/` (`/gsd-map-codebase`) |
| Ingested ADR intel | `.planning/intel/`, `.planning/INGEST-CONFLICTS.md` |
| Configure agents | `/gsd-config` → `.planning/config.json` |
| Plan a phase | `/gsd-plan-phase N` |
| Execute plans | `/gsd-execute-phase` |
| Onboard / re-bootstrap | `/gsd-onboard` → map → ingest → scaffold |
| Manager / next step | `/gsd-manager` |

**Three state files — do not collapse**

| File | Owns | Must not replace |
|------|------|------------------|
| `.planning/STATE.md` | GSD phase/plan progress | Harness milestone lease |
| `.kutha/STATE.md` | Harness Active Milestone / Slice / freeze / L_* | GSD roadmap |
| `docs/ADR/*` | Architecture map (Proposed ≠ Accepted) | Delivery backlog or GSD phases |

Governor green ≠ ADR Accepted ≠ capability ≠ GSD phase complete. Honeycomb **Proposed** ≠ GSD v1 requirements. Do not turn ADR-010–093 into sequential GSD phases.

Current GSD overlay (steel thread): Phase 1 legal PIT fitness → Phase 2 honest harness/freeze → Phase 3 lease-gated next slice. Phases 1–2 are mostly **verification-first** against crates already in tree. Phase 3 stays blocked until `.kutha/STATE.md` names an Active Slice.

Russian chat verbs for **commit / changelog / PR** still route via `.cursor/rules/ce-skills-ru.mdc` (`ce-commit`, `kutha-changelog`, `ce-commit-push-pr`). Prefer **GSD** for plan / execute / map / onboard / config. Do not patch plugin-cache skill `description` fields.

## Two planes (do not collapse)

| Plane | Owns | Must not own |
|-------|------|----------------|
| **Product** (`crates/kutha-*`) | Temporal graph truth: event log = SoT | Roadmap ceremony |
| **Harness** (`scripts/kutha_gov`, `.kutha/`) | Trajectory honesty, freeze, lifecycle non-collapse | Architecture decisions, legal/product readiness |

Literature cards ≠ shipping list. GSD `.planning/` is an agent planning overlay — not product SoT and not a second governor.

## Product snapshot

- **Product:** Kutha — hybrid AI-native temporal graph engine (research stage).
- **Idea stack (top → down):** **STCA** (ADR-002) → vision (ADR-001) → locks D1–D10 (ADR-000) → honeycomb cells (ADR-010–093, all **Proposed**).
- **Formula:** event log = SoT; graph = deterministic fold; CSR/HNSW/views = droppable leases; LLM proposes; dictionaries + log own audited truth.
- **Wedge:** legal / normative temporal agents (norm **AS OF** a date). Science next; finance/clinical are receipt/hold riders until a pack exists.
- **P0 spike (in crates, not Accepted product):** log, fold, quantum emit→idle, receipt, snapshot, WAL-cousin, CSR lease, leapfrog intersect, `Materializer` trait, `fork_at`, named `as_of`/`live_at`, FF6 relation allowlist. **Not in spike:** RocksDB crate, Cypher, HNSW, ABAC, legal corpus, ADR-050 six dictionaries.

## Current execution position

1. Read **`.kutha/STATE.md`** first (harness lease, freeze, Active Slice).
2. Read **`.planning/STATE.md`** for GSD phase focus.
3. Honor both: a GSD plan does not authorize thawing freeze or starting an unleased slice.

**Harness lease (cite):** M011 active (`L_delivery=M011-S03-done`); Active Slice **None**; Phase **H4**. Do not start a legal pack. Do not start M002 Rocks until STATE names it. Further M011 slices need an explicit Active Slice lease.

**Freeze until explicit M002:** RocksDB crate, Cypher/GPML parser, HNSW, ADR-050 six dictionaries, ADR-080/081, full ADR-090/093 packs, ADR-100+, Consensus Query 103+.

Literature bound is **closed** (163 cards). Do not mint aggregator waves. Matrix SoT: `.compound-engineering/artifacts/research/applicability/` (`cards/` = SoT; `matrix.md` = rollup).

## Repository layout

```text
kutha-graph/
├── README.md                          # human entry
├── CLAUDE.md                          # shim → this file
├── AGENTS.md                          # this file
├── CHANGELOG.md                       # dated product/process history
├── STRATEGY.md                        # wedge, metrics, non-goals
├── Cargo.toml / pyproject.toml        # Rust product + uv harness
├── .planning/                         # GSD overlay (primary agent planning)
│   ├── STATE.md / PROJECT.md / REQUIREMENTS.md / ROADMAP.md
│   ├── config.json                    # /gsd-config
│   ├── codebase/                      # /gsd-map-codebase
│   ├── intel/                         # /gsd-ingest-docs outputs
│   └── INGEST-CONFLICTS.md
├── .kutha/                            # harness pyramid (process plane)
│   ├── STATE.md                       # lease: milestone / slice / freeze / L_*
│   ├── ROADMAP.md                     # harness milestones (not honeycomb backlog)
│   ├── dictionaries/                  # checks, fsm, invariants, bridges, honeycomb
│   └── events.jsonl                   # gitignored H0 observations
├── .cursor/                           # GSD skills/core, MCP, CBM adapter rules
├── crates/kutha-common|kutha-runtime  # product plane
├── docs/ADR/                          # spine + honeycomb (Proposed)
├── docs/process/                      # harness + governor intake + subagents
├── scripts/kutha_gov/               # governor CLI interpreter
└── .compound-engineering/artifacts/   # legacy CE docs_root (research/handoffs archive)
```

Do **not** add repo-root `ports/` / `adapters/` / `domain/` (ADR-022). Do **not** put Python inside `kutha-runtime`.

Durable agent artifacts that must survive sessions: prefer **`.planning/`** (GSD) or, for literature/research already landed, `.compound-engineering/artifacts/`. Do **not** write team knowledge to `/tmp`. One-shot scratch may use `mktemp`.

## Commands

Product:

```text
cargo test --workspace
```

Harness (Python **3.13** via **uv** only — not system `python3`):

```text
uv run kutha-gov ci          # FSM quantum: relations → checks → observe → emit → tenant → fold
uv run kutha-gov precommit   # dictionary checks only (no cargo, no JSONL)
uv run kutha-gov fsm         # print the process machine
uv run kutha-gov map         # compact L_map index
uv run kutha-gov py          # ruff + ty + pyrefly
uv run kutha-gov fold | list | explain …
uv run pytest
```

Pin: `.python-version`. New governor check = YAML row in invariants/bridges then checks — not a new Python class. New CI phase = `fsm.yaml` row. Commit hook: `uvx pre-commit install --overwrite` (precommit, not full `ci`).

## Working conventions

1. Honor locked ADR-000 **D1–D10**. Do not revive: pure Samyama product, pure ActiveGraph without hot projections, hard FSM as sole agent control, TypeScript as graph core, RVF as primary storage, Graphiti/Dify/Hindsight as SoT.
2. STCA first. New product detail → honeycomb ADR-010+ (`docs/ADR/README.md`), never silent rewrites of 000/001/002. **Accepted** only when that cell is in the running engine.
3. Honeycomb is a **map**. One steel thread at a time. “Promote all” is forbidden. GSD phases track fitness/freeze/lease — not cell enumeration.
4. Prefer falsifiable spikes over generic “build a graph DB” advice.
5. Core stays self-contained Rust (no mandatory external graph DB / Graphiti runtime / LLM for temporal truth).
6. Harness is a **parallel STCA plane** (`docs/process/kutha-harness.md`). H0 = files + JSONL + dictionaries + FSM; H2 = typed triples on the Kutha log via `kutha-tenant`. Do **not** copy `stca-guide.md` §5 JSON merge-patch as the product write surface (typed `Op` / ADR-010). Do not clone sprawling multi-hundred milestone GSD installs — this repo’s GSD overlay is a short steel thread. Control loop → check: `docs/process/governor-intake.md`.
7. Three harness lifecycles stay orthogonal: **L_map** · **L_delivery** · **L_capability**. Bridges may cite; they may not copy state machines.
8. Intern map (ADR-011) ≠ agent dictionaries (ADR-050). Do not collapse them.
9. Humans start at `README.md`. Agents follow this file, then `.kutha/STATE.md` + `.planning/STATE.md`. Dated history: `CHANGELOG.md` via `.cursor/skills/kutha-changelog/SKILL.md`. Commits: **ce-commit** (Russian: коммит / закоммить). Do not bump `0.0.0`, tag, or publish GitHub Releases unless STATE/process explicitly allows. Git user-rule stays safety-only.

## Code graph (CBM) + GSD

**codebase-memory-mcp (CBM)** is the structural **code** evidence plane inside GSD and CE. It is not product SoT, not honeycomb, and not a substitute for `/gsd-plan-phase`. Tool matrix: `.cursor/skills/codebase-memory/SKILL.md`. Always-on: `.cursor/rules/code-graph-cbm.mdc`. MCP: `.cursor/mcp.json`.

| GSD moment | CBM |
|------------|-----|
| `/gsd-map-codebase`, plan research | Scout / Verify lookups |
| `/gsd-plan-phase` | Verify symbols/paths in scope |
| `/gsd-execute-phase` | Before edits: graph + inbound `trace_path` or `detect_changes`; after: coverage on touched paths |
| Review / debug | `detect_changes` / `trace_path` |

Keep GSD **`intel.enabled`** / **`graphify.enabled`** Off unless you explicitly need GSD-native intel/graphify — do not dual-index with CBM. Do not put MCP choreography into ROADMAP or PLAN files (plans stay portable: symbols, paths, risks).

Forbidden: `delete_project`, CBM `manage_adr` (`docs/ADR/` owns ADRs). Only parent/integrator runs `index_repository` when missing/stale or user-asked. Graph coverage ≠ governor green ≠ GSD verification.

**GitNexus is secondary.** Do not dual-query with CBM. Reindex with `gitnexus analyze --index-only` or a local `.gitnexusrc` (`skipAgentsMd` + `skipSkills`) — never let analyze rewrite this file or `CLAUDE.md`.

## Subagents

Parent owns the steel thread, freeze, integration, and the final Russian reply. Pass `.kutha/STATE.md` freeze/lifecycle into every helper. Parallelism does not lease a new milestone. Details: `docs/process/codex-subagents.md`.

Every `Task` that needs structure must name **`codebase-memory-mcp`** (`list_projects` first; graph before Grep; `check_index_coverage` on cited paths). Children do not inherit Cursor rules. No MCP → read source; do not claim graph verification.

| Codex-style role | Cursor `Task` |
|------------------|---------------|
| codebase-memory-scout | `explore` (narrow) |
| codebase-memory (verify) | `generalPurpose` (heavier when span is large) |
| codebase-memory-auditor | `generalPurpose` (bounded audit) |
| implementation-worker | owned files only |
| correctness-reviewer | `code-reviewer` (diff review ≠ CBM auditor) |
| gsd-* specialists | matching `gsd-*` subagent types when present |

Disjoint file ownership across helpers. Only the integrator touches shared lockfiles, `.kutha/events.jsonl`, tenant data, or graph indexes, and runs `uv run kutha-gov ci` when the task requires it.

## Legacy: Compound Engineering

CE remains installed for **commit / PR / changelog / handoff** skills and for the closed literature store under `.compound-engineering/artifacts/`. It is **not** the primary planning SoT anymore — prefer `.planning/` and GSD commands above.

If a CE plan/handoff is still useful, treat it as archive input to GSD intel or a one-shot read — do not grow a second living roadmap under `artifacts/plans/`. `ce-setup` does not author `README.md`.

## Research notes (agent memory)

### Leapfrog Triejoin (arXiv:1210.0481v5)

- **Paper:** Todd L. Veldhuizen, *Leapfrog Triejoin: A Simple, Worst-Case Optimal Join Algorithm* (ICDT 2014 / LogicBlox).
- **URL:** https://arxiv.org/html/1210.0481v5
- **Result:** Full conjunctive queries in \(O(Q^*\log M)\) (AGM bound \(Q^*\); \(M\) = largest relation). Hash-table variant reaches \(O(Q^*)\).
- **Kutha:** baseline for hot data-plane join (CSR + Cypher-style multi-hop). Does not change event-log SoT (D1/D2). P0 has `leapfrog_intersect` on sorted rows, not full variable-ordered MATCH.

### Applicability matrix

163 closed cards. Query/Data/Time dense; Security/Packaging thin as *kernels*. Trap cluster (not core): Graphiti, Dify, Hindsight, Harvey-as-SoT, CRDT/Geo-Raft as SoT, empty Cypher, LegalSearch-R1-as-engine. GTM: legal over-mapped on purpose; science present; finance/clinical = receipts/vacuum riders.
