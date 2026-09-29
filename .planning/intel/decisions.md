# Synthesized decisions (ADR)

All ingested ADRs have classification `locked: false` and source Status **Proposed** (not Accepted). GSD `status` is therefore `proposed`. ADR-000 marks D1–D10 as foundation locks in narrative; those entries remain `proposed` (not GSD locked).

## ADR-000: Kutha Hybrid Architecture — Research Foundation
- source: docs/ADR/ADR-000-kutha-hybrid-architecture-research.md
- status: proposed
- decision: Kutha is a hybrid AI-native graph engine on Rust: event-sourced reactive core (control / agent / lineage plane) + pluggable high-performance materializations (data plane) + meta-prompt + external dictionaries as the universal agent control layer. Not Samyama-on-ActiveGraph and not a pure Samyama replacement. D1–D10 are named in this ADR as foundation locks; honeycomb and product runtime remain Proposed / not Accepted.
- scope: Kutha, hybrid architecture, event log, D1–D10, materializations, meta-prompt, dictionaries, bi-temporal, CSR, HNSW, Rust, agent runtime

## ADR-000 D1: Two-plane model
- source: docs/ADR/ADR-000-kutha-hybrid-architecture-research.md
- status: proposed
- decision: Control plane SoT is the event log + behaviors (not hot adjacency). Data plane is pluggable reversible materializations (not SoT). Agent plane is meta-prompt + dictionaries + memory packs (not an unconstrained LLM). Storage named in this ADR: RocksDB / object store + WAL (+ optional Raft). Client surface named: Cypher + Hybrid + Temporal AS OF + Agent API.
- scope: control plane, data plane, agent plane, event log, materializations

## ADR-000 D2: Formal reactivity model
- source: docs/ADR/ADR-000-kutha-hybrid-architecture-research.md
- status: proposed
- decision: Log L = [e1…en] with e_i = ⟨id, type, payload, causedBy⟩; G(L) = foldl(apply_event, G0, L); B:(e,G)→Option(Patch). Direct graph mutate is forbidden; only patch → validate → patch.proposed / patch.applied. Relation Behaviors coordinate on edges. Pack ≈ Cui generalized item h_i; global logic is composition / max-convolution. Runtime quantum: emit → log+project → trigger B/R_B → cascade to idle. Identifiers: UUID v7.
- scope: event log, fold, behaviors, UUID v7, runtime quantum

## ADR-000 D3: Agent control via meta-prompt + dictionaries (not hard FSM)
- source: docs/ADR/ADR-000-kutha-hybrid-architecture-research.md
- status: proposed
- decision: Hard FSM as the sole control model is rejected. Adopt versioned temporal Meta-prompt plus six external dictionaries as first-class temporal graph entities (Controlled vocabulary, Action, Relation, State/mode, Policy, Domain ontology/schema). Cycle: request → load meta-prompt@T → load dictionaries@T → propose → validate(dicts+security) → execute → event log. FSM/statecharts are derived from State+Action dictionaries, not hardcoded. Enterprise names ABAC + temporal policies + capability sandbox; policy-as-graph.
- scope: meta-prompt, dictionaries, D3 agent cycle, FSM derived

## ADR-000 D4: Bi-temporal fact semantics
- source: docs/ADR/ADR-000-kutha-hybrid-architecture-research.md
- status: proposed
- decision: Every fact/edge carries valid_from, valid_to, ingested_at, invalidated_at (+ provenance). Invalidation is a behavior, not silent overwrite. Query AS OF and temporal filters are native.
- scope: bitemporal facts, valid-time, invalidation, AS OF

## ADR-000 D5: Materializations as plugins
- source: docs/ADR/ADR-000-kutha-hybrid-architecture-research.md
- status: proposed
- decision: CSR/HNSW/temporal slices/memory views are incrementally maintained by behaviors, reversible (unload rolls back side effects), and may compete. RVF-like containers are a transport/packaging layer, not a replacement for the hot RocksDB/CSR path.
- scope: materializations, CSR, HNSW, RVF packaging

## ADR-000 D6: Core language and runtime
- source: docs/ADR/ADR-000-kutha-hybrid-architecture-research.md
- status: proposed
- decision: Core engine is Rust + Tokio + RocksDB (event store). TypeScript/Cordis-like SDK is allowed for agent I/O only, not as materializer. Java/C# is enterprise wrapping, not the engine core. Compile-time hexagon: crates + pub(crate). Behavior errors → behavior.failed (Result), not panic. Content-addressed cache of external tool/LLM calls is mandatory for cheap replay/fork.
- scope: Rust, Tokio, RocksDB, content-addressed cache

