# Kutha harness state

**Lease** of process intent + last fold — not process SoT. Process SoT (H0) is `.kutha/events.jsonl` (gitignored observations). Product SoT is `crates/kutha-runtime`. Not an ADR.

**STCA plane:** Space = check slices; Time = harness JSONL; Composition = `--budget`.

**Active Milestone:** M017
**Active Slice:** S02
**Phase:** H5

## Lifecycles (do not collapse)

```text
L_map=honeycomb-proposed
L_delivery=M017-S01-done
L_capability=ff5-green
```

| Lifecycle | Current | Must not read as |
|-----------|---------|------------------|
| L_map | ADR-000–002 and 010–093 are **Proposed** | Product ready / Accepted |
| L_delivery | M017 S02 active (replay integrity) | Capability proven / Rocks started / M002 leased / typed clocks / ADR-050 six dictionaries |
| L_capability | FF5 green (`as_of(2015) ≠ as_of(2021)` on statute fixture) | Governor CI green |

## Next action

**M017 S02 leased** (`L_delivery=M017-S01-done`, Active Milestone **M017**, Active Slice **S02**, 2026-10-03). GSD Phase 42 only: replay integrity (product code; CBM + GitNexus pair). Do **not** start Phase 43, M002, Cypher/HNSW, or ADR-050 six dictionaries until leased. Honeycomb stays Proposed. No SemVer product tag (crates remain 0.0.0).

## Freeze (until explicit M002 lease)

RocksDB crate, Cypher/GPML parser, HNSW, ADR-050 six dictionaries, ADR-080/081, full ADR-090 ontology, ADR-093, Consensus Query 103+.
