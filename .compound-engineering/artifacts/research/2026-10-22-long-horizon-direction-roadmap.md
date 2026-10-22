# Long-horizon direction roadmap (proposal, 2026-10-22)

**Status:** proposal for owner decision. It is not a living roadmap, does not lease a milestone, and does not thaw any item in the `.kutha/STATE.md` Thaw map. After the owner picks a direction, the chosen arcs are copied into `.planning/ROADMAP.md` via `/gsd-new-milestone` (phases continue from 226, harness ids from M152). This file stays as the rationale.

**Evidence base:** fresh CBM index `kutha-graph` / alias `root-kutha-graph` (15710 nodes / 37791 edges, 2026-10-22, `index_mode=full`), `rg` over `crates/`, `STRATEGY.md`, ADR-012/021/043/051/052/060/061/062/070/071/090/093 headers, `.planning/REQUIREMENTS.md`. Graph is best-effort and misses Rust `Self::` callers; claims below that rest on absence were re-checked with `rg`.

## 1. Where the project is

| Fact | Value | Source |
|------|-------|--------|
| Product code | about 12k lines Rust, 573 `#[test]`, 2 crates (`kutha-common`, `kutha-runtime`) | `rg`, CBM architecture |
| Harness | 36 Python files, 104 checks, 151 harness milestones, 1112 commits | `checks.yaml`, `git log` |
| Capability proven | FF5 `as_of(2015) != as_of(2021)` on a statute fixture; replay, fork_at, typed clocks, Horn rules, sealed segments | `.planning/PROJECT.md` |
| Frozen (no code at HEAD) | Cypher/GPML, HNSW, ABAC/sandbox, legal pack, science pack, RVF, enrichment, planner | CBM + `rg -il` (only an example and a fixture mention them) |
| Missing from "in-spike" ADRs | ADR-012 vacuum/tiers, ADR-061 diff, ADR-021 pack lifecycle, ADR-051 capabilities | `rg vacuum` empty; ADR text |
| Last 4 milestones (M148-M151) | all about cargo observe output, none about product | `CHANGELOG.md` |

Diagnosis: the harness is ahead of the product by an order of magnitude, and the wedge (legal AS OF for agents) is proven only as an API-level mechanism on one fixture. A direction choice is needed because every remaining ADR depends on a missing layer: query surface, hot-path leases, pack lifecycle, or security.

## 2. Dependency spine (what blocks what)

```text
ADR-010/011/013/060 (log, fold, bitemporal, replay)       in spike
        |
        +-- ADR-012 vacuum/tiers ---------------------------- missing (needed for any long-lived store)
        +-- ADR-061 diff -------------------------------- missing (fork_at exists)
        +-- ADR-040/041 leases (CSR exists) --+
        |                                      +-- ADR-042 HNSW --+
        |                                      +-- ADR-043 planner +-- ADR-070 Cypher AS OF -- ADR-071 hybrid
        +-- ADR-021 pack lifecycle ---- ADR-050 dicts ---- ADR-051 caps ---- ADR-080/081 security
                     |
                     +-- ADR-090 legal pack ---- ADR-093 science pack ---- ADR-091 RVF capsules
```

Key observation: ADR-090 (the wedge) consumes ADR-021, 050, 070 and 071 "when opened". Today none of them exists. So "do the legal pack" really means "do a thin typed slice of each dependency", and that is the main design risk of Direction A.

## 3. Candidate directions

### A. Wedge-first: thin legal vertical on the existing core

Goal: prove legal AS OF end to end on a small real corpus (a few statutes with several editions), using a typed Rust query API. No Cypher, no HNSW.

- Needs: owner lifts `I-legal-pack-not-next` (STATE must name a legal pack). Thaw gate: "STATE names a legal pack".
- Proves: the product claim (norm AS OF a date with replayable lineage, LLM outside the trust boundary).
- Risks: pack lifecycle (ADR-021) and dictionaries (ADR-050) get invented ad hoc inside the pack; ADR-090 ontology creep ("full ontology" is frozen).
- Falsifier: PIT probe correct on N editions with zero replay divergence; if it needs more than a thin typed ingest + query facade, stop and re-plan.

### B. Physics-first: finish P0, then fast pictures

