# Architecture Decision Records (ADR)

Architecture decisions for **Kutha** (`kutha-graph`): self-contained Rust temporal graph engine.

## Idea stack (locked framing)

1. **STCA** — Spatiotemporal Compositional Architecture (space × time × composition) → [ADR-002](./ADR-002-stca-paradigm.md)
2. **Vision** — product north star and non-goals → [ADR-001](./ADR-001-kutha-overall-vision.md)
3. **Foundation locks** — hybrid research decisions D1–D10 → [ADR-000](./ADR-000-kutha-hybrid-architecture-research.md)
4. **Honeycomb cells** — multidimensional follow-ons (ADR-010+) that deepen one facet without rewriting the spine

```
                    ADR-001 Vision
                          │
                    ADR-002 STCA
                          │
         ┌────────────────┼────────────────┐
         ▼                ▼                ▼
      Space cells     Time cells     Composition cells
         │                │                │
         └────────┬───────┴───────┬────────┘
                  ▼               ▼
            Data / Agent / Verify / Query / Security / Vertical
```

## Status vocabulary (harness vs product)

ADR files keep **`## Status: Proposed`** until the cell is implemented in the running engine (**Accepted**). The compact L_map index (`.kutha/dictionaries/honeycomb.yaml`) adds orthogonal **`map`** (document status), **`delivery`** (`map-only` / `frozen` / `spike`), and **`capability`** (`none` / `named` test evidence). Governor green and honeycomb **Proposed** do not promote a cell to Accepted.

## Template

```markdown
# ADR-XXX: Title

## Status
[Research | Proposed | Accepted | Deprecated | Superseded]

## Date
YYYY-MM-DD

## Context
## Decision
## Consequences
## Alternatives Considered
## Open Research Questions
## Related Decisions
## Honeycomb coordinates (optional)
- Axes: [Space | Time | Composition | Data | Agent | Verify | Query | Security | Vertical]
- Depends on: ADR-…
```

## Index — spine

| ADR | Title | Status | Date |
|-----|-------|--------|------|
| [000](./ADR-000-kutha-hybrid-architecture-research.md) | Kutha Hybrid Architecture — Research Foundation | Proposed | 2026-08-15 |
| [001](./ADR-001-kutha-overall-vision.md) | Kutha Overall Vision | Proposed | 2026-08-16 |
| [002](./ADR-002-stca-paradigm.md) | Spatiotemporal Compositional Architecture (STCA) | Proposed | 2026-08-16 |

## Index — honeycomb cells (opened)

| ADR | Title | Status | Date |
|-----|-------|--------|------|
| [010](./ADR-010-event-log-runtime-quantum.md) | Event Log & Reactive Runtime Quantum | Proposed | 2026-08-18 |
| [011](./ADR-011-lean-event-schema-lineage.md) | Lean Event Schema & Lineage | Proposed | 2026-08-18 |
| [012](./ADR-012-snapshots-tiers-vacuum.md) | Snapshots, Tiers, and Vacuum | Proposed | 2026-08-18 |
| [013](./ADR-013-bitemporal-facts-invalidation.md) | Bi-temporal Facts & Invalidation | Proposed | 2026-08-18 |
| [014](./ADR-014-cascade-budgets-quantum-receipts.md) | Cascade Budgets & Quantum Receipts | Proposed | 2026-08-18 |
| [020](./ADR-020-cargo-workspace-port-isolation.md) | Cargo Workspace & Port Isolation | Proposed | 2026-08-18 |
| [021](./ADR-021-pack-plugin-lifecycle.md) | Pack / Plugin Lifecycle | Proposed | 2026-08-18 |
| [022](./ADR-022-vertical-slice-module-conventions.md) | Vertical-Slice Module Conventions | Proposed | 2026-08-18 |
| [023](./ADR-023-test-strategy-and-oracle-matrix.md) | Test Strategy and Oracle Matrix | Proposed | 2026-10-05 |
| [030](./ADR-030-cui-budget-traits.md) | Cui Budget Traits / Max-Convolution Allocation | Proposed | 2026-08-18 |
| [031](./ADR-031-pack-scheduling-under-v.md) | Pack Scheduling Under V | Proposed | 2026-08-18 |
| [040](./ADR-040-materialization-plugin-protocol.md) | Materialization Plugin Protocol | Proposed | 2026-08-18 |
| [041](./ADR-041-csr-graphblas-hot-path.md) | CSR / GraphBLAS Hot Path | Proposed | 2026-08-18 |
| [042](./ADR-042-hnsw-access-method-fence.md) | HNSW Access-Method Fence | Proposed | 2026-08-18 |
| [043](./ADR-043-hybrid-query-planner.md) | Hybrid Query Planner | Proposed | 2026-08-18 |
| [050](./ADR-050-meta-prompt-dictionaries.md) | Meta-Prompt & Dictionaries | Proposed | 2026-08-18 |
| [051](./ADR-051-capability-security.md) | Capability Security | Proposed | 2026-08-18 |
| [052](./ADR-052-genai-enrichment-pack.md) | GenAI Enrichment as Optional Pack | Proposed | 2026-08-18 |
| [060](./ADR-060-strict-replay.md) | Strict Replay | Proposed | 2026-08-18 |
| [061](./ADR-061-fork-and-diff.md) | Fork-and-Diff | Proposed | 2026-08-18 |
| [062](./ADR-062-regimes-gated-loop.md) | Regimes Gated Loop (Optional) | Proposed | 2026-08-18 |
| [070](./ADR-070-cypher-temporal-as-of.md) | Cypher + Temporal AS OF Surface | Proposed | 2026-08-18 |
| [071](./ADR-071-hybrid-retrieval-composition.md) | Hybrid Retrieval Composition | Proposed | 2026-08-18 |
| [080](./ADR-080-abac-multi-tenant-slices.md) | ABAC & Multi-Tenant Slices | Proposed | 2026-08-18 |
| [081](./ADR-081-agent-sandbox.md) | Agent Sandbox | Proposed | 2026-08-18 |
| [090](./ADR-090-legal-reference-pack.md) | Legal Reference Pack — Temporal Normative AST & Practice Overlay | Proposed | 2026-08-16 |
| [091](./ADR-091-rvf-portable-capsules.md) | RVF Portable Capsules | Proposed | 2026-08-18 |
| [092](./ADR-092-naming-and-license.md) | Naming & License | Proposed | 2026-08-18 |
| [093](./ADR-093-scientific-archive-pack.md) | Scientific Archive Pack — Scholarly Revision AST & Evidence Graph | Proposed | 2026-08-16 |

