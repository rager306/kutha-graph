# ADR-093: Scientific Archive Pack — Scholarly Revision AST & Evidence Graph

## Status

**Proposed** (vertical pack architecture — requirements and layering; not product runtime)

## Date

2026-08-16

## Honeycomb coordinates

- Axes: **Vertical** (primary) · **Time** · **Space** · **Agent** · **Verify** · **Query** · **Data**
- Depends on: ADR-000 (D1–D10), ADR-001, ADR-002, ADR-090 (sibling vertical; shared TR-* discipline)
- Anticipates / consumes (when opened): ADR-013 (bi-temporal facts), ADR-021 (packs), ADR-040–043 (materializations / hybrid query), ADR-050 (dicts), ADR-060/061 (replay/fork), ADR-091 (RVF capsules)

## Context

Kutha’s second natural vertical is **scientific / scholarly knowledge**: ingest papers (PDF/HTML), preserve revision history, extract structure and evidence, embed, and support agent retrieval — without treating the LLM as scientific authority and without collapsing “latest arXiv HTML” into work identity.

Neighboring product **daily-archive** (`/root/daily-archive`) already implements much of this as a local-first scientific knowledge engine: Rust hexagonal crates (`da-*`), universal `kg-*` ontology/storage/embeddings/algorithms/pipeline, Samyama as graph+vector store (ADR-040), RuVector as agent brain, RVF as Tier-3 experience, bi-temporal **edges** (ADR-046), PaperRevision expressions (ADR-058), EvidenceBundle/Claim activation (ADR-042), Research Process Plane (ADR-043), and fail-closed graph import (D127).

Kutha must not absorb daily-archive as a second SoT engine. It should host the **domain pack** on Kutha physics (event log → fold → reversible materializations), remapping Samyama/RVF roles to ADR-000/001 locks:

| daily-archive (binding ADR-040) | Kutha target |
|---------------------------------|--------------|
| Samyama = sole knowledge SoT | Event log = SoT; Samyama-class graph/HNSW = **materialization** |
| RuVector = mandatory brain | Optional intelligence pack (ADR-052 class) |
| RVF = Tier-3 experience store | Portable/experience **packaging** (ADR-091), not hot SoT |

This cell states pack shape, shared TR-* temporality, scholarly AST contracts, evidence/claim surfaces, and an explicit mapping from `kg-*` / daily-archive ADRs to Kutha ports.

## Decision

### D093-1. Pack shape

Ship **Scientific Archive** as a **vertical STCA pack**, sibling to Legal Reference (ADR-090), not a Graphiti memory sidecar and not a mandatory Samyama runtime.

```text
kutha-core (log → fold → behaviors → reversible materializations)
    ▲ ports
scientific-archive pack
    • Paper / PaperRevision / Manifestation identity
    • temporal-edge fact model (ADR-046-compatible)
    • EvidenceBundle / Claim / citation graph
    • pipeline stage ledger + admit gates (D127-style)
    • scientific dictionaries + meta-prompt facets
    • optional RVF export (experience / sealed corpus slice)
```

Domain semantics track daily-archive ADRs as **compatibility references**. Project-local evidence kernel remains Kutha; terms are adopted via mapping tables and fail-closed tests.

### D093-2. Relationship to ADR-090 and shared TR-*

Both verticals obey the same cross-plane temporality requirements **TR-01…TR-10** defined in ADR-090 (named anchors, no silent substitution, events immutable / intervals projections, Assert/Retract/Correct, invalidate≠delete, typed non-success, dual-process ingest, hybrid retrieve, agent outside trust boundary, packaging≠storage).

| Concern | Legal pack (090) | Scientific pack (093) |
|---------|------------------|------------------------|
| Work identity | Normative Work / CC | Paper (versionless) |
| Expression @t | CTV / `edition_ast_at` | PaperRevision + revision-scoped Section AST |
| Manifestation | Source bytes / provider XML | SourceManifestation (pdf\|html\|tei, sha256) |
| Force / status | NormativeState ⊥ CTV | Publication/retraction/errata edges — **not** legal InForce |
| Practice overlay | Courts / FAS / control | Citations, EvidenceBundle, peer claims |
| Process plane | Applicability (deferred) | Research Process Plane (ADR-043-class) |
| Admit gate | Promotion/publication authority | `import_eligible` / human go (D127-class) |

