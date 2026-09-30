# Phase 4: Partial correction with residual intervals - Pattern Map

**Mapped:** 2026-09-29
**Files analyzed:** 9 (8 required + 1 optional honeycomb evidence)
**Analogs found:** 9 / 9 (files). Interval leftover algebra has **no in-repo analog** — copy `Fact` construction from `Op::Correct`, not SQL/Postgres.

CBM project `kutha-graph` is indexed (`list_projects`; generation `2026-09-16T07:18:52Z`). `check_index_coverage` on all cited analog paths: `no_recorded_issue` but freshness **`metadata_changed`** (`recommended_action: read_source_and_reindex`). Line numbers and excerpts below are from **Read of git-tracked source**, not from the stale graph as SoT. Integrator may reindex later; this mapper did not call `index_repository`.

**Do not edit** `.kutha/STATE.md`. Do not mark `.kutha/ROADMAP.md` S04 `[x]` while Active Slice is S04. Do not thaw freeze. Do not add Python check kinds.

## File Classification

| New/Modified File | Role | Data Flow | Closest Analog | Match Quality |
|-------------------|------|-----------|----------------|---------------|
| `crates/kutha-common/src/event.rs` | model | transform (typed log encoding) | self — `Op::Correct` + `Event::new` + `digest_bytes` | exact (evolve enum) |
| `crates/kutha-runtime/src/fold.rs` | service | event-driven (deterministic fold) | self — `Op::Correct` arm + `Fact` push + `is_live_at` | exact for CORR-02 freeze; new arm copies Correct then splits |
| `crates/kutha-runtime/src/quantum.rs` | service | request-response (`emit` admit) | self — `UnknownFact` gate, `RuntimeError` + `Display`, `op_relation`, `derivation_eligible_at` | exact |
| `crates/kutha-runtime/tests/m011_partial_correction.rs` | test | request-response | `crates/kutha-runtime/tests/m011_claim_supports.rs` + `ff5_legal_pit.rs` + `ff6_allowlist.rs` | exact (composite) |
| `.kutha/dictionaries/fsm.yaml` | config | event-driven (observe) | self — `states.observe_cargo.required` | exact |
| `.kutha/dictionaries/checks.yaml` | config | batch (static needles) | `m011-claim-supports` check (new sibling id, do not overload S01–S03 needles) | exact |
| `.kutha/dictionaries/bridges.yaml` | config | batch (cite product) | `B-m011-claim-supports` | exact |
| `CHANGELOG.md` | config (dated history) | batch | `## 2026-09-16 — Product + Process: M011 S03…` | exact structure; **do not** copy “lease S0N-done” |
| `.kutha/dictionaries/honeycomb.yaml` | config (optional) | batch | `ADR-013` `evidence:` list | exact (append names only; `map: Proposed`) |

**Do not create/modify (cite-only / freeze):**

| Path | Role | Note |
|------|------|------|
| `.kutha/STATE.md` | config (harness lease) | Active Slice stays S04; no `L_delivery=M011-S04-done` |
| `Op::Correct` fold arm | service | CORR-02: bit-compatible; do not reinterpret narrower VT as residuals |
| `crates/kutha-runtime/src/store.rs` | service | `defined_names` / Define-strip use `_` / `!Define` — new variant needs no edit |
| `crates/kutha-runtime/src/wal.rs` | service | Keep `VERSION: u32 = 1`; JSON `Event` payload |
| `.kutha/dictionaries/invariants.yaml` | config | Product fence is a **bridge**, not a control-loop invariant |
| `scripts/kutha_gov/**` | utility | YAML row only; no new Python kind |
| `docs/ADR/ADR-013-*.md` | config | Do not flip Status Accepted |
| `crates/kutha-runtime/src/lib.rs` | config | `RuntimeError` already `pub use`; new variant exports automatically |

## Pattern Assignments

### `crates/kutha-common/src/event.rs` (model, transform)

