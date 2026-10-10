# ADR-023: Test Strategy and Oracle Matrix

## Status

**Proposed** (Verify — map of oracles; not a coverage number and not Accepted)

## Date

2026-10-05

## Honeycomb coordinates

- Axes: **Verify** (primary) · **Space** (numbered with 020–022 conventions)
- Depends on: ADR-000 (D9), ADR-002, ADR-020
- Anticipates: M019 S05–S07 named tests; does not authorise those tests

## Context

Post-M002 audits (2026-10-04) listed durability residuals, semantic characterizations, and harness theatre. D9 says testing is a first-class capability; the repo had no machine-readable map of which invariant is pinned by which `#[test]` or governor check, which gaps belong to which slice, and which rows wait on an owner decision (C01, Horn, `as_of` vs eligibility).

This cell records the strategy. It does not rewrite D1–D10, does not accept itself, and does not ship features. Governor green on the matrix check means named oracles exist and statuses are internally consistent — not that the product is tested (D-05).

Prior-art verdicts live in `.compound-engineering/artifacts/research/2026-10-05-test-strategy-prior-art.md`, not in this body.

## Decisions

### D023-1. Cell and status

Honeycomb cell 023 is **Proposed**, `delivery: map-only`, `capability: none`. Evidence lists only real governor or test names (empty until a named product test is this cell's own). The cell does not authorise features and does not Accept honeycomb.

### D023-2. Oracle taxonomy

| Kind | Meaning |
|------|---------|
| Decision oracle | Pins a decided behaviour; changing it needs a decision |
| Characterization oracle | Pins current behaviour; labelled in the fn name or doc comment; may be replaced when the owner decides |
| Property / differential | Seeded (LCG, `m017_determinism.rs` pattern); no new dependency |
| Crash / failpoint | Recovery after a named fault |
| Kill-test | Verifier-authored mutation that must fail |
| Governor oracle | Dictionary check id |
| Bench gate | Controller honour, not a coverage percentage |

Layering: every durability invariant needs a named test and a named `kill_mutation` that fails it. A `cfg(feature)` test needs an FSM observe row (`observe_rocks` is the template). Owner-gated invariants are `blocked` rows with a decision id, never `#[ignore]` / xfail and never a green placeholder. Characterization must say so in the name or doc comment.

Determinism: seeded LCG only; EventIds are not stable (UUID v7); compare derived triples, not ids, until the owner decides identity; no wall-clock or sleep in oracles.

Mutation policy: the verifier runs a mutation list per slice. `cargo-mutants` may be a host tool (this host has 27.1.0); it is never a repository dependency and never a gate.

### D023-3. Matrix is machine-readable first

`docs/architecture/test-matrix.yaml` is the source of truth. `docs/architecture/test-matrix.md` is the human page (how to read, counts, gap list by slice, blocked list by decision) and must not duplicate rows by hand.

Row fields: `id`, `plane` (`product` \| `harness`), `invariant`, `layer`, `oracle` (test fn names or check ids), `kill_mutation`, `status` (`killed` \| `characterization` \| `gap` \| `blocked` \| `spec-only` \| `gate`), `gate` (`default` \| `feature:<observe-row>` \| `controller`), `owner` (slice `M###-S##` or a decision id), `source`.

Do not mark `killed` unless the test exists and a mutation is recorded. Every `gap` names the M019 slice that will close it. `unowned` is rejected.

### D023-4. Governor check

Check `test-matrix` (kind `test_matrix`) proves: named product oracles exist as asserting `#[test]` and are not `#[ignore]`; harness oracles exist in `checks.yaml`; `blocked` owners are on the decision allowlist; `gap` owners exist as `M###-S##` under that heading in `.kutha/ROADMAP.md`; `feature:` gates name an `observe_cargo` FSM state; `killed` forbids `kill_mutation: none`. Selftest derives a mutation that breaks the kind's own rule. The matrix is not added to `observe_cargo.required`.

Existing kinds cannot express ROADMAP slice ids (`M019-S05` is not a literal token in ROADMAP) or the killed × `kill_mutation` cross-field rule without theatre. The kind is the last responsible moment (`docs/process/governor-intake.md`).

### D023-5. Do not overclaim

The matrix is a map of oracles, not a coverage number. Governor green ≠ ADR Accepted ≠ capability ≠ “the product is tested”.

## Consequences

### Positive

- S05–S07 have an honest backlog of named gaps.
- Owner-gated work stays `blocked` instead of xfail.
- Feature-gated Rocks tests stay tied to `observe_rocks`.

### Negative / risks

- A stale matrix row is a process HIGH, not a product bug.
- Listing an M010–M017 oracle does not re-open that slice.

### Non-goals

- Writing the missing tests (S05–S07).
- Deciding C01, Horn-vs-knowledge-cut, or `as_of` vs eligibility.
- New Cargo dependencies; cargo-mutants as CI; coverage percentages.
- Changing named M010–M017 oracle assertions.

## Evidence

- Matrix: `docs/architecture/test-matrix.yaml`, `docs/architecture/test-matrix.md`
- Check: `test-matrix` / kind `test_matrix`
- Audits: `.compound-engineering/artifacts/research/2026-10-04-post-m002-audit-{code-tests,governor,adr}.md`
- Prior art: `.compound-engineering/artifacts/research/2026-10-05-test-strategy-prior-art.md`
- Semantic gaps: `docs/architecture/semantic-gap-review.md`

### Evidence (2026-10-06, Phase 48 / M018 S04; Proposed)

`ORACLE-if15` and `ORACLE-if16` are killed. GATE-01 equals `BENCH-controller` (existing `gsd-auto-contract` Bench gate at product close; `status: gate`). Status stays **Proposed**.

### Clarification (2026-10-09; Phase 85 / M026 S01; Proposed)

**Generators and missing oracles (G-C04 / G-C05 / G-C11):**

1. `MixStyle::Coverage` extends `ingest_mix` without changing BehaviorFree/WithHorn LCG schedules: Retract-before-Assert, overlapping Asserts, keyed retry, Correct/Retract of derived Behavior, empty `n_ops=0`.
2. Oracles: `m026_generators.rs` (user Assert EventId persist/open; facade retract/correct/derive with two live facts; empty-runtime persist/open).
3. Matrix rows `SEM-m026-*` are `killed` with non-`none` kill_mutation. Cell remains **Proposed**.

### Clarification (2026-10-09; Phase 86 / M026 S02; Proposed)

**Matrix honesty (kill-claim / tautology):**

1. Honor-system / overstated kills: `SEM-seeded-det` → `characterization`; `SEM-horn-kc` / `SEM-c01-decision` / `SEM-tr02-legal` / `DUR-wrong-role-active` / `DUR-sealed-trailer-lie` keep `killed` with kill_mutation text matching the real product injection.
2. Tautologies relabelled `characterization`: `SEM-naive-as-of-alias`, `SEM-known-at-mirror`, `SEM-seed-budget` (oracle `seed_budget_is_env_floor_not_domain`).
3. Optional `cargo-mutants` on `fold.rs`/`admission.rs` remains verifier evidence, not a gate (not run this slice). Cell remains **Proposed**.

### Clarification (2026-10-10; Phase 101 / M041 S01; Proposed)

**Adapter identity + shrink gate (Helix-informed; extend existing `sem.rs`, not a parallel ReplayEngine):**

1. Independent `Naive` (`m019_sem_model`), `shrink_events` / `replay_runtime`, and layout schedule already exist. Extensions: `candidate_deps_hold`, `shrink_events_ddmin`, `try_persist_with_layout` (Rollover seam reject ≠ oracle fail), optional `KUTHA_SAVE_SHRUNK`.
2. **Two EventId expectations:** `expected_persist_event_ids` = export_jsonl Define-prefix encoding; `expected_open_event_ids` = non-Define live ids after `persist`+`open` (Defines leave the opened log). Do not compare raw live ids to opened ids, and do not use the export helper for the open path.
3. Oracle `persist_roundtrip_trace_identity_across_layouts` pins open EventId identity across Legacy/Segmented/Rollover/Migrated. `SEM-seeded-det` stays characterization (triples); identity for open is `SEM-m041-persist-trace-identity`. Cell remains **Proposed**.

## Open questions

Decided 2026-10-06 (Phase 46 / M018 S02; recorded in ADR-013; this cell stays Proposed):

1. C01 / FIXG-01 — pack stamp (a). Not a fifth axis, not a Horn body, not `valid_to` as application-end. Oracle remains `fixg01_c01_sunset_inexpressible_as_horn`.
2. HORN-KC-01 — ingest-at-head. Horn fires at log-head TT (`search_prems` / `live_horns` at `u64::MAX`). `known04_horn_fires_when_premise_hidden_on_early_cut` is the killed pin (ingest mint + `as_known_on` TT hide; do not invert). Historical-cut mint is not an ingest-time law.
3. ASOF-ELIG-01 — `as_of` is support-live; eligibility is `derivation_eligible_at`. Query still lists ineligible live triples. Oracle: `asof_elig01_as_of_contains_derived_when_ineligible`.

Decided 2026-10-06 (Phase 47 / M018 S03; this cell stays Proposed):

4. DET-02 — derived EventIds are SHA-256-stable (`Event::stable_derived`); user Assert stays UUID v7. Oracle: `det02_two_ingest_derived_event_ids_equal`.
6. HORN-CORR-01 — `Correct` / `CorrectInterval` of a Horn premise `retract_derived` then `horn_fires`. Oracles: `horn_correct_derived_not_live_after_premise_correct`, `horn_correct_interval_derived_not_live_after_patch`.

Still open:

5. TR-02 — force vs publication vs decision clocks (ADR-090). FF5 is two VT windows, not this.

## Related Decisions

- ADR-000 D9 — testing is first-class
- ADR-010 / 012 / 014 — durability encodings
- ADR-013 — bi-temporal cuts; C01 pack stamp / Horn ingest-at-head / `as_of` support-live recorded 2026-10-06; DET-02 / HORN-CORR-01 recorded 2026-10-06; TR-02 still open
- ADR-020 — conventions neighbour (format, not crate layout)
- ADR-060 — strict replay
- ADR-090 TR-02 — legal clocks, mapped not implemented
