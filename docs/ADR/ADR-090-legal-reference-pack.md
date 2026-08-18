# ADR-090: Legal Reference Pack — Temporal Normative AST & Practice Overlay

## Status

**Proposed** (vertical pack architecture — requirements and layering; not product runtime)

## Date

2026-08-16

## Honeycomb coordinates

- Axes: **Vertical** (primary) · **Time** · **Space** · **Agent** · **Verify** · **Query** · **Data** (materialized ASTs)
- Depends on: ADR-000 (D1–D10), ADR-001, ADR-002
- Anticipates / consumes (when opened): ADR-013 (bi-temporal facts), ADR-021 (packs), ADR-040 (materializations), ADR-050 (dicts), ADR-060/061 (replay/fork), ADR-070/071 (AS OF + hybrid retrieve), ADR-091 (RVF capsules), ADR-093 (sibling scientific vertical)

## Context

Kutha’s first regulated vertical is **legal / normative reference**: agents must reason over Russian normative acts *as of a date*, distinguish text from force from case applicability, and overlay judicial / FAS / control-organ practice without treating the LLM as legal authority.

Neighboring product **law-nexus** already designed (and partially bounded) the domain spine: five-clock safety (ADR-0009), Component Temporal Versioning / temporal AST projections (ADR-0017, `edition_ast_at`), NormativeState (ADR-0018), hierarchy/conflict (ADR-0019), practice overlay (ADR-0020), applicability ownership (ADR-0023). Kutha must not fork that ontology casually; it should host it as an **STCA pack** on Kutha’s engine physics.

Cross-cut research (2025–2026 temporal graphs + law-nexus) also clarifies packaging: **RuVector RVF** is a portable cognitive/witness container — not a legal SoT and not a substitute for CTV. The abbreviation “CVR” does not appear in RuVector; the domain canon is **CTV** (Component Temporal Version).

We need a honeycomb cell that states:

1. Multi-layer temporality requirements (KB / agent / experience / trace).
2. How a Legal Reference Pack maps onto STCA ports.
3. Temporal AST + hierarchical links + practice overlay contracts.
4. Explicit CTV ↔ RVF ↔ Kutha core boundaries and non-goals.

## Decision

### D090-1. Pack shape

Ship **Legal Reference** as a **vertical STCA pack** (Cordis-aligned), not as a second engine and not as Graphiti-class agent memory.

```text
kutha-core (log → fold → behaviors → reversible materializations)
    ▲ ports
legal-reference pack
    • normative identity + CTV event canons
    • temporal AST materializers
    • force / hierarchy / practice projections
    • legal dictionaries + meta-prompt facets
    • optional RVF export/import (ADR-091)
```

Domain semantics may track law-nexus ADRs as **compatibility references** (D046-style ladder): project-local evidence kernel remains Kutha; law-nexus terms are adopted only with explicit mapping tables and fail-closed tests.

### D090-2. Four planes and clock discipline

Every legal-agent workload spans four planes. Temporality is **role-bound**; planes must not silently substitute clocks.

| Plane | Contents | Temporal role | Authority |
|-------|----------|---------------|-----------|
| **L_KB** | Norms, CTV/AST, Force, hierarchy, practice | World / legal-order clocks + derived intervals | Domain evidence events |
| **L_AGENT** | Meta-prompt, dictionaries, capabilities @T | Policy / schema time | Control plane (ADR-050 class) |
| **L_XP** | Episodes, extracted facts, salience | Observation / valid / ingest (memory) | Derived; never overrides L_KB |
| **L_TRACE** | Behavior/tool/LLM lineage | Execution / causal order | **Run SoT** = Kutha event log |

**Hard separations (must fail closed if collapsed):**

```text
Text/CTV        ≠  Force/InForce     ≠  ApplicableToCase
Published       ≠  Observed          ≠  LegallyEffective
Practice        ≠  Kernel mutation
Risk            ≠  Legal conclusion
XP memory fact  ≠  Trace event
RVF capsule     ≠  Hot SoT
```

### D090-3. Common temporality requirements (cross-plane)

| ID | Requirement | Notes |
|----|-------------|-------|
| **TR-01** | Named orthogonal anchors | Kutha core: VT × TT + log order. Legal pack stamps: map law-nexus five clocks (`factual_event`, `proceeding`, `legal_act_effect`, `source_publication`, `system_observation`) as **typed stamps**, not five peer algebraic axes in the core. |
| **TR-02** | No silent clock substitution | Hostile if `source_publication` used when `legal_act_effect` required (law-nexus HC-09 pattern). |
| **TR-03** | Immutable events; intervals are projections | `effective_from/to`, AS OF views, EditionAst are folds — never sole source fields. |
| **TR-04** | Assert / Retract / Correct | World change vs errata vs typed ex-tunc status events (anticipates ADR-013). |
| **TR-05** | Invalidate ≠ delete | Supersession / audit retention; losers preserved for belief AS OF. |
| **TR-06** | Typed non-success | `Unknown`, `Conflict`, `MissingAnchor`, `Incomplete`, `Abstain` are valid product outcomes. |
| **TR-07** | Dual-process ingest | Hot write without mandatory LLM; extract/consolidate async; same-turn read of admitted evidence. |
| **TR-08** | Hybrid retrieve | Structured facts/AST + raw evidence spans; lean context; as-of filter; abstention gate. |
| **TR-09** | Agent outside trust boundary | LLM plans/verbalizes; deterministic operators compute temporal joins and claim traces (TGMS-lite direction). |
| **TR-10** | Packaging ≠ storage | RVF (ADR-091) seals portable units; hot path remains log + materializations. |

