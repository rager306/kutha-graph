---
last_mapped_commit: e77132d9275bd36ea766b8bef9cff28128dfc636
last_mapped_at: 2026-09-29
---
# Codebase Structure

**Analysis Date:** 2026-09-29

## Directory Layout

```text
kutha-graph/
├── AGENTS.md                 # Agent operating notes (English docs; Russian chat)
├── CLAUDE.md                 # Shim → AGENTS.md
├── README.md                 # Human entry
├── STRATEGY.md               # Product strategy
├── CHANGELOG.md              # Dated product/process history
├── Cargo.toml                # Rust workspace (kutha-common, kutha-runtime)
├── Cargo.lock
├── pyproject.toml            # Harness package kutha-harness (Python ≥3.13, uv)
├── uv.lock
├── .python-version           # 3.13
├── .env.example              # Env template (do not commit secrets; .env local)
├── crates/                   # PRODUCT plane
│   ├── kutha-common/         # Event, Op, intern
│   └── kutha-runtime/        # Log, fold, quantum, leases, tenant bin
├── scripts/                  # HARNESS plane (Python package root via setuptools)
│   ├── kutha-gov             # uv wrapper executable
│   ├── kutha_gov/            # Interpreter package
│   └── tests/                # pytest for harness
├── .kutha/                   # Harness pyramid (process lease + dictionaries)
│   ├── STATE.md              # Lease: active milestone / lifecycles
│   ├── ROADMAP.md
│   ├── META.md               # Harness constitution
│   ├── dictionaries/         # checks, fsm, invariants, bridges, relations, honeycomb
│   ├── events.jsonl          # H0 process SoT (gitignored)
│   └── tenant/               # H2 tenant picture (gitignored)
├── docs/
│   ├── ADR/                  # Spine 000–002 + honeycomb 010–093
│   ├── architecture/         # STCA guide, semantic contracts
│   └── process/              # Harness + governor intake (not ADRs)
├── .compound-engineering/    # CE docs_root (plans, research, handoffs, solutions)
├── .planning/codebase/       # GSD codebase maps (this folder)
├── .cursor/                  # Cursor rules, MCP, repo skills (incl. CBM adapter)
└── target/                   # Rust build (generated, not SoT)
```

## Directory Purposes

**`crates/`:**
- Purpose: Product temporal-graph engine (Rust)
- Contains: Workspace members only — `kutha-common`, `kutha-runtime`
- Key files: `crates/kutha-runtime/src/lib.rs`, `crates/kutha-common/src/event.rs`, `crates/kutha-runtime/src/bin/kutha-tenant.rs`

**`crates/kutha-common/src/`:**
- Purpose: Shared encoding for the P0 heartbeat
- Contains: `lib.rs`, `event.rs`, `intern.rs`
- Key files: `event.rs` (`Op`, `Event`), `intern.rs` (`TermDictionary`)

**`crates/kutha-runtime/src/`:**
- Purpose: In-memory runtime: log = SoT, fold = picture, quantum = emit→idle
- Contains: One module per concern (see Naming); `bin/kutha-tenant.rs`
- Key files: `quantum.rs`, `fold.rs`, `log.rs`, `store.rs`, `tenant.rs`, `materializer.rs`, `csr.rs`, `leapfrog.rs`, `allow.rs`

**`crates/kutha-runtime/tests/`:**
- Purpose: Integration / fitness tests (FF + dogfood + milestone slices)
- Contains: `ff5_legal_pit.rs`, `ff6_allowlist.rs`, `h2_harness_tenant.rs`, `h4_process_allows.rs`, `m010_semantic_open.rs`, `m011_claim_supports.rs`, `fixtures/`
- Key files: Named tests required by FSM `observe_cargo.required` in `.kutha/dictionaries/fsm.yaml`

**`crates/kutha-runtime/dictionaries/`:**
- Purpose: Product relation allowlist (plane-isolated from process)
- Contains: `relations.yaml` (`kutha-relations/v1`)
- Key files: `relations.yaml`

**`scripts/kutha_gov/`:**
- Purpose: Harness interpreter (composition root for process STCA)
- Contains: CLI, FSM, kinds, dictionary loaders, adapters (`observe`, `tenant`, `gitdiff`, `time_log`, `honeycomb`)
- Key files: `__main__.py`, `fsm.py`, `kinds.py`, `dictionary.py`, `protocol.py`, `time_log.py`, `checks/__init__.py` (re-exports `get_checks` only)

