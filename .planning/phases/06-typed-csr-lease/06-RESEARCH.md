# Phase 6: Typed CSR lease - Research

**Researched:** 2026-09-30
**Domain:** Kutha droppable CSR lease / typed edge projection of live Facts (ADR-040/041)
**Confidence:** HIGH (in-repo CSR/Fact/Runtime/governor GATE patterns); MEDIUM (exact TypedEdge field set beyond D-T1 minimum — resolved by recommendation below)

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

- **D-T1:** Add a **separate** typed lease type (working name `TypedCsrLease`) built from the same live `GraphFold` cut as today’s CSR. Do **not** replace or reinterpret `CsrLease` / `CsrLease::from_fold` — that remains the untyped neighbor-set (CSR-02). Typed rows are CSR offsets over **edge records**, each carrying at least `relation: TermId`, `object: TermId`, and `claim_id: EventId` (from live `Fact`). Exact Rust field/type names are planner/researcher discretion. Do **not** invent a second SoT or persist typed CSR to disk this phase (lease stays droppable RAM picture). — **Reversibility:** costly — once callers depend on `TypedCsrLease` / `typed_csr_lease_at`, renaming or merging into `CsrLease` touches API + tests

- **D-T2:** Untyped path keeps today’s `sort_unstable` + **`dedup` on object only** (one neighbor id per subject). Typed path **must not** collapse distinct live facts: same `(subject, object)` with different `relation` → distinct edges; same `(subject, relation, object)` with different `claim_id` (independent supports) → distinct edges (support multiplicity). Sort typed rows deterministically (recommended key `(relation, object, claim_id)` or equivalent) so seek/filter stays cheap; do not require leapfrog on typed rows in this phase. — **Reversibility:** reversible — lease build rule; untyped contract frozen

- **D-T3:** Keep `Runtime::csr_lease_at` → `CsrLease` and `neighbors` / `seek` unchanged for leapfrog / FF5 / materializer. Add a parallel entry (working name `typed_csr_lease_at`) returning the typed lease, plus an edges-out accessor (e.g. `edges_out(v) -> &[TypedEdge]`). Optional relation filter is **not** required for the thin oracles. Do **not** change `CsrMaterializer` to mount typed CSR this phase unless a one-line optional accessor is free; default is leave materializer on untyped `CsrLease`. — **Reversibility:** reversible — additive API

- **D-T4:** Ship **two named oracles** only: (1) CSR-01 — fixture with same endpoints, ≥2 relations and ≥2 supports on one `(s,r,o)`; after building typed lease at a named cut, labels and multiplicity are visible; untyped `neighbors` still shows a single object id when only objects collide. (2) CSR-02 — existing FF5 / untyped CSR drop-rebuild path stays green (reuse `ff5_legal_pit` / `csr_drop_rebuild_and_seek` behavior; add a thin regression in the new test file if needed that untyped lease still matches fold neighbors). Suggested exact names (planner may shorten but GATE needles must match): `typed_csr_preserves_relation_labels_and_support_multiplicity`, `untyped_csr_neighbor_set_and_ff5_still_hold`. Do **not** implement n-ary incidence, journey/waiting, or shared-build cost accounting (semantic-contract probes beyond S06). — **Reversibility:** reversible — test scope

- **D-T5:** Inherit Phase 2 **D-G1…D-G3** and **D-10 / D-11 / D-15**, and Phase 4/5 wave-close: every execute wave runs `uv run kutha-gov ci` (HIGH stops) + `explain trajectory` + D-10 Trajectory in SUMMARY; cargo on crate-touching waves. — **Reversibility:** reversible

- **D-T6:** Named cargo tests registered in governor (FSM observe + check/bridge needles), GATE-01, `ci` HIGH 0. Active Slice remains **S06** for the phase (GATE-02). Freeze / Proposed honeycomb (GATE-03). Do not edit `.kutha/STATE.md` during delivery. — **Reversibility:** reversible

- **D-T7:** Do not change fold Assert/Retract/Correct/CorrectInterval semantics; typed CSR is a **projection** of live facts only. Quantum outcomes sidecar (S05) is out of scope. — **Reversibility:** reversible

### Claude's Discretion

User said «реши эти вопросы разумно и при необходимости доисследуй» and required **codebase-memory-mcp** during execution → D-T1…D-T4 (plus D-T5…D-T7). Planner/researcher/executor MUST use CBM (`list_projects` first; `search_graph` / `trace_path` / `get_code_snippet`; `check_index_coverage` on touched paths; `detect_changes` after edits). Exact edge struct layout and test fixture constants are discretionary; must not thaw freeze, expand into S07–S08, or break untyped CSR callers (`csr_lease_at`, leapfrog, `CsrMaterializer.build`).

