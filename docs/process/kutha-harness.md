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
| **H1** | Evidence: `cargo test` / named FF tests as observations (not SoT) | FF1–FF5 in crates |
| **H2** | Assert harness events onto the **Kutha log** (delivery facts, not product norms); query **AS OF** the process | FF5 green — **unlocked, not started** |
| **H3** | Fail-closed writes through a **relation allowlist** (stub of ADR-050) | FF6 stub (S03) |
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
| freeze | CargoPort | `crates/*/Cargo.toml` | product deps |
| honeycomb-map | IntentPort | STATE+ROADMAP text | treat honeycomb as M001 |
| dogfood | ManifestPort | `docs/process/kutha-harness.md` | mix into `kutha-runtime` |
| meta-prompt | ConstitutionPort | `.kutha/META.md` + dictionaries | Python Check subclasses as intake |

Hexagon lives **inside** a slice (ADR-022). Do not grow a repo-root `ports/` / `adapters/` / `domain/` tree — that is the cohesion failure the manifesto forbids. Composition root is `uv run kutha-gov` (Python 3.13). Later a Rust `kutha-harness` bin that **only** wires adapters. Checks are Behaviors: they propose findings; they do not mutate ADRs.

### Time (ActiveGraph / ADR-010, typed)

1. Human **asserts** intent (active milestone) in STATE — until H2 this is a typed fact on the Kutha log.
2. Each `ci` quantum **emits** `harness.run` triples into `.kutha/events.jsonl` (append-only).
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

1. Domain ignorance: check logic does not import `kutha_runtime` at H0; H2 talks **only** through the runtime API (append/fold/as_of).
2. Private-by-default: kind interpreters stay in `kinds.py`; check *lists* stay in the dictionary.
3. One composition root: CLI / future `kutha-harness` bin. Python stays repository-control (neighbors’ ADR-0007 lesson), never inside the graph crate.

### Mapping H0 records → Kutha `Op` (H2, not now)

```text
assert  subject=harness.run  relation=status   object=ok|fail
assert  subject=harness.run  relation=high     object=<int>
assert  subject=M001         relation=active   object=S01     (intent)
```

Intern map (ADR-011) ≠ process dictionaries (ADR-050). Process allowlist at H3.

## CLI (H0)

```text
uv run kutha-gov list
uv run kutha-gov explain trajectory
uv run kutha-gov fsm
uv run kutha-gov ci
uv run kutha-gov ci --budget 4
uv run kutha-gov fold
uv run kutha-gov py
uv run pytest
```

Pin: `.python-version` → `3.13`. Copy `.env.example` to `.env` (`KUTHA_GOV_BUDGET`, `KUTHA_GOV_FAIL_ON_WARN`). CLI flags override env; env overrides `defaults.budget` in `.kutha/dictionaries/fsm.yaml`. Toolchain: **uv** + **ruff** + **ty** (Astral) + **pyrefly** (Meta). `kutha-gov py` is recursive dogfood of the harness Python. Do not invoke system `python3` (this host may be 3.12). HIGH findings → exit 1. LOW → exit 0 unless `--fail-on-warn`.

`ci` walks the FSM in `.kutha/dictionaries/fsm.yaml` (idle → load constitution/dictionaries → run checks → emit → fold → decide → ok|fail). Unknown FSM kind or missing transition → fail-closed. Do not hardcode a new CI phase in Python.

Adding a check: append a row to `.kutha/dictionaries/checks.yaml` using a kind from `.kutha/META.md`. Adding a CI phase: append a state/transition in `fsm.yaml` using an allowed FSM kind. Do not add `scripts/kutha_gov/checks/*.py`. A new *kind* is a rare kernel change (`kinds.py` or `fsm.py` + META allowlist + a test). Unknown kind → HIGH (fail-closed). LLM does not execute checks. Do not add `scripts/ports/`.

## Non-goals

- Porting law-nexus `governor.py` or daily-archive hexagonal YAML fleet (shape of “add a row, not a class” is adopted; their fleet is not).
- Initializing a 100+ GSD milestone machine.
- Marking honeycomb Accepted because the harness is green.
- Python inside `kutha-runtime`.
- System `python3` / 3.12 as the harness interpreter (must be uv + 3.13).
- Harness as a workflow engine (Cui remains pack composition, not GSD).
- Implementing the STCA-guide §5 tutorial runtime as a second graph (JSON merge-patch objects). That skeleton is **pedagogical**; Kutha events are typed `Op`.
- Legal / science **product** packs (ADR-090/093) as the next crate — first product steel thread is M001; S01/S02 (named AS OF + CSR cut) are done; next is S03 (FF6 allowlist stub).
