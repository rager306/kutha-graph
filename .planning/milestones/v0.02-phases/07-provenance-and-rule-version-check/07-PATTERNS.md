# Phase 7: Provenance and rule-version check - Pattern Map

**Mapped:** 2026-09-30
**Files analyzed:** 11
**Analogs found:** 11 / 11

CBM session: `list_projects` → `kutha-graph` (`nodes=17834`, `index_mode: full`, `indexed_at: 2026-09-30T01:45:29Z`). `trace_path` inbound on `Runtime::replay_check` (include_tests): **27** callers — success contract must stay `Ok` iff fold fingerprint matches **and** every `caused_by` is an earlier id. Cited crate paths `check_index_coverage` → `no_recorded_issue` / `metadata_match`. Dictionary YAML paths → `metadata_changed` (read source; parent reindexes if needed). `search_code` for `Op::Behavior` returned 0 rows (graph text gap); constructors located with Grep. All analog paths verified `git ls-files` tracked.

## File Classification

| New/Modified File | Role | Data Flow | Closest Analog | Match Quality |
|-------------------|------|-----------|----------------|---------------|
| `crates/kutha-common/src/event.rs` (`rule_version` on `Op::Behavior`; `digest_bytes`) | model | transform | same file — `Assert.claim` `#[serde(default)]` + exhaustive `digest_bytes` Behavior arm | exact |
| `crates/kutha-runtime/src/quantum.rs` (`provenance_fingerprint` / `provenance_check` / `ProvenanceMismatch`; `follow_ons` ctor) | service | request-response | same file — `replay_check` + `ReplayDivergence` + `follow_ons` inverse_knows | exact |
| `crates/kutha-runtime/tests/m011_provenance.rs` | test | request-response | `tests/m011_typed_csr.rs` (GATE file) + `tests/m011_claim_supports.rs` (Behavior emit + `from_dict_and_events`) | exact (GATE) / role-match (oracle) |
| `crates/kutha-runtime/tests/m011_claim_supports.rs` (Behavior field add) | test | request-response | same file — two exhaustive `Op::Behavior { … }` sites | exact |
| `.kutha/dictionaries/fsm.yaml` | config | — | same file `observe_cargo.required` S06 names | exact |
| `.kutha/dictionaries/checks.yaml` | config | — | `m011-typed-csr` (tests + product-symbol steps) | exact |
| `.kutha/dictionaries/bridges.yaml` | config | — | `B-m011-typed-csr` | exact |
| `.kutha/dictionaries/honeycomb.yaml` | config | — | `ADR-060` / `ADR-011` evidence append (same rows as Phase 6 did for ADR-040/041) | exact |
| `CHANGELOG.md` | config | — | 2026-09-30 Product typed-CSR + Process GATE / Trajectory | exact |
| `crates/kutha-runtime/src/fold.rs` | service | transform | same file — Behavior `..` + `fingerprint` Facts-only | exact (verify-only) |
| `crates/kutha-runtime/src/lib.rs` | config | — | `pub use quantum::{… RuntimeError}` already | exact (verify-only) |

**Frozen (do not modify this phase):** Assert/Retract/Correct/CorrectInterval fold arms; typed/untyped CSR; quantum outcomes sidecar; `replay_check` BrokenLineage contract; `.kutha/STATE.md`; ROADMAP S07 checkbox; honeycomb `map:` (stay Proposed).

## Pattern Assignments

### `crates/kutha-common/src/event.rs` (model, transform)

**Analog:** same file — `Op::Assert.claim` serde default (lines 22–29) and exhaustive `Op::Behavior` in `digest_bytes` (lines 149–166). `Event::new` already uses `..` on Behavior (lines 72–95) — **keep**; it will compile after the field add.

**Imports pattern** (lines 1–2) — unchanged:

```rust
use sha2::{Digest, Sha256};
use uuid::Uuid;
```