**Analog:** same file — add a sibling of `Op::Correct`, not optional fields on Correct (D-C1 / ADR-013).

**Imports pattern** (lines 1–14): keep `sha2`, `uuid`, existing type aliases. No new crates.

```rust
use sha2::{Digest, Sha256};
use uuid::Uuid;
// EventId, TermId, ValidTime, TransactionTime already in this file
```

**Core pattern — existing Correct payload** (lines 17–39). Copy field style; rename VT fields to `patch_from` / `patch_to`:

```rust
    Correct {
        fact_seq: u64,
        object: TermId,
        valid_from: ValidTime,
        valid_to: Option<ValidTime>,
    },
```

**Recommended new variant** (planner may keep this identifier from RESEARCH):

```rust
    /// Explicit interval patch (M011 S04). Does not change whole-version `Correct`.
    CorrectInterval {
        fact_seq: u64,
        object: TermId,
        patch_from: ValidTime,
        patch_to: Option<ValidTime>,
    },
```

Keep `#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]` on `Op` (line 17). Default externally tagged JSON: old `{"Correct":{...}}` stays readable; new tag is a new object key. Do not add `#[serde(untagged)]`.

**`Event::new` object_ids** (lines 65–81). Bind `CorrectInterval` with Correct (replacement is an interned object, not S/R/O triple):

```rust
            Op::Retract { .. } | Op::Define { .. } => vec![],
            Op::Correct { object, .. } => vec![*object],
```

Become: `Op::Correct { object, .. } | Op::CorrectInterval { object, .. } => vec![*object]`.

**Digest pattern** (lines 118–128). New tag **must not** equal `b"correct"`:

```rust
            Op::Correct {
                fact_seq,
                object,
                valid_from,
                valid_to,
            } => {
                h.update(b"correct");
                h.update(fact_seq.to_le_bytes());
                h.update(object.to_le_bytes());
                h.update(valid_from.to_le_bytes());
                h.update(valid_to.unwrap_or(u64::MAX).to_le_bytes());
            }
```

Copy this arm for `CorrectInterval` with `h.update(b"correct-interval")` and `patch_from` / `patch_to.unwrap_or(u64::MAX)` (same `None` = ∞ encoding as Correct).

**Auth/Guard:** none (product write is interned ids + allowlist at emit).

**Validation:** types only here; empty/inverted/non-intersect reject lives in `Runtime::emit`.

**Testing:** `kutha-common` has no `event` unit tests. Put serde round-trip in the runtime integration test (WAL analog below), not a new crate test file unless compile forces it.

---

### `crates/kutha-runtime/src/fold.rs` (service, event-driven)

**Analog:** same file — **leave the Correct arm verbatim** (CORR-02). New `Op::CorrectInterval` arm copies invalidate + `Fact { claim_id }` then inserts 0–2 residuals + one replacement.

**Imports pattern** (line 1): unchanged.

```rust
use kutha_common::{Event, EventId, Op, TermId, TransactionTime, ValidTime};
```

**Half-open liveness (copy comparisons, do not clip original VT)** (lines 29–42):

```rust
    pub fn is_live_at(&self, tt: TransactionTime, vt: ValidTime) -> bool {
        if self.ingested_at > tt {
            return false;
        }
        if self.invalidated_at.is_some_and(|inv| inv <= tt) {
            return false;
        }
        if vt < self.valid_from {
            return false;
        }
        if self.valid_to.is_some_and(|to| vt >= to) {
            return false;
        }
        true
    }
```

Treat `valid_to` / `patch_to` `None` as `+∞`. Intersection of `[A,B)` and `[C,D)` is `[max(A,C), min(B,D))`, empty iff `from >= to` after substituting ∞. **No analog** for leftover `range_minus` — 15–30 line private helper next to `apply` is expected. Do **not** mutate `old.valid_from` / `old.valid_to`.

**AS OF oracle for tests** (lines 72–75):

