# Phase 8: End-to-end candidate fixture - Pattern Map

**Mapped:** 2026-09-30
**Files analyzed:** 11
**Analogs found:** 11 / 11

CBM session: `list_projects` → `kutha-graph` (`nodes=17834`, `index_mode: full`, `indexed_at: 2026-09-30T01:45:29Z`). `trace_path` inbound `Runtime::attach_outcomes` (`include_tests`): **9** callers — `store::open` hop 1 plus M010 / S05 / H2 / `persist_open_round_trip`. Inbound `store::persist`: **12** including `kutha-tenant` `main`. `get_code_snippet` for `derivation_eligible_at` returned **stale** `provenance_fingerprint` body (graph lines 515–537 vs source 556–578). Discrete excerpts below are from **Read**. `check_index_coverage`: `store.rs` / `fold.rs` / `csr.rs` / `lib.rs` / `snapshot.rs` / `m011_quantum_outcome.rs` / `m011_typed_csr.rs` / `m010_semantic_open.rs` → `metadata_match`; `quantum.rs` / `event.rs` / `m011_claim_supports.rs` / dictionary YAML → `metadata_changed`; `m011_provenance.rs` → `not_tracked`. All analog paths verified `git ls-files` tracked.

## File Classification

| New/Modified File | Role | Data Flow | Closest Analog | Match Quality |
|-------------------|------|-----------|----------------|---------------|
| `crates/kutha-runtime/src/store.rs` (`JUSTIFICATIONS_REL` persist/open) | service | file-I/O | same file — `OUTCOMES_REL` / `write_outcomes` / `load_outcomes` / four `attach_outcomes` returns | exact |
| `crates/kutha-runtime/src/quantum.rs` (`Justification` buffer, `record_justification`, `check_admission`, `conflict_report_at`, `AdmissionDenied`) | service | event-driven + request-response | same file — `PersistedQuantumOutcome` + `record_resume` + `DuplicateResume` + `derivation_eligible_at` + `live_supports` via fold | exact (buffer) / role-match (admission + conflict) |
| `crates/kutha-runtime/src/lib.rs` | config | request-response | same file — `pub use quantum::{… PersistedQuantumOutcome, RuntimeError}` | exact |
| `crates/kutha-runtime/tests/m011_e2e_fixture.rs` | test | file-I/O + request-response | glue: `m011_quantum_outcome.rs` (persist/open/snapshot discard) + `m011_claim_supports.rs` (two supports + P→Q) + `m011_partial_correction.rs` (CorrectInterval residuals) + `m011_typed_csr.rs` (CSR drop-rebuild) + `m011_provenance.rs` (`rule_version: "r1"`) + `m010_semantic_open.rs` (open without snapshot) | exact (GATE shape) / role-match (narrative) |
| `.kutha/dictionaries/fsm.yaml` | config | batch | same file `observe_cargo.required` S07 names (lines 50–51) | exact |
| `.kutha/dictionaries/checks.yaml` | config | batch | `m011-provenance` (lines 351–373) | exact |
| `.kutha/dictionaries/bridges.yaml` | config | batch | `B-m011-provenance` (lines 59–62) | exact |
| `.kutha/dictionaries/honeycomb.yaml` | config | batch | Phase 7 append to existing cells; **target** ADR-013 / ADR-011 / ADR-012 / ADR-040 | exact |
| `CHANGELOG.md` | config | batch | 2026-09-30 Product provenance + Process GATE / Trajectory | exact |
| `crates/kutha-runtime/src/fold.rs` | service | transform | same file — `live_supports` / CorrectInterval `claim_id` copy / `replay` | exact (verify-only; D-F7) |
| `crates/kutha-runtime/src/snapshot.rs` | model | file-I/O | same file — no sidecar fields | exact (verify-only) |

**Frozen (do not modify this phase):** Assert/Retract/Correct/CorrectInterval fold arms; `derivation_eligible_at` mix; quantum outcome disposition / auto-push on `emit`; typed/untyped CSR `from_fold`; `replay_check` / `provenance_fingerprint` mix; `.kutha/STATE.md`; ROADMAP S08 checkbox; honeycomb `map:` (stay Proposed).

## Pattern Assignments

### `crates/kutha-runtime/src/store.rs` (service, file-I/O)

**Analog:** same file — D-O1 outcomes sidecar (Phase 5). Clone; do **not** invent a second persist pipeline.

