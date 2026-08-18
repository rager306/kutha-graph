# Temporal Graph Systems — Deep Read for Kutha

---

## 1. TGMS — Agent-Native Bi-Temporal Graph Management System  
**ID:** [arXiv:2607.10265](https://arxiv.org/abs/2607.10265) (Zhang, Memphis; Apache-2.0, `github.com/zxf-work/tgms`)

### Core thesis
LLM agents fail at temporal-graph QA because they invent IDs, botch arithmetic, and cannot reconstruct *belief states* after corrections. TGMS puts the LLM **outside** the trust boundary: it only plans over a fixed operator algebra and verbalizes results; the store owns bi-temporal semantics, computation, bounds, provenance, and claim checking. Quote: *“We call a database interface agent-native when it is designed for a fallible machine planner: operations have machine-readable input and output contracts, explicit bounds and costs, deterministic result identities, and evidence metadata that supports automated checking.”*

### Temporal model
- **Valid time** `[vt_s, vt_e)` + **transaction time** `[tt_s, tt_e)` (half-open; int64 µs; open end = \(2^{62}\)).
- Snapshot \(G(t,tt)\); every operator takes **`as_of_tt`**.
- Writes: **`assert`** (carve overlapping VT of same identity), **`retract`** (close VT — world changed), **`correct`** (close TT — we were wrong; preserve error).
- **No physical delete**; failed batches stay in WAL; **bi-temporal immutability**: past `as_of_tt` results stay byte-identical after later corrections.
- Distinct from “event time only”: correction ≠ evolution.

### Storage & query architecture
- Append-only **write-ahead event log** → replay to byte-identical digests across backends (Kùzu / DuckDB adapters; README also: native Rust columnar + temporal-CSR via PyO3).
- Hybrid logical clock for strictly increasing TT.
- Query surface = **13 fixed operators** (O1–O13): history, snapshot, diff, temporal reachability/paths, δ-motifs, metrics/bursts, neighborhood evolution, Allen co-active join, `resolve_entities`, `compute`.
- Plans = JSON DAG + limited `$ref`; MCP or in-process tools.

### Agent interface
- Planner → **static verifier** (schema, acyclicity, grounding, output fields, cost) → executor (content-addressed digests, **truncation taint**) → reporter → **claim verifier** (counts/values/entities/orderings gated; patterns checked but not yet gated).
- Output contracts + completeness metadata called load-bearing (Lesson 1–2).
- Evolution memory: LLM summaries quarantined if correction overlaps their VT; summaries are hints, never evidence.

### Strengths / limitations / failure modes
- **Strengths:** belief-state probes (CollegeMsg probes **0.897** vs latest-state baselines **0**); verifier catches 500/500 injected count/entity errors; contracts move silent runtime deaths to repairable rejections.
- **Limitations:** workload co-designed with operators; 14B planner **coverage 0** on planted multi-operator motif tasks; pattern claims not gated; write-back disabled; concurrency/isolation open.
- **Failure mode:** abstention as designed failure; “correct count over truncated page” without taint.

### Kutha-adoptable mechanisms
1. `assert` / `retract` / `correct` triad + pairwise-disjoint VT per identity.  
2. WAL as SoT + cross-backend digest replay.  
3. Operator registry with **input+output JSON Schema**, cost guards, pagination vs work bounds.  
4. Static plan IR + identifier grounding rule.  
5. Trace digests + truncation taint + claim gating.  
6. Quarantine of derived summaries under overlapping correction.  
7. Exact multi-label reachability `(node, arrival)` (not greedy under max-wait).

### Diff vs Graphiti-style bi-temporal memory
| | TGMS | Graphiti/Zep |
|--|------|----------------|
| Purpose | Temporal **analytics** operators + verified answers | Agent **memory** extraction → KG → retrieve into prompt |
| LLM role | Plan/report only; no graph in context | Extract, consolidate, answer from retrieved text |
| Verification | Trace-grounded claim gating | Typically none on final claims |
| Construction | Event log / bulk ingest of graph versions | LLM episode→entity/edge pipeline |
| Shared | Bi-temporal VT×TT; cite Graphiti as memory peer, not analytics peer |

---

## 2. Graph-Native Bitemporal Memory Store  
**ID:** [arXiv:2607.26520](https://arxiv.org/abs/2607.26520) — **exists** (Niksarli & Baheti, Davidson)

### Core thesis
Agent memory should be **local-first** (Neo4j + HNSW) with a full bitemporal model so updates do not erase history, avoiding both full-history context blowups and third-party memory APIs. Quote: *“Each memory is stored as an immutable identity node linked to versioned content nodes carrying two closed-open time intervals.”*

### Temporal model
- **Valid time** `[valid_from, valid_to)` + **transaction time** `[tx_from, tx_to)`; NULL upper = open.
- VT set by caller (session date); TT assigned by DB.
- **`update_memory`**: close `tx_to`, drop `:CurrentVersion`, new version.  
- **`delete_memory`**: close both `tx_to` and `valid_to` — **logical close, no physical remove**.
- Point-in-time filter: version active if VT and TT contain query instants.

### Storage & query architecture
- Neo4j property graph: `:Memory` (immutable id) —`HAS_VERSION`→ `:MemoryVersion` (+ `:CurrentVersion` label).
- **Dual HNSW**: `current_version_embedding` vs full-history `memory_version_embedding`.
- Time-travel: over-fetch \(10\times k\) from history index → post-filter VT/TT → B-tree on interval columns.
- Auto `RELATED_TO` edges at write (cosine ≥ 0.75 on top-5).
- Stack: Neo4j 5.27, Python Bolt, Titan Embed 1024-d, Claude tool loop.

### Agent interface
Nine tools: `save/update/delete_memory`, `get/search/semantic_search_memories`, **`as_of_semantic_search`**, `get_related_memories`, `get_memory_history`.  
No plan verifier / claim contracts — tool results go straight back into Claude.

### Strengths / limitations / failure modes
- **Strengths:** clean identity/version split; Cypher-expressible AS OF; local privacy story; KU 80% R@10.
- **Limitations:** N=60 LongMemEval sample; indexes **user turns only**; multi-session aggregation 30% R@10; no structured SPO KG.
- **Failure mode — “post-filter dilution”:** time-travel R@10 on temporal-reasoning **50% → 37.5%** because over-fetch fills ANN candidates with temporally invalid versions; survivors re-ranked without recency.

### Kutha-adoptable mechanisms
1. Identity node vs content version chain (stable edges on identity).  
2. Dual current vs history indexes (label-driven current view).  
3. Over-fetch + temporal post-filter (and the lesson to **re-rank after filter** with |valid_at − query|).  
4. Write-time similarity edges as cheap “related” navigation.  
5. Soft delete = close intervals, keep embeddings for travel.

### Diff vs Graphiti
Same niche (conversational memory + bi-temporal), but Graphiti builds an **extracted entity/edge KG** with LLM consolidation; this paper is **document-version + HNSW** on Neo4j with thinner graph (RELATED_TO only). Weaker construction/conflict semantics; stronger local Neo4j embedding story. No Graphiti-like contradiction invalidation algebra.

---

## 3. Engram — Less Context, More Accuracy  
**ID:** [arXiv:2606.09900](https://arxiv.org/abs/2606.09900) (Wang; dual AGPL-3.0 + commercial; `github.com/ly-wang19/engram`)

### Core thesis
A **lean hybrid retrieved slice** can beat stuffing full history on accuracy, not only cost. Engram is dual-process (hot write / async consolidate) on a bi-temporal model with invalidate-never-delete. Quote: *“answers from a ∼9.6k-token retrieved slice… 83.6% vs. 73.2% for full-context (+10.4 points…) at ∼8× fewer tokens.”*

### Temporal model
- **Episode:** event time + ingested-at (TT).  
- **Fact (SPO):** `valid_at`/`invalid_at` + `created_at`/`expired_at` + **`supersedes`**.  
- **Invariant:** never hard-delete — invalidate + supersession chain.  
- Read: bi-temporal **as-of** filter (“what we believed true at T”).

### Storage & query architecture
- System-1: append episode, light embed, enqueue (**no LLM**, &lt;50 ms).  
- System-2: extract SPO → bi-temporal KG → conflict resolve → salience/decay → summaries/profile.  
- **Cheap-then-escalate** conflicts: slot match / embedding / subsumption → rule invalidate; LLM only if ambiguous.  
- Hybrid read: dense + BM25 + graph n-hop + recency/salience → **RRF** → as-of → abstention gate → facts **+ raw chunks** + summaries.  
- Pluggable stores (Kuzu/Neo4j, LanceDB/Qdrant, LiteLLM) + offline fallbacks.

### Agent interface
API-shaped memory: `add(messages)`, `search(query)` → context pack; abstention gate.  
No TGMS-style operator IR or answer claim verifier — correctness via retrieval quality + judge harness.

### Strengths / limitations / failure modes
- **Strengths:** accuracy &gt; full context on full LongMemEval_S; hybrid path load-bearing; reproducibility harness + official judge; KU 87.5%, temporal 81.1%.  
- **Limitations:** one benchmark / one answerer; no full controlled facts-only ablation yet; headroom on multi-session aggregation & preference.  
- **Failure mode:** facts-only loses recall (extraction sheds detail); measurement pitfalls (full-history “lean”, truncated baselines, home-grown judges).

### Kutha-adoptable mechanisms
1. Event log episodes → async fold → SPO graph (maps to Kutha event→fold→graph).  
2. Invalidate + `supersedes` chain as default conflict policy.  
3. Cheap-then-escalate resolution (rules first).  
4. Hybrid context assembly (structured facts + raw chunks).  
5. RRF multi-signal fusion + as-of filter + abstention.  
6. Salience decay / reinforcement for store hygiene.  
7. Dual-process: LLM off critical write path.

### Diff vs Graphiti
Closest peer (paper: *“Zep/Graphiti is the closest in spirit”*). Shared: bi-temporal KG, invalidate, agent memory. Engram emphasizes **hybrid facts+chunks**, **rule-first conflict cost**, and **beating full-context accuracy** with a published harness; Graphiti/Zep emphasizes productized temporal KG construction and retrieval for agent sessions. Neither offers TGMS-grade answer verification.

---

## 4. AeonG / Bitemporal Property Graphs (Anselma et al., ADBIS 2025)

**Cite:** Anselma, Ballerini, Giordano, Raina, Terenziani — *Bitemporal Property Graphs: Dealing with Both Valid and Transaction Time*, ADBIS 2025 / LNCS 16043, pp. 234–249. DOI [10.1007/978-3-032-05281-0_15](https://doi.org/10.1007/978-3-032-05281-0_15). Extends **AeonG** (Hou et al., VLDB 2024) from mono-temporal (inferred TT) to full BT.

### Core thesis
Relational bitemporal theory is mature; property graphs rarely support **both** VT and TT. Extend AeonG’s model + Cypher so point/slice queries over VT, TT, or both remain expressive with **limited overhead**. Quote: *“despite the increased expressiveness, our framework adds only a limited overhead to AeonG.”*

### Temporal model
- Domains \(\Omega_{vt}, \Omega_{tt}\) (ordered, with \(\infty\)); intervals **closed-open**.  
- Existence \(\sigma(o, c_v, c_t)\); properties \(\tau\) over bitemporal periods.  
- **Logical delete** = close TT upper bound; **no physical delete**.  
- Classical Merrie-rank example: promotion vs **correction** (close TT of wrong rank, open new version).  
- Constraints: at any TT, edges require endpoints exist; properties only while object exists. Authors argue AeonG-style **VT sync of properties with vertex lifetime is too strong** for real world (great-grandfather, causation).

### Storage & query architecture
- Inherits AeonG: **current + historical** stores; **anchor+delta**; Memgraph (recent deltas) + RocksDB (anchors/past).  
- Query: temporal OpenCypher extensions — `FOR TT AS OF t`, `FOR VT AS OF t`, combined, and `FROM…TO` slices.  
- Eval: T-mgBench / Pokec-derived; TT overhead vs AeonG ~negligible; separate VT workload.

### Agent interface
None — human/DBMS Cypher surface, not agent tools.

### Strengths / limitations / failure modes
- **Strengths:** formal BCDM-aligned model; clear TT vs VT correction examples; query syntax for dual AS OF; low TT overhead claim.  
- **Limitations:** research prototype on AeonG stack; not agent-native; future work: per-element different VTs in one query, Allen qualitative constraints.  
- **Failure mode:** over-constraining VT co-lifetime of vertices/edges/properties loses real phenomena.

### Kutha-adoptable mechanisms
1. Formal \(\sigma/\tau\) existence + property periods.  
2. Dual `FOR VT AS OF` / `FOR TT AS OF` / combined slice syntax (or dict equivalent).  
3. Anchor+delta historical storage pattern (from base AeonG).  
4. Explicit modeling of correction as TT closure (Merrie C).  
5. Relax VT synchronicity constraints for edges/properties vs vertices.

### Diff vs Graphiti
Classical DBMS temporal PG vs agent memory KG. AeonG/Anselma = storage+query correctness for graphs; Graphiti = LLM-built memory with bi-temporal edges for retrieval. Complementary layers for Kutha (engine semantics vs agent construction).

---

## 5. Minigraf — Embedded Bi-Temporal Graph (industry adjacent)

**Sources:** [blog](https://adityamukho.com/minigraf-a-spiritual-successor-to-recallgraph); [DEV 1.0](https://dev.to/adityamukho/minigraf-10-an-embedded-bi-temporal-datalog-database-in-rust-4bg5); `github.com/project-minigraf/minigraf` (Rust, Apache-2.0). Spiritual successor to RecallGraph (ArangoDB Foxx, archived).

### Core thesis
Versioned graphs for agents need an **embedded, single-file** engine with native bi-temporality—not a facade over a server DB. Positioning: *“SQLite of bi-temporal graph databases”* / *“Datomic but embedded.”* Agents need “what did I believe and why,” not only vector similarity.

### Temporal model
- Every fact: **transaction time** + **valid time**.  
- Retract preserves history; **`:as-of`** (and valid-time scoping) in Datalog.  
- RecallGraph shipped TT well; VT “never shipped” — Minigraf makes both first-class (Snodgrass taxonomy “load-bearing”).

### Storage & query architecture
- EAV triples + covering indexes **EAVT / AEVT / AVET / VAET**.  
- Recursive Datalog; window fns in `:find`; prepared statements; one `.graph` file.  
- Targets: native, WASM, Android/iOS (UniFFI); **not** distributed / billion-node (→ XTDB/Neo4j).  
- Sibling: `temporal_reasoning` skill for coding agents.

### Agent interface
Library API (`Minigraf::open`, `execute` Datalog `(transact …)` / `(query … :as-of …)`). Dict-friendly for agents; no verifier algebra. “Agent memory skill” demonstrates end-to-end bi-temporal reasoning.

### Strengths / limitations / failure modes
- **Strengths:** Rust self-contained core (aligns with Kutha); embedding story; true bi-temporal EAV; Datalog reachability-as-rules.  
- **Limitations:** sub-million-node class; not a vector/hybrid memory engine; not temporal-graph analytics Motifs/CSR like TGMS.  
- **Failure mode (learned):** facade-on-Arango ops tax; retrofitting VT onto TT-only is hard → rewrite.

### Kutha-adoptable mechanisms
1. Embedded Rust single-file / library-first deployment.  
2. EAV + four covering indexes for bi-temporal facts.  
3. Datalog `:as-of` as query primitive (or compile to Kutha folds).  
4. Design lesson: **build versioning into the engine**, not as a microservice facade.  
5. Agent-process / WASM co-location pattern.

### Diff vs Graphiti
Minigraf = **storage+query substrate** (EAV/Datalog); Graphiti = **construction+retrieval pipeline** over a KG. Minigraf can underpin Graphiti-like memory; Graphiti does not replace an embedded bi-temporal engine. Closer to Kutha’s “self-contained Rust core” than Engram/Neo4j papers.

---

## COMPARISON TABLE

| System | Bi-temporal? | Agent-native? | Verification? | Construction? | Open-source? | Language |
|--------|:------------:|:-------------:|:-------------:|---------------|:------------:|----------|
| **TGMS** [2607.10265] | Yes (VT×TT; assert/retract/correct; `as_of_tt`) | **Yes** — 13 ops + MCP; LLM outside trust boundary | **Strong** — static plan + claim gating + truncation taint | Event log / bulk graph versions; summaries quarantined | **Yes** Apache-2.0 | Python + **Rust** engine (PyO3) |
| **Graph-Native BT Store** [2607.26520] | Yes (closed-open VT+TT; soft close) | Partial — 9 Claude tools; no plan IR | Weak / none on answers | Ingest user turns → MemoryVersion + HNSW; RELATED_TO | Unclear / academic (not claimed) | **Python** + Neo4j |
| **Engram** [2606.09900] | Yes (episode + SPO `invalid_at`/`supersedes`; as-of) | Yes as **memory API** (add/search), not analytics ops | Soft — abstention + harness; no trace claim gating | Dual-process: episodes → async SPO KG + hybrid retrieve | **Yes** AGPL-3.0 (+ commercial) | **Python** (pluggable backends) |
| **AeonG-BT (Anselma 2025)** | Yes (formal BT PG; TT logical delete) | No (Cypher for humans/apps) | DBMS correctness, not LLM claims | DB updates (CREATE/SET/DELETE as TT/VT periods) | Research prototype (AeonG stack) | AeonG / Memgraph+RocksDB (+ Cypher) |
| **Minigraf** | Yes (EAV TT+VT; retract; `:as-of`) | Library-in-agent; Datalog queries | Engine-level history, not answer verifier | `(transact)` facts; agent skill sibling | **Yes** Apache-2.0 | **Rust** (WASM/mobile) |
| *Graphiti/Zep (baseline)* | Yes (VT + ingestion TT; invalidate) | Memory-native retrieval into context | Generally no claim verifier | LLM extract episodes → temporal KG | Zep product / Graphiti OSS ecosystem | Typically **Python** |

---

### Kutha takeaway (one line)
Closest architectural rhymes: **TGMS** for agent-trust boundary + operators + verification; **Engram** for event→fold→invalidate→hybrid retrieve; **Minigraf** for embedded Rust bi-temporal substrate; **Anselma/AeonG** for formal AS OF / constraint semantics; **2607.26520** for identity/version + dual-index pitfalls to avoid in time-travel ANN.