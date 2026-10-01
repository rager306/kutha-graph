# Phase 25 Context: Genesis log

**Milestone:** v0.06 / harness M013 S04
**Requirements:** GEN-01, GEN-02, GEN-03
**Depends on:** Phase 24
**Source of gaps:** `docs/architecture/semantic-gap-review.md` addendum F10; ADR-050 clarification 2026-10-01; D1 (log = SoT)
**Run rules:** `docs/process/gsd-auto-contract.md`

## Locked decisions

- **D-01:** Genesis events are the first events of a new log: one `Op::AllowRelation` per initial relation (the op still exists until Phase 26 converts it). The YAML (`allow.rs`, `ENV_PATH` / `relations_path()`) is read only by the code that creates a new log.
- **D-02:** `Runtime.allowed` (the `HashSet<String>` loaded by `load_allowed_names()` in `new`, `from_snapshot`, `from_dict_and_events`) is deleted. `admit` consults `fold.relation_allowed_at` only.
- **D-03:** Opening an existing log never reads the YAML. A log without genesis events fails closed for every product relation; there is no silent fallback. Crates are `0.0.0` and unpublished, so no legacy migration is built (recorded here as a decision; revisit if a persisted log of value exists).
- **D-04:** Genesis events are ordinary events: counted in the log, covered by `replay_check` and the provenance fingerprint, retractable.
- **D-05:** Interning stays as in M010: bootstrap terms stay silent, live interns append `Define`. Genesis must not change `Define` id stability (M012a S04 oracles).
- **D-06:** The process plane is separate. `kutha-tenant` and the harness relations (`process.relations`, `.kutha/dictionaries/relations.yaml`) are not merged into product genesis (`plane-mix-dicts`).
- **D-07:** RED first: retract a seeded relation, then admit an Assert on it at a later cut. On current code it is admitted via the YAML set; after the change it fails closed at the later cut and still admits at the earlier cut.

## Spike anchors

- `crates/kutha-runtime/src/quantum.rs` — `admit` (allowed-set branch), the three constructors, `delivery_key_retry` (now indexed)
- `crates/kutha-runtime/src/allow.rs`; `tests/ff6_allowlist.rs`; `tests/m012_allowlist_facts.rs`
- `crates/kutha-runtime/src/store.rs` — `open`, `persist` (genesis must survive a round trip)

## Tool contract (both tools)

CBM inbound `trace_path` on `load_allowed_names`, `admit`, `Runtime::new`, `Runtime::default` (fan-in about 85: record it); GitNexus `impact` upstream on `new`, `from_snapshot`, `admit`; `rg 'Runtime::(new|default)' crates` count recorded. After edit: `detect_changes` in both.

## Verifier kill-tests

- Retract a seeded relation: admit fails closed after, passes before (`as_of` pair).
- Delete or rename the YAML after creating a log, then `open` and emit: still works (no file read).
- `persist` then `open` keeps genesis; `replay_check` passes; tampering with a genesis event is detected.
- A log with genesis stripped rejects every product relation.

## Non-goals

Rule or policy bodies, six dictionaries, tenant relations, legacy-format migration.

## Harness

Active Milestone M013, Active Slice **S04** while this phase runs (controller leases it).