**Imports / consts** (lines 1–11) — add `Justification` beside `PersistedQuantumOutcome`:

```rust
use crate::quantum::{cascade_limit, PersistedQuantumOutcome, Runtime, RuntimeError};
// ...
pub const OUTCOMES_REL: &str = "quantum_outcomes.jsonl";
```

**Apply:**

```rust
pub const JUSTIFICATIONS_REL: &str = "justifications.jsonl";
```

Keep `OUTCOMES_REL` public (S05 tests import `store::OUTCOMES_REL`). New tests import `store::JUSTIFICATIONS_REL`.

**Persist order** (lines 15–30) — events + WAL first, then terms, snapshot, outcomes. **Then** justifications (RESEARCH Pitfall 6). Do not write cites before the log:

```rust
    write_terms(dir, runtime.dictionary().strings())?;
    let snap = runtime.snapshot();
    let mut sf = File::create(dir.join("snapshot.json"))?;
    serde_json::to_writer(&mut sf, &snap).map_err(json_err)?;
    // Outcomes after events so mid-crash cannot orphan a Full row (RESEARCH pitfall 3).
    write_outcomes(dir, runtime.outcome_records())?;
    Ok(())
```

**Apply:** after `write_outcomes`, `write_justifications(dir, runtime.justification_records())?;` then `Ok(())`. Comment: justifications last so a crash cannot leave cites without matching events; missing file on open = empty vec (cannot admit).

**JSONL write twin** (lines 124–131):

```rust
fn write_outcomes(dir: &Path, rows: &[PersistedQuantumOutcome]) -> std::io::Result<()> {
    let mut f = File::create(dir.join(OUTCOMES_REL))?;
    for row in rows {
        serde_json::to_writer(&mut f, row).map_err(json_err)?;
        f.write_all(b"\n")?;
    }
    Ok(())
}
```

Copy as `write_justifications` / `JUSTIFICATIONS_REL` / `&[Justification]`. Do **not** add `serde-jsonlines`.

**JSONL load twin** (lines 133–147) — missing file = empty, never invent rows:

```rust
fn load_outcomes(path: &Path) -> std::io::Result<Vec<PersistedQuantumOutcome>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let f = File::open(path)?;
    let mut out = Vec::new();
    for line in BufReader::new(f).lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        out.push(serde_json::from_str(&line).map_err(json_err)?);
    }
    Ok(out)
}
```

**Open: load once, attach on every return** (lines 54–83). Today `let outcomes = load_outcomes(...)` then **four** `rt.attach_outcomes(outcomes)` sites (snapshot early-return, Define dict, terms file, empty-events default). Adding `attach_justifications` beside **each** `attach_outcomes` keeps M010 / H2 / `kutha-tenant` compiling and empty-safe.

```rust
    let outcomes = load_outcomes(&dir.join(OUTCOMES_REL))?;
    if snap_path.exists() {
        // ...
        let mut rt = Runtime::from_snapshot(snap, graph);
        rt.attach_outcomes(outcomes);
        return Ok(rt);
    }
```

**Apply:** `let justifications = load_justifications(&dir.join(JUSTIFICATIONS_REL))?;` then `rt.attach_justifications(justifications);` on all four success paths. Old store dirs without the file must `open` with empty cites (same as missing `quantum_outcomes.jsonl`).

**Do not** add fields to `Snapshot` (see snapshot.rs below). Error mapping stays `json_err` → `InvalidData`.

**Inbound freeze (`trace_path` persist):** S05 tests, M010 open tests, H2 tenant, `kutha-tenant` `main`, `persist_open_round_trip`. New file after persist is additive.

---

### `crates/kutha-runtime/src/quantum.rs` (service, event-driven + request-response)

**Analog:** outcomes RAM buffer + explicit `record_resume` (not inferred on `open`). Conflict report analog is `GraphFold::live_supports`. Admission analog is `admit_claim` / `DuplicateResume` fail-closed.

**Imports** (lines 1–10) — already has `EventId`, `Op`, serde via outcomes. No new crates.

**Serde row analog** (lines 75–85) — copy field discipline for `Justification` (RESEARCH locked names):

```rust
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct PersistedQuantumOutcome {
    pub quantum_id: String,
    pub disposition: OutcomeDisposition,
    pub aborted_on_budget: bool,
    pub events_in_quantum: usize,
    pub event_ids: Vec<EventId>,
    pub receipt_digest_hex: String,
    pub resume_of: Option<String>,
}
```

