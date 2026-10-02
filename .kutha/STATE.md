# Kutha harness state

**Lease** of process intent + last fold — not process SoT. Process SoT (H0) is `.kutha/events.jsonl` (gitignored observations). Product SoT is `crates/kutha-runtime`. Not an ADR.

**STCA plane:** Space = check slices; Time = harness JSONL; Composition = `--budget`.

**Active Milestone:** M016
**Active Slice:** None
**Phase:** H5

## Lifecycles (do not collapse)

```text
L_map=honeycomb-proposed
L_delivery=M016-S03-done
L_capability=ff5-green
```

| Lifecycle | Current | Must not read as |
|-----------|---------|------------------|
| L_map | ADR-000–002 and 010–093 are **Proposed** | Product ready / Accepted |
| L_delivery | M016 S03 done; S04 unleased | Capability proven / Rocks started / M002 leased / typed clocks / ADR-050 six dictionaries |
| L_capability | FF5 green (`as_of(2015) ≠ as_of(2021)` on statute fixture) | Governor CI green |

## Next action

**M016 S03 CLOSED** (`L_delivery=M016-S03-done`, Active Milestone **M016**, Active Slice **None**, 2026-10-02). Phase 38 verified. Next lease **S04** (Policy + execution replay). Honeycomb stays Proposed. No SemVer tag.


## Freeze (until explicit M002 lease)

RocksDB crate, Cypher/GPML parser, HNSW, ADR-050 six dictionaries, ADR-080/081, full ADR-090 ontology, ADR-093, Consensus Query 103+.