### D090-4. Temporal normative AST model (pack kernel)

Adopt the **three-canon L2 pattern** (law-nexus ADR-0017 / KBO-R045) as the pack’s AST contract:

```text
edition_ast_at(t) =
  CompositionAst  := fold(membership_events ≤ t)     // StructuralAst
  EditionAst      := filter(CompositionAst, presence ≤ t)
  TextAst         := resolve_CTV(cc, t)               // may Abstain/Unknown
```

**Entities (pack vocabulary):**

| Entity | Meaning |
|--------|---------|
| **CC** | Component Concept — stable identity (e.g. article) |
| **CTV** | Component Temporal Version — semantic content @ derived interval |
| **CLV** | Language realization of a CTV (ru-primary) |
| **AmendmentEvent** | N-ary causal node with facets: `structural` \| `industrial` \| `text` \| `force` |
| **EditionOracle** | Provider consolidated edition — **checksum** of fold, not event canon |

**Validity rule:** CTV intervals are **event-sourced** (create/terminate micro-events), not static attributes written as truth.

**Evidence classes on amendments (mandatory):** `legislative` > `hypothesized_from_oracle_diff` > `editorial_hint`. Overview files never upgrade to legislative.

### D090-5. Hierarchical temporal relations

Edges are **relation behaviors** (write-time programs), not inert labels:

| Relation | Temporal semantics |
|----------|-------------------|
| `PART_OF` / membership | Attach/detach @ effect day; CompositionAst |
| `AMENDS` → via `AmendmentEvent` | Author, target CC, facets, clocks, evidence_class |
| `SUPERSEDES` / force transition | NormativeState @ `legal_act_effect` |
| `CITES` / cross-ref | Resolve to target **CTV + Force @ governing t** — never “latest known text” |
| `INTERPRETS` / `APPLIES` | PracticeEvidence → `EffectiveInterpretation(t)` |
| `APPEALS` / `CONTROLS` | Proceeding clock + case path; does not rewrite CTV |
| `CONFLICTS_WITH` | Explainable hierarchy resolution (lex superior / specialis / posterior) |

Cascade depth and LLM token use under these behaviors are governed by **Cui budgets** (ADR-030 class).

### D090-6. Practice overlay (judicial / FAS / control / appeal)

Practice is a **first-class temporal projection**, not a sixth core clock and not Graphiti memory.

| Source | Primary clock stamp | Overlay role |
|--------|---------------------|--------------|
| Plenum of the Supreme Court | `legal_act_effect` (ex nunc) | Abstract interpretation |
| Constitutional Court | `legal_act_effect` (ex nunc **or** typed ex tunc) | Interpretation; annulment → separate status event |
| FAS / Presidium practice analogs | `proceeding` | Precedent-analog |
| Control organs (Treasury, Accounts Chamber, sectoral) | `legal_act_effect` | Departmental reading |
| Appeals / complaints | `proceeding` + `factual_event` | Case path / control trail |

**Rules:**

1. Practice does **not** mutate CTV text or NormativeState except via a separately typed, provenance-backed status event (e.g. КС ex-tunc).
2. `EffectiveInterpretation(t)` is derived, lifecycle-bounded, non-authoritative for kernel mutation.
3. Missing practice → `Unknown` coverage — never “low risk” by default.
4. Applicability (`NormRule → Predicate → CaseFacts → Decision → Trace`) remains **deferred** at pack MVP; default is **abstain** until ADR-0023-class protocol lands behind Kutha ports.

### D090-7. Layer map: agent loops → pack surfaces

| Agent need | Plane | Pack surface |
|------------|-------|--------------|
| Text of article X on date Y | L_KB | `resolve_CTV` / TextAst |
| Whole-act edition on date Y | L_KB | `edition_ast_at(t)` |
| Is it in force? | L_KB | NormativeState ⊥ CTV |
| How courts/FAS apply it? | L_KB overlay | `EffectiveInterpretation(t)` |
| Applies to my facts? | L_AGENT + L_KB | Abstain until applicability protocol |
| What the agent already learned | L_XP | Bi-temporal XP facts; cannot override L_KB |
| Why this answer / counterfactual | L_TRACE | Strict replay, fork-diff, quantum receipts |
| Sealed portable case file | Packaging | RVF capsule (ADR-091) |