**Apply (D-F2):**

```rust
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Justification {
    pub justification_id: String,
    pub target_claim: EventId,
    pub source_claim_ids: Vec<EventId>,
    pub source_event_ids: Vec<EventId>,
    pub source_fact_seqs: Vec<u64>,
    pub rule_version: String,
    pub tt: u64,
    pub vt: u64,
}
```

Do **not** put summary text on a fold Fact. Do **not** mix this row into `provenance_fingerprint` (D-F7).

**Conflict report analog** — `fold.rs` `live_supports` + `Fact::object()` (fold 76–79, 146–151). Put the method on `Runtime` (RESEARCH) so tests stay on the public crate surface:

```rust
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConflictReport {
    pub positive_supports: Vec<u64>,
    pub negative_supports: Vec<u64>,
}

impl Runtime {
    pub fn conflict_report_at(
        &self,
        claim: EventId,
        tt: u64,
        vt: u64,
        positive: TermId,
        negative: TermId,
    ) -> ConflictReport {
        let mut positive_supports = Vec::new();
        let mut negative_supports = Vec::new();
        for f in self.fold.live_supports(claim, tt, vt) {
            if f.object() == positive {
                positive_supports.push(f.seq);
            } else if f.object() == negative {
                negative_supports.push(f.seq);
            }
        }
        ConflictReport {
            positive_supports,
            negative_supports,
        }
    }
}
```

Report only. Both non-empty = conflict. No winner enum (D-F3).

**Runtime struct buffer** (lines 124–135) — pair `justifications: Vec<Justification>` next to `outcomes`:

```rust
    /// In-memory buffer of durable quantum outcomes (SoT is `quantum_outcomes.jsonl`).
    outcomes: Vec<PersistedQuantumOutcome>,
```

**Constructors that today set `outcomes: Vec::new()`** — **must** also set `justifications: Vec::new()` (Pitfall 4 / E0063):

| Constructor | Lines |
|-------------|-------|
| `Runtime::new` | 148–158 |
| `from_snapshot` | 259–269 |
| `from_dict_and_events` | 295–305 |
| `fork_at` | 317–327 |

`fork_at` dropping justifications (like outcomes) is acceptable (RESEARCH A2) — document in the attach/record API comment.

**Accessor / attach analog** (lines 200–208):

```rust
    pub fn outcome_records(&self) -> &[PersistedQuantumOutcome] {
        &self.outcomes
    }

    pub fn attach_outcomes(&mut self, rows: Vec<PersistedQuantumOutcome>) {
        self.outcomes = rows;
    }
```

**Apply:** `justification_records` / `attach_justifications`. Open never infers cites from the log.

**Explicit record analog** (lines 210–230) — `record_justification` is like `record_resume`: caller-driven, `open` never calls it. Do **not** auto-push justifications from `emit` (emit **does** auto-push outcomes at lines 443–451 — that is **not** the justification pattern).

```rust
    pub fn record_resume(&mut self, quantum_id: &str) -> Result<(), RuntimeError> {
        if self
            .outcomes
            .iter()
            .any(|r| r.resume_of.as_deref() == Some(quantum_id))
        {
            return Err(RuntimeError::DuplicateResume {
                resume_of: quantum_id.to_string(),
            });
        }
        self.outcomes.push(PersistedQuantumOutcome { /* ... */ });
        Ok(())
    }
```

**Apply:** `record_justification(...)` appends a `Justification` (mint `justification_id` with a stable string, e.g. uuid-like or `"j:{target}:{tt}:{vt}"` — planner discretion). Return the id. Thin anti-duplicate on id is optional; FIX-02 cares about **stale replay**, not duplicate ids.

**Fail-closed error analog** (lines 13–39, 42–61) — additive `AdmissionDenied`; Display arm required (E0004). Tests `matches!(err, RuntimeError::AdmissionDenied { reason: "stale_support", .. })`, not Display:

```rust
    DuplicateResume {
        resume_of: String,
    },
```

**Apply:**

```rust
    AdmissionDenied {
        justification_id: String,
        reason: &'static str,
    },
```

Display: `RuntimeError::AdmissionDenied { .. } => write!(f, "AdmissionDeniedError"),`  
Locked `reason` strings (RESEARCH): `"unknown_justification"`, `"stale_support"`, `"ineligible"`, `"rule_version"`.

