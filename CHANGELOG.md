# Changelog

All notable changes to this repository are recorded here. Process plane (harness) and product plane (crates) stay distinct.

## 2026-08-18 — Wave 4: H2 tenant ingest + unnamed CSR cut

### Product

- Removed `Runtime::csr_lease()` (silent valid-time `0`). Callers name a cut with `csr_lease_at(tt, vt)`.
- H2 tenant: `ingest_harness_jsonl` maps process `harness.run`/`status` → `runStatus` and `harness.observe`/`cargo` → `observed`, with monotonic `valid_to` windows. Same log, two process times, different live status (fixture).
- Binary `kutha-tenant` persists the tenant picture under `KUTHA_TENANT_DIR` (default `.kutha/tenant`, gitignored).
- Allowlist rows `runStatus` and `observed` are delivery facts on the FF6 list, not a second dictionary kind (ADR-050 note).

### Process

- FSM kind `emit_tenant` after `emit_log`: run `kutha-tenant`, require `as_of(last)` to match last status.
- Fold pictures `last_cargo` and `last_tenant`.
- Check `unnamed-csr` forbids bringing `csr_lease()` back.

### Trajectory

- Phase **H2**. Next: H3 (process-plane allowlist). M002 Rocks stays frozen until STATE names it.

## 2026-08-18 — Wave 3: H1 cargo observation as evidence

### Process (harness)

- FSM kind `observe_cargo`: after dictionary checks, run `cargo test --workspace --offline` and require named FF tests (`ff5_…`, `ff6_…`) to appear as `... ok`.
- Cargo output is **evidence**, not product SoT. Success is recorded as `harness.observe` triples; missing/FAILED tests are HIGH and still walk emit/fold so the log exists.
- `emit_log` counts FSM HIGH findings (not only dictionary check highs).
- Timeout from `KUTHA_GOV_CARGO_TIMEOUT_SEC` then `fsm.yaml` `timeout_sec`.
- Static check `observe-required-fn` keeps required names aligned with Rust `fn` lines.
- Split glued FSM tests (`glob_none` vs budget-from-env).

### Trajectory

- Phase **H1**. Next: H2 (harness facts on the Kutha log). M002 Rocks stays frozen until STATE names it.

## 2026-08-18 — Wave 2: FF6 relation allowlist + harness hygiene

### Product

- Fail-closed writes: `Runtime::emit` admits only relations listed in `crates/kutha-runtime/dictionaries/relations.yaml` (`schema: kutha-relations/v1`). Unknown relation → `RuntimeError::UnknownRelation`; log does not grow.
- Path override: `KUTHA_RELATIONS_PATH`. Cascade default: `KUTHA_MAX_CASCADE` (also from repo `.env`).
- FF6 tests: `crates/kutha-runtime/tests/ff6_allowlist.rs`.

### Process

- Kind `glob_none`: forbidden globs (setuptools `*.egg-info` must not live in the tree).
- Check `relation-allowlist` guards the product dictionary.
- FSM transitions must use `event:` (test rejects YAML 1.1 boolean keys).

### Trajectory

- M001 S03 done. Active slice **None**. Next: H1 (cargo test as harness evidence). M002 Rocks stays frozen until STATE names it.

## 2026-08-18 — Wave 1: meta-prompt FSM + Legal PIT FF5

### Process (harness)

- `kutha-gov ci` walks a dictionary FSM (`.kutha/dictionaries/fsm.yaml`): constitution → checks → emit → fold → `ok`|`fail`.
- Unknown FSM kind or missing transition fails closed (`unknown-fsm-kind` / `unknown-transition`).
- Budget and warn-fail come from `.env` (`KUTHA_GOV_BUDGET`, `KUTHA_GOV_FAIL_ON_WARN`), then `fsm.yaml` defaults; CLI flags win. Template: `.env.example`.
- Trajectory also requires **Active Slice** to exist on ROADMAP.
- YAML transition field is `event:` (not `on:` — YAML 1.1 would coerce `on` to boolean `true`).

### Product (kutha-runtime)

- Named cuts: `GraphFold::live_at(tt, vt)`, `GraphFold::as_of(vt)` (no “now” default), `Runtime::csr_lease_at(tt, vt)`.
- FF5: same statute-shaped log, `as_of(2015) ≠ as_of(2021)`; CSR neighbors disagree the same way.
- FF3: dropping a CSR lease does not change log length or fold fingerprint.
- Fixture note: `crates/kutha-runtime/tests/fixtures/legal_pit.md`.

### Trajectory

- M001 S01 and S02 done. Active slice is S03 (FF6 relation-allowlist stub). Honeycomb ADRs stay Proposed.

