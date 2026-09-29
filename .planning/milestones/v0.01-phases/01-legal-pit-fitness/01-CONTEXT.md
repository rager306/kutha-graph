# Phase 1: Legal PIT fitness - Context

**Gathered:** 2026-09-29
**Status:** Ready for planning

<domain>
## Phase Boundary

Prove that existing legal/process fitness already in crates stays green: FF5 AS OF PIT, FF6 fail-closed relations, M010 semantic open, M011 claim/support + thin P→Q, and H2/H4 tenant AS OF dogfood. This phase is **verification-first** — no new product capability, no legal pack, no freeze thaw, no Active Slice work.

</domain>

<decisions>
## Implementation Decisions

### Verification command surface
- **D-01:** Phase 1 hard gate is `cargo test --workspace --offline` (same args as harness `observe.py`). — **Reversibility:** reversible — command string only
- **D-02:** Named FIT tests are an **evidence map** in SUMMARY/VERIFICATION (test fn → FIT-id), not a cargo `--test` / `--exact` CLI filter. — **Reversibility:** reversible
- **D-03:** `uv run kutha-gov ci` and `precommit` are **not** Phase 1 must_haves. Full governor quantum belongs to Phase 2 (GOV-*). — **Reversibility:** reversible

### Evidence & checkbox hygiene
- **D-04:** Phase evidence SoT is `*-VERIFICATION.md` / plan SUMMARY with the FIT→test map. Do not require mid-execute edits to REQUIREMENTS checkboxes. — **Reversibility:** reversible
- **D-05:** When Phase 1 verification passes, update REQUIREMENTS.md FIT-01…05 to `[x]` in one batch (verify/complete step), plus STATE progress — not piecemeal during tasks. — **Reversibility:** reversible

### Red fitness policy
- **D-06:** Any failing hard-gate FIT blocks phase completion (hard gate). No “green with caveat” for FIT-01…05. — **Reversibility:** reversible — policy only
- **D-07:** If red: stop, record blocker in STATE with failing test name; do not quarantine or skip without an explicit human lease outside this CONTEXT. — **Reversibility:** reversible

### H2/H4 depth bound
- **D-08:** FIT-05 is satisfied only by existing tests in `crates/kutha-runtime/tests/h2_harness_tenant.rs` and `h4_process_allows.rs` (plus workspace run). No new harness assertions, no ADR-090 ontology, no legal corpus. — **Reversibility:** reversible
- **D-09:** Do not add product-crate features to “help” H2/H4; Active Slice remains None. — **Reversibility:** reversible

### Claude's Discretion
User deferred (“you decide”) on verification surface Q1–Q3 and asked Claude to decide the three follow-on gray areas reasonably. Locked defaults above: cargo-offline hard gate; evidence in VERIFICATION; hard-fail on red FIT; H2/H4 = existing tests only.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### GSD overlay
- `.planning/ROADMAP.md` — Phase 1 goal and success criteria
- `.planning/REQUIREMENTS.md` — FIT-01…FIT-05
- `.planning/PROJECT.md` — core value and locked decisions
- `.planning/STATE.md` — GSD position (cite harness lease)
- `.kutha/STATE.md` — harness lease / freeze / Active Slice None

### Fitness & tests
- `crates/kutha-runtime/tests/ff5_legal_pit.rs` — FF5 / FIT-01
- `crates/kutha-runtime/tests/ff6_allowlist.rs` — FF6 / FIT-02
- `crates/kutha-runtime/tests/m010_semantic_open.rs` — M010 / FIT-03
- `crates/kutha-runtime/tests/m011_claim_supports.rs` — M011 / FIT-04
- `crates/kutha-runtime/tests/h2_harness_tenant.rs` — H2 / FIT-05
- `crates/kutha-runtime/tests/h4_process_allows.rs` — H4 / FIT-05
- `.planning/codebase/TESTING.md` — cargo vs kutha-gov surfaces
- `scripts/kutha_gov/observe.py` — default `cargo test --workspace --offline`

### Ingest / architecture fences
- `.planning/INGEST-CONFLICTS.md` — ADR-002 vs stca-guide; typed Op
- `docs/ADR/ADR-002-stca-paradigm.md` — STCA paradigm
- `docs/ADR/ADR-010-event-log-runtime-quantum.md` — typed Op write surface
- `AGENTS.md` — GSD-first; two planes; freeze

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- Integration test files above already encode FIT success criteria
- Governor observe already runs workspace cargo offline — Phase 1 reuses that command without requiring full `ci`

### Established Patterns
- Fitness fn names are snake_case and stable (`ff5_as_of_t1_differs_from_as_of_t2_on_statute_log`, etc.)
- Bridges/`observe-required-fn` may cite Rust test names — do not rename lightly

### Integration Points
- Phase 1 plans should wire must_haves to cargo + evidence map only
- Phase 2 owns `kutha-gov ci` honesty (GOV-01…)

</code_context>

<specifics>
## Specific Ideas

- Hard gate command string: `cargo test --workspace --offline`
- Evidence table columns: FIT-id | test file | test fn | pass/fail

</specifics>

<deferred>
## Deferred Ideas

- Full `uv run kutha-gov ci` as hard gate — Phase 2
- REQUIREMENTS checkbox mid-task updates — deferred; batch at verify
- New H2/H4 fixtures or ADR-090 work — freeze / Phase 3 lease
- File-filtered or `--exact` cargo invocation — not chosen; may revisit if workspace runtime becomes painful

None else — discussion stayed within phase scope

</deferred>

---

*Phase: 1-Legal PIT fitness*
*Context gathered: 2026-09-29*