## ADR-000 D7: Load profiles (one product, three modes)
- source: docs/ADR/ADR-000-kutha-hybrid-architecture-research.md
- status: proposed
- decision: Profile A OLTP — WAL IOPS, lean events, hot entity cache. Profile B Temporal KG — RAM-critical in-mem graph, fold/Cypher/as-of, CSR projection residency. Profile C Agents — lineage volume, tiered archive, compression.
- scope: OLTP, temporal KG, agents, load profiles

## ADR-000 D8: Antipattern controllers
- source: docs/ADR/ADR-000-kutha-hybrid-architecture-research.md
- status: proposed
- decision: Replay tax O(N) → snapshots + log segmentation. Single-writer bottleneck → RocksDB/SQLite WAL, batching. Fat events → lean deltas + ids; blobs via object store / read ports.
- scope: snapshots, WAL, lean events

## ADR-000 D9: Testing as first-class capability
- source: docs/ADR/ADR-000-kutha-hybrid-architecture-research.md
- status: proposed
- decision: Required test classes: sociable unit + in-memory ports; Strict replay (ReplayDivergenceError); Fork & diff (counterfactual); Adapter + Testcontainers.
- scope: strict replay, fork-and-diff, testing

## ADR-000 D10: Positioning (value prop)
- source: docs/ADR/ADR-000-kutha-hybrid-architecture-research.md
- status: proposed
- decision: Event-sourced bi-temporal graph engine on Rust where agents are first-class citizens, governed by meta-prompts and external dictionaries, with performance delivered by pluggable materializations — not agents bolted onto storage. White space vs Graphiti/Zep, FalkorDB, RavenDB, Samyama as named in the source.
- scope: positioning, value prop

## ADR-001: Kutha Overall Vision
- source: docs/ADR/ADR-001-kutha-overall-vision.md
- status: proposed
- decision: Kutha is a self-contained Rust graph engine: append-only event log is SoT; graph is a deterministic fold; domain capability is vertical packs behind ports; hot CSR/HNSW/temporal views are reversible materializations, not a second SoT; agents are governed by versioned meta-prompts and dictionaries (LLM optional, never sole authority for audited fact truth). Idea stack top→down: STCA (ADR-002), Kutha product deltas, honeycomb cells (ADR-010+). Non-goals: required client of Neo4j/FalkorDB/Graphiti/cloud LLM for core assert→invalidate→AS OF; pure ActiveGraph without hot projections; pure state-first DB with agents bolted on; RVF as primary hot storage. ADR-000 keeps D1–D10; ADR-001 does not reopen those locks.
- scope: Kutha, STCA, event log, graph engine, Rust, honeycomb ADRs, vertical packs, materializations, dict-first agents

## ADR-002: Spatiotemporal Compositional Architecture (STCA)
- source: docs/ADR/ADR-002-stca-paradigm.md
- status: proposed
- decision: Adopt STCA as Kutha’s architectural paradigm. Space: vertical slices; hexagon inside a module; modules talk only through Ports; pack effects reversible where they touch projections. Time: event log = sole SoT; G_t = foldl(apply, G0, L_t); no direct mutate; Behaviors/Relation Behaviors until idle or budget exhausted; UUID v7. Composition: packs expose h_i(v); global allocation uses max-convolution, not a hard-coded workflow engine. Verification: strict replay, fork-and-diff, optional regimes gated loops. Detail algorithms and Cargo isolation laws: docs/architecture/stca-guide.md. STCA does not replace hot materializations (D5/040+), native bi-temporal facts (013), dict-first agents (D3/050+), or self-contained core (ADR-001).
- scope: STCA, event log, vertical slices, ports, behaviors, packs, max-convolution, Kutha