**`scripts/tests/`:**
- Purpose: pytest for harness
- Contains: `test_kutha_gov.py`, `test_fsm.py`
- Key files: same

**`.kutha/`:**
- Purpose: Process pyramid — lease files + meta-prompt dictionaries + H0/H2 storage paths
- Contains: Markdown leases, YAML dictionaries, gitignored JSONL/tenant
- Key files: `STATE.md`, `META.md`, `dictionaries/checks.yaml`, `dictionaries/fsm.yaml`

**`docs/ADR/`:**
- Purpose: Architecture decision map (Proposed ≠ Accepted ≠ backlog)
- Contains: ADR-000–002 spine, ADR-010–093 honeycomb, `README.md` index
- Key files: `README.md`, `ADR-000-kutha-hybrid-architecture-research.md`, `ADR-010-event-log-runtime-quantum.md`

**`docs/process/`:**
- Purpose: Harness and intake contracts (not ADRs)
- Contains: `kutha-harness.md`, `governor-intake.md`, `codex-subagents.md`
- Key files: `kutha-harness.md`

**`docs/architecture/`:**
- Purpose: STCA manifesto and cross-cell contracts
- Contains: `stca-guide.md`, `semantic-contract-validation.md`
- Key files: `stca-guide.md` (do not copy §5 JSON tutorial into harness)

**`.compound-engineering/artifacts/`:**
- Purpose: CE durable outputs (`docs_root`)
- Contains: `plans/`, `research/applicability/` (163 cards), `handoffs/`, `solutions/`, `ideation/`
- Key files: `research/applicability/matrix.md`, plans under `artifacts/plans/`

**`.cursor/`:**
- Purpose: Editor/agent tooling for this repo
- Contains: rules (`code-graph-cbm.mdc`, `ce-skills-ru.mdc`), skills (`codebase-memory`, `kutha-changelog`, GSD mirrors), MCP config
- Key files: `.cursor/skills/codebase-memory/SKILL.md`, `.cursor/mcp.json`

**`.planning/codebase/`:**
- Purpose: GSD map-codebase outputs for plan/execute phases
- Contains: `ARCHITECTURE.md`, `STRUCTURE.md`, and sibling maps from other focuses
- Generated: Written by `gsd-codebase-mapper`; commit when the orchestrator chooses
- Committed: Team choice; not product SoT

## Key File Locations

**Entry Points:**
- `scripts/kutha_gov/__main__.py`: Harness CLI (`uv run kutha-gov …`)
- `crates/kutha-runtime/src/bin/kutha-tenant.rs`: H2 tenant ingest binary
- `crates/kutha-runtime/src/lib.rs`: Product library surface
- `scripts/kutha-gov`: Thin uv wrapper to the package script

**Configuration:**
- `Cargo.toml`: Workspace members and shared deps
- `pyproject.toml`: Harness package, pytest/ruff/ty, `package-dir = scripts`
- `.kutha/dictionaries/*.yaml`: Process checks, FSM, allowlists, map index
- `crates/kutha-runtime/dictionaries/relations.yaml`: Product allowlist
- `.compound-engineering/config.yaml`: CE `docs_root`
- `.env.example` / local `.env`: `KUTHA_GOV_*`, `KUTHA_MAX_CASCADE`, `KUTHA_RELATIONS_PATH`, tenant paths (existence only — never commit secrets)

**Core Logic:**
- `crates/kutha-runtime/src/quantum.rs`: Runtime emit / cascade / replay / fork
- `crates/kutha-runtime/src/fold.rs`: Deterministic fold and temporal cuts
- `crates/kutha-common/src/event.rs`: Ops and events
- `scripts/kutha_gov/fsm.py`: Process quantum
- `scripts/kutha_gov/kinds.py`: Check kind runners
- `scripts/kutha_gov/time_log.py`: Process JSONL append/fold

**Testing:**
- `crates/kutha-runtime/tests/`: Product integration / fitness
- Inline `#[cfg(test)]` modules in runtime sources (e.g. `quantum.rs`, `materializer.rs`)
- `scripts/tests/`: Harness pytest
- Run: `cargo test --workspace` · `uv run pytest` · `uv run kutha-gov ci`

## Naming Conventions

