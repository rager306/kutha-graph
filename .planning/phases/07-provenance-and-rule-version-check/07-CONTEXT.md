# Phase 7: Provenance and rule-version check - Context

**Gathered:** 2026-09-30
**Status:** Ready for planning

<domain>
## Phase Boundary

Deliver M011 **S07** (leased Active Slice): **provenance verification** that detects a change to only a Behavior `caused_by` reference (PROV-01) or only a pinned **rule version** (PROV-02) even when the **state** fold fingerprint still matches — **without** execution replay. Named cargo tests + governor registration (GATE-01).

**In scope:** PROV-01, PROV-02, GATE-01 (pattern from Phases 4–6). ADR-060 clarification obligations 1 (state) vs 2 (provenance); keep obligation 3 (execution replay) unimplemented.

**Out of this phase:** S08 end-to-end fixture / summary-action cites (FIX-02), full provenance polynomials, execution replay / CA-cache rerun, ADR-050 meta_prompt binding, Cypher/HNSW/Rocks, legal pack. Do not treat honeycomb cells as Accepted. Do not change Correct / CorrectInterval, quantum outcomes sidecar, or typed/untyped CSR semantics.

</domain>

<decisions>
## Implementation Decisions

### PROV-01 caused_by oracle (Claude decided)

- **D-P1:** Primary oracle is **swap to another valid prior**, not ghost. Fixture: at least two earlier events (e.g. Assert A1 and Assert A2); emit `Op::Behavior` with `caused_by = A1` and a fixed triple; build a second log identical except `caused_by = A2`. Assert `fold.fingerprint()` (or `replay_check` state path) **matches** across both, and **provenance** verification **fails** (or provenance digests differ) on the swapped log. Keep existing `replay_rejects_behavior_without_prior_cause` (ghost / absent prior → `BrokenLineage`) as regression — it does **not** satisfy PROV-01 alone, because today's `replay_check` already accepts any earlier id. Construct logs via `Runtime::from_dict_and_events` (or equivalent); do not invent a production mutate-in-place API for live SoT. — **Reversibility:** reversible — test + check surface

### Rule version carrier (Claude decided)

- **D-P2:** Add **`rule_version: String`** on `Op::Behavior` (serde `#[serde(default)]` so existing fixtures deserialize). Fold continues to **ignore** `name`, `caused_by`, and `rule_version` when projecting Facts (fingerprint stays state-only). Empty default is allowed for legacy rows; the PROV-02 oracle uses non-empty pins (e.g. `"r1"` → `"r2"`) with identical triple and `caused_by`. Do **not** put rule version only in a droppable lease or only in quantum outcomes sidecar. Do **not** implement ADR-050 six dictionaries or meta_prompt binding. — **Reversibility:** costly — once Behavior events carry `rule_version` in the log/WAL schema, removing the field needs forever-compat decode or a migration

### Verification API shape (Claude decided)

- **D-P3:** Keep **`Runtime::replay_check`** as today's **state** obligation: rebuild fold, compare `GraphFold::fingerprint`, plus `caused_by` **exists earlier** (`BrokenLineage`). Add a **separate provenance surface** (working names: `provenance_fingerprint() -> [u8; 32]` and/or `provenance_check` / `assert_provenance_matches(expected)`): hash Behavior lineage fields in log order — at least `(event.id, caused_by, name, rule_version)` — so PROV-01/02 oracles fail when only those fields change while state fingerprint matches. Do **not** merge execution replay into this API. Do **not** weaken `replay_check` to skip BrokenLineage. Exact error variant names are planner discretion (`ProvenanceMismatch` or compare digests in tests). — **Reversibility:** reversible — additive API; fingerprint mix is local

### Fixture width (Claude decided)

- **D-P4:** Ship **two named oracles** only: (1) PROV-01 — swap `caused_by` among valid priors, state fingerprint equal, provenance diverges; (2) PROV-02 — change only `rule_version`, state fingerprint equal, provenance diverges; both confirm `replay_check` still `Ok` on the tampered logs when priors exist (state path green). Suggested names (planner may shorten; GATE needles = exact fn names): `provenance_detects_caused_by_swap_when_state_fingerprint_matches`, `provenance_detects_rule_version_change_when_state_fingerprint_matches`. Do **not** implement summary/action citation records, stale-cache admission, or the full semantic-contract t1–t3 narrative (S08 / FIX-*). — **Reversibility:** reversible — test scope

### Governor and process inheritance (Claude decided)

- **D-P5:** Inherit Phase 2 **D-G1…D-G3** and **D-10 / D-11 / D-15**, and Phase 4–6 wave-close: every execute wave runs `uv run kutha-gov ci` (HIGH stops) + `explain trajectory` + D-10 Trajectory in SUMMARY; cargo on crate-touching waves. — **Reversibility:** reversible

- **D-P6:** Named cargo tests registered in governor (FSM observe + check/bridge needles), GATE-01, `ci` HIGH 0. Active Slice remains **S07** for the phase (GATE-02). Freeze / Proposed honeycomb (GATE-03). Do not edit `.kutha/STATE.md` during delivery. — **Reversibility:** reversible

- **D-P7:** Do not change Assert/Retract/Correct/CorrectInterval fold semantics; Behavior still projects the same Fact triple. Typed CSR and quantum outcomes stay untouched. — **Reversibility:** reversible

### Claude's Discretion

User said «реши эти вопросы разумно и при необходимости доисследуй» and required **codebase-memory-mcp** during execution → D-P1…D-P4 (plus D-P5…D-P7). Planner/researcher/executor MUST use CBM (`list_projects` first; `search_graph` / `trace_path` / `get_code_snippet`; `check_index_coverage` on touched paths; `detect_changes` after edits). Exact hash field order, error enums, and fixture term constants are discretionary; must not thaw freeze, expand into S08 FIX-*, or implement execution replay.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### GSD overlay
- `.planning/ROADMAP.md` — Phase 7 goal, success criteria, S07 lease
- `.planning/REQUIREMENTS.md` — PROV-01, PROV-02, GATE-01…03; out-of-scope: full polynomials / execution replay
- `.planning/PROJECT.md` — v0.02 Semantic core close; lease rule
- `.planning/STATE.md` — GSD position (not harness lease)
- `.kutha/STATE.md` — Active Slice **S07**; M011; freeze; `L_delivery=M011-S06-done`
- `.kutha/ROADMAP.md` — M011 S07 “After this” line
- `.planning/phases/06-typed-csr-lease/06-CONTEXT.md` — GATE / trajectory inheritance; out-of-scope fence for provenance
- `.planning/phases/05-persisted-quantum-outcome/05-CONTEXT.md` — GATE pattern; outcomes not the rule-version carrier
- `.planning/phases/04-partial-correction-with-residual-intervals/04-CONTEXT.md` — GATE-01 trio pattern

### Product contract
- `docs/architecture/semantic-contract-validation.md` — probe “Change only a causal reference or rule version”; observation 4 deferred to S08
- `docs/ADR/ADR-060-strict-replay.md` — three verification obligations; clarification on `replay_check` limits
- `docs/ADR/ADR-011-lean-event-schema-lineage.md` — `caused_by` / typed dependency links; not provenance polynomials in P0
- `docs/ADR/ADR-040-materialization-plugin-protocol.md` — exact vs approximate; separate state/provenance/execution
- `docs/ADR/ADR-014-cascade-budgets-quantum-receipts.md` — receipts ≠ this slice’s rule pin
- `docs/ADR/ADR-000-foundation-locks.md` — D1–D10

### Implementation anchors
- `crates/kutha-common/src/event.rs` — `Op::Behavior { name, caused_by, … }` (add `rule_version`)
- `crates/kutha-runtime/src/fold.rs` — Behavior arm ignores lineage fields; `GraphFold::fingerprint` is Facts-only
- `crates/kutha-runtime/src/quantum.rs` — `Runtime::replay_check` (state + BrokenLineage); `derivation_eligible_at`
- `crates/kutha-runtime/tests/m011_claim_supports.rs` — ghost `caused_by` oracle; P→Q eligibility (not PROV-01/02)
- `AGENTS.md` — planes, Active Slice lease, freeze; CBM as structural evidence plane

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable assets (CBM-verified 2026-09-30)
- `Runtime::replay_check` — state fingerprint + exists-earlier `caused_by` only (`get_code_snippet`; inbound callers include FF5/FF6/H2/H4/M010/M011 suites — do not break contract)
- `GraphFold::fingerprint` — hashes live Fact rows only; Behavior `name` / `caused_by` / (future) `rule_version` do not affect it (`get_code_snippet`)
- `GraphFold::apply` Behavior arm — projects subject/relation/object/VT; lineage fields ignored via `..` (`get_code_snippet`)
- `Runtime::from_dict_and_events` + `m011_claim_supports::replay_rejects_behavior_without_prior_cause` — pattern for synthetic Behavior logs
- Phase 4–6 GATE-01 YAML trio — copy pattern for `m011-provenance` (or similar)

### Established patterns
- State replay ≠ provenance ≠ execution replay (ADR-060)
- Named `m011_*` integration tests + FSM `observe_cargo.required` needles
- Fail-closed lineage: ghost cause → `BrokenLineage` (already shipped; insufficient for PROV-01)

### Integration points
- Additive field on `Op::Behavior` in `kutha-common`; update serde round-trips / any Behavior construct sites
- New provenance API beside `replay_check` in `quantum.rs`
- New integration test file under `crates/kutha-runtime/tests/` (e.g. `m011_provenance.rs`)
- Governor check/observe/bridge registration

### CBM note for execute
- Project `kutha-graph` coverage on `event.rs` / `quantum.rs` / `fold.rs` / `m011_claim_supports.rs`: `metadata_match` at discuss time. Re-check coverage after touching paths; parent reindexes only if stale/missing/asked. Executor must `list_projects` first and use graph before Grep for symbols.

</code_context>

<specifics>
## Specific Ideas

- Prefer hashing `(event.id, caused_by, name, rule_version)` so renaming a behavior or swapping cause both move the provenance digest without touching Fact state.
- Suggested GATE check id: `m011-provenance` / bridge `B-m011-provenance` (planner may adjust; needles = exact test fn names).
- Do not reuse Behavior.`name` alone as the rule version — keep name as behavior id and `rule_version` as the pin.

</specifics>

<deferred>
## Deferred Ideas

- Summary/action records citing source revisions + rule version (FIX-02 / Phase 8)
- Execution replay / CA-cache rerun (ADR-060 obligation 3)
- Full provenance polynomials / recursive provenance representation (ADR-011)
- ADR-050 meta_prompt_version / six dictionaries
- Cross-branch wrong-claim references (semantic-contract probe; ADR-061)

</deferred>

---

*Phase: 7-Provenance and rule-version check*
*Context gathered: 2026-09-30*