## ADR-010: Event Log & Reactive Runtime Quantum
- source: docs/ADR/ADR-010-event-log-runtime-quantum.md
- status: proposed
- decision: D010-1: append-only event log is the only source of temporal truth; graph/CSR/HNSW/indexes/receipts are droppable leases; LLM never writes truth. D010-2: one quantum is admit → append → project → trigger B/R_B → cascade until idle or ADR-014 budget; idle is a scoped completion claim (fixed point of declared rules), not an empty queue; budget exhaustion is incomplete evaluation. D010-3: writes are typed assert/retract/correct (TGMS direction), not free MERGE-as-truth. D010-4: RocksDB WAL is the crash cousin; semantic replay is fold-from-log. Hard separations: event log ≠ fold; semantic replay ≠ WAL recovery; typed writes ≠ LLM narrative; quantum ≠ workflow DAG; lease ≠ second database.
- scope: event log, runtime quantum, deterministic fold, WAL recovery, TGMS operators, behaviors, droppable leases

## ADR-011: Lean Event Schema & Lineage
- source: docs/ADR/ADR-011-lean-event-schema-lineage.md
- status: proposed
- decision: D011-1: lean events; N-object (OCPM); no required case_id; UUID v7 for events; hop keys interned. D011-2: intern map is a lease, not SoT, and not D3 dictionaries. D011-3: how-provenance is a semiring polynomial (derivation lease, not log integrity). D011-4: caused_by/witness ≠ agent dictionaries or ABAC. Clarification (Proposed): semantic recovery of term meanings from retained history; separate event/claim/support/delivery identities; typed dependency links; n-ary incidence; thin P→Q eligibility (M011 S03 named in source).
- scope: lean event schema, intern map, how-provenance polynomials, lineage, OCPM, UUID v7, caused_by, claim_id, semantic recovery

## ADR-012: Snapshots, Tiers, and Vacuum
- source: docs/ADR/ADR-012-snapshots-tiers-vacuum.md
- status: proposed
- decision: D012-1: snapshots are leases of the log, placed by query load; layout compaction allowed; silently dropping losers is TOKI-illegal. D012-2: aging pipeline mutable adj → frozen CSR (BACH-shaped); LSM levels are not truth. D012-3: tiers in-RAM fold, semi-external edge shards, cold archive; crash recovery from the log. D012-4: vacuum ≠ compaction; legal hold pins SoT; LLM does not choose what to forget; P0 may be append-only-forever. D012-5: temporal vector tiering is a Proposed adapter mapping (ruvector-temporal-tensor), droppable lease, does not authorize HNSW or M002.
- scope: snapshots, vacuum, legal hold, CSR aging, semi-external graph tiers, event log SoT, LSM compaction, temporal vector tiering

## ADR-013: Bi-temporal Facts & Invalidation
- source: docs/ADR/ADR-013-bitemporal-facts-invalidation.md
- status: proposed
- decision: D013-1: native VT × TT on facts; legal five-clock stamps (ADR-090) are typed annotations, not five peer algebraic axes. D013-2: invalidation is typed; losers remain queryable until vacuum. D013-3: supersession is deterministic, not cosine/RAG; LLM is not on the write path except as a logged proposal that still must validate. D013-4: interval index is a droppable lease; T-GQL is a language cousin; query surface is ADR-070. Graphiti edges ≠ native fact semantics.
- scope: bitemporal facts, valid-time, transaction-time, invalidation, supersession, interval index, TOKI contradiction operators, GraphFold live_at/as_of

## ADR-014: Cascade Budgets & Quantum Receipts
- source: docs/ADR/ADR-014-cascade-budgets-quantum-receipts.md
- status: proposed
- decision: D014-1: one constant-size receipt per emit→idle quantum (or budget-abort); certifies that the quantum existed; not how-provenance and not an LLM envelope certificate. D014-2: budgets compose by max-convolution, not a workflow engine; LLM does not pick v; pack scheduling under V is ADR-031. D014-3: optional Merkle/anchor ≠ SoT; public blockchain as graph store is rejected. D014-4: receipt may bind meta_prompt_version and dictionary snapshot ids; CA cache keys belong in quantum evidence. D014-5: read provenance receipts are a separate Proposed adapter (ruvector-retrieval-receipt) when a query surface exists. Partial progress is permitted; budget stop ≠ rollback promise.
- scope: quantum receipts, cascade budgets, max-convolution, emit→idle, Merkle anchors, retrieval receipts, meta-prompt versioning

## ADR-020: Cargo Workspace & Port Isolation
- source: docs/ADR/ADR-020-cargo-workspace-port-isolation.md
- status: proposed
- decision: D020-1: workspace = compile-time Space; target shape from STCA guide: app-shell (sole composition root) + common + modules/<slice> with domain/internal/adapters/Ports; hot path has no mandatory network adapters. D020-2: isolation laws — domain ignorance, private-by-default, single composition root; repo-wide ports/adapters folders as primary structure are rejected. D020-3: runtime Space (named graphs, tenant banks) is not the crate map; a crate is not a tenant.
- scope: Cargo workspace, Port isolation, STCA Space, app-shell, domain/internal/adapters, compile-time isolation, named graphs, tenant slices

