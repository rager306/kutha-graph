# ADR-000: Kutha Hybrid Architecture — Research Foundation

## Status

**Proposed** (foundation ADR — D1–D10 locked; honeycomb Proposed; P0 heartbeat+snapshot spike exists; not Accepted product runtime)

### Clarification (2026-10-02; Proposed)

`ValidTime` and `TransactionTime` are still `u64` type aliases in product code (`event.rs`; F7/F13). Typed clocks are M014 scope — this pointer does not rewrite the lock bodies above.

### Clarification (2026-10-03; Proposed)

`ValidTime`, `TransactionTime`, and `WallTime` are newtypes in `crates/kutha-common/src/event.rs` (`Vt` / `Tt` aliases). There is no `crates/kutha-runtime/src/event.rs`. `Op` variants, `Event.ingested_at`, and `Snapshot.next_tt` still carry raw `u64`; typed `Op` clocks are not shipped. The 2026-10-02 “u64 type aliases” line is historical. This pointer does not rewrite D1–D10.

## Date

2026-08-15

## Context

### Problem

A graph engine is needed in which:

1. **agents and GenAI enrichment live inside the runtime**, not as an add-on on top of storage;
2. **time and audit** are first-class (bi-temporal, lineage, point-in-time, fork);
3. **performance** on multi-hop / vector / analytics is comparable to a state-first graph-vector DBMS (reference: Samyama / FalkorDB-class);
4. **extensibility** — plugins with reversible effects, without rewriting the core.

Classic options do not cover everything at once:

| Approach | Strength | Weakness for the goal |
|--------|-----------------|-------------------|
| State-first graph DB (Samyama-like) | Throughput, CSR, MVCC, HNSW | Agents / full history / fork — add-on |
| Pure event-sourced reactive graph (ActiveGraph) | Lineage, replay, behaviors | Write amp, latency multi-hop, storage bloat |
| Memory frameworks (Graphiti/Zep, Mem0) | Temporal agent memory | Not a DBMS; depend on an external graph |
| Document + in-DB GenAI (RavenDB) | Agents inside DB | Weak native graph |

### Synthesis sources (research inputs)

- **ActiveGraph**: event log = SoT; graph = deterministic projection; behaviors.
- **Cordis**: Everything is a Plugin; spatiotemporal composability; reversible effects.
- **Cui («背包问题九讲»)**: generalized items → max-convolution / composition of local transition functions.
- **Samyama**: RocksDB, arenas/MVCC, CSR, vectorized executor, cost planner, enrichment.
- **FalkorDB**: sparse matrix / GraphBLAS-like hot path, low-latency traversal + vector.
- **Graphiti / Zep**: bi-temporal edges, fact invalidation, episodic ingestion.
- **RavenDB GenAI**: continuous GenAI tasks + server-side agents with tools.
- **ruVector / RVF**: HNSW+GNN, agent memory types, COW, witness chain, portable cognitive containers.
- **HelixDB**: indexes as access paths; tiered storage.
- **Tarantool**: compute-close-to-data; hot in-mem / cold disk.

### Target verticals (priority order)

1. Legal temporal agents (bi-temporal norms, deterministic audit)
2. Regulated finance / compliance
3. Scientific / biomedical knowledge evolution
4. Horizontal agent runtime (shared temporal memory)
5. Enterprise knowledge + security plane

### Naming

Working name of the product/repo: **Kutha** / **kutkha** (the Kamchatkan creator-raven; the tech namespace is relatively clean). The `.com/.io/.ai` domains are taken; `.dev` is a candidate. The final brand is a separate ADR.

---

## Decision

**Adopted research hypothesis (working architecture):**

> Kutha is a hybrid AI-native graph engine on Rust: **event-sourced reactive core** (control / agent / lineage plane) + **pluggable high-performance materializations** (data plane) + **meta-prompt + external dictionaries** as the universal agent control layer.

This is **not** “Samyama on top of ActiveGraph” and **not** a pure replacement for Samyama. It is a differentiated product in the *high temporal × high hybrid × in-DB agents* quadrant.

