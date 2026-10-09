# Proposed roadmap after M022 (for GSD auto)

Source: `research/2026-10-08-post-m022-synthesis-context.md` and the four audits it cites. **Proposal, not a lease.** `.kutha/ROADMAP.md` stays the delivery pyramid; copy a milestone there only when the controller leases it. One lease at a time. Honeycomb stays Proposed; thaw map unchanged.

Ids (next free): GSD tag `v0.17` → M023 (Phases 75–), `v0.18` → M024, `v0.19` → M025. Crate SemVer stays 0.0.0. Slice format follows `.kutha/ROADMAP.md`.

## M023: Green HEAD and truth reset (recommended first lease)

**Why first:** HEAD fails `cargo test --workspace` (2 M022 regressions, 1 stale naive model, 3 flaky durability tests) and M022 is recorded as verified. Every later slice needs a trustworthy gate. Not a new capability.

**Success:** `cargo test --workspace --offline --no-fail-fast` has zero failures on consecutive runs; full `uv run kutha-gov ci` 0 HIGH; false "verified" claims are corrected by a dated entry, not by rewriting history; living overlay matches STATE; bench gate ≤10% holds.

- [ ] **S01: Restore green** `risk:medium` `depends:[]`
  > After this: `delivery_key_retry` short-circuits when the key's *current live* support exists (follows `Correct`/`CorrectInterval` replacements, not only the minting Assert) and re-mints only when every support for the key is dead; the two m013 tests pass unchanged and `m022_keyed_retry` stays green; a new oracle covers keyed retry after Retract of a CorrectInterval id (G-A01, G-C06). `m019_sem_model` naive `Retract` invalidates all live residuals, and a seed forces CI-then-Retract so the model and the fold disagree if either regresses (G-C-tautology). The three `m019_dur_a` tests are isolated or fixed (unique scratch dirs per test or serialized group) and run ≥5 times in the workspace run without a failure. Bench gate ≤10%.
- [ ] **S02: Close-gate honesty** `risk:low` `depends:[S01]`
  > After this: `72-VERIFICATION.md`, `v0.16-MILESTONE-AUDIT.md` and a new CHANGELOG entry state the regression, its fix commit and the missing gate; `docs/process/gsd-auto-contract.md` close checklist requires a recorded full `cargo test --workspace --no-fail-fast` and full `ci` (not `precommit`); a governor or pytest row proves the close checklist names both (YAML row, not a new Python class). No `crates/` change.
- [ ] **S03: Living overlay truth** `risk:low` `depends:[S02]`
  > After this: `PROJECT.md`, `REQUIREMENTS.md` (every id the roadmap uses, with phase and oracle), `MILESTONES.md`, `.kutha/ROADMAP.md` (no "M021 active", unique numbering), `WINDOWS.md` (open count recomputed), `test-matrix.md` counts (derived from YAML, ideally checked by the `test-matrix` row), `legal-fixture-recon.md` (C01 pack stamp), `semantic-gap-review.md:24,29,30`, `semantic-contract-validation.md:29,30`, ADR-000 lines 9/110/182/211, ADR-010 L105, ADR-011 L36, `p0-spike-inventory.md:9` and honeycomb.yaml evidence (`keyed_assert_retry_*`, `retract_correct_interval_*`) match HEAD; ADRs stay Proposed; `.cursor/skills/codebase-memory` names one canonical CBM project and notes the Rust `Self::`/method-edge blind spot (rg arbiter). Docs/YAML only.

## M024: Fold identity and write semantics

**Source:** G-A02/C01, G-A07, G-A05, G-C12, G-A09, G-A10. Fold semantics only; no new clocks, no Cypher.

**Owner/controller gates:** (a) Retract/Correct of an unknown or dead id: fail closed or pinned no-op (G-A05). (b) `Correct` on a multi-residual id: patch all residuals, or reject with a typed error (G-A02). Decide before the RED oracle is written; record in ADR-013 Clarification.

- [ ] **S01: Decisions G-A02 / G-A05 / G-A07** `risk:high` `gate:controller+user decision` `depends:[]`
  > After this: ADR-013 records one rule for Correct/CorrectInterval/Retract by `event_id` across CorrectInterval residuals and for unknown/dead ids; oracles are RED first; `fact_index_by_event_id` first-live is either replaced by an explicit all-live selector or documented as "mint only".
- [ ] **S02: Implement and differential** `risk:high` `depends:[S01]`
  > After this: `Correct(ci_id)`, `Retract(original)` after CI and unknown-id behavior match the S01 rule in fold, CSR lease and persist/open; the naive model implements the same rule independently; generator reaches Correct/Retract of CI and derived ids; bench gate ≤10%.