## ADR-021: Pack / Plugin Lifecycle
- source: docs/ADR/ADR-021-pack-plugin-lifecycle.md
- status: proposed
- decision: D021-1: a pack is a versioned artifact on the log (operators + optional dictionary facets + lease registrations); lifecycle events PackInstall / PackActivate / PackRetire; activate appends events, does not mutate SoT otherwise; rollback = retire + drop leases. D021-2: pack ≠ view (ADR-040) and pack ≠ schema SMO (ADR-050). D021-3: packs register into Kutha; they do not vendor DuckDB/Neo4j/Graphiti as SoT.
- scope: pack lifecycle, PackInstall, PackActivate, PackRetire, leases, dictionaries, operators, schema SMO

## ADR-022: Vertical-Slice Module Conventions
- source: docs/ADR/ADR-022-vertical-slice-module-conventions.md
- status: proposed
- decision: D022-1: slice = one module crate with hexagon inside. D022-2: slices communicate only via Ports + the log; shared kernel types in common. D022-3: default vs named graph is compile scope; compiled queries must not silently union every pack; tenant quotas stay ADR-080.
- scope: vertical slice, hexagon, Ports, event log, named graphs, pack crates, kutha-common, materializers, WorldDB, tenant quotas

## ADR-030: Cui Budget Traits / Max-Convolution Allocation
- source: docs/ADR/ADR-030-cui-budget-traits.md
- status: proposed
- decision: D030-1: packs expose h_i(v) as a Port; global envelope is tropical max-convolution; LLM does not pick v; hard FSM is not the allocator. D030-2: P0 may use a concave/greedy envelope; do not pretend CNN convolution. D030-3: SPARQL MQO-class work sharing under V is allowed, not Dify/oxify orchestration. Clarification: scalar H(V)=max(sum h_i(v_i)) subject to sum v_i ≤ V assumes one additive resource and separable utilities; greedy is exact only under stated conditions; no general Cui allocator is claimed for current P0 or harness slice-count budget.
- scope: Cui budget traits, max-convolution allocation, h_i(v) ports, quantum receipt algebra, multi-query work sharing, Composition honeycomb

## ADR-031: Pack Scheduling Under V
- source: docs/ADR/ADR-031-pack-scheduling-under-v.md
- status: proposed
- decision: D031-1: after compile, compare cost envelope to remaining V; outcomes run / queue / reject; reject is a logged why-not; LLM is not the governor. D031-2: scheduling under V ≠ building V (030/014 compose h_i). D031-3: P0 may be unlimited; do not train a learned governor as P0.
- scope: pack scheduling, cost envelope V, admission control, materializers, Composition axis, CASA

## ADR-040: Materialization Plugin Protocol
- source: docs/ADR/ADR-040-materialization-plugin-protocol.md
- status: proposed
- decision: D040-1: every hot picture is a lease of the fold (CSR, HNSW, interval indexes, named-graph slices, Cypher views); reversible = drop and fold again; competing projections allowed; routing is ADR-043. D040-2: view contract build / apply(delta) / snapshot / unload / rollback ≠ pack lifecycle (ADR-021). D040-3: IVM/DBSP is maintenance algebra, not a second log. D040-4: mandatory external graph DBs are forbidden as materializers-of-truth. Clarification: lease identity binds branch/prefix, log offset, TT/VT cut, schema/term interpretation, projection version, relation filter, and set/bag/provenance mode; current CsrLease::from_fold is an untyped neighbor set (named in source).
- scope: materialization plugin protocol, view lease contract, CSR, HNSW, IVM, DBSP, fold at offset, MaterializerPort

## ADR-041: CSR / GraphBLAS Hot Path
- source: docs/ADR/ADR-041-csr-graphblas-hot-path.md
- status: proposed
- decision: D041-1: Kutha owns CSR (or GraphBLAS-equivalent sparse adj) over interned ids; Samyama is kernel evidence, not Kutha SoT; TypeScript is not the graph core (D6). D041-2: conjunctive MATCH uses LFTJ-class WCOJ; this cell does not invent another WCOJ algorithm. D041-3: late materialization — hops on compact ids/CSR. D041-4: GraphBLAS is an API shape, not a second SoT; Falkor remains spec until a tree is read.
- scope: CSR, GraphBLAS, LFTJ, leapfrog triejoin, frozen adjacency, interned ids, late materialization, hot path

