# Phase 6: Typed CSR lease - Context

**Gathered:** 2026-09-30
**Status:** Ready for planning

<domain>
## Phase Boundary

Deliver M011 **S06** (leased Active Slice): a **typed CSR lease** where the same endpoints can carry different relations or supports and still expose **labels and support multiplicity** (CSR-01), while the existing **untyped neighbor-set** path and FF5 lease-agrees-with-fold checks stay green (CSR-02). Named cargo tests + governor registration (GATE-01).

**In scope:** CSR-01, CSR-02, GATE-01 (pattern from Phases 4–5). ADR-040 clarification on untyped neighbor-set limits; ADR-041 CSR as droppable hop picture (not SoT).

**Out of this phase:** S07 provenance-only checks, S08 end-to-end fixture, GraphBLAS / Falkor runtime, Cypher/GPML, HNSW, full LFTJ over typed edges, Materializer cut-consistency verification, ADR-050 six dictionaries, legal/science packs. Do not treat honeycomb cells as Accepted. Do not change Correct / CorrectInterval or quantum outcome sidecar semantics.

</domain>

<decisions>
## Implementation Decisions

### Typed storage shape (Claude decided)

- **D-T1:** Add a **separate** typed lease type (working name `TypedCsrLease`) built from the same live `GraphFold` cut as today’s CSR. Do **not** replace or reinterpret `CsrLease` / `CsrLease::from_fold` — that remains the untyped neighbor-set (CSR-02). Typed rows are CSR offsets over **edge records**, each carrying at least `relation: TermId`, `object: TermId`, and `claim_id: EventId` (from live `Fact`). Exact Rust field/type names are planner/researcher discretion. Do **not** invent a second SoT or persist typed CSR to disk this phase (lease stays droppable RAM picture). — **Reversibility:** costly — once callers depend on `TypedCsrLease` / `typed_csr_lease_at`, renaming or merging into `CsrLease` touches API + tests

### Multiplicity vs dedup (Claude decided)

- **D-T2:** Untyped path keeps today’s `sort_unstable` + **`dedup` on object only** (one neighbor id per subject). Typed path **must not** collapse distinct live facts: same `(subject, object)` with different `relation` → distinct edges; same `(subject, relation, object)` with different `claim_id` (independent supports) → distinct edges (support multiplicity). Sort typed rows deterministically (recommended key `(relation, object, claim_id)` or equivalent) so seek/filter stays cheap; do not require leapfrog on typed rows in this phase. — **Reversibility:** reversible — lease build rule; untyped contract frozen

### Query API (Claude decided)

- **D-T3:** Keep `Runtime::csr_lease_at` → `CsrLease` and `neighbors` / `seek` unchanged for leapfrog / FF5 / materializer. Add a parallel entry (working name `typed_csr_lease_at`) returning the typed lease, plus an edges-out accessor (e.g. `edges_out(v) -> &[TypedEdge]`). Optional relation filter is **not** required for the thin oracles. Do **not** change `CsrMaterializer` to mount typed CSR this phase unless a one-line optional accessor is free; default is leave materializer on untyped `CsrLease`. — **Reversibility:** reversible — additive API

### Fixture oracles (Claude decided)

- **D-T4:** Ship **two named oracles** only: (1) CSR-01 — fixture with same endpoints, ≥2 relations and ≥2 supports on one `(s,r,o)`; after building typed lease at a named cut, labels and multiplicity are visible; untyped `neighbors` still shows a single object id when only objects collide. (2) CSR-02 — existing FF5 / untyped CSR drop-rebuild path stays green (reuse `ff5_legal_pit` / `csr_drop_rebuild_and_seek` behavior; add a thin regression in the new test file if needed that untyped lease still matches fold neighbors). Suggested exact names (planner may shorten but GATE needles must match): `typed_csr_preserves_relation_labels_and_support_multiplicity`, `untyped_csr_neighbor_set_and_ff5_still_hold`. Do **not** implement n-ary incidence, journey/waiting, or shared-build cost accounting (semantic-contract probes beyond S06). — **Reversibility:** reversible — test scope

### Governor and process inheritance (Claude decided)

- **D-T5:** Inherit Phase 2 **D-G1…D-G3** and **D-10 / D-11 / D-15**, and Phase 4/5 wave-close: every execute wave runs `uv run kutha-gov ci` (HIGH stops) + `explain trajectory` + D-10 Trajectory in SUMMARY; cargo on crate-touching waves. — **Reversibility:** reversible

- **D-T6:** Named cargo tests registered in governor (FSM observe + check/bridge needles), GATE-01, `ci` HIGH 0. Active Slice remains **S06** for the phase (GATE-02). Freeze / Proposed honeycomb (GATE-03). Do not edit `.kutha/STATE.md` during delivery. — **Reversibility:** reversible

- **D-T7:** Do not change fold Assert/Retract/Correct/CorrectInterval semantics; typed CSR is a **projection** of live facts only. Quantum outcomes sidecar (S05) is out of scope. — **Reversibility:** reversible