### D093-3. Four planes (scientific specialization)

| Plane | Contents | Temporal role |
|-------|----------|---------------|
| **L_KB** | Paper, revisions, sections, entities, claims, citations | Publication / revision / valid-edge time |
| **L_AGENT** | Operators, MCP tools, prompt/optimizer FSM facets | Policy @T |
| **L_XP** | Query trajectories, SONA-like weights, compacted memory | Experience; promote only via gate |
| **L_TRACE** | Pipeline stage ledger, parse/extract cache lineage, Governor | Execution / admit proof |

**Hard separations:**

```text
Paper (Work)     ≠  PaperRevision (Expression)  ≠  Manifestation (bytes)
Section @ rev    ≠  shared Claim identity
SUPPORTS edge    ≠  Claim node lifetime
Episodic source  ≠  derived extraction
Process outcome  ≠  published claim truth
RVF experience   ≠  L_KB SoT
Samyama view     ≠  event log
```

### D093-4. Scholarly revision AST (pack kernel)

Adopt daily-archive **ADR-058** shape behind Kutha events:

```text
Paper                         # Work; vid:paper:{id}
  └─ HAS_REVISION → PaperRevision
       ├─ HAS_MANIFESTATION → SourceManifestation (pdf|html|…)
       ├─ hasPart → revision-scoped Section
       └─ SUPERSEDES → previous observed PaperRevision
```

**Rules:**

1. `Paper` VID remains versionless; never one Paper node per `vN`.
2. Section / extraction / embedding lineage **must** include `revision_vid` + manifestation digest.
3. Work-level “current” is an explicit projection — **not** inferred from insertion order.
4. `SUPERSEDES` means observed predecessor; missing intermediate revisions allowed and explicit.
5. Catalog files and load reports are provenance, not graph truth until admit.

**Point-in-time surface (research name):**

```text
revision_ast_at(paper, rev|as_of) =
  StructureAst := fold(membership/parts for revision)
  TextAst      := sections + spans bound to manifestation sha
  EmbedView    := optional materialization (reversible)
```

### D093-5. Temporal edge model (facts on edges)

Adopt daily-archive **ADR-046** / GRAPH-CORE-TEMPORAL-DESIGN principle: **entities persist; facts change; temporality lives on edges.**

Required fields on temporal fact edges (SUPPORTS, REFUTES, CITES, MENTIONS, REFERENCES, …):

| Field | Axis |
|-------|------|
| `valid_at` / `invalid_at` | Valid time |
| `created_at` / `expired_at` | Transaction time |
| `reference_time` | Source episode timestamp |

**Non-temporal (structural):** `HAS_PART`, `FROM_SOURCE`, `AUTHORED_BY`, `HAS_MANIFESTATION`, classification membership — `created_at` only.

**EpisodicNode** (or Kutha equivalent episode event): the record of observed source content, not proof that its claims are true; edge `reference_time` chains to episode.

**Invalidation:** an explicit correction, retraction, or supersession policy identifies the statement being invalidated (ADR-011/013); overlapping intervals or shared endpoints alone do not establish replacement. Independent `SUPPORTS` / `CONTRADICTS` evidence may coexist. Preserve the evidence and unresolved conflict unless a named admission/resolution policy justifies a different derived view; no physical delete.

Optional legal-aware extensions (`retroactive_to`, `overlap_allowed`) may appear when this pack cites normative objects; they do not replace ADR-090 clocks.

### D093-6. Evidence, claims, and activation

Adopt ADR-042-class separation:

