# Kutha harness state

**Lease** of process intent + last fold — not process SoT. Process SoT (H0) is `.kutha/events.jsonl` (gitignored observations). Product SoT is the event log (D1): sealed `KUTHASEG` segments plus `manifest.json`, with legacy `events.jsonl` + `events.wal` until `store::migrate`; `crates/kutha-runtime` folds it. `events.jsonl` after migrate is an export. Not an ADR.

**STCA plane:** Space = check slices; Time = harness JSONL; Composition = `--budget`.

**Active Milestone:** None
**Active Slice:** None
**Phase:** H5
**Release:** 0.0.01 (`VERSION`, format `N.N.NN`). Packaging SemVer in Cargo and pyproject is `0.0.1` because Cargo rejects a leading zero. `publish` stays false.

## Lifecycles (do not collapse)

```text
L_map=honeycomb-proposed
L_delivery=M176-S01-closed
L_capability=ff5-green
```

| Lifecycle | Current | Must not read as |
|-----------|---------|------------------|
| L_map | ADR-000–002 and 010–093 are **Proposed** | Product ready / Accepted |
| L_delivery | M176-S01 closed (C1 lifecycle foundation; GSD v1.69) | Full ABAC / tenant product |
| L_capability | FF5 green (`as_of(2015) ≠ as_of(2021)` on statute fixture) | Governor CI green |

## Next action

M176-S01 is closed under GSD **v1.69 C1 pack lifecycle foundation**. Delivered typed D021 `Op::PackInstall` / `PackActivate` / `PackRetire` meta events with deterministic length-prefixed digests, empty object_ids, no graph facts, and fail-closed admission (duplicate install, invalid activate/retire, empty identifiers, user-forged Pack*). Runtime exposes dictionary-mode `emit_pack_*` and a replay-derived `pack_lifecycle()` lease; state survives persist/open, fork prefix cuts, and checkpoint_sealed_prefix; graph fingerprint is unchanged. Tests: schema 4/4, runtime 5/5, workspace 762 passed / 0 failed. No pack artifact loading, operator registration, dictionary facets, lease rebuild, WASM, marketplace, schema SMO, or ADR Accepted. M175-S01 remains closed (C4 path-policy foundation). M174/M173/M172 remain closed. M171/M170 residuals stay non-blocking. Residuals stay separate: eCFR edition ambiguity, remote rocks observe skipped, GSD security audit warning. Honeycomb stays Proposed; `I-legal-pack-not-next` stays.

## Thaw map

The freeze list came from agent-assisted commits (`1ee5942`, `85db1f7`, 2026-08-18, Cursor co-authored); the owner states he did not decide it. M002 is thawed (2026-10-03, owner's request). Every other item keeps its own named gate; nothing is thawed by "an M002 lease".

| Item | Thaws when |
|------|------------|
| RocksDB crate | Thawed (Phase 51/55): optional dependency `rocksdb` in kutha-runtime only; `freeze-rocks` is a durable rule (optional dependencies only) |
| Cypher/GPML parser | Thawed (M003a leased 2026-10-30): read-only subset parser; M003b binder swap stays gated on C2 |
| HNSW | M005 is leased |
| ADR-050 six dictionaries | SIX-01 is leased |
| ADR-080/081 | a security milestone is leased (after M003) |
| full ADR-090 ontology | STATE names a legal pack (`I-legal-pack-not-next`) |
| ADR-093 | a science pack is leased (M004) |
| Consensus Query 103+ | the owner lifts the literature bound |