### Claude's Discretion

User said «реши эти вопросы разумно и при необходимости доисследуй» and required **codebase-memory-mcp** during execution → D-T1…D-T4 (plus D-T5…D-T7). Planner/researcher/executor MUST use CBM (`list_projects` first; `search_graph` / `trace_path` / `get_code_snippet`; `check_index_coverage` on touched paths; `detect_changes` after edits). Exact edge struct layout and test fixture constants are discretionary; must not thaw freeze, expand into S07–S08, or break untyped CSR callers (`csr_lease_at`, leapfrog, `CsrMaterializer.build`).

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### GSD overlay
- `.planning/ROADMAP.md` — Phase 6 goal, success criteria, S06 lease
- `.planning/REQUIREMENTS.md` — CSR-01, CSR-02, GATE-01…03
- `.planning/PROJECT.md` — v0.02 Semantic core close; lease rule
- `.planning/STATE.md` — GSD position (not harness lease)
- `.kutha/STATE.md` — Active Slice **S06**; M011; freeze; `L_delivery=M011-S05-done`
- `.kutha/ROADMAP.md` — M011 S06 “After this” line
- `.planning/phases/05-persisted-quantum-outcome/05-CONTEXT.md` — GATE / trajectory inheritance; out-of-scope fence for CSR
- `.planning/phases/04-partial-correction-with-residual-intervals/04-CONTEXT.md` — GATE-01 trio pattern

### Product contract
- `docs/architecture/semantic-contract-validation.md` — probe “Same endpoints, different relations/supports”
- `docs/ADR/ADR-040-materialization-plugin-protocol.md` — Clarification: current CSR is untyped neighbor-set
- `docs/ADR/ADR-041-csr-graphblas-hot-path.md` — CSR lease ≠ SoT; LFTJ on sorted neighbors
- `docs/ADR/ADR-011-lean-event-schema-lineage.md` — claim/support identity (`claim_id`)
- `docs/ADR/ADR-000-foundation-locks.md` — D1–D10

### Implementation anchors
- `crates/kutha-runtime/src/csr.rs` — `CsrLease::from_fold` (object-only + dedup)
- `crates/kutha-runtime/src/fold.rs` — `Fact { subject, relation, object, claim_id, … }`
- `crates/kutha-runtime/src/quantum.rs` — `Runtime::csr_lease_at`
- `crates/kutha-runtime/src/materializer.rs` — `CsrMaterializer` mounts untyped lease
- `crates/kutha-runtime/src/leapfrog.rs` — consumers of `neighbors`
- `crates/kutha-runtime/tests/ff5_legal_pit.rs` — FF5 / CSR drop oracles
- `AGENTS.md` — planes, Active Slice lease, freeze; CBM as structural evidence plane

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable assets (CBM-verified 2026-09-30 after reindex)
- `CsrLease` / `from_fold` / `neighbors` / `seek` — untyped adjacency; inbound callers: `Runtime::csr_lease_at`, `CsrMaterializer.build`, `leapfrog_common_neighbors_on_csr`, FF5 tests (`trace_path` inbound on `from_fold`: 7 callers)
- `Fact` — `subject`, `relation`, `object()`, `claim_id` available for typed projection (`get_code_snippet` on `Fact`)
- `Materializer` trait + `CsrMaterializer` — stays on untyped lease this phase
- Phase 4/5 GATE-01 YAML trio — copy pattern for `m011-typed-csr` (or similar)

### Established patterns
- Droppable leases rebuilt from fold at TT×VT cut; discard/rebuild must not change log/fold (FF3/FF5)
- Named `m011_*` / `ff*` integration tests + FSM `observe_cargo.required` needles

### Integration points
- New typed builder alongside `csr.rs` (or sibling module); `Runtime` additive method; tests under `crates/kutha-runtime/tests/`
- Do not rewrite leapfrog to require typed edges in S06

### CBM note for execute
- Project `kutha-graph` reindexed `full` 2026-09-30; `check_index_coverage` on `csr.rs`/`fold.rs` was `metadata_match` after rebuild. Re-check coverage after touching paths; parent reindexes only if stale/missing/asked.

</code_context>

<specifics>
## Specific Ideas

- Prefer `claim_id` on typed edges for support multiplicity rather than a bare count — matches M011 Fact identity and retract-one-support tests.
- Suggested GATE check id: `m011-typed-csr` / bridge `B-m011-typed-csr` (planner may adjust names; needles = exact test fn names).

</specifics>

<deferred>
## Deferred Ideas

- GraphBLAS / sparse-matrix CSR backends (ADR-041 non-goal for this slice)
- Typed LFTJ / Cypher MATCH over labeled edges
- `CsrMaterializer` cut-consistency checks (ADR-040 noted gap)
- Shared one-build / two-query cost accounting (semantic-contract probe)
- N-ary incidence / hypergraph projection (ADR-011)

</deferred>
