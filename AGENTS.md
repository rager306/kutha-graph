# AGENTS.md — Kutha / kutha-graph

Agent operating notes for this repository. Read this before honeycomb ADRs or the matrix. Chat with the human is **Russian**; this file and all other docs stay **English**.

## Language policy

| Surface | Language |
|---------|----------|
| Chat with the user (questions, summaries, menus, clarifications) | **Russian** |
| Documents (ADR, README, plans, process notes, comments in docs) | **English** |
| Source code, identifiers, commit messages, PR bodies, CI logs | **English** |
| Inline code comments and docstrings | **English** |

Do not mix languages inside a single artifact.

## Two planes (do not collapse)

| Plane | Owns | Must not own |
|-------|------|----------------|
| **Product** (`crates/kutha-*`) | Temporal graph truth: event log = SoT | Roadmap ceremony |
| **Harness** (`scripts/kutha_gov`, `.kutha/`) | Trajectory honesty, freeze, lifecycle non-collapse | Architecture decisions, legal/product readiness |

Governor green ≠ ADR Accepted ≠ capability. Honeycomb **Proposed** ≠ delivery backlog. Literature cards ≠ shipping list.

## Product snapshot

- **Product:** Kutha — hybrid AI-native temporal graph engine (research stage).
- **Idea stack (top → down):** **STCA** (ADR-002) → vision (ADR-001) → locks D1–D10 (ADR-000) → honeycomb cells (ADR-010–093, all **Proposed**).
- **Formula:** event log = SoT; graph = deterministic fold; CSR/HNSW/views = droppable leases; LLM proposes; dictionaries + log own audited truth.
- **Wedge:** legal / normative temporal agents (norm **AS OF** a date). Science next; finance/clinical are receipt/hold riders until a pack exists.
- **P0 spike (in crates, not Accepted product):** log, fold, quantum emit→idle, receipt, snapshot, WAL-cousin, CSR lease, leapfrog intersect, `Materializer` trait, `fork_at`, named `as_of`/`live_at`, FF6 relation allowlist. **Not in spike:** RocksDB crate, Cypher, HNSW, ABAC, legal corpus, ADR-050 six dictionaries.

## Current execution position

Read `.kutha/STATE.md` first (lease, not SoT). **M001 S01–S03 are done**. Harness **H3** is on: process JSONL writes fail-closed against `.kutha/dictionaries/relations.yaml`. Tenant AS OF uses the emitted cut. Next: **H4** (ADR-090 overlay — do not start a legal pack). Do not start M002 Rocks until STATE names it.

Until explicit M002: do not add RocksDB, Cypher/GPML parser, HNSW, ADR-050 six dictionaries, ADR-080/081, full ADR-090/093 packs, ADR-100+, or Consensus Query 103+.

Literature bound is **closed** (163 cards). Do not mint aggregator waves. Matrix: `.compound-engineering/artifacts/research/applicability/` (`cards/` = SoT; `matrix.md` = rollup; `architecture-gtm-readout.md` = GTM translation).

Compound Engineering `docs_root` is `.compound-engineering/artifacts` (set in `.compound-engineering/config.yaml`). Plans, research notes, ideation, and handoffs live there — not under `docs/`. Captured learnings, when a solved problem is written down, go in `.compound-engineering/artifacts/solutions/` (category folders, YAML frontmatter: `module`, `tags`, `problem_type`). That store is relevant when implementing or debugging in a documented area; it is not architecture SoT and not a delivery backlog. `ce-setup` owns config health; it does not author `README.md`.

## Repository layout

