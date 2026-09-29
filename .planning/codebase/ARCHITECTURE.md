---
last_mapped_commit: e77132d9275bd36ea766b8bef9cff28128dfc636
last_mapped_at: 2026-09-29
---
<!-- refreshed: 2026-09-29 -->

# Architecture

**Analysis Date:** 2026-09-29

## System Overview

```text
┌─────────────────────────────────────────────────────────────────────────────┐
│  Idea stack (map, not runtime)                                              │
│  STCA ADR-002 → Vision ADR-001 → Locks D1–D10 ADR-000 → Honeycomb 010–093 │
│  `docs/ADR/`                                                                │
└───────────────────────────────────┬─────────────────────────────────────────┘
                                    │ cites / does not authorize delivery
          ┌─────────────────────────┴─────────────────────────┐
          ▼                                                   ▼
┌──────────────────────────────┐               ┌──────────────────────────────┐
│  PRODUCT PLANE               │               │  HARNESS PLANE               │
│  Temporal graph truth        │               │  Trajectory / freeze / CI    │
├──────────────────────────────┤               ├──────────────────────────────┤
│  kutha-common                │               │  kutha-gov CLI               │
│  `crates/kutha-common/`      │◄──H2 tenant───│  `scripts/kutha_gov/`        │
│  Event, Op, TermDictionary   │               │  dictionary FSM + checks     │
├──────────────────────────────┤               ├──────────────────────────────┤
│  kutha-runtime               │               │  .kutha/ pyramid             │
│  `crates/kutha-runtime/`     │               │  STATE lease, dictionaries,  │
│  log → fold → quantum        │               │  events.jsonl (H0 SoT)       │
│  CSR lease, WAL, allowlist   │               │  tenant/ (gitignored H2)     │
└──────────────┬───────────────┘               └──────────────┬───────────────┘
               │                                              │
               ▼                                              ▼
┌──────────────────────────────┐               ┌──────────────────────────────┐
│  EventLog = product SoT      │               │  Process JSONL = H0 SoT      │
│  GraphFold = droppable pic   │               │  STATE.md = lease, not SoT   │
│  store::persist / WAL cousin │               │  fold_log → last-run picture │
└──────────────────────────────┘               └──────────────────────────────┘
```

## Component Responsibilities

| Component | Responsibility | File |
|-----------|----------------|------|
| `kutha-common` | Lean event schema, ops, intern map | `crates/kutha-common/src/` |
| `Event` / `Op` | UUID-v7 events; Assert/Retract/Correct/Behavior/Define | `crates/kutha-common/src/event.rs` |
| `TermDictionary` | Hot-path `TermId` ↔ string lease (≠ ADR-050 agent dicts) | `crates/kutha-common/src/intern.rs` |
| `Runtime` | Quantum emit→idle; admit; cascade; replay; fork | `crates/kutha-runtime/src/quantum.rs` |
| `EventLog` | Append-only product SoT | `crates/kutha-runtime/src/log.rs` |
| `GraphFold` | Deterministic fold; bi-temporal `live_at` / `as_of` | `crates/kutha-runtime/src/fold.rs` |
| `CsrLease` | Droppable CSR adjacency view | `crates/kutha-runtime/src/csr.rs` |
| `Materializer` | Reversible view protocol (CSR impl) | `crates/kutha-runtime/src/materializer.rs` |
| `leapfrog_intersect` | Sorted-row LFTJ cousin (P0, not full MATCH) | `crates/kutha-runtime/src/leapfrog.rs` |
| Relation allowlist | Fail-closed product relations (FF6) | `crates/kutha-runtime/src/allow.rs`, `crates/kutha-runtime/dictionaries/relations.yaml` |
| `store` / `wal` | File persist + WAL durability cousin (not Rocks-as-SoT) | `crates/kutha-runtime/src/store.rs`, `wal.rs` |
| `kutha-tenant` | H2: ingest harness JSONL onto product log | `crates/kutha-runtime/src/bin/kutha-tenant.rs`, `tenant.rs` |
| `kutha-gov` | Harness composition root (Python 3.13) | `scripts/kutha_gov/__main__.py` |
| FSM interpreter | Load `.kutha/dictionaries/fsm.yaml`; run quantum | `scripts/kutha_gov/fsm.py` |
| Kind runners | Closed check-kind set; YAML rows add checks | `scripts/kutha_gov/kinds.py` |
| Process time log | Append/fold `.kutha/events.jsonl` | `scripts/kutha_gov/time_log.py` |
| Honeycomb map index | Compact L_map (`kutha-gov map`) | `scripts/kutha_gov/honeycomb.py`, `.kutha/dictionaries/honeycomb.yaml` |
| ADRs | Design map / locks / honeycomb cells | `docs/ADR/` |
| CE artifacts | Plans, research, handoffs (not product SoT) | `.compound-engineering/artifacts/` |