**Serde default for a new optional-at-decode field** (lines 22–29) — copy onto `rule_version`:

```rust
    Assert {
        // ...
        #[serde(default)]
        claim: Option<EventId>,
    },
```

**Apply (D-P2):**

```rust
    Behavior {
        name: String,
        caused_by: EventId,
        #[serde(default)]
        rule_version: String,
        subject: TermId,
        relation: TermId,
        object: TermId,
        valid_from: ValidTime,
        valid_to: Option<ValidTime>,
    },
```

`String` default is `""` (legacy rows). Do not put the pin only on outcomes/CSR.

**Core digest mix** (lines 149–166) — exhaustive match **must** name `rule_version`:

```rust
            Op::Behavior {
                name,
                caused_by,
                subject,
                relation,
                object,
                valid_from,
                valid_to,
            } => {
                h.update(b"behavior");
                h.update(name.as_bytes());
                h.update(caused_by.as_bytes());
                h.update(subject.to_le_bytes());
                // ...
            }
```

**Apply (RESEARCH Q4 / A1):** after `caused_by.as_bytes()`, `h.update(rule_version.as_bytes())` with **no** extra domain tag. Empty `""` adds zero bytes so existing Behavior receipts stay identical.

**Do not `Event::new` the swapped Behavior in oracles** (lines 88–89): `id: Uuid::now_v7()` — a new id moves `Fact.claim_id` (`fold.rs` 204) and the **state** fingerprint. Clone `Event` and patch `op` only (`Event` is `Clone`, line 63).

**WAL/JSONL consumers (no code change if serde default is present):** `store::persist` `serde_json::to_writer` (store.rs 20–22); `wal::append_events` `to_vec` (wal.rs 21); `open` / `recover_events` deserialize `Event` (store.rs 48, wal.rs 79).

---

### `crates/kutha-runtime/src/fold.rs` (service, transform) — **verify-only**

**Analog:** `GraphFold::apply` Behavior arm (lines 186–207) and `fingerprint` (lines 129–143).

**Keep Behavior `..` (D-P7)** — lineage (`name`, `caused_by`, future `rule_version`) must not enter Facts:

```rust
            Op::Behavior {
                subject,
                relation,
                object,
                valid_from,
                valid_to,
                ..
            } => {
                // ...
                    claim_id: event.id,
```

**State fingerprint is Facts-only** — do **not** mix `name` / `caused_by` / `rule_version` here (ADR-060 obligation 1 stays this hash):

```rust
    pub fn fingerprint(&self) -> [u8; 32] {
        let mut h = Sha256::new();
        for f in &self.facts {
            h.update(f.seq.to_le_bytes());
            // ... subject/relation/object/VT/TT/claim_id ...
        }
        h.finalize().into()
    }
```

Hasher setup (`Sha256::new` / `update` / `finalize().into()`) is the analog for `provenance_fingerprint` in `quantum.rs`, but the **payload** must be Behavior lineage, not Fact rows.

---

### `crates/kutha-runtime/src/quantum.rs` (service, request-response)

**Analog:** `RuntimeError::ReplayDivergence` + `Display` (lines 12–16, 38–40); `replay_check` (lines 490–509); `from_dict_and_events` (lines 267–300); `follow_ons` inverse_knows ctor (lines 472–481).

**Imports pattern** (top of file) — add `Digest, Sha256` if not already imported for fingerprint:

```rust
use sha2::{Digest, Sha256};
```

(`fold.rs` already uses this; `replay_check` does not hash itself.)

**Error variant pattern** (lines 12–16) — additive twin; do **not** replace ReplayDivergence:

```rust
    ReplayDivergence {
        expected: [u8; 32],
        actual: [u8; 32],
    },
```

**Apply:**

```rust
    ProvenanceMismatch {
        expected: [u8; 32],
        actual: [u8; 32],
    },
```

**Display exhaustive match** (lines 38–56) — new arm required (rustc E0004). Mirror ReplayDivergence:

