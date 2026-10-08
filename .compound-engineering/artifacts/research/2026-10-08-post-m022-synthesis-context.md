# Post-M022 synthesis: planned vs done, gaps, test strength (2026-10-08)

Context pack for a GSD auto controller. Evidence base: four read-only audits plus controller re-verification at HEAD `18151af`. Not an ADR, not a lease. Honeycomb stays **Proposed**.

## 1. Evidence base and tool limits

| Source | File |
|--------|------|
| ADR 000–014 vs code | `research/2026-10-08-audit-adr-spine-vs-code.md` (G-A01..A12) |
| Cells 020–093 + wedge readiness | `research/2026-10-08-audit-honeycomb-cells-vs-repo.md` (G-B01..B13) |
| Test semantics | `research/2026-10-08-audit-test-semantic-coverage.md` (G-C01..C12) |
| Plan vs delivery + residual register | `research/2026-10-08-audit-plan-vs-delivery.md` (G-D01..D12, R-01..R-40) |

Indexes refreshed 2026-10-08: CBM `kutha-graph` (11 492 nodes) and `root-kutha-graph` (same root, duplicate name); GitNexus `kutha-graph` (4 081 nodes, `--index-only --skip-agents-md`).

**Graph limits (rg is arbiter).** CBM and GitNexus both miss Rust `Self::f()` and `self.field.method()` edges. CBM listed 23 "no-caller" methods and GitNexus reported 0 callers for `delivery_key_retry`; `rg` showed live callers for all checked. Do not use either graph for dead-code or "no caller" claims in Rust. Real finding by `rg`: `Runtime::check_admission` has no non-test caller (the product has no consumer beyond tests, one example and the harness `kutha-tenant`).

## 2. HEAD is red (controller-verified, not a model claim)

`cargo test --workspace --offline --no-fail-fast` at `18151af`:

| Failure | Cause | Attribution |
|---------|-------|-------------|
| `m013_write_path::keyed_correct_preserves_delivery_key_and_retry`, `keyed_correct_interval_preserves_delivery_key_and_retry` (log 16 → 18) | `delivery_key_retry` live-gate looks only at the minting Assert; after `Correct` it is invalidated, so the same key re-mints (G-A01) | M022 Phase 72 (`ddcf6b4`). Phase 72 ran only `m022_*` tests |
| `m019_sem_model::bitemporal_model_matches_fold_live_at_and_as_of` | Naive model `Retract` is still first-live; `GraphFold` now retracts all live residuals | Likely M021 Phase 69 (`08184cf`); model not updated |
| `m019_dur_a` ×3 (`checkpoint_omitted_qo…`, `jsonl_inode_changes…`, `persist_migrate_race…`) | Fail in the workspace run, pass alone: parallel-binary flake, cause not investigated | Pre-existing class (handoff: observe_cargo 101 flake) |

Consequence: "Phase 72 verified" in `72-VERIFICATION.md`, `v0.16-MILESTONE-AUDIT.md` and the 2026-10-08 CHANGELOG close entry is **false until fixed**. `precommit` does not run cargo and a full `kutha-gov ci` was not run green at M022 close; `observe_cargo` already makes `cargo test --workspace` exit≠0 a HIGH (`observe.py` ~121). Close gate must be a full `ci`, not `precommit` plus targeted tests.

## 3. Planned vs delivered (condensed)

- M001–M022 ledger: delivered as planned; every close has an audit except v0.01–v0.05 and v0.11. 14 residuals closed with evidence, 23 open, 3 stale (R-register). ~7 open items are owner-gated.
- ADR spine: D1/D2/D3/D4/D8/D10 partial, D6/D010-4 stale in lock text (no Tokio, Rocks is an unwired index), D7/D012-2..5/D013-4/D014-2/D014-5 not started. No decision is *contradicted* except D6 text.
- Cells 020–093: honeycomb.yaml is honest (frozen = no code, spike = partial). 080/081/091/093/071/042/070 have no code. Name clash only: harness `tenant.rs` ≠ ADR-080.
- Wedge (norm AS OF a date) has physics (FF5, TR-02 three moments, C01 pack stamp) but no answer surface: `as_of` returns bare triples; no TR-06 outcome; clocks are raw `u64`; no query API but Rust fns.

## 4. Logical and semantic gaps (ranked, deduplicated)

