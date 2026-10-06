# Phase 46: C01 and knowledge-cut — Research

**Researched:** 2026-10-06  
**Domain:** ADR-013 decision record + oracle pins (no fold rewrite)  
**Confidence:** HIGH (in-repo SoT; CBM `root-kutha-graph` crates `metadata_match`)

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- **D-C01 (a) pack stamp:** C01 sunset / `tempus regit actum` is a typed pack annotation, not core algebra. Forbidden: fifth axis (D013-1), Horn body, treating `valid_to` as application-end. Oracle stays `fixg01_c01_sunset_inexpressible_as_horn`. SPIKE-LAW-01 waits for a pack. Out of wedge for *core*.
- **D-HORN-KC ingest-at-head:** Horn fires at log-head TT (`search_prems` / `live_horns` at `u64::MAX` = current). `as_known_on` never re-derives; a derived support minted after a cut is hidden by TT like any Assert. Do not invert `known04_horn_fires_when_premise_hidden_on_early_cut`; promote it from characterization to the killed pin of this rule. Reject “must not mint at a historical cut” as an ingest-time requirement.
- **D-ASOF-ELIG support-live:** `as_of` = `live_at(MAX, vt)` stays support-live (Phase 45). Retract of a premise does not retract derived supports. Eligibility is `derivation_eligible_at`. Reject “`as_of` must hide ineligible rows.” Add a named pin if missing.
- **D-SOT acknowledged:** sealed `KUTHASEG` + `manifest.json` are D1; `events.jsonl` after migrate is an export. Not a fifth clock.
- **D-SCOPE:** `eval.rs` / fold behaviour unchanged unless a pin proves a contradiction. Honeycomb stays **Proposed**. Verifier ≠ executor. Controller owns `.kutha/STATE.md`.

### Claude's Discretion
None in CONTEXT.md (discuss skipped). Honour verifier kill-tests: ADR-013 names option (a); `known04_*` not inverted; named `as_of` support-live pin; `git diff` on `eval.rs` empty; matrix SEM-c01-decision / SEM-horn-kc / SEM-asof-elig leave `blocked`.

### Deferred Ideas (OUT OF SCOPE)
DET-02, HORN-CORR-01, CASC-01 → Phase 47. TR-02 / legal pack with C01. Residual harness cleanup. Persister wiring.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| C01-01 | ADR-013 records one C01 option + named oracle / out-of-wedge | Append 2026-10-06 Verdict; keep FIXG-01 oracle |
| HORN-KC-01 | Horn vs `as_known_on` decided + named pin | Keep `known04_*` assertions; relabel characterization→killed pin |
| ASOF-ELIG-01 | `as_of` vs eligibility decided + pin | Query-plane pin; do not filter `as_of` by eligibility |
</phase_requirements>

## Summary

Phase 46 is a **docs + matrix + test-comment / one new pin** slice. Crates already implement ingest-at-`TT=MAX` Horn, `as_of`=`live_at(MAX,vt)`, and a separate `derivation_eligible_at`. Do **not** reopen options. **Primary recommendation:** append ADR-013 dated Verdict (a)+(HORN-KC ingest-at-head)+(ASOF query ≠ eligibility); rewrite three `blocked` invariant strings (today they still state rejected options); restage honeycomb evidence only; add `as_of` pin if current tests only use `claim_supported_at`.

## Architectural Responsibility Map

| Capability | Primary Tier | Secondary | Rationale |
|------------|--------------|-----------|-----------|
| C01 pack-stamp verdict | Docs / L_map | — | Core algebra stays two axes (D013-1) |
| Horn ingest vs known-on | API/runtime (unchanged) | Tests | `eval.rs` already `u64::MAX`; pin only |
| `as_of` vs eligibility | API/runtime (unchanged) | Tests | Query vs provenance walk |
| Matrix / honeycomb | Harness dictionaries | Governor `test_matrix` | Status vocab + blocked owners |

## Project Constraints (from `.cursor/rules/`)

- Chat RU; this file EN. No LifeOS. CBM: `list_projects` first; no `delete_project` / `index_repository` / `manage_adr`. GitNexus not paired (not M013 24–27). Artifact budget: ≤2 PLAN ×150, SUMMARY≤60, VERIFICATION≤60, CONTEXT≤80.

