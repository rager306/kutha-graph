# Phase 6: Typed CSR lease - Pattern Map

**Mapped:** 2026-09-30
**Files analyzed:** 9
**Analogs found:** 9 / 9

CBM session: `list_projects` → `kutha-graph` (`nodes≈17717`, `index_mode: full`, `indexed_at: 2026-09-30T01:27:49Z`). `trace_path` inbound on `from_fold`: 7 callers (must stay on untyped `CsrLease`). Cited analog paths `check_index_coverage` → `no_recorded_issue` / `metadata_match`. All analog paths verified `git ls-files` tracked.

## File Classification

| New/Modified File | Role | Data Flow | Closest Analog | Match Quality |
|-------------------|------|-----------|----------------|---------------|
| `crates/kutha-runtime/src/csr.rs` (+ `TypedEdge`, `TypedCsrLease`) | model | transform | `crates/kutha-runtime/src/csr.rs` (`CsrLease`) | exact |
| `crates/kutha-runtime/src/quantum.rs` (+ `typed_csr_lease_at`) | service | request-response | `crates/kutha-runtime/src/quantum.rs` (`csr_lease_at`) | exact |
| `crates/kutha-runtime/src/lib.rs` (`pub use` typed types) | config | — | `crates/kutha-runtime/src/lib.rs` (`pub use csr::CsrLease`) | exact |
| `crates/kutha-runtime/tests/m011_typed_csr.rs` | test | request-response | `crates/kutha-runtime/tests/m011_quantum_outcome.rs` + `m011_claim_supports.rs` + `ff5_legal_pit.rs` | exact (GATE shape) / role-match (fixture) |
| `.kutha/dictionaries/fsm.yaml` | config | — | `.kutha/dictionaries/fsm.yaml` (`observe_cargo.required` S05 names) | exact |
| `.kutha/dictionaries/checks.yaml` | config | — | `.kutha/dictionaries/checks.yaml` (`m011-quantum-outcome`) | exact |
| `.kutha/dictionaries/bridges.yaml` | config | — | `.kutha/dictionaries/bridges.yaml` (`B-m011-quantum-outcome`) | exact |
| `.kutha/dictionaries/honeycomb.yaml` | config | — | `.kutha/dictionaries/honeycomb.yaml` (`ADR-014` evidence append; `ADR-040`/`ADR-041` rows) | exact |
| `CHANGELOG.md` | config | — | `CHANGELOG.md` (2026-09-29 Product / Process / Trajectory S05) | exact |

**Frozen (do not modify this phase):** `materializer.rs`, `leapfrog.rs`, `CsrLease::from_fold` / `neighbors` / `seek`, fold Assert/Retract/Correct/CorrectInterval, `.kutha/STATE.md`.

## Pattern Assignments

### `crates/kutha-runtime/src/csr.rs` (model, transform)

**Analog:** same file — `CsrLease` (lines 1–51). Field payload from `Fact` in `fold.rs` (62–95).

**Imports pattern** (lines 1–2) — keep; add `EventId` for typed edges:

```rust
use crate::fold::GraphFold;
use kutha_common::{EventId, TermId};
```

**Core untyped builder (must remain byte-compatible)** (lines 12–33):

```rust
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

**Slice accessor pattern for typed `edges_out`** — copy `neighbors` bounds (lines 35–43):

```rust
pub fn neighbors(&self, v: TermId) -> &[TermId] {
    let i = v as usize;
    if i + 1 >= self.offsets.len() {
        return &[];
    }
    let a = self.offsets[i] as usize;
    let b = self.offsets[i + 1] as usize;
    &self.neighbors[a..b]
}
```

**Fact fields for typed projection** (`fold.rs` 62–74, 77–79):

```rust
pub struct Fact {
    pub seq: u64,
    pub subject: TermId,
    pub relation: TermId,
    object: TermId,
    // ...
    pub claim_id: EventId,
}
// object() accessor; is_live_at(tt, vt) — same cut gate as untyped
```

**Typed divergence (D-T1/D-T2 / RESEARCH Q1):** parallel `TypedEdge { relation, object, claim_id, fact_seq }` + `TypedCsrLease { offsets, edges }`; `from_fold` pushes one edge per live Fact; `sort_unstable_by_key(|(relation, object, claim_id, fact_seq)|)`; **never** `dedup`. Doc comment style: `/// Droppable CSR picture … (ADR-041). Not SoT.` (line 4).

**Error handling:** none — pure projection; invalid subjects skipped via `s < buckets.len()` (same as untyped).

---

### `crates/kutha-runtime/src/quantum.rs` (service, request-response)

**Analog:** `Runtime::csr_lease_at` (lines 325–327).

**Imports pattern** (lines 1–8) — extend CSR import:

```rust
use crate::csr::{CsrLease, TypedCsrLease};
```

**Core entry pattern** (lines 325–327) — additive twin:

```rust
pub fn csr_lease_at(&self, tt: u64, vt: u64) -> CsrLease {
    CsrLease::from_fold(&self.fold, tt, vt, self.dict.len())
}

// Add:
pub fn typed_csr_lease_at(&self, tt: u64, vt: u64) -> TypedCsrLease {
    TypedCsrLease::from_fold(&self.fold, tt, vt, self.dict.len())
}
```