```text
kutha-graph/
├── README.md                          # human entry (status, commands, where to read)
├── CLAUDE.md                          # shim → this file (Claude Code)
├── AGENTS.md                          # this file
├── CHANGELOG.md                       # dated product/process history
├── STRATEGY.md                        # product strategy (wedge, metrics, non-goals)
├── Cargo.toml                         # Rust workspace (kutha-common, kutha-runtime)
├── pyproject.toml                     # harness only: Python >=3.13, uv, kutha-gov
├── uv.lock
├── .python-version                    # 3.13
├── .kutha/                            # harness pyramid (process plane)
│   ├── STATE.md                       # lease: active milestone / lifecycles
│   ├── ROADMAP.md                     # M001 slices; honeycomb is a map
│   ├── META.md                        # harness constitution (allowed kinds + FSM kinds)
│   ├── dictionaries/checks.yaml       # governor checks (append a row to add one)
│   ├── dictionaries/fsm.yaml          # CI quantum states/transitions
│   └── events.jsonl                   # process log (gitignored; H0 Time axis)
├── crates/                            # product plane (Rust)
│   ├── kutha-common/                  # Event, Op::{Assert,Retract,Correct,Behavior}, intern, UUID v7
│   └── kutha-runtime/                 # log, fold, quantum, receipt, snapshot, WAL, CSR, LFTJ, allowlist
├── docs/
│   ├── ADR/                           # spine 000–002 + honeycomb 010–093 (all Proposed)
│   ├── architecture/stca-guide.md     # STCA manifesto (do not copy §5 JSON tutorial into harness)
│   └── process/kutha-harness.md       # harness contract (not an ADR)
├── scripts/
│   ├── kutha-gov                      # uv wrapper
│   ├── kutha_gov/                     # harness interpreter (kinds.py; checks are YAML)
│   └── tests/                         # pytest for harness
└── .compound-engineering/
    ├── config.yaml                    # CE team defaults (docs_root)
    └── artifacts/                    # CE docs_root (not product SoT)
        ├── research/applicability/    # 163 cards, matrix, GTM readout
        ├── plans/                     # CE plans (spine-without-sprawl, P0 spikes)
        ├── ideation/                  # ADR crystallization HTML
        ├── handoffs/                 # session continuity snapshots
        └── solutions/                 # ce-compound learnings when captured
```

Do **not** add repo-root `ports/` / `adapters/` / `domain/` (ADR-022: hexagon lives *inside* a slice). Do **not** put Python inside `kutha-runtime`.

## Commands

Product:

```text
cargo test --workspace
```

Harness (Python **3.13** via **uv** only — not system `python3`):

```text
uv run kutha-gov ci          # FSM quantum: relations → checks → observe → emit → tenant → fold
uv run kutha-gov precommit   # dictionary checks only (no cargo, no JSONL); optional --check ID
uv run kutha-gov fsm         # print the process machine
uv run kutha-gov py          # ruff + ty (Astral) + pyrefly (Meta)
uv run kutha-gov fold        # fold .kutha/events.jsonl
uv run kutha-gov list
uv run kutha-gov explain trajectory
uv run pytest
```

Pin: `.python-version`. Copy `.env.example` to `.env` for `KUTHA_GOV_BUDGET` / `KUTHA_GOV_FAIL_ON_WARN` (CLI flags win). Dev tools live in `pyproject.toml` dependency group `dev`. Add a governor check by appending `.kutha/dictionaries/invariants.yaml` (control loop) or `.kutha/dictionaries/bridges.yaml` (cite product), then `.kutha/dictionaries/checks.yaml`; add a CI phase by appending `.kutha/dictionaries/fsm.yaml`. Kutha requirements stay in ADRs / STATE / crates tests. Do not add a Python class. Commit hook: `uvx pre-commit install --overwrite` (`.pre-commit-config.yaml` calls `kutha-gov precommit`, not `ci`).

## Working conventions

