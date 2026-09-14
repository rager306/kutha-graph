# Kutha harness (parallel control plane)

Lifecycle: process contract, not an ADR.  
Authority: none over D1–D10, honeycomb cells, legal/product readiness, or GSD-style Attempts.  
Companion: `.kutha/STATE.md`, `.kutha/ROADMAP.md`, `scripts/kutha_gov/`.

## What this is

A **parallel harness** that co-evolves with the engine (**dogfood**). It is not the event-log SoT, not a second graph product, and not a clone of law-nexus / daily-archive / reactivegraph installations.

Neighbors proved the same split:

| Plane | Owns | Must not own |
|-------|------|----------------|
| **Product** (`crates/kutha-*`) | Temporal graph truth | Roadmap ceremony |
| **Harness** (`scripts/kutha_gov`, `.kutha/`) | Trajectory honesty, freeze, lifecycle non-collapse | Architecture decisions |

Governor green ≠ capability. Milestone complete ≠ ADR Accepted. ADR Proposed ≠ backlog.

## Pyramid (execution, not honeycomb)

```text
North star     STRATEGY + ADR-001 / STCA
  Milestone    shippable probe (one active)
    Slice      vertical demo (steel thread)
      Task     one context window
```

Honeycomb ADR-010–093 is a **map**. The harness fails if STATE treats the map as the active milestone.

## Three lifecycles (do not collapse)

| ID | Name | Terminal success |
|----|------|------------------|
| **L_map** | Design map (ADRs, matrix) | Cell named and Proposed/Accepted *as a document* |
| **L_delivery** | Harness milestone / slice | STATE + ROADMAP + checks agree |
| **L_capability** | Product proof (fitness functions) | Named test green (e.g. FF5 legal PIT) |

Bridges may cite; they may not copy state machines (law-nexus continuity contract, adapted).

## Dogfood ladder (harness grows with the engine)

Each rung is a harness capability that **uses a newly real product surface**. Do not skip to “trajectory graph in Kutha” before H2 is wired.

| Rung | Harness does | Unlocks when |
|------|----------------|--------------|
| **H0** | File trajectory + dictionary FSM: STATE ↔ ROADMAP, ADR vocabulary, freeze, one active milestone/slice | Now |
| **H1** | Evidence: `cargo test` / named FF tests as observations (not SoT) | **Now** (FF5/FF6 in crates) |
| **H2** | Assert harness events onto the **Kutha log** (delivery facts, not product norms); query **AS OF** the process | **Now** (FF5 green + `kutha-tenant`) |
| **H3** | Fail-closed writes through a **relation allowlist** (stub of ADR-050) | **Now** (process JSONL + `.kutha/dictionaries/relations.yaml`) |
| **H4** | Legal pack dictionaries version the *process* rules the same way as norms | ADR-090 overlay + H2 |

H2 is the Kutha-specific dogfood the neighbors cannot do with markdown alone: the control plane becomes a **tenant of the engine**, still not architecture authority.

## STCA applied to the harness (same paradigm, second plane)

The engine is STCA. The harness is **also** STCA — a parallel spatiotemporal context — not a bag of scripts. It must not copy the tutorial `ActiveRuntime` in `docs/architecture/stca-guide.md` §5 (JSON `object.created` / merge-patch). Product and process both use **typed assert/retract** (`kutha_common::Op` at H2; lean JSONL twins at H0).

```
  [ SPACE ]                              [ TIME ]
  Check slices + Ports inside            .kutha/events.jsonl (H0)
  (trajectory, freeze, adr, …)           → kutha-runtime log (H2 tenant)
           │                                      │
           ▼                                      ▼
  File/Git adapters (H0)                 Fold = last run picture
           │                                      │
           └──────────────────┬───────────────────┘
                              ▼
                 [ harness spatiotemporal context ]
                              ▲
                              │
                 Cui V = --budget (H0 truncate;
                 max-convolution when packs compete)
```

Nakajima still holds: **graph = world, behaviors = physics, log = proof.** For this plane the “graph” is trajectory + freeze + lifecycle facts; STATE.md is a **lease**.

### Space (Drotbohm / ADR-022)

