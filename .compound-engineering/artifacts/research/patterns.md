# Kutha Feature Research: Spatio-Temporal KG × Pattern Mining Cross-Cut

**Verdict:** The literature converges on *flat bi-temporal property graphs* (Graphiti-class) as necessary but insufficient. White space that fits Kutha’s STCA thesis is **write-time executable edge semantics**, **isolation-typed contradiction contracts**, **nested/scoped worlds**, **first-class provenance + schema roles**, and **deterministic temporal operators**—not another LongMemEval memory sidecar.

Sources skimmed: Plamper et al. STKG survey (arXiv:2512.16487); Edelweiss & Palazzo TF-ORM lineage (arXiv:2608.05342); TOKI (arXiv:2606.06240) via TGMS related work (arXiv:2607.10265); WorldDB (arXiv:2604.18478); Memanto (arXiv:2604.22085); Emergent Mind TKG topic (rep. learning focus).

---

## 1. Pattern Catalog

| Pattern | Description | Papers / systems | Maturity |
|--------|-------------|------------------|----------|
| **Node-annotated time** | Validity/lifetime on nodes; edges static connectors | Campos et al.; STKG survey §III.B | Mature (domain KGs) |
| **Edge-annotated time** | Timestamped / interval edges; nodes largely static | Rossetti et al.; Lou et al.; money-transfer / social nets | Mature |
| **Node–edge annotated (temporal property graph)** | Existence + property functions over time periods on both | Moffitt & Stoyanovich; Rost/GRADOOP; Byun; Debrouvier | Mature (DB research) |
| **Graph-annotated / snapshot sequence** | \(G = \{G_{t_i}\}\) discrete slices | Semertzidis & Pitoura; classic dynamic-network literature | Mature; weak for continuous correction |
| **Duration-labeled edges** | Traversal cost \(\lambda\) + start time (flight/call) | Wu et al. | Niche but crisp |
| **Interval + `now` semantics** | Half-open valid intervals; open-ended current | Clifford/`now`; Debrouvier; Graphiti/Zep | Production in agent memory |
| **Timestamp / link-stream** | Discrete activation instants; motif/clique mining | Latapy/Viard; Impedovo temporal pattern mining | Mature analytics |
| **Bi-temporal (valid × transaction/system)** | Orthogonal world-truth vs belief/record time; AS OF | Snodgrass; Edelweiss TF-ORM queries; Graphiti; TOKI; TGMS; WorldDB | Mature theory; uneven productization |
| **Assert / retract / correct write ops** | Carve valid intervals; close VT vs close TT for errata | TGMS write API | Emerging (agent-native DBs) |
| **Isolation-typed contradiction operators** | LWW / evidence-merge / await-confirm / policy as \(\oplus\) with isolation preconditions + audit row | **TOKI** | Emerging (strong theory; limited adoption) |
| **Edges as write-time programs** | `on_insert`/`on_delete`/`on_query_rewrite` per edge type (supersedes closes VT; contradicts keeps both) | **WorldDB** | Emerging |
| **Recursive / nested worlds** | Node = container with interior subgraph + ontology scope + composed embedding | **WorldDB** | Emerging |
| **Content-addressed immutable blobs + mutable VT columns** | Merkle/edit propagation; VT closable without hash rewrite | WorldDB | Emerging (Git-like memory) |
| **Typed episodic/semantic/procedural stores** | 13+ memory categories with distinct decay/priority | Memanto; ENGRAM | Emerging product |
| **Conflict gate before persist** | Detect contradiction → supersede / retain / annotate | Memanto D5 | Emerging |
| **Zero-index / write-available retrieval** | Instant write→search; no HNSW lag / sync LLM extract | Memanto D6 (critiques Graphiti) | Contested (engine-dependent) |
| **Role-based schema evolution** | Object keeps class identity; roles activate/suspend behaviors over time; schema temporalities | Edelweiss CLEI’97 TF-ORM | Classical DB; **underused in agent KGs** |
| **Spatial annotation (node / node–edge / graph)** | Coordinates, polyline edges, region containers | STKG survey §III.C; street/landscape graphs | Mature GIS; sparse in agent memory |
| **Distance-inferred topology** | Edges from \(D(u,v)<\epsilon\) (Tobler/Waxman) | Barthélemy; STKG survey | Mature spatial nets |
| **Allen-interval / temporal QA operators** | Fixed operator algebra over VT/TT; plan + trace gating | TGMS; TempoQR (QA) | Emerging |
| **TKG completion / forecasting embeddings** | Interpolation vs extrapolation; tensors/seq models | Emergent Mind TKG surveys | Mature ML; **orthogonal** to Kutha core SoT |
| **Provenance-as-algebra** | K-semiring / audit polynomials on facts | TOKI (Green et al.); STKG challenge “Provenance” | Theory mature; engines weak |

---

## 2. White Space / Under-Served Capabilities

Opinionated gaps that papers *complain about* or leave open—ranked by fit to Kutha:

1. **Reusable generic ST model vs app-specific graphs** — STKG survey: heterogeneity of annotation loci/semantics; few *M*-focus reusable engines; most STKGs are domain throwaways.
2. **Provenance coupled to time** — Survey §IV.3: provenance treated as auxiliary metadata; hard to separate historical vs inferred vs corrected. TOKI elevates audit rows; Graphiti-class stores rarely expose algebraic provenance.
3. **Incremental update + invalidate inferred knowledge** — Survey §IV.5: many STKGs rebuild snapshots; inferred edges lack revision when evidence arrives. Maps directly to Kutha Behaviors + invalidation.
4. **Flat bi-temporal is not enough** — WorldDB: Graphiti/Memento/Hydra keep typed edges as *labels*; supersession semantics live in app code; no recursive composition, no content-addressed invariants.
5. **Write-time contracts undeclared** — TOKI: production memories use LWW/merge/confirm/policy without isolation levels → replay inconsistency, belief-drift skew, audit erasure. TGMS cites TOKI as write-time counterpart to Graphiti’s storage model.
6. **Ingestion latency / mandatory LLM extract** — Memanto D6 vs Zep/Graphiti: sync extraction turns writes into multi-second ops; agent cannot read what it just wrote.
7. **Contradiction handling** — Memanto Table I: Zep/Mem0 marked weak on D5; MemoryAgentBench multi-hop conflict still unsolved industry-wide. Graphiti invalidates facts but does not ship isolation-typed adjudication.
8. **Schema evolution / roles** — Edelweiss lineage anticipated ontology evolution and role lifecycles; agent KGs almost ignore schema versioning as first-class temporal objects.
9. **Analyses beyond forecasting** — STKG survey §IV.8: literature skews GNN forecast; motif mining, anomaly, clustering under-served for *engine* APIs.
10. **Space × time as product surface** — STKG survey: spatial mostly node-point; higher-dim regions rare; agent memory papers ignore space almost entirely → open vertical for legal jurisdiction / asset / logistics packs.
11. **LLM–TKG fusion without making LLM the SoT** — Emergent Mind open challenges (TG-RAG etc.) vs Kutha ADR-001: LLM optional for audited truth—white space is *operator-backed* temporal claims (TGMS style), not bigger RAG.

---

## 3. NEW Feature Candidates for Kutha (12)

STCA mapping: **Space** (packs/ports/dicts) · **Time** (log/behaviors/bi-temporal) · **Composition** (budgets/packs) · **Materialization** (CSR/HNSW/views).

| # | Name | User value | Research grounding | Cx | STCA | Risk |
|---|------|------------|-------------------|----|------|------|
| 1 | **TOKI-style Contradiction Operator Pack** | Auditable belief revision: LWW / evidence / await-human / policy with explicit isolation; losers preserved | TOKI dual-row + \(\oplus_{t,p,?,c}\); TGMS related work | **L** | Time + Composition | Formal overfit before P0; LLM judge on write path fights “LLM not SoT” unless judge is logged/CA-cached |
| 2 | **Edge Behavior Ontology (write-time programs)** | `SUPERSEDES`/`CONTRADICTS`/`SAME_AS`/`DERIVED_FROM` *do* work at commit—no app reimplementation | WorldDB edge handlers; Kutha Relation Behaviors already planned | **M** | Time | Handler non-determinism; cascade storms without Cui budgets |
| 3 | **Scoped Worlds / Containment Ports** | Nested case files, jurisdictions, agent sandboxes; queries don’t leak across worlds without `REFERS_TO` | WorldDB recursive worlds; Cordis packs | **L** | Space + Time | Merkle/world hashing ≠ event-log SoT—must be a *materialization*, not second truth |
| 4 | **Assert · Retract · Correct API** | Legal/agent ops: world change vs record errata; TT-pinned belief history | TGMS three write ops; Edelweiss VT/TT queries | **M** | Time | Easy to conflate with Graphiti invalidate; need crisp semantics docs |
| 5 | **Role Lifecycles (TF-ORM-lite)** | Entities keep identity while roles (party, officer, norm subject) activate/suspend with schema temporalities | Edelweiss CLEI’97 roles + schema temporalities | **L** | Space + Time | Modeling complexity; migration tooling |
| 6 | **Provenance Quantum (K-semiring or CA lineage)** | Every fact/cascade cites write events + strategy; fork/replay explains “why this winner” | TOKI provenance; STKG provenance challenge; ADR Quantum Receipt | **M** | Time + Composition | Storage bloat; semiring UX hard for users |
| 7 | **Temporal Operator Surface (TGMS-lite)** | Agents plan over fixed ops (`as_of`, Allen, count/window) with trace-gated answers | TGMS operator algebra + claim checking | **L** | Time + Space (agent pack) | Closed operator set vs expressiveness; co-designed workloads don’t transfer |
| 8 | **Conflict Gate Behavior (Memanto D5)** | Block silent overwrite; force supersede/retain/annotate before commit | Memanto conflict resolution; BeliefShift unresolved contradiction rates | **S** | Time | UX friction; false-positive semantic conflict if LLM-scored |
| 9 | **Sync-Ingest / Deferred Extract Profile** | Write→query same turn; LLM extraction optional async enrichment | Memanto D6 critique of Graphiti sync pipelines | **M** | Materialization + Time | Raw text nodes pollute graph until enriched; dual representation |
| 10 | **Typed Memory Dictionaries (13→N categories)** | Dict-first retrieval by fact/decision/commitment/procedure with decay priors | Memanto typed schema; Kutha D3 dictionaries | **S–M** | Space | Duplicates Memanto product surface if sold as “memory app” not engine |
| 11 | **ST Annotation Pack (space × time)** | Jurisdiction, venue, asset location as first-class annotations + interval facts | STKG survey taxonomy (node/edge/graph spatial + interval time) | **M** | Space + Time | Scope creep; GIS quality bar |
| 12 | **Temporal Motif / Pattern Mining Pack** | Detect recurring legal/process motifs, sliding-window neighborhoods, anomaly edges | Impedovo temporal pattern mining; GRADOOP; survey §IV.8 | **L** | Materialization + Composition | Analytics pack may starve core P0–P1; GPU/ML dependency temptation |