```rust
            RuntimeError::ReplayDivergence { .. } => write!(f, "ReplayDivergenceError"),
```

**Apply:** `RuntimeError::ProvenanceMismatch { .. } => write!(f, "ProvenanceMismatchError"),`  
Tests should `matches!(err, RuntimeError::ProvenanceMismatch { .. })`, not the Display string (RESEARCH A3).

**State replay — do not merge provenance (D-P3).** Copy structure beside it, never inside it:

```rust
    pub fn replay_check(&self) -> Result<GraphFold, RuntimeError> {
        let rebuilt = GraphFold::replay(self.log.as_slice());
        let expected = self.fold.fingerprint();
        let actual = rebuilt.fingerprint();
        if expected != actual {
            return Err(RuntimeError::ReplayDivergence { expected, actual });
        }
        let mut seen = HashSet::new();
        for e in self.log.iter() {
            if let Op::Behavior { caused_by, .. } = &e.op {
                if !seen.contains(caused_by) {
                    return Err(RuntimeError::BrokenLineage {
                        caused_by: *caused_by,
                    });
                }
            }
            seen.insert(e.id);
        }
        Ok(rebuilt)
    }
```

`Op::Behavior { caused_by, .. }` already ignores extra fields — **keep**. Ghost prior still `BrokenLineage`.

**Provenance API (locked mix — RESEARCH Q2):** hash **Behavior rows only**, `EventLog` order (`log.rs` 22–24 `iter` = vec order):

```rust
    pub fn provenance_fingerprint(&self) -> [u8; 32] {
        let mut h = Sha256::new();
        h.update(b"kutha-prov-v1");
        for e in self.log.iter() {
            if let Op::Behavior {
                name,
                caused_by,
                rule_version,
                ..
            } = &e.op
            {
                h.update(e.id.as_bytes());
                h.update(caused_by.as_bytes());
                h.update((name.len() as u64).to_le_bytes());
                h.update(name.as_bytes());
                h.update((rule_version.len() as u64).to_le_bytes());
                h.update(rule_version.as_bytes());
            }
        }
        h.finalize().into()
    }

    pub fn provenance_check(&self, expected: [u8; 32]) -> Result<(), RuntimeError> {
        let actual = self.provenance_fingerprint();
        if expected != actual {
            return Err(RuntimeError::ProvenanceMismatch { expected, actual });
        }
        Ok(())
    }
```

Domain tag `kutha-prov-v1` so the 32-byte array cannot be confused with `GraphFold::fingerprint`. Length-prefix strings (Pitfall 4). Do **not** mix Fact triples.

**Fixture constructor** (lines 267–282) — oracles rebuild via clone + this API; dict **must** contain `"knows"` and `"knownBy"`:

```rust
    pub fn from_dict_and_events(
        dict_strings: Vec<String>,
        all_events: Vec<Event>,
        max_cascade: usize,
    ) -> Result<Self, RuntimeError> {
        let dict = TermDictionary::from_strings(dict_strings);
        let knows = dict.id("knows").ok_or_else(|| RuntimeError::UnknownRelation {
            name: "knows".into(),
        })?;
        let known_by = dict.id("knownBy").ok_or_else(|| RuntimeError::UnknownRelation {
            name: "knownBy".into(),
        })?;
        // apply each event; EventLog::from_events; outcomes: Vec::new()
```

Accessors for clone-rebuild: `log()` (173–175), `fold()` (186–188), `dictionary()` (190–192), `max_cascade` field (123). `dictionary().strings().to_vec()` (intern.rs 34–36).

**Exhaustive Behavior ctor in cascade** (lines 472–481) — **must** add `rule_version: String::new()`:

```rust
                vec![Event::new(
                    Op::Behavior {
                        name: "inverse_knows".into(),
                        caused_by: event.id,
                        subject: *object,
                        relation: self.known_by,
                        object: *subject,
                        valid_from: *valid_from,
                        valid_to: *valid_to,
                    },
                    event.ingested_at,
                )]
```