**Files:**
- Rust modules: `snake_case.rs` matching concern (`fold.rs`, `leapfrog.rs`)
- Rust bins: `kebab-case` binary name → `src/bin/kutha-tenant.rs`
- Python modules: `snake_case.py` (`time_log.py`, `process_allow.py`)
- ADRs: `ADR-NNN-kebab-title.md` under `docs/ADR/`
- Fitness / slice tests: `ffN_*.rs`, `hN_*.rs`, `mNNN_*.rs`
- Process dictionaries: plural noun `.yaml` (`checks.yaml`, `bridges.yaml`)

**Directories:**
- Crates: `kutha-<role>` (`kutha-common`, `kutha-runtime`)
- Python package: `kutha_gov` (underscore) under `scripts/`
- Do **not** create repo-root `ports/`, `adapters/`, or `domain/`

**Symbols (prescriptive):**
- Rust types: `PascalCase` (`Runtime`, `GraphFold`, `CsrLease`)
- Rust functions/methods: `snake_case` (`emit`, `live_at`, `fork_at`, `leapfrog_intersect`)
- Python: `snake_case` functions; dataclasses `PascalCase` (`Machine`, `Finding`, `Context`)

## Where to Add New Code

**New product runtime capability (leased slice only):**
- Primary code: new or existing module under `crates/kutha-runtime/src/`; export from `lib.rs` when public
- Shared schema change: `crates/kutha-common/src/` (keep intern map ≠ agent dictionaries)
- Product allowlist member: append `crates/kutha-runtime/dictionaries/relations.yaml`
- Tests: prefer `crates/kutha-runtime/tests/<slice_or_ff>.rs`; add required name to FSM `observe_cargo.required` only when it is a harness evidence gate
- Persist/recovery: `store.rs` / `wal.rs` / `snapshot.rs` — keep WAL as durability cousin, not Rocks-as-SoT

**New harness check:**
- Intake row: `.kutha/dictionaries/invariants.yaml` (control loop) or `bridges.yaml` (cites product)
- Check registration: `.kutha/dictionaries/checks.yaml`
- Runner: reuse a kind in `scripts/kutha_gov/kinds.py`; add to `ALLOWED_KINDS` only for a new kind
- Do not add a Python Check subclass tree under `checks/`

**New FSM phase:**
- Append state/transition in `.kutha/dictionaries/fsm.yaml`
- Implement kind in `fsm.py` `ALLOWED_STATE_KINDS` + `_execute` only if kind is new

**New honeycomb cell (design map only):**
- ADR file in `docs/ADR/`; index in `docs/ADR/README.md`
- Compact index row in `.kutha/dictionaries/honeycomb.yaml`
- Does **not** authorize delivery — need Active Slice in `.kutha/STATE.md`

**New CE plan / handoff / learning:**
- Plan: `.compound-engineering/artifacts/plans/`
- Handoff: `.compound-engineering/artifacts/handoffs/<topic>.md` (not `/tmp`)
- Solution: `.compound-engineering/artifacts/solutions/<category>/` with YAML frontmatter

**Utilities:**
- Product helpers: private `fn` in the owning module under `crates/kutha-runtime/src/`
- Harness helpers: module beside the slice (`gitdiff.py`, `process_allow.py`) — not a global `scripts/ports/`

## Special Directories

**`target/`:**
- Purpose: Cargo build artifacts
- Generated: Yes
- Committed: No

**`.kutha/events.jsonl` / `.kutha/tenant/`:**
- Purpose: H0 process log SoT; H2 tenant persist picture
- Generated: Yes (runtime of harness/tenant)
- Committed: No (gitignored)

**`.venv/` / `.pytest_cache/`:**
- Purpose: Local Python tooling
- Generated: Yes
- Committed: No

**`.compound-engineering/artifacts/research/applicability/`:**
- Purpose: Closed 163-card applicability matrix (SoT for cards)
- Generated: No (authored research)
- Committed: Yes when team knowledge

**`.gitnexus/` / CBM index stores:**
- Purpose: Secondary / tool indexes (not product SoT)
- Generated: Yes
- Committed: Local/tooling policy; do not treat as architecture authority

**Frozen / do not add until STATE leases:**
- RocksDB crate, Cypher parser crate, HNSW, legal pack tree, ADR-100+ — keep out of `crates/` until `.kutha/STATE.md` names the work

---

*Structure analysis: 2026-09-29*
