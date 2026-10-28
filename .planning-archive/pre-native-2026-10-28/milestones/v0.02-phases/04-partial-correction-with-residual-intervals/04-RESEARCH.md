# Phase 4: Partial correction with residual intervals - Research

**Researched:** 2026-09-29
**Domain:** Kutha event-log SoT / bi-temporal fold (interval-patch residuals)
**Confidence:** HIGH (in-repo contract and call sites); MEDIUM (serde JSON shape from official docs); LOW (SQL:2011 leftover analog — wiki/search only)

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

- **D-C1:** Add a **new `Op` variant** for explicit interval patch (working name `CorrectInterval`; exact Rust identifier is planner/researcher discretion). Payload must name the target fact (`fact_seq`) and the **patch VT range** (`patch_from` / `patch_to`, half-open) plus the replacement `object` for that range. **Do not** overload or reinterpret today's `Op::Correct` fold path so that a narrower `valid_from`/`valid_to` silently invents residuals. Whole-version `Correct` remains: invalidate the whole live fact and emit one replacement version (CORR-02). — **Reversibility:** costly — once events with the new variant are logged/WAL'd, removing the variant needs a migration or forever-compat decode

- **D-C2:** After an interval patch on a live fact spanning a wider VT, fold must leave **residual Fact rows** for the uncovered prefixes/suffixes (e.g. patch `[2015,2020)` inside `[2010,∞)` → residuals `[2010,2015)` and `[2020,∞)`). Residuals keep the **same `claim_id`** (and subject/relation) as the corrected fact; they continue the same support lineage (M011 S01). The mid-interval replacement is a new Fact at the same `claim_id` with the patch VT and new object. Do not mint a new claim for residuals. — **Reversibility:** reversible — fold semantics; claim_id rule matches existing Correct

- **D-C3:** Ship a **minimal residual oracle** only: arrange one support/fact for source `a` over a wide VT, apply explicit interval patch covering `[2015,2020)` (or equivalent fixture constants), then assert at one TT cut that `as_of`/live cuts at VT **2012** and **2021** still see residual versions of `a`, and that the patched interior matches the replacement. **Do not** implement the full semantic-contract t1/t2/t3 narrative, dual-source `b`, conflict `P'=not-P`, summary/action records, or incremental-vs-reconstruct parity in this phase (those are S08 / FIX-*). Reuse existing claim/support and FF5-style cut helpers where they fit. — **Reversibility:** reversible — test scope only

- **D-C4:** S04 records the replacement object on the patched interval via the interval-patch op only. **Admission policy, conflict reporting, and “allowed action”** remain out of scope — deferred to **Phase 8 / S08** (FIX-01). No new ABAC or admission dictionary in S04. — **Reversibility:** reversible — scope fence

- **D-C5:** Inherit Phase 2 **D-G1…D-G3** and **D-10 / D-11 / D-15** for every execute wave: `uv run kutha-gov ci` required; HIGH stops; WARN ledgered; SUMMARY trajectory block; cargo at tracer + pre-verify, and on any wave that touches `crates/`. — **Reversibility:** reversible — process gate

- **D-C6:** Named cargo test (CORR-01 residual oracle; CORR-02 regression that whole-version Correct still invents no residuals) is registered in the governor (FSM observe + check needle) so GATE-01 holds and `ci` stays 0 HIGH. — **Reversibility:** reversible

- **D-C7:** Intervals stay **half-open** `[from,to)` as in ADR-013; infinity is `valid_to: None` as today. Reject or no-op an interval patch that does not actually intersect the live fact's VT (exact reject vs no-op is planner discretion; must be fail-closed and tested). — **Reversibility:** reversible

### Claude's Discretion

User said «реши эти вопросы разумно» for gray areas 1–4 → D-C1…D-C4 (plus D-C5…D-C7 companions). Planner may refine Rust field names and exact fixture constants; must not change Correct whole-version semantics, thaw freeze, or expand into S05–S08 delivery.

### Deferred Ideas (OUT OF SCOPE)

- Full semantic-contract observations 1–2, 4–5 and conflict admission → Phase 8 / S08
- Persisted quantum outcome / crash resume → Phase 5 / S05
- Typed CSR labels and support multiplicity → Phase 6 / S06
- Provenance/rule-version check without execution replay → Phase 7 / S07
- Thin ADR-050 dictionaries, Rocks, Cypher, legal pack → later harness milestones
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| CORR-01 | Developer can apply an explicit interval-patch correction and observe residual versions on both sides of the corrected VT range at the same TT cut (fixture: source `a` keeps VT 2012 and 2021) | New `Op` variant (D-C1) + fold split that invalidates the whole targeted `Fact` then pushes prefix/mid/suffix rows sharing `claim_id` (D-C2). Named integration test using `as_of(2012)` / `as_of(2021)` plus interior replacement. |
| CORR-02 | Developer can still use whole-version `Correct` and observe unchanged behavior, with no implicit residuals | Do not edit the existing `Op::Correct` fold arm. Add a regression test that `Correct` with a narrower `valid_from`/`valid_to` leaves **no** live residual at 2012/2021. |
| GATE-01 | Each slice ships a named cargo test registered in the governor (FSM observe + check needle), and `uv run kutha-gov ci` stays at 0 HIGH | Append `fsm.yaml` `observe_cargo.required`, a `checks.yaml` `file_contains` check, and a `bridges.yaml` row (same pattern as `m011-claim-supports`). Same git commit as `CHANGELOG.md` (`docs-coupling`). Do not edit `.kutha/STATE.md`. |
</phase_requirements>

