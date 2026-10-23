# ADR-040: Materialization Plugin Protocol

## Status

**Proposed** (Data-band protocol — reversible views/leases; CSR/HNSW are instances, not this cell’s kernels)

### Evidence (2026-10-02; Proposed)

Fold-internal ingest leases include `allows_by_name`, delivery-key, and EventId maps (`fold.rs`). Oracles: `green_ingest_visits_constant_in_n`, `lazy_as_of_matches_eager_after_same_ingest` (`m013_fold_indexes.rs`, `m013_fold_index_mode.rs`).

### Evidence (2026-10-23, Phase 231 / M157 S01; Proposed)

Hops consume the existing CSR lease (`typed_csr_lease_at` / `csr_lease_at`): droppable, not SoT. `execute(QueryIr)` does not add a materializer plugin. Status stays **Proposed**.

## Date

2026-08-18

## Honeycomb coordinates

- Axes: **Data** (primary) · **Composition** · **Time** · **Query**
- Depends on: ADR-000 (D5), ADR-010, ADR-014
- Anticipates: ADR-021 (pack install ≠ view contract), ADR-041 (CSR), ADR-042 (HNSW), ADR-043 (planner over leases)

## Context

D5 already says CSR/HNSW/temporal slices are **reversible plugins**. This cell is the **view contract**: mount / apply_delta / snapshot / unload / rollback — distinct from packing a module into the host.

Grounding cards:

- `paper-pg-materialized-views` — `.compound-engineering/artifacts/research/applicability/cards/paper-pg-materialized-views.md`
- `paper-dbsp-ivm` — `.compound-engineering/artifacts/research/applicability/cards/paper-dbsp-ivm.md`
- `paper-graph-pack-plugin-lifecycle` — `.compound-engineering/artifacts/research/applicability/cards/paper-graph-pack-plugin-lifecycle.md`

PG views: virtual rewrite vs stored materialization; DRed/B-F when recursive. DBSP: IVM as a stream calculus so log deltas become view deltas without per-language heuristics. Pack lifecycle: install/activate/retire of a **versioned module** (operators + dictionary + leases) is not the same noun as `build`/`drop` of one view. DuckPGQ injects planner hooks — do not vendor DuckDB. WASM sandbox is how untrusted *code* runs (051/081), not how a view is maintained.

## Decision

### D040-1. Every hot picture is a lease of the fold

CSR, HNSW, interval indexes, named-graph slices, Cypher views, and process-mining graphs are the same protocol noun: derived structures keyed by log offset. Reversible = drop and fold again. Incremental = apply a changeset. Both must be valid. Competing projections of the same type are allowed; routing is ADR-043.

### D040-2. Plugin lifecycle ≠ pack lifecycle

View contract: `build` / `apply(delta)` / `snapshot` / `unload` / `rollback`. Pack lifecycle (ADR-021, not opened here): install/activate/retire of a versioned artifact that *may register* plugins. Activating a pack is not silently mutating SoT.

### D040-3. IVM is the maintenance algebra, not a second log

DBSP-shaped incrementalization is the target for rich views. DRed-style delete/rederive remains the conservative cousin for recursive views. The event log stays SoT; view deltas are not events unless explicitly logged (e.g. schema/pack events).

### D040-4. Mandatory external graph DBs are forbidden as materializers-of-truth

Neo4j / Graphiti / Falkor / Samyama may inform **kernels** (041/042) but must not be required for assert → invalidate → AS OF.

### Clarification (2026-09-13): a lease preserves a declared query contract

Proposed lease identity binds branch/prefix, log offset, TT/VT cut, schema/term interpretation, projection version, relation filter, and set/bag/provenance mode. An arbitrary offset supplied beside a fold does not establish that they describe the same cut. Query execution must reject an incompatible/stale lease or rebuild it; authorization is checked separately (ADR-080), including any permission-filtered cache scope.

