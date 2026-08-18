# Event-Sourced & Graph Agent Memory → Kutha STCA

Research baseline for Kutha’s STCA stack: **append-only event log → fold → materializations**; **dict-first agents**; **behaviors**. Sources deep-read against Kutha ADR-000/002 and `docs/architecture/stca-guide.md`.

---

## 1. ActiveGraph — *The Log is the Agent* ([arXiv:2605.21997](https://arxiv.org/abs/2605.21997))

**Claim:** Invert the usual stack. Log is primary; graph and “memory” are projections; behaviors react and emit.

| Dimension | Finding |
|-----------|---------|
| **SoT** | **Append-only event log.** Working graph = deterministic fold. Embeddings/summaries are optional derived views, not authority. Explicitly contrasts Zep/Graphiti/Mem0 as “memory layered onto an agent whose primary representation lives elsewhere.” |
| **Time** | Total order of events in the log (monotonic ids). Provenance on every object (`behavior`, causing event, LLM call). No native bi-temporal *world* validity model; time is **execution/lineage time**. |
| **Update / invalidation** | No silent overwrite of SoT. Changes = new events (`object.*`, `relation.*`, patches, `behavior.*`, `llm.req/resp`). Graph rebuilt by fold. Schema evolution = migration tooling (operational burden called out). |
| **Multi-agent / concurrency** | Coordination via shared graph + reactive behaviors (no orchestrator). **Single-run ordering is well-defined; concurrent/distributed writers and multi-agent contention are explicitly unresolved.** Frames = in-run parallel subcontexts; forks = durable branches. |
| **Audit / fork / replay** | **Strict vs permissive replay**; content-addressed cache for LLM/tool I/O; **fork-at-event** with shared prefix + structural diff; full lineage goal→artifact→model call. Self-improvement sketched as fork-and-diff of rule/behavior changes. |
| **STCA mapping** | **Event Log:** SoT (D1/D2). **Fold/Behaviors:** core model (behaviors + relation-behaviors). **Hot materializations:** out of scope (graph is working projection, not CSR/HNSW). **Agent dict:** packs of types/prompts/policies; closest cousin is “pack,” not external temporal dictionaries. |

**Kutha delta already planned:** ActiveGraph *model* without shipping ActiveGraph runtime; add bi-temporal facts + hot CSR/HNSW + dict control plane.

---

## 2. ESAA — Event Sourcing for Autonomous Agents ([arXiv:2602.23193](https://arxiv.org/abs/2602.23193))

**Claim:** Separate LLM *intention* from deterministic *mutation*; CQRS + contracts for software-engineering agents.

| Dimension | Finding |
|-----------|---------|
| **SoT** | **Event store** (`activity.jsonl`). Agents never write project state; they emit `agent.result` / `issue.report`. **Materialized view** `roadmap.json` is CQRS read-model with `projection_hash_sha256`. Repo files are *effects* of orchestrator, not SoT. |
| **Time** | Wall-clock timestamps + `event_seq` total order. Phase/task lifecycle time, not bi-temporal world facts. |
| **Update / invalidation** | **Immutability of done:** completed tasks do not reopen; defects → `issue.report` → hotfix path. Orchestrator validates schema + boundary contracts before append/effect. |
| **Multi-agent / concurrency** | **Empirical:** 4 heterogeneous LLMs, claims serialized into one log; conflict detection for overlapping files is *future work*. Agents get a **purified view**, not raw log. |
| **Audit / fork / replay** | `esaa verify` = reproject + hash compare. Forensic trail + “time-travel debugging” via replay. **No first-class fork-and-diff** like ActiveGraph. |
| **STCA mapping** | **Event Log:** strong (lean vocab, intention vs effect). **Fold/Behaviors:** orchestrator pipeline, not reactive graph behaviors. **Materializations:** single JSON roadmap (not CSR/HNSW). **Agent dict:** **PARCER metaprompts + `AGENT_CONTRACT.yaml`** ≈ Kutha meta-prompt + dictionaries (closest paper to D3). |

---

## 3. Graph-based Agent Memory survey ([arXiv:2602.05665](https://arxiv.org/abs/2602.05665))

**Claim:** Graph is the 2025–2026 frontier for agent memory; taxonomy + lifecycle (extract → store → retrieve → evolve).