Goal: complete ADR-012 (vacuum with receipt), ADR-061 (diff), benchmark CSR hot path, then M005 HNSW, ADR-043 planner, M003 Cypher AS OF.

- Needs: nothing thawed for vacuum and diff; HNSW needs M005, Cypher needs M003.
- Proves: STRATEGY track 1 and 2 (trustworthy time, fast enough).
- Risks: no vertical pull; easy to over-engineer indexes with no workload; STRATEGY says "close on projections", measured against Samyama/FalkorDB-class baselines that are not set up yet.
- Falsifier: fork/vacuum cost and multi-hop latency on a fixed workload versus a stated baseline.

### C. Agent-plane first: dictionaries, capabilities, enrichment

Goal: ADR-050 six dictionaries, ADR-051 capabilities, ADR-052 enrichment pack. This is the "in-DB agents" differentiator.

- Needs: SIX-01 leased (frozen). Security ADR-080/081 gated behind M003.
- Risks: highest drift toward the rejected "hard FSM / Dify" shape (D3), and the least falsifiable.
- Not recommended as the first direction.

### D. Consolidate only (no new product scope)

Goal: hygiene, structure refactors, harness moratorium. Useful as a short Arc 0 under any of A/B/C, bad as a direction.

### Comparison

| | A wedge | B physics | C agent | D consolidate |
|---|---|---|---|---|
| Proves product claim | strong | weak | weak | none |
| Thaw needed | legal pack (one gate) | none first, then M003/M005 | SIX-01 | none |
| Falsifiable early | yes (PIT probe) | yes (bench) | hard | n/a |
| Risk of ad hoc dependencies | high | low | high | none |
| Reversibility | high (pack = droppable) | high | medium | high |

## 4. Owner decision (2026-10-22): order B, then C, then A; harness moratorium accepted

The owner chose **B → C → A** and accepted the moratorium (a new harness milestone only with a RED that fired in CI or review). This order follows the dependency spine in section 2: ADR-090 (A) consumes ADR-021, 050, 070 and 071, so building those first removes the main risk of Direction A (ad hoc dependencies invented inside the legal pack). The cost is that the wedge is proven last, so each arc carries a standing wedge-class probe (FF5 plus a growing PIT fixture) to catch drift early.

Thaw rule: this order records intent, not thaw. Each frozen item (M003 Cypher, M005 HNSW, SIX-01, ADR-080/081, legal pack) keeps its own gate and is thawed only when its milestone is leased, by the owner, in the Thaw map. Nothing here changes `.kutha/STATE.md`.

## 5. Arcs

One steel thread at a time; every arc ends in a gate. Phase numbers continue from 226 and harness ids from M152 when copied into `.planning/ROADMAP.md`.

### Arc 0: Hygiene (1 milestone, no product scope; no thaw)

- Commit the 13 untracked handoffs and the untracked research and review artifacts; rename branch `chore/m013-roadmap-reset` to `work/main-line` and push it as a backup (not `main`, no force). The 729-commit history exists only locally until then.
- CBM names `kutha-graph` (canonical, per `.cursor/skills/codebase-memory/SKILL.md`) and `root-kutha-graph` (historical alias) are both intended. `index_repository` without `name` writes the alias, so refresh with `name: "kutha-graph"`. This was fixed on 2026-10-22 (15710 nodes / 37791 edges). An earlier note in this file calling the canonical project a stale duplicate was wrong.
- Record the moratorium rule in `docs/process/gsd-auto-contract.md` (docs only, with CHANGELOG entry).
- Exit: clean `git status`, `ci --jobs 1` with `run_high=0`.

### Arc B: Physics (Time + Data + Query)

Order inside the arc is fixed by dependencies. Each item is its own milestone.

| Step | ADR | Slice | Thaw |
|------|-----|-------|------|
| B1 | 012 | Thin vacuum/tier with a receipt op; strict replay stays exact; sealed log never shrinks without a receipt | none |
| B2 | 061 | `diff` of two `fork_at` results as a fact-set diff (no GED) | none |
| B3 | 040/041 | Hot-path benchmark on the existing CSR lease: fixed multi-hop workload, stated baseline, regression budget | none |
| B4 | 042 | HNSW as a droppable lease behind the `Materializer` trait; recall and rebuild-from-log probe | M005 leased |
| B5 | 043 | Planner over the two real leases (CSR, HNSW), not before both exist | none beyond B4 |
| B6 | 070 | Cypher/GPML subset with native AS OF compiled to the typed IR; parser is a leaf | M003 leased |