## Pattern Overview

**Overall:** Dual-plane STCA — product event-sourced temporal graph + parallel harness control plane that dogfoods the same paradigm.

**Key Characteristics:**
- **Event log = SoT; graph = fold.** Never treat `GraphFold`, CSR, snapshots, or `STATE.md` as authoritative truth.
- **Two planes stay orthogonal.** Product lives in `crates/kutha-*`. Harness lives in `scripts/kutha_gov` + `.kutha/`. Do not put Python inside `kutha-runtime`. Do not grow repo-root `ports/` / `adapters/` / `domain/` (ADR-022: hexagon lives inside a slice).
- **Fail-closed dictionaries.** Unknown product relation → reject emit. Unknown process relation / check kind / FSM state kind → HIGH finding. New check = YAML row; new kind = rare Python kernel change.
- **Honeycomb is a map.** ADR-010–093 Proposed cells are not a delivery backlog. Active work is leased in `.kutha/STATE.md` (`L_delivery`).
- **Three lifecycles do not collapse:** `L_map` (ADRs) · `L_delivery` (milestones) · `L_capability` (fitness tests). Governor green ≠ ADR Accepted ≠ capability.

## Layers

**Idea / map layer:**
- Purpose: Name decisions and coordinates without shipping authority
- Location: `docs/ADR/`, `docs/architecture/stca-guide.md`
- Contains: Spine ADR-000–002, honeycomb ADR-010–093 (all Proposed until runtime implements the cell)
- Depends on: Nothing in crates
- Used by: Agents and humans for framing; harness may *cite* via bridges, never copy ADR Status into process FSM

**Product encoding layer (`kutha-common`):**
- Purpose: Shared lean schema for the heartbeat
- Location: `crates/kutha-common/src/`
- Contains: `Event`, `Op`, `TermId`, `TermDictionary`, digests
- Depends on: `uuid`, `sha2`, `serde`
- Used by: `kutha-runtime` only (workspace path dep)

**Product runtime layer (`kutha-runtime`):**
- Purpose: Log, fold, quantum, leases, persist, tenant ingest
- Location: `crates/kutha-runtime/src/`
- Contains: Modules listed in `lib.rs`; integration tests under `tests/`; product allowlist YAML
- Depends on: `kutha-common`, workspace `sha2` / `serde` / `crc32fast`
- Used by: `cargo test`, `kutha-tenant` binary, harness observe subprocess (evidence only)

**Harness interpreter layer (`kutha_gov`):**
- Purpose: Repository control plane — trajectory honesty, freeze, lifecycle non-collapse
- Location: `scripts/kutha_gov/`
- Contains: CLI, FSM, kinds, dictionary loaders, time_log, tenant/observe adapters
- Depends on: PyYAML; subprocess to `cargo` / `kutha-tenant`; does **not** import `kutha_runtime` as a library
- Used by: `uv run kutha-gov`, pre-commit (`kutha-gov precommit`)

**Harness state layer (`.kutha/`):**
- Purpose: Process constitution, dictionaries, H0 log, lease files
- Location: `.kutha/`
- Contains: `META.md`, `STATE.md`, `ROADMAP.md`, `dictionaries/*.yaml`, `events.jsonl` (gitignored), `tenant/` (gitignored)
- Depends on: Interpreted by `kutha_gov`
- Used by: CI quantum; humans editing lease text under freeze rules

**Compound Engineering layer:**
- Purpose: Durable session/process docs under CE `docs_root`
- Location: `.compound-engineering/` (`config.yaml` sets `docs_root: .compound-engineering/artifacts`)
- Contains: plans, research/applicability (163 cards), handoffs, solutions
- Depends on: Not product SoT
- Used by: CE skills (`ce-plan`, `ce-work`, handoffs); not governor checks as architecture authority

## Data Flow

### Primary product path (emit → fold → query cut)

1. Caller builds `Op` / calls `Runtime::intern` then `Runtime::emit` (`crates/kutha-runtime/src/quantum.rs`).
2. Admit checks relation allowlist (`allow.rs` + `dictionaries/relations.yaml`); unknown relation → `RuntimeError::UnknownRelation` (no append).
3. Claim/lineage gates (M011): unknown `claim` / broken `caused_by` → fail-closed without append.
4. Event appends to `EventLog`; `GraphFold::apply` updates the droppable picture (`fold.rs`).
5. Cascade `Behavior` follow-ons (e.g. `knows` → `knownBy`) until idle or `max_cascade` (`KUTHA_MAX_CASCADE`, default 32).
6. Quantum returns `QuantumReceipt` (`receipt.rs`).
7. Temporal read: `fold.as_of(vt)` or `fold.live_at(tt, vt)` — no implicit “now” (FF5).
8. Optional lease: `CsrLease::from_fold` / `CsrMaterializer::build`; drop anytime and rebuild from fold.
9. Optional persist: `store::persist` writes `events.jsonl` + WAL + snapshot + derived `terms.jsonl` under a directory (tenant uses `.kutha/tenant/`).

