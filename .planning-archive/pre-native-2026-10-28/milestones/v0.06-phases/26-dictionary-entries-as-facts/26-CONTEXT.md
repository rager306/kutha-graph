# Phase 26 Context: Dictionary entries as facts

**Milestone:** v0.06 / harness M013 S05
**Requirements:** SUB-01, SUB-02, SUB-03, SUB-04
**Depends on:** Phase 25 (genesis)
**Source of gaps:** `docs/architecture/semantic-gap-review.md` addendum F11, F14; ADR-011 clarification 2026-10-01
**Run rules:** `docs/process/gsd-auto-contract.md`

## Locked decisions

- **D-01:** Relation allowance, rule, policy, admission, and action records become ordinary bi-temporal facts whose relations live in a reserved `kutha:` namespace (for example `kutha:allows`, `kutha:rule`, `kutha:policy`, `kutha:admission`, `kutha:action`); the subject is the entry identity. A relation prefix is chosen over a new term class because it needs no schema change (ADR-011 open question; the plan may overrule with a reason).
- **D-02:** Rule and policy bodies are content-addressed: the body is stored once, the fact carries its hash. Reuse the SHA-256 digest that `RegisterRule` hashes with today (`sha2` in `quantum.rs`). Net `Op` variants decrease: the five meta variants are removed, at most one generic blob variant is added (or `Define` with a body namespace). The plan must state which.
- **D-03:** Removed: `Op::{AllowRelation, RegisterRule, PinPolicy, RecordAdmission, RecordAction}`, their five entry structs and five `is_live_at` copies in `fold.rs`, and `has_allow_entry`, `has_rule_entry`, `has_policy_entry`, `has_admission_entry`, `has_action_entry`. Retraction of an entry is the generic Retract by `EventId`.
- **D-04:** `justification_id` is the `EventId` of the justification record; `policy_version` is the body hash. The RAM buffer `self.justifications` (linear `find`) and its `attach_*` helpers go away; `check_admission` reads the fold.
- **D-05:** Namespace hygiene: product queries (`as_of`, `live_at`, CSR lease, conflict report) exclude `kutha:` relations by default; a separate accessor returns dictionary entries at a cut. FF5, CSR, and M011 fixtures keep their counts.
- **D-06:** Named oracles (all `m012_*`, `m012a_*`, `m011_*`, `ff5`, `ff6`) keep their assertions. Constructor calls change to helper functions; every rename is listed old to new in the SUMMARY. A changed assertion stops the phase.
- **D-07:** No legacy decoding of the five old variants (same decision as Phase 25 D-03).
- **D-08:** RED first: an oracle that reads an admission record and a policy entry through the same `as_of` path as any fact, and retracts one through generic Retract at a later cut; it fails on current code (separate side tables).

## Tool contract (both tools)

CBM `search_code` for each of the five variant names (baseline `rg -c` counts: `quantum.rs` 6–7 each, `fold.rs` 3 each, `event.rs` 2 each, plus `m012_*` tests); `trace_path` inbound on `apply`, `emit`, `check_admission`; `get_code_snippet` for the five entry structs. GitNexus `impact` upstream on `apply` and `emit`, `context` on `GraphFold`. `rg` is the arbiter for variant questions. After edit: `detect_changes` in both, `check_index_coverage` on `event.rs`, `fold.rs`, `quantum.rs`.

## Verifier kill-tests

- `rg 'Op::(AllowRelation|RegisterRule|PinPolicy|RecordAdmission|RecordAction)' crates` is empty.
- An admission recorded at cut A is readable AS OF A after a policy change at cut B.
- Retract an allow entry by `EventId`: the relation fails closed after, admits before.
- `as_of` over the FF5 fixture returns the same count with dictionary entries present.
- Replay from an empty runtime reproduces the same fingerprint including all entries.

## Non-goals

Rule evaluation (M015), typed clocks, six dictionaries, new capabilities.

## Harness

Active Milestone M013, Active Slice **S05** while this phase runs (controller leases it).