- Falsifiers: replay divergence 0 after vacuum; fork diff exact on a counterfactual fixture; latency inside the B3 budget; HNSW rebuild from log byte-equal in membership; AS OF clause result equals `as_of()` facade on FF5.
- Kill: if B3 shows no gap against the baseline, skip B4/B5 and go to B6; do not build indexes without a workload.
- Gate GB: Cypher AS OF equals the typed facade on all fixtures; unfreeze security (ADR-080/081 depends on M003).

### Arc C: Agent and pack plane (Agent + Space + Security)

| Step | ADR | Slice | Thaw |
|------|-----|-------|------|
| C1 | 021 | Pack install/activate/retire: manifest, versioned registration of operators and leases, retire leaves the log untouched | none |
| C2 | 050 | Dictionaries, split by need: only those a pack consumes first (SIX-01 stays six, but is leased one at a time) | SIX-01 leased |
| C3 | 051 | Capability references: unforgeable authority for an operator invoke, checked at validate | none beyond C2 |
| C4 | 080 | Tenant slice policy with ABAC rewrite into the same IR from B6 | security milestone leased |
| C5 | 052 | GenAI enrichment as an optional pack with execution receipt; LLM never on the trust write path | none beyond C1 |
| C6 | 081 | Agent sandbox (code side) | security milestone leased |

- Guard against D3 drift: no hard FSM and no workflow DAG; a dictionary entry is data, validated by receipt, not a state machine.
- Falsifiers: pack retire then replay is byte-equal to a log without the pack; capability forgery attempt fails closed; enrichment off equals enrichment absent for audited truth.
- Gate GC: a toy pack (not legal) installs, queries via Cypher AS OF, and retires with zero core edits. If core edits were needed, fix ADR-021 before Arc A.

### Arc A: Legal vertical (the wedge, last)

| Step | ADR | Slice | Thaw |
|------|-----|-------|------|
| A1 | 090 | Thin legal pack: edition ops with VT, TT and wall clocks, using pack lifecycle from C1 (no full ontology) | legal pack named in STATE (lifts `I-legal-pack-not-next`) |
| A2 | 090/070 | Corpus of 3-5 real acts with at least 3 editions each; AS OF via Cypher from B6 | none beyond A1 |
| A3 | 090/061 | Counterfactual amendment: fork, diff, replay | none |
| A4 | 071 | Hybrid retrieve over norm text only if the PIT probe needs it | M005 already leased |
| A5 | 093/091 | Science pack as the second-vertical test; RVF capsules last | science pack leased (M004) |

- Falsifiers: PIT exact at every edition boundary; zero replay divergence; legal pack adds no core edits (it reuses C1, C2, B6).
- Gate GA: legal AS OF demo on a real corpus with replayable lineage. This is the product claim from `STRATEGY.md`.

## 6. Standing probes (so the wedge is not first proved at the end)

- FF5 (`as_of(2015) != as_of(2021)`) stays in every closeout.
- From B1 on, a "PIT fixture" grows by one edition per arc step; failure to answer it blocks the step.
- Each arc closes with a short PIT report in `.compound-engineering/artifacts/research/`.

## 7. Risks of this order

| Risk | Why | Mitigation |
|------|-----|------------|
| Product claim proven only in Arc A | B and C are infrastructure | Standing probes; kill rules that skip unused indexes |
| Index work without workload (B4/B5) | No real corpus yet | B3 baseline decides; skip if no gap |
| Agent plane drift to hard FSM (C) | The rejected shape D3 | Dictionary entries as data plus receipt; review against D3 each C step |
| Frozen items thawed by momentum | The order lists them | Thaw only at lease; owner names it in the Thaw map |
| Harness regrowth | 104 checks and growing | Moratorium; a check only for a product invariant introduced by the same slice |

## 8. Gates summary

| Gate | After | Question | If no |
|------|-------|----------|-------|
| G0 | now | Moratorium accepted | Accepted 2026-10-22 |
| GB | B6 | Cypher AS OF equals typed facade on all fixtures | Fix surface before security |
| GC | C6 | Toy pack installs and retires with zero core edits | Fix ADR-021 before any vertical |
| GA | A3 | Legal AS OF on a real corpus with lineage | Re-scope the pack, not the core |