| Object | Role |
|--------|------|
| **ConceptCluster** | Derived community — not evidence |
| **EvidenceBundle** | Source-grounded n-ary unit (experiment/result/citation context) |
| **Claim** | Proposition-bearing node |
| Edges | `SUPPORTS` / `CONTRADICTS` / `QUALIFIES` (temporal) |

**Query-local evidence activation** (HyCERAG-inspired, revised): retrieve → incidence subgraph → structural activation (PPR-like) → assemble context → LLM. Chains are **ephemeral** by default; persist only to L_XP / RVF with `retrieval_eligible=false` until gated promotion.

**Clarification (2026-09-13, Proposed; not implemented):** Source observation, extracted proposition, and admitted claim remain distinct. Schema validity and successful pipeline execution establish neither scientific truth nor admission. Admission identifies the evidence and policy that justify the permitted use (ADR-014/060). Revision-bound derivations follow ADR-011/052: a correction, retraction, or supersession makes affected summaries, embeddings, cached answers, and action justifications ineligible for current use until re-evaluated, while retaining their historical records and lineage. Revision of one source must not silently invalidate independent evidence for the same claim.

### D093-7. Research Process Plane

Keep publication KB and process memory distinct (ADR-043-class):

| Plane | Question | Store on Kutha |
|-------|----------|----------------|
| Publication | What was published? | L_KB events + AST materializations |
| Research Process | How was knowledge obtained/tested? | L_KB process objects **or** separate process events — never overwrite publication facts |
| Experience | How did the agent operate? | L_XP / RVF |

Scalar reward must not collapse “execution failure” with “hypothesis refuted.”

### D093-8. Pipeline ledger and admit gates

Pack owns a **stage ledger** (daily-archive 14-stage pattern as compatibility reference): acquisition → catalog → parse → revision materialization → extract → embed → … → query.

Invariants:

1. Each stage declares context gained/lost, failure class, cache mode.
2. Candidates ≠ authoritative retention.
3. **Fail-closed admit:** default `import_eligible=false` (D127-class) until human/process gate.
4. Context-conservation diagnostics must not mint an independent admission verdict.

### D093-9. `kg-*` → Kutha port mapping (research)

| daily-archive / kg-* | Kutha landing |
|----------------------|---------------|
| `kg-ontology` temporal helpers / YAML schemas | Pack ontology DATA + validators; core keeps generic bi-temporal primitives (ADR-013) |
| `kg-storage` GraphStore | Adapter writing **events** (not bypassing log); read via materializations |
| `kg-embeddings` | Materialization / access-method pack (ADR-042 class) |
| `kg-algorithms` temporal resolve / PPR | Query / algorithm packs behind ports |
| `kg-pipeline` | Pack application services; Cui budgets for cascades |
| `da-mcp` read-only KB | Optional MCP façade over Kutha query ports |
| Samyama embedded | Optional GraphStore adapter / hot view — **not** SoT |
| RuVector SONA/GNN | Optional ADR-052 enrichment |
| RVF Tier 3 | ADR-091 capsules for experience + sealed corpus slices |

### D093-10. Layer map: agent loops → pack surfaces

| Agent need | Plane | Pack surface |
|------------|-------|--------------|
| Paper identity | L_KB | `vid:paper:*` |
| Exact v1 vs v5 structure | L_KB | PaperRevision + `revision_ast_at` |
| Section / span grounded quote | L_KB | Manifestation sha + EvidenceBundle |
| Claim support/contradiction @t | L_KB | Temporal SUPPORTS/CONTRADICTS |
| “What did we run / try?” | Process | Research Process objects |
| Prior retrieval trajectory | L_XP | RVF / experience store |
| Why this context was admitted | L_TRACE | Pipeline ledger + admit gate |
| Portable corpus slice | Packaging | RVF seal (ADR-091) |

### D093-11. Implementation staging (falsifiable)

