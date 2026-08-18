# Kutha harness state

**Lease** of process intent + last fold — not process SoT. Process SoT (H0) is `.kutha/events.jsonl` (gitignored observations). Product SoT is `crates/kutha-runtime`. Not an ADR.

**STCA plane:** Space = check slices; Time = harness JSONL; Composition = `--budget`.

**Active Milestone:** M001
**Active Slice:** S03
**Phase:** H0

## Lifecycles (do not collapse)

```text
L_map=honeycomb-proposed
L_delivery=M001-active
L_capability=ff5-green
```

| Lifecycle | Current | Must not read as |
|-----------|---------|------------------|
| L_map | ADR-000–002 and 010–093 are **Proposed** | Product ready / Accepted |
| L_delivery | M001 Legal PIT steel thread | Capability proven |
| L_capability | FF5 green (`as_of(2015) ≠ as_of(2021)` on statute fixture) | Governor CI green |

## Next action

Implement slice **S03**: one relation allowlist, fail-closed unknown relation (FF6 stub, not ADR-050’s six dictionaries). Do not open ADR-100. Do not implement Rocks/Cypher/HNSW yet.

## Freeze (until S03 / explicit STATE change)

RocksDB crate, Cypher/GPML parser, HNSW, ADR-050 six dictionaries, ADR-080/081, full ADR-090 ontology, ADR-093, Consensus Query 103+.
