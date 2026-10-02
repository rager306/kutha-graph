# Phase 33 Research: Fixture migration

**Date:** 2026-10-02. **CBM:** `root-kutha-graph` indexed 2026-10-02T05:50:18Z (Phase 31/32 paths `metadata_changed` / `not_tracked`; read source). GitNexus unused (D-06, outside M013).

## Locked (from CONTEXT)

D-01: FF5 and M011/M012/M013 use `ValidTime` / `TransactionTime` / `WallTime` constructors where literals touch the **public clock API**. D-02: assertion predicates unchanged; SUMMARY old-to-new table. D-03: executor does not run `kutha-gov ci`. D-04: no new temporal features. D-05: no executor `33-VERIFICATION`.

## Public clock API (wrap) vs log integers (do not wrap)

Query/cut methods take `impl Into` clocks (`as_of`, `live_at`, `csr_lease_at`, `typed_csr_lease_at`, `live_count`, `claim_supported_at`, `dictionary_as_of`, `derivation_eligible_at`, `conflict_report_at`, `admission_status_at`, `action_record_at`, `policy_hash_live_at`, `live_policy_pin_at`, `record_justification`/`record_action` last two args). Bare `u64` still type-checks; FIX-01 makes the type visible at the call.

`Op` `valid_from`/`valid_to` and `Event.ingested_at` stay `u64` (Phase 31). Do not wrap those fields.

Wrap map: vt arg → `ValidTime::from(x)`; tt `u64::MAX` → `TransactionTime::MAX`; other tt → `TransactionTime::from(tt)`. No `as_known_on` in FIX-01 files (WallTime row empty). TIME-01 YearCe lets already wrapped (31-02); remaining `as_of(2017)` in that file is this phase.

## Call-site inventory (rg arbiter)

Clock API present: `ff5_legal_pit.rs`, `ff6_allowlist.rs` (`live_count` only), `m011_{e2e_fixture,typed_csr,partial_correction,claim_supports}.rs`, `m012_{admission_facts,action_record,rule_registry,multi_hop,allowlist_facts}.rs`, `m012a_{time_scale,hot_indexes,log_native,stable_refs,idempotent_ingest}.rs`, `m013_{dual_mode,write_path,fold_index_mode,dictionary_facts}.rs`.

No clock API (leave bytes; SUMMARY "—"): `m011_provenance.rs`, `m011_quantum_outcome.rs`, `m013_fold_indexes.rs`, `m013_genesis_log.rs`, `m012a_verify_persist.rs`. Out of FIX-01: `m014_as_known_on.rs`, `m010_*`, facade/h2/h4.

## Plan shape

One plan (mechanical wraps). Tracer = FF5. Expand = remaining inventory. Named oracles must stay green. Do not edit `.kutha/STATE.md`.