**`check_admission` analog mix:** `admit_claim` fail-closed (363–374) + `Fact::is_live_at` (fold 81–95) + `derivation_eligible_at` (556–578) + live Behavior `rule_version` from the log.

Liveness of cited supports is **`source_fact_seqs`**, not `claim_id` alone (Pitfall 2). After CorrectInterval, `claim_supported_at` stays true while the t1 fact_seq is invalidated.

```rust
    pub fn check_admission(&self, justification_id: &str) -> Result<(), RuntimeError> {
        let row = self
            .justifications
            .iter()
            .find(|j| j.justification_id == justification_id)
            .ok_or(RuntimeError::AdmissionDenied {
                justification_id: justification_id.into(),
                reason: "unknown_justification",
            })?;
        for seq in &row.source_fact_seqs {
            let live = self
                .fold
                .facts()
                .iter()
                .any(|f| f.seq == *seq && f.is_live_at(row.tt, row.vt));
            if !live {
                return Err(RuntimeError::AdmissionDenied {
                    justification_id: row.justification_id.clone(),
                    reason: "stale_support",
                });
            }
        }
        if !self.derivation_eligible_at(row.target_claim, row.tt, row.vt) {
            return Err(RuntimeError::AdmissionDenied {
                justification_id: row.justification_id.clone(),
                reason: "ineligible",
            });
        }
        let pin = match self.log.iter().find(|e| e.id == row.target_claim) {
            Some(e) => match &e.op {
                Op::Behavior { rule_version, .. } => rule_version.as_str(),
                _ => "",
            },
            None => "",
        };
        if pin != row.rule_version.as_str() {
            return Err(RuntimeError::AdmissionDenied {
                justification_id: row.justification_id.clone(),
                reason: "rule_version",
            });
        }
        Ok(())
    }
```

Polarity is FIX-01’s job (`conflict_report_at`). After t2, t1 rows fail on **`stale_support`**. Do not require `derivation_eligible_at == false` at t3 if a `not-P` residue still holds `claim_p` (Pattern 4 / D-F7).

**Do not change `derivation_eligible_at`** (source lines 552–578 — CBM snippet was stale). Eligibility is **claim-liveness**, not object polarity:

```rust
    pub fn derivation_eligible_at(&self, derived: EventId, tt: u64, vt: u64) -> bool {
        if !self.fold.claim_supported_at(derived, tt, vt) {
            return false;
        }
        // ... premise = Assert.claim.unwrap_or(cause.id) or Behavior id ...
        self.fold.claim_supported_at(premise, tt, vt)
    }
```

**CSR leases unchanged** (331–337) — FIX-03 **calls** them:

```rust
    pub fn csr_lease_at(&self, tt: u64, vt: u64) -> CsrLease {
        CsrLease::from_fold(&self.fold, tt, vt, self.dict.len())
    }

    pub fn typed_csr_lease_at(&self, tt: u64, vt: u64) -> TypedCsrLease {
        TypedCsrLease::from_fold(&self.fold, tt, vt, self.dict.len())
    }
```

**Emit outcomes auto-record** (443–451) — **do not** copy this for justifications. Cites are policy records, not quantum progress.

---

### `crates/kutha-runtime/src/fold.rs` (service, transform) — **verify-only**

**Analog:** support queries (145–159), CorrectInterval `claim_id` copy (243–290), `replay` (296–301). D-F7: do **not** change apply.

**Support glue for FIX-01/03:**

```rust
    pub fn live_supports(&self, claim: EventId, tt: TransactionTime, vt: ValidTime) -> Vec<&Fact> {
        self.facts
            .iter()
            .filter(|f| f.claim_id == claim && f.is_live_at(tt, vt))
            .collect()
    }

    pub fn live_support_count(&self, claim: EventId, tt: TransactionTime, vt: ValidTime) -> usize {
        self.live_supports(claim, tt, vt).len()
    }

    pub fn claim_supported_at(&self, claim: EventId, tt: TransactionTime, vt: ValidTime) -> bool {
        self.live_support_count(claim, tt, vt) > 0
    }
```

Values at a cut: `live_at` (116–122) / `as_of` (124–127). Completeness for FIX-03: `fingerprint` (129–143) plus equal support counts / `conflict_report_at` — **not** CSR neighbor identity as SoT.

**CorrectInterval copies `claim_id` onto prefix / replacement / suffix** (265–281) — feature for residuals; trap if eligibility is mistaken for polarity:

```rust
                let claim_id = old.claim_id;
                // ...
                self.facts[idx].invalidated_at = Some(event.ingested_at);
                let mut push_row = |obj: TermId, vf: ValidTime, vt: Option<ValidTime>| {
                    // ...
                        claim_id,
```

After t2 with object `not-P`, `claim_supported_at(claim_p, t2, 2017)` is **true**. Observation 2 uses **empty live Facts with `object == P`**, not `derivation_eligible_at == false`.

**Replay reconstruct** (296–301):

```rust
    pub fn replay(events: &[Event]) -> Self {
        let mut fold = Self::default();
        for e in events {
            fold.apply(e);
        }
        fold
    }
```

FIX-03: `GraphFold::replay(rt.log().as_slice()).fingerprint() == rt.fold().fingerprint()`.

`Fact::object()` is the polarity accessor (object field is private). `facts()` is public (106–108) for fact_seq liveness in `check_admission`.

---

### `crates/kutha-runtime/src/snapshot.rs` (model) — **verify-only**

**Analog:** lines 6–14. Sidecars must **not** join this struct (D-F2 / FIX-03 discard snapshot):

```rust
pub struct Snapshot {
    pub log_offset: usize,
    pub next_tt: u64,
    pub max_cascade: usize,
    pub fold: GraphFold,
    pub dict_strings: Vec<String>,
    pub knows: TermId,
    pub known_by: TermId,
}
```

---

### `crates/kutha-runtime/src/lib.rs` (config / barrel)

**Analog:** lines 21–24.

```rust
pub use quantum::{
    disposition, OutcomeDisposition, PersistedQuantumOutcome, QuantumOutcome, Runtime,
    RuntimeError,
};
```

**Apply:** add `ConflictReport`, `Justification` so integration tests use `kutha_runtime::{Justification, ConflictReport, store, Runtime}` — not `kutha_runtime::quantum::`. `Fact` is already re-exported from `fold`.

---

### `crates/kutha-runtime/tests/m011_e2e_fixture.rs` (test, file-I/O + request-response)

**Primary analog (GATE file shape):** `tests/m011_typed_csr.rs` / `m011_quantum_outcome.rs` — module doc + named `#[test]` fns whose names are FSM/check needles.

**Glue analogs (do not copy whole bodies — D-F1):**

| Narrative piece | Analog | Lines |
|-----------------|--------|-------|
| Two supports, shared `claim_id` | `m011_claim_supports.rs` | 13–38, 115–145 |
| P→Q Behavior + eligibility at t1 | `m011_claim_supports.rs` | 147–167 |
| `receipt.event_ids[0]` as claim id | `m011_claim_supports.rs` | 23–25, 159 |
| Year-like VT + CorrectInterval residuals | `m011_partial_correction.rs` | 8–13, 44–80 |
| `rule_version: "r1"` | `m011_provenance.rs` | 40–50 |
| persist → delete `snapshot.json` → `open` | `m010_semantic_open.rs` 40–44; `m011_quantum_outcome.rs` 108–121 | |
| Missing sidecar = empty | `m011_quantum_outcome.rs` | 55–64 |
| CSR drop/rebuild | `m011_typed_csr.rs` | 105–132 |
| `uuid_like` temp dirs | `m011_quantum_outcome.rs` | 127–135 |

**Imports / module doc** (quantum_outcome 1–4, claim_supports 1–4):

```rust
//! M011 S08: one candidate fixture — supports/conflict, stale admission, incremental vs reconstruct.

use kutha_common::Op;
use kutha_runtime::{store, Runtime};
```

Add `ConflictReport` / `Justification` / `RuntimeError` as re-exported.

**Shared builder (D-F1)** — intern `a`, `b`, `P`, `Q`, `relatedTo`, `true`, `not-P`. Relation **`relatedTo`**, not `knows` (Pitfall 7 — inverse cascade). Subjects are sources `a`/`b`; object is interned `P` (S04 shape), not subject=`P` object=`true` (S03 shape). Planner may keep S03 `true` intern unused.

t1: Assert `a` `claim: None`; Assert `b` `claim: Some(claim_p)`; Behavior `derive_pq` `caused_by: claim_p` `rule_version: "r1"` `subject: Q`. Store `fact_seq` of a and b from fold after each emit (RESEARCH A1: cross-check receipt vs `facts().last()`).