### Deferred Ideas (OUT OF SCOPE)

- GraphBLAS / sparse-matrix CSR backends (ADR-041 non-goal for this slice)
- Typed LFTJ / Cypher MATCH over labeled edges
- `CsrMaterializer` cut-consistency checks (ADR-040 noted gap)
- Shared one-build / two-query cost accounting (semantic-contract probe)
- N-ary incidence / hypergraph projection (ADR-011)
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| CSR-01 | Developer can query a CSR lease where the same endpoints carry different relations or supports and see labels and support multiplicity preserved | Separate `TypedCsrLease` + `typed_csr_lease_at` + `edges_out` projecting each live `Fact` with `relation` / `object` / `claim_id` (+ recommended `fact_seq`); fixture oracle `typed_csr_preserves_relation_labels_and_support_multiplicity` (D-T1–D-T4) |
| CSR-02 | Developer still gets the untyped neighbor-set path, and the FF5 lease-agrees-with-fold check stays green | Leave `CsrLease::from_fold` / `csr_lease_at` / `neighbors` / `seek` / `CsrMaterializer` / leapfrog untouched; named oracle `untyped_csr_neighbor_set_and_ff5_still_hold`; keep `ff5_as_of_t1_differs_from_as_of_t2_on_statute_log` in `observe_cargo.required` (D-T2/D-T3/D-T4) |
| GATE-01 | Named cargo tests registered in governor; `uv run kutha-gov ci` stays 0 HIGH | Copy Phase 5 trio: `fsm.yaml` `observe_cargo.required` + `checks.yaml` `m011-typed-csr` + `bridges.yaml` `B-m011-typed-csr`; docs-coupling CHANGELOG (D-T5/D-T6) |
| GATE-02 | Execute only while Active Slice is S06 | `.kutha/STATE.md` already names **Active Slice: S06** — executable; do not edit STATE / do not check ROADMAP S06 during delivery |
| GATE-03 | Freeze unstarted; honeycomb stays Proposed | No Rocks/Cypher/HNSW/ADR-050 six dicts; ADR-040/041 `map: Proposed`; evidence list append only |
</phase_requirements>

## Summary

Phase 6 delivers M011 **S06** (Active Slice leased): a **parallel typed CSR lease** that preserves relation labels and support multiplicity for the semantic-contract probe “Same endpoints, different relations/supports”, while the existing **untyped neighbor-set** CSR and FF5 cut agreement stay green.

Today `CsrLease::from_fold` pushes only `f.object()`, then `sort_unstable` + `dedup` — relation and claim identity are discarded. ADR-040 Clarification names this limit explicitly. `Fact` already carries `subject`, `relation`, `object()`, and `claim_id`. Inbound callers of `from_fold` (Runtime, materializer, leapfrog, FF5/FF3 tests) must keep consuming untyped `CsrLease`. The phase is additive: new types + `Runtime::typed_csr_lease_at`, two named integration tests, GATE-01 governor needles. No new crates, no disk persistence of typed CSR, no fold/op changes, no leapfrog rewrite.

**Primary recommendation:** Implement `TypedEdge` + `TypedCsrLease` in `crates/kutha-runtime/src/csr.rs` (same module as untyped), build one edge per live `Fact` (include `fact_seq` so shared-`claim_id` supports from M011 S01 cannot collapse), expose `Runtime::typed_csr_lease_at` + `edges_out`, ship the two D-T4 oracles under `tests/m011_typed_csr.rs`, then register GATE-01 like Phase 5 (`m011-typed-csr` / `B-m011-typed-csr`). Leave `CsrMaterializer` on untyped lease.

## Architectural Responsibility Map

| Capability | Primary Tier | Secondary Tier | Rationale |
|------------|-------------|----------------|-----------|
| Untyped neighbor-set CSR (`CsrLease`) | API / Backend (`kutha-runtime` CSR lease) | — | Existing hop picture; CSR-02 frozen contract |
| Typed edge CSR (`TypedCsrLease`) | API / Backend (new parallel lease) | — | Projection of live Facts; droppable RAM (D-T1) |
| Live fact cut (`GraphFold` / `Fact::is_live_at`) | API / Backend (fold picture) | — | Both leases rebuild from same TT×VT cut; not SoT |
| Event log SoT | Database / Storage (`EventLog`) | — | D-T7: no Op/fold semantic change |
| Leapfrog intersect | API / Backend (`leapfrog` on untyped neighbors) | — | Must keep using `CsrLease::neighbors`; no typed LFTJ |
| Materializer mount | API / Backend (`CsrMaterializer`) | — | Stays on untyped `CsrLease` (D-T3) |
| CSR-01/CSR-02 oracles | Test runner (integration tests) | — | Named cargo fns are fitness evidence |
| GATE-01 registration | Harness dictionaries (YAML) | — | Observe names; bridge cites product tests |
| Freeze / honeycomb map | Harness + ADR status | — | GATE-03; map stays Proposed |

