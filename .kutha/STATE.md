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
L_delivery=M157-closed
L_capability=ff5-green
```

| Lifecycle | Current | Must not read as |
|-----------|---------|------------------|
| L_map | ADR-000–002 and 010–093 are **Proposed** | Product ready / Accepted |
| L_delivery | M157 closed (v1.51) | Capability proven / Cypher started |
| L_capability | FF5 green (`as_of(2015) ≠ as_of(2021)` on statute fixture) | Governor CI green |

## Next action

M157 is closed: `query_ir.rs` gives a thin typed IR (`Scan`, `Hops`) interpreted over `live_at` and the typed/untyped CSR leases, with `prepare` / `Prepared::run` / `PreparedHops::run_from` (a snapshot bound to its (tt, vt) cut) and fail-closed named lookup; a self-relative `hop_k` bench records a band (ratios against a calibration loop; walk is linear in starts, lease build is measured separately). Hops return a sorted set of end nodes; Scan keeps the `as_of` bag. The independent verifier found that the first bench measured n x CSR rebuild; the prepare/run split fixed it, and the `m019_dur_a` lock flake that fired in this slice's gate was fixed in the test file (production lock code unchanged). Leapfrog is still off the query path. Next: the roadmap kill rule ("if B3 shows no gap against the baseline, skip B4/B5 and go to B6") needs the competitor numbers, so the next step is an owner decision: (1) set up the Samyama and FalkorDB baseline (docker, at least 20 GB free disk, OK to pause `fd_api` / `9router` for a run), or (2) reorder to B6 Cypher AS OF (M003) or Arc C with their own leases and fresh REDs. Residuals that stay open until a fired RED: ordinary `StoreBlob` digests are not verified on open; `horn_behavior_reproducible` fails open on a missing rule blob; dangling lineage ids after a checkpoint; vintage checkpoint payloads; `cause_of_behavior` last-wins on duplicate event ids; checkpoint has no operator surface; B1b vacuum; about 20 older tests still use non-RAII scratch dirs (`m002_rock00.rs` leaks `tmp/p51-store-*`); the `hop_k` band is wide because the host is shared with a foreign cargo job (record on a quiet host before it is used as a regression gate). Verus V2 waits for a slice that rewrites `leapfrog.rs`. The harness moratorium applies. Honeycomb stays Proposed. Not Cypher/HNSW until their milestones are leased; `I-legal-pack-not-next` stays. Not a hard FSM (ADR-000 D3).

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