```rust
    pub fn as_of(&self, vt: ValidTime) -> Vec<(TermId, TermId, TermId)> {
        self.live_at(u64::MAX, vt)
    }
```

Assert residuals/replacement on `as_of` / `facts()` (`claim_id`, `valid_from`, `object()`). Do **not** use CSR as the residual oracle (`CsrLease` dedups neighbor objects).

**Core Correct arm — DO NOT EDIT** (lines 163–190) — CORR-02 oracle: one replacement, op-supplied VT, same `claim_id`:

```rust
            Op::Correct {
                fact_seq,
                object,
                valid_from,
                valid_to,
            } => {
                if let Some(old) = self.facts.iter_mut().find(|f| f.seq == *fact_seq) {
                    if old.invalidated_at.is_none() {
                        old.invalidated_at = Some(event.ingested_at);
                        let s = old.subject;
                        let r = old.relation;
                        let claim_id = old.claim_id;
                        let seq = self.next_seq;
                        self.next_seq += 1;
                        self.facts.push(Fact {
                            seq,
                            subject: s,
                            relation: r,
                            object: *object,
                            valid_from: *valid_from,
                            valid_to: *valid_to,
                            ingested_at: event.ingested_at,
                            invalidated_at: None,
                            claim_id,
                        });
                    }
                }
            }
```

**New arm (copy then extend):**

1. Find live fact by `fact_seq` (same `iter_mut().find` as Correct).
2. **Compute intersection and leftovers before setting `invalidated_at`.** Empty intersection → no-op (replay-safe if a bad event is already on the log). Do not invalidate then discover empty.
3. Invalidate original row only (`invalidated_at = Some(event.ingested_at)`); **keep original VT bounds**.
4. Copy `subject`, `relation`, `claim_id` like Correct (D-C2: do not mint a new claim).
5. Push order for `fingerprint` (lines 77–90 hashes vec order including losers): **prefix residual (old object), replacement (new object, clipped intersection), suffix residual (old object)**. Skip empty halves.
6. New rows: `ingested_at: event.ingested_at`, `invalidated_at: None`, `object` field is private — construct `Fact { object: … }` in this module like Correct (line 180).

**Fact construction fields** (lines 122–132 Assert / 177–187 Correct) — same struct literal; residuals use `old.object()` for object.

**Error handling:** fold has no `Result`. Missing seq / already invalidated → no-op (same as Correct). Emit-layer reject is the fail-closed write gate.

**Fingerprint:** after interval patch, `facts()` length = original (loser kept) + 1–3 new rows. CORR-02 Correct still yields original + **one** new fact.

---

### `crates/kutha-runtime/src/quantum.rs` (service, request-response)

**Analog:** same file — extend exhaustive `Op` matches and the missing-seq gate; add a Display arm.

**Imports pattern** (lines 1–9): unchanged; `RuntimeError` stays in this module; `lib.rs` already `pub use quantum::{… RuntimeError}`.

**Error type pattern** (lines 11–44). Copy a fact-seq variant; RESEARCH name `IntervalPatchRejected { fact_seq: u64 }` is assumed — planner may pick another identifier if tests/`Display` match:

```rust
    UnknownFact {
        fact_seq: u64,
    },
```

```rust
            RuntimeError::UnknownFact { fact_seq } => write!(f, "unknown fact {fact_seq}"),
            RuntimeError::UnknownClaim { claim } => write!(f, "unknown claim {claim}"),
```

`match self` on `Display` is exhaustive — a new variant **must** get an arm (compile will catch). Keep `impl std::error::Error for RuntimeError {}` (line 54).

**Auth/allowlist — skip relation like Correct** (lines 47–51, 243–258):

```rust
fn op_relation(op: &Op) -> Option<TermId> {
    match op {
        Op::Assert { relation, .. } | Op::Behavior { relation, .. } => Some(*relation),
        Op::Retract { .. } | Op::Correct { .. } | Op::Define { .. } => None,
    }
}
```