## Project Constraints (from .cursor/rules/)

- Chat with the user is Russian; this RESEARCH.md and all planning artifacts stay English.
- Do **not** load LifeOS / PAI / notify endpoints (`quiet-no-lifeos`).
- Structural search: `codebase-memory-mcp`; `list_projects` first; graph before Grep for symbols; `check_index_coverage` on cited paths; `detect_changes` after edits. **Do not** call `index_repository` (unless missing/stale/user-asked), `delete_project`, or `manage_adr` (`code-graph-cbm.mdc`).
- CBM this session: project `kutha-graph` present (`nodes≈17717`, `index_mode: full`, `indexed_at: 2026-09-30T01:27:49Z`). Cited paths `csr.rs` / `fold.rs` / `quantum.rs` / `materializer.rs` / `leapfrog.rs` / `ff5_legal_pit.rs` / `lib.rs` reported `no_recorded_issue` + `freshness: metadata_match`. Claims below that quote discrete values cite **Read** line ranges.
- Graphify overlay is **disabled** in GSD config — no `graphify query`.
- GitNexus is secondary — not queried for this phase.
- CE skill routing for Russian verbs stays on `.cursor/rules/ce-skills-ru.mdc` (коммит → `ce-commit`, etc.) at execute time — not a research deliverable.
- Product plane: `crates/kutha-*`. Harness: `scripts/kutha_gov`, `.kutha/`. No Python in `kutha-runtime`. No repo-root hexagon folders.
- Freeze until STATE names M002: no RocksDB, Cypher/GPML, HNSW, ADR-050 six dictionaries, legal pack.
- Honeycomb cells stay **Proposed**. Governor green ≠ ADR Accepted ≠ `L_capability`.
- New governor check = YAML row. Bridge cites product tests. Do not add `scripts/kutha_gov/checks/*.py`.
- `docs-coupling`: crate diffs and harness dictionary diffs need `CHANGELOG.md`.
- Do **not** edit `.kutha/STATE.md` during S06 delivery. ROADMAP S06 checkbox stays unchecked while leased.

## Standard Stack

### Core

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| Rust edition | `2021` (workspace) | Language | Existing workspace |
| `kutha-common` | `0.0.0` path | `TermId`, `EventId` (= `Uuid`), `Op` | Log SoT types; no new deps |
| `kutha-runtime` | `0.0.0` path | fold, CSR, Runtime, materializer, leapfrog | Sole implementation surface |
| `uuid` | workspace `1` (features v7/std/serde) | `EventId` / `claim_id` ordering | Already required; `Uuid` derives `Ord` |

### Supporting

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `serde` / `serde_json` | workspace `1` | Existing fold/store | Do **not** serialize typed CSR this phase (D-T1) |
| cargo / uv / kutha-gov | local toolchain | Tests + governor | GATE-01 / D-T5 |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Separate `TypedCsrLease` | Overload `CsrLease` with typed payloads | Forbidden by D-T1/CSR-02 — would break untyped callers |
| New `typed_csr.rs` module | Keep types in `csr.rs` | Sibling module fine; same-module preferred for dual-lease cohesion |
| GraphBLAS / sparse crate | In-repo CSR vectors | Deferred; freeze / ADR-041 non-goal for S06 |
| Typed leapfrog | Untyped `leapfrog_intersect` only | Deferred (D-T2/D-T3) |

**Installation:**

```bash
# No new crates. Workspace members stay kutha-common + kutha-runtime only.
cargo test -p kutha-runtime --offline
uv run kutha-gov ci
```

**Version verification:** Workspace `Cargo.toml` members `kutha-common`, `kutha-runtime` at `0.0.0`; `edition = "2021"`. No `cargo search` / PyPI installs for this phase. [VERIFIED: Cargo.toml workspace members via `cargo metadata --no-deps`]

## Package Legitimacy Audit

> Phase installs **no** external packages. Legitimacy gate N/A for new names.

| Package | Registry | Age | Downloads | Source Repo | Verdict | Disposition |
|---------|----------|-----|-----------|-------------|---------|-------------|
| — | — | — | — | — | N/A | No installs |

