# Kutha harness state

**Lease** of process intent + last fold — not process SoT. Process SoT (H0) is `.kutha/events.jsonl` (gitignored observations). Product SoT is `crates/kutha-runtime`. Not an ADR.

**STCA plane:** Space = check slices; Time = harness JSONL; Composition = `--budget`.

**Active Milestone:** M010
**Active Slice:** S01
**Phase:** H4

## Lifecycles (do not collapse)

```text
L_map=honeycomb-proposed
L_delivery=M010-S01
L_capability=ff5-green
```

| Lifecycle | Current | Must not read as |
|-----------|---------|------------------|
| L_map | ADR-000–002 and 010–093 are **Proposed** | Product ready / Accepted |
| L_delivery | M010 S01 active (semantic recovery without snapshot lease) | Capability proven / Rocks started / M002 leased |
| L_capability | FF5 green (`as_of(2015) ≠ as_of(2021)` on statute fixture) | Governor CI green |

## Next action

Deliver **M010 S01**: `store::open` recovers intern meanings from retained `terms.jsonl` + event replay when `snapshot.json` is absent. Do **not** start a legal pack. Do **not** start M002 (Rocks) until STATE names it. Do not open ADR-100. Do not implement Cypher/HNSW.

## Freeze (until explicit M002 lease)

RocksDB crate, Cypher/GPML parser, HNSW, ADR-050 six dictionaries, ADR-080/081, full ADR-090 ontology, ADR-093, Consensus Query 103+.
