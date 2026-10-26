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
L_delivery=M161-closed
L_capability=ff5-green
```

| Lifecycle | Current | Must not read as |
|-----------|---------|------------------|
| L_map | ADR-000–002 and 010–093 are **Proposed** | Product ready / Accepted |
| L_delivery | M161 closed (v1.55) | Capability proven / Cypher started |
| L_capability | FF5 green (`as_of(2015) ≠ as_of(2021)` on statute fixture) | Governor CI green |

## Next action

M161 is closed: a Behavior EventId shared by two Horn paths keeps every distinct `caused_by`, and retracting that id removes it from `derived_by_cause` under each cause. Indexing the same cause twice does not store it twice. A single-cause head still clears its one reverse list. No new Op. Release id stays `0.0.01`. Next: the roadmap kill rule still needs Samyama and FalkorDB numbers (docker, and the owner's OK to pause `fd_api` / `9router`), or a later lease of B6 Cypher AS OF (M003) or Arc C with a fresh RED. Residuals that stay open until a fired RED: dangling lineage ids after a checkpoint; vintage checkpoint payloads; checkpoint has no operator surface; B1b vacuum; parallel `b1_checkpoint_kill3` WouldBlock. `derived_by_cause` can still list the same head twice if the same cause is indexed twice; unindex removes every copy. The harness moratorium applies. Honeycomb stays Proposed. Not Cypher/HNSW until their milestones are leased; `I-legal-pack-not-next` stays. Not a hard FSM (ADR-000 D3).

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