**Packages removed due to [SLOP] verdict:** none
**Packages flagged as suspicious [SUS]:** none

*Do not add GraphBLAS, ndarray CSR crates, or Cypher parsers in this phase.*

## Architecture Patterns

### System Architecture Diagram

```text
                    Op::Assert / Retract / …
                            │
                            ▼
                     EventLog (SoT)
                            │
                            ▼
                      GraphFold (picture)
                            │
              ┌─────────────┴─────────────┐
              │ live Facts @ (tt, vt)     │
              └─────────────┬─────────────┘
                            │
         ┌──────────────────┼──────────────────┐
         ▼                                     ▼
 CsrLease::from_fold                    TypedCsrLease::from_fold
 (object-only; sort+dedup)              (one edge per live Fact;
         │                               sort; NO dedup)
         ▼                                     ▼
 Runtime::csr_lease_at                  Runtime::typed_csr_lease_at
 neighbors / seek                       edges_out(v) -> &[TypedEdge]
         │                                     │
         ├─ leapfrog_intersect                 └─ CSR-01 oracle
         ├─ CsrMaterializer.build
         └─ FF5 / CSR-02 oracle
```

### Recommended Project Structure

```text
crates/kutha-runtime/
├── src/
│   ├── csr.rs              # CsrLease (unchanged) + TypedEdge + TypedCsrLease
│   ├── quantum.rs          # + typed_csr_lease_at beside csr_lease_at
│   ├── lib.rs              # pub use TypedCsrLease, TypedEdge
│   ├── materializer.rs     # unchanged (untyped)
│   └── leapfrog.rs         # unchanged (untyped neighbors)
└── tests/
    └── m011_typed_csr.rs   # CSR-01 + CSR-02 named oracles

.kutha/dictionaries/
├── fsm.yaml                # + two observe_cargo.required names
├── checks.yaml             # + m011-typed-csr
├── bridges.yaml            # + B-m011-typed-csr
└── honeycomb.yaml          # ADR-040/041 evidence append; map Proposed
```

### Pattern 1: Dual lease from one cut
**What:** Untyped and typed leases rebuild independently from the same `GraphFold` + `(tt, vt)` + `vertex_count`.
**When to use:** Always for S06 — never mutate one lease into the other.
**Example:**

```rust
// Source: crates/kutha-runtime/src/quantum.rs:324-327 (existing untyped)
pub fn csr_lease_at(&self, tt: u64, vt: u64) -> CsrLease {
    CsrLease::from_fold(&self.fold, tt, vt, self.dict.len())
}

// Additive (recommended):
pub fn typed_csr_lease_at(&self, tt: u64, vt: u64) -> TypedCsrLease {
    TypedCsrLease::from_fold(&self.fold, tt, vt, self.dict.len())
}
```

### Pattern 2: GATE-01 trio (Phase 5)
**What:** FSM observe names + `file_contains` check + bridge citing the test file.
**When to use:** Every M011 slice close (D-T6).
**Example needles (exact fn names must match observe + check):**

```yaml
# fsm.yaml observe_cargo.required append:
# - typed_csr_preserves_relation_labels_and_support_multiplicity
# - untyped_csr_neighbor_set_and_ff5_still_hold

# checks.yaml:
# - id: m011-typed-csr
#   steps: file_contains on crates/kutha-runtime/tests/m011_typed_csr.rs
#   category: m011-s06

# bridges.yaml:
# - id: B-m011-typed-csr
#   check: m011-typed-csr
#   cites: crates/kutha-runtime/tests/m011_typed_csr.rs
```

### Anti-Patterns to Avoid

- **Replacing `CsrLease` fields with typed rows:** Breaks leapfrog/FF5/materializer (D-T1/CSR-02).
- **Dedup on typed `(relation, object, claim_id)` only:** Collapses M011 S01 supports that share one `claim_id` but are distinct Facts — see Open Question Q1.
- **Persisting typed CSR / mounting it in `CsrMaterializer`:** Out of scope (D-T1/D-T3).
- **Changing fold Assert/Retract/Correct/CorrectInterval:** Forbidden (D-T7).
- **Checking ROADMAP S06 `[x]` or editing STATE while leased:** Trajectory honesty / GATE-02.
- **Treating governor green as ADR Accepted:** D-10 Trajectory must say so.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Neighbor-set CSR | New adjacency crate | Existing `CsrLease` | Untyped path must stay byte-compatible for FF5 |
| Typed edge projection | GraphBLAS / Cypher MATCH | `TypedCsrLease::from_fold` over `Fact` | Lease ≠ SoT; ADR-041 spike already owns CSR vectors |
| Claim / support identity | Bare multiplicity counter | `Fact.claim_id` (+ `fact_seq`) | Matches M011 S01 / ADR-011 |
| Cut liveness | Custom time checks | `Fact::is_live_at` | Same cut rule as untyped CSR |
| GATE registration | Ad-hoc scripts | YAML check/bridge/fsm trio | Harness constitution |
| Sorted seek on untyped | Custom binary search | `CsrLease::seek` | Already correct for leapfrog |