## Summary

Phase 4 is M011 **S04** (Active Slice leased): add an **explicit interval-patch write** that leaves residual valid-time versions on both sides of the patched range, while **whole-version `Op::Correct` stays bit-compatible**. The semantic-contract note’s observation 3 is the oracle (VT 2012 and 2021 keep source `a`); observations 1–2 and 4–5, dual source `b`, and admission stay S08.

Today `Correct` always invalidates the targeted live fact and pushes **one** replacement `Fact` whose VT comes from the op fields — it never splits. There is **no** named cargo test whose name contains `correct` under `crates/`. Emit already fail-closes missing `fact_seq` via `RuntimeError::UnknownFact`. A new enum variant is the locked API (D-C1); serde’s default externally tagged JSON keeps old `Correct` events readable.

**Primary recommendation:** Add `Op::CorrectInterval { fact_seq, object, patch_from, patch_to }` with `patch_to: Option<ValidTime>`. In `GraphFold::apply`, if the live fact’s VT intersects the patch, **invalidate the original row without changing its VT bounds**, then push 0–2 residual facts (original object) plus one replacement (new object), all with the **same `claim_id`**. Reject at `Runtime::emit` (do not append) when the fact is missing, not live, the patch range is empty/inverted, or intersection is empty. Register two named tests in the governor. No new crates. Do not thaw freeze. Do not mark ROADMAP S04 done while STATE still names Active Slice S04.

## Architectural Responsibility Map

| Capability | Primary Tier | Secondary Tier | Rationale |
|------------|-------------|----------------|-----------|
| Interval-patch write (`CorrectInterval`) | API / Backend (`kutha-common` `Op` + `Runtime::emit`) | Database / Storage (append-only log / WAL cousin) | Typed log event is SoT (ADR-010 D010-1). WAL is bytes, not algebra. |
| Residual / replacement `Fact` picture | API / Backend (`GraphFold::apply`) | — | Fold is a droppable picture; VT×TT liveness is `Fact::is_live_at`. |
| Whole-version `Correct` non-regression | API / Backend (existing fold arm, unchanged) | — | CORR-02 is a behavioral freeze on that arm. |
| Named oracle (2012 / 2021 / interior) | Test runner (`cargo test` integration) | Harness observe (`observe_cargo`) | Fitness lives in crates; governor observes names, does not own product truth. |
| GATE-01 registration | Harness dictionaries (YAML) | — | Bridge + FSM needles; no new Python kind. |
| Admission / conflict / allowed action | — (out of scope) | — | D-C4 / S08. |

## Project Constraints (from .cursor/rules/)

- Chat with the user is Russian; this RESEARCH.md and all planning artifacts stay English.
- Do **not** load LifeOS / PAI / notify endpoints.
- Structural search: `codebase-memory-mcp`; `list_projects` first; graph before Grep for symbols; `check_index_coverage` on cited paths. **Do not** call `index_repository`, `delete_project`, or `manage_adr`.
- CBM cited-path freshness this session: `metadata_changed` on `event.rs`, `fold.rs`, `quantum.rs`, M011/FF5 tests, and governor YAML (index `indexed_at` `2026-09-16T07:18:52Z`). Claims below are from **Read of source**, not from the stale graph as SoT. Integrator may reindex later; this researcher must not.
- Graphify overlay is **disabled** in GSD config — no `graphify query`.
- GitNexus is secondary — not queried.
- Product plane: `crates/kutha-*`. Harness: `scripts/kutha_gov`, `.kutha/`. No Python in `kutha-runtime`. No repo-root hexagon folders.
- Freeze until STATE names M002: no RocksDB, Cypher/GPML, HNSW, ADR-050 six dictionaries, legal pack.
- Honeycomb cells stay **Proposed**. Governor green ≠ ADR Accepted ≠ `L_capability`.
- New governor check = YAML row (kinds allowlist in `.kutha/META.md`). Bridge cites product tests. Do not add `scripts/kutha_gov/checks/*.py`.
- `docs-coupling`: a `crates/**/*.rs` diff must include `CHANGELOG.md` in the same commit; a `checks.yaml` diff must include `invariants.yaml` **or** `bridges.yaml`.
- Do **not** edit `.kutha/STATE.md` in this phase (lease cite only). Trajectory check requires Active Slice S04 to remain an **unchecked** ROADMAP line `- [ ] **S04:` while leased.
- Compound Engineering verbs (коммит / changelog / PR) stay on CE / `kutha-changelog` skills at execute time — this research does not author those skills.

## Standard Stack

### Core

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| Rust edition | `2021` (`Cargo.toml` workspace.package) | Language | Existing workspace |
| `kutha-common` | path `crates/kutha-common` | `Event`, `Op`, `ValidTime` | Product write surface |
| `kutha-runtime` | path `crates/kutha-runtime` | log, fold, emit, WAL | Product runtime |
| `serde` | `1.0.229` (`Cargo.lock`) | `Op` / `Event` JSON | WAL + `events.jsonl` |
| `serde_json` | `1.0.151` (`Cargo.lock`) | Payload codec | Same |
| `uuid` | `1.24.1` (`Cargo.lock`), features `v7`, `serde` | `EventId` | Existing |
| `sha2` | `0.10.9` (`Cargo.lock`) | `digest_bytes` / fold fingerprint | Existing |