| Dimension | Finding |
|-----------|---------|
| **SoT** | Usually the **graph (or hybrid graph+vectors)** is the memory store. Survey treats plain logs/vectors as *degenerate* graphs. SoT is rarely an append-only *execution* log; often LLM-extracted KG over episodes. |
| **Time** | Dedicated coverage of **temporal / bi-temporal KGs**: Graphiti-style valid vs transaction time; invalidation over overwrite; mention-time vs event-time (TReMu); chronological path constraints (MemoTime). |
| **Update / invalidation** | **Memory evolution** as a first-class stage: incremental insert, conflict resolution, temporal close of edges, decay, community refresh. Mix of LLM updaters and rule/RL policies. |
| **Multi-agent / concurrency** | Memory as shared substrate for MAS; less formal concurrency theory than ESAA/ActiveGraph. |
| **Audit / fork / replay** | Provenance/citation to episodes emphasized for some systems (esp. Graphiti lineage). **Deterministic run replay / fork** not the survey’s center of gravity. |
| **STCA mapping** | **Event Log:** under-specified in most systems. **Fold/Behaviors:** “evolution operators,” not reactive physics. **Hot materializations:** embeddings + graph indices as *the* store. **Agent dict:** ontology/schema as knowledge memory, not control dictionaries. |

Useful taxonomy axes from the paper: STM vs LTM; **knowledge vs experience**; non-structural vs structural; implementations = KG / temporal / hyper / hierarchical / hybrid.

---

## 4. Graphs Meet AI Agents survey — memory sections ([arXiv:2506.18019](https://arxiv.org/abs/2506.18019))

**Claim:** Graphs empower planning, execution, **memory**, and multi-agent coordination (and agents empower graph learning).

| Dimension | Finding |
|-----------|---------|
| **SoT** | Memory = graph(+vector) structures: episodic/semantic/community layers. Zep/Graphiti cited as temporal hierarchical KG over conversational data. SoT ≠ agent execution log. |
| **Time** | Temporal-aware hierarchical memory; historical relationships preserved; incremental graph update (HippoRAG, LightRAG, KG-Agent, InstructRAG). |
| **Update / invalidation** | Incremental updates; LLM/RL agents as maintainers. Less emphasis on invalidation-as-behavior than Graphiti product model. |
| **Multi-agent / concurrency** | Graphs as coordination topology (roles, message/dependency graphs)—orthogonal to log serialization. |
| **Audit / fork / replay** | Secondary; focus is capability empowerment, not forensic substrates. |
| **STCA mapping** | Positions **where** graphs help agents; Kutha needs the missing **control-plane SoT** those systems usually omit. Aligns with “graphs for memory/planning”; Kutha adds “log for proof.” |

---

## 5. Causal-Temporal Event Graphs — CTEGs ([arXiv:2604.17557](https://arxiv.org/abs/2604.17557)) — available

**Claim:** Formal model of recursive agent *execution traces* as rooted arborescences with typed, timestamped nodes.

| Dimension | Finding |
|-----------|---------|
| **SoT** | **Trace graph** (CTEG sequence), not domain KG and not embeddings. Formalizes the *record* of execution under single-parent causal semantics. |
| **Time** | Real-valued timestamps; **strict increase along causal paths**; siblings may be simultaneous. Tie-breaking for clock resolution discussed. |
| **Update / invalidation** | Dynamics = **grafting**: direct emissions or subagent invocation grafts. Well-formedness preserved under **partial failure**. Stabilization: full Kleene hierarchy vs opacity at \(\mathscr{E}_1\) if subagent internals are opaque. |
| **Multi-agent / concurrency** | Compositional local→global traces **without central coordinator**; recursive subagents. Not a shared mutable world graph—causal trees of sessions. |
| **Audit / fork / replay** | Natural for chronology/replay of sessions; **Merkle commitments** for tamper-evident verification; RDB encoding. Branching as tree structure (not ActiveGraph-style cheap log fork of a projected world state). |
| **STCA mapping** | **Event Log:** formal *shape* of lineage (caused-by tree, recursion depth). **Fold/Behaviors:** orthogonal (no domain fold). **Materializations:** optional RDB/Merkle. **Agent dict:** none. Strong candidate math for Quantum Receipt / witness chains (ADR-000 D5 packaging). |

---