### D1. Two-plane model

```
Client / Cypher + Hybrid + Temporal AS OF + Agent API
                    │
        Behavior & Agent Runtime (Cordis-style)
        • Behaviors / Relation Behaviors
        • GenAI Enrichment (RavenDB-style)
        • Meta-prompt + Dictionaries (control)
        • Spatiotemporal composition + reversible effects
                    │ emit / react
        Append-only Event Log (SoT, lineage, fork)
                    │ project
        Materialized Projections (pluggable, reversible)
        • Hot CSR / sparse / GraphBLAS-like
        • HNSW (+ optional GNN)
        • Bi-temporal views
        • Agentic memory views
        • Secondary / FTS / property columns
                    │
        Storage: RocksDB / object store + WAL (+ optional Raft)
```

| Plane | SoT / role | Is not |
|-----------|------------|-------------|
| **Control** | Event log + behaviors | Hot adjacency layout |
| **Data** | Materializations | Source of truth |
| **Agent** | Meta-prompt + dictionaries + memory packs | A free LLM without validation |

### D2. Formal reactivity model

\[
L = [e_1,\ldots,e_n],\quad
e_i=\langle id, type, payload, causedBy\rangle
\]

\[
G(L)=\mathrm{foldl}(\mathrm{apply\_event}, G_0, L)
\]

\[
B:(e,G)\to \mathrm{Option}(Patch)
\]

- Direct mutate of the graph is forbidden; only patch → validate → `patch.proposed` / `patch.applied`.
- **Relation Behaviors** encapsulate coordination on edges.
- A module/pack ≈ **Cui generalized item** \(h_i\); global logic is a composition of local functions (max-convolution / analog).
- Runtime quantum: emit → log+project → trigger \(B\)/\(R_B\) → cascade until stabilization.
- Identifiers: **UUID v7** (monotonicity + indexability).

### D3. Agent control: meta-prompt + dictionaries (not hard FSM)

A hard FSM as the sole model is rejected. A universal layer is adopted:

1. **Meta-prompt** — a versioned “constitution” (modes, tool policy, temporal semantics, conflict handling); itself temporal.
2. **External dictionaries** (first-class temporal graph entities):
   - Controlled vocabulary
   - Action dictionary
   - Relation dictionary
   - State / mode dictionary
   - Policy dictionary
   - Domain ontology / schema

**Cycle:** request → load meta-prompt@T → load dictionaries@T → propose → validate(dicts+security) → execute → event log.

FSM / statecharts are **derivable** from the State+Action dictionaries (for audit/visualization), not hardcoded in code.

Enterprise: ABAC + temporal policies + agent capability sandbox; policy-as-graph.

Clarification (2026-10-03): five reserved facts kutha:allows|rule|policy|admission|action exist in the spike; the six agent dictionaries are frozen (see ADR-050).

### D4. Bi-temporal as fact semantics

Every fact/edge: `valid_from`, `valid_to`, `ingested_at`, `invalidated_at` (+ provenance).  
Invalidation is a behavior, not a silent overwrite. Query: `AS OF`, temporal filters — native.

### D5. Materializations as plugins

CSR/HNSW/temporal slices/memory views:

- incrementally maintained by behaviors;
- **reversible** (unload → rollback of side effects);
- allow competing projections.

RVF-like containers: portable agent memory, sealed knowledge units, checkpoint/fork with a witness chain — **transport / packaging layer**, not a replacement for the hot RocksDB/CSR path.

### D6. Core language and runtime

| Layer | Choice | Why |
|------|--------|--------|
| Core engine | **Rust + Tokio + RocksDB (event store)** | No GC, workspaces=boundaries, serde, channels |
| Agent I/O / early composition | A TS/Cordis-like SDK is admissible | I/O-bound LLM; not a materializer |
| Java / C# | Enterprise app wrapping | Not the engine core |