For each supported **exact** query contract, require `answer(rebuild(history, cut)) = answer(apply_deltas(view, history_tail), cut)` after assertions, retractions, and corrections, including support/multiplicity when requested. [DBSP](https://arxiv.org/abs/2203.16684) grounds incremental maintenance; positive provenance is a different algebra (ADR-011). Approximate leases such as HNSW instead declare quality metrics, allowed candidate differences, and controlled nondeterminism if identical results are required. Temporal cuts, authorization, and mandatory filters remain hard constraints in both cases. A lease may omit information only if its advertised operator does not need it, or if execution rechecks the base fold.

Current `CsrLease::from_fold` is an **untyped neighbor set**: it discards relation labels and deduplicates neighbors. It is not a typed MATCH/provenance view. `CsrMaterializer::build` records a supplied offset without verifying cut consistency. These are declared spike limits, not claims that typed projection or generic IVM is implemented.

### Open question (2026-09-30)

Open, from docs/architecture/semantic-gap-review.md (F8): the 2026-09-13 clarification already declares spike limits on untyped neighbor sets and offset recording without cut verification. Whether the hot path must invoke the materializer protocol, versus building a lease beside the trait, is not a new Decision. This question does not rewrite D040-1…D040-4 and does not mark this cell Accepted.

**Hard separations:**

```text
View / lease           ≠  Event log SoT
apply(delta)           ≠  Append to SoT
Pack install           ≠  View contract
IVM stream             ≠  Second database
Unload                 ≠  Vacuum of history
WASM sandbox           ≠  Materializer plugin
```

### Clarification (2026-10-01; Proposed)

Target contract, from docs/architecture/semantic-gap-review.md (F15): lookup indexes inside the fold (`EventId` to fact, delivery key to event) are leases in the ADR-040 sense: derived from the log, droppable, and rebuilt by replay. They restore constant-time ingest and carry no truth of their own. A benchmark baseline with a declared target size is the evidence, not the index itself. This subsection does not rewrite D040-1…D040-4 and does not mark this cell Accepted.

### Research variant (2026-10-19; Proposed): piecewise-linear position lease

A PGM-shaped index is a **candidate read lease**, not a new SoT and not an Accepted access method. The event log stays the source of truth (ADR-000 D1). Eager `BTreeMap` maintenance on the ingest path stays the incremental structure. HNSW (ADR-042) and Cypher stay frozen. This note does not rewrite D040-1…D040-4.

**Primary model.** Kraska et al. frame a learned index as a CDF from key to position (SIGMOD 2018, [doi:10.1145/3183713.3196909](https://doi.org/10.1145/3183713.3196909)). Ferragina and Vinciguerra turn that into the Piecewise Geometric Model: error-bounded linear segments, predecessor and range queries, and a fully dynamic variant with worst-case bounds (PVLDB 2020, [doi:10.14778/3389133.3389135](https://doi.org/10.14778/3389133.3389135)). The error is a **position** window. The answer inside the window is an exact search, not an approximate fact.

**What the PVLDB experiments actually report** (same paper, not a universal constant):

- Static predecessor/range: query time in the band of a cache-optimised static B+-tree, about **83×** less space.
- Fully dynamic inserts and deletes: up to about **71%** better query and update time than a B+-tree, about **1140×** less space.
- Across the suite: up to **three orders of magnitude** less space than the traditional and learned indexes they compared.

**What this repository may study.** Only a droppable lease over a **sealed, sorted `u64` column** (segment offset, wall, or another monotonic key) plus an exact fallback scan. `f64` slopes are not fold-fingerprint inputs. Multimaps (`valid_from` / `ingested_at` → many fact indexes) stay posting lists. `as_of` still checks liveness after the key cut. `as_known_on` resolves the anchor predecessor, then `known_at` walks `tt_by_ingest` for `ingested_at <=` that TT when hot maps are warm (M109); Lazy cold still scans until rebuild. Rebuild-on-seal matches D040-1. In-place insert on the Eager path does not: the static builder cannot apply a one-fact delta.

**Decision matrix, checked against sources.** The four rows below are a workload heuristic. They are not Kutha acceptance criteria. Numbers that the 2020 paper does not state are marked unverified.

| Workload | Heuristic | Grounding | Kutha reading |
|----------|-----------|-----------|----------------|
| Frequent in-place updates | Keep a B-tree / B+-tree | Reasonable as a default for page splits and locking. The 2020 dynamic PGM **can** beat a B+-tree on updates in that paper’s setting (up to 71%, 1140× less space). A later disk study finds the B+-tree robust across workloads, with learned indexes winning only on specific ones (Lan, Bao, Peng, PACMMOD 2023). | Eager ingest stays `BTreeMap`. Do not swap it for a static model. |
| Immutable or read-heavy sorted data | PGM | This is the static predecessor/range case the 2020 paper treats as optimal for the model class. Same shape as an SSTable or a sealed segment, not as a live page that splits. | Fit for a lease rebuilt when a segment seals. Not wired. |
| Tight RAM or cache for index metadata | PGM | Paper: up to ~10³× less **index** space in their comparisons. A “14,000×” figure is **not** that result. | Do not budget Kutha on 14,000×. |
| Faster than a contiguous binary search (claimed ~2.9×, 20 cache lines down to 4) | PGM | **Not** a 2020 paper result. A 2024 study finds PGM lookup memory-bound and reports a tuned variant up to 2.31× over the original PGM, not a 4-line guarantee (Liu et al., PVLDB 2024). | Do not treat 2.9× or “4 cache lines” as a lease requirement. |

**Implementation checked, not adopted.** `learned_index_lab` 0.2.0 (`2247c0430b73b711503be7c364b0d33642f2968e`, Apache-2.0, no dependencies) is a static one-level greedy corridor over `u64`, not the dynamic or recursive PGM. `find` trusts the ε window with no full-array fallback. `lower_bound` / `range` fall back to a full scan, so `O(log S + log ε)` is not always true. Duplicate keys can select the wrong segment. Its README tables (about 3.1 KB, sub-100 ns, ~12,000× versus a printed 45.78 MB B-tree) are not CI artifacts; the dense “miss” path is mostly keys below the minimum, and the index-only size excludes the owned key array. That crate is a spike candidate for the fallback-shaped API only.

**Not decided here.** No product code, no honeycomb Accept, no thaw of ADR-042. A later slice needs a killed oracle at the declared bench size (`KUTHA_BENCH_N`, target noted as 1_000_000 in `docs/architecture/benchmark-baseline.md`) against `BTreeMap::range` and `slice::binary_search`, with in-range misses and measured bytes.

## Consequences

### Positive

- 041/042 can be instances of one protocol.
- Planner (043) has a catalog of droppable access paths.
- Vertical packs (090/093) attach AST materializers without forking the engine.

### Negative / risks

- Dual-writing to an external graph “for speed” will recreate a second SoT.
- IVM of HNSW is harder than CSR; 042 must not pretend otherwise.

### Non-goals (this ADR)

- Specifying CSR layout (041) or HNSW repair (042).
- Opening ADR-021.
- Mandatory Graphiti/Neo4j.

## Alternatives Considered

| Alternative | Why not |
|-------------|---------|
| Periodic full rebuild only | Too slow after appends; DBSP exists |
| Pack marketplace as the view | Wrong noun |
| Graphiti/Neo4j as required materializer | Violates D1/D5 and STRATEGY |

## Open Research Questions

1. Trait shape (`MaterializerPort`) vs per-kind plugins — spike, not this ADR’s crate.
2. When apply(delta) must fall back to rebuild (HNSW delete-repair is 042).
3. Versioning leases when a dictionary/ontology pack changes (Bellini lifecycle).
4. Query rewrite through views (Han) vs always hitting base fold.
5. Whether a sealed-segment piecewise-linear position lease beats `BTreeMap::range` at the declared bench size, with an exact fallback. The 2026-10-19 note records the question; it does not authorize the swap.

## Related Decisions

- ADR-000 — D5
- ADR-010 — fold at offset
- ADR-014 — materializers consume cascade budget
- ADR-041, ADR-042, ADR-043 — instances and compiler
