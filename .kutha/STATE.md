# Kutha harness state

**Lease** of process intent + last fold — not process SoT. Process SoT (H0) is `.kutha/events.jsonl` (gitignored observations). Product SoT is `crates/kutha-runtime`. Not an ADR.

**STCA plane:** Space = check slices; Time = harness JSONL; Composition = `--budget`.

**Active Milestone:** M013
**Active Slice:** None
**Phase:** H5

## Lifecycles (do not collapse)

```text
L_map=honeycomb-proposed
L_delivery=M013-S03-done
L_capability=ff5-green
```

| Lifecycle | Current | Must not read as |
|-----------|---------|------------------|
| L_map | ADR-000–002 and 010–093 are **Proposed** | Product ready / Accepted |
| L_delivery | M013 S01–S03 done; next slice unleased | Capability proven / Rocks started / M002 leased / rules or typed clocks started / ADR-050 six dictionaries |
| L_capability | FF5 green (`as_of(2015) ≠ as_of(2021)` on statute fixture) | Governor CI green |

## Next action

**M013 S03 closed** (`L_delivery=M013-S03-done`, Active Slice **None**, 2026-10-01). Fold ingest indexes + FoldIndexMode (Eager/Lazy); D-06 write-cost accepted under Eager with Lazy thin-write option. Do **not** start S04–S07, M014/M015, ADR-050 six dictionaries, a legal pack, Cypher/HNSW, or M002 Rocks until controller leases the next slice. Honeycomb stays Proposed (GATE-03). Controller protocol: `docs/process/gsd-auto-contract.md`.


## Freeze (until explicit M002 lease)

RocksDB crate, Cypher/GPML parser, HNSW, ADR-050 six dictionaries, ADR-080/081, full ADR-090 ontology, ADR-093, Consensus Query 103+.
