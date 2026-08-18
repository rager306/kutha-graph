# Architecture and GTM readout from the applicability matrix

Date: 2026-08-18.  
Sources: 163 closed cards (`matrix.md` = rollup), `axis-layer-synthesis.md`, ADR-000 D1–D10, ADR-001/002, STRATEGY.md, honeycomb ADR-010–093 (Proposed), P0 spike in `crates/`.  
Not a TAM study. Not a Consensus wave. Cards remain SoT for nouns; this note is the **product translation** of that dictionary.

## Verdict (one screen)

Kutha is a **self-contained Rust temporal graph engine**: append-only **event log = source of truth**; the working graph is a **deterministic fold**; speed comes from **droppable pictures** of that fold (CSR / later HNSW / views); agents are **dict-first** (LLM proposes, dictionaries + log own audited truth).

Literature for honeycomb **nouns is closed** (163 cards, R1–R6 done). What was empty was not arXiv — it was the **honeycomb ADRs** (now Proposed) and **kernels** (P0 spike: log, fold, quantum, receipt, snapshot, CSR lease, leapfrog intersect, WAL-cousin). Remaining research is **implementation and a legal PIT fixture**, not another paper hunt.

**Primary GTM wedge (matrix Layer 5 + STRATEGY):** legal / normative temporal agents — statute (and grants) **AS OF a date**, with replayable lineage, **without** the LLM as legal authority.

**Do not position as:** “another graph DB”, “Graphiti on Neo4j”, “Samyama with agents bolted on”, or “Harvey-class firm RAG as the engine.”

---

## 1. Architecture that the matrix actually supports

### 1.1 Three planes (ADR-000 D1)

```text
Client: Cypher/GPML + AS OF + hybrid retrieve + agent API
                    │
Control / Agent     meta-prompt@version + 6 dictionaries
                    validate fail-closed → typed assert/retract/correct
                    │ emit
Time SoT            append-only event log  (UUID v7, quantum = emit→idle)
                    receipts, replay, fork-at-offset
                    │ project (leases)
Data pictures       CSR / LFTJ · interval index · HNSW fence · views
                    drop / rebuild; never a second SoT
                    │
Space               vertical packs behind Ports (legal, science, later finance)
```

One sentence the cards repeat (Layer 2): **log = SoT; everything else is a droppable lease; LLM compiles or proposes, never writes truth.**

### 1.2 What is load-bearing physics vs pack vs trap

| Kind | What | Matrix role |
|------|------|-------------|
| **P0 physics (H/H/H, 42 cards)** | Log as SoT, VT×TT + TOKI invalidation, interned IDs, CSR/WCOJ, IVM/views, interval AS OF, receipts, isolation | Ship in core |
| **Honeycomb packs (H/M/H, 70 cards)** | HNSW+repair, hybrid planner, RPQ, vacuum+hold, path-ABAC, capabilities, WASM, named graphs, OBDA compile | Ports + packs; not a second engine |
| **Trap cluster (D=high ∧ O=low, 10)** | Graphiti, Dify, Hindsight, Harvey-as-SoT, CRDT-as-SoT, Geo-Raft-as-SoT, YOTG `claim`, empty Cypher, LegalSearch-R1-as-engine, oxify DAG | Contrast only — never core SoT |

### 1.3 Competitive architecture (why this shape)

Incumbents each own **one** corner; Kutha’s bet is the **intersection**:

| Quadrant | Typical product | What they lack for Kutha’s job |
|----------|-----------------|--------------------------------|
| Fast state-first graph | Samyama / Falkor-class | History, fork, agents as first-class |
| Event-sourced graph | ActiveGraph-class | Multi-hop latency (no hot CSR picture) |
| Agent memory | Graphiti / Zep / Hindsight | Not a DB; BYO graph; LLM on the write path |
| In-DB GenAI | Raven-class | Weak native graph |
| Legal RAG / firm OS | Harvey, LegalSearch-R1 | Retrieval product, not temporal SoT |