**Bonus (only if legal vertical pulls it):** **Temporal ABAC on slices** — CARE/FAIR + time-dependent access from STKG ethics challenge; aligns ADR-000 security notes. Complexity **M**, STCA Space+Time, risk: policy graph becomes product before core exists.

---

## 4. Graphiti Duplicate vs Kutha Differentiator

### Would DUPLICATE Graphiti (do later / thin-compat only)

| Capability | Note |
|------------|------|
| Bi-temporal edge fields (valid + ingested) | Reference semantics already in ADR-000 D4 |
| Fact invalidation / supersession *as label behavior* | Needed for parity; alone does not differentiate |
| Episodic node + entity/edge extraction pipeline | Memory-framework shape; ADR-001 non-goal if mandatory |
| Hybrid vector + graph recall for chat agents | Crowded (Zep, Mem0, Memanto); not Kutha’s wedge |
| LongMemEval chase features | WorldDB/Memanto already compete there |

### DIFFERENTIATES Kutha (prefer these)

| Capability | Why it is not Graphiti |
|------------|------------------------|
| **Event-log SoT + strict replay / fork-diff** | Graphiti is a memory layer on external graph DBs |
| **Relation Behaviors = WorldDB-style write-time edge programs** | Graphiti edges are labels; WorldDB’s main critique |
| **TOKI isolation-typed contradiction + audit rows** | Graphiti lacks declared concurrency/isolation contract |
| **Assert/Retract/Correct with TT belief history** | Stronger than “invalidate fact” alone (TGMS) |
| **Nested worlds as ports/packs (materialized)** | Flat Graphiti graph; WorldDB differentiator, STCA-native if worlds ≠ SoT |
| **Dict-first / meta-prompt agents; LLM optional for truth** | Graphiti/Zep assume LLM extract on the write path |
| **Role-based schema evolution** | Edelweiss gap in all agent memories |
| **Cui cascade budgets + quantum receipts on behaviors** | Engine physics, not memory UX |
| **Deterministic temporal operator algebra + traces** | TGMS direction; Graphiti is storage+retrieval |
| **Self-contained Rust core (no Neo4j/Falkor required)** | Explicit product non-goal vs Graphiti |

**Opinionated priority for Kutha roadmap:** ship **#2 + #4 + #8** early (behaviors + bi-temporal write API + conflict gate)—they reuse STCA Time axis and answer WorldDB/Memanto/TOKI critiques without cloning Graphiti. Defer **#3 worlds** until materialization story is clear (avoid second SoT). Treat **#1 TOKI pack** as the legal/regulated differentiator after P1 bi-temporal exists. Treat **#10 typed memory** as dictionary content, not a Memanto clone.

---

### Source map (quick)

| Source | What it contributed |
|--------|---------------------|
| STKG survey 2512.16487 | Annotation/semantics taxonomy; open challenges (modeling, provenance, incremental, analyses) |
| Edelweiss 2608.05342 | Historical VT/TT queries, roles, schema temporalities → lineage for Kutha schema packs |
| TOKI 2606.06240 (+ TGMS 2607.10265) | Write-time operator algebra; Graphiti as storage peer, TOKI as contradiction peer |
| WorldDB 2604.18478 | Flat bi-temporal critique of Graphiti/Memento/Hydra; worlds + edge programs + CA |
| Memanto 2604.22085 | Graphiti weak on conflict (D5) + ingest latency (D6); typed schema; as-of/changed-since |
| Emergent Mind TKG | Completion/forecasting maturity; scalability/continuous-time/LLM fusion open—mostly *not* Kutha core |