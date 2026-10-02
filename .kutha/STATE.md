# Kutha harness state

**Lease** of process intent + last fold — not process SoT. Process SoT (H0) is `.kutha/events.jsonl` (gitignored observations). Product SoT is `crates/kutha-runtime`. Not an ADR.

**STCA plane:** Space = check slices; Time = harness JSONL; Composition = `--budget`.

**Active Milestone:** None
**Active Slice:** None
**Phase:** H5

## Lifecycles (do not collapse)

```text
L_map=honeycomb-proposed
L_delivery=M013-closed
L_capability=ff5-green
```

| Lifecycle | Current | Must not read as |
|-----------|---------|------------------|
| L_map | ADR-000–002 and 010–093 are **Proposed** | Product ready / Accepted |
| L_delivery | M013 CLOSED (S01–S09); next milestone unleased | Capability proven / Rocks started / M002 leased / rules or typed clocks started / ADR-050 six dictionaries |
| L_capability | FF5 green (`as_of(2015) ≠ as_of(2021)` on statute fixture) | Governor CI green |

## Next action

**M013 CLOSED** (`L_delivery=M013-closed`, Active Milestone **None**, Active Slice **None**, 2026-10-02). GSD v0.06 archived under `.planning/milestones/v0.06-*` and tagged `v0.06` (process tag; crate SemVer stays `0.0.0`). Residual audit debt acknowledged. Do **not** start M014 until a controller lease commit names it. Do **not** start M015/M002, ADR-050 six dictionaries, Cypher/HNSW. Honeycomb stays Proposed (GATE-03).


## Freeze (until explicit M002 lease)

RocksDB crate, Cypher/GPML parser, HNSW, ADR-050 six dictionaries, ADR-080/081, full ADR-090 ontology, ADR-093, Consensus Query 103+.
