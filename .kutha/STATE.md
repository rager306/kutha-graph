# Kutha harness state

**Lease** of process intent + last fold — not process SoT. Process SoT (H0) is `.kutha/events.jsonl` (gitignored observations). Product SoT is `crates/kutha-runtime`. Not an ADR.

**STCA plane:** Space = check slices; Time = harness JSONL; Composition = `--budget`.

**Active Milestone:** M018
**Active Slice:** S01
**Phase:** H5

## Lifecycles (do not collapse)

```text
L_map=honeycomb-proposed
L_delivery=M017-closed
L_capability=ff5-green
```

| Lifecycle | Current | Must not read as |
|-----------|---------|------------------|
| L_map | ADR-000–002 and 010–093 are **Proposed** | Product ready / Accepted |
| L_delivery | M018 S01 active (doc truth round 2); M017 closed | Capability proven / Rocks started / M002 leased / typed clocks / ADR-050 six dictionaries |
| L_capability | FF5 green (`as_of(2015) ≠ as_of(2021)` on statute fixture) | Governor CI green |

## Next action

**M018 S01 leased** (`L_delivery=M017-closed`, Active Milestone **M018**, Active Slice **S01**, 2026-10-03). GSD Phase 45 only: documents, no `crates/` change. Phases 46–48 stay unleased; S02 is a decision gate (C01, Horn vs knowledge cut). Do **not** start M002 until leased. Honeycomb stays Proposed. No SemVer product tag (crates remain 0.0.0).

## Freeze (until explicit M002 lease)

RocksDB crate, Cypher/GPML parser, HNSW, ADR-050 six dictionaries, ADR-080/081, full ADR-090 ontology, ADR-093, Consensus Query 103+.
