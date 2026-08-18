# Spine without sprawl — architecture advancement at this stage

Created: 2026-08-18

Sources: applicability matrix (163 cards) + `architecture-gtm-readout.md`; ADR-000 D1–D10; ADR-001/002; STRATEGY.md; P0 crates; Cockburn walking skeleton; Hunt/Thomas tracer bullets; Ford/Parsons/Kua evolutionary architecture + fitness functions; Nygard ADRs; steel-thread / mission-thread practice.

This is a **governance + sequencing plan**, not a honeycomb expansion and not a Consensus wave.

## Problem frame

Honeycomb ADRs 010–093 are a **map** of the design space. Treating them as a backlog (open every cell, then “promote all”) is how a research-stage engine sprawls: many Proposed documents, little product evidence.

The spine is already locked: **STCA → Kutha deltas → honeycomb cells that deepen one facet**. Advancement now means **one end-to-end thread that can fail**, not more cells.

## Methodologies (what we keep, what we discard)

| Practice | Keep as | Discard as |
|----------|---------|------------|
| **Walking skeleton** (Cockburn) | Tiny *end-to-end* function that links the *types* of planes we already named (log, fold, lease, receipt, agent gate) | Completing every honeycomb band before any product scenario |
| **Tracer bullet** (Pragmatic Programmer) | Keep the P0 path *in the product* and thicken it; aim, fire, adjust | Throwaway spikes that do not become the runtime |
| **Steel / mission thread** | Thinnest production-grade path through the *riskiest* product claim (legal AS OF) | Horizontal layers: “finish Query, then Data, then Security” |
| **Evolutionary architecture + fitness functions** (Ford et al.) | Encode D1–D10 as tests that fail when a change violates the spine | Architecture reviews that only read ADRs |
| **ADR lifecycle** (Nygard) | `Proposed` = named decision; `Accepted` = *this cell is in the running engine* | Marking 28 cells Accepted because they are written |
| **Last responsible moment / YAGNI** | Rocks, Cypher parser, HNSW, ABAC, RVF wait until a thread *needs* them | “We already have ADR-042 so we should build HNSW” |
| **Cynefin / Wardley (complex = probe)** | Probe–sense–respond on the legal PIT fixture | Completeness of literature as progress |
| **Rule of three** | Do not generalize pack protocol until a *second* vertical actually runs | Building ADR-021 marketplace for one fixture |

**Spike vs skeleton (C2):** a spike answers “can we?” and may be thrown away. A walking skeleton *is* the architecture and grows. Kutha’s P0 crates are already a skeleton of the **engine**. They are **not** yet a skeleton of the **wedge**.

## Spine (do not reopen)

1. Event log = SoT. Fold, CSR, HNSW, views = droppable leases.
2. LLM proposes; dictionaries + log own audited truth.
3. VT × TT on facts; retract keeps losers; cosine is not stale-fact.
4. Intern map (ADR-011) ≠ agent dictionaries (ADR-050).
5. Legal pack first; science second; finance/clinical are riders.
6. Literature bound closed. No Consensus Query 103+.
7. New work cites honeycomb; does not rewrite D1–D10. No silent ADR-100 “umbrella.”

## Fitness functions (gates, not slogans)

Put these in `crates/kutha-runtime` tests (and later a tiny `kutha-verify` if the file grows). A PR that fails one does not ship.

| ID | Function | Already in P0? | Next thread |
|----|----------|----------------|-------------|
| FF1 | Strict replay: golden log → same fold fingerprint; tamper → divergence | Yes | Keep |
| FF2 | Quantum: emit→idle under budget or abort *with receipt* | Yes | Keep |
| FF3 | Lease unload: CSR drop does not change log digest | Partial (`Materializer`) | Assert explicitly |
| FF4 | Losers remain after retract | Yes | Keep |
| FF5 | **AS OF T1 ≠ AS OF T2** on the same log (MemStrata: “now” is forbidden when T was requested) | `is_live_at` exists; **no named API, no legal fixture** | **This is the gap** |
| FF6 | Fail-closed write: unknown relation / missing dict → no append | No | Stub only (one allowlist) |
| FF7 | LLM never appears as an `Op` that mutates truth | Structural (no such Op) | Keep; do not add |

