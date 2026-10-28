# Phase 32 Research: tt_anchor and as_known_on

**Date:** 2026-10-02. **CBM:** `root-kutha-graph` indexed 2026-10-02T05:50:18Z (Phase 31 paths `metadata_changed`; read source). `kutha-graph` CBM stale. GitNexus unused (D-06, outside M013). Graphify disabled.

## Current surface

`as_of` = `live_at(u64::MAX, vt)` (product; `kutha:` excluded). `Op`: Assert, Retract, Correct, CorrectInterval, Behavior, Define, QuantumOutcome, JustificationCite, StoreBlob. Op/Event/Fact clocks stay `u64`. `WallTime` exported, unused. No `as_known_on` / `TtAnchor`.

CBM inbound (stale lines): `Op` 145; `Runtime.emit` 87; `Runtime.as_of` 18 (non-test trace 3). Exhaustive `Op` (rg): `event.rs` (`new`, `digest_bytes`), `quantum.rs` (`op_relation`, `op_affects_fold`), `fold.rs` `apply`, `provenance.rs` derivation walk. `follow_ons` / admit / justification use `_`.

## Design (D-01): `Op::TtAnchor { wall: u64 }`

Mapped tt = `Event.ingested_at`. Reject `Op::Assert` + reserved rel: `Fact.valid_from` is valid-time; wall there mixes clocks. Do not intern tt/wall as terms.

Fold lease: `BTreeMap<u64, u64>` wall→tt, `#[serde(skip)]` like `blobs`. `apply` inserts; later same wall wins. `rebuild_tt_anchors` from the log in `hydrate_from_log` next to `rebuild_blobs`. `op_affects_fold` false (StoreBlob-class). No `Fact`. Leave `fingerprint` as facts-only (replay rebuilds from log).

`emit_tt_anchor(wall: impl Into<WallTime>)` uses `ReceiptMode::Dictionary`. Exhaustive: `Event::new` empty `object_ids`; digest tag `tt-anchor` + wall le-bytes; derivation walk `return false`; `op_relation` `None`.

## as_known_on (D-02)

`Runtime::as_known_on(wall: impl Into<WallTime>) -> Option<Vec<(TermId, TermId, TermId)>>`. Greatest wall ≤ date (`range(..=wall).next_back()`), then `live_at(tt, u64::MAX)` product. `None` = no anchor ≤ date (CONTEXT kill-test). Wall is not valid-time. Do not edit `as_of`. Test walls use YearCe integers (same numbers as FF5 years) as `WallTime`.

## Named test (D-03)

`crates/kutha-runtime/tests/m014_as_known_on.rs`: anchor@2014, statute `valid_from` 2010, anchor@2020. `as_of(2015)` sees statute; `as_known_on(2015)` uses 2014 tt and does not. Miss and W1 < D ≤ W2. FF5 bytes unchanged (FIX-01 is Phase 33).

## Out of scope (D-04)

Fixture wraps. Retract-by-EventId. Rules. M002. ADR-013 five-clocks. New crates. `.kutha/STATE.md`. Executor `32-VERIFICATION` (D-05). `kutha-gov ci`.