Add `| Op::CorrectInterval { .. }` to the `None` arm. `admit` already returns `Ok(())` when `op_relation` is `None` (lines 243–246) — patch does not re-check FF6 (original Assert already did).

**Fail-closed emit (missing seq)** (lines 274–280):

```rust
        if let Op::Retract { fact_seq } | Op::Correct { fact_seq, .. } = &op {
            if !self.fold.facts().iter().any(|f| f.seq == *fact_seq) {
                return Err(RuntimeError::UnknownFact {
                    fact_seq: *fact_seq,
                });
            }
        }
```

Extend the `if let` to `| Op::CorrectInterval { fact_seq, .. }`. **Additionally, interval-patch only** (do not tighten Correct): if fact exists but `invalidated_at` is set, or patch inverted (`patch_to` is `Some(t)` and `patch_from >= t`), or live-fact VT ∩ patch is empty — `return Err(…)` **before** `Event::new` / append. Copy fail-closed tests: capture `log().len()`, assert unchanged.

Do **not** change Correct when the target is already invalidated (today: event still appends; fold no-ops).

**`derivation_eligible_at` premise match** (lines 396–399) — exhaustive; patch is not a claim opener:

```rust
            Op::Retract { .. } | Op::Correct { .. } | Op::Define { .. } => return false,
```

Include `CorrectInterval` in that arm.

**No change:** `follow_ons` `_ => vec![]` (line 348); `graph_len` excludes only `Define` (lines 125–129) — interval patch is a graph op.

**`admit_claim`** (lines 260–271) matches `Op::Assert` only — CorrectInterval falls through `else { Ok(()) }`. Leave it.

---

### `crates/kutha-runtime/tests/m011_partial_correction.rs` (test, request-response)

**Analogs:**

| Concern | File | Why |
|---------|------|-----|
| File crate, intern, `claim_id`, `replay_check`, fail-closed log length | `crates/kutha-runtime/tests/m011_claim_supports.rs` | Same milestone prefix `m011_`; GATE-01 named fns |
| Year-like VT + `as_of` contains/not contains | `crates/kutha-runtime/tests/ff5_legal_pit.rs` | Observation 3 clocks |
| `unwrap_err` + `matches!` + `log().len()` | `crates/kutha-runtime/tests/ff6_allowlist.rs` + `unknown_claim_does_not_append` | D-C7 non-intersect |
| Naming / layout | `.planning/codebase/TESTING.md` | `m011_` file prefix; behavior-named `fn` |

**Imports / file header** (`m011_claim_supports.rs` lines 1–4):

```rust
//! M011 S04: explicit interval patch leaves residual VT versions; whole-version Correct does not.

use kutha_common::Op;
use kutha_runtime::Runtime;
```

Import `RuntimeError` if matching a new variant (`ff6_allowlist.rs` line 5: `use kutha_runtime::{Runtime, RuntimeError};`).

**Arrange pattern** (`m011_claim_supports.rs` lines 8–23): `Runtime::default()`, intern allowlisted relation **`relatedTo`** (not a new dictionary row), subject `"a"`, objects `"P"` then `"P-prime"` (distinct — RESEARCH pitfall 3). Wide VT `valid_from: 2010`, `valid_to: None`. Capture **`seq` and `claim_id` before patch** — do not use `facts()[0]` after intern+patch (Define no-ops fold; patch appends loser + residuals).

**FF5 clock style** (`ff5_legal_pit.rs` lines 7–11, 42–52) — map D-C3:

```rust
const VF_WIDE: u64 = 2010;
const PATCH_FROM: u64 = 2015;
const PATCH_TO: u64 = 2020;
const VT_LEFT: u64 = 2012;
const VT_RIGHT: u64 = 2021;
const VT_INTERIOR: u64 = 2017;
```