### Harness CI quantum (`kutha-gov ci`)

1. CLI `main` detects repo root (requires `.kutha` + `docs/ADR`) (`scripts/kutha_gov/__main__.py`).
2. `run_quantum` loads FSM from `.kutha/dictionaries/fsm.yaml` (`fsm.py`).
3. States walk: require dictionary files → `run_checks` (YAML-driven via `dictionary.py` + `kinds.py`) → `observe_cargo` (`observe.py`, evidence) → `emit_log` / membership sync (`time_log.py`) → `emit_tenant` (`tenant.py` → built `kutha-tenant`) → `fold_log` → `decide` → terminal `ok`/`fail`.
4. Process SoT appends to `.kutha/events.jsonl`; `fold_log` reproduces last-run picture.
5. H2/H4: tenant maps process relations onto product ops (`tenant.rs`); tip→JSONL membership sync on emit; AS OF cuts on tenant picture.

### H2 tenant bridge

1. `kutha-tenant` reads `KUTHA_HARNESS_LOG` (default `.kutha/events.jsonl`) (`kutha-tenant.rs`).
2. `ingest_harness_jsonl` maps process rows onto product asserts (`tenant.rs`).
3. `replay_check` then `store::persist` to `KUTHA_TENANT_DIR`.
4. Harness treats tenant output as evidence/cut, not architecture SoT.

**State Management:**
- Product: mutable `Runtime` owns log + fold + dict; snapshots/CSR are leases.
- Harness: append-only JSONL + folded last-run; `STATE.md` is a **lease** of intent, not process SoT.
- No shared in-process product/harness memory; H2 crosses planes via files + binary.

## Key Abstractions

**`Op` / `Event`:**
- Purpose: Lean write operators and UUID-v7 log records (ADR-011)
- Examples: `crates/kutha-common/src/event.rs`
- Pattern: Assert carries optional portable `claim`; Behavior carries `caused_by`; Define is fold no-op for term meanings on the durable log

**`GraphFold` / `Fact`:**
- Purpose: Deterministic picture with bi-temporal liveness and claim supports
- Examples: `crates/kutha-runtime/src/fold.rs`
- Pattern: Losers stay on retract; `claim_id` may be shared across Facts; `as_of` = valid-time on `tt = MAX`

**`Runtime` quantum:**
- Purpose: Emit→cascade→idle with receipt and replay fingerprint
- Examples: `crates/kutha-runtime/src/quantum.rs`
- Pattern: Single composition object for the in-memory P0 engine; `fork_at(n)` replays prefix without shared tentatives

**`Materializer` / `CsrLease`:**
- Purpose: Droppable hot-path views (ADR-040/041)
- Examples: `materializer.rs`, `csr.rs`
- Pattern: Build from fold at explicit `(tt, vt)`; unload clears lease; never write views back as SoT

**Dictionary-driven checks / FSM:**
- Purpose: Meta-prompt control without Python Check subclasses as intake
- Examples: `.kutha/dictionaries/{checks,fsm,invariants,bridges,relations,honeycomb}.yaml`, `kinds.py`, `fsm.py`
- Pattern: YAML owns lists; Python owns closed kind interpreters; unknown kind → HIGH

**Process vs product allowlists:**
- Purpose: Plane isolation for relation membership
- Examples: `.kutha/dictionaries/relations.yaml` vs `crates/kutha-runtime/dictionaries/relations.yaml`
- Pattern: Mixing schemas is a HIGH `plane-mix` finding; do not unify into one file

## Entry Points

**`cargo test --workspace`:**
- Location: crate unit tests + `crates/kutha-runtime/tests/*.rs`
- Triggers: Developer / harness `observe_cargo`
- Responsibilities: Product fitness (FF5/FF6, H2/H4, M010/M011)

**`kutha-tenant` binary:**
- Location: `crates/kutha-runtime/src/bin/kutha-tenant.rs`
- Triggers: FSM `emit_tenant`; env `KUTHA_HARNESS_LOG`, `KUTHA_TENANT_DIR`, optional `KUTHA_TENANT_BIN`
- Responsibilities: Ingest process JSONL → product Runtime → persist tenant dir

**`uv run kutha-gov`:**
- Location: `scripts/kutha_gov/__main__.py` (script entry in `pyproject.toml`)
- Triggers: Humans, pre-commit (`precommit`), CI (`ci`)
- Responsibilities: list / explain / fsm / map / ci / precommit / py / fold