t2 conflict branch (FIX-01 **must** run this): `CorrectInterval` on **a**’s fact_seq, `object: not_p`, `patch_from=2015` `patch_to=Some(2020)` — copy constants from S04 (VF_WIDE=2010, VT_LEFT=2012, VT_INTERIOR=2017, VT_RIGHT=2021).

t3: `Retract` **b**’s fact_seq only (observation 3 residuals are of `a`).

**P→Q emit analog** (claim_supports 147–158) with `"r1"`:

```rust
    let derived = rt
        .emit(Op::Behavior {
            name: "derive_pq".into(),
            caused_by: claim_p,
            rule_version: "r1".into(),
            subject: q,
            relation: rel,
            object: true_, // or interned P-as-object narrative: object = p_term
            valid_from: 2010,
            valid_to: None,
        })
        .unwrap();
```

Semantic-contract narrative uses object `P` on `a`/`b` (partial_correction 23–29). Builder should match S04 triples `(a, relatedTo, P)` / `(b, relatedTo, P)` plus Behavior asserting `Q`.

**Exact fn names (GATE needles — D-F6; do not shorten after Wave 1 ships):**

- `e2e_fixture_supports_and_conflict_at_named_cuts`
- `e2e_justification_cites_sources_and_rejects_stale_admission`
- `e2e_incremental_matches_reconstruct_after_discarding_leases`

**FIX-01 oracle:** at (t1, 2017) both supports live, `derivation_eligible_at` true, conflict report positive-only. After t2: `b` still supports `P`; `a` replacement is `not-P`; report both sides; residuals at 2012/2021 keep `P` on `a`. After t3: `positive_supports` empty; Q Fact still live (`claim_supported_at(claim_q)`); do **not** assert `derivation_eligible_at == false` if `not-P` residue holds `claim_p`. `replay_check().unwrap()` at end.

**FIX-02 oracle** (clone disposition_after_persist_open 108–121 + crash missing-file 55–64):

```rust
    store::persist(&rt, &dir).unwrap();
    std::fs::remove_file(dir.join("snapshot.json")).unwrap();
    let opened = store::open(&dir).unwrap();
    opened.check_admission(&jid).unwrap();
```

After t2, original jid → `stale_support`. Mint a new row after reevaluation. After t3, old rows still deny (cannot renew). Do not `Event::new` to “refresh” a cite (new UUID).

**FIX-03 oracle:** compare incremental Runtime vs `GraphFold::replay(log)` vs persist → `remove_file(snapshot.json)` → `open` vs drop both CSR leases and rebuild. Assert fold `fingerprint` / `live_at` / support counts / `conflict_report_at` / `derivation_eligible_at` at named cuts **and** justification rows survive snapshot delete. CSR: compare `neighbors` / `edges_out` pre/post drop as lease identity, **not** as SoT.

**Temp dir helper** — copy `uuid_like` (m011_quantum_outcome 127–135); prefix `kutha-m011-s08-`.

Prior `m011_*` files stay regression. Do **not** re-run them as the S08 GATE observe set.

---

### `.kutha/dictionaries/fsm.yaml` (config, batch)

**Analog:** `observe_cargo.required` (lines 31–51) — append after S07 names. Do not remove provenance pair.

```yaml
      - provenance_detects_caused_by_swap_when_state_fingerprint_matches
      - provenance_detects_rule_version_change_when_state_fingerprint_matches
      # Append (Wave 2, after Wave 1 fns exist):
      - e2e_fixture_supports_and_conflict_at_named_cuts
      - e2e_justification_cites_sources_and_rejects_stale_admission
      - e2e_incremental_matches_reconstruct_after_discarding_leases
```

Do not change `observe_cargo` args. Wave 2 only — otherwise `observe-required-fn` HIGH.

---

### `.kutha/dictionaries/checks.yaml` (config, batch)

**Analog:** live `m011-provenance` (lines 351–373) — tests-file needles **plus** product-symbol steps.

```yaml
  - id: m011-e2e
    description: M011 S08 — candidate fixture distinguishes history, evidence, and stale admission at named cuts
    steps:
      - kind: file_contains
        path: crates/kutha-runtime/tests/m011_e2e_fixture.rs
        needles:
          - "fn e2e_fixture_supports_and_conflict_at_named_cuts"
          - "fn e2e_justification_cites_sources_and_rejects_stale_admission"
          - "fn e2e_incremental_matches_reconstruct_after_discarding_leases"
        require: all
        category: m011-s08
        message: "M011 e2e fixture tests missing {missing}"
      - kind: file_contains
        path: crates/kutha-runtime/src/store.rs
        needles: ["JUSTIFICATIONS_REL"]
        require: all
        category: m011-s08
        message: "JUSTIFICATIONS_REL missing: {missing}"
      - kind: file_contains
        path: crates/kutha-runtime/src/quantum.rs
        needles: ["conflict_report_at", "check_admission"]
        require: all
        category: m011-s08
        message: "e2e APIs missing: {missing}"
```