1. Honor locked ADR-000 **D1–D10**. Do not revive: pure Samyama product, pure ActiveGraph without hot projections, hard FSM as sole agent control, TypeScript as graph core, RVF as primary storage, Graphiti/Dify/Hindsight as SoT.
2. STCA first. New product detail → honeycomb ADR-010+ (`docs/ADR/README.md`), never silent rewrites of 000/001/002. **Accepted** only when that cell is in the running engine.
3. Honeycomb is a **map**. One steel thread at a time (next: H4 waits on ADR-090; not M002). “Promote all” is forbidden.
4. Prefer falsifiable spikes over generic “build a graph DB” advice.
5. Core stays self-contained Rust (no mandatory external graph DB / Graphiti runtime / LLM for temporal truth).
6. Harness is a **parallel STCA plane** that dogfoods with the engine (`docs/process/kutha-harness.md`). H0 = files + JSONL + **meta-prompt dictionaries + FSM**; H2 = same typed triples on the Kutha log via `kutha-tenant`. Do not clone law-nexus 171-milestone GSD or copy `stca-guide.md` §5 merge-patch runtime. Control loop → check: append `.kutha/dictionaries/invariants.yaml` first (`docs/process/governor-intake.md`). A fence that cites product is `.kutha/dictionaries/bridges.yaml`. Kutha requirements stay in ADRs / STATE / crates tests. New check = YAML row; new CI phase = FSM row; new kind = rare `kinds.py` / `fsm.py` change. Unknown kind → HIGH.
7. Three lifecycles stay orthogonal: **L_map** (ADRs) · **L_delivery** (`.kutha` milestones) · **L_capability** (fitness tests). Bridges may cite; they may not copy state machines.
8. Intern map (ADR-011) ≠ agent dictionaries (ADR-050). Do not collapse them.
9. Humans start at `README.md`. Agents follow this file. Dated history goes in `CHANGELOG.md` (product vs process; do not collapse Trajectory into “the product shipped”).

## Codex subagents

Delegate bounded, independent work to subagents when it improves speed or confidence. The parent owns the current steel thread, scope decisions, integration, and the final Russian response. Small or tightly coupled tasks stay local. Use the workflow in `docs/process/codex-subagents.md`.

- Read `.kutha/STATE.md` and pass the relevant freeze and lifecycle constraints to every helper. Parallelism does not authorize another milestone or a new research wave.
- Prefer `codebase-memory-scout` for provisional discovery, `codebase-memory` for verified questions, `codebase-memory-auditor` for bounded audits, `implementation-worker` for assigned edits, and `correctness-reviewer` for independent review. Use an available built-in role with the same brief if a custom role is missing.
- Before code delegation, provide project/generation, relevant graph queries and pagination, paths/symbols, coverage gaps, source fallback, and unresolved questions. Do not assume a helper has MCP access. Configuration and documentation outside the code graph require exact source evidence and an explicit not-applicable scope.
- Assign disjoint file ownership. Keep product and harness responsibilities explicit, including any cross-plane contract. Workers share the workspace and must preserve other edits. Only the designated integrator changes shared lockfiles, process events, tenant data, or graph indexes, and runs `uv run kutha-gov ci` when required by the task.
- Use up to three helpers, subject to the runtime limit; reuse helpers for related follow-ups. Do not recursively delegate by default. Coordinate cargo/uv checks that share output directories.
- Require changed paths or evidence locations, check results, and limitations in each handoff. The parent validates the combined result; graph coverage, review approval, and governor green retain their distinct meanings.

## Research notes (agent memory)

### Leapfrog Triejoin (arXiv:1210.0481v5)

- **Paper:** Todd L. Veldhuizen, *Leapfrog Triejoin: A Simple, Worst-Case Optimal Join Algorithm* (ICDT 2014 / LogicBlox).
- **URL:** https://arxiv.org/html/1210.0481v5
- **Result:** Full conjunctive queries in \(O(Q^*\log M)\) (AGM fractional edge cover bound \(Q^*\); \(M\) = largest relation). Hash-table variant (Ken Ross) reaches \(O(Q^*)\).
- **Kutha:** baseline for the hot data-plane join path (CSR + Cypher-style multi-hop). Does not change event-log SoT (D1/D2). P0 has `leapfrog_intersect` on sorted rows, not full variable-ordered MATCH.

### Applicability matrix

163 closed cards. Query/Data/Time dense; Security/Packaging thin as *kernels*. Trap cluster (demand high, optimality low — not core): Graphiti, Dify, Hindsight, Harvey-as-SoT, CRDT/Geo-Raft as SoT, empty Cypher, LegalSearch-R1-as-engine. GTM: legal over-mapped on purpose; science present; finance/clinical = receipts/vacuum riders.
