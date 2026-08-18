# ADR-001: Kutha Overall Vision

## Status

**Proposed** (north-star vision — locks product intent; details live in ADR-002+ honeycomb cells)

## Date

2026-08-16

## Context

ADR-000 fixed hybrid research decisions **D1–D10** (event-log SoT, behaviors, dict-first agents, bi-temporal facts, pluggable materializations, Rust+RocksDB, profiles, tests). Ideation and the STCA manifesto clarified the *ordering* of ideas: the top idea is not a vendor feature list and not a thin memory framework — it is a **Spatiotemporal Compositional Architecture (STCA)** engine product named **Kutha**.

We need a short, durable vision ADR so later cells (runtime, temporal algebra, materializers, agents, verticals) hang from one spine.

## Decision

### Product one-liner

**Kutha** is a self-contained **Rust** graph engine: an append-only event log is the source of truth; the graph is a deterministic fold of that log; domain capability is delivered as vertical packs behind ports; hot CSR/HNSW/temporal views are reversible materializations — not a second SoT; agents are governed by versioned meta-prompts and dictionaries (LLM optional, never sole authority for audited fact truth).

### Idea stack (top → down)

1. **STCA** (ADR-002) — spatial isolation × temporal event graph × Cui-style compositional budgets.
2. **Kutha product deltas** on top of STCA — hot materializations, native bi-temporal fact semantics (Graphiti as *reference only*), dict-first in-DB agents, legal/finance/science verticals.
3. **Honeycomb cells** (ADR-010+) — multidimensional ADRs that deepen one facet without rewriting the vision.

### Non-goals (vision-level)

- Kutha as a required client of Neo4j / FalkorDB / Graphiti / cloud LLM for core assert → invalidate → AS OF.
- Pure ActiveGraph without a path to hot projections.
- Pure state-first graph DB with agents bolted on.
- RVF (or any portable pack) as primary hot storage.

### Relationship to ADR-000

ADR-000 remains the research foundation and keeps **D1–D10** locked. ADR-001 does not reopen those locks; it states the **vision framing** and the **ADR honeycomb process**. Implementation acceptance still requires follow-on cells and spikes (P0+).

## Consequences

### Positive

- Clear “top idea” for humans and coding agents: STCA first.
- Room to grow via honeycomb cells without thrashing ADR-000.
- Aligns ideation HTML, `docs/architecture/stca-guide.md`, and backlog language.

### Negative / risks

- Renumbering older “ADR-001 = runtime” sketches — see Related Decisions; runtime becomes a honeycomb cell (ADR-010 class).
- Vision ADR can become slogan-only if cells are not opened with falsifiable exits.

## Alternatives Considered

| Alternative | Why not |
|-------------|---------|
| Keep only ADR-000 forever | Too large; no process for multidimensional growth |
| Lead with Graphiti-compat product | Wrong product shape (memory framework / BYO graph) |
| Lead with Samyama-class throughput DB | Abandons event-log SoT and agent control plane |

## Open Research Questions

- Exact honeycomb numbering ranges and owners (see ADR README).
- When ADR-001 / ADR-002 graduate Research → Proposed → Accepted (gated by P0 spike evidence).
- Whether a future brand/name ADR remains a leaf cell or stays under vision.

## Related Decisions

- ADR-000 — hybrid architecture research foundation (D1–D10)
- ADR-002 — STCA paradigm (space × time × composition)
- ADR-090 — Legal Reference Pack (temporal normative AST & practice overlay)
- ADR-093 — Scientific Archive Pack (scholarly revision AST & evidence graph)
- `docs/architecture/stca-guide.md` — STCA manifesto / algorithms / Rust isolation laws
- Ideation: `.compound-engineering/artifacts/ideation/2026-08-15-kutha-adr-crystallization-ideation.html`