- [ ] **S03: Write-path atomicity** `risk:medium` `depends:[S02]`
  > After this: `intern` Define is either part of a committed quantum or its crash window is characterized with a named oracle (G-A09); receipt shape decision for D014-1 recorded (constant-size digest vs `event_ids` list) and, if changed, oracle RED first (G-A10); GROUP-01 stays a documented residual unless the decision changes.

## M025: Wedge answer surface (thaw-safe value)

**Source:** G-B01..B04, G-C03, G-C09, G-C10, R-22. Non-goals: M003 Cypher, M005 HNSW, SIX-01, ADR-080/081, full ADR-090 ontology, five-axis clocks, law corpus as a pack, Pi Durable.

- [ ] **S01: `as_known_on` VT decision** `risk:medium` `gate:controller decision` `depends:[]`
  > After this: ADR-013 states whether `as_known_on` is TT-only by design (G-A08/G-C03); a pin or fix oracle exists; `semantic-contract-validation.md` agrees.
- [ ] **S02: Clock-safe facade** `risk:high` `depends:[S01]`
  > After this: `as_of` takes a valid-time type and `as_known_on` a wall/anchor type at public call sites; raw `u64` no longer compiles for the wrong role (compile_fail UI tests, G-B02/G-C09); `Op` storage may stay `u64`; no fifth axis; existing pins `known04_*`, `asof_elig01_*`, `tr02_*` and `c01_*` unchanged; bench gate ≤10%.
- [x] **S03: Legal answer envelope** `risk:high` `depends:[S02]`
  > After this: a `norm_view(vt, wall)` returns triples with eligibility, `conflict_report_at` and typed non-success (`Conflict`, `Abstain`, `Unknown`, `MissingAnchor`) (G-B01/B04/C10); `as_of` and the ASOF-ELIG-01 decision are unchanged; live ∩ eligible is the facade's job (R-22); oracles include a retracted-premise case and a missing-anchor case.
- [x] **S04: Demo binary and real-text fixtures** `risk:medium` `depends:[S03]`
  > After this: a thin bin (extend `examples/statute_as_of`, not ADR-070) answers "norm in force on D, as known on W" over fixture ops; 2–3 real-text statute fixtures are test data only (`m016` pattern), no pack thaw, `I-legal-pack-not-next` stays.

## M026: Test strength (can run before or after M024)

**Source:** G-C02, C04, C05, C08, C11, kill-claim honesty. No product behavior change unless an oracle exposes a bug (then a fix slice).

- [ ] **S01: Generators and missing oracles** `risk:medium` `depends:[]`
  > After this: `ingest_mix` reaches Retract before Assert, Correct/Retract of derived `Behavior` ids, overlapping Asserts on one (s,r), keyed retry and empty log; oracles for user Assert `EventId` stability across persist/open (G-C04), facade `retract/correct/derive` with two live facts (G-C05), empty-runtime persist/open (G-C11); every new oracle is RED against a mutant first.
- [ ] **S02: Matrix honesty** `risk:low` `depends:[S01]`
  > After this: honor-system rows (`SEM-horn-kc`, `SEM-seeded-det`, `SEM-c01-decision`, `SEM-tr02-legal`, `DUR-wrong-role-active`, `DUR-sealed-trailer-lie`) are re-labelled `characterization` or given a real product mutant; tautologies (`model_as_of_alias_is_live_at_max`, `known_at_equals_is_known_at_filter`, `assert_seed_budget`) are replaced or relabelled; optional one-off `cargo-mutants` run on `fold.rs`/`admission.rs` is recorded as verifier evidence, not a gate.

## Parked (not roadmap; need an owner lease)

R-12 rollover seal/manifest window; R-13 GROUP-01; R-14 MEM-01; R-15 O(N) `open`; R-16 Persister/SegmentIndex wiring; R-17..19 lock/IO-01/VAC-01; G-B05 leapfrog on a real path (only with a MATCH/query consumer); G-B06 fork-diff of live triples (ADR-061 restriction, candidate after M025); G-B07 strict replay `Unavailable`; G-B08 YAML genesis; Pi Durable; ADR-092 license; jsonl-as-export acknowledgement.

## Open questions for the owner

1. Lease M023 now (recommended) or accept the red HEAD for one more slice?
2. Does the wedge come before fold-identity cleanup (M025 before M024)? Default: M024 first, because the envelope would otherwise inherit the first-live Correct bug.
3. Are G-A02/G-A05 decisions (fail closed vs documented no-op) owner decisions or controller calls?