## Index — honeycomb cells (planned)

All coordinate-band working titles listed in this README are **opened** as Proposed cells. Numbers remain bands, not a waterfall. Clarifications belong in the existing cell without silently rewriting D1–D10. New ADR-100+ cells remain frozen under `.kutha/STATE.md`; this index does not authorize opening them.

This index lists **32** ADR files; all are **Proposed**.

## Engine evidence (2026-10-18 audit)

Status stays **Proposed**. This is spike evidence, not acceptance. Source: CBM `kutha-graph` reindex and GitNexus `analyze --index-only` the same day. Residuals closed in the working tree after that audit (still not Accepted):

| Cell | Evidence in the engine | Still not this cell |
|------|------------------------|---------------------|
| 010–014 | log, quantum, bi-temporal `as_of`, Horn cascade, receipts | Accepted runtime |
| 013 | overlap-aware premises; all premise combinations emit a head window; replay requires the conjunction to cover the Behavior VT | Cypher MATCH |
| 040–041 | CSR lease; leapfrog intersect | GraphBLAS planner |
| 050 | reserved `kutha:*` facts | six dictionary kinds (frozen) |
| 060 | `replay_check` fold is the fold `open` keeps; Behavior replay checks VT coverage | full provenance polynomial |
| 070 / 042 / 080 / 090 / 093 | frozen by `.kutha/STATE.md` thaw map | do not thaw from this note |

Hot-path notes (Proposed): policy pin election walks `policy_by_ingest` newest-first; Behavior unindex uses `premise_rels_of_behavior` the way M104 used `cause_of_behavior`. Duplicate modern `Define` lines are dropped on Op-wire rewrite when the name is already interned. `FoldIndexMode::Lazy` still scans until `ensure_hot_maps` — that thin-write mode is intentional.

Research note (2026-10-19, still Proposed): ADR-040 records a piecewise-linear position model (PGM) as a possible droppable read lease on a sealed sorted `u64` column. It is not an engine spike, not Accepted, and not a thaw of ADR-042. The “14,000×” and “4 cache lines / 2.9× versus binary search” figures are not the PVLDB 2020 results.

| Band | Axis | Notes |
|------|------|--------|
| **010–019** | Time | Opened 010–014 |
| **020–029** | Space | Opened 020–023. **ADR-023 is Verify-primary** (`honeycomb.yaml` `axis: Verify`) though numbered in this Space band |
| **030–039** | Composition | Opened 030–031 |
| **040–049** | Data | Opened 040–043. **ADR-043 is Query-primary** (`honeycomb.yaml` `axis: Query`) though numbered in this Data band |
| **050–059** | Agent | Opened 050–052 |
| **060–069** | Verify | Opened 060–062 (062 optional, default off) |
| **070–079** | Query | Opened 070–071 |
| **080–089** | Security / tenancy | Opened 080–081 |
| **090–099** | Vertical / packaging | Opened 090–093 |

Manifest / algorithms: [`../architecture/stca-guide.md`](../architecture/stca-guide.md).

Cross-cell contract review (2026-09-13): [semantic contract validation](../architecture/semantic-contract-validation.md). It connects recovery, supports, temporal cuts, trace/admission, and completion to proposed acceptance probes. It is not a new milestone or evidence that those capabilities ship.

Semantic-gap review (2026-09-30): [semantic gap review](../architecture/semantic-gap-review.md). Proposed review, not an ADR, not delivery authorization.

## Process

1. **Research** — hypothesis and detail plan (not final).
2. **Proposed** — concrete decision ready to accept.
3. **Accepted** — affirmed and implemented.
4. **Superseded** — replaced by a newer ADR.

**Rule:** New work cites ADR-001/002 coordinates; do not silently rewrite ADR-000 D1–D10. Prefer opening a honeycomb cell over expanding vision ADRs indefinitely.

Honeycomb and vision ADRs (000–002, 010–093) are **Proposed**: decisions are named and ready to accept. **Accepted** requires an implemented runtime for that cell. P0 spike covers log/fold/quantum/replay/snapshot/CSR-lease/LFTJ-intersect/WAL-cousin/materializer trait plus SEG-01 (`KUTHASEG` + `manifest.json`) and an optional unwired `rocks-index` locator (`rocks_index::SegmentIndex`). Rocks **event store**, Cypher, HNSW, ABAC remain unaccepted.

Parallel **harness** (not an ADR): [`../process/kutha-harness.md`](../process/kutha-harness.md). Execution pyramid: `.kutha/STATE.md`. Governor: `uv run kutha-gov ci` (Python 3.13 / uv). Python dogfood: `uv run kutha-gov py` (ruff, ty, pyrefly).