Compile-time hexagon: crates + `pub(crate)`. Behavior errors → `behavior.failed` (`Result`), not panic. A content-addressed cache of external tool/LLM calls is mandatory for cheap replay/fork.

### Clarification (2026-10-03; Proposed)

The implemented spike stores `events.jsonl` as SoT with `events.wal` (magic `KUTHAWAL`) as a covering lease (`crates/kutha-runtime/src/store.rs` `load_events_d04`). `snapshot.json` and `events.wal` are replaced via tmp+sync+rename. RocksDB is NOT a dependency (`crates/kutha-runtime/Cargo.toml`) and stays an optional index store behind the frozen M002 durability protocol. This pointer does not rewrite D1–D10.

### D7. Load profiles (one product — three modes)

| Profile | RAM | CPU | Disk | Design implication |
|---------|-----|-----|------|------------------------|
| **A OLTP** | low (aggregates) | serde / outbox | **WAL IOPS** | lean events, hot entity cache |
| **B Temporal KG** | **critical** (in-mem graph) | fold / Cypher / as-of | read-heavy + temporal idx | CSR projection residency |
| **C Agents** | medium | low on the host | **lineage volume** | tiered archive, compression |

### D8. Mandatory antipattern controllers

1. **Replay tax \(O(N)\)** → snapshots + log segmentation.  
2. **Single-writer bottleneck** → RocksDB/SQLite WAL, batching.  
3. **Fat events** → lean deltas + ids; blobs via object store / read ports.

### D9. Testing as a first-class capability

| Class | Purpose |
|-------|------------|
| Sociable unit + in-memory ports | Use-cases without a mock zoo |
| Strict replay | `ReplayDivergenceError` |
| Fork & diff | Counterfactual |
| Adapter + Testcontainers | SQLx/Postgres etc. |

### D10. Positioning (value prop)

> Event-sourced bi-temporal graph engine on Rust where agents are first-class citizens, governed by meta-prompts and external dictionaries, with performance delivered by pluggable materializations — not agents bolted onto storage.

White space vs Graphiti/Zep (memory layer), FalkorDB (perf, weak agents/temporal), RavenDB (agents, weak graph), Samyama (perf + enrichment as add-on).

---

## Consequences

### Becomes easier

- Audit, PITR, fork, legal/finance explainability.
- Adding indexes / enrichment / solvers as packs.
- Replay/fork tests instead of heavy testbeds.
- Universal agent control across verticals (dictionaries change, not the runtime).

### Becomes harder

- Two planes (log + materializers) — more consistency semantics.
- Incremental materialization and conflict detection — R&D cost.
- In-DB agents → a hard security plane (otherwise unshippable for enterprise).
- Hiring/development speed on Rust is higher in people, lower in infra TCO.

### Fit assessments (research)

| Direction | Assessment |
|-------------|--------|
| Pure Samyama replacement on raw throughput | 5–6/10 |
| AI-native / agentic KG + GraphRAG + multi-agent memory | 8.5–9.5/10 |
| Temporal / auditable / regulated | 9/10 |
| Hybrid (this ADR) | 8–9/10 |

---

## Alternatives Considered

| Alternative | Verdict |
|--------------|---------|
| Pure state-first Samyama + bolted agents | Rejected as a product; borrow the materializers |
| Pure ActiveGraph without hot projections | Rejected for B/C scale |
| Hard FSM as the sole agent control | Rejected; FSM is derivable from dictionaries |
| TS/Node as the graph core | Rejected for B; admissible for agent I/O |
| External ETL + external agent orchestrator | Rejected as the target ideology; ingestion = reactive packs |
| RVF as primary storage | Rejected; RVF = portable/cognitive packaging |

---

## Open Research Questions

Questions that **must** spawn a follow-on ADR or spike before Accepted status of the corresponding decisions.

### R1. Consistency & concurrency

- Which model between log append and projection apply? (single-writer serial / sharded logs / causal)
- How to detect patch conflicts under parallel behaviors?
- MVCC equivalent on versioned events vs arena-MVCC only in the projection?