Hot-path proximity vs Samyama (STRATEGY) is **not** a fitness function until FF5 is green. Speed on the wrong semantics is sprawl.

## Diagnosis of *this* stage

```text
Map (163 cards + 28 honeycomb ADRs)     ████████████  complete as a map
Engine skeleton (log→fold→quantum→CSR)  ████████░░░░  walks, in-process
Wedge skeleton (norm @ date)            ██░░░░░░░░░░  VT fields exist; product thread does not
Security / Cypher / HNSW / Rocks        ░░░░░░░░░░░░  correctly frozen
```

`Fact::is_live_at(tt, vt)` and CSR already filter by both clocks. Tests still pass `vt = 0` / `tt = MAX`. The architecture did not fail — **the walking skeleton stopped at engine physics**.

## The one next thread (prescribed)

**Name:** Legal PIT steel thread  
**Job:** three interned facts that look like “edition in force,” one retract/supersession, query **as of T**, replay, receipt.  
**Not:** Cypher, law-nexus ontology, Harvey overlay, RocksDB, HNSW, path-ABAC, six dictionary kinds, RVF.

### In scope (thin)

- Named fold API: `live_at(tt, vt)` / `as_of(vt)` returning interned triples (reuse `is_live_at`).
- Golden fixture: statute-shaped `(s,r,o)` with `valid_from`/`valid_to`; after supersession, T_old and T_new differ.
- FF5 + FF3 tests; FF6 as a **single** relation allowlist (not ADR-050’s six kinds).
- Graduate to **Accepted** only: ADR-013 (semantics already in code) and the *slice* of ADR-070 that is “AS OF is native” — **not** the Cypher grammar.
- One English fixture note under `.compound-engineering/artifacts/research/` or `crates/kutha-runtime/tests/fixtures/` (tiny).

### Out of scope (freeze until FF5 is green)

| Temptation | Why it sprawls now |
|------------|-------------------|
| RocksDB crate | Persistence already has JSONL + WAL-cousin; Rocks is a *later* adapter |
| Cypher / GPML parser | Language surface; AS OF is a fold cut, not a grammar |
| HNSW / hybrid planner | Wrong plane for the wedge; trap if it becomes retrieve-as-truth |
| ADR-050 six dictionaries | Intern + one allowlist is enough to fail-closed |
| ADR-080/081 | Enterprise buyer constraint; not the PIT probe |
| ADR-090 full pack | Overlay ontology; the thread needs 3 facts, not CTV |
| ADR-093 science | Second vertical before the first walks |
| Opening ADR-100+ | Map is complete |
| Marking honeycomb Accepted in bulk | Nygard: Accepted = implemented |
| Consensus / more cards | Bound closed |

### Definition of done (this thread only)

1. A test named for **legal PIT**: same log, two valid-times, different live triples; cosine/RAG is not in the test (it must not be the oracle).
2. Replay of that fixture is bit-stable (FF1).
3. CSR rebuilt at T_old vs T_new disagrees in the same way as the fold (lease agrees with SoT cut).
4. README/ADR-070 note: AS OF is a **cut**, Cypher remains Proposed.
5. No new crate. No new honeycomb ADR.

## How later threads attach (cable, not cathedral)

Only after FF5 is green, **one** of these, in this order:

1. **PIT + interned legal tokens** — versioned allowlist entries as facts (stub of 050), still no LLM.
2. **Rocks adapter** — same events, same replay; WAL-cousin remains the crash story until proven otherwise.
3. **Cypher skin for AS OF** — parser over the *already correct* cut (ADR-070 remainder). Empty success is a trap.
4. **Science fixture** — second vertical; *then* pack lifecycle (021) has a reason to exist.
5. **HNSW fence** — only as retrieve-not-truth (042), after exact AS OF cannot be faked.

If a later thread needs a new noun, open **one** honeycomb cell. If it is a cousin of a closed card, do not.

## Agent / process rules (anti-sprawl)

- One active steel thread. “Promote all” is forbidden.
- Honeycomb stays a **map**: read cells for constraints; do not implement a cell because it is Proposed.
- Chat in Russian; new durable docs in English; code in English.
- Fitness functions live in tests, not in STRATEGY adjectives.

## Success

Advancement is visible when **FF5 fails if someone queries “now” for a dated fact**. That is the spine becoming a product. Filling ADR-080 is not.