## 9. Owner answers (2026-10-22, second round)

1. **Hygiene first.** Arc 0 is the first lease (M152), before B1.
2. **Baseline:** run Samyama and FalkorDB locally for B3; the environment has to be set up (section 11).
3. **Corpus:** law-nexus is usable, and an English-language dataset is to be found (section 12).
4. **M003 timing:** needs an elegant strategy, not a binary "after B5 or after C1" (section 10).

## 10. M003 strategy: split the parser from the contract

The question "M003 after B5 or after C1" mixes three different things. Separating them removes the dilemma.

| Piece | What it is | Depends on | Frozen? |
|-------|-----------|------------|---------|
| Query IR | Typed Rust structure for pattern + hops + filters + AS OF cut (the ADR-043/070 contract) | existing fold facade, leases | no, it is a typed API, not a parser |
| Vocabulary binder | Resolves labels and relations to `TermId` (today: intern + FF6 allowlist; later: pack dictionaries) | intern now, ADR-021/050 later | no |
| Surface parser | GPML/Cypher text to Query IR | IR, binder | **yes, M003** |

Findings that drive the strategy:

- B3 (benchmark) and B5 (planner) need only the IR and a handwritten plan. Samyama and FalkorDB speak Cypher, but the workload can be defined once as data (pattern, hops, filters) and rendered two ways: Cypher text for the competitors, IR for Kutha. No Kutha parser is needed to compare engines.
- C4 (ABAC rewrite) rewrites the IR, not text. The Thaw map says "security milestone after M003", so the gate stays, but what it technically needs is a stable IR plus a parser to prove text and IR agree.
- Cypher labels must bind to vocabulary. If the parser is written against pack dictionaries (after C1/C2), it waits for the agent plane and delays the wedge. If it is written against the intern map, it works now, and binding becomes a swap.

Recommended strategy:

1. **IR first, no thaw (B3-B5).** The IR is introduced as the planner input and the benchmark plan format. Falsifier: handwritten IR equals the `as_of` facade on FF5 and on the PIT fixture.
2. **M003a right after B5 (not after C1):** GPML subset parser into the IR using a `VocabBinder` trait with the intern/allowlist implementation. Subset: node and relationship patterns, fixed hops and bounded variable length, property filters, `AS OF`, `RETURN`, `LIMIT`. No writes, no aggregation, no path functions. Differential falsifier: `parse(render(workload)) == IR` for the whole benchmark workload and FF5; unknown label or relation fails closed (FF6).
3. **Binder swap after C2 (M003b, small):** second `VocabBinder` implementation over pack dictionaries. Parser and IR unchanged; the test is that the same corpus of queries gives identical IR under both binders for the shared vocabulary.
4. **Security (C4) consumes the IR**, so M003a is its only prerequisite; the C1 ordering is not needed for it.

Why this is the optimal middle: the parser is a leaf and is leased as late as the IR is stable (after B5), but before anything that rewrites queries (C4) or demonstrates the wedge (A2). It never waits for pack lifecycle, and pack dictionaries never block it.

Risks and guards:

- Parser scope creep toward the full openCypher TCK: forbidden; the subset is fixed by the workload and the legal PIT fixture. Conformance numbers are not a goal.
- IR drift while the planner exists: IR changes require the differential test to stay green.
- `AS OF` has no competitor equivalent: benchmarks compare the non-temporal subset; AS OF cost is reported separately as overhead versus the same query without it.

Thaw record: M003 is leased once, as M003a. M003b is a separate small lease after C2. Neither thaws HNSW, SIX-01 or the legal pack.

## 11. Baseline environment: Samyama and FalkorDB

Facts found on the host (2026-10-22):