## Exact pins (planner file:line)

**ADR-013** still **Proposed**. FIXG-01 already fail-closed C01-as-Horn (`docs/ADR/ADR-013-bitemporal-facts-invalidation.md:11`, `:138-144`: *“No option is chosen.”*). D013-1 (`:38-40`): *“Core algebra is **two axes**.”* HORN-KC characterization (`:148-150`); ASOF open (`:152-154`): *“`as_of(vt)` = `live_at(u64::MAX, vt)` … **SUPPORT-LIVE**”* / *“Derivation eligibility is a separate API `derivation_eligible_at`”*. **Append** 2026-10-06 Verdict after `:156`; do not rewrite D013-1…D013-4 or delete FIXG-01.

**Code (read; do not edit src):**  
`fold_query.rs:82-84` `as_of` → `self.live_at(u64::MAX, vt)`. `eval.rs:28-57` `rule_follow_ons`: Assert/Behavior → `horn_fires`; Retract → `retract_derived`; other Ops empty. `eval.rs:129` `search_prems(..., valid_from, u64::MAX)`. `eval.rs:153` `is_live_at(u64::MAX, vt)`. `provenance.rs:122-132` `derivation_eligible_at`. `quantum.rs:563-574` delegates `as_of` / `as_known_on`.

**Oracles:** `m016_golden_fixture.rs:11-50` `fixg01_c01_sunset_inexpressible_as_horn` (keep asserts). `m017_known_on_horn.rs:1-2,17-86` `known04_horn_fires_when_premise_hidden_on_early_cut` — mint at MAX + `as_known_on(2014)` hides premise/derived; **do not invert**.

**Matrix** `docs/architecture/test-matrix.yaml:534-599` + human page `:46-55`. Statuses `_MATRIX_STATUSES` = `killed|characterization|gap|blocked|spec-only|gate` (`scripts/kutha_gov/kinds.py:2358`). `killed` forbids `kill_mutation: none` (`:2552-2566`). Blocked owners allowlist includes `C01`, `HORN-KC-01`, `ASOF-ELIG-01` (`:2363-2372`). **Keep those three rows `status: blocked`** (CONTEXT + PATTERNS). **Must rewrite invariants:** `SEM-horn-kc` *“Horn must not mint…”* (`yaml:570`) and `SEM-asof-elig` *“as_of must hide derivation-ineligible rows”* (`:592`) are the **rejected** options. `SEM-c01-decision` still lists three options (`:581`). `SEM-horn-tt-max` / `SEM-c01-inexpressible` stay `characterization` (`:534-555`) with the same oracles. `SEM-asof-elig` `oracle: []` (`:594`) — fill with new pin name. `test-matrix.md` recount only; never copy YAML rows (ADR-023 D023-3).

**ASOF pin gap:** `m011_claim_supports.rs:217-226` `derived_q_loses_eligibility_when_last_premise_support_withdrawn` already: after last P retract, `!derivation_eligible_at` **and** `claim_supported_at(claim_q)` still true. It does **not** call `as_of`. `m012_multi_hop.rs:117-143` hop2 `is_live_at(MAX,2017)` while ineligible. Add a **named** `#[test]` that `as_of(vt)` still contains the derived triple when eligibility is false. Do **not** invert `v44_a_inverse_retract_as_of_earlier` (`m017_verify44_killtests.rs:162-189`): direct Horn `retract_derived` **does** drop current `as_of` for `caused_by==premise`. If CONTEXT “Retract of a premise does not retract derived supports” is read as deleting `retract_derived`, **that contradicts `eval.rs` → D-SCOPE stop**. Record ASOF-ELIG as **query does not hide ineligible live rows**, not as “no Retract follow-on”.

## Honeycomb restage