CORR-01: `as_of(2012)` and `as_of(2021)` still contain `(a, relatedTo, P)`; interior `as_of(2017)` contains `(a, relatedTo, P-prime)` not `P`; all live rows for that claim share `claim_id`. End with `rt.replay_check().unwrap()` (line 46 of m011 file).

**CORR-02** (`m011_claim_supports.rs` emit + fold queries): `emit(Op::Correct { fact_seq, object: p_prime, valid_from: 2015, valid_to: Some(2020) })`. Then `as_of(2012)` / `as_of(2021)` must **not** contain residual `P`; live facts for that `claim_id` at those VTs: **0**; `as_of(2017)` replacement only; `facts()` = invalidated original + **one** new live fact (not three).

**Fail-closed** (`m011_claim_supports.rs` lines 50–72 and `ff6_allowlist.rs` lines 13–31):

```rust
    let n = rt.log().len();
    let err = rt.emit(/* non-intersect or inverted CorrectInterval */).unwrap_err();
    assert!(matches!(err, RuntimeError::…));
    assert_eq!(n, rt.log().len(), "fail-closed: … must not append");
```

Include touching-endpoint `[2010,2015)` vs patch `[2015,2020)` (half-open: empty intersect). Missing `fact_seq` → existing `UnknownFact`.

**Named functions** (must match FSM + `fn ` needles exactly):

- `interval_patch_leaves_vt_2012_and_2021_residuals` — CORR-01
- `whole_version_correct_does_not_invent_residuals` — CORR-02

Optional third `fn` for non-intersect is fine if also listed in FSM **or** kept as a helper assertion inside CORR-01; GATE-01 requires registered names to exist as `fn `.

**Serde / WAL (support, not S05):** `kutha-common` has no `serde_json` tests. Analog is `wal.rs` JSON payload (lines 21–22 encode, 79–81 decode; unknown variant `Err` → `break`). Prefer `serde_json::to_value` / `from_value` on an `Event` with `CorrectInterval` inside the CORR-01 test. Full `store::persist`/`open` (`quantum.rs` `persist_open_round_trip`, lines 580–601) is nice-to-have, not this slice.

**Anti-pattern:** do not assert CSR neighbors for residuals (`ff5_legal_pit.rs` CSR block is for FF5 statute, not S04).

---

### `.kutha/dictionaries/fsm.yaml` (config, event-driven)

**Analog:** same file `states.observe_cargo.required` (lines 31–43).

**Core pattern — append, do not replace:**

```yaml
    required:
      - ff5_as_of_t1_differs_from_as_of_t2_on_statute_log
      # … existing names …
      - derived_q_loses_eligibility_when_last_premise_support_withdrawn
```

Append the two new identifiers **identical** to `fn` names. `observe-required-fn` prefixes `fn ` and scans `crates/**/*.rs` (`checks.yaml` lines 138–147).

Do not change `args: ["test", "--workspace", "--offline"]` or `kutha-tenant` build.

---

### `.kutha/dictionaries/checks.yaml` (config, batch)

**Analog:** `m011-claim-supports` (lines 245–289) — **new check id**, do not pile S04 needles onto S01–S03.

**Core pattern:**

```yaml
  - id: m011-claim-supports
    description: M011 — independent supports; unknown claim fail-closed; replay lineage; thin P→Q eligibility
    steps:
      - kind: file_contains
        path: crates/kutha-runtime/tests/m011_claim_supports.rs
        needles: ["fn retracting_one_support_leaves_claim_supported"]
        require: all
        category: m011-s01
        message: "M011 claim-support test missing {missing}"
```

Copy for `id: m011-partial-correction` (planner may shorten):

- `file_contains` on `crates/kutha-runtime/tests/m011_partial_correction.rs` with `fn interval_patch_leaves_vt_2012_and_2021_residuals` and `fn whole_version_correct_does_not_invent_residuals`
- Optional product needles: `CorrectInterval` in `event.rs`, `IntervalPatchRejected` (or chosen name) in `quantum.rs`
- `category: m011-s04`
- `kind: file_contains` only (META allowlist). No `scripts/kutha_gov/checks/*.py`