| Slice (vertical) | Port (inside the slice) | Adapter now | Must not |
|------------------|-------------------------|-------------|----------|
| trajectory | StatePort, RoadmapPort | filesystem `.kutha/*.md` + YAML row | global `scripts/ports/` |
| lifecycles | StatePort | same | collapse L_* into one enum |
| adr-status | AdrCorpusPort | `docs/ADR/*.md` | rewrite ADR Status |
| freeze | CargoPort | `crates/*/Cargo.toml` (bridge cites STATE freeze) | product deps as SoT |
| honeycomb-map | IntentPort | STATE+ROADMAP text | treat honeycomb as M001 |
| dogfood | ManifestPort | `docs/process/kutha-harness.md` | mix into `kutha-runtime` |
| meta-prompt | ConstitutionPort | `.kutha/META.md` + dictionaries | Python Check subclasses as intake |
| observe-required-fn | EvidencePort | FSM `required` list → `fn` in `crates/**/*.rs` | duplicate name lists / cargo green as SoT |
| plane-mix-dicts | PlanePort | process vs product relation schemas | one allowlist for both planes |
| tenant-bin | CutPort | built `kutha-tenant`; `KUTHA_TENANT_BIN` | `cargo run` as a second compile |
| unnamed-csr | CutPort | `csr_lease_at` only (bridge cites crates) | silent `csr_lease()` now |
| harness-relations | ProcessAllowPort | `.kutha/dictionaries/relations.yaml` | log unknown process relations |
| invariants-ledger | IntakePort | `.kutha/dictionaries/invariants.yaml` | honeycomb cells / L_capability as process dispositions |
| bridges-ledger | BridgePort | `.kutha/dictionaries/bridges.yaml` | copy ADR Status or fitness into the harness |

Hexagon lives **inside** a slice (ADR-022). Do not grow a repo-root `ports/` / `adapters/` / `domain/` tree — that is the cohesion failure the manifesto forbids. Composition root is `uv run kutha-gov` (Python 3.13). Later a Rust `kutha-harness` bin that **only** wires adapters. Checks are Behaviors: they propose findings; they do not mutate ADRs.

### Time (ActiveGraph / ADR-010, typed)

1. Human **asserts** intent (active milestone) in STATE — until H2 this is a typed fact on the Kutha log.
2. Each `ci` quantum **emits** `harness.run` triples (and H1 `harness.observe` evidence) into `.kutha/events.jsonl` (append-only).
3. `kutha_gov fold` is \(G = \mathrm{foldl}(\mathrm{apply}, G_0, L)\).
4. Replay of the JSONL must reproduce the same last-run picture (process Strict Replay). Fork-and-diff of process history waits for product `fork_at` at H2.

Relation Behaviors (later): `finding --raisedOn--> check` triggers lifecycle warn; `milestone --blocks--> freeze` keeps Rocks out. Not a workflow engine.

### Composition (Cui)

`--budget V` is H0 truncation (one resource: number of slices). Real max-convolution is when trajectory vs freeze vs adr packs **compete** for the same cascade budget on the product runtime (ADR-030/031). Do not encode a GSD/workflow DAG as composition.

### Verification (guide §4, Kutha-shaped)

| Guide | Harness now | Product |
|-------|-------------|---------|
| Strict Replay | fold JSONL twice → same picture | `ReplayDivergenceError` on event log |
| Fork-and-diff | not yet | `fork_at` in kutha-runtime |
| Regimes gated loop | **off** (ADR-062 optional) | never auto-patch D1–D10 |

### Isolation laws (guide §3)

1. Domain ignorance: check logic does not import `kutha_runtime`. H1 observes via a `cargo test` subprocess (evidence, not SoT). H2 talks **only** through the runtime API (`kutha-tenant` → `emit` / `as_of` / `store::persist`).
2. Private-by-default: kind interpreters stay in `kinds.py`; check *lists* stay in the dictionary.
3. One composition root: CLI / future `kutha-harness` bin. Python stays repository-control (neighbors’ ADR-0007 lesson), never inside the graph crate.

### Mapping H0 records → Kutha `Op` (H2)

```text
assert  subject=harness.run      relation=runStatus   object=ok|fail
assert  subject=harness.observe  relation=observed    object=ok|fail
```

Windows: successive status rows close `valid_to` at the next monotonic valid-from. Same unix second is legal on the process JSONL; the tenant **bumps** the emitted cut so AS OF last status is live (`[from, to)`). `last_valid_from` is that emitted cut, not the source `valid_from`. Tenant ingest maps only `status`→`runStatus` and `cargo`→`observed`; `high`/`checks` stay on the process JSONL. Per-test FF names are CLI evidence, not process relations. Tenant picture is `.kutha/tenant/` (`KUTHA_TENANT_DIR`), gitignored — not architecture SoT.