Hand-edit `.kutha/dictionaries/honeycomb.yaml` only: prepend `# 2026-10-06 restage (M018 S02 Phase 46: C01 (a) / HORN-KC / ASOF-ELIG): ADR-013[, ADR-023]; map/delivery/capability unchanged.` Under `id: ADR-013` (`:174-199`) keep `map: Proposed`, `delivery: spike`, `capability: named`; append evidence names + comment `; cell still Proposed`. Dump (does **not** rewrite YAML): `uv run kutha-gov map` / `uv run kutha-gov map ADR-013`. Do not Accept the cell. Update `I-F13-typed-clocks` claim (`invariants.yaml:218-222`) — still `disposition: deferred`, not a new check. Close semantic-gap F13 remainder / A-F05 **OPEN** wording (`semantic-gap-review.md:241,256`) to “decided; behaviour unchanged”. ADR-023 Open questions 1–3 (`:97-99`) dated closed-as-decided. CHANGELOG last-commit (kutha-changelog); **not** `.kutha/STATE.md` (controller).

## Phase 47 — must not do here

DET-02 EventId identity; HORN-CORR-01 `Correct`/`CorrectInterval` re-derive; CASC-01 admit-every-hop. Do not invert `SEM-horn-correct-char`. No legal pack / SPIKE-LAW-01 / fifth clocks / `CLOCK-01`. No `eval.rs`/`fold.rs`/`fold_query.rs` behaviour change.

## Don't Hand-Roll

| Problem | Don't | Use |
|---------|-------|-----|
| C01 sunset | Fifth axis, Horn Const calendar, `valid_to` as application-end | Pack stamp later; oracle `fixg01_*` |
| Knowledge-cut Horn | Re-derive in `as_known_on` | Existing TT hide |
| Eligibility in `as_of` | Filter live triples by `derivation_eligible_at` | Separate API |
| Honeycomb | Python Check subclass / Accepted | Comment + evidence list |

## Common Pitfalls

1. **Silently `killed` + `kill_mutation: none` → HIGH** (`kinds.py:2552-2566`). Honour “leave `blocked`”; put “killed pin” in the **test comment**, not matrix status, unless adding a real mutation **and** CONTEXT is waived.
2. **Leaving rejected invariant text** while recording the opposite verdict.
3. **Inverting `known04_*`** — that *is* the chosen rule.
4. **Touching `eval.rs`** to “fix” CONTEXT’s retract sentence — stop instead.
5. **Promoting honeycomb / SemVer / tags.**

## Standard Stack / Package Legitimacy

No new packages. Workspace: `cargo test -p kutha-runtime --offline --test m017_known_on_horn --test m016_golden_fixture` (+ new ASOF test). `uv run kutha-gov precommit`; product close: `uv run kutha-gov ci`. **Packages removed/SUS:** none.

## Environment Availability

| Dependency | Available | Version | Fallback |
|------------|-----------|---------|----------|
| cargo/rustc | ✓ | 1.99.0 | — |
| uv | ✓ | 0.12.13 | — |

## Security Domain

Docs/oracles only. ASVS V2–V4/V6 n/a. V5: fixture strings stay in tests (C01 quote already in `m016_golden_fixture.rs:8`). No new parsers. `security_enforcement` on; `nyquist_validation: false` — Validation Architecture omitted.

## Assumptions Log

| # | Claim | Risk |
|---|-------|------|
| A1 | CONTEXT “leave `blocked`” outranks D-HORN-KC “promote → killed” for **YAML status** | If controller wanted `killed`+mutation, `test_matrix` HIGH until mutation text exists |

## Open Questions

1. **Retract cascade vs ASOF wording** — treat as query-plane (recommended) vs stop under D-SCOPE.
2. Dual `SEM-horn-tt-max` (characterization) + `SEM-horn-kc` (blocked, same oracle) — keep both; rewrite kc invariant only.

## Sources

- CBM `root-kutha-graph` (prefer; `kutha-graph` crates `metadata_changed`/`not_tracked` as of 2026-10-02). Coverage: eval/fold_query/provenance/quantum/tests `metadata_match`; ADR-013 + test-matrix.yaml `metadata_changed` (read source).
- ADR-013, ADR-000:16, ADR-023, `kinds.py`, honeycomb.yaml, ROADMAP M018 S02–S03, REQUIREMENTS C01-01/HORN-KC-01/ASOF-ELIG-01, 46-CONTEXT, 46-PATTERNS.

**Valid until:** 2026-11-05 (decision record; crate APIs stable).