**docs-coupling:** a `checks.yaml` diff must include `invariants.yaml` **or** `bridges.yaml`. Use **bridges**, not invariants.

---

### `.kutha/dictionaries/bridges.yaml` (config, batch)

**Analog:** `B-m011-claim-supports` (lines 39–42):

```yaml
  - id: B-m011-claim-supports
    claim: "Independent supports share claim_id; unknown claim and dangling caused_by fail closed; thin P→Q eligibility tracks premise supports"
    cites: "crates/kutha-runtime/tests/m011_claim_supports.rs"
    check: m011-claim-supports
```

Copy: `id: B-m011-partial-correction`, `cites` the new test file, `check:` must equal the new checks.yaml `id`. Claim sentence: interval patch leaves VT 2012/2021 residuals; whole-version Correct does not invent them.

---

### `CHANGELOG.md` (config, batch)

**Analog:** dated M011 S03 heading (lines 7–20) + kutha-changelog skill (Product / Process / Trajectory; insert below legend).

```markdown
## 2026-09-16 — Product + Process: M011 S03 thin P→Q eligibility

### Product

- `Runtime::derivation_eligible_at`: …
- Named test `derived_q_loses_eligibility_when_last_premise_support_withdrawn`: …

### Process

- Lease `L_delivery=M011-S03-done`. FSM observe + `m011-claim-supports` needles cover S03.

### Trajectory

- ADR-011 / semantic-contract-validation / honeycomb evidence record the thin oracle. Cells remain Proposed; full provenance polynomials and M002 freeze unchanged.
```

**Copy structure, not the lease-done Process bullet.** This phase must **not** claim `L_delivery=M011-S04-done` (STATE is not edited). Process: FSM observe + new check/bridge needles. Trajectory: ADR-013 stays Proposed; optional honeycomb `evidence` only. Same git commit as `crates/**/*.rs` (docs-coupling). English. No SemVer / GitHub Release.

---

### `.kutha/dictionaries/honeycomb.yaml` (config, optional)

**Analog:** ADR-013 cell (lines 108–117):

```yaml
  - id: ADR-013
    axis: Time
    must: "bi-temporal facts; AS OF cuts differ"
    path: docs/ADR/ADR-013-bitemporal-facts-invalidation.md
    map: Proposed
    delivery: spike
    capability: named
    depends_on: [ADR-000, ADR-010, ADR-011]
    locks: [D4]
    evidence: [ff5_as_of_t1_differs_from_as_of_t2_on_statute_log]
```

If edited: append the two test names to `evidence:` only. Do **not** set `map: Accepted`. ADR body edits would also require honeycomb + CHANGELOG; RESEARCH recommends **no** ADR-013 narrative/status change.

## Shared Patterns

### Exhaustive `Op` matches (compile gate)

**Source:** `crates/kutha-common/src/event.rs`, `crates/kutha-runtime/src/fold.rs`, `crates/kutha-runtime/src/quantum.rs`

**Apply to:** every `match &op` / `match &event.op` / `match &cause.op` without `_`.

Sites that **must** gain a `CorrectInterval` arm (Grep `Op::Correct` on 2026-09-29):

| File | Lines | Change |
|------|-------|--------|
| `event.rs` `Event::new` | 80 | Same `object_ids` as Correct |
| `event.rs` `digest_bytes` | 118–128 | New tag `b"correct-interval"` |
| `fold.rs` `apply` | 163–191 | New arm; Correct unchanged |
| `quantum.rs` `op_relation` | 50 | `None` like Correct |
| `quantum.rs` `emit` | 275 | Bind `fact_seq`; extra interval checks |
| `quantum.rs` `derivation_eligible_at` | 399 | `return false` like Correct |

Sites that **need no edit** (wildcard / Define-only): `follow_ons` `_`, `graph_len` / store Define-strip, `defined_names` `_ => None`.