**Key insight:** The hard part is **not** inventing CSR storage — it is keeping two projections honest: typed bag of edges vs untyped set of objects, rebuilt from the same live Facts without touching SoT.

## Common Pitfalls

### Pitfall 1: Dedup wiping support multiplicity
**What goes wrong:** Typed builder copies untyped `dedup` habit; two live Facts for the same `(s,r,o)` become one edge.
**Why it happens:** `CsrLease::from_fold` teaches `sort_unstable` + `dedup` as the default.
**How to avoid:** Typed path: sort only; **never** `dedup`. One edge per live Fact.
**Warning signs:** `edges_out(s).len() == 1` while `fold.live_support_count(...) == 2`.

### Pitfall 2: Breaking untyped callers
**What goes wrong:** Changing `neighbors` return type or `from_fold` signature.
**Why it happens:** “Unify” typed and untyped into one struct.
**How to avoid:** Additive API only (D-T3). Run FF5 + `csr_drop_rebuild_and_seek` + leapfrog unit test every crate wave.
**Warning signs:** Compile errors in `leapfrog.rs` / `materializer.rs`; FF5 neighbor asserts fail.

### Pitfall 3: GATE needle / fn name drift
**What goes wrong:** Test renamed but `observe_cargo.required` or `file_contains` needles not updated → `ci` HIGH.
**Why it happens:** Planner shortens names inconsistently.
**How to avoid:** Lock D-T4 names early; copy Phase 5 pattern; one commit adds both tests and needles.
**Warning signs:** `observe-required-fn` missing messages in ci output.

### Pitfall 4: Treating lease as SoT or Accepted ADR
**What goes wrong:** Persisting typed CSR, flipping honeycomb to Accepted, or claiming L_capability from ci green.
**Why it happens:** Success criteria confusion.
**How to avoid:** D-T1 droppable RAM; GATE-03 Proposed; D-10 Trajectory text.
**Warning signs:** New store files; `map: Accepted` in honeycomb; STATE edits.

### Pitfall 5: Vertex-count / subject bounds
**What goes wrong:** Subject `TermId` ≥ `vertex_count` silently dropped inconsistently between leases.
**Why it happens:** Both builders use `if s < buckets.len()`.
**How to avoid:** Use identical `self.dict.len()` for both Runtime entry points; same TT/VT in oracles.
**Warning signs:** Typed edges missing for late-interned subjects.

## Code Examples

### Untyped from_fold (must remain)

```rust
// Source: crates/kutha-runtime/src/csr.rs:12-33 [VERIFIED]
pub fn from_fold(fold: &GraphFold, tt: u64, vt: u64, vertex_count: usize) -> Self {
    let mut buckets: Vec<Vec<TermId>> = vec![Vec::new(); vertex_count];
    for f in fold.facts() {
        if !f.is_live_at(tt, vt) {
            continue;
        }
        let s = f.subject as usize;
        if s < buckets.len() {
            buckets[s].push(f.object());
        }
    }
    // ... sort_unstable(); dedup(); ...
}
```

### Fact fields available for typed projection

```rust
// Source: crates/kutha-runtime/src/fold.rs:62-74 [VERIFIED]
pub struct Fact {
    pub seq: u64,
    pub subject: TermId,
    pub relation: TermId,
    object: TermId,
    pub valid_from: ValidTime,
    pub valid_to: Option<ValidTime>,
    pub ingested_at: TransactionTime,
    pub invalidated_at: Option<TransactionTime>,
    /// Portable claim identity (ADR-011). Multiple Facts may share one claim_id (supports).
    #[serde(default = "nil_claim")]
    pub claim_id: EventId,
}
```

### Recommended TypedEdge / TypedCsrLease skeleton