### Supporting

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `crc32fast` | workspace `1` | WAL framing | Unchanged WAL cousin |
| `uv` + Python | `uv 0.12.13`, Python `3.13.12` | `kutha-gov ci` | GATE-01 / D-C5 |
| `cargo` / `rustc` | `1.98.1` | `cargo test --workspace --offline` | FSM `observe_cargo` |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| New `Op` variant | Optional fields on `Op::Correct` | **Forbidden by D-C1 / ADR-013.** Would make CORR-02 a moving target. |
| Interval-patch | `Retract` + two `Assert`s | Would mint new `claim_id`s unless every Assert carries `claim: Some(...)`; not one audited op; easy to get TT/VT wrong. |
| In-repo split helper | PostgreSQL `range_minus_multi` / SQL:2011 `FOR PORTION OF` | Frozen / wrong plane. Algebra is ~20 lines and must compose with Kutha **invalidation**, not in-place VT mutation. |

**Installation:** none — do not add crates or Python packages.

**Version verification:** `Cargo.lock` packages `serde` `1.0.229` (line 201–203), `serde_json` `1.0.151` (line 231–233), `uuid` `1.24.1` (line 295–296), `sha2` `0.10.9` (line 244–245). [VERIFIED: Cargo.lock]

## Package Legitimacy Audit

This phase **does not install external packages**. Existing workspace pins stay.

| Package | Registry | Age | Downloads | Source Repo | Verdict | Disposition |
|---------|----------|-----|-----------|-------------|---------|-------------|
| — | — | — | — | — | — | No new packages |

**Packages removed due to [SLOP] verdict:** none
**Packages flagged as suspicious [SUS]:** none

## Architecture Patterns

### System Architecture Diagram

```text
Developer / test
    |  Op::CorrectInterval { fact_seq, object, patch_from, patch_to }
    v
Runtime::emit
    |  UnknownFact if seq missing
    |  IntervalPatchRejected if not live / empty patch / no VT intersection  [ASSUMED name]
    |  admit (op_relation = None → skip relation allowlist, same as Correct)
    v
Event::new → log append (SoT)
    v
GraphFold::apply  [new arm only]
    |  compute intersection of live Fact VT with patch (half-open)
    |  if empty → no-op (replay-safe)
    |  else: set original.invalidated_at = event.ingested_at  (do not change original VT)
    |        push residual prefix (old object) if any
    |        push replacement (new object, clipped intersection)
    |        push residual suffix (old object) if any
    |        all new Facts: same subject, relation, claim_id; ingested_at = patch TT
    v
Fact::is_live_at(tt, vt) / GraphFold::as_of(vt)
    |  old TT < patch TT → original still live (invalidated_at > tt)
    |  tt ≥ patch TT, vt in prefix/suffix → residual original object
    |  tt ≥ patch TT, vt in patch ∩ fact → replacement object
    v
Named cargo test → fsm.yaml observe_cargo.required → kutha-gov ci
```

### Recommended Project Structure

```text
crates/kutha-common/src/event.rs     # Op variant + Event::new + digest_bytes
crates/kutha-runtime/src/fold.rs     # split helper + apply arm; Correct arm UNCHANGED
crates/kutha-runtime/src/quantum.rs  # emit gate, op_relation, derivation_eligible_at match, RuntimeError
crates/kutha-runtime/tests/m011_partial_correction.rs   # CORR-01 + CORR-02 (new file)
.kutha/dictionaries/fsm.yaml         # observe_cargo.required names
.kutha/dictionaries/checks.yaml      # file_contains needles
.kutha/dictionaries/bridges.yaml     # B-m011-partial-correction
.kutha/dictionaries/honeycomb.yaml   # optional: ADR-013 evidence list only; map stays Proposed
CHANGELOG.md                         # Product + Process (docs-coupling)
```

Do **not** add files under `scripts/kutha_gov/checks/`. Do **not** add Rocks/Cypher/HNSW crates.

### Pattern 1: Whole-version Correct (preserve exactly)

**What:** Invalidate the live targeted fact; push **one** new `Fact` with op-supplied VT and object; copy `claim_id`.
**When to use:** CORR-02 regression; do not call this path for interior residuals.

Verbatim current arm [VERIFIED: crates/kutha-runtime/src/fold.rs:163-190]:

```163:190:crates/kutha-runtime/src/fold.rs
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

If a caller passes `valid_from: 2015, valid_to: Some(2020)` against a fact `[2010, ∞)`, this path **does not** leave `[2010,2015)` or `[2020, ∞)` live. That is the CORR-02 oracle.

### Pattern 2: Explicit interval patch (new arm)

**What:** Same invalidation of the **whole** original row (loser remains, VT bounds on that row stay as ingested). New live rows cover `fact_VT − patch` (0–2 residuals, original object) and `fact_VT ∩ patch` (replacement object).
**When to use:** CORR-01 only.

**Identifier:** use `CorrectInterval` (CONTEXT working name). Payload names from D-C1: `fact_seq`, `patch_from`, `patch_to`, `object`.

**`patch_to` type:** `Option<ValidTime>` so infinity matches existing `valid_to: None` [VERIFIED: crates/kutha-common/src/event.rs:10-14, 34-39]:

```10:14:crates/kutha-common/src/event.rs
/// Valid-time instant as an opaque integer clock (world).
pub type ValidTime = u64;

