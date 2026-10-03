# Kutha harness state

**Lease** of process intent + last fold — not process SoT. Process SoT (H0) is `.kutha/events.jsonl` (gitignored observations). Product SoT is `crates/kutha-runtime`. Not an ADR.

**STCA plane:** Space = check slices; Time = harness JSONL; Composition = `--budget`.

**Active Milestone:** M002
**Active Slice:** S04
**Phase:** H5

## Lifecycles (do not collapse)

```text
L_map=honeycomb-proposed
L_delivery=M002-S03-done
L_capability=ff5-green
```

| Lifecycle | Current | Must not read as |
|-----------|---------|------------------|
| L_map | ADR-000–002 and 010–093 are **Proposed** | Product ready / Accepted |
| L_delivery | M002 S04 active (incremental framed append); M018 paused at its S02 decision gate | Capability proven / Rocks started / typed clocks / ADR-050 six dictionaries |
| L_capability | FF5 green (`as_of(2015) ≠ as_of(2021)` on statute fixture) | Governor CI green |

## Next action

**M002 S04 leased** (`L_delivery=M002-S03-done`, Active Milestone **M002**, Active Slice **S04**, 2026-10-03). GSD Phase 52 only: APPEND-01 and PERF-GATE in `crates/kutha-runtime` store/WAL (a `Persister` that appends framed records with one log sync per quantum; no segments, no commit frame, no Rocks wiring). M018 stays paused at its S02 decision gate. Honeycomb stays Proposed. No SemVer product tag (crates remain 0.0.0).

## Thaw map

The freeze list came from agent-assisted commits (`1ee5942`, `85db1f7`, 2026-08-18, Cursor co-authored); the owner states he did not decide it. M002 is thawed (2026-10-03, owner's request). Every other item keeps its own named gate; nothing is thawed by "an M002 lease".

| Item | Thaws when |
|------|------------|
| RocksDB crate | Thawed while Active Milestone is M002 (Phase 51): optional feature `rocks-index-spike` only; `freeze-rocks` bans it under any other milestone; wiring waits for S07 |
| Cypher/GPML parser | M003 is leased |
| HNSW | M005 is leased |
| ADR-050 six dictionaries | SIX-01 is leased |
| ADR-080/081 | a security milestone is leased (after M003) |
| full ADR-090 ontology | STATE names a legal pack (`I-legal-pack-not-next`) |
| ADR-093 | a science pack is leased (M004) |
| Consensus Query 103+ | the owner lifts the literature bound |
