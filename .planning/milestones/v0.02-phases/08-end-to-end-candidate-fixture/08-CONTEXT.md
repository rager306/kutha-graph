# Phase 8: End-to-end candidate fixture - Context

**Gathered:** 2026-09-30
**Status:** Ready for planning

<domain>
## Phase Boundary

Deliver M011 **S08** (leased Active Slice): **one end-to-end candidate fixture** that distinguishes preserved history, current evidence, and allowed action at named (TT, VT) cuts — FIX-01 (supports + conflict), FIX-02 (summary/action cites + stale cannot renew admission), FIX-03 (incremental maintenance agrees with clean reconstruction; discarded CSR/snapshots change no answer). Named cargo tests + governor registration (GATE-01). GATE-02 / GATE-03 are primary owners of this phase’s close discipline.

**In scope:** FIX-01, FIX-02, FIX-03, GATE-01 (pattern from Phases 4–7), GATE-02 (execute only under Active Slice **S08**), GATE-03 (freeze unstarted; honeycomb stays **Proposed**; no ADR Accepted). Compose against the semantic-contract narrative (sources `a`/`b`, proposition `P`, rule `r` → `Q`, TT t1/t2/t3, conflict variant `P'=not-P`).

**Out of this phase:** Rocks/Cypher/HNSW, ADR-050 six dictionaries, ADR-080/081 ABAC packs, legal/science packs, execution replay, full provenance polynomials, four-valued logic as a product kernel, vacuum/legal hold, journey/waiting, n-ary incidence. Do not reopen Correct / CorrectInterval / quantum outcomes / typed CSR / provenance digest contracts except as **callers** of those APIs. Do not flip honeycomb cells to Accepted.

</domain>

<decisions>
## Implementation Decisions

### Fixture packaging (Claude decided)

- **D-F1:** Ship **one shared fixture builder** plus **three named oracles** (FIX-01 / FIX-02 / FIX-03) in a new integration file (suggested `crates/kutha-runtime/tests/m011_e2e_fixture.rs`). The builder encodes the semantic-contract `a`/`b`/`P`/`Q` narrative once (t1 → t2 interval-patch on `a` → t3 withdraw `b`; optional conflict branch with `P'=not-P`). Prefer **glue over reimplementation**: call existing APIs (`claim_supported_at`, `live_support_count`, `derivation_eligible_at`, `CorrectInterval`, `typed_csr_lease_at` / untyped CSR drop-rebuild, `provenance_fingerprint` / `rule_version`, persist/open) rather than copying prior test bodies. Do **not** invent a second SoT. Do **not** require re-running every prior `m011_*` file as the S08 gate — prior suites stay regression; S08 owns the composed narrative. — **Reversibility:** reversible — test layout

### FIX-02 summary / action cites (Claude decided)

- **D-F2:** Persist **justification / admission citation** rows as an **authoritative durable sidecar** beside the log (same plane pattern as `quantum_outcomes.jsonl` — write on `store::persist`, load on `store::open`; **not** a droppable lease; discarding `snapshot.json` must not erase cites). Working names: file `justifications.jsonl`, row type `Justification` / `AdmissionCite`, API `record_justification` / `check_admission` (planner may refine). Each row must cite at least: derived/target claim or action id, **source claim_ids / event ids**, **`rule_version`**, and the **(TT, VT) cut** used. Do **not** add a fold-projected graph Fact for “summary text”. Do **not** put cites only in RAM or only in CSR. **Stale cannot renew admission:** `check_admission` (or equivalent) **fails closed** when cited supports are no longer live, `derivation_eligible_at` is false, or cited `rule_version` no longer matches the live Behavior pin — caller must **mint a new justification after reevaluation**; replaying an old row must not grant admission. Thin admission only — **no** ADR-051/080 ABAC pack. — **Reversibility:** costly — once store directories carry the sidecar, open/persist contracts depend on it (same class as D-O1)

### FIX-01 conflict variant (Claude decided)

- **D-F3:** **Preserve both sides** as independent supports/claims (no implicit supersession). Conflict variant: after `a` proposes `P'=not-P` (or opposite object) while `b` still supports `P` on the overlapping VT, both remain distinguishable at the named cut. Expose a **thin evidence/conflict report** for the fixture (working name `conflict_report_at` / evidence summary returning positive vs negative support sets, aligned with ADR-013’s proposed `(positive_supports, negative_supports)` view) — **report**, do not pick a winner. Do **not** install four-valued logic, ADR-093 ontology, or admission policy dictionaries. — **Reversibility:** reversible — query/report API; fold already keeps independent supports

### FIX-03 incremental vs reconstruct (Claude decided)

- **D-F4:** At each named cut used by the fixture, assert agreement on: live **values** (`as_of` / `live_at` as appropriate), **active supports** (`claim_supported_at` / counts), **derivation eligibility** for `Q`, and **completeness** for this exact contract (no silent hole). Paths to compare: (1) incremental Runtime after the narrative mutations; (2) clean reconstruct via `GraphFold::replay(log)` and/or `store::persist` → `open` **without** snapshot; (3) after **discarding** CSR leases (typed + untyped) and rebuilding — answers unchanged. Do **not** require incremental IVM (DBSP) or GraphBLAS. Do **not** treat approximate HNSW quality as in-scope. — **Reversibility:** reversible — oracle scope

### Governor and process inheritance (Claude decided)

- **D-F5:** Inherit Phase 2 **D-G1…D-G3** and **D-10 / D-11 / D-15**, and Phase 4–7 wave-close: every execute wave runs `uv run kutha-gov ci` (HIGH stops) + `explain trajectory` + D-10 Trajectory in SUMMARY; cargo on crate-touching waves. — **Reversibility:** reversible

- **D-F6:** Named cargo tests registered in governor (FSM observe + check/bridge needles), GATE-01, `ci` HIGH 0. Active Slice remains **S08** for the phase (GATE-02 — primary owner). Freeze / Proposed honeycomb (GATE-03 — primary owner); do **not** mark honeycomb cells Accepted at close. Do not edit `.kutha/STATE.md` during delivery. — **Reversibility:** reversible

- **D-F7:** Do not change Assert/Retract/Correct/CorrectInterval fold semantics, quantum outcome disposition rules, typed/untyped CSR build rules, or `replay_check` / `provenance_fingerprint` mix — S08 **composes** them. — **Reversibility:** reversible

### Claude's Discretion

User said «реши эти вопросы разумно и при необходимости доисследуй» and required **codebase-memory-mcp** during execution → D-F1…D-F4 (plus D-F5…D-F7). Planner/researcher/executor MUST use CBM (`list_projects` first; `search_graph` / `trace_path` / `get_code_snippet`; `check_index_coverage` on touched paths — note `quantum.rs` / `event.rs` may show `metadata_changed` until parent reindexes; `detect_changes` after edits). Exact sidecar field names, conflict report shape, and fixture constants are discretionary; must not thaw freeze, Accept ADRs, or expand into Rocks/Cypher/HNSW/ABAC packs.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### GSD overlay
- `.planning/ROADMAP.md` — Phase 8 goal, success criteria, S08 lease
- `.planning/REQUIREMENTS.md` — FIX-01, FIX-02, FIX-03, GATE-01…03
- `.planning/PROJECT.md` — v0.02 Semantic core close; lease rule
- `.planning/STATE.md` — GSD position (not harness lease)
- `.kutha/STATE.md` — Active Slice **S08**; M011; freeze; `L_delivery=M011-S07-done`
- `.kutha/ROADMAP.md` — M011 S08 “After this” line
- `.planning/phases/07-provenance-and-rule-version-check/07-CONTEXT.md` — provenance / rule_version; deferred FIX-02 fence
- `.planning/phases/06-typed-csr-lease/06-CONTEXT.md` — typed vs untyped CSR
- `.planning/phases/05-persisted-quantum-outcome/05-CONTEXT.md` — durable sidecar pattern (D-O1) to mirror for justifications
- `.planning/phases/04-partial-correction-with-residual-intervals/04-CONTEXT.md` — CorrectInterval residuals (obs 3)
- `.planning/phases/05-persisted-quantum-outcome/05-CONTEXT.md` — GATE inheritance

### Product contract
- `docs/architecture/semantic-contract-validation.md` — candidate fixture narrative; observations 1–5; stop condition
- `docs/ADR/ADR-013-bitemporal-facts-invalidation.md` — no implicit supersession; residual intervals; `(positive_supports, negative_supports)` conflict view (Proposed)
- `docs/ADR/ADR-011-lean-event-schema-lineage.md` — claim/support identity; typed dependency links
- `docs/ADR/ADR-014-cascade-budgets-quantum-receipts.md` — outcomes sidecar cousin; Ok ≠ completion
- `docs/ADR/ADR-040-materialization-plugin-protocol.md` — exact lease rebuild; approximate out of scope
- `docs/ADR/ADR-060-strict-replay.md` — state vs provenance vs execution (execution still out)
- `docs/ADR/ADR-012-snapshots-tiers-vacuum.md` — snapshot ≠ SoT; discard leases
- `docs/ADR/ADR-000-foundation-locks.md` — D1–D10

### Implementation anchors
- `crates/kutha-runtime/src/quantum.rs` — `derivation_eligible_at`, outcomes buffer, `provenance_fingerprint`, `typed_csr_lease_at`
- `crates/kutha-runtime/src/fold.rs` — `claim_supported_at`, `live_support_count`, `fingerprint`, CorrectInterval
- `crates/kutha-runtime/src/store.rs` — `persist` / `open`, `OUTCOMES_REL` pattern for new sidecar
- `crates/kutha-runtime/src/csr.rs` — typed / untyped leases
- `crates/kutha-runtime/tests/m011_claim_supports.rs` — supports + P→Q eligibility
- `crates/kutha-runtime/tests/m011_partial_correction.rs` — VT residuals
- `crates/kutha-runtime/tests/m011_quantum_outcome.rs` — persist/open outcomes
- `crates/kutha-runtime/tests/m011_typed_csr.rs` — CSR drop/rebuild
- `crates/kutha-runtime/tests/m011_provenance.rs` — rule_version / caused_by
- `AGENTS.md` — planes, Active Slice lease, freeze; CBM as structural evidence plane

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable assets (CBM + source 2026-09-30)
- `GraphFold::claim_supported_at` / `live_support_count` — FIX-01 support observations (`search_graph`)
- `Runtime::derivation_eligible_at` — P→Q eligibility at cut (source `quantum.rs` ~556–578; CBM snippet for this symbol was stale — prefer Read)
- `Op::CorrectInterval` + `m011_partial_correction` — observation 3 residuals
- `quantum_outcomes.jsonl` + `PersistedQuantumOutcome` / `store::OUTCOMES_REL` — **pattern** for D-F2 sidecar
- `TypedCsrLease` / `CsrLease` drop-rebuild — FIX-03 lease discard
- `provenance_fingerprint` + `Behavior.rule_version` — FIX-02 cite pins
- `store::persist` / `open` without snapshot — FIX-03 reconstruct path (M010)

### Established patterns
- Event log = SoT; sidecars that outlive snapshot are rare and explicit (Define, outcomes)
- Named `m011_*` tests + FSM observe needles
- Thin oracles over full honeycomb delivery
- Conflict = preserve both sides; admission is policy, not automatic supersession (ADR-013)

### Integration points
- New sidecar + Runtime APIs beside outcomes; store persist/open hooks
- New `m011_e2e_fixture.rs` (or similar) with three named tests
- Governor GATE-01 trio (`m011-e2e` / `B-m011-e2e` suggested)
- Honeycomb evidence append only; `map: Proposed`

### CBM note for execute
- `list_projects` first. Several product paths after S07 show `metadata_changed` — Read source; parent reindexes only if missing/stale/asked. Do not trust graph alone for `derivation_eligible_at` line ranges until coverage is fresh.

</code_context>

<specifics>
## Specific Ideas

- Suggested GATE check id: `m011-e2e` / bridge `B-m011-e2e`; needles = exact FIX-01/02/03 test fn names (planner may shorten).
- Suggested test names (planner may adjust): `e2e_fixture_supports_and_conflict_at_named_cuts`, `e2e_justification_cites_sources_and_rejects_stale_admission`, `e2e_incremental_matches_reconstruct_after_discarding_leases`.
- Prefer mirroring outcomes sidecar field discipline (serde JSONL rows) over inventing a second Op enum for natural-language summaries.

</specifics>

<deferred>
## Deferred Ideas

- Full ADR-051/080 ABAC / multi-tenant admission packs
- Four-valued logic kernel / ADR-093 ontology
- Execution replay / CA-cache (ADR-060 obligation 3)
- Provenance polynomials
- Vacuum + legal hold (ADR-012 product vacuum)
- Journey/waiting, n-ary incidence, shared CSR cost accounting (semantic-contract probes beyond S08 stop condition)
- Marking honeycomb cells Accepted (explicit milestone review only)

</deferred>

---

*Phase: 8-End-to-end candidate fixture*
*Context gathered: 2026-09-30*