| Stage | Exit criterion | Ceiling |
|-------|----------------|---------|
| **S0** | This ADR + glossary map vs daily-archive ADR-046/058/042/043/040 | Research |
| **S1** | Pack skeleton + ports; episode + temporal-edge schema on events | Bounded scaffold |
| **S2** | Paper / PaperRevision / Manifestation coexist; SUPERSEDES; revision-scoped sections | Revision AST |
| **S3** | Temporal edge invalidation + as-of selection tests | Bi-temporal facts |
| **S4** | EvidenceBundle → Claim edges + query-local activation (synthetic) | Evidence graph |
| **S5** | Pipeline ledger + fail-closed admit on representative arXiv pair (e.g. v1/v5) | Ingest honesty |
| **S6** | RVF round-trip of sealed revision slice; optional RuVector rerank behind port | Packaging / enrichment |

No stage promotes by documentation alone. D127-class lock remains until explicit unlock evidence.

## Consequences

### Positive

- Second vertical without cloning Legal pack; shared TR-* and STCA ports.
- Reuses battle-tested daily-archive ontology decisions (058, 046, 042) under Kutha SoT rules.
- Clarifies Samyama/RVF remapping early — avoids ADR-040 lock conflicting with ADR-000.
- `kg-*` becomes a candidate shared library boundary across Kutha packs.

### Negative / risks

- Dual-maintenance vs daily-archive if mapping drifts.
- Temptation to keep Samyama write path as SoT “for speed.”
- Process plane scope creep (full lab OS) before revision AST honesty.
- Import unlock social process — same class of risk as legal publication authority.

### Non-goals

- Claiming scientific correctness, peer-review quality, or complete OpenAlex coverage.
- Binding Kutha core to Samyama or RuVector.
- Merging Legal CTV and PaperRevision into one type.
- LLM-mandatory extraction on the hot write path.

## Alternatives Considered

| Alternative | Why not |
|-------------|---------|
| Vendor daily-archive as Kutha | Wrong SoT; Samyama lock conflicts ADR-000 |
| Only Legal pack | Leaves scientific GTM and corpus engine out of honeycomb |
| Graphiti-only paper memory | Weak revision/manifestation model; LLM-centric write |
| One Paper node per arXiv version | Breaks citation/enrichment joins (rejected in ADR-058) |
| RVF as primary paper store | Rejected by ADR-001; packaging/experience only |

## Open Research Questions

1. Extract `kg-ontology` / `kg-storage` as true workspace crates shared by Kutha vs reimplement thin ports.
2. Minimal gold pair for S5 (candidate: arXiv `2506.07853` v1/v5 — already motivational in ADR-058).
3. How Research Process objects share IDs with Legal practice when a paper analyzes a statute (cross-pack edges).
4. Whether HyCERAG activation stays ephemeral-only until Verify cell ADR-060 lands claim gating.
5. MCP surface: pack-local vs core Kutha query gateway.

## Related Decisions

- ADR-000 — D1 log SoT, D4 bi-temporal, D5 materializations; Samyama as materialization not product SoT
- ADR-001 — vision; RVF not hot storage; science vertical
- ADR-002 — STCA packs
- ADR-090 — Legal Reference Pack (sibling; shared TR-01…TR-10)
- ADR-091 (planned) — RVF portable capsules
- ADR-013 (planned) — native Assert/Retract/Correct
- External compatibility (not Kutha canon): daily-archive ADR-040, 042, 043, 046, 047, 050, 058; GRAPH-CORE-TEMPORAL-DESIGN.md

## Glossary (pack-facing)

| Term | Meaning in this ADR |
|------|---------------------|
| **Paper** | Scholarly Work identity (versionless) |
| **PaperRevision** | Immutable Expression (e.g. arXiv v5) |
| **SourceManifestation** | Content-addressed bytes (pdf/html/…) |
| **EpisodicNode** | Raw source provenance node/event |
| **EvidenceBundle** | Source-grounded n-ary evidence unit |
| **Temporal edge** | Fact relationship with VT×TT fields |
| **D127-class** | Fail-closed admit until explicit human/process unlock |
| **kg-*** | Universal graph subsystem crates from daily-archive |