Empty pin is **not** a PROV-02 fixture. Matches that already use `..` (`op_relation` 90–92, `derivation_eligible_at` 522, 530) keep compiling.

**Inbound freeze (`trace_path` include_tests, hop 1):** do not change `replay_check` `Ok` meaning. Callers include `ff5_legal_pit`, `ff6_allowlist`, H2/H4 tenant tests, M010 open tests, `m011_claim_supports` (ghost + P→Q), `m011_partial_correction`, quantum unit tests (`replay_detects_tamper` 639–657), `kutha-tenant` `main`. Adding `ProvenanceMismatch` is additive; existing `matches!(err, RuntimeError::…)` stay valid.

**Regression unit analog** (lines 639–657) — state tamper still `ReplayDivergence`; do not expect `ProvenanceMismatch` here:

```rust
        rt.tamper_fold();
        let err = rt.replay_check().unwrap_err();
        assert!(matches!(err, RuntimeError::ReplayDivergence { .. }));
```

**Do not change:** `derivation_eligible_at` (515–537), outcomes sidecar, `typed_csr_lease_at` / `csr_lease_at`.

---

### `crates/kutha-runtime/src/lib.rs` (config / barrel) — **verify-only**

**Analog:** lines 21–24.

```rust
pub use quantum::{
    disposition, OutcomeDisposition, PersistedQuantumOutcome, QuantumOutcome, Runtime,
    RuntimeError,
};
```

`RuntimeError` is already public; new variant needs **no** extra `pub use`. Do not re-export a provenance type unless planner introduces a new struct (not recommended).

---

### `crates/kutha-runtime/tests/m011_provenance.rs` (test, request-response)

**Primary analog (GATE file shape):** `crates/kutha-runtime/tests/m011_typed_csr.rs` (module doc + two `#[test]` fns whose names are FSM/check needles).

**Fixture analog (ghost + Behavior emit):** `crates/kutha-runtime/tests/m011_claim_supports.rs` — `replay_rejects_behavior_without_prior_cause` (75–103) for `from_dict_and_events`; `derived_q_loses_eligibility_when_last_premise_support_withdrawn` (107–188) for `emit(Op::Behavior { name: "derive_pq", … })`.

**Imports / module doc** (typed_csr 1–4, claim_supports 1–3):

```rust
//! M011 S07: provenance digest moves when caused_by or rule_version changes; state fingerprint does not.

use kutha_common::{Event, EventId, Op};
use kutha_runtime::Runtime;
```

**Ghost regression stays in claim_supports — do not reimplement as PROV-01** (75–103):

```rust
    let ghost = EventId::nil();
    let events = vec![Event::new(
        Op::Behavior {
            name: "inverse_knows".into(),
            caused_by: ghost,
            // + rule_version: String::new() after field add
            subject: 2,
            relation: 1,
            object: 0,
            valid_from: 0,
            valid_to: None,
        },
        0,
    )];
    let rt = Runtime::from_dict_and_events(dict, events, 32).unwrap();
    let err = rt.replay_check().unwrap_err();
    assert!(
        matches!(err, kutha_runtime::RuntimeError::BrokenLineage { caused_by } if caused_by == ghost),
        "{err:?}"
    );
```

That proves exists-earlier only. PROV-01 needs **two recorded priors** and both `replay_check` `Ok`.

**P→Q emit analog** (claim_supports 147–156) — after field add, every exhaustive ctor needs `rule_version`:

```rust
    let derived = rt
        .emit(Op::Behavior {
            name: "derive_pq".into(),
            caused_by: claim_p,
            subject: q,
            relation: rel,
            object: true_,
            valid_from: 2010,
            valid_to: None,
        })
        .unwrap();
```

**Clone-then-rebuild (D-P1)** — do **not** `Event::new` the swapped Behavior:

