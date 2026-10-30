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
L_delivery=M172-S01-closed
L_capability=ff5-green
```

| Lifecycle | Current | Must not read as |
|-----------|---------|------------------|
| L_map | ADR-000–002 and 010–093 are **Proposed** | Product ready / Accepted |
| L_delivery | M172-S01 closed (B3 no-gap; B4/B5 skipped; GSD v1.65) | Capability proven / Cypher started |
| L_capability | FF5 green (`as_of(2015) ≠ as_of(2021)` on statute fixture) | Governor CI green |

## Next action

M172-S01 is closed under GSD **v1.65 B3 hot-path benchmark**. Pinned Kutha baseline: `hop2.walk.n` 170,568 ns; 2n 353,765; 4n 714,556; ratio floors 2.28/4.61/2.22. Competitor medians: FalkorDB 4,779 µs, Samyama 14,511 µs; Kutha walk-only ≈29.5×/≈89.6× faster, prepared ≈9.3×/≈28.3×. Decision: **skip B4 (HNSW) and B5 (planner)**; **B6 (Cypher AS OF) still needs M003** and is not thawed. No product code change. M171-S01 remains closed (ingest tradeoff accepted; `KUTHA_FOLD_INDEX_MODE=lazy|thin`). M170-S01 remains closed and its follow-ups stay non-blocking: shared scan helper, capacity pre-pass/over-reservation on heavily retracted cuts, and the m019 store-lock flake. Residuals stay separate: eCFR edition ambiguity, remote rocks observe skipped, GSD security audit warning. Honeycomb stays Proposed; `I-legal-pack-not-next` stays.

## Thaw map

The freeze list came from agent-assisted commits (`1ee5942`, `85db1f7`, 2026-08-18, Cursor co-authored); the owner states he did not decide it. M002 is thawed (2026-10-03, owner's request). Every other item keeps its own named gate; nothing is thawed by "an M002 lease".

| Item | Thaws when |
|------|------------|
| RocksDB crate | Thawed (Phase 51/55): optional dependency `rocksdb` in kutha-runtime only; `freeze-rocks` is a durable rule (optional dependencies only) |
| Cypher/GPML parser | M003 is leased |
| HNSW | M005 is leased |
| ADR-050 six dictionaries | SIX-01 is leased |
| ADR-080/081 | a security milestone is leased (after M003) |
| full ADR-090 ontology | STATE names a legal pack (`I-legal-pack-not-next`) |
| ADR-093 | a science pack is leased (M004) |
| Consensus Query 103+ | the owner lifts the literature bound |
