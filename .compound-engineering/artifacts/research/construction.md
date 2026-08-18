Deep-read from arXiv HTML/PDF (+ AlphaXiv abs pages verified). STAR-RAG is [arxiv:2510.16715](https://arxiv.org/abs/2510.16715) / [OpenReview](https://openreview.net/forum?id=kwro5432AI). Paper `2509.15464` is TREK (**EvoReasoner + EvoKG**), not a standalone “EvoKG” paper.

---

# Temporal KG Construction & Temporal GraphRAG — Kutha Research Brief

**Lens:** Kutha = self-contained Rust temporal graph core; Graphiti = semantic reference only; LLM construction = optional ingest agents.

---

## 1. ATOM — `2510.22590`

**Title:** ATOM: AdapTive and OptiMized dynamic temporal knowledge graph construction using LLMs  
**Links:** [arXiv](https://arxiv.org/abs/2510.22590) · [AlphaXiv](https://www.alphaxiv.org/abs/2510.22590)

| Dimension | Mechanism |
|-----------|-----------|
| **Build from text** | **3 modules:** (1) LLM **atomic-fact decomposition** of chunks → minimal self-contained facts; (2) parallel LLM **5-tuple extraction** `(es, rp, eo, tstart, tend)` → **atomic TKGs** `Git`; (3) **parallel pairwise merge** of atomic TKGs → snapshot `Gst`, then merge into DTKG `G^t`. End-actions rewritten at extract time so merge can match begin/end without LLM. Relative times (“a month ago”) resolved using **observation time** as context. |
| **Dual-time** | Explicit **dual-time:** `Tobs` (when fact is observed / ingested) vs **validity** `[tstart, tend]`. Snapshot at observation `t`; DTKG is parallel merge of snapshots. Critiques Graphiti for collapsing observation into `tstart`. |
| **Conflict / merge** | **LLM-free merge** (vs Graphiti): (i) entity resolve exact then cosine > `θE`; (ii) relation synonym merge > `θR` independent of endpoints/times; (iii) **temporal resolution** aligns same `(es,rp,eo)`, extends validity history, pairs end-actions with beginnings. Parallel tournament merge (Alg. A.1/A.2). |
| **Retrieval / QA** | Paper is **construction-focused** (agent memory / GraphRAG substrate). No novel QA path; designed so TKGs support temporal retrieval downstream. |
| **Metrics** | ~**18%** higher factual-temporal exhaustivity; ~**33%** better stability; **>90%** latency cut vs Graphiti/iText2KG (93.8% / 95.3% in Fig. 4). Table 2 (atomic facts): `Rf` 0.720, `Rf,t` 0.354. Table 3 DTKG: PER/RER ≈ **0.99**, relation resolution F1 **1.0**. |
| **LLM vs structure** | **LLM-bound:** decompose + 5-tuple extract. **Structure-bound:** dual-time schema, merge, validity history, parallel update — explicitly **no LLM in merge**. |

**Kutha takeaway:** Strongest construction blueprint for a non-LLM core: **accept pre-extracted 5-tuples; implement dual-time + embedding-threshold merge in Rust**.

---

## 2. TREK / EvoKG + EvoReasoner — `2509.15464`

**Title:** Temporal Reasoning with Large Language Models Augmented by Evolving Knowledge Graphs  
**Links:** [arXiv](https://arxiv.org/abs/2509.15464) · [AlphaXiv](https://www.alphaxiv.org/abs/2509.15464) · [code](https://github.com/junhongmit/TREK)

| Dimension | Mechanism |
|-----------|-----------|
| **Build from text** | Document chunks → `f_extract` (LLM) → partial KG → `f_merge` into `G_t`. Two tracks: **entity contextualization** + **relation evolution**. |
| **Dual-time / validity** | Edges store **`valid_from` / `valid_until`** (open `⊥` allowed). Not full bi-temporal obs/valid like ATOM; temporal intervals on non-exclusive edges. |
| **Conflict / merge** | Split relations: **`R_excl`** (birthdate, primary affiliation) → multi-candidate property bags with confidence `C(o)` from **frequency × temporal decay × source weight**; **no overwrite**. **`R_non-excl`** → insert parallel interval edges (Obama Senator then President). Entity align: Enc(type,name,desc) + cosine + score threshold `θ`. Relation synonym via schema embedding. Edge ops: Insert / Skip / Merge / Map-synonym (App. B). |
| **Retrieval / QA** | **EvoReasoner:** multi-route LLM plan → cost `(b·n)^h` → temporal-context global grounding Enc(mention,τ) → beam local expand with verbalized triplets including validity → path Conf product → weighted majority vote. |
| **Metrics** | TimeQuestions / MultiTQ: up to **+23.3% / +18.1%** abs vs best baselines. CRAG Movie/Sports end-to-end: **+2–8%** after EvoKG; LLaMA-8B Sports **18.6→37.0%**, ≈ DeepSeek-V3 671B IO (~38%). |
| **LLM vs structure** | **LLM-bound:** extract, route plan, init/rel scoring prompts. **Structure-bound:** exclusive vs non-exclusive schema, confidence multi-value store, interval edges, merge rules — portable to Rust if extractors are agents. |

**Kutha takeaway:** Adopt **exclusive/non-exclusive + confidence bags + interval edges** as first-class core types; keep multi-route reasoning in the agent layer.

---

## 3. TG-RAG — `2510.13590`

**Title:** RAG Meets Temporal Graphs: Time-Sensitive Modeling and Retrieval for Evolving Knowledge  
**Links:** [arXiv](https://arxiv.org/abs/2510.13590) · [AlphaXiv](https://www.alphaxiv.org/abs/2510.13590) · [code](https://github.com/hanjiale/Temporal-GraphRAG)

| Dimension | Mechanism |
|-----------|-----------|
| **Build from text** | Chunk corpus → LLM **temporal quadruples** `(v1,v2,e,τ)` → **bi-level graph:** lower **timestamped TKG** (same pair @ different τ = **distinct edges**); upper **hierarchical time graph** (year→quarter→month→day); cross-layer links time nodes ↔ active edges; bottom-up **multi-granularity time reports** (LLM summaries). |
| **Dual-time** | Single **event timestamp τ** on edges + time hierarchy. Not obs/valid dual-time; “time” is **fact time**, not transaction time. |
| **Conflict / merge** | Ambiguity avoided by **not collapsing** same-relation different-τ edges. Incremental: extract new quadruples, merge; **regenerate reports only for new leaf time nodes + ancestors** (vs full GraphRAG rebuild). No explicit invalidation logic. |
| **Retrieval / QA** | LLM extracts query time set `T^q` → semantic top-K edges → filter by `T^q` → **local:** PPR on temporally filtered seeds, score chunks; **global:** retrieve time-node summaries. Then LLM generate. |
| **Metrics** | ECT-QA (earnings calls). Base QA Correct **0.599** vs GraphRAG ~0.405 / HippoRAG2 ~0.410; ROUGE-L **0.493**. Index cost **6.3M/7.1M** tokens vs GraphRAG **37.1M/17.7M**. Incremental update protocol: base/new corpus splits; wins on Temporal Coverage / Overall LLM-judge. |
| **LLM vs structure** | **LLM-bound:** quadruple extract, time reports, query-time ID, generation. **Structure-bound:** bi-level topology, parallel temporal edges, ancestor-only summary invalidation, PPR local retrieval. |

**Kutha takeaway:** Implement **time hierarchy + cross-links + parallel temporal edges** in core; treat reports as optional materialized views from agents.

---

## 4. LLM-empowered KG construction survey — `2510.20345` (skim)

**Title:** LLM-empowered knowledge graph construction: A survey  
**Links:** [arXiv](https://arxiv.org/abs/2510.20345) · [AlphaXiv](https://www.alphaxiv.org/abs/2510.20345)

| Dimension | Temporal-relevant patterns |
|-----------|----------------------------|
| **Build** | Classical pipeline: **ontology → extraction → fusion**. LLM paradigms: **schema-based** (static CQ/ontology populate; dynamic AdaKGC/AutoSchemaKG) vs **schema-free** (structured generative / OpenIE). Bottom-up: GraphRAG-style instance graph → cluster → schema; **EDC** Extract–Define–Canonicalize. |
| **Dual-time** | Survey does **not** center bi-temporal modeling; flags **dynamic knowledge memory** and continual update as future work (§6). |
| **Conflict / fusion** | Entity alignment + conflict as fusion stage; LLM fusion frameworks for heterogeneity. Temporal invalidation under-specified vs construction papers above. |
| **Retrieval** | Positions KGs as **external memory for LLMs / RAG**, not a retrieval algorithm. |
| **Metrics** | Survey paper — no single benchmark. |
| **LLM vs structure** | Frames LLMs as **cognitive engines** for the whole pipeline — useful map of what to **push out of Kutha core** (ontology assist, OpenIE, canonicalize) vs keep (fusion operators, schema constraints). |

**Kutha takeaway:** Align optional agents with **EDC / schema-co-evolve**; keep fusion operators structure-bound.

---

## 5. RoMem — `2604.11544`

**Title:** Time is Not a Label: Continuous Phase Rotation for Temporal Knowledge Graphs and Agentic Memory  
**Links:** [arXiv](https://arxiv.org/abs/2604.11544) · [AlphaXiv](https://www.alphaxiv.org/abs/2604.11544)

| Dimension | Mechanism |
|-----------|-----------|
| **Build from text** | Stream of episodes → **temporal OpenIE** → facts `(h,r,t)` with **`thappen`** + **`tobs`**; append-only memory `m = (f, thappen, tobs, src)`; heterogeneous graph from facts. |
| **Dual-time** | Explicit **`thappen` / `tobs`**; if no valid time, `thappen` unknown. Time enters scoring as continuous `τ`, not discrete metadata. |
| **Conflict / merge** | **No DELETE/UPDATE at ingest.** Append-only; conflicts resolved at query via **geometric shadowing**: `θ_r(τ) = s · α_r · τ · ω`, Rot in `U(1)^d`. **Semantic Speed Gate** `α_r = σ(MLP(ϕ(r)))` — static relations `α≈0` (born_in), dynamic `α≈1` (president_of). Critiques Graphiti/Mem0/Zep LLM UPDATE/DELETE and recency sort. |
| **Retrieval / QA** | Score `skge(h,r,t \| τ)` by phase alignment to `τ_q` (default `τ_now`); obsolete facts rotate out of phase. Drop-in over HippoRAG-style graphs. |
| **Metrics** | ICEWS05-15 **72.6 MRR** (TKGE SOTA claim). MultiTQ (GPT-5-mini): MRR **0.337** vs HippoRAG **0.203**, Acc@5 **0.366** vs **0.112** (~2–3×). LoCoMo avg **0.857**; DMR-MSC static preserved (**0.856** MRR); FinTMMBench zero-shot **0.728** MRR. |
| **LLM vs structure** | **LLM-bound:** OpenIE / NER for graph build. **ML-bound (not pure structure):** trained gate + ChronoR-style embeddings. **Structure-ish:** append-only store of dual timestamps — fits core; rotation is an **optional ranking plugin**. |

**Kutha takeaway:** Store **happen/obs** always; do **not** put phase rotation in the mandatory core — optional vector scorer beside deterministic validity filters.

---

## 6. STAR-RAG — `2510.16715` / OpenReview `kwro5432AI`

**Title:** Right Answer at the Right Time — Temporal Retrieval-Augmented Generation via Graph Summarization  
**Links:** [arXiv](https://arxiv.org/abs/2510.16715) · [OpenReview](https://openreview.net/forum?id=kwro5432AI) · [AlphaXiv](https://www.alphaxiv.org/abs/2510.16715)

| Dimension | Mechanism |
|-----------|-----------|
| **Build from text** | **Assumes existing TKG events** `(s,r,o,t)`. Does **not** construct from episodes. Builds **rule graph:** Apriori entity labels from relation sets → rule nodes `⟨cs, r, co⟩` → Hamming-neighbor candidates → **MDL** edge selection → time-aligned summary graph. |
| **Dual-time** | Event timestamp `t` only; “time-aligned” edges via temporal proximity of category co-occurrence — not bi-temporal validity. |
| **Conflict / merge** | No fact invalidation; summarization compresses search space. MDL keeps edges that best explain trends. |
| **Retrieval / QA** | Seed Top-K1 events by cos(q, F) → map to rule graph → **seeded PPR** → Top-K2 rules → support events → re-rank Top-K1 by cosine → LLM answer. Training-free. |
| **Metrics** | Hit@1: CronQuestions **76.9**, Forecast **39.8**, MultiTQ **30.5** (beats T-GRAG / TS-Retriever). Multi-event MultiTQ Hit@1 **44.4** vs T-GRAG **23.8**. Token cut up to **97%** vs MedicalGraphRAG. Backbone swap ≤ **4.4%** drop. |
| **LLM vs structure** | **Structure-bound:** rule graph, MDL, PPR (ideal Rust). **LLM-bound:** final generation only. **Not** an ingest pipeline. |

**Kutha takeaway:** Excellent **retrieval index layer** over an already-built temporal store; complementary to ATOM/EvoKG construction.

---

## Synthesis: 2025–2026 construction patterns ranked for Kutha

Rank = fit for **non-LLM Rust core** + **optional LLM ingest agents**.

| Rank | Pattern | Primary papers | Core (Rust) | Agent (LLM) | Why |
|------|---------|----------------|-------------|-------------|-----|
| **1** | **Dual-time facts** (`tobs` / `thappen` or obs vs `[tstart,tend]`) | ATOM, RoMem, (partial EvoKG) | First-class edge properties; never collapse obs→valid | Relative-time resolution, missing-bound inference | Fixes Graphiti-class errors; pure data model |
| **2** | **Validity intervals + exclusive vs non-exclusive** | EvoKG | Interval edges; exclusivity flags; multi-value property bags + confidence formula | Extract + classify exclusivity | Deterministic conflict policy without LLM arbitration |
| **3** | **Atomic facts → parallel structure merge** | ATOM | Tournament merge, `θE`/`θR`, temporal begin/end pairing, LLM-free | Atomic decompose + 5-tuple extract | Directly answers “core must not require LLM”; Graphiti contrast explicit |
| **4** | **Bi-level graph (fact TKG ⊕ time hierarchy)** | TG-RAG | Time tree, cross-links, parallel τ-edges, incremental ancestor dirty-set | Quadruples + time reports | Update-friendly indexing without community rebuild |
| **5** | **Time-aligned rule / summary graph + PPR** | STAR-RAG | Apriori labels, MDL edges, seeded PPR | Answer generation | Retrieval efficiency; assumes construction elsewhere |
| **6** | **Confidence / multi-hypothesis retention** | EvoKG | Append candidates; score by freq×decay×source | Source reliability priors | Prefer over destructive overwrite |
| **7** | **Geometric / continuous phase time** | RoMem | Optional: store embeddings + `α_r` cache | OpenIE; train gate offline | Powerful conflict shadowing but ML-heavy — **plugin**, not kernel |
| **8** | **LLM schema co-evolution / EDC / CQ ontology** | Survey, AutoSchemaKG/EDC lineage | Schema registry + canonicalize hooks | Ontology assist, open extract | Agents only |
| **9** | **Per-ingest LLM UPDATE/DELETE** | Graphiti / Mem0 / Zep (critiqued by ATOM & RoMem) | Avoid as required path | Optional override tool | Latency + context blow-up; antithetical to Kutha core |

### Recommended Kutha pipeline split

```
[Optional LLM agents]          [Rust temporal core]
episodes/docs
  → atomic facts / OpenIE
  → 5-tuples or quadruples
        │
        ▼
   ingest API  ──►  dual-time store
                    exclusive/non-excl + confidence
                    interval / parallel-τ edges
                    embedding-threshold merge
                    time hierarchy index
                    (optional) rule-graph / PPR
                    (optional) RoMem scorer
        │
        ▼
   structure retrieval ──► agent QA / synthesis
```

### Confidence notes

- [HIGH] Mechanisms above from paper PDFs (arXiv abs URLs HTTP 200 verified).  
- [HIGH] STAR-RAG = `2510.16715` / OpenReview `kwro5432AI`.  
- [MED] AlphaXiv “overview” pages exist for these IDs but add little beyond arXiv for mechanism detail.  
- [HIGH] For Kutha: **steal ATOM merge + dual-time + EvoKG exclusivity; steal TG-RAG/STAR retrieval topology; keep RoMem as optional ranker; treat Graphiti as semantic UX reference, not merge algorithm.**