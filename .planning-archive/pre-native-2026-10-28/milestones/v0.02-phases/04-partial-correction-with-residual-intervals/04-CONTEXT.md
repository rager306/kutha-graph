# Phase 4: Partial correction with residual intervals - Context

**Gathered:** 2026-09-29
**Status:** Ready for planning

<domain>
## Phase Boundary

Deliver M011 **S04** (leased Active Slice): an **explicit interval-patch** correction that leaves residual versions on both sides of the patched VT range, while **whole-version `Op::Correct` stays behaviorally unchanged**. Named cargo test proves the fixture oracle (source `a` keeps VT 2012 and 2021 residuals at the same TT cut). Register the test in the governor (GATE-01).

**In scope:** CORR-01, CORR-02, GATE-01 (primary owner for named-test pattern). Half-open VT intervals per ADR-013.

**Out of this phase:** S05 quantum outcomes, S06 typed CSR, S07 provenance-only checks, S08 full end-to-end fixture / conflict admission, Rocks/Cypher/HNSW, ADR-050 six dictionaries, legal/science packs. Do not treat honeycomb cells as Accepted.

</domain>

<decisions>
## Implementation Decisions

### Interval-patch API shape (Claude decided)

- **D-C1:** Add a **new `Op` variant** for explicit interval patch (working name `CorrectInterval`; exact Rust identifier is planner/researcher discretion). Payload must name the target fact (`fact_seq`) and the **patch VT range** (`patch_from` / `patch_to`, half-open) plus the replacement `object` for that range. **Do not** overload or reinterpret today's `Op::Correct` fold path so that a narrower `valid_from`/`valid_to` silently invents residuals. Whole-version `Correct` remains: invalidate the whole live fact and emit one replacement version (CORR-02). — **Reversibility:** costly — once events with the new variant are logged/WAL'd, removing the variant needs a migration or forever-compat decode

### Residual identity (Claude decided)

- **D-C2:** After an interval patch on a live fact spanning a wider VT, fold must leave **residual Fact rows** for the uncovered prefixes/suffixes (e.g. patch `[2015,2020)` inside `[2010,∞)` → residuals `[2010,2015)` and `[2020,∞)`). Residuals keep the **same `claim_id`** (and subject/relation) as the corrected fact; they continue the same support lineage (M011 S01). The mid-interval replacement is a new Fact at the same `claim_id` with the patch VT and new object. Do not mint a new claim for residuals. — **Reversibility:** reversible — fold semantics; claim_id rule matches existing Correct

### Fixture width for S04 (Claude decided)

- **D-C3:** Ship a **minimal residual oracle** only: arrange one support/fact for source `a` over a wide VT, apply explicit interval patch covering `[2015,2020)` (or equivalent fixture constants), then assert at one TT cut that `as_of`/live cuts at VT **2012** and **2021** still see residual versions of `a`, and that the patched interior matches the replacement. **Do not** implement the full semantic-contract t1/t2/t3 narrative, dual-source `b`, conflict `P'=not-P`, summary/action records, or incremental-vs-reconstruct parity in this phase (those are S08 / FIX-*). Reuse existing claim/support and FF5-style cut helpers where they fit. — **Reversibility:** reversible — test scope only

### Admission of replacement P' (Claude decided)

- **D-C4:** S04 records the replacement object on the patched interval via the interval-patch op only. **Admission policy, conflict reporting, and “allowed action”** remain out of scope — deferred to **Phase 8 / S08** (FIX-01). No new ABAC or admission dictionary in S04. — **Reversibility:** reversible — scope fence

### Governor and process inheritance (Claude decided)

- **D-C5:** Inherit Phase 2 **D-G1…D-G3** and **D-10 / D-11 / D-15** for every execute wave: `uv run kutha-gov ci` required; HIGH stops; WARN ledgered; SUMMARY trajectory block; cargo at tracer + pre-verify, and on any wave that touches `crates/`. — **Reversibility:** reversible — process gate

- **D-C6:** Named cargo test (CORR-01 residual oracle; CORR-02 regression that whole-version Correct still invents no residuals) is registered in the governor (FSM observe + check needle) so GATE-01 holds and `ci` stays 0 HIGH. — **Reversibility:** reversible

- **D-C7:** Intervals stay **half-open** `[from,to)` as in ADR-013; infinity is `valid_to: None` as today. Reject or no-op an interval patch that does not actually intersect the live fact's VT (exact reject vs no-op is planner discretion; must be fail-closed and tested). — **Reversibility:** reversible

### Claude's Discretion

User said «реши эти вопросы разумно» for gray areas 1–4 → D-C1…D-C4 (plus D-C5…D-C7 companions). Planner may refine Rust field names and exact fixture constants; must not change Correct whole-version semantics, thaw freeze, or expand into S05–S08 delivery.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### GSD overlay
- `.planning/ROADMAP.md` — Phase 4 goal, success criteria, lease note
- `.planning/REQUIREMENTS.md` — CORR-01, CORR-02, GATE-01…03
- `.planning/PROJECT.md` — v0.02 Semantic core close; lease rule
- `.planning/STATE.md` — GSD position (not harness lease)
- `.kutha/STATE.md` — Active Slice **S04**; M011; freeze; lifecycles
- `.kutha/ROADMAP.md` — M011 S04 “After this” line
- `.planning/milestones/v0.01-phases/02-honest-harness-and-freeze/02-CONTEXT.md` — D-G1…G3, D-10/11/15
- `.planning/milestones/v0.01-phases/03-lease-gated-next-slice/03-CONTEXT.md` — lease discipline (S04 is now the leased slice)

### Product contract
- `docs/architecture/semantic-contract-validation.md` — observation 3 (VT 2012/2021 residuals); remaining decisions
- `docs/ADR/ADR-013-bitemporal-facts-invalidation.md` — whole-version vs interval correction; half-open intervals
- `docs/ADR/ADR-011-lean-event-schema-lineage.md` — claim/support identity
- `docs/ADR/ADR-000-foundation-locks.md` — D1–D10

### Implementation anchors
- `crates/kutha-common/src/event.rs` — `Op::Correct` today
- `crates/kutha-runtime/src/fold.rs` — whole-version Correct fold path
- `crates/kutha-runtime/tests/` — pattern for named M011 / FF5 tests
- `AGENTS.md` — planes, Active Slice lease, freeze

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `Op::Correct { fact_seq, object, valid_from, valid_to }` — whole-version path to preserve unchanged (CORR-02)
- `GraphFold::apply` Correct arm — invalidate old + push one new Fact with same `claim_id`
- M011 claim_id / support tests — lineage patterns for residuals keeping `claim_id`
- FF5 / `as_of` cut helpers — pattern for VT 2012 vs 2021 assertions

### Established Patterns
- Event log = SoT; fold is deterministic picture
- Half-open VT; `valid_to: None` = open end
- New product surface = leased slice + named test + governor needle
- Fail-closed: unknown / non-intersecting ops must not silently invent truth

### Integration Points
- New Op variant in `kutha-common` + serde/WAL round-trip
- Fold arm in `kutha-runtime` `fold.rs`
- Emit/admit path in quantum if Correct is gated there
- New integration test under `crates/kutha-runtime/tests/`
- Governor check/observe registration (YAML dictionaries)

</code_context>

<specifics>
## Specific Ideas

- Fixture numbers from the semantic-contract note: residual checks at VT **2012** and **2021** after correcting interior `[2015,2020)` (or equivalent constants documented in the test).
- ADR-013 wording: do not silently reinterpret today's Correct API as an interval-patch operator.

</specifics>

<deferred>
## Deferred Ideas

- Full semantic-contract observations 1–2, 4–5 and conflict admission → Phase 8 / S08
- Persisted quantum outcome / crash resume → Phase 5 / S05
- Typed CSR labels and support multiplicity → Phase 6 / S06
- Provenance/rule-version check without execution replay → Phase 7 / S07
- Thin ADR-050 dictionaries, Rocks, Cypher, legal pack → later harness milestones

</deferred>

---

*Phase: 4-Partial correction with residual intervals*
*Context gathered: 2026-09-29*