**Fit hypotheses (ADR-000, not marketing KPIs):** regulated/temporal ~9/10; hybrid engine ~8–9/10; raw Samyama-replacement throughput ~5–6/10. Re-score only against probes below.

### 1.4 What the P0 spike already instantiates

In-process, not product runtime: emit→cascade→idle, intern map, retract keeps losers, quantum receipt, strict replay, snapshot+tail, file semantic log + framed WAL-cousin (truncated tail dropped), CSR lease + leapfrog intersection, `Materializer` build/unload, `fork_at`. **Not yet:** RocksDB crate, full variable-ordered MATCH, HNSW, Cypher surface, ABAC rewrite, legal corpus.

---

## 2. Domains we can lay down (Layer 5)

37 of 163 cards name a niche; 126 are `no niche` (engine physics). Layer 5 is **GTM color**, not an intake filter and not 37 products.

### 2.1 Primary wedge — Legal / normative (over-mapped on purpose)

Job: **assert → invalidate → AS OF date** on norms and grants; replay why a hop existed; LLM outside the trust boundary.

Matrix-backed effects (not a feature dump):

- Statutory QA: temporal validity as a **hard retrieve constraint** (not cosine; MemStrata).
- Edition / force / applicability must not collapse clocks (law-nexus overlay; ADR-090).
- Path-shaped authorization (patent/legal KG): rewrite so unauthorized subgraphs never return.
- Grants as bi-temporal facts (HIPAA-shaped: permission @T, not today’s role table).
- Counsel–client **tenant slices** ≠ path-ABAC ≠ noisy-neighbor quotas.
- Named graphs: statute vs commentary vs docket — one log, not three SoTs.
- Vacuum + **litigation hold** on the log (not LSM compaction).
- Why-not on missing statute hops (not an LLM apology).
- XML/Akoma Ntoso / RDF as **document packs** (oxixml) — not the event log.
- Practice overlay (Harvey) is **firm DMS retrieval**, not Kutha SoT and not statutory RAG.

**Pack:** ADR-090 Legal Reference. **Must not ship:** Harvey-as-engine, LegalSearch-R1 RL as SoT.

### 2.2 Second wedge — Scientific / scholarly archive

Job: revision-scoped evidence; claims and figures only from **executed** views (LLM-compiler-not-executor).

- daily-archive schema lifecycle; PaperRevision expressions.
- Biomedical NL→Cypher over a projected schema (AZ); citations as Observations.
- oxixml / ontology MCP: generate→validate→log; Oxigraph is working memory.
- RVF sealed export of a revision (packaging, med/med/med — optional).

**Pack:** ADR-093. Same engine physics as legal; dictionaries change.

### 2.3 Later / riders — not primary wedges yet

| Domain | What the matrix actually tagged | Do not invent |
|--------|----------------------------------|---------------|
| **Finance / compliance** | Receipts, vacuum/hold, envelope verify | A finance *pack* (no overlay ADR) |
| **Clinical / HIPAA** | Grants @T, receipts, Allen intervals, AZ patient-safety consume | A clinical product |
| **Enterprise / patent** | Path-ABAC, ReBAC overlay, tenants, STE outsource of a *lease*, k-anonymity publish, watermark | CMK/air-gap as a graph feature |
| **Journalism** | BANKS keyword connecting trees | A news pack |

Federation (`SERVICE` PubMed / opposing docket) is a **port**, not a second SoT.

---

## 3. Marketing metrics we can lay down now

STRATEGY already labels these **engineering falsifiers**, not PRD-locked GTM KPIs. The matrix adds **which claims are honest**.

### 3.1 Claims we may make (once fixtures exist)