**Vertex count:** always `self.dict.len()` so both leases share subject bounds (RESEARCH Pitfall 5).

**Do not change:** `CsrMaterializer`, leapfrog callers, quantum outcome sidecar (D-T3/D-T7).

**Regression unit already present** (lines 735–758) — leave green; CSR-02 thin oracle may cite the same cut style (`csr_lease_at(u64::MAX, 0)` + `neighbors`).

---

### `crates/kutha-runtime/src/lib.rs` (config / barrel)

**Analog:** lines 16–17.

```rust
pub use csr::CsrLease;
pub use fold::{Fact, GraphFold};
```

**Apply:** `pub use csr::{CsrLease, TypedCsrLease, TypedEdge};` (or two `pub use` lines). Do not re-export materializer changes.

---

### `crates/kutha-runtime/tests/m011_typed_csr.rs` (test, request-response)

**Primary analog (GATE file shape):** `crates/kutha-runtime/tests/m011_quantum_outcome.rs` (module doc + two `#[test]` fns whose names are FSM/check needles).

**Fixture analog (supports / Assert):** `crates/kutha-runtime/tests/m011_claim_supports.rs` (lines 1–47).

**CSR assert / cut analog:** `crates/kutha-runtime/tests/ff5_legal_pit.rs` (lines 56–64, 85–87) + unit `csr_drop_rebuild_and_seek` (quantum.rs 735–758).

**Imports pattern** (from quantum_outcome / claim_supports):

```rust
//! M011 S06: typed CSR lease preserves relation labels and support multiplicity.

use kutha_common::Op;
use kutha_runtime::{Runtime, TypedCsrLease}; // TypedEdge as needed
```

**Emit / intern fixture** (claim_supports 8–36 — adapt for two relations + two independent claims):

```rust
let mut rt = Runtime::default();
let s = rt.intern("S");
let o = rt.intern("O");
let knows = rt.intern("knows");
let related = rt.intern("relatedTo");
// Two Asserts claim: None on (s, knows, o) → distinct claim_id (D-T2 multiplicity)
// One Assert (s, relatedTo, o)
// Cut: tt = u64::MAX, vt matching valid_from
```

**CSR-01 core asserts (D-T4):**

```rust
let typed = rt.typed_csr_lease_at(u64::MAX, vt);
let edges = typed.edges_out(s);
assert!(edges.len() >= 3);
// relations include knows + relatedTo; ≥2 edges with relation==knows and distinct claim_id/fact_seq
let untyped = rt.csr_lease_at(u64::MAX, vt);
assert_eq!(untyped.neighbors(s), &[o]); // object-only dedup
```

**CSR-02 thin regression:** rebuild untyped lease, assert neighbors match fold live objects; optional scoped drop like ff3 (lines 85–90) — **do not** re-copy full FF5 statute fixture (RESEARCH Q4). Existing `ff5_as_of_t1_differs_from_as_of_t2_on_statute_log` stays in `observe_cargo.required`.

**Exact fn names (GATE needles):**

- `typed_csr_preserves_relation_labels_and_support_multiplicity`
- `untyped_csr_neighbor_set_and_ff5_still_hold`

---

### `.kutha/dictionaries/fsm.yaml` (config)

**Analog:** `observe_cargo.required` (lines 31–47) — append Phase 5 style names after S05 entries.

```yaml
required:
  # ... existing ...
  - budgets_0_1_2_distinguish_zero_partial_full_after_persist_open
  - crash_after_prefix_has_no_terminal_success_until_explicit_resume
  # Append:
  - typed_csr_preserves_relation_labels_and_support_multiplicity
  - untyped_csr_neighbor_set_and_ff5_still_hold
```

Keep `ff5_as_of_t1_differs_from_as_of_t2_on_statute_log` (CSR-02 observe). Do not remove prior needles.

---

### `.kutha/dictionaries/checks.yaml` (config)

**Analog:** `m011-quantum-outcome` (lines 315–325).

```yaml
  - id: m011-typed-csr
    description: M011 S06 — typed CSR keeps labels/multiplicity; untyped neighbor-set + FF5 path hold
    steps:
      - kind: file_contains
        path: crates/kutha-runtime/tests/m011_typed_csr.rs
        needles:
          - "fn typed_csr_preserves_relation_labels_and_support_multiplicity"
          - "fn untyped_csr_neighbor_set_and_ff5_still_hold"
        require: all
        category: m011-s06
        message: "M011 typed-csr tests missing {missing}"
```

Optional product needles (Phase 4 style) for `TypedCsrLease` / `typed_csr_lease_at` in `csr.rs` / `quantum.rs` if planner wants extra fail-closed — Phase 5 check was tests-only; prefer tests-only unless wave splits need crate evidence.

---

### `.kutha/dictionaries/bridges.yaml` (config)

**Analog:** `B-m011-quantum-outcome` (lines 49–52).

