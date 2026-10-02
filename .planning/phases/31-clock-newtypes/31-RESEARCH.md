# Phase 31 Research: Clock newtypes

**Date:** 2026-10-02. **CBM:** `root-kutha-graph` (indexed 2026-10-02T05:50:18Z). `kutha-graph` CBM is stale (2026-09-30). GitNexus not used (D-06, outside M013). Graphify disabled.

## Current aliases

`crates/kutha-common/src/event.rs`: `ValidTime` and `TransactionTime` are `u64` aliases (lines 30 and 37) plus `TimeScale` / `VALID_TIME_SCALE` / `TRANSACTION_TIME_SCALE`. They compile interchangeably. `EventId`/`TermId` are also aliases (not this phase).

`Op` clock fields, `Event.ingested_at`, and `Fact` clocks use those names. Query surface already spells them: `Runtime::as_of(vt: ValidTime)`, `live_at(tt, vt)`; `GraphFold` equivalents; `live_count` / `claim_supported_at` / dictionary cuts. `csr_lease_at`, `typed_csr_lease_at`, `CsrLease::from_fold`, `Materializer::build`, `record_justification`, `record_action`, `derivation_eligible_at`, `conflict_report_at` still take bare `u64`.

## Call sites (rg arbiter; CBM inbound `Runtime.as_of` = 30 incl. tests)

- `ValidTime` token: 67 matches under `crates/` (CBM `search_code` on `root-kutha-graph`).
- `valid_from:` struct fields: 35 files (tests + `pipeline` / `quantum` / `fold` / `admission` / benches / examples).
- `.as_of(` / `.live_at(`: ~20 files. Named oracles use `u64` consts (`T_OLD: u64 = 2015`) and `as_of(T_OLD)`.
- Exception: `tests/m012a_time_scale.rs` binds `let y2015: ValidTime = 2015` and `assert_eq!(y2015, 2015)`.

Coverage on cited paths: `metadata_match` except `fold.rs` `metadata_changed` (read source).

## Recommended pattern (D-01)

Keep Rust names `ValidTime` / `TransactionTime` as tuple structs. ROADMAP `Vt`/`Tt` are conceptual: `pub type Vt = ValidTime; pub type Tt = TransactionTime`. Add `WallTime` (same shape), exported, unused until Phase 32 (D-04).

Each clock: `#[serde(transparent)]` tuple struct; `Copy` `Eq` `Ord` `Hash`; `From<u64>`/`Into<u64>`; `as_u64` / `to_le_bytes` / `MAX`; `Display`; `PartialEq<u64>` so TIME-01 `assert_eq!(y2015, 2015)` stays. No `Deref`. No `From` between clocks.

**Write/storage fields (`Op`, `Event.ingested_at`, `Fact`) become explicit `u64`.** Integer struct literals in FF5/M011–M013 stay compiling (CLK-03, D-04: fixture explicit clocks are Phase 33). Query/cut methods take `impl Into<ValidTime>` / `impl Into<TransactionTime>` so `as_of(2015)` and `live_at(u64::MAX, vt)` still type-check; passing the other clock does not.

## trybuild vs doctest (D-02)

**Doctest `compile_fail` on `ValidTime` in `event.rs`.** No new crate (T-31-SC). Mix `TransactionTime` into a `ValidTime` parameter. `cargo test -p kutha-common --doc --offline`. trybuild would need a Package Legitimacy Audit and a workspace dep; out of scope.

## Out of scope

`tt_anchor` / `as_known_on` (Phase 32). FF5 field wraps to `ValidTime(...)` (Phase 33 FIX-01). Retract-by-EventId. New cargo deps. `.kutha/STATE.md`.