## ADR-042: HNSW Access-Method Fence
- source: docs/ADR/ADR-042-hnsw-access-method-fence.md
- status: proposed
- decision: D042-1: HNSW is a droppable pack, never temporal truth; cosine cannot supersede facts. D042-2: delete-repair is neighbor rewiring / index maintenance under ADR-040, not constraint repair. D042-3: filtered kNN uses existing HNSW (ACORN/NaviX as literature poles); this cell does not invent a specialized hybrid structure; Compass/SIEVE is ADR-043. D042-4: ports own the contract; RuVector is an adapter; do not vendor the RuVector monorepo; .kutha/STATE.md still forbids implementing this cell until HNSW is named.
- scope: HNSW, vector access method, RuVector, filtered kNN, event log, delete-repair, interned node ids

## ADR-043: Hybrid Query Planner
- source: docs/ADR/ADR-043-hybrid-query-planner.md
- status: proposed
- decision: D043-1: planner is a pack over leases, not an LLM; LLM may propose a plan; the engine validates and costs it. D043-2: no new join algorithm in this cell; exact MATCH stays LFTJ-class (041); fail-closed if the plan cannot guarantee exact MATCH. D043-3: hybrid = cooperative existing indexes (Compass-shaped); SIEVE-shaped collections are catalog policy under 040. D043-4: cardinality is the missing kernel; P0 may ship naive/AGM-pessimistic estimates; learned GNN CardEst is demand, not P0.
- scope: hybrid query planner, leases, LFTJ, HNSW, cardinality estimation, index collection, exact MATCH

## ADR-050: Meta-Prompt & Dictionaries
- source: docs/ADR/ADR-050-meta-prompt-dictionaries.md
- status: proposed
- decision: D050-1: six dictionary kinds as bi-temporal graph entities (Controlled vocabulary, Action, Relation, State/mode, Policy, Domain ontology/schema) plus MetaPrompt, Dictionary, DictionaryEntry, AgentInstance. D050-2: D3 cycle is fail-closed; LLM proposes; dictionaries validate; FSM derived, not hard-coded sole control. D050-3: intern map (ADR-011) ≠ control dictionaries. D050-4: compiler-not-executor; routing is a topic/intent table, not unconstrained LLM. D050-5: ontology/schema workbench is a pack; SchemaModify/KGCL on the Kutha log. D050-6: validate(+security) is required; object capabilities, WASM, path-ABAC are ADR-051/080/081.
- scope: MetaPrompt, Dictionary, DictionaryEntry, AgentInstance, D3 agent cycle, L_AGENT, intern map fence, topic routing

## ADR-051: Capability Security
- source: docs/ADR/ADR-051-capability-security.md
- status: proposed
- decision: D051-1: authority is an unforgeable reference (capabilities as Ports), not a role string alone and not an LLM-edited ACL table. D051-2: tool allow-list is necessary and insufficient; PACT-grain: untrusted content must not bind authority-bearing arguments; fail closed. D051-3: P0 = operator allow-list; honeycomb = grant/revoke events on the log (VT×TT); recording a grant does not make an arbitrary event reference an unforgeable bearer capability. Capability ≠ path-ABAC (080) ≠ WASM (081).
- scope: capability security, agent validate kernel, ADR-050, PACT argument provenance, operator allow-list, grant/revoke events, confused deputy

## ADR-052: GenAI Enrichment as Optional Pack
- source: docs/ADR/ADR-052-genai-enrichment-pack.md
- status: proposed
- decision: D052-1: enrichment is a reversible ADR-021 pack; outputs are proposed events requiring explicit admission; CA cache of model calls is mandatory; unload drops derived leases, not the log. D052-2: dual-process ingest — hot write of admitted evidence does not require LLM. D052-3: derived ≠ kernel fact; enrichment cannot override L_KB / statutory facts. D052-4: temporal coherence gating is a retrieve/ranking lease (ruvector-temporal-coherence), not fact validity. Observed source response, extracted proposition, and admitted claim are distinct records.
- scope: GenAI enrichment, optional pack, derived events, LLM extract/summarize/embed, content-addressed cache, dual-process ingest, temporal coherence gating, ruvector-temporal-coherence