| Metric | Honest claim | Probe |
|--------|----------------|-------|
| **Replay integrity** | Golden log → same fold; tamper → `ReplayDivergenceError` | ADR-060; P0 test exists |
| **Quantum bound** | emit→idle finishes under cascade budget or **aborts with a receipt**, not a silent truncate of truth | ADR-014; P0 `max_cascade` |
| **Legal PIT correctness** | Norm/grant **as of T** matches fixture edition; cosine/RAG must *fail* the same fixture | ADR-090 + MemStrata/statutory-QA cards |
| **Hot-path proximity** | Multi-hop on CSR/LFTJ **close to** Samyama/Falkor class on *projections* — not “we beat Samyama” | ADR-041; AGM/WCOJ bound |
| **Fork cost** | Counterfactual branch at offset; losers preserved | ADR-061; P0 `fork_at` |
| **Fail-closed agents** | No number / legal conclusion without an executed fold + dict validate | ADR-050; TGMS / LLM-compiler cards |
| **Lease reversibility** | Unload CSR/HNSW; SoT unchanged | ADR-040; P0 materializer unload |

### 3.2 Claims we must not make from this matrix

- **TAM / “high demand”** — Layer 4 is “engineers will ask,” 128 high-demand cards from biased intake, not a market size.
- **Fit scores 9/10** — research hypotheses, not conversion metrics.
- **Empty Cypher success**, GraphRAG facades, “self-reconstructing agent memory.”
- **Blockchain / CRDT / Geo-Raft as the database.**
- **Always-faster-than-Samyama** — contradicted by design (5–6/10 replacement fit).

### 3.3 Buyer table (enterprise later)

Required before “enterprise ship,” **sketched** in 080/081, not P0: ABAC + temporal grants, agent sandbox, multi-tenant slices, immutable audit, CMK/air-gap (GTM checkbox, **not** a matrix noun).

---

## 4. Competitive advantages (earned vs hypothesized)

### Earned from the matrix (can say in a brief)

1. **Temporal truth is owned** — log + VT×TT + typed retract; not “memory edges in Neo4j.”
2. **Speed is not a second database** — CSR/HNSW are leases; fail the product if they become SoT.
3. **Agents without LLM-as-judge** — compiler/proposer + dictionary validate; Graphiti/Hindsight/Dify researched and **rejected as core**.
4. **Same physics, swap dictionaries** — legal then science; Harvey practice is an overlay, not a fork of the engine.
5. **Audit that is algebraic** — receipts (integrity) ≠ how-polynomials (derivation) ≠ why-not (absence).
6. **Security as rewrite, not a filter in the chatbot** — path-ABAC + grants @T (paper-shaped; kernel still a spike).

### Hypothesized (need probes before slogans)

- Legal PIT latency competitive with RAG while being **correct**.
- In-DB agents cheaper to replay than external orchestrators.
- Pack marketplace (021) without becoming Dify.

### Anti-advantages (honest)

- Will **lose** a bake-off that is only hops/s against Samyama.
- Will **lose** a bake-off that is only “chat over a DMS” against Harvey.
- Security/Packaging are literature-thin as *kernels* (16 and 7 cards); do not advertise CHERI/CMK as shipped.

---

## 5. What is *not* unfinished research

| Kind | Status |
|------|--------|
| A. More aggregator papers | **Closed.** Bound R1–R6; no Query 103+. |
| B. Honeycomb design cells | **Opened and Proposed** (010–093). |
| C. Trap cluster | **Researched; do not build as core.** |
| D. Out of scope | GPU, GNN-as-MATCH, third clock, another WCOJ. |
| Kernels | **Started** (P0 spike). Debt = Rocks adapter, variable-ordered MATCH, HNSW fence, Cypher AS OF, ABAC rewrite, legal fixture. |

Unfinished *product* work is **P1–P3 implementation + one legal PIT golden set**, not a fourth literature program.

How to advance without sprawling the honeycomb: `.compound-engineering/artifacts/plans/2026-08-18-1441-spine-without-sprawl-plan.md`.

---

## 6. Suggested one-liners (aligned with STRATEGY)

- **Product:** Event-sourced bi-temporal graph engine on Rust; agents first-class; meta-prompts and dictionaries govern; performance from pluggable pictures of the log.
- **Message:** LLM proposes; the log and dictionaries own audited truth.
- **Wedge:** Legal temporal agents first; science on the same physics; finance/clinical as receipt and hold riders until a pack exists.
