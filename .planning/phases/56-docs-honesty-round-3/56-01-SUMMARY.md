# Phase 56 Summary (executor)

**Slice:** M019 S01 / DOC-08. No crate or `scripts/` edit. ADRs stay **Proposed**.

## Per file

- `docs/ADR/ADR-000-…`: Clarification 2026-10-04 D-02 sentence; D1 Storage / D6 Tokio+Rocks / D8 single-writer / 2026-10-03 “RocksDB is NOT a dependency” marked Historical. D1–D10 text kept.
- `docs/ADR/ADR-010/012/014/060`: Historical on pre-SEG-01 Evidence; Current encoding (HEAD) with `store::persist_until`, `segment::persist_log`, `Persister`, `commit::committed_prefix`, `rocks_index::SegmentIndex`. F11 option A closed; B/C residual. D010-4 / D012-5 / D014-3 dated pointers.
- `docs/ADR/README.md`: unwired `rocks-index` vs unaccepted Rocks event store.
- `docs/architecture/m002-cloud-kernel-io-intake.md`: M002 CLOSED; IO-01 intake.
- `docs/architecture/p0-spike-inventory.md`: Rocks optional index in spike; “Not in spike: RocksDB crate” Historical.
- `.planning/PROJECT.md`, `REQUIREMENTS.md`: D6/Validated persist; DOC-06 Historical; WRITE-01 intern-admit vs segmented persist.
- `.planning/codebase/*`: superseded-map banner; STRUCTURE Rocks freeze Historical.
- `STRATEGY.md`: `last_updated` 2026-10-04; hot path not Rocks-as-SoT.
- `.kutha/dictionaries/honeycomb.yaml`: ADR-010/012 evidence (real `m002_*` fns); capability `none`; rock01 comment (feature-gated).
- `.kutha/ROADMAP.md`: M001–M017 collapsed; compact `[x]` ids; M018/M002/M019 kept; Later “Next lease: M018 S02”.
- `CHANGELOG.md`: one Process entry under the lease (docs-coupling).

## Needles

Removed as living: `M002 remains **frozen**`, `Not in spike: RocksDB crate`, present-tense `jsonl remains SoT`, D6 “no Tokio/Rocks”. Kept inside Historical/Clarification: `RocksDB is NOT a dependency`, quoted `jsonl remains SoT`, quoted Rocks-crate line.

Added: D-02 in ADR-000 and ADR-010; `Historical` in 010/012/014/060; F11 option A in 010/014.

## Code facts (CBM `root-kutha-graph`, `metadata_match`)

`persist_until` → `persist_log` unless `writes_legacy_layout`; `export_jsonl` unread after migrate; `SegmentIndex` inbound 0; `kutha-tenant` uses `store::persist` not `Persister`; no `tokio` in Cargo.toml.

## Could not / owner

- CHANGELOG close-M002 still has `v0.12 is for the controller to create` (dated history; D-07).
- ROADMAP M019 still says “planned, unleased” (D-05 keep intact; STATE already leased S01).
- honeycomb `capability` stays `none` (S02 observe row).
