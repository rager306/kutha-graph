# Kutha harness state

**Lease** of process intent + last fold — not process SoT. Process SoT (H0) is `.kutha/events.jsonl` (gitignored observations). Product SoT is the event log (D1): sealed `KUTHASEG` segments plus `manifest.json`, with legacy `events.jsonl` + `events.wal` until `store::migrate`; `crates/kutha-runtime` folds it. `events.jsonl` after migrate is an export. Not an ADR.

**STCA plane:** Space = check slices; Time = harness JSONL; Composition = `--budget`.

**Active Milestone:** M155
**Active Slice:** S01
**Phase:** H5

## Lifecycles (do not collapse)

```text
L_map=honeycomb-proposed
L_delivery=M155-S01
L_capability=ff5-green
```

| Lifecycle | Current | Must not read as |
|-----------|---------|------------------|
| L_map | ADR-000–002 and 010–093 are **Proposed** | Product ready / Accepted |
| L_delivery | M155-S01 leased (v1.49): checkpoint is a cut, not a hole | Capability proven / Cypher started |
| L_capability | FF5 green (`as_of(2015) ≠ as_of(2021)` on statute fixture) | Governor CI green |

## Next action

M155 S01 (finish thin B1; audit 2026-10-23 found the gaps in the M154 workflow): after a checkpoint, `fork_at` on a cut inside the compacted prefix must fail closed (typed error, `n > len` too) instead of returning an empty or later fold; a live Retract/Correct of a prefix Horn premise must cascade exactly like the un-checkpointed twin (RED first; if it already passes the finding is wrong and is dropped); `migrate_compat` / `kutha-migrate` must fail closed on a checkpointed store. No new Op, no thaw, fingerprint contract unchanged. Then B2 (ADR-061 fork diff over the now-honest `fork_at`). Residuals that stay open until a fired RED: ordinary `StoreBlob` digests are not verified on open; `horn_behavior_reproducible` fails open on a missing rule blob; dangling lineage ids (`caused_by`, sidecar cites) after a checkpoint; checkpoint has no operator surface; B1b vacuum proper. Before B3: Samyama and FalkorDB baseline environment with the owner. The harness moratorium applies. Honeycomb stays Proposed. Not Cypher/HNSW until their milestones are leased; `I-legal-pack-not-next` stays. Not a hard FSM (ADR-000 D3).

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