### Fail-closed writes (no append)

**Source:** `unknown_claim_does_not_append` (`m011_claim_supports.rs` 50–72), `ff6_unknown_relation_does_not_append` (`ff6_allowlist.rs` 8–32), `emit` `UnknownFact` (`quantum.rs` 274–280)

**Apply to:** missing seq (`UnknownFact`), non-intersect / inverted / not-live interval patch (new error). Assert error variant **and** `log().len()` unchanged.

### Invalidation without clipping VT

**Source:** Correct / Retract (`fold.rs` 156–189), `retract_keeps_loser` (`quantum.rs` 437–457)

**Apply to:** interval patch original row. Losers remain; old TT still sees old version (`is_live_at` uses `invalidated_at`, not rewritten VT).

### Claim identity on replacement rows

**Source:** Correct copies `let claim_id = old.claim_id` (`fold.rs` 174–186); M011 S01 Assert `claim: Some(claim)` (`m011_claim_supports.rs` 13–25)

**Apply to:** residuals and mid-interval replacement. Same `claim_id`, subject, relation.

### Governor GATE-01 trio

**Source:** `fsm.yaml` required list + `observe-required-fn` + `m011-claim-supports` / `B-m011-claim-supports`

**Apply to:** every leased slice. Identical identifier in `fn name`, FSM list, and `file_contains` needles. `uv run kutha-gov ci` (D-C5); HIGH stops. Do not add invariants.yaml rows for this product fence.

### Changelog + coupling

**Source:** `CHANGELOG.md` M011 entries; `.cursor/skills/kutha-changelog/SKILL.md` planes

**Apply to:** crate + dictionary waves. One commit: crates + dictionaries + CHANGELOG. Do not bump `0.0.0`. Do not edit STATE in the changelog as if the lease closed.

### WAL / serde

**Source:** `wal.rs` 8, 21–22, 79–81 — `VERSION` 1, JSON `Event`, decode `Err` drops record and tail

**Apply to:** do not bump WAL version. Ensure `CorrectInterval` round-trips in-tree so recover does not truncate the log.

## No Analog Found

| File / piece | Role | Data Flow | Reason |
|--------------|------|-----------|--------|
| Half-open leftover helper (`fact_VT − patch`) | utility (private in `fold.rs`) | transform | No `range_minus` / SQL:2011 `FOR PORTION OF` in crates. Copy `Fact` literals from Correct; implement 0–2 leftovers locally. Do not clip the original row. |
| `RuntimeError::IntervalPatchRejected` | model | request-response | Identifier is RESEARCH-assumed. Pattern is `UnknownFact` / `UnknownClaim` + exhaustive `Display`. |
| Semantic-contract dual source `b` / conflict admission | — | — | Out of scope (D-C4 / S08). Do not invent an analog. |

## Metadata

**Analog search scope:** CBM `list_projects` → `search_graph` / `get_code_snippet` / `trace_path` (inbound `GraphFold.apply`) → Read of tracked source; Grep `Op::Correct` and `Op::` matches under `crates/`; git `ls-files` on analog paths (all non-empty / tracked). GitNexus not queried.

**Files scanned:** CBM `kutha-graph` (~3601 nodes); `event.rs`, `fold.rs`, `quantum.rs`, `wal.rs`, `store.rs`, `lib.rs` (both crates), `m011_claim_supports.rs`, `ff5_legal_pit.rs`, `ff6_allowlist.rs`, governor YAML (`fsm`, `checks`, `bridges`, `honeycomb`), `CHANGELOG.md`, `.planning/codebase/TESTING.md`.

**Pattern extraction date:** 2026-09-29

**Coverage caveat:** CBM freshness `metadata_changed` on every cited path. Graph `GraphFold.apply` snippet still matched disk (lines 109–193). Test-file BM25 on `m011*.rs` was empty in-graph; tests were Read from disk.