**Library API (tests and future bins):**
- Location: `kutha_runtime::Runtime` re-exports in `crates/kutha-runtime/src/lib.rs`
- Triggers: Integration tests, future Rust harness bin (composition root only)
- Responsibilities: Emit, fold cuts, persist/open, materialize CSR

## Architectural Constraints

- **Threading:** Single-threaded in-memory Runtime; no async runtime required for P0. Harness uses subprocess timeouts for cargo/tenant.
- **Global state:** Product allowlist + dotenv load via `Once` in `allow.rs`. Harness Context is per-run. No process-wide product singleton.
- **Circular imports:** Not applicable across Rust crates (`kutha-runtime` → `kutha-common` only). Python package is flat modules under `kutha_gov` without a ports tree.
- **Freeze (until STATE leases M002):** No RocksDB crate, Cypher/GPML parser, HNSW, ADR-050 six dictionaries, ADR-080/081, full ADR-090/093 packs, ADR-100+, Consensus Query 103+.
- **Plane isolation:** Checks must not import `kutha_runtime`. H1 observes via `cargo test` subprocess. H2 talks only through built `kutha-tenant` API surface.
- **Lifecycle non-collapse:** Do not encode honeycomb as the active milestone; do not treat governor green as `L_capability`.

## Anti-Patterns

### Collapse planes or SoT

**What happens:** Treat fold/CSR/`STATE.md`/tenant dir as product truth, or put Python inside `kutha-runtime`.
**Why it's wrong:** Breaks D1/D2 (log = SoT) and harness isolation laws; pollutes the graph crate with process ceremony.
**Do this instead:** Append to the correct log (`EventLog` or `.kutha/events.jsonl`); keep leases droppable; keep harness in `scripts/kutha_gov`.

### Repo-root hexagon sprawl

**What happens:** Add top-level `ports/`, `adapters/`, `domain/`.
**Why it's wrong:** Violates ADR-022 (hexagon inside a slice); fragments cohesion.
**Do this instead:** Keep adapters next to the harness slice that uses them (`observe.py`, `tenant.py`, filesystem reads of `.kutha/*.md`).

### Promote honeycomb as backlog

**What happens:** Open ADR-100+ or implement Cypher/HNSW/Rocks because a cell exists in `docs/ADR/README.md`.
**Why it's wrong:** Honeycomb is a map; delivery requires an Active Slice lease in `.kutha/STATE.md`.
**Do this instead:** Extend only the leased steel thread; cite cells from bridges without copying their state machines.

### New check as Python subclass

**What happens:** Add a Check class instead of a YAML row.
**Why it's wrong:** Breaks meta-prompt intake (`docs/process/governor-intake.md`); drifts from dictionary SoT for check lists.
**Do this instead:** Append `.kutha/dictionaries/invariants.yaml` or `bridges.yaml`, then a row in `checks.yaml`. Add a kind in `kinds.py` only when the closed set must grow.

### Silent “now” temporal defaults

**What happens:** Query APIs that imply wall-clock “current” without named `as_of` / `live_at`.
**Why it's wrong:** Breaks FF5 / MemStrata-style explicit cuts.
**Do this instead:** Pass explicit `(tt, vt)` or named `as_of(vt)` on `GraphFold`.

## Error Handling

**Strategy:** Typed fail-closed errors on product admit; findings with severity on harness checks; exit codes on CLIs.

**Patterns:**
- Product: `RuntimeError` variants (`ReplayDivergence`, `UnknownFact`, `UnknownRelation`, `UnknownClaim`, `BrokenLineage`) in `quantum.rs` — reject append where specified.
- Harness: `Finding` + `Severity` (`HIGH` fails CI; `LOW` optional via `KUTHA_GOV_FAIL_ON_WARN` / `--fail-on-warn`) in `protocol.py`.
- Tenant/CLI: `ExitCode` 0/1/2; stderr messages mark evidence vs SoT explicitly.
- Unknown FSM state kind or check kind → HIGH / fail, not skip.

## Cross-Cutting Concerns

**Logging:** Product has no structured logger in P0; harness prints check status lines and appends typed JSONL rows. Prefer append-only observations over mutating docs.

**Validation:** Schema strings on dictionaries (`kutha-relations/v1`, `kutha-harness-fsm/v1`, etc.); allowlists; FSM terminal decide.

**Authentication:** Not applicable in P0 spike (no ABAC / multi-tenant product security — ADR-080 frozen).

**Budgets:** Product cascade `KUTHA_MAX_CASCADE`; harness `--budget` / FSM `defaults.budget` truncates check slices (Cui V cousin, not pack max-convolution yet).

---

*Architecture analysis: 2026-09-29*