Do not pile needles onto `m011-provenance`. No `scripts/kutha_gov/checks/*.py`.

---

### `.kutha/dictionaries/bridges.yaml` (config, batch)

**Analog:** `B-m011-provenance` (lines 59–62).

```yaml
  - id: B-m011-e2e
    claim: "Candidate fixture: independent supports and conflict report at named cuts; justifications cannot renew stale admission; incremental matches reconstruct after discarding snapshot and CSR"
    cites: "crates/kutha-runtime/tests/m011_e2e_fixture.rs"
    check: m011-e2e
```

`check:` must equal the new check id. Do not add `invariants.yaml`.

---

### `.kutha/dictionaries/honeycomb.yaml` (config, batch)

**Analog:** Phase 7 append to **existing** cells; keep `map: Proposed`. RESEARCH Q4:

| Cell | Lines today | Append |
|------|-------------|--------|
| ADR-013 | 108–120 (block list) | FIX-01 fn + conflict report evidence |
| ADR-011 | 86–95 (inline `[…]`) | FIX-01 / P→Q glue fn |
| ADR-012 | 97–106 `evidence: []` | FIX-03 snapshot-discard fn |
| ADR-040 | 190–201 (block list) | FIX-03 CSR drop-rebuild fn |

Preserve each row’s YAML style (inline vs block). Do not edit `docs/ADR/*.md` status. Do not set `map: Accepted`.

Suggested evidence names = the three GATE fn identifiers (same strings as fsm).

---

### `CHANGELOG.md` (config / docs-coupling)

**Analog:** 2026-09-30 Product provenance (lines 17–34) — Wave 1 Product crate bullets; Wave 2 Process GATE + Trajectory (RESEARCH Q5). Newest dated Process lease for S08 already exists (lines 7–15) — **do not** rewrite that lease sentence as “S08 delivered”.

**Product wave pattern:**

```markdown
## YYYY-MM-DD — Product: end-to-end candidate fixture

### Product

### Added

- Durable `justifications.jsonl` sidecar (`JUSTIFICATIONS_REL`) beside outcomes; `record_justification` / `check_admission` fail-closed on stale fact_seqs / ineligible / rule_version mismatch.
- `conflict_report_at` reports `(positive_supports, negative_supports)` without picking a winner.
- Named tests `e2e_fixture_supports_and_conflict_at_named_cuts`, `e2e_justification_cites_sources_and_rejects_stale_admission`, `e2e_incremental_matches_reconstruct_after_discarding_leases`.
```

**Process / Trajectory wave pattern** (do **not** edit `.kutha/STATE.md`):

```markdown
### Process

- FSM `observe_cargo.required` plus `m011-e2e` / `B-m011-e2e` needles (GATE-01).

### Trajectory

- Active Slice remains **S08**; ADR-013/011/012/040 evidence names FIX oracles; map stays Proposed (not Accepted, not L_capability). Green governor is not ADR Accepted and not L_capability. Conflict report is not four-valued logic.
```

Do not bump `0.0.0`. Follow `.cursor/skills/kutha-changelog/SKILL.md` at execute time. Crate diffs → `### Product`; dictionaries → `### Process`; honeycomb evidence → `### Trajectory`.

## Shared Patterns

### JSONL sidecar (authoritative, not a lease)
**Source:** `store.rs` `OUTCOMES_REL` (10–11, 28–29, 54, 62, 69, 76, 81, 124–147)
**Apply to:** `JUSTIFICATIONS_REL`
- `File::create` rewrite + `serde_json::to_writer` + `\n`
- Missing file ⇒ `Vec::new()`, never invent admission
- Written **after** events (and after outcomes)
- Loaded on **all four** `open` returns
- Distinct from droppable `snapshot.json` / CSR

### Explicit RAM buffer, never inferred
**Source:** `quantum.rs` `outcomes` field + `record_resume` (133–134, 210–230); `open` never auto-resumes
**Apply to:** `justifications` + `record_justification`
- Four constructors init empty
- `emit` must **not** append justification rows (unlike outcomes at 443–451)
- `fork_at` starts empty (A2)