| ID | Sev | Gap | Falsifiable oracle (RED first) |
|----|-----|-----|-------------------------------|
| G-A01 / C06 | high | Keyed retry re-mints after Correct (**active regression**) | existing m013 pair + keyed retry after Retract of a CorrectInterval id |
| G-A02 / C01 | high | `Correct`/`CorrectInterval` on an id with 3 residuals patch first-live only (`fact_index_by_event_id`) | CI 2015–2020 then Correct(ci): all residuals handled or fail-closed |
| G-A07 | med | Retract(original Assert) after CI leaves residuals (they carry the CI id) | Retract(original): no window of that claim stays live (or documented) |
| G-A05 | med | Retract/Correct of unknown or dead `event_id` is a silent no-op | emit Retract(unknown) → Err (or pinned no-op, owner call) |
| G-A08 / C03 | med | `as_known_on` ignores VT; closed future-VT facts visible | TtAnchor 2017, Assert valid_from=2020: known contains, as_of empty (pin or fix) |
| G-B01 | high | `as_of` shows ineligible Horn derivations (decided, ASOF-ELIG-01); a legal demo that prints it as "in force" is wrong | `norm_view` lists eligible vs ineligible; pin `asof_elig01_*` unchanged |
| G-B02 / C09 | high | Raw `u64` lets a publication year flow into `as_of` (TR-02 held by pins, not types) | compile_fail / reject: `as_of(WallTime)` |
| G-B04 / C10 | high | No typed non-success (`Abstain`/`Conflict`/`Unknown`/`MissingAnchor`) | answer type carries them |
| G-C02 | high | Retract/Correct of a derived `Behavior` id × `as_of` × `as_known_on` untested | generator targets derived ids |
| G-C04 | high | User Assert `EventId` stability across persist/open untested (only Define/derived) | ids equal after persist+open |
| G-A04 / B10 | med | Horn is ingest-at-head; "as known on D" never re-derives (decided HORN-KC-01) | keep pin; document in answer envelope |
| G-A09 | med | `intern` writes `Define` bypassing pipeline/QO | intern-only crash vs pipeline atomicity |
| G-A10 | med | Receipt `event_ids: Vec` leaks quantum size (D014-1 constant-size) | receipt length independent of cascade width |
| G-B07 | med | `replay_check` ≠ strict ADR-060 (Horn-only, no `Unavailable`) | missing CA input → typed Unavailable |
| G-B06 | med | `fork_at` without diff | live-triple diff of two forks |
| G-B08 | med | YAML/`.env` still genesis root of trust for an empty log | empty dir cannot admit `inForceAs` until a log `Allow` |
| G-A03 | high (doc) | `as_of` support-live conflicts with the 2026-09-13 contract text; decided in Phase 46, doc not reconciled | doc-only |

## 5. Test strength

- Core behaviors (emit Assert/Retract/Correct/CI, as_of vs as_known_on, FF5/FF6, Horn-correct, retract-all-live, keyed retry) have independent semantic oracles. 77 matrix rows (killed 67, characterization 5, spec-only 4, gate 1); `test-matrix.md` still says 76/66.
- Honor-system or partial kill claims: `SEM-horn-kc`, `SEM-seeded-det`, `SEM-c01-decision`, `SEM-tr02-legal`, `DUR-wrong-role-active`, `DUR-sealed-trailer-lie`, `SEM-bitemporal-model` (now failing).
- Tautologies: `model_as_of_alias_is_live_at_max`, `known_at_equals_is_known_at_filter`, `assert_seed_budget`; naive model `Retract` mirrors the old fold.
- Generators (`ingest_mix` N=8/24) never reach: Retract before Assert, Correct of derived ids, keyed retry, empty log, same-TT cascade, overlapping Asserts on one (s,r).
- Untested material fns: `claim_known`, `has_live_triple_except`, `leftover_halves`, `admit_claim`, facades `retract/correct/derive`.
- Harness tests are real for process fail-closed; `test_ci_green_*` and `test_no_high_on_live_tree` are smoke/stubbed.

## 6. Doc drift (index, no design)

`PROJECT.md` (v0.14/M020 era), `REQUIREMENTS.md` (6 ids only), `MILESTONES.md`, `.kutha/ROADMAP.md` ("M021 active" in Later, duplicate numbering), `test-matrix.md` counts, `legal-fixture-recon.md` (C01 "open"), `semantic-gap-review.md:24,29,30`, `semantic-contract-validation.md:29,30`, ADR-000 L9/110/182/211, ADR-010 L105, ADR-011 L36, `p0-spike-inventory.md:9`, honeycomb.yaml (no `keyed_assert_retry_*`), WINDOWS `open_count:11`. Planning is ~505 markdown files vs ~8.3k LOC product src; the living overlay is stale while archives grow.

## 7. Controller rules to carry

One lease at a time; honeycomb Proposed; thaw map unchanged (Cypher, HNSW, SIX-01, 080/081, full 090, 093, Consensus 103+ stay frozen). Env: `source .env`, `unset CARGO_TARGET_DIR`, `KUTHA_BENCH_N=10000`; bench gate ≤10% on `ingest_plain.n` / `as_of.n`. Never run bench or `ci` concurrently with cargo/selftest. Independent VERIFICATION per slice, re-verify after edits. Close gate: `cargo test --workspace --offline --no-fail-fast` (zero failures) and full `uv run kutha-gov ci`. Controller owns `.kutha/STATE.md`; lease commit = STATE + CHANGELOG only.
