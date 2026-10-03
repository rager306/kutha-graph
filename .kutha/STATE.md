# Kutha harness state

**Lease** of process intent + last fold — not process SoT. Process SoT (H0) is `.kutha/events.jsonl` (gitignored observations). Product SoT is `crates/kutha-runtime`. Not an ADR.

**STCA plane:** Space = check slices; Time = harness JSONL; Composition = `--budget`.

**Active Milestone:** M002
**Active Slice:** S01
**Phase:** H5

## Lifecycles (do not collapse)

```text
L_map=honeycomb-proposed
L_delivery=M018-S01-done
L_capability=ff5-green
```

| Lifecycle | Current | Must not read as |
|-----------|---------|------------------|
| L_map | ADR-000–002 and 010–093 are **Proposed** | Product ready / Accepted |
| L_delivery | M002 S01 active (thaw map and freeze governor honesty); M018 paused after S01 | Capability proven / Rocks started / typed clocks / ADR-050 six dictionaries |
| L_capability | FF5 green (`as_of(2015) ≠ as_of(2021)` on statute fixture) | Governor CI green |

## Next action

**M002 S01 leased** (`L_delivery=M018-S01-done`, Active Milestone **M002**, Active Slice **S01**, 2026-10-03). GSD Phase 49 only: thaw map and freeze governor honesty (documents, YAML, governor tests; no `crates/` change). M002 is thawed at the owner's request; its slice list is in `.kutha/ROADMAP.md`. M018 is paused after S01 at the S02 decision gate (C01, Horn vs knowledge cut). Honeycomb stays Proposed. No SemVer product tag (crates remain 0.0.0).

## Thaw map

The freeze list was written by earlier AI agents (2026-08-18), not decided by the owner. M002 is thawed (2026-10-03, owner's request). Every other item keeps its own named gate; nothing is thawed by "an M002 lease".

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
