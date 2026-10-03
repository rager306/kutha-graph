# Kutha harness state

**Lease** of process intent + last fold — not process SoT. Process SoT (H0) is `.kutha/events.jsonl` (gitignored observations). Product SoT is `crates/kutha-runtime`. Not an ADR.

**STCA plane:** Space = check slices; Time = harness JSONL; Composition = `--budget`.

**Active Milestone:** M002
**Active Slice:** None
**Phase:** H5

## Lifecycles (do not collapse)

```text
L_map=honeycomb-proposed
L_delivery=M002-S02-done
L_capability=ff5-green
```

| Lifecycle | Current | Must not read as |
|-----------|---------|------------------|
| L_map | ADR-000–002 and 010–093 are **Proposed** | Product ready / Accepted |
| L_delivery | M002 S02 done (SNAP-01 and fail-closed WAL open); S03 unleased; M018 paused at its S02 decision gate | Capability proven / Rocks started / typed clocks / ADR-050 six dictionaries |
| L_capability | FF5 green (`as_of(2015) ≠ as_of(2021)` on statute fixture) | Governor CI green |

## Next action

**M002 S02 closed** (`L_delivery=M002-S02-done`, Active Milestone **M002**, Active Slice **None**, 2026-10-03). GSD Phase 50 verified independently (7/7): atomic sidecars with directory fsync, log-ahead-of-snapshot opens, fail-closed WAL header and I/O, crash matrix over all six persist steps. Next: S03 (RocksDB build spike, Phase 51) needs a lease; its first task is the `unless:` kernel extension for `freeze-rocks`. M018 stays paused at its S02 decision gate (C01, Horn vs knowledge cut). Honeycomb stays Proposed. No SemVer product tag (crates remain 0.0.0).

## Thaw map

The freeze list came from agent-assisted commits (`1ee5942`, `85db1f7`, 2026-08-18, Cursor co-authored); the owner states he did not decide it. M002 is thawed (2026-10-03, owner's request). Every other item keeps its own named gate; nothing is thawed by "an M002 lease".

| Item | Thaws when |
|------|------------|
| RocksDB crate | M002 Phase 51 (S03) is leased; until then `freeze-rocks` bans the dependency |
| Cypher/GPML parser | M003 is leased |
| HNSW | M005 is leased |
| ADR-050 six dictionaries | SIX-01 is leased |
| ADR-080/081 | a security milestone is leased (after M003) |
| full ADR-090 ontology | STATE names a legal pack (`I-legal-pack-not-next`) |
| ADR-093 | a science pack is leased (M004) |
| Consensus Query 103+ | the owner lifts the literature bound |