## ADR-060: Strict Replay
- source: docs/ADR/ADR-060-strict-replay.md
- status: proposed
- decision: D060-1: semantic replay is fold(log, CA-cache); LLM and external effect endpoints are not called during strict replay; divergence is a test failure. D060-2: WAL recovery is a cousin, not the harness; both required, not interchangeable. D060-3: retroaction is optional on top of replay; cut grain is quantum / event offset. Three independent verification obligations: state replay, provenance/integrity verification, execution replay. Runtime::replay_check currently checks fold fingerprint and Behavior.caused_by (M011 S02); it does not rerun behaviors or compare receipts.
- scope: strict replay, semantic replay, D9, ReplayDivergenceError, fold-from-log, WAL recovery, execution replay, provenance verification, content-addressed cache, Runtime::replay_check

## ADR-061: Fork-and-Diff
- source: docs/ADR/ADR-061-fork-and-diff.md
- status: proposed
- decision: D061-1: a branch is a named overlay on the log (ref → event-id plus optional overlay events); merge/publish is a logged commit/compensating event, not CRDT. D061-2: fork-at-offset remains; named branches compose on top. D061-3: diff is GED-class (or cheaper restriction), not cosine. Clarification: portable references must bind stable event/claim identity and branch ancestry, not a bare local fact_seq; P0 fork_at is a prefix experiment, not a merge or effect-isolation implementation.
- scope: fork-and-diff, named branches, fork-at-offset, graph edit distance, event log, Verify

## ADR-062: Regimes Gated Loop (Optional)
- source: docs/ADR/ADR-062-regimes-gated-loop.md
- status: proposed
- decision: D062-1: default off; P0–P2 do not require a regimes loop; if enabled later it is a pack using dictionaries (050) + receipts (014), not a second runtime. D062-2: regimes derived from dictionaries, not hard-coded; a log commit alone does not atomically commit or undo an external effect; compensation is a new authorized recorded action. D062-3: not an orchestrator (no Airflow/Dify/oxify DAG). Effect boundaries apply independently of whether the optional pack is enabled.
- scope: regimes gated loop, Verify, Agent, Composition, ADR-050 dictionaries, semantic transaction effects, external adapter idempotency, P0 scope

## ADR-070: Cypher + Temporal AS OF Surface
- source: docs/ADR/ADR-070-cypher-temporal-as-of.md
- status: proposed
- decision: D070-1: one GPML IR; Cypher and GQL as skins; SQL/PGQ optional view pack, not a second SoT. D070-2: temporal AS OF is native (VT and TT cuts); compiled plans must not silently use “now”; native fold cut GraphFold::as_of(vt) and Runtime::csr_lease_at(tt, vt) exist in P0; Cypher/GPML grammar remains Proposed and frozen under current .kutha/STATE.md. D070-3: writes are typed operators, not free MERGE-as-truth; LLM-produced Cypher is a proposal. D070-4: named-graph / USE scope is explicit. Clarification: snapshot path vs co-temporal interval path vs journey are distinct; P0 implements sorted-row leapfrog_intersect, not full variable-ordered MATCH.
- scope: GPML IR, Cypher, GQL, Temporal AS OF, LFTJ, leapfrog_intersect, GraphFold, CSR lease, named graphs, query compiler

## ADR-071: Hybrid Retrieval Composition
- source: docs/ADR/ADR-071-hybrid-retrieval-composition.md
- status: proposed
- decision: D071-1: approximate operators are explicit named families (keyword tree, BM25+dense fusion, ANN kNN), never a silent substitute for exact MATCH; fail closed if the caller asked for exact. D071-2: compose through the same planner (043); 042 remains the HNSW fence. D071-3: keyword ≠ Cypher ≠ BM25-on-chunks; BANKS-class connecting subtree is not MATCH and not chunk RAG.
- scope: hybrid retrieval, exact MATCH, BM25+dense fusion, keyword connecting trees, HNSW, planner composition, community detection lease, ANN kNN