```rust
// Recommended shape (discretion within D-T1 "at least" fields)
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypedEdge {
    pub relation: TermId,
    pub object: TermId,
    pub claim_id: EventId,
    pub fact_seq: u64, // recommended — see Open Question Q1
}

pub struct TypedCsrLease {
    offsets: Vec<u32>,
    edges: Vec<TypedEdge>,
}

impl TypedCsrLease {
    pub fn from_fold(fold: &GraphFold, tt: u64, vt: u64, vertex_count: usize) -> Self {
        let mut buckets: Vec<Vec<TypedEdge>> = vec![Vec::new(); vertex_count];
        for f in fold.facts() {
            if !f.is_live_at(tt, vt) {
                continue;
            }
            let s = f.subject as usize;
            if s < buckets.len() {
                buckets[s].push(TypedEdge {
                    relation: f.relation,
                    object: f.object(),
                    claim_id: f.claim_id,
                    fact_seq: f.seq,
                });
            }
        }
        let mut offsets = Vec::with_capacity(vertex_count + 1);
        let mut edges = Vec::new();
        offsets.push(0);
        for mut row in buckets {
            row.sort_unstable_by_key(|e| (e.relation, e.object, e.claim_id, e.fact_seq));
            // NO dedup
            edges.extend(row);
            offsets.push(edges.len() as u32);
        }
        Self { offsets, edges }
    }

    pub fn edges_out(&self, v: TermId) -> &[TypedEdge] {
        let i = v as usize;
        if i + 1 >= self.offsets.len() {
            return &[];
        }
        let a = self.offsets[i] as usize;
        let b = self.offsets[i + 1] as usize;
        &self.edges[a..b]
    }
}
```

### CSR-01 fixture sketch (satisfies D-T4)

```rust
// Same endpoints s→o; two relations; two supports on one (s,r,o)
// 1) Assert s knows o (claim None) → claim_a
// 2) Assert s knows o (claim None) → claim_b  // ≥2 supports, different claim_id (D-T2)
// 3) Assert s relatedTo o (claim None)         // ≥2 relations
// typed.edges_out(s): len >= 3; relations include knows + relatedTo; two knows edges
// untyped.neighbors(s): single [o] after dedup
```

### Inbound callers that must stay on untyped (CBM trace_path)

`CsrLease::from_fold` inbound (include_tests): `Runtime::csr_lease_at`, `CsrMaterializer.build`, `leapfrog_common_neighbors_on_csr`, `csr_drop_rebuild_and_seek`, `csr_materializer_build_unload`, `ff5_as_of_t1_differs_from_as_of_t2_on_statute_log`, `ff3_csr_lease_drop_does_not_change_log_or_fold` — **7 callers**. Do not redirect these to typed.

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Untyped neighbor-set only (`CsrLease`) | Dual lease: untyped + typed edge CSR | Phase 6 / S06 | CSR-01 oracle becomes executable without breaking FF5 |
| ADR-040 Clarification documents limit | Code implements typed projection | This phase | Clarification remains true for `CsrLease`; typed is separate type |
| GraphBLAS / Falkor as aspirational hot path | Still deferred | Until M002+ / later | Do not vendor sparse-matrix runtimes |

**Deprecated/outdated:**

- Treating current CSR as typed MATCH/provenance view — ADR-040 Clarification forbids that reading of `CsrLease`.

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | Optional relation-filter API is unnecessary for thin oracles | Architecture (D-T3) | Locked by CONTEXT — low risk |
| A2 | `fact_seq` on `TypedEdge` is the right discrete identity for shared-`claim_id` supports | Open Question Q1 | If omitted, S01-style supports collapse under sort+accidental uniqueness assumptions |
| A3 | Placing typed types in `csr.rs` (not a new file) is preferred | Structure | Cosmetic; either works |

**If `fact_seq` is accepted by planner:** A2 becomes a locked implementation decision rather than an assumption.

## Open Questions

1. **Q1 — Must `TypedEdge` include `fact_seq` beyond D-T1’s `{relation, object, claim_id}`?**
   - What we know: D-T1 requires at least those three fields. D-T2 says distinct edges when `claim_id` differs. M011 S01 independent supports **share** one `claim_id` across multiple Facts (`fold.rs` comment: “Multiple Facts may share one claim_id”). [VERIFIED: crates/kutha-runtime/src/fold.rs:71-73]
   - What's unclear: Whether the CSR-01 fixture will only use different `claim_id`s (literal D-T2) or also exercise shared-`claim_id` supports.
   - **Recommendation (planner lock):** Include `pub fact_seq: u64` on every typed edge; project **one edge per live Fact**; never dedup. Fixture may still use two `claim: None` Asserts (different claim ids) for D-T2 wording, but the type stays correct for S01. Sort key `(relation, object, claim_id, fact_seq)`.