## 6. Industry baseline — Graphiti / Zep temporal KG  
([Zep temporal KG docs](https://www.getzep.com/ai-agents/temporal-knowledge-graph); engine paper [arXiv:2501.13956](https://arxiv.org/abs/2501.13956))

| Dimension | Finding |
|-----------|---------|
| **SoT** | **Bi-temporal knowledge graph** built continuously from episodes (chat, JSON, docs). Episodes retained (non-lossy episodic subgraph); facts/entities/communities derived. Embeddings used for entity resolve + fact retrieval—not SoT. |
| **Time** | **Bi-temporal:** valid time (world) vs transaction/ingestion time. Product framing: four stamps on edges — **valid from / valid to / observed / recorded**. Point-in-time and “now” queries filter open validity windows. |
| **Update / invalidation** | Contradictions → **invalidate (close) old edges**, do not delete. Dynamic incremental construction. |
| **Multi-agent / concurrency** | Multi-tenant Context Lake / Context Graph Engine; shared user/agent memory at product scale. Not an event-sourced multi-writer theory. |
| **Audit / fork / replay** | Provenance episode→fact; citation. **Not** deterministic agent-run replay/fork of behaviors. Latency-oriented retrieval (sub-200ms p95 claimed at Zep scale). |
| **STCA mapping** | **Event Log:** episodes ≈ soft log of observations, not behavior/tool SoT. **Fold/Behaviors:** extraction/update pipelines. **Hot materializations:** hierarchical KG + vector indices ≈ what Kutha wants as *native* bi-temporal + HNSW **without Graphiti runtime**. **Agent dict:** schema/ontology implicit in extraction prompts—not versioned control dictionaries. |

ActiveGraph’s critique applies: Graphiti is excellent **derived memory**, weak as **whole-run substrate**.

---

## Cross-walk: STCA layers

| Layer | ActiveGraph | ESAA | Graph-mem surveys | CTEG | Graphiti/Zep | Kutha STCA (target) |
|-------|-------------|---------|-------------------|------|--------------|---------------------|
| **Event Log** | Primary SoT | Primary SoT | Rare / episodic only | Formal causal tree | Episodes (obs) | **Sole SoT** (UUID v7) |
| **Fold / Behaviors** | Reactive behaviors + relation-behaviors | Orchestrator + contracts | Evolution operators | Grafting dynamics | Extract/invalidate | Behaviors + relation-behaviors + cascade budgets |
| **Hot materializations** | Working graph only | `roadmap.json` | Graph+vector indices | RDB/Merkle | Hierarchical KG+emb | **Pluggable CSR/HNSW/temporal views** |
| **Agent dict interface** | Packs/prompts | PARCER + YAML contracts | Ontologies as knowledge | — | Prompted extraction | **Meta-prompt@T + external dictionaries@T** |

---

## (a) Taxonomy of memory architectures (2025–2026)

```text
                    AUTHORITY (what is true?)
         ┌─────────────────┼─────────────────┐
    EXECUTION LOG      DOMAIN GRAPH      EMBEDDING / RAG
    (causal proof)     (relational facts) (similarity)
         │                  │                  │
         ├─ ActiveGraph     ├─ Graphiti/Zep    ├─ classical Mem*
         ├─ ESAA            ├─ TKG agents      └─ hybrid add-ons
         └─ CTEG (formal)   └─ survey systems
                             (2602.05665 / 2506.18019)

                    TEMPORALITY
         ┌─────────────────┼─────────────────┐
    ORDER ONLY         BI-TEMPORAL          CAUSAL TREE
    (seq / UUID)       (valid ∩ txn)        (parent→child)
         │                  │                  │
    ActiveGraph/ESAA   Graphiti/Zep         CTEG
                       MemoTime/TReMu

                    CONTROL SURFACE
         ┌─────────────────┼─────────────────┐
    REACTIVE PHYSICS   CONTRACT ORCH.      RETRIEVAL LOOP
    (behaviors)        (JSON schema)       (extract→retrieve)
         │                  │                  │
    ActiveGraph        ESAA/PARCER         most graph-memory
```

**Five clusters that matter for Kutha:**

1. **Log-as-agent** — SoT = ordered events; graph/memory = fold ([2605.21997](https://arxiv.org/abs/2605.21997), [2602.23193](https://arxiv.org/abs/2602.23193)).
2. **Temporal KG memory** — SoT = bi-temporal edges; retrieval-first ([2501.13956](https://arxiv.org/abs/2501.13956), surveys).
3. **Structural/hybrid graph memory** — KG/hierarchy/hypergraph + vectors; evolution lifecycle ([2602.05665](https://arxiv.org/abs/2602.05665), [2506.18019](https://arxiv.org/abs/2506.18019)).
4. **Causal-trace formalisms** — audit shape of recursive sessions ([2604.17557](https://arxiv.org/abs/2604.17557)).
5. **Contracted intention emitters** — LLM proposes; kernel validates ([2602.23193](https://arxiv.org/abs/2602.23193)) — bridges to dict-first control.

---

## (b) Patterns Kutha already aligns with

From ADR-000 / ADR-002 / STCA guide:

- **Log = SoT; graph = fold** — ActiveGraph-aligned ([2605.21997](https://arxiv.org/abs/2605.21997)).
- **Behaviors / relation-behaviors; no direct mutate** — ActiveGraph + STCA Algorithms 1–2.
- **Strict replay + CA cache + fork-and-diff** — ActiveGraph §§4–5; STCA §4.
- **Intention vs effect + schema contracts** — ESAA ([2602.23193](https://arxiv.org/abs/2602.23193)); Kutha D3 validate(dicts+security).
- **Bi-temporal facts + invalidate-not-delete** — Graphiti/Zep reference semantics ([2501.13956](https://arxiv.org/abs/2501.13956)); Kutha D4 native.
- **CQRS / projections ≠ SoT** — ESAA read-model; Kutha D1/D5 materializations.
- **Dict-first / meta-prompt control** — ESAA PARCER/contracts generalized; Kutha D3.
- **Lineage / witness packaging** — ActiveGraph provenance + CTEG Merkle/RDB ([2604.17557](https://arxiv.org/abs/2604.17557)) ↔ Quantum Receipt / RVF-like containers.

---

## (c) Gaps Kutha could uniquely fill

No surveyed system simultaneously owns all four STCA planes. Unique wedge:

1. **Log-primary *and* bi-temporal domain KG** — ActiveGraph has lineage time without world valid/txn; Graphiti has bi-temporal without behavior/tool SoT. Kutha: facts invalidated by **behaviors**, still foldable from one log.
2. **Hot reversible materializations (CSR/HNSW) over the fold** — surveys treat indices as memory SoT; Kutha treats them as **data-plane plugins** with unload/reversibility (D5)—engine product, not agent framework.
3. **Dict-first temporal control plane** — versioned meta-prompt + dictionaries as **graph entities @T**, not only YAML beside a repo agent (stronger than ESAA, orthogonal to Graphiti retrieval memory).
4. **Concurrency story ActiveGraph deferred** — serialize multi-writer contention (ESAA-style) *inside* a reactive behavior runtime; conflict detection on patches as first-class events (ADR-000 open Q).
5. **CTEG-grade causal receipts on industrial storage** — Merkle/arborescence formalisms attached to RocksDB event segments for sealed sessions / forks—audit productization surveys don’t ship.
6. **Self-contained Rust core** — temporal truth without mandatory external Graphiti/LLM for AS OF queries (ADR-001); embeddings optional enrichment.

**One-line positioning:** Kutha STCA = **ActiveGraph control physics** + **Graphiti temporal fact semantics** + **ESAA contract discipline** + **Samyama-class hot projections**, unified under one append-only log.

---

### Primary citations

| ID | Role |
|----|------|
| [2605.21997](https://arxiv.org/abs/2605.21997) | Event-sourced reactive graphs; replay/fork/lineage |
| [2602.23193](https://arxiv.org/abs/2602.23193) | ESAA; intention/orchestrator; multi-agent log; PARCER |
| [2602.05665](https://arxiv.org/abs/2602.05665) | Graph agent memory taxonomy & lifecycle |
| [2506.18019](https://arxiv.org/abs/2506.18019) | Graphs×agents survey (memory & MAS) |
| [2604.17557](https://arxiv.org/abs/2604.17557) | CTEG formal causal-temporal traces + Merkle |
| [2501.13956](https://arxiv.org/abs/2501.13956) | Zep/Graphiti bi-temporal engine (industry baseline) |