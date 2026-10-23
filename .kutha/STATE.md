# Kutha harness state

**Lease** of process intent + last fold — not process SoT. Process SoT (H0) is `.kutha/events.jsonl` (gitignored observations). Product SoT is the event log (D1): sealed `KUTHASEG` segments plus `manifest.json`, with legacy `events.jsonl` + `events.wal` until `store::migrate`; `crates/kutha-runtime` folds it. `events.jsonl` after migrate is an export. Not an ADR.

**STCA plane:** Space = check slices; Time = harness JSONL; Composition = `--budget`.

**Active Milestone:** M157
**Active Slice:** S01
**Phase:** H5

## Lifecycles (do not collapse)

```text
L_map=honeycomb-proposed
L_delivery=M157-S01
L_capability=ff5-green
```

| Lifecycle | Current | Must not read as |
|-----------|---------|------------------|
| L_map | ADR-000–002 and 010–093 are **Proposed** | Product ready / Accepted |
| L_delivery | M157-S01 leased (v1.51): B3-lite hop workload | Capability proven / Cypher started |
| L_capability | FF5 green (`as_of(2015) ≠ as_of(2021)` on statute fixture) | Governor CI green |

## Next action

M157 S01 (B3-lite, Kutha side only): a workload as data (start, relation, k, tt/vt cut, limit), a thin typed query IR interpreted over existing read APIs (`live_at`, `typed_csr_lease_at`), equal to `as_of` on FF5 and the statute fixture and to a nested join of `live_at` triples on a chain; a `hop_k.{n,2n,4n}` bench with a calibration loop gives a self-relative budget (ratios, not ns). No parser, no planner, no new Op, no wire change, no thaw, fingerprint contract unchanged, `eval.rs`, `fold_query.rs` and `leapfrog.rs` bodies untouched. The competitor baseline (Samyama, FalkorDB) is deferred: it needs docker, at least 20 GB of disk and the owner's OK to pause `fd_api` / `9router`. Residuals that stay open until a fired RED: ordinary `StoreBlob` digests are not verified on open; `horn_behavior_reproducible` fails open on a missing rule blob; dangling lineage ids after a checkpoint; vintage checkpoint payloads; `cause_of_behavior` last-wins on duplicate event ids; checkpoint has no operator surface; B1b vacuum; non-RAII test scratch dirs and `m019_dur_a` parallel lock flake (a fired RED only if the flake recurs in a slice gate). The harness moratorium applies. Honeycomb stays Proposed. Not Cypher/HNSW until their milestones are leased; `I-legal-pack-not-next` stays. Not a hard FSM (ADR-000 D3).

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