/// Transaction-time as log sequence (system).
pub type TransactionTime = u64;
```

```34:39:crates/kutha-common/src/event.rs
    Correct {
        fact_seq: u64,
        object: TermId,
        valid_from: ValidTime,
        valid_to: Option<ValidTime>,
    },
```

**Half-open liveness** [VERIFIED: crates/kutha-runtime/src/fold.rs:29-42]:

```29:42:crates/kutha-runtime/src/fold.rs
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

Treat `None` as `+∞` when comparing `valid_to` / `patch_to`. Intersection of `[A,B)` and `[C,D)` is `[max(A,C), min(B,D))` (empty iff `from >= to` after substituting `∞`). Residuals are the two halves of `fact − intersection`.

**Fail-closed emit (D-C7):** mirror missing-seq on Correct [VERIFIED: crates/kutha-runtime/src/quantum.rs:274-280]:

```274:280:crates/kutha-runtime/src/quantum.rs
        if let Op::Retract { fact_seq } | Op::Correct { fact_seq, .. } = &op {
            if !self.fold.facts().iter().any(|f| f.seq == *fact_seq) {
                return Err(RuntimeError::UnknownFact {
                    fact_seq: *fact_seq,
                });
            }
        }
```

Extend that `if let` to also bind `CorrectInterval { fact_seq, .. }`. **Additionally** (interval-patch only): if the fact exists but `invalidated_at` is set, or patch is inverted (`patch_to` is `Some(t)` and `patch_from >= t`), or intersection with the live fact VT is empty — return an error and **do not append**. Recommendation: new variant `RuntimeError::IntervalPatchRejected { fact_seq: u64 }` [ASSUMED identifier] plus a `Display` arm (the current `match self` on `Display` is exhaustive: `ReplayDivergence`, `UnknownFact`, `UnknownRelation`, `UnknownClaim`, `BrokenLineage` [VERIFIED: crates/kutha-runtime/src/quantum.rs:12-28, 31-44]).

Do **not** change Correct’s behavior when the fact is already invalidated (fold no-op, event still appends today). CORR-02 forbids tightening Correct.

**Fold must compute splits before invalidate.** If you invalidate first and then discover an empty intersection, you have destroyed the live fact. Retract/Correct already no-op on missing seq; the new arm should no-op on empty intersection so WAL-injected bad events cannot erase history.

**Push order (fingerprint):** prefix, replacement, suffix. `GraphFold::fingerprint` hashes facts in vec order including losers [VERIFIED: crates/kutha-runtime/src/fold.rs:77-90].

**Exhaustive `Op` matches to update (compile will catch):**

| Site | Today | Change |
|------|-------|--------|
| `Event::new` `object_ids` | `Op::Correct { object, .. } => vec![*object]` [VERIFIED: crates/kutha-common/src/event.rs:80] | Same for `CorrectInterval` |
| `Event::digest_bytes` | tag `b"correct"` [VERIFIED: crates/kutha-common/src/event.rs:118-128] | New tag `b"correct-interval"` (must not equal `b"correct"`) [ASSUMED bytes] |
| `GraphFold::apply` | no wildcard; `Define` last | New arm |
| `op_relation` | `Op::Retract { .. } \| Op::Correct { .. } \| Op::Define { .. } => None` [VERIFIED: crates/kutha-runtime/src/quantum.rs:47-51] | Include `CorrectInterval` in the `None` arm (replacement is an interned object, not a relation name; Correct already skips FF6) |
| `derivation_eligible_at` premise | `Op::Retract { .. } \| Op::Correct { .. } \| Op::Define { .. } => return false` [VERIFIED: crates/kutha-runtime/src/quantum.rs:396-399] | Include `CorrectInterval` (a patch event is not a premise claim opener) |
| `follow_ons` | `_ => vec![]` | No change |
| `store::defined_names` | `Op::Define { name }` / `_` | No change |
| WAL `VERSION` | `const VERSION: u32 = 1` [VERIFIED: crates/kutha-runtime/src/wal.rs:8] | Do **not** bump; payload remains JSON `Event` |

### Pattern 3: GATE-01 governor registration (copy M011)

**What:** Named `fn` in `crates/**/*.rs` + FSM required list + check needles + bridge.
**When to use:** every leased slice.

FSM required names today include M011 S01–S03 [VERIFIED: .kutha/dictionaries/fsm.yaml:31-43]:

```31:43:.kutha/dictionaries/fsm.yaml
    required:
      - ff5_as_of_t1_differs_from_as_of_t2_on_statute_log
      - ff6_unknown_relation_does_not_append
      - h2_harness_status_as_of_t1_differs_from_as_of_t2
      - h2_same_second_status_as_of_uses_emitted_cut
      - h4_prior_cut_keeps_status_membership_after_later_edition_drops_it
      - open_without_snapshot_recovers_intern_meanings
      - open_without_snapshot_or_terms_file_recovers_from_define_ops
      - intern_appends_define_for_new_terms_only
      - retracting_one_support_leaves_claim_supported
      - unknown_claim_does_not_append
      - replay_rejects_behavior_without_prior_cause
      - derived_q_loses_eligibility_when_last_premise_support_withdrawn
```

Check `observe-required-fn` requires each of those strings to appear as `fn ` in `crates/**/*.rs` [VERIFIED: .kutha/dictionaries/checks.yaml:138-147].

Bridge pattern [VERIFIED: .kutha/dictionaries/bridges.yaml:39-42]:

```39:42:.kutha/dictionaries/bridges.yaml
  - id: B-m011-claim-supports
    claim: "Independent supports share claim_id; unknown claim and dangling caused_by fail closed; thin P→Q eligibility tracks premise supports"
    cites: "crates/kutha-runtime/tests/m011_claim_supports.rs"
    check: m011-claim-supports
```

**Do not** put this product fence in `invariants.yaml` (control-loop ledger). Intake: bridge + `checks.yaml` [VERIFIED: docs/process/governor-intake.md:27-29].

**Recommended names (planner may shorten):**

- `interval_patch_leaves_vt_2012_and_2021_residuals` — CORR-01
- `whole_version_correct_does_not_invent_residuals` — CORR-02

File: `crates/kutha-runtime/tests/m011_partial_correction.rs` (prefix `m011_` per `.planning/codebase/TESTING.md`).

Honeycomb: ADR-013 stays `map: Proposed` [VERIFIED: .kutha/dictionaries/honeycomb.yaml:108-117]. Optional: append the new test names to `evidence:` only. Do not set Status Accepted.

### Anti-Patterns to Avoid