### D090-8. CTV ↔ RVF ↔ Kutha core

| Artifact | Role | Non-role |
|----------|------|----------|
| **CTV / temporal AST** | Domain versioning & point-in-time text/structure | Not vector index; not agent chat memory |
| **Kutha event log** | Sole run + domain SoT after admit | Not provider XML tree as canon |
| **Materializations** | CSR/HNSW/AST views — reversible | Not second SoT |
| **RVF** | Portable sealed unit: vectors, graph slice, witness, fork transport | Not primary hot store; not legal authority |

Export path (research): `admit → fold → edition_ast_at → optional embed → RVF seal`. Import path must re-validate clocks and evidence classes; never trust capsule as silent promotion.

### D090-9. Implementation staging (falsifiable)

| Stage | Exit criterion | Ceiling |
|-------|----------------|---------|
| **S0** | This ADR + glossary map CTV/clocks/practice; hostile substitution tests named | Research |
| **S1** | Pack skeleton + ports; five-clock stamps on evidence events; no silent subst | Bounded pack scaffold |
| **S2** | Membership fold → StructuralAst; AmendmentEvent facets; EditionOracle checksum | Structural AST |
| **S3** | `resolve_CTV` + `edition_ast_at` on representative fixtures; Unknown/Conflict paths | Text/edition AST |
| **S4** | PracticeEvidence port + EffectiveInterpretation projection (synthetic then corpus) | Practice overlay |
| **S5** | RVF round-trip of sealed edition+practice slice (ADR-091) | Packaging |
| **S6** | Applicability protocol behind ports (or explicit defer) | Product legal Q&A |

No stage may promote lifecycle by documentation alone.

## Consequences

### Positive

- Gives Kutha a concrete first vertical without cloning Graphiti or absorbing law-nexus as a monolith.
- Aligns hypertemporal *discipline* (named clocks, projections) with STCA layering.
- Separates CTV (domain) from RVF (packaging) — removes CVR/RVF confusion.
- Makes abstention and Unknown first-class — required for regulated agents.

### Negative / risks

- Ontology dual-maintenance vs law-nexus if mapping tables drift.
- Temptation to implement full applicability before AST/CTV honesty.
- Practice corpus acquisition cost; risk of treating FAS snippets as force.
- Pack cascade storms if relation behaviors lack Cui budgets.

### Non-goals (this ADR)

- Claiming legal correctness or citation-safe product answers.
- Selecting Consultant/Garant parsers inside Kutha core (may remain law-nexus or adapter packs).
- Making RVF or Neo4j/Falkor/Graphiti required for AS OF truth.
- Implementing full bi-temporal algebra in ADR-090 (belongs ADR-013 + core).

## Alternatives Considered

| Alternative | Why not |
|-------------|---------|
| Embed law-nexus as Kutha | Wrong product boundary; couples engines |
| Graphiti-only legal memory | Collapses clocks; LLM on write path; no CTV AST |
| RVF-primary legal store | Rejected by ADR-000/001; packaging only |
| Sixth “practice clock” in core | Violates closed role model; practice is projection |
| Static `valid_from` fields as SoT | Prevents correction/provenance; law-nexus anti-pattern |

## Open Research Questions

1. Exact port trait set in Rust (`NormativeAdmitPort`, `CtvResolvePort`, `PracticeOverlayPort`, …) vs shared Kutha generic temporal ports.
2. How much of law-nexus `ln-temporal` can be extracted as a pure library vs reimplemented behind Kutha events.
3. Minimal representative corpus for S3–S4 (e.g. 44-ФЗ edition pairs + one FAS decision) without claiming legal gold.
4. Dictionary taxonomy for L_AGENT (norm/decision/commitment/procedure) vs Memanto-style product surface — keep engine-shaped.
5. Whether Quantum Receipts (ADR-014 class) bind AmendmentEvent digests into sealed RVF exports.

## Related Decisions

- ADR-000 — D1 log SoT, D3 dict agents, D4 bi-temporal, D5 materializations, D8 legal vertical sketch
- ADR-001 — vision; legal vertical; RVF not hot SoT
- ADR-002 — STCA packs / behaviors / budgets
- ADR-013 (planned) — Assert/Retract/Correct + native bi-temporal
- ADR-091 (planned) — RVF portable capsules
- External references (compatibility, not Kutha canon): law-nexus ADR-0009, 0016–0023; RuVector RVF docs; de Martim CTV research (arXiv:2506.07853) as cited by law-nexus

## Glossary (pack-facing)

| Term | Meaning in this ADR |
|------|---------------------|
| **CTV** | Component Temporal Version — point-in-time component content |
| **RVF** | RuVector Format — portable sealed capsule |
| **EditionAst** | Temporal AST projection of an act edition at `t` |
| **EffectiveInterpretation** | Practice overlay projection at `t` |
| **EditionOracle** | Provider snapshot used as fold checksum |
| **CVR** | *Not used* — historical misnomer; see CTV / RVF |