### R2. Materialization protocol

- API reversible plugin: mount / apply_delta / snapshot / unload / rollback.
- Incremental CSR/HNSW update cost vs periodic rebuild.
- Several competing projections of one type — query routing semantics.

### R3. Bi-temporal query & storage

- Physical layout: edge versions vs separate temporal index.
- AS OF + multi-hop cost model.
- Invalidation cascade bounds (do not explode reactive fan-out).

### R4. Agent control plane

- Entity schema: `MetaPrompt`, `Dictionary`, `DictionaryEntry`, `AgentInstance`.
- Who is in charge: LLM propose vs dictionary validate (fail-closed).
- Capability model vs RBAC; policy-as-graph schema.
- Link of meta-prompt@version to the decision's witness/provenance.

### R5. Event schema & storage

- Lean event taxonomy (create/patch/invalidate/behavior.* / llm.*).
- Snapshot format; segment compaction; tiered cold storage.
- Content-addressed LLM/tool cache keys and eviction.

### R6. Query surface

- Cypher subset + temporal extensions + hybrid (vector+graph+FTS) composition.
- Planner: cost across projections; late materialization borrow from Samyama ADR-012.

### R7. Security & multi-tenancy

- ABAC attributes on node/edge/temporal slice/agent action/container.
- Sandbox for in-DB agents (WASM? process? crate-level?).
- Shared public knowledge (law corpus) + private tenant data.

### R8. RVF / portable units

- Which subset of segments is needed in v1 (WITNESS, VEC, GRAPH, META, WASM)?
- Mapping: graph fork ↔ RVCOW; event segment ↔ RVF file.

### R9. Benchmarks & success metrics

- Legal PIT: correctness + latency for norm@date.
- Agent replay: divergence rate; fork cost.
- Graph: hops/s and ingest vs Samyama/FalkorDB baselines (the goal is not to always outrun — the goal is “close on the hot path”).

### R10. Branding & licensing

- kutkha vs kutha vs alternatives; trademark pass.
- License for core vs agent packs vs legal ontology packs.

### Open question (2026-09-30)

Pointer only, from docs/architecture/semantic-gap-review.md (F7, D4): D4 names `valid_from` / `valid_to` / `ingested_at` / `invalidated_at` and native AS OF. It does not declare a calendar or epoch scale, nor a mapping from transaction-time to wall time. This question does not rewrite D1–D10 lock wording.

---

## Research & Detailing Backlog

Detailing order (each → a separate ADR or spike report).

| Phase | Topic | Output | Depends |
|-------|------|-------|---------|
| **P0** | Event log + apply + behavior quantum (Rust spike) | Working mini-runtime; **ADR-010** (+ Vision/STCA ADR-001/002) | — |
| **P0** | Lean event schema + UUID v7 + caused_by | **ADR-011** | P0 runtime |
| **P0** | Snapshot + segment strategy | **ADR-012** | P0 log |
| **P1** | Bi-temporal edge model + invalidation behavior | **ADR-013** | P0 |
| **P1** | CSR materialization plugin (reversible stub) | **ADR-040** | P0 |
| **P1** | Meta-prompt + dictionary entity model | **ADR-050** | P0 |
| **P2** | HNSW plugin + hybrid retrieve stub | ADR Hybrid Query | P1 CSR |
| **P2** | GenAI enrichment behavior (RavenDB-pattern) | ADR Enrichment | P1 agent |
| **P2** | Security plane sketch (ABAC + capabilities) | ADR Security | P1 agent |
| **P3** | Legal reference vertical (norm@date MVP) | Case study + API | P1 temporal, P1 agent |
| **P3** | RVF packaging subset | ADR Portable Containers | P0–P2 |
| **P3** | Benchmark harness vs baselines | BENCHMARKS.md | P1–P2 |
| **P4** | Raft / multi-node (optional) | ADR Distribution | storage mature |
| **P4** | GNN learned index (ruVector-inspired) | ADR Learned Indexes | P2 HNSW |