```rust
    let dict = rt.dictionary().strings().to_vec();
    let events = rt.log().as_slice().to_vec();
    let mut swapped = events.clone();
    // patch only Op::Behavior.caused_by or rule_version; keep id, ingested_at, triple
    let rt2 = Runtime::from_dict_and_events(dict, swapped, rt.max_cascade).unwrap();
    assert_eq!(rt.fold().fingerprint(), rt2.fold().fingerprint());
    assert_ne!(rt.provenance_fingerprint(), rt2.provenance_fingerprint());
    rt.replay_check().unwrap();
    rt2.replay_check().unwrap();
```

Baseline construction: `Runtime::default()` + intern (typed_csr 8–12) + two `Op::Assert { claim: None, … }` so two distinct event ids exist as valid priors, then `emit` Behavior with `caused_by = A1`, `name: "derive_pq"`, `rule_version: "r1"`, fixed Q triple. Dict from a live runtime already includes `"knows"` / `"knownBy"` (Default intern).

**Exact fn names (GATE needles — D-P4; do not shorten):**

- `provenance_detects_caused_by_swap_when_state_fingerprint_matches`
- `provenance_detects_rule_version_change_when_state_fingerprint_matches`

PROV-02: same baseline clone; change **only** `rule_version` `"r1"` → `"r2"`; same id / triple / `caused_by`.

Optional (not a GATE observe name — RESEARCH Q6): `behavior_without_rule_version_field_deserializes` via `serde_json::from_str` on JSON missing `rule_version` → `""`. No in-repo Event serde unit test exists; WAL path is the production analog (`store.rs` 48).

---

### `crates/kutha-runtime/tests/m011_claim_supports.rs` (test, request-response)

**Analog:** same file — exhaustive `Op::Behavior` at lines 86–96 and 147–156.

**Apply:** add `rule_version: String::new()` (or `"r1"` only if a test starts asserting provenance — **do not**; these tests stay ghost + eligibility regressions). Keep `replay_rejects_behavior_without_prior_cause` in `observe_cargo.required` (already listed fsm.yaml 42).

---

### `.kutha/dictionaries/fsm.yaml` (config)

**Analog:** `observe_cargo.required` (lines 31–49) — append after S06 names. Keep ghost + P→Q names.

```yaml
    required:
      # ... existing including:
      - replay_rejects_behavior_without_prior_cause
      - derived_q_loses_eligibility_when_last_premise_support_withdrawn
      - typed_csr_preserves_relation_labels_and_support_multiplicity
      - untyped_csr_neighbor_set_and_ff5_still_hold
      # Append:
      - provenance_detects_caused_by_swap_when_state_fingerprint_matches
      - provenance_detects_rule_version_change_when_state_fingerprint_matches
```

Wave 2 only (after Wave 1 fns exist) — otherwise `observe-required-fn` HIGH. Do not remove prior needles. Do not change `observe_cargo` args.

---

### `.kutha/dictionaries/checks.yaml` (config)

**Analog:** live `m011-typed-csr` (lines 327–349) — tests-file needles **plus** product-symbol `file_contains` steps.

**Tests-only core (always):**

```yaml
  - id: m011-provenance
    description: M011 S07 — provenance digest detects caused_by swap and rule_version change while state fingerprint matches
    steps:
      - kind: file_contains
        path: crates/kutha-runtime/tests/m011_provenance.rs
        needles:
          - "fn provenance_detects_caused_by_swap_when_state_fingerprint_matches"
          - "fn provenance_detects_rule_version_change_when_state_fingerprint_matches"
        require: all
        category: m011-s07
        message: "M011 provenance tests missing {missing}"
```

**Copy Phase 6 extra product steps** (typed-csr 338–349 style):

```yaml
      - kind: file_contains
        path: crates/kutha-common/src/event.rs
        needles: ["rule_version"]
        require: all
        category: m011-s07
        message: "rule_version missing: {missing}"
      - kind: file_contains
        path: crates/kutha-runtime/src/quantum.rs
        needles: ["provenance_fingerprint"]
        require: all
        category: m011-s07
        message: "provenance_fingerprint missing: {missing}"
```

Do not pile needles onto `m011-typed-csr` / `m011-claim-supports`. No `scripts/kutha_gov/checks/*.py`.

---

### `.kutha/dictionaries/bridges.yaml` (config)

**Analog:** `B-m011-typed-csr` (lines 54–57).

```yaml
  - id: B-m011-provenance
    claim: "Provenance digest detects caused_by swap and rule_version change while state fold fingerprint still matches; replay_check stays Ok on valid priors"
    cites: "crates/kutha-runtime/tests/m011_provenance.rs"
    check: m011-provenance
```

`check:` must equal the new check id. Do not add `invariants.yaml` (product fence is a bridge).

---

### `.kutha/dictionaries/honeycomb.yaml` (config)

**Analog:** Phase 6 append to **existing** cell evidence while leaving `map: Proposed`. Target rows are **ADR-060** (lines 273–282) and **ADR-011** (lines 86–95), not new cells.

**ADR-060 today:**

```yaml
    map: Proposed
    evidence: [replay_rejects_behavior_without_prior_cause]
```

**ADR-011 today:** inline list already includes `replay_rejects_behavior_without_prior_cause` and `derived_q_loses_eligibility_when_last_premise_support_withdrawn`.

**Apply (RESEARCH Q3):** append both oracle fn names to **both** evidence lists; keep `map: Proposed`. Do not edit `docs/ADR/ADR-060-strict-replay.md` / `ADR-011-*.md` status. Do not treat this as Accepted.

Preserve each row’s YAML style (inline `[…]` vs block list) when appending.

---

### `CHANGELOG.md` (config / docs-coupling)

**Analog:** 2026-09-30 Product typed-CSR (lines 17–32) — Wave 1 Product crate bullets; Wave 2 Process GATE + Trajectory (RESEARCH Q5). Newest dated Process lease for S07 already exists (lines 7–15) — **do not** rewrite that lease sentence as “S07 delivered”.

**Product wave pattern:**

```markdown
## YYYY-MM-DD — Product: provenance check apart from state replay

### Product

- `Op::Behavior.rule_version` (`#[serde(default)]`); fold still ignores lineage; `Runtime::provenance_fingerprint` / `provenance_check` beside unchanged `replay_check`.
- Named test `provenance_detects_caused_by_swap_when_state_fingerprint_matches` (PROV-01).
- Named test `provenance_detects_rule_version_change_when_state_fingerprint_matches` (PROV-02).
```

**Process / Trajectory wave pattern** (copy S06 Process GATE honesty; do **not** edit `.kutha/STATE.md` — Active Slice already S07):

```markdown
### Process

- FSM `observe_cargo.required` plus `m011-provenance` / `B-m011-provenance` needles (GATE-01).

### Trajectory

