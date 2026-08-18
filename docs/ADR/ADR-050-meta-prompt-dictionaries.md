# ADR-050: Meta-Prompt & Dictionaries

## Status

**Proposed** (Agent control plane — design graduation of ADR-000 D3; not a dedicated literature noun; not runtime)

## Date

2026-08-18

## Honeycomb coordinates

- Axes: **Agent** (primary) · **Time** · **Verify** · **Query**
- Depends on: ADR-000 (D3, R4), ADR-010, ADR-011, ADR-013
- Anticipates: ADR-014 (receipt binds versions), ADR-051 (capabilities — **not opened**), ADR-070 (Cypher surface), ADR-080 (ABAC — **not opened**), ADR-090/093 (`L_AGENT` facet contents)

## Context

STRATEGY P1 and ADR-000 R4 already demand an entity model: `MetaPrompt`, `Dictionary`, `DictionaryEntry`, `AgentInstance`. Applicability synthesis recorded a **hole**: there is no dedicated “agent dictionary” card. Agent literature is either (a) LLM outside the trust boundary (keep) or (b) memory/RAG products that fail D1/D2 (traps). This cell is **design**, not Consensus Query 103+.

Cousin cards (required):

- `paper-llm-compiler-not-executor` — `.compound-engineering/artifacts/research/applicability/cards/paper-llm-compiler-not-executor.md`
- `paper-rdf-term-dictionary` — `.compound-engineering/artifacts/research/applicability/cards/paper-rdf-term-dictionary.md` (**fence only** — intern map owned by ADR-011)
- `paper-online-pg-schema-evolution` — `.compound-engineering/artifacts/research/applicability/cards/paper-online-pg-schema-evolution.md`
- `paper-ontology-temporal-versioning` — `.compound-engineering/artifacts/research/applicability/cards/paper-ontology-temporal-versioning.md`
- `paper-obda-ontology-compile` — `.compound-engineering/artifacts/research/applicability/cards/paper-obda-ontology-compile.md`
- `paper-topic-modeled-tool-routing` — `.compound-engineering/artifacts/research/applicability/cards/paper-topic-modeled-tool-routing.md`
- `open-ontologies-mcp-govern` — `.compound-engineering/artifacts/research/applicability/cards/open-ontologies-mcp-govern.md`

Optional cousins: `paper-pact-argument-provenance`, `paper-proof-carrying-llm-envelope`, `typegraph-typed-sql-kg`, `law-nexus-kb-ontology-catalog`.

Confining Nondeterminism: LLM may edit the **plan**; executor never calls the LLM for asserted results. Schema change is a logged SMO (`SchemaModify`), not discovery. τOWL/KGCL: dictionaries are dated; “latest `.owl` in git” is not SoT. OBDA: TBox+mappings compile to a query plan (Cypher/WCOJ analog), not a second triple store. Topic routing: compile table over dictionaries, not always-hit embedding NN and not a Dify Agent node. Ontology MCP: generate → validate/certify → log; Oxigraph is working memory, not the event log.

ADR-090 already names plane **L_AGENT**: meta-prompt, dictionaries, capabilities @T. This cell owns the **mechanism**; verticals supply facet *contents*.

## Decision

### D050-1. Six dictionary kinds as temporal graph entities

External dictionaries are first-class, bi-temporal (ADR-013) graph entities, not prompt files:

| Kind | Role |
|------|------|
| Controlled vocabulary | Legal tokens / labels @T |
| Action | What an agent may attempt |
| Relation | Legal edge types / behaviors |
| State / mode | Modes from which FSM is *derived* |
| Policy | Tool and conflict policy (argument grain → PACT cousin) |
| Domain ontology / schema | TBox/schema; compiles (OBDA-shaped), evolves via SMO/KGCL events |

Entity sketch (design, not Rust): `MetaPrompt` (versioned constitution: modes, tool policy, temporal semantics, conflict handling — itself temporal), `Dictionary` (kind ∈ table), `DictionaryEntry` (interned id + VT×TT + payload), `AgentInstance` (bound to `meta-prompt@version` + a dictionary snapshot set).

### D050-2. D3 cycle is fail-closed

```text
request → load meta-prompt@T → load dictionaries@T
       → propose → validate(dicts + security) → execute → event log
```

LLM proposes; dictionaries **validate**. If validate fails, no SoT write. FSM/statecharts are **derived** from State+Action for audit/visualization — not hard-coded per vertical as the sole control model.

### D050-3. Intern map ≠ control dictionaries

ADR-011 owns string↔id intern (hot path). This cell owns *which* actions/relations/policies/tokens are legal @T. Citing `paper-rdf-term-dictionary` here is a **fence**, not a second intern design. CSR stores interned ids; it does not store the constitution.

### D050-4. Compiler-not-executor; routing is a table

LLM may compile/propose pipelines and Cypher (AZ/OBDA direction). Asserted numbers and facts require an execution behind them. Tool/pack routing is a **topic/intent table** (or small supervised set-router) over dictionaries — not an unconstrained LLM node, not always-hit cosine over tool docs, not HNSW-as-the-only-gate (042).

### D050-5. Ontology/schema workbench is a pack, not SoT

LLM-drafted OWL/schema: MCP validate/diff/certify (open-ontologies shape) → `SchemaModify` / KGCL payload on the **Kutha log**. Oxigraph/working SPARQL is remote working memory. TypeGraph-style graph-extension JSON is an SMO *candidate*, never LLM mutate-tables-as-truth.

### D050-6. Security is named, not specified

`validate(+security)` is required. Object capabilities, WASM sandbox, and path-ABAC rewrite are **ADR-051 / 080 / 081** — this cell must not invent those kernels.

**Hard separations:**

```text
Meta-prompt@version    ≠  prompt.md in git as SoT
Control dictionaries   ≠  Intern / term map (ADR-011)
Validate fail-closed   ≠  LLM judge-as-truth
Derived FSM            ≠  Hard FSM sole control
Topic routing table    ≠  Dify / oxify DAG
Ontology MCP           ≠  Event-log SoT
OBDA compile           ≠  Materialized second ABox
L_AGENT mechanism      ≠  Vertical facet contents (090/093)
Capabilities / ABAC    ≠  This cell (051/080)
Planner cost model     ≠  This cell (ADR-043)
Hindsight / Graphiti   ≠  Agent memory SoT
```

## Consequences

### Positive

- D3 and STRATEGY P1 have a honeycomb home.
- Receipts (014) can bind versions without defining entities twice.
- Legal/science packs attach dictionaries without forking the runtime.

### Negative / risks

- Zero dedicated card means encoding spikes can thrash; keep Open Research Questions explicit.
- Temptation to implement 051/080 inside 050 “just to validate.”

### Non-goals (this ADR)

- Opening 051, 052, 080.
- Shipping Rust structs.
- Treating Harvey / YOTG / Hindsight / Graphiti as Kutha memory.
- New Consensus queries.

## Alternatives Considered

| Alternative | Why not |
|-------------|---------|
| Hard FSM as sole agent control | D3 / STRATEGY; every vertical forks the runtime |
| Dify / oxify workflow DAG as SoT | Trap; orchestration ≠ dictionaries |
| Hindsight four-network TEMPR | Memory product; low optimality as engine |
| Graphiti bitemporal edges as agent SoT | Trap; clocks + LLM write path |
| Harvey firm DMS as engine | Practice overlay retrieval, not Kutha SoT |
| YOTG layered context graph (`claim`) | Claim-grade; not D3 |
| `prompt.md` / latest OWL file as constitution | Not temporal; not logged |
| Intern map as Action dictionary | KD7 / D050-3 |
| LLM as cost-based planner | ADR-043 |

## Open Research Questions

1. Canonical fields for `DictionaryEntry` and `MetaPrompt` (minimum viable vs vertical facets).
2. How `meta_prompt_version` + dictionary snapshot ids appear on ADR-014 receipts.
3. Derived-FSM snapshot format for audit (read-only picture of State+Action @T).
4. Policy dictionary vs 080 ABAC: where tool-argument grain (PACT) stops and path rewrite starts.
5. Whether topic routing logs misroutes as events in P1 or stays a pack table.

## Related Decisions

- ADR-000 — D3, R4
- ADR-010 — execute appends to the log
- ADR-011 — intern fence
- ADR-013 — VT×TT on entries
- ADR-014 — receipt bind
- ADR-043 — engine costs plans; this cell constrains vocabulary
- ADR-090 / ADR-093 — L_AGENT contents
- ADR-051 / ADR-080 — later security cells