## ADR-080: ABAC & Multi-Tenant Slices
- source: docs/ADR/ADR-080-abac-multi-tenant-slices.md
- status: proposed
- decision: D080-1: policy enforcement is query rewrite on the GPML IR; app-layer post-filters are insufficient; LLM is not the PEP. D080-2: grants are log facts with VT×TT; historical permission audit evaluates grants at the named cut, not today’s role table; current authorization to execute a query is checked independently of the historical data/grant cut (AS OF must not restore present access). D080-3: tenant is a slice of one fold, not a second Postgres per counsel. D080-4: vacuum/hold authority is policy-as-graph. D080-5: fail-open-on-absent-policy is forbidden for enterprise; P0 spike may stay single-tenant.
- scope: ABAC, multi-tenant slices, GPML IR, grants, ReBAC, vacuum, legal hold, query rewrite

## ADR-081: Agent Sandbox
- source: docs/ADR/ADR-081-agent-sandbox.md
- status: proposed
- decision: D081-1: compile Cypher/GPML in-process (trusted engine); run pack UDFs behind WASM (or later CHERI) with capability-limited WASI; UDF writes the log only through TOKI/receipt paths. D081-2: shared-memory views are a cost requirement; no SGX-as-SoT. D081-3: sandbox ≠ semantic transaction ≠ ABAC.
- scope: agent sandbox, WASM, SFI, pack UDFs, WASI, capabilities, shared-memory CSR views, Security, Composition

## ADR-090: Legal Reference Pack — Temporal Normative AST & Practice Overlay
- source: docs/ADR/ADR-090-legal-reference-pack.md
- status: proposed
- decision: D090-1: ship Legal Reference as a vertical STCA pack, not a second engine and not Graphiti-class agent memory; law-nexus ADRs are compatibility references with explicit mapping tables and fail-closed tests. D090-2: four planes L_KB / L_AGENT / L_XP / L_TRACE with role-bound clocks; hard separations include Text/CTV ≠ Force, Published ≠ Observed ≠ LegallyEffective, RVF ≠ hot SoT. D090-3 TR-01–TR-10: named orthogonal anchors; no silent clock substitution; immutable events / intervals as projections; Assert/Retract/Correct; invalidate ≠ delete; typed non-success; dual-process ingest; hybrid retrieve; agent outside trust boundary; packaging ≠ storage. Vertical pack architecture; not product runtime.
- scope: Legal Reference Pack, STCA vertical pack, temporal normative AST, CTV, practice overlay, four planes, law-nexus compatibility, RVF packaging, NormativeState, EditionAst

## ADR-091: RVF Portable Capsules
- source: docs/ADR/ADR-091-rvf-portable-capsules.md
- status: proposed
- decision: D091-1: RVF is transport/freeze, not the log; hot path remains log + Rocks + leases; import is ingest-as-events or read-only lease, never “the capsule is now SoT.” D091-2: witness in RVF ≠ quantum receipt algebra (014); RVF may embed those tuples. D091-3: subset of RVF segments (WITNESS, VEC, GRAPH, META, WASM) is still open; do not vendor the whole RuVector monorepo.
- scope: RVF capsules, portable packaging, export/import, WitnessChain, RuVector COW freeze, ADR-014 constant-size evidence, graph fork export

## ADR-092: Naming & License
- source: docs/ADR/ADR-092-naming-and-license.md
- status: proposed
- decision: D092-1: working product name remains Kutha; this ADR does not execute a rename; historical “kutkha” is a dialogue leftover. D092-2: license is undecided (core vs packs vs ontologies); no SPDX choice is Accepted here. D092-3: trademark/clearance is a later human process, not an agent task. Does not rename or file trademarks.
- scope: Kutha product naming, SPDX and licensing, engine core vs agent packs, ontology packs, trademark clearance, Packaging honeycomb

## ADR-093: Scientific Archive Pack — Scholarly Revision AST & Evidence Graph
- source: docs/ADR/ADR-093-scientific-archive-pack.md
- status: proposed
- decision: D093-1: ship Scientific Archive as a vertical STCA pack, sibling to Legal Reference (ADR-090), not a Graphiti memory sidecar and not a mandatory Samyama runtime. D093-2: both verticals obey TR-01…TR-10 from ADR-090; scientific specialization maps Paper/PaperRevision/SourceManifestation/EvidenceBundle/Claim vs legal Work/CTV/Force. D093-3: four planes L_KB / L_AGENT / L_XP / L_TRACE with scientific contents. Domain semantics track daily-archive ADRs as compatibility references. Vertical pack architecture; not product runtime.
- scope: Scientific Archive pack, PaperRevision, Paper, SourceManifestation, EvidenceBundle, Claim, temporal edges, Research Process Plane, pipeline ledger, kg-* port mapping, TR-01–TR-10, daily-archive