- Active Slice remains **S07**; ADR-060 and ADR-011 evidence names PROV-01/PROV-02 tests; map stays Proposed (not Accepted, not L_capability). Green governor is not ADR Accepted and not L_capability. `replay_check` Ok is not execution replay (obligation 3 unimplemented).
```

Do not bump `0.0.0`. Follow `.cursor/skills/kutha-changelog/SKILL.md` at execute time.

## Shared Patterns

### Dual verification from one log
**Source:** `quantum.rs` `replay_check` (490–509) + `fold.rs` `fingerprint` (129–143)
**Apply to:** `provenance_fingerprint` / `provenance_check`
- Same `EventLog`; independent hashes
- Never fold lineage into `GraphFold::fingerprint`
- Never skip `BrokenLineage`
- Never call provenance from inside `replay_check`

### Clone events, then `from_dict_and_events`
**Source:** `m011_claim_supports.rs` 75–97 + `quantum.rs` 267–300
**Apply to:** both PROV oracles
- Stable `Event.id` so Behavior `Fact.claim_id` does not move
- Dict must intern `"knows"` / `"knownBy"`
- Pass `rt.max_cascade`; do not invent mutate-in-place SoT API

### Exhaustive `Op::Behavior` sites after field add
**Source:** Grep `Op::Behavior` (graph `search_code` gap)
**Apply to:** compile-fix list
| Site | Pattern | Action |
|------|---------|--------|
| `event.rs` `digest_bytes` 149–166 | named fields | add `rule_version` + mix bytes |
| `quantum.rs` `follow_ons` 473–481 | named fields | `rule_version: String::new()` |
| `m011_claim_supports.rs` 86–96, 147–156 | named fields | `rule_version: String::new()` |
| `event.rs` `Event::new` 80–85 | `..` | none |
| `fold.rs` apply 186–192 | `..` | none |
| `quantum.rs` `replay_check` 499, `derivation_eligible_at` 522/530, `op_relation` 92 | `..` | none |

### GATE-01 trio (Phase 6 live)
**Source:** `fsm.yaml` 31–49 + `checks.yaml` `m011-typed-csr` 327–349 + `bridges.yaml` `B-m011-typed-csr` 54–57
**Apply to:** `m011-provenance` / `B-m011-provenance` / two PROV fn names
- Exact `fn ` needles = test identifiers = `observe_cargo.required`
- New check = YAML row only
- Wave 1 crates/tests; Wave 2 YAML + honeycomb + Process changelog

### Integration test module shape
**Source:** `tests/m011_typed_csr.rs`, `tests/m011_claim_supports.rs`
**Apply to:** `tests/m011_provenance.rs`
- Module doc cites M011 S07 + ADR-060 obligation 2
- Named `#[test] fn …` are fitness evidence IDs
- `replay_check().unwrap()` on **both** logs after valid-prior swap

### Serde forever-compat
**Source:** `event.rs` 28–29 `#[serde(default)]` on `claim`; WAL/JSONL `serde_json` Event codec
**Apply to:** `rule_version: String`
- Old files without the field must `open`
- Empty string = legacy pin, not PROV-02 oracle

### Receipt mix vs provenance mix
**Source:** `Event::digest_bytes` (97–173) vs recommended `kutha-prov-v1`
**Apply to:** do not conflate
- Receipts: no extra tag on `rule_version` bytes (A1)
- Provenance: domain tag + length-prefixed name/version
- Neither is a MAC (ADR-014 receipts stay separate)

## No Analog Found

| File | Role | Data Flow | Reason |
|------|------|-----------|--------|
| — | — | — | All planned files have tracked in-repo analogs. `provenance_fingerprint` is new but copies `replay_check` / `fingerprint` hasher + error-pair shape. Optional serde JSON fixture has no dedicated Event unit test; use `store.rs` deserialize path as the analog. |

## Metadata

**Analog search scope:** `crates/kutha-common/src/{event,intern}.rs`, `crates/kutha-runtime/src/{quantum,fold,lib,log,wal,store}.rs`, `crates/kutha-runtime/tests/{m011_claim_supports,m011_typed_csr,m011_quantum_outcome}.rs`, `.kutha/dictionaries/{fsm,checks,bridges,honeycomb}.yaml`, `CHANGELOG.md`
**Files scanned:** 16+ (CBM graph + Read on analogs; Grep for `Op::Behavior` after `search_code` gap)
**CBM tools used:** `list_projects`, `search_graph`, `trace_path`, `get_code_snippet`, `check_index_coverage`, `search_code` (0 hits on `Op::Behavior`)
**Pattern extraction date:** 2026-09-30
**Tracked-source gate:** all named analogs passed `git ls-files`
**Coverage caveat:** dictionary YAML `freshness: metadata_changed` — excerpts taken from live files, not graph nodes.