- 12 cores, 47 GB RAM, but **root disk is 97% full, 9.6 GB free**. `target/` of kutha-graph is 2.9 GB. `law-nexus/consru_export` is 4.6 GB, `vendor-source` 8.7 GB, and none of them is this repo's to delete.
- Docker is available. Running containers (`fd_api`, `9router`) share the machine, so runs are noisy. Present images: `redis/redis-stack` (no FalkorDB), TEI CPU, grobid.
- Samyama source is checked out at `/root/samyama-graph` (1.2 GB, no `target/`). It carries `benches/ldbc_benchmark.rs`, `benchmarks/`, `graphalytics_benchmark.rs`, `adjacency_walk.rs`. A source build would need several GB of `target/`, so the prebuilt image `public.ecr.aws/f9f6l5u4/samyama-graph:1.1.0` is preferred (HTTP 8080, Redis protocol 6379).
- FalkorDB: `docker run -p 6379:6379 falkordb/falkordb:latest` (needs Redis 8 inside the image; the official image includes it). Use the server image without the browser for runs. Query over Redis `GRAPH.QUERY` or the Python/Rust client.
- Samyama's own benchmark guide gives the discipline to copy: calibration loop before and after each run, both engines measured back to back in one sitting, and quoting ratios rather than absolute milliseconds. A competitor number from another session or host is not a baseline.

Plan (one thin slice before B3, not part of Arc 0):

1. Free disk first (owner decision): at least 20 GB headroom before pulling images or generating data. Candidate: `cargo clean` in repos the owner agrees to, and pruning unused Docker layers.
2. Pin images by digest in a repo `.env.example` (zero hardcoding): `KUTHA_BENCH_FALKORDB_IMAGE`, `KUTHA_BENCH_SAMYAMA_IMAGE`, ports, dataset path.
3. Workload as data: LDBC-SNB-like small multi-hop subset plus the PIT-shaped graph from section 12 (versions of sections as nodes with VT intervals). Rendered to Cypher for the competitors, to IR for Kutha.
4. Harness: one script that runs calibration, then engine A, engine B, engine Kutha, back to back, three runs each, and reports ratios. It runs on an idle machine: stop `fd_api` and `9router` for the run only if the owner agrees.
5. Output: a short report under `.compound-engineering/artifacts/research/`; the B3 budget (regression floor) is derived from the ratio, not from absolute time.

## 12. Corpus: law-nexus and an English dataset

law-nexus (`/root/law-nexus`, Russian Consultant/Garant exports): `law-source/` is 284 MB and `consru_export/` is 4.6 GB. Both are proprietary commercial sources. Use them as a **private local evaluation corpus only**; do not commit them or derived text to this repository. The law-nexus product ADR-0016..0022 temporal ontology (design `[proposed]`) is a reference for the model, not a dependency.

English open datasets with machine-readable as-of versions (from web search, details to verify in a spike):

| Source | Why it fits | Format and access | License |
|--------|-------------|-------------------|---------|
| **eCFR Versioner API** (US Code of Federal Regulations) | Direct as-of date: `/api/versioner/v1/full/{date}/title-{n}.xml`, `structure/{date}/title-{n}.json`, `versions/title-{n}.json` with per-section `amendment_date`, `issue_date`, `removed` | XML and JSON over HTTP, no bulk dump needed | US federal government work, public domain (to confirm in the spike) |
| **legislation.gov.uk** (UK Acts) | Point-in-time versions of revised primary legislation (base date 1991-02-01), Atom/XML API, crawling and bulk access explicitly allowed | XML, Atom feeds | Open Government Licence v3.0 |

Recommendation: **eCFR first** (cleanest as-of contract, sections have explicit amendment histories, deterministic fixtures by date), **legislation.gov.uk second** to test a different jurisdiction model (acts, sections, commencement, extent). Keep committed fixtures small (a few parts or sections, with dates pinned and the fetch script recorded) and verify the licence text in the spike before committing any content.

Use in the arcs:

- B (benchmark and PIT fixture): eCFR section versions as the graph for the temporal workload; no Russian text needed.
- A (wedge): the thin legal pack is built on English open fixtures; law-nexus is the private realism check, so the committed repository stays redistributable.

## 13. Open questions (remaining)

1. Disk: may I propose a cleanup plan (build artifacts and Docker layers) and which locations are off limits?
2. May the baseline runs stop `fd_api` and `9router` temporarily on an idle window?
3. Hygiene (Arc 0), decided: branch `work/main-line` with a backup push of that branch. Still open: modified `reviews/2026-10-11-ce-whole-repo/report.md` (the owner's answer was unclear; left untouched).