### Fail-closed identity
**Source:** `admit_claim` / `UnknownClaim` (363–374); `DuplicateResume` (37–39, 211–219); missing outcomes file (m011_quantum_outcome 55–64)
**Apply to:** `check_admission`
- Unknown id → deny
- Dead `source_fact_seqs` at the **row’s** `(tt, vt)` → `stale_support`
- `rule_version` ≠ live Behavior pin → deny
- Missing sidecar after open → cannot admit

### Glue over reimplementation (D-F1 / D-F7)
**Source:** S03–S07 named tests
**Apply to:** `m011_e2e_fixture.rs`
- Call `claim_supported_at`, `derivation_eligible_at`, `CorrectInterval`, CSR leases, `provenance_fingerprint` / Behavior pin, persist/open
- Do not change fold/eligibility/CSR/outcome disposition
- Do not require prior `m011_*` fns as S08 observe needles

### Polarity vs eligibility (Pattern 4)
**Source:** `derivation_eligible_at` 556–578 + CorrectInterval `claim_id` 265–281
**Apply to:** FIX-01/03 oracles and FIX-02 stale arm
- Eligibility = premise **claim** still has a live Fact
- Conflict / “last positive support gone” = `conflict_report_at` / live Facts with `object == P`
- After interval-patch to `not-P`, do not rewrite S03 to make eligibility false

### GATE-01 trio (Phase 7 live)
**Source:** `fsm.yaml` 31–51 + `checks.yaml` `m011-provenance` 351–373 + `bridges.yaml` `B-m011-provenance` 59–62
**Apply to:** `m011-e2e` / `B-m011-e2e` / three FIX fn names
- Exact `fn ` needles = test identifiers = `observe_cargo.required`
- New check = YAML row only
- Wave 1 crates/tests + Product changelog; Wave 2 YAML + honeycomb + Process/Trajectory changelog

### Integration test module shape
**Source:** `tests/m011_quantum_outcome.rs`, `m011_claim_supports.rs`, `m011_partial_correction.rs`
**Apply to:** `tests/m011_e2e_fixture.rs`
- Module doc cites M011 S08 + semantic-contract observations
- Named `#[test] fn …` are fitness evidence IDs
- `relatedTo` narrative; `uuid_like` temp dirs; `replay_check().unwrap()`

### Snapshot / CSR are leases
**Source:** `snapshot.rs` 4–14; `csr.rs` `from_fold` 12–33 / 73–97; `m010_semantic_open.rs` 40–44; `m011_typed_csr.rs` 105–132
**Apply to:** FIX-02 durability + FIX-03 reconstruct
- Delete `snapshot.json`; cites and fold answers remain
- Drop typed + untyped leases; rebuild; fold answers unchanged

## No Analog Found

| File | Role | Data Flow | Reason |
|------|------|-----------|--------|
| — | — | — | All planned files have tracked in-repo analogs. `Justification` / `check_admission` / `conflict_report_at` are new symbols but copy outcomes JSONL + `record_resume` fail-closed + `live_supports` partition. No prior ABAC/admission pack — do not invent one. |

## Metadata

**Analog search scope:** `crates/kutha-runtime/src/{store,quantum,fold,csr,lib,snapshot}.rs`, `crates/kutha-runtime/tests/{m011_claim_supports,m011_partial_correction,m011_quantum_outcome,m011_typed_csr,m011_provenance,m010_semantic_open}.rs`, `.kutha/dictionaries/{fsm,checks,bridges,honeycomb}.yaml`, `CHANGELOG.md`; CBM `list_projects` / `search_graph` / `trace_path` / `get_code_snippet` / `check_index_coverage`
**Files scanned:** 18+ tracked sources
**CBM tools used:** `list_projects`, `index_status`, `search_graph`, `trace_path`, `get_code_snippet`, `check_index_coverage`
**Pattern extraction date:** 2026-09-30
**Tracked-source gate:** all named analogs passed `git ls-files`
**Coverage caveat:** `quantum.rs` / dictionaries / `m011_claim_supports.rs` `metadata_changed`; `m011_provenance.rs` `not_tracked`; `derivation_eligible_at` graph snippet stale — excerpts from live Read (eligibility 556–578, `record_resume` 210–230, persist 15–30). Parent reindexes only if asked.