2. **Q2 — Module file: extend `csr.rs` or add `typed_csr.rs`?**
   - What we know: `csr.rs` is 51 lines; `lib.rs` already `pub use csr::CsrLease`.
   - **Recommendation:** Keep in `csr.rs` and export `TypedCsrLease` / `TypedEdge` from `lib.rs`. Splitting is optional if the planner wants clearer file ownership — not required.

3. **Q3 — Honeycomb evidence: ADR-040, ADR-041, or both?**
   - What we know: Both cells have `map: Proposed` and empty `evidence: []` today. [VERIFIED: .kutha/dictionaries/honeycomb.yaml ADR-040/041 blocks]
   - **Recommendation:** Append both oracle fn names to **ADR-040** and **ADR-041** evidence lists; leave `map: Proposed`. Do not Accept cells.

4. **Q4 — Does CSR-02 oracle re-assert FF5 statute numbers or only thin untyped regression?**
   - What we know: D-T4 allows reuse of FF5 / `csr_drop_rebuild_and_seek` behavior plus a thin regression in the new file. `ff5_as_of_t1_differs_from_as_of_t2_on_statute_log` already stays in `observe_cargo.required`.
   - **Recommendation:** Implement `untyped_csr_neighbor_set_and_ff5_still_hold` as a **thin** test: emit two relations to same object, assert untyped `neighbors` is a single object id, rebuild lease after drop, and optionally call `csr_lease_at` cut agreement on a tiny statute-shaped pair — do **not** duplicate the full FF5 statute fixture unless cheap. Keep existing FF5 test untouched and still observed.

5. **Q5 — Changelog / wave split?**
   - **Recommendation:** Wave 1 = crate API + `m011_typed_csr.rs` + Product changelog. Wave 2 = fsm/checks/bridges/honeycomb + Process/Trajectory changelog + `kutha-gov ci`. Matches Phase 5.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| cargo / rustc | crate tests | ✓ | 1.98.1 | — |
| uv | kutha-gov ci | ✓ | 0.12.13 | — |
| kutha-gov | GATE-01 / D-T5 | ✓ | workspace scripts | — |
| codebase-memory-mcp | execute verification | ✓ | project indexed 2026-09-30 | Grep only if coverage gap |
| New crates / GraphBLAS | — | N/A | — | Must not install |
| RocksDB / Cypher / HNSW | — | frozen | — | Out of scope |

**Missing dependencies with no fallback:** none

**Missing dependencies with fallback:** none

Step 2.6: external tools required are cargo + uv only — both present.

## Validation Architecture

> `workflow.nyquist_validation` is **true** in `.planning/config.json`.

### Test Framework

| Property | Value |
|----------|-------|
| Framework | Rust `cargo test` (lib + integration) |
| Config file | workspace `Cargo.toml` / crate defaults (no jest/pytest for product) |
| Quick run command | `cargo test -p kutha-runtime --offline typed_csr -- --nocapture` |
| Full suite command | `cargo test --workspace --offline` then `uv run kutha-gov ci` |

### Phase Requirements → Test Map

| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| CSR-01 | Typed lease shows ≥2 relations + ≥2 supports; labels/multiplicity visible | integration | `cargo test -p kutha-runtime --offline typed_csr_preserves_relation_labels_and_support_multiplicity -- --exact` | ❌ Wave 0 — create `tests/m011_typed_csr.rs` |
| CSR-02 | Untyped neighbor-set + FF5 path still hold | integration + existing | `cargo test -p kutha-runtime --offline untyped_csr_neighbor_set_and_ff5_still_hold -- --exact`; also `ff5_as_of_t1_differs_from_as_of_t2_on_statute_log` | ❌ Wave 0 new thin test; ✅ existing `tests/ff5_legal_pit.rs` |
| GATE-01 | FSM observe + check needles; ci HIGH 0 | harness | `uv run kutha-gov ci` | ❌ Wave 0 — YAML rows after tests exist |
| Regression | Untyped drop/rebuild/seek | unit | `cargo test -p kutha-runtime --offline --lib csr_drop` | ✅ `quantum::tests::csr_drop_rebuild_and_seek` |
| Regression | Leapfrog on untyped neighbors | unit | `cargo test -p kutha-runtime --offline --lib leapfrog_common_neighbors_on_csr` | ✅ in `leapfrog.rs` |

### Sampling Rate

- **Per task commit:** `cargo test -p kutha-runtime --offline typed_csr` (after Wave 1) or targeted lib tests
- **Per wave merge:** `cargo test --workspace --offline` + `uv run kutha-gov ci` (Wave 2+)
- **Phase gate:** Full suite green before `/gsd-verify-work`