```yaml
  - id: B-m011-typed-csr
    claim: "Typed CSR lease preserves relation labels and support multiplicity; untyped neighbor-set and FF5 path still hold"
    cites: "crates/kutha-runtime/tests/m011_typed_csr.rs"
    check: m011-typed-csr
```

---

### `.kutha/dictionaries/honeycomb.yaml` (config)

**Analog:** `ADR-014` evidence list (lines 131–133) for append shape; **target rows** `ADR-040` / `ADR-041` (lines 190–210) currently `evidence: []`, `map: Proposed`.

```yaml
  - id: ADR-040
    map: Proposed   # unchanged
    evidence:
      - typed_csr_preserves_relation_labels_and_support_multiplicity
      - untyped_csr_neighbor_set_and_ff5_still_hold

  - id: ADR-041
    map: Proposed   # unchanged
    evidence:
      - typed_csr_preserves_relation_labels_and_support_multiplicity
      - untyped_csr_neighbor_set_and_ff5_still_hold
```

Do not flip `map` to Accepted (GATE-03).

---

### `CHANGELOG.md` (config / docs-coupling)

**Analog:** 2026-09-29 Product entry (lines 17–33) — Wave 1 Product crate bullets; Wave 2 Process GATE + Trajectory (RESEARCH Q5).

**Product wave pattern:**

```markdown
## YYYY-MM-DD — Product: typed CSR lease (labels + support multiplicity)

### Product

- `TypedEdge` / `TypedCsrLease` beside untyped `CsrLease`; `Runtime::typed_csr_lease_at` + `edges_out`.
- Named test `typed_csr_preserves_relation_labels_and_support_multiplicity` (CSR-01).
- Named test `untyped_csr_neighbor_set_and_ff5_still_hold` (CSR-02 thin); untyped path unchanged.
```

**Process / Trajectory wave pattern** (copy S05 Process + Trajectory honesty; do **not** edit `.kutha/STATE.md` during delivery — Active Slice already S06):

```markdown
### Process

- FSM `observe_cargo.required` plus `m011-typed-csr` / `B-m011-typed-csr` needles (GATE-01).

### Trajectory

- Active Slice remains **S06**; ADR-040/041 evidence names CSR-01/CSR-02 tests; map stays Proposed (not Accepted, not L_capability). Green governor is not ADR Accepted and not L_capability.
```

## Shared Patterns

### Dual lease from one cut
**Source:** `csr.rs` `CsrLease::from_fold` + `quantum.rs` `csr_lease_at`
**Apply to:** `TypedCsrLease::from_fold`, `typed_csr_lease_at`
- Same `(tt, vt, vertex_count)` / `self.dict.len()`
- Same `Fact::is_live_at` gate
- Independent rebuild; never mutate untyped into typed

### Untyped caller freeze (CBM inbound)
**Source:** `trace_path` on `from_fold` — 7 callers
**Apply to:** do not redirect
- `Runtime::csr_lease_at`, `CsrMaterializer::build`, `leapfrog_common_neighbors_on_csr`
- `csr_drop_rebuild_and_seek`, `csr_materializer_build_unload`
- `ff5_as_of_t1_differs_from_as_of_t2_on_statute_log`, `ff3_csr_lease_drop_does_not_change_log_or_fold`

### GATE-01 trio (Phase 5)
**Source:** `fsm.yaml` + `checks.yaml` `m011-quantum-outcome` + `bridges.yaml` `B-m011-quantum-outcome`
**Apply to:** `m011-typed-csr` / `B-m011-typed-csr` / two CSR fn names
- Exact `fn ` needles must match test identifiers and `observe_cargo.required`
- New check = YAML row only (no `scripts/kutha_gov/checks/*.py`)

### Integration test module shape
**Source:** `tests/m011_quantum_outcome.rs`, `tests/m011_claim_supports.rs`
**Apply to:** `tests/m011_typed_csr.rs`
- Module doc cites M011 slice + ADR
- `Runtime::default()` + `intern` + `Op::Assert { claim: None, … }`
- Named `#[test] fn …` are fitness evidence IDs

### Lease ≠ SoT
**Source:** `csr.rs` line 4 doc; ADR-041 / FF3 drop test
**Apply to:** typed lease — RAM only; no serde/persist; drop must not change log/fold

## No Analog Found

| File | Role | Data Flow | Reason |
|------|------|-----------|--------|
| — | — | — | All planned files have tracked in-repo analogs |

## Metadata

**Analog search scope:** `crates/kutha-runtime/src/{csr,fold,quantum,lib,materializer,leapfrog}.rs`, `crates/kutha-runtime/tests/{ff5_legal_pit,m011_claim_supports,m011_quantum_outcome}.rs`, `.kutha/dictionaries/{fsm,checks,bridges,honeycomb}.yaml`, `CHANGELOG.md`
**Files scanned:** 15+ (CBM graph + Read on analogs)
**CBM tools used:** `list_projects`, `search_graph`, `trace_path`, `get_code_snippet`, `check_index_coverage`
**Pattern extraction date:** 2026-09-30
**Tracked-source gate:** all named analogs passed `git ls-files`
