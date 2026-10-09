# ADR-013: Bi-temporal Facts & Invalidation

## Status

**Proposed** (Time/Query fact semantics — Graphiti *semantics* native, without Graphiti runtime)

### Clarification (2026-10-09; Phase 94 / M034 S01; Proposed)

**Correct Horn follow-ons mirror CorrectInterval except-set and fire order** (CE review P1 #9). `rule_follow_ons` for `Op::Correct` builds a retract except-set, calls `horn_fires_except`, and orders fires-then-retracts so LIFO applies retracts before re-fires. A same-triple Correct (e.g. VT-only) no longer loses its derived inverse to suppression by the still-live old row. Object-changing Correct re-fires the new inverse. Oracles: `correct_same_triple_keeps_derived_inverse_live`, `correct_object_change_re_fires_new_inverse`. Status stays **Proposed**.

### Clarification (2026-10-09; Phase 93 / M033 S01; Proposed)

**Admission `stale_support` covers any live CorrectInterval residual at `row.vt`** (CE review P1 #8). `check_admission_row` uses `live_fact_indices_by_event_id` and admits when any residual is live at `(tt=MAX, row.vt)`. A single `fact_index_by_event_id` representative (often the prefix) wrongly denied patch/suffix justifications. Dead CI id still yields `stale_support`. Oracles: `admission_admits_when_cited_ci_residual_covers_patch_vt`, `admission_still_denies_when_no_ci_residual_covers_vt`. Status stays **Proposed**.

### Clarification (2026-10-09; Phase 92 / M032 S01; Proposed)

**Facade `derive()` premise lookup filters live facts only** (CE review P1 #4). `Runtime::derive` selects the latest product fact for the subject with `invalidated_at.is_none()`, matching `retract` / `correct`. A sole retracted premise yields `UnknownFact` (no Behavior). When a later Assert is retracted, derive cites the prior live Assert as `caused_by`. Oracles: `derive_after_sole_premise_retract_fails_closed`, `derive_skips_invalidated_latest_uses_prior_live`. Status stays **Proposed**.

### Evidence (2026-10-02; Proposed)

M014 shipped typed clocks (`ValidTime` / `TransactionTime` / `WallTime`) and `as_known_on` via `tt_anchor` (`true_on_d_differs_from_known_on_d`). Phase 34 (M015 S01) made the known-on cut TT-only (`closed_vt_interval_survives_as_known_on`) and fail-closed duplicate/backwards walls. FF5 still compares two valid-time windows at transaction-time max; it is not the known-on query. Gap-review F13 clock/anchor ship is in crates.

**FIXG-01 / M016 S05 (2026-10-02; fail-closed):** C01 sunset («пункт 15.1 действует до 01.01.2028») cannot be a Named/Horn body distinct from valid-time. `RuleTerm::Const` is rule-ident only (no calendar tokens); RuleAst has no temporal atoms. Folding the sunset into `valid_to` is VT, not an application clock. Two axes + typed annotations + `as_known_on` remain sufficient for known-on vs true-on; they are **not** sufficient for transitional-rule / application-end semantics without a fifth core axis. This verdict **stops** before inventing five clocks. Cell remains **Proposed**. Named oracle: `fixg01_c01_sunset_inexpressible_as_horn`.

## Date

2026-08-18

## Honeycomb coordinates

- Axes: **Time** (primary) · **Query** · **Verify** · **Agent**
- Depends on: ADR-000 (D4), ADR-010, ADR-011
- Anticipates: ADR-043 (interval index as an access path), ADR-050 (dicts use VT×TT), ADR-070 (AS OF surface)

## Context

ADR-000 D4 already requires valid-time × transaction-time on facts, with invalidation as behavior. Graphiti popularized *bitemporal fact edges* as a product; the matrix judged Graphiti **low optimality** as Kutha SoT. This cell takes the **semantics** and rejects the runtime.

Grounding cards:

- `paper-toki-contradiction-ops` — `.compound-engineering/artifacts/research/applicability/cards/paper-toki-contradiction-ops.md`
- `paper-memstrata-stale-fact` — `.compound-engineering/artifacts/research/applicability/cards/paper-memstrata-stale-fact.md`
- `paper-tgql-intervals` — `.compound-engineering/artifacts/research/applicability/cards/paper-tgql-intervals.md`
- `paper-temporal-interval-index` — `.compound-engineering/artifacts/research/applicability/cards/paper-temporal-interval-index.md`

TOKI: isolation-typed contradiction operators; losers stay in an audit row; LLM-as-judge on the write path is itself logged or it produces write-time anomalies. MemStrata: cosine cannot see contradiction (AUROC ~0.59); supersession is a deterministic (s,r,o) rule on a bi-temporal ledger. T-GQL: validity-interval property graphs and temporal *path* semantics — Neo4j is a host, not SoT. Interval indexes (Timeline Index, TIDE, MAP21, …) are the **access path** for valid-time; the log is ordered by transaction time.

## Decision

### D013-1. Native VT × TT on facts

Every fact/edge carries `valid_from` / `valid_to` (world) and `ingested_at` / `invalidated_at` (system), plus provenance. Core algebra is **two axes**. Legal five-clock stamps (ADR-090) map as **typed annotations**, not five peer algebraic axes.

### D013-2. Invalidation is typed; losers remain

Assert / retract / correct (ADR-010) plus TOKI-class contradiction operators. Isolation is declared at write time. Losers remain queryable as audit/belief-as-of until vacuum policy (ADR-012) applies. Silent overwrite is forbidden.

### D013-3. Supersession is deterministic, not cosine

An explicit correction or declared, versioned supersession policy retires the targeted claim/version/support. Equal endpoints or overlapping valid-time alone do not establish supersession: independent conflicting evidence must remain distinguishable. Embedding similarity and RAG cannot own stale-fact. LLM is not on the write path except as a logged proposal that still must validate.

### D013-4. Interval index is a lease; T-GQL is a language cousin

Valid-time overlap/contain/AS-OF uses an interval access path keyed by interned ids (ADR-011). The index is droppable. Temporal path algorithms (T-GQL family) sit on “which edges are live in `[t0,t1)`” — they are not the log. Query surface language is ADR-070.

**Hard separations:**

```text
Valid-time             ≠  Transaction-time
Invalidation           ≠  Delete
TOKI write contract    ≠  Chat heuristic / LWW-as-default
Cosine / RAG           ≠  Stale-fact detector
T-GQL / Neo4j host     ≠  Kutha SoT
Interval index         ≠  Snapshot placement (ADR-012)
Graphiti edges         ≠  Native fact semantics
```

## Consequences

### Positive

- Legal statutory QA can fail closed on superseded law.
- Dictionary entries and meta-prompt versions (050) reuse the same stamps.
- Planner (043) has a named valid-time access path.

### Negative / risks

- (s,r,o) keying vs free-text nodes is still a spike.
- TOKI isolation formalization before P0 may over-specify.

### Non-goals (this ADR)

- Implementing Graphiti, T-GQL on Neo4j, or a third clock.
- Cypher grammar (ADR-070).
- Vacuum of losers (ADR-012).

## Alternatives Considered

| Alternative | Why not |
|-------------|---------|
| Graphiti as runtime | Trap; LLM write path; not Kutha SoT |
| Latest-state RAG | Cannot reconstruct prior belief; MemStrata fail |
| CRDT merge | Drops losers; not audited |
| Hindsight four-network memory | Agent memory product, not fact algebra |
| Valid_from fields as SoT | Prevents correction/provenance |

## Open Research Questions

1. Minimal TOKI operator subset for P1 (LWW vs await-confirm vs policy).
2. Fact key: interned (s,r,o) vs attributed hyperedge.
3. Interval index choice (Timeline vs B+ duration×endpoint) as a lease pack.
4. How legal five-clock stamps attach without becoming core axes.

## Related Decisions

- ADR-000 — D4
- ADR-010 — typed writes
- ADR-011 — interned ids
- ADR-012 — vacuum of history
- ADR-043 — planner may pick interval index
- ADR-090 — L_KB clocks as stamps

## Implementation note (2026-08-18)

P0 now has named `GraphFold::live_at` / `as_of` (no silent “now”) and FF5 on a statute-shaped fixture (`crates/kutha-runtime/tests/ff5_legal_pit.rs`). Status stays **Proposed** until a harness promotion packet lists this cell; governor still forbids bulk honeycomb Accepted.

## Semantic clarification (2026-09-13; Proposed)

- `live_at(tt, vt)` asks what was known at `tt` about `vt`. `as_of(vt)` uses current transaction knowledge (`tt = MAX`); it is not historical belief replay. FF5 currently compares two VT values, not the same VT before and after a late correction.
- Intervals are half-open. Current `Correct` replaces the entire targeted version at its new TT; it is **not** an interval-patch operator. A future partial correction of `[2015,2020)` within `[2010,infinity)` must explicitly preserve `[2010,2015)` and `[2020,infinity)` as residual versions, or reject the partial operation. Do not silently reinterpret today's API. Old TT queries must still see the old version.
- Withdrawing one source removes its support, not independent supports for the same claim. Derived eligibility must be reevaluated from remaining supports; withdrawal is not itself evidence of the opposite proposition. The P0 inverse behavior does not yet implement this maintenance.
- For unresolved conflict, a candidate policy-facing view is `(positive_supports, negative_supports)`: neither, positive only, negative only, or both. This is a proposed evidence summary, not an installed four-valued logic. Query completeness (`complete`, budget-limited, unavailable) is a separate axis; missing or truncated evidence cannot become a complete negative answer. Resolution/admission remains a versioned policy decision (ADR-051/093).

### Clarification (2026-09-30; Proposed)

Target contract, from docs/architecture/semantic-gap-review.md (F3): a proposition has an identity distinct from any one support slot. Independent supports for the same proposition remain distinguishable (D013-3). Conflict is a generic operator over those supports; polarity is not supplied by the caller as the truth of the proposition. The 2026-09-13 `(positive_supports, negative_supports)` view remains a proposed evidence summary, not four-valued logic. This subsection does not rewrite D013-1…D013-4 and does not mark this cell Accepted.

### Open question (2026-09-30)

Open, from docs/architecture/semantic-gap-review.md (F2, F7): retract and correct targeting is typed invalidation (D013-2) without a required stable id for the target. D013-1 names valid-time × transaction-time fields; it does not declare a time scale or a mapping from transaction-time to wall time. This question does not rewrite D013-1…D013-4 and does not mark this cell Accepted.

### Clarification (2026-10-01; Proposed)

Target contract, from docs/architecture/semantic-gap-review.md (F13): each clock is a distinct type, not an alias of one integer. At least valid time, transaction time (log sequence), and wall time cannot be mixed without an explicit conversion; a stamp carries its clock in its type. The mapping from transaction time to wall time is a stream of `tt_anchor` facts on the log, so “as known on date D” is a greatest-anchor lookup, not an extra axis in the core algebra. This keeps D013-1 (two axes in the core; the legal clocks are typed annotations) and makes ADR-090 TR-02 a compile-time property. This subsection does not rewrite D013-1…D013-4 and does not mark this cell Accepted.

### Open question (2026-10-01)

Open, from docs/architecture/semantic-gap-review.md (F13): whether two axes plus typed annotations and anchors are sufficient for effective-date, promulgation, and transitional-rule semantics (`tempus regit actum`). The test is a real-text golden fixture with one transitional rule (M016); this question is not answered by argument. This question does not rewrite D013-1…D013-4 and does not mark this cell Accepted.

### Verdict (2026-10-02; Proposed)

Answered fail-closed by FIXG-01 (`fixg01_c01_sunset_inexpressible_as_horn`): two axes plus typed annotations and `as_known_on` are **not** sufficient for C01 application-end / transitional-rule semantics. Do not add five peer clocks. This subsection does not rewrite D013-1…D013-4 and does not mark this cell Accepted.

### Evidence (2026-10-03; Proposed)

FIXG-01 remains **OPEN**. C01 sunset («пункт 15.1 действует до 01.01.2028») is not a Horn (`fixg01_c01_sunset_inexpressible_as_horn`; `RuleTerm::Const` is rule-ident only) and is not `valid_to`. Options: (a) application stamp in a pack, (b) a fifth core axis (conflicts with D013-1), (c) stop / out of wedge scope. **No option is chosen.** This blocks the legal pack, not M002. D013-1 is unchanged.

`GraphFold::as_known_on` takes the greatest `Op::TtAnchor` with `wall` ≤ D, then TT-only `known_at` (ignores valid-time). `Runtime::as_known_on` delegates to that fold method.

### Evidence (2026-10-03; KNOWN-04 / A-F05; Proposed)

Characterization (`known04_horn_fires_when_premise_hidden_on_early_cut`): a two-premise Horn `relatedTo(A,C) :- relatedTo(A,B), relatedTo(B,C)` mints a derived support while remaining-premise lookup uses `TT = u64::MAX` (`horn_fires` → `search_prems(..., u64::MAX)`; `live_horns` uses `is_live_at(u64::MAX, vt)`). Fixture: `TtAnchor` 2014, then Assert A–B and B–C (`valid_from` 2010), then `TtAnchor` 2020. Observed today: derived `relatedTo(A,C)` is live on the current fold; `as_known_on(2014)` hides A–B and the derived row; `as_known_on(2020)` shows A–B. Decision pending whether Horn should honour the knowledge cut. D013-1 unchanged. Cell remains **Proposed**.

### Clarification (2026-10-03; Proposed)

(a) `as_of(vt)` = `live_at(u64::MAX, vt)` (`fold_query.rs`) is **SUPPORT-LIVE**: a Horn-derived fact is not retracted when its premise support is retracted. `rule_follow_ons` (`eval.rs`) handles `Assert` / `Behavior` / `Retract` only; `Correct` / `CorrectInterval` do not trigger Horn re-derivation. Derivation eligibility is a separate API `derivation_eligible_at` (`provenance.rs`). (b) Horn fires at TT=`u64::MAX` (`search_prems` / `live_horns`); `as_known_on` never re-derives. (a) and (b) stay open for M018 S02/S03 (HORN-KC-01, ASOF-ELIG-01, HORN-CORR-01) — **no option chosen**. (c) Gap-review F2 EventId targeting: `Retract` / `Correct` on a missing or already-invalidated `event_id` is a silent no-op in `GraphFold::apply` (`fold.rs`); fail-closed is not claimed. C01 remains open; no option chosen. This subsection does not rewrite D013-1…D013-4 and does not mark this cell Accepted.

### Evidence (2026-10-05, Phase 62 / M019 S07; Proposed)

Seeded list-scan model (`SEM-bitemporal-model`, `m019_sem_model.rs`) agrees with `GraphFold::live_at` / `as_of` on random Assert/Retract/Correct/CorrectInterval logs and on the FF5 statute fixture. Differential persist rebuilds and CSR cuts are ADR-041/060. Still open: Horn@TT=`MAX` (HORN-KC-01), `as_of` vs derivation eligibility (ASOF-ELIG-01), C01. This subsection does not rewrite D013-1…D013-4. Status stays **Proposed**.

### Verdict (2026-10-06; Phase 46 / M018 S02; Proposed)

(a) C01 sunset / `tempus regit actum` is a **pack stamp** (typed pack annotation), not core algebra. Forbidden: fifth axis (D013-1), Horn body, treating `valid_to` as application-end. Oracle remains `fixg01_c01_sunset_inexpressible_as_horn`. SPIKE-LAW-01 waits for a pack. Out of wedge for *core*.

HORN-KC ingest-at-head: Horn fires at log-head TT (`search_prems` / `live_horns` at `u64::MAX`). `as_known_on` never re-derives; a derived support minted after a cut is hidden by TT like any Assert. `known04_horn_fires_when_premise_hidden_on_early_cut` is the killed pin of this rule (ingest mint + `as_known_on` TT hide). Reject “must not mint at a historical cut” as an ingest-time requirement.

ASOF: `as_of` = `live_at(u64::MAX, vt)` stays **support-live**. Eligibility is `derivation_eligible_at`. Query still lists ineligible live triples (`asof_elig01_as_of_contains_derived_when_ineligible`). Reject “`as_of` must hide ineligible rows.”

This subsection does not rewrite D013-1…D013-4. Cell remains **Proposed**.

### Evidence (2026-10-06; Phase 47 / M018 S03; Proposed)

Derived Horn EventIds are hash-stable (`Event::stable_derived`, prefix `kutha-derived-id`). User Assert stays `Event::new` UUID v7. `Correct` / `CorrectInterval` of a Horn premise `retract_derived` then `horn_fires` on the corrected triple; stale derived is not live at the post-correct cut. Oracles: `det02_two_ingest_derived_event_ids_equal`, `horn_correct_derived_not_live_after_premise_correct`, `horn_correct_interval_derived_not_live_after_patch`. This subsection does not rewrite D013-1…D013-4. Cell remains **Proposed**.

### Evidence (2026-10-07; Phase 69 / M021 S02; Proposed)

`Op::Retract { event_id }` invalidates **every** live Fact whose `event_id` matches (CorrectInterval may mint prefix/patch/suffix residuals sharing one id). `fact_index_by_event_id` remains first-live for Correct / CorrectInterval targeting. Oracles: `retract_correct_interval_event_id_clears_all_residuals`, `retract_assert_event_id_still_clears_single_row`. D013-1…D013-4 unchanged. Cell remains **Proposed**.

### Clarification (2026-10-08; Phase 84 / M025 S04; Proposed)

**Demo binary + real-text fixtures (not pack thaw):**

1. `examples/statute_as_of` answers "norm in force on D, as known on W" via `Runtime::norm_view` (prints typed envelope).
2. Three fixture excerpts under `crates/kutha-runtime/fixtures/statute/` are test data only (`m016` pattern); `I-legal-pack-not-next` stays.
3. Oracles: `m025_demo_fixtures.rs`. Not ADR-070 Cypher; not ADR-090 ontology thaw.

D013-1…D013-4 unchanged. Cell remains **Proposed**.

### Clarification (2026-10-08; Phase 83 / M025 S03; Proposed)

**G-B01 / G-B04 / G-C10 / R-22 — legal answer envelope `norm_view`:**

1. `Runtime::norm_view(vt, wall)` resolves knowledge TT via greatest `TtAnchor` ≤ wall (`GraphFold::knowledge_tt_at`), then `live_at(tt, vt)`.
2. Behavior-derived facts require `derivation_eligible_at`; Assert facts pass. Live ∩ eligible is the facade's job — `as_of` and ASOF-ELIG-01 stay support-live.
3. Typed outcomes: `Ok(NormTriple…)` / `Conflict` (both polarities) / `Abstain` (live but none eligible) / `Unknown` (empty force∩knowledge) / `MissingAnchor` (no anchor). No Incomplete in this slice.
4. Cross-cite ADR-090 TR-06; pack ontology stays frozen.

Oracles: `m025_norm_view.rs`. D013-1…D013-4 unchanged. Cell remains **Proposed**.

### Clarification (2026-10-08; Phase 82 / M025 S02; Proposed)

**G-C09 / G-B02 — clock-safe public facade:**

1. `Runtime::as_of` / `GraphFold::as_of` take [`ValidTime`] (not `impl Into<ValidTime>`).
2. `Runtime::as_known_on` / `GraphFold::as_known_on` take [`WallTime`] (not `impl Into<WallTime>`).
3. Bare `u64` and wrong-role clocks (`as_of(WallTime)`, `as_known_on(ValidTime)`) fail at compile time (trybuild `ui_clock_facade`).
4. `Op` / `Fact` / `Event` clock fields stay `u64`. Other cuts (`live_at`, `known_at`, CSR) may still take `impl Into` until a later slice. No fifth axis.

Oracles: `ui_clock_facade` / `tests/ui/as_of_rejects_*` / `as_known_on_rejects_*`. D013-1…D013-4 unchanged. Cell remains **Proposed**.

### Clarification (2026-10-08; Phase 81 / M025 S01; Proposed)

**G-A08 / G-C03 — `as_known_on` is TT-only by design (pin, not a fix):**

1. `GraphFold::as_known_on(wall)` resolves the greatest `Op::TtAnchor` with `wall` ≤ D, then `known_at(tt)` / `Fact::is_known_at` — **no ValidTime filter**.
2. A fact ingested at that TT with `valid_from` in the future relative to a VT query can appear in `as_known_on` while absent from `as_of(vt)` (TR-02 trap: knowledge ≠ force).
3. Closed past VT intervals also remain visible at a knowledge cut (KNOWN-02 / Phase 34). Combining known-on with VT force is a **facade** job (M025 S03 `norm_view`), not a change to this cut.
4. Reject “`as_known_on` must hide not-yet-in-force VT” as a core fold rule — that would collapse knowledge and force axes.

Oracles: `m025_as_known_on_vt.rs` (`as_known_on_includes_not_yet_in_force_vt_fact`, `as_known_on_keeps_closed_vt_that_as_of_sees_in_window`); prior pins `closed_vt_interval_survives_as_known_on`, `true_on_d_differs_from_known_on_d`. D013-1…D013-4 unchanged. Cell remains **Proposed**.

### Clarification (2026-10-08; Phase 79 / M024 S01; Proposed)

Fold identity for Correct/Retract by `event_id` (G-A02 / G-A05 / G-A07):

1. **Live gate:** user `Retract` / `Correct` / `CorrectInterval` require at least one **live** residual for the id; otherwise `UnknownFact` (fail-closed). Dead ids (including Retract of the original Assert after CorrectInterval) no longer silent-no-op.
2. **Multi-residual Correct:** invalidate every live residual under the id and mint **one** replacement with the Correct payload (claim-level whole correct).
3. **Multi-residual CorrectInterval:** `IntervalPatchRejected` — interval patch targets exactly one live VT window.
4. **G-A07:** after CI, clear windows via `Retract(ci_id)` (already all-live); `Retract(original)` is dead → UnknownFact.

`live_fact_indices_by_event_id` is the multi-residual selector (fold identity and, as of Phase 93, admission VT coverage). `fact_index_by_event_id` stays first-live-or-dead for single-representative lookup paths. Oracles: `m024_fold_identity.rs`. D013-1…D013-4 unchanged. Cell remains **Proposed**.

### Verdict (2026-10-07; Phase 68 / M021 S01; Proposed)

Confirm Phase 46: C01 is pack stamp (a). Pack expresses application-end as ordinary `Assert` (e.g. `appliesTo` with its own VT) — not Horn Const, not a fifth axis, not `valid_to` of the text treated as application-end by core. Oracles: `fixg01_c01_sunset_inexpressible_as_horn`, `c01_applies_to_assert_encodes_application_window`.

Confirm HORN-KC ingest-at-head and ASOF support-live (`known04_*`, `asof_elig01_*`). Matrix SEM-c01-decision / SEM-horn-kc / SEM-asof-elig move to `killed`.

TR-02 (cross-cite ADR-090): force = VT of the force/applicability fact; publication/knowledge = `TtAnchor` → `as_known_on`; case decision clock = `as_of(vt)` argument. Oracle: `tr02_force_vt_publication_anchor_decision_as_of`. No fifth peer axis in core.

This subsection does not rewrite D013-1…D013-4. Cell remains **Proposed**.