### Wave 0 Gaps

- [ ] `crates/kutha-runtime/tests/m011_typed_csr.rs` — covers CSR-01 + CSR-02 named oracles
- [ ] `TypedCsrLease` / `TypedEdge` / `typed_csr_lease_at` — product API under test
- [ ] `.kutha/dictionaries/{fsm,checks,bridges,honeycomb}.yaml` — GATE-01 registration (after oracles exist)
- [ ] Product + Process `CHANGELOG.md` entries — docs-coupling

*(Existing infrastructure: cargo workspace tests, `ff5_legal_pit.rs`, governor observe_cargo — reuse; no new test framework install.)*

## Security Domain

> `security_enforcement` enabled (ASVS level 1) in `.planning/config.json`.

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | no | No user auth surface in S06 |
| V3 Session Management | no | No sessions |
| V4 Access Control | no | ABAC / ADR-080 frozen; lease is in-process |
| V5 Input Validation | yes | Bound `TermId`/`subject` against `vertex_count`; use existing admit/allowlist for Assert emit in fixtures |
| V6 Cryptography | no | No new crypto; do not invent receipt authenticity here |

### Known Threat Patterns for Kutha CSR leases

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Treating droppable CSR as SoT / tampering answers by mutating lease | Tampering | Rebuild from fold; FF3 drop does not change log/fold; do not persist typed CSR |
| Collapsing supports → false singleton evidence | Tampering / Information disclosure | One edge per live Fact; no typed dedup |
| Relation smuggling via typed path bypassing allowlist | Elevation | Typed lease is read-only projection; emit still goes through `Runtime::admit` / FF6 allowlist |
| Accidental Accepted/ADR promotion via evidence | Spoofing (process) | honeycomb `map: Proposed` only; Trajectory text |

## Sources

### Primary (HIGH confidence)

- CBM `list_projects` / `search_graph` / `trace_path` / `get_code_snippet` / `check_index_coverage` on `kutha-graph` (2026-09-30 full index; cited paths `metadata_match`)
- `crates/kutha-runtime/src/csr.rs` — `CsrLease` untyped build [VERIFIED: lines 6-50]
- `crates/kutha-runtime/src/fold.rs` — `Fact` fields [VERIFIED: lines 62-95]
- `crates/kutha-runtime/src/quantum.rs` — `csr_lease_at` [VERIFIED: lines 324-327]; `csr_drop_rebuild_and_seek` unit test
- `crates/kutha-runtime/src/materializer.rs` — `CsrMaterializer::build` mounts `CsrLease::from_fold`
- `crates/kutha-runtime/src/leapfrog.rs` — untyped neighbor consumers
- `crates/kutha-runtime/tests/ff5_legal_pit.rs` — FF5 CSR cut agreement
- `crates/kutha-runtime/tests/m011_claim_supports.rs` — shared `claim_id` supports
- `.kutha/dictionaries/{fsm,checks,bridges,honeycomb}.yaml` — GATE-01 Phase 5 pattern
- `docs/ADR/ADR-040-materialization-plugin-protocol.md` Clarification — untyped neighbor-set
- `docs/ADR/ADR-041-csr-graphblas-hot-path.md` — CSR lease ≠ SoT
- `docs/architecture/semantic-contract-validation.md` — “Same endpoints, different relations/supports”
- `.planning/phases/05-persisted-quantum-outcome/05-02-PLAN.md` — GATE wave template
- `.kutha/STATE.md` — Active Slice S06
- uuid crate `Uuid` derives `Ord` [VERIFIED: registry uuid-1.x `derive(... Ord ...)`]

### Secondary (MEDIUM confidence)

- Phase 5 RESEARCH / SUMMARY process inheritance (D-T5 parallel)
- Research-plan seam: exa/context7 providers unavailable in this runtime; digests stored from curated in-repo sources instead

### Tertiary (LOW confidence)

- None material to planning — external CSR-with-payload libraries deliberately not selected

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — no new packages; workspace crates verified
- Architecture: HIGH — dual-lease pattern forced by D-T1–D-T3 and current callers
- Pitfalls: HIGH — dedup/GATE/SoT traps evidenced by existing code + Phase 5

**CBM coverage (this session):** `csr.rs`, `fold.rs`, `quantum.rs`, `materializer.rs`, `leapfrog.rs`, `ff5_legal_pit.rs`, `lib.rs` → `metadata_match` after 2026-09-30 reindex.

**Research date:** 2026-09-30
**Valid until:** 2026-10-30 (stable in-repo spike; re-check if CSR module heavily refactored)