- **Reinterpret `Op::Correct` as interval-patch** when `valid_from`/`valid_to` are narrower than the old fact. Forbidden by D-C1 and ADR-013.
- **Mutate the original fact’s `valid_from`/`valid_to` in place.** Old TT would then see the clipped interval (`is_live_at` uses current bounds). ADR-013: old TT must still see the old version. Invalidate + insert, like Correct.
- **Mint a new `claim_id` for residuals.** D-C2; Correct already copies `claim_id`.
- **Use CSR as the residual oracle.** `CsrLease::from_fold` stores neighbor **objects** only and `dedup`s [VERIFIED: crates/kutha-runtime/src/csr.rs:12-32]. Assert on `fold.as_of` / `facts()` (`claim_id`, `valid_from`, `object()`).
- **Full semantic-contract fixture** (source `b`, conflict, summaries). D-C3 / S08.
- **Check ROADMAP S04 `[x]` while Active Slice is S04.** Trajectory `require_template` is `- [ ] **{id}:`.
- **Edit `.kutha/STATE.md`** to claim `L_delivery=M011-S04-done` in this phase.
- **New Python check kind** or `checks/*.py`.
- **Thaw freeze** (Rocks, Cypher, HNSW, ADR-050 six dicts, legal pack).
- **Coalesce** residual rows with other live facts. Out of scope.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| JSON `Op` codec | Custom tag parser | existing `serde` / `serde_json` on `Op` | Externally tagged default [CITED: https://serde.rs/enum-representations.html] |
| WAL framing | New MAGIC/VERSION | `wal.rs` as-is (`VERSION` `1`) | Interval patch is still an `Event` JSON payload |
| Relation allowlist | New dictionary row for “patch” | `op_relation` → `None` like Correct | Patch does not name a relation |
| Interval leftover algebra | PostgreSQL / SQL:2011 engine | 15–30 line helper in `fold.rs` | Wrong plane; freeze; must pair with invalidation |
| Governor observe | Python `Check` subclass | YAML `file_contains` + FSM `required` | META allowlist |
| Dual-source conflict | Admission dictionary | nothing in S04 | D-C4 |

**Key insight:** SQL:2011 `UPDATE … FOR PORTION OF` leftover count (zero, one, or two) is a **checksum on the helper**, not a product dependency. Postgres application-time UPDATE **clips the targeted row in place** and inserts leftovers; Kutha **must not clip the original row’s VT** because TT invalidation is a separate field (`invalidated_at`). Wiki analog (untrusted):

DATA_k7m2p9qx_START
In an update, the row's application time is automatically changed according to the targeted portion. The new start/end times are the old values intersected with the target.

Then if the row included any history outside the targeted portion, the database inserts new row(s) with the old values (but appropriately updated start/end times) to preserve that history. In effect these new rows cover `old_history - targeted_history`. For range or period columns, there can be zero, one, or two such "temporal leftover" records.
DATA_k7m2p9qx_END

DATA_w3n8c1va_START
The temporal update/delete leftovers are treated like other new rows: their System-Time is from `current_time` to `Infinity`.
DATA_w3n8c1va_END

Map leftovers → new `Fact`s at patch TT; map “old row still queryable at old system time” → original `Fact` kept with `invalidated_at = patch TT`. [CITED: https://wiki.postgresql.org/wiki/SQL2011Temporal] [LOW for SQL standard text — official ISO was not fetched]

## Common Pitfalls

### Pitfall 1: Half-open endpoint looks like overlap

**What goes wrong:** Treating `[2010,2015)` and `[2015,2020)` as intersecting.
**Why:** `is_live_at` uses `vt >= to` → `vt == 2015` is **not** live on the first interval.
**How to avoid:** Intersection empty ⇒ emit reject; add a named assertion in the fail-closed test.
**Warning signs:** Residual at VT 2015 after a patch starting at 2015, or a successful patch that does not touch the fact.

### Pitfall 2: `facts()[0]` after intern/patch

**What goes wrong:** Tests index `facts()[0]` as “the” live support.
**Why:** `intern` fold-no-ops `Define` so the first Assert is seq 0, but after patch the original row stays in the vec (invalidated) and three new rows append.
**How to avoid:** Capture `seq` and `claim_id` **before** the patch; filter `is_live_at` / `claim_id`.

### Pitfall 3: Same object on replacement hides CORR-01

**What goes wrong:** Interior `as_of(2017)` still looks like the original triple.
**How to avoid:** Replacement `object` must be a distinct interned term (e.g. `"P"` vs `"P-prime"`). Residuals keep the original object.

### Pitfall 4: WAL recover drops unknown variants

**What goes wrong:** An older binary reading a log that contains `CorrectInterval` hits `serde_json::from_slice` `Err` and `break`s [VERIFIED: crates/kutha-runtime/src/wal.rs:79-82], dropping that record **and the tail**.
**How to avoid:** In-tree only one decoder; add a persist/open or JSON round-trip in tests. Do not bump WAL `VERSION` for this.

### Pitfall 5: Cascade budget / `graph_len`

**What goes wrong:** Treating `CorrectInterval` like `Define` (stripped from snapshot offset).
**Why:** `graph_len` excludes only `Op::Define` [VERIFIED: crates/kutha-runtime/src/quantum.rs:125-129]. Interval patch is a graph op — correct. `follow_ons` will not emit inverse-`knows` (not an `Assert` of `knows`).

### Pitfall 6: Governor HIGH from missing needles

**What goes wrong:** Test exists but FSM/`fn ` needle mismatch (`observe-required-fn`).
**How to avoid:** Identical identifier in `fn name`, `fsm.yaml` required list, and `checks.yaml` needles. Run `uv run kutha-gov ci`.

### Pitfall 7: docs-coupling / STATE

**What goes wrong:** Crate diff without `CHANGELOG.md`; or editing STATE and then failing README coupling; or marking S04 done in ROADMAP while leased.
**How to avoid:** One commit: crates + dictionaries + CHANGELOG. Leave STATE and S04 checkbox alone.

## Code Examples

### CORR-01 fixture constants and cuts

Use year-like `u64` clocks like FF5 [VERIFIED: crates/kutha-runtime/tests/ff5_legal_pit.rs:7-11]:

```7:11:crates/kutha-runtime/tests/ff5_legal_pit.rs
/// Year-like valid-time instants for a statute-shaped fixture (not wall-clock).
const T_OLD: u64 = 2015;
const T_NEW: u64 = 2021;
const VF_FIRST: u64 = 2010;
const VF_SECOND: u64 = 2020;
```

Map D-C3 / observation 3: wide VT from `2010`, patch `[2015,2020)`, probes `2012` and `2021`, interior e.g. `2017`. Relation must be allowlisted [VERIFIED: crates/kutha-runtime/dictionaries/relations.yaml:6-13]:

```6:13:crates/kutha-runtime/dictionaries/relations.yaml
relations:
  - knows
  - knownBy
  - relatedTo
  - inForceAs
  - runStatus
  - observed
  - processAllows
```

Recommend `relatedTo` (already used in M011 tests), subject intern `"a"`, objects intern `"P"` then `"P-prime"`. `as_of` is `live_at(u64::MAX, vt)` [VERIFIED: crates/kutha-runtime/src/fold.rs:72-75].

### CORR-02 regression shape

`emit(Op::Correct { fact_seq, object: p_prime, valid_from: 2015, valid_to: Some(2020) })` then:

- `as_of(2012)` and `as_of(2021)` do **not** contain `(a, relatedTo, P)`
- live facts for that `claim_id` at those VTs: **0**
- `as_of(2017)` contains the replacement only
- `fold.facts()` has the original **invalidated** plus **one** new live fact (not three)

Call `replay_check()` as M011 tests do.

### Fail-closed non-intersect

Capture `log().len()`, emit a patch whose `[patch_from, patch_to)` does not intersect the live fact (including the touching-endpoint case `[2010,2015)` vs `[2015,2020)`), expect error, `log().len()` unchanged (same idea as `unknown_claim_does_not_append` [VERIFIED: crates/kutha-runtime/tests/m011_claim_supports.rs:50-72]).

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Whole-version `Correct` only | Explicit `CorrectInterval` + unchanged `Correct` | S04 (this phase) | Observation 3 becomes a named oracle; Correct stays honest |
| SQL:2011 `FOR PORTION OF` in-place clip | Invalidate original `Fact`; insert leftovers + replacement | N/A (analog) | Preserves Kutha TT (`invalidated_at`) |
| Optional fields on `Correct` | New `Op` variant | D-C1 2026-09-29 | Avoids silent residuals |

**Deprecated/outdated:**

- Using today’s `Correct` API “as if it supplied those residuals automatically” — semantic-contract observation 3 [VERIFIED: docs/architecture/semantic-contract-validation.md:43]: `Under an explicit interval-patch contract, VT 2012 and 2021 retain residual versions of a. The current whole-version Correct API must not be used as if it supplied those residuals automatically.`
- ADR-013 still **Proposed**; implementation note [VERIFIED: docs/ADR/ADR-013-bitemporal-facts-invalidation.md:111-112]: `Intervals are half-open. Current Correct replaces the entire targeted version at its new TT; it is not an interval-patch operator. A future partial correction of [2015,2020) within [2010,infinity) must explicitly preserve [2010,2015) and [2020,infinity) as residual versions, or reject the partial operation. Do not silently reinterpret today's API. Old TT queries must still see the old version.`

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | New `RuntimeError` identifier `IntervalPatchRejected` | Pattern 2 | Planner picks another name; tests/`Display` must match |
| A2 | Digest tag bytes `b"correct-interval"` | Pattern 2 | Any unique tag works; must not collide with `b"correct"` |
| A3 | Two named tests rather than one | GATE-01 | D-C6 wording is singular “test”; one fn covering both IDs is also valid if both oracles are asserted and **that** name is registered |
| A4 | Overhanging patch (intersects but not subset) is **clipped** to intersection rather than rejected | Pattern 2 | D-C7 only mandates fail-closed on **non-intersect**. Reject-unless-subset is a valid stricter policy; fixture is subset so either works if tested |
| A5 | Already-invalidated target **rejects** for `CorrectInterval` (stricter than Correct) | Pattern 2 | Matching Correct (append + fold no-op) is also fail-closed on truth but not on “successful emit” |

**If this table is empty:** All claims in this research were verified or cited — no user confirmation needed.

A3–A5 are planner discretion under CONTEXT; they do **not** need a user checkpoint if the plan picks one and tests it.

## Open Questions (RESOLVED)

1. **Reject vs no-op for non-intersect (D-C7)** (RESOLVED)
   - What we know: emit-reject-without-append matches `UnknownClaim` / missing `fact_seq`. Fold no-op is required for replay of a bad logged event.
   - **PLAN lock (04-01):** emit reject without append (`UnknownFact` for missing seq; `IntervalPatchRejected` for non-intersect, inverted, and not-live); fold empty-intersection no-op for replay. Fail-closed tests in 04-01 task 2.

2. **Optional persist/open round-trip** (RESOLVED)
   - What we know: WAL JSON must include the new variant; `recover_events` drops JSON errors.
   - **PLAN lock (04-01 tracer):** `serde_json` to_value / from_value of an `Event` whose op is `CorrectInterval` inside `interval_patch_leaves_vt_2012_and_2021_residuals`. Full `store::persist` / `open` stays out (not S05).

3. **Honeycomb evidence vs ADR body** (RESOLVED)
   - What we know: ADR edit implies `honeycomb.yaml` + CHANGELOG (`docs-coupling`).
   - **PLAN lock (04-03):** do not edit `docs/ADR/ADR-013-bitemporal-facts-invalidation.md`; append the two test names to honeycomb ADR-013 `evidence` only; `map:` stays Proposed.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| `cargo` / `rustc` | product tests, FSM observe | ✓ | 1.98.1 | — |
| `uv` | `kutha-gov ci` / pytest | ✓ | 0.12.13 | — |
| Python | harness | ✓ | 3.13.12 (`requires-python = ">=3.13"` [VERIFIED: pyproject.toml:5]) | — |
| RocksDB / Cypher / HNSW | — | n/a | — | Frozen; do not use |
| Context7 / Exa MCP | research-plan providers | ✗ (not in this agent’s MCP catalog) | — | Official serde.rs via WebFetch; wiki/search for SQL analog |

**Missing dependencies with no fallback:** none for execution.

**Missing dependencies with fallback:** Context7/Exa (docs already fetched via serde.rs + wiki).

**Step 2.6:** not skipped — toolchain probed.

## Validation Architecture

`workflow.nyquist_validation` is `true` [VERIFIED: .planning/config.json:24].

### Test Framework

| Property | Value |
|----------|-------|
| Framework | Rust `cargo test` (edition 2021 workspace); harness pytest ≥8 |
| Config file | none for Rust (per-crate); `pyproject.toml` `[tool.pytest.ini_options]` |
| Quick run command | `cargo test -p kutha-runtime --test m011_partial_correction --offline` |
| Full suite command | `cargo test --workspace --offline` then `uv run kutha-gov ci` |

### Phase Requirements → Test Map

| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| CORR-01 | Interval patch leaves VT 2012 and 2021 residuals of `a`; interior is replacement; same `claim_id` | integration | `cargo test -p kutha-runtime --offline interval_patch_leaves_vt_2012_and_2021_residuals -- --exact` | ❌ Wave 0 |
| CORR-02 | Whole-version `Correct` does not invent residuals at 2012/2021 | integration | `cargo test -p kutha-runtime --offline whole_version_correct_does_not_invent_residuals -- --exact` | ❌ Wave 0 |
| GATE-01 | FSM required names + check needles; `ci` HIGH==0 | harness | `uv run kutha-gov ci` | ❌ Wave 0 (needles) |
| (support) | Non-intersect / inverted patch does not append | integration | same test file, extra `fn` or cases in CORR-01 file | ❌ Wave 0 |
| (support) | `replay_check` after patch | unit/integration | called from CORR-01/02 | pattern exists in M011 |

### Sampling Rate

- **Per task commit:** `cargo test -p kutha-runtime --test m011_partial_correction --offline` (once the file exists); if dictionaries changed, `uv run kutha-gov precommit`
- **Per wave merge:** `cargo test --workspace --offline` and `uv run kutha-gov ci` (D-C5 / D-15: cargo on any wave that touches `crates/`)
- **Phase gate:** Full `uv run kutha-gov ci` HIGH-free; WARN ledgered (D-11)

### Wave 0 Gaps

- [ ] `crates/kutha-runtime/tests/m011_partial_correction.rs` — CORR-01, CORR-02, fail-closed non-intersect
- [ ] `Op::CorrectInterval` + fold/emit/digest/`Display` arms
- [ ] `.kutha/dictionaries/fsm.yaml` required names
- [ ] `.kutha/dictionaries/checks.yaml` + `bridges.yaml` (not invariants)
- [ ] `CHANGELOG.md` Product + Process
- [ ] Framework install: none

Existing M011/FF5 helpers are reusable; they do not cover observation 3.

## Security Domain

`security_enforcement` is enabled (`true`, ASVS level 1) [VERIFIED: .planning/config.json:48-50].

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | no | No user/session surface |
| V3 Session Management | no | — |
| V4 Access Control | no | D-C4: no ABAC / admission dictionary |
| V5 Input Validation | yes | Typed `ValidTime` / `TermId` / `fact_seq`; reject empty/inverted/non-intersecting patch; unknown `fact_seq` → `UnknownFact` |
| V6 Cryptography | no | Do not add hashes; existing SHA-256 digest/fingerprint stay as-is — never hand-roll a new construction |

### Known Threat Patterns for this stack

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Silent rewrite of history (in-place VT clip) | Tampering | Invalidate original `Fact`; losers remain (ADR-013) |
| Implicit residuals via `Correct` | Elevation of privilege / integrity lie | New op only; CORR-02 regression |
| Unknown / non-intersecting patch invents truth | Tampering | Fail-closed emit (no append) |
| Unallowlisted **relation** on patch | Tampering | N/A — patch does not carry a relation; original Assert already passed FF6 |
| Log injection of unknown `Op` tag | Tampering | serde unknown variant fails; WAL drops record (fail-closed decode) |
| Using CSR/ANN as oracle | Information disclosure / integrity | Assert on fold cuts only |

Do not add network, SQL, or HTML parsers. Interval clocks are `u64`, not attacker-controlled strings.

## Sources

### Primary (HIGH confidence)

- `crates/kutha-common/src/event.rs` — `Op::Correct`, `Event::new`, `digest_bytes` (Read this session)
- `crates/kutha-runtime/src/fold.rs` — `Fact`, `is_live_at`, `Correct` arm, fingerprint (Read)
- `crates/kutha-runtime/src/quantum.rs` — `emit`, `op_relation`, `RuntimeError`, `derivation_eligible_at` (Read)
- `crates/kutha-runtime/src/wal.rs` — JSON payload, `VERSION` `1`, decode `Err` → `break` (Read)
- `crates/kutha-runtime/tests/m011_claim_supports.rs`, `ff5_legal_pit.rs` (Read)
- `.kutha/dictionaries/{fsm,checks,bridges}.yaml`, `relations.yaml` (Read)
- `docs/ADR/ADR-013-bitemporal-facts-invalidation.md`, `docs/architecture/semantic-contract-validation.md` (Read)
- `.planning/phases/04-partial-correction-with-residual-intervals/04-CONTEXT.md` (Read)
- CBM `list_projects` / `search_graph` / `trace_path` / `check_index_coverage` (project `kutha-graph`; coverage `metadata_changed`)

### Secondary (MEDIUM confidence)

- [CITED: https://serde.rs/enum-representations.html] — default externally tagged JSON `{"Request":{...}}` (WebFetch this session)
- [CITED: https://docs.rs/serde/1.0.229/serde/] — crate version 1.0.229 matches `Cargo.lock`

### Tertiary (LOW confidence)

- [CITED: https://wiki.postgresql.org/wiki/SQL2011Temporal] — `FOR PORTION OF` leftovers 0–2; bitemporal leftovers at current system time (wiki, not ISO SQL:2011 text)
- WebSearch snippets of PostgreSQL 19 DML temporal docs (official URL WebFetch returned 404)

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — workspace `Cargo.lock` / `Cargo.toml` / probed toolchain
- Architecture: HIGH — fold/emit/Op matches Read + CBM snippet of `GraphFold.apply` still matches disk despite stale index
- Pitfalls: HIGH for in-repo (half-open, WAL, governor); LOW for SQL analog

**Research date:** 2026-09-29
**Valid until:** 30 days (stable in-repo contract; serde 1.x representation is stable)

**GSD init:** `phase_dir` `.planning/phases/04-partial-correction-with-residual-intervals`, `padded_phase` `04`, `commit_docs` true. Graphify disabled.