Intern map (ADR-011) ≠ process dictionaries (ADR-050). Process allowlist at H3: `.kutha/dictionaries/relations.yaml`. Product allowlist remains `crates/kutha-runtime/dictionaries/relations.yaml`. Mixing the two schemas is a HIGH `plane-mix` finding.

## CLI (H0–H3)

```text
uv run kutha-gov list
uv run kutha-gov explain trajectory
uv run kutha-gov fsm
uv run kutha-gov ci
uv run kutha-gov ci --budget 4
uv run kutha-gov precommit
uv run kutha-gov precommit --check docs-coupling
uv run kutha-gov fold
uv run kutha-gov py
uv run pytest
```

Pin: `.python-version` → `3.13`. Copy `.env.example` to `.env` (`KUTHA_GOV_BUDGET`, `KUTHA_GOV_FAIL_ON_WARN`, `KUTHA_GOV_CARGO_TIMEOUT_SEC`, `KUTHA_TENANT_DIR`, `KUTHA_TENANT_BIN`, `KUTHA_HARNESS_RELATIONS_PATH`). CLI flags override env; env overrides `defaults.budget` in `.kutha/dictionaries/fsm.yaml`. Toolchain: **uv** + **ruff** + **ty** (Astral) + **pyrefly** (Meta). `kutha-gov py` is recursive dogfood of the harness Python. Do not invoke system `python3` (this host may be 3.12). HIGH findings → exit 1. LOW → exit 0 unless `--fail-on-warn`.

`precommit` is the neighbor **check-only** surface (daily-archive `--check-only`: no trajectory artifact write; law-nexus `--check` / `--list-checks`): it runs the dictionary, including `git_path_implies` against the **staged** diff, and does not walk observe_cargo / emit / tenant. `--check ID` is valid only with `json` or `precommit`. Full `ci` still runs coupling with `git_against=auto` (staged if nonempty, else worktree, else last commit). Optional hook file: `.pre-commit-config.yaml` (`uvx pre-commit install --overwrite`). Cargo stays path-filtered in neighbors; here cargo stays in `ci`, not in the hook.

`ci` walks the FSM in `.kutha/dictionaries/fsm.yaml` (idle → load constitution/dictionaries including **relations.yaml**, **invariants.yaml**, and **bridges.yaml** → run checks → **observe_cargo** (test + build `kutha-tenant`) → emit → **emit_tenant** (built binary) → fold → decide → ok|fail). Unknown process relation → no JSONL append (`unknown-relation`). `observe_cargo` records named FF tests as evidence; `emit_tenant` ingests the process JSONL through `kutha-tenant` and queries AS OF the emitted cut. Neither is product SoT. Unknown FSM kind or missing transition → fail-closed. Do not hardcode a new CI phase in Python.

Adding a check: control loop → `.kutha/dictionaries/invariants.yaml`; a fence that cites product → `.kutha/dictionaries/bridges.yaml`; then a row in `.kutha/dictionaries/checks.yaml` using a kind from `.kutha/META.md` (`docs/process/governor-intake.md`). Kutha requirements stay in ADRs / STATE / crates tests. Adding a CI phase: append a state/transition in `fsm.yaml` using an allowed FSM kind. Do not add `scripts/kutha_gov/checks/*.py`. A new *kind* is a rare kernel change (`kinds.py` or `fsm.py` + META allowlist + a test). Unknown kind → HIGH (fail-closed). LLM does not execute checks. Do not add `scripts/ports/`.

## Non-goals

- Porting law-nexus `governor.py` or daily-archive hexagonal YAML fleet (CLI check-only / `--check` / non-mutating hooks are borrowed; their check fleets and GSD machines are not).
- Initializing a 100+ GSD milestone machine.
- Marking honeycomb Accepted because the harness is green.
- Python inside `kutha-runtime`.
- System `python3` / 3.12 as the harness interpreter (must be uv + 3.13).
- Harness as a workflow engine (Cui remains pack composition, not GSD).
- Implementing the STCA-guide §5 tutorial runtime as a second graph (JSON merge-patch objects). That skeleton is **pedagogical**; Kutha events are typed `Op`.
- Legal / science **product** packs (ADR-090/093) as the next crate — M001 S01–S03 are done; do not start Rocks until STATE names M002. Next harness rung is H4 (needs ADR-090 overlay; do not start a legal pack).