### Exit criteria from Research → Proposed for this ADR

- [x] Spike P0 runtime demonstrates emit → cascade → stable graph + replay.
- [x] Lean event types and snapshot story are recorded. *(lean events: `kutha-common`; snapshot = fold+dict+offset lease, JSONL semantic log; Rocks WAL still crash cousin, not SoT)*
- [x] Draft ADRs Temporal + Materialization API + Agent Control are written. *(honeycomb 010–014, 040–043, 050 Proposed)*
- [x] Explicitly measured: replay time vs log length; cascade depth bounds. *(2026-08-18: `replay_check` log_len=200 elapsed_ms≈2.0; `max_cascade` abort covered)*
- [x] Security threats for in-DB agents are listed and have a mitigation sketch. *(ADR-051/080/081 Proposed — sketch, not kernel)*

---

## Follow-on ADR Skeleton (to open)

**Superseded numbering (2026-08-16):** the spine is now **ADR-001 Overall Vision** → **ADR-002 STCA**, then **honeycomb cells ADR-010+** (see `docs/ADR/README.md`). Historical working titles below are remapped; do not open new ADRs under the old 001–012 runtime list.

| Old sketch | New cell (working) |
|------------|--------------------|
| ADR-001 Event Log & Reactive Runtime Quantum | **ADR-010** (Time band) |
| ADR-002 Lean Event Schema & Lineage | **ADR-011** |
| ADR-003 Snapshots, Segments & Tiered Storage | **ADR-012** |
| ADR-004 Bi-Temporal Facts & Invalidation | **ADR-013** |
| ADR-005 Materialization Plugin Protocol | **ADR-040** (Data band) |
| ADR-006 Meta-Prompt & Dictionary Agent Control | **ADR-050** (Agent band) |
| ADR-007 Capability Security Plane | **ADR-080** / **ADR-051** |
| ADR-008 Hybrid Query Surface | **ADR-070** / **ADR-043** |
| ADR-009 GenAI Enrichment Behaviors | **ADR-052** |
| ADR-010 Portable Cognitive Containers (RVF) | **ADR-091** |
| ADR-011 Legal Vertical Reference Architecture | **ADR-090** |
| ADR-012 Naming, Branding & License | **ADR-092** |

Spine (open / Research):

| ID | Title |
|----|--------|
| [ADR-001](./ADR-001-kutha-overall-vision.md) | Kutha Overall Vision (STCA on top of idea stack) |
| [ADR-002](./ADR-002-stca-paradigm.md) | Spatiotemporal Compositional Architecture (STCA) |

---

## References

### Internal synthesis (session 2026-08-15)

- Grok dialogue: hybrid synthesis, RVF, naming (kutkha), competitors, legal/scientific GTM, FSM → meta-prompt+dictionaries.
- ActiveGraph + Cordis + Cui formalization + Rust prototype sketch.
- Resource profiles A/B/C; Rust/Java/C#/TS comparison.
- Rust reliability / async / bottlenecks / replay-fork testing / strangler scale.

### External systems (comparative)

Graphiti/Zep, FalkorDB, HelixDB, Samyama, ruVector/RVF, RavenDB GenAI, Tarantool, Mem0, Letta; architecture lists (awesome-software-architecture, system-design-primer) as general background only.

### Related Samyama ADRs (borrow / contrast)

- ADR-001 Rust, ADR-002 RocksDB, ADR-006 Tokio — align.
- ADR-012 Late materialization, ADR-015 planning, ADR-020 MVCC — contrast: SoT in Kutha is log, not arena state.
- ADR-016 distributed — defer until single-node hybrid proven.

---

## Related Decisions

- None yet (root research ADR).
- Supersedes: informal notes from Grok share “DeepSeek Harness: Plugin Paradigm for AI”.

---

## One-liner

**Kutha = event log as truth + reversible projection plugins for speed + meta-prompt/dictionaries for in-DB agents — researched on Rust, validated by replay/fork, aimed at legal/regulated temporal agent workloads first.**
