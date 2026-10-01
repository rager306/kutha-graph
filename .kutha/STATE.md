# Kutha harness state

**Lease** of process intent + last fold — not process SoT. Process SoT (H0) is `.kutha/events.jsonl` (gitignored observations). Product SoT is `crates/kutha-runtime`. Not an ADR.

**STCA plane:** Space = check slices; Time = harness JSONL; Composition = `--budget`.

**Active Milestone:** M012
**Active Slice:** None
**Phase:** H5

## Lifecycles (do not collapse)

```text
L_map=honeycomb-proposed
L_delivery=M012-S04-done
L_capability=ff5-green
```

| Lifecycle | Current | Must not read as |
|-----------|---------|------------------|
| L_map | ADR-000–002 and 010–093 are **Proposed** | Product ready / Accepted |
| L_delivery | M012 leased (dictionaries as facts — allowlist entries, rule registry, admission meta-facts, policy version); M012a S06 remains the prior closed record | Capability proven / Rocks started / M002 leased / ADR-050 six dictionaries |
| L_capability | FF5 green (`as_of(2015) ≠ as_of(2021)` on statute fixture) | Governor CI green |

## Next action

**M012 S04 delivered** (`L_delivery=M012-S04-done`). Active Slice cleared. Next product wave needs an explicit S05 lease (multi-hop derivation / Phase 21). Do **not** implement ADR-050 six dictionaries, a legal pack, Cypher/HNSW, or M002 Rocks until STATE names them. Honeycomb stays Proposed (GATE-03). H5 dogfood remains the current harness rung.


## Freeze (until explicit M002 lease)

RocksDB crate, Cypher/GPML parser, HNSW, ADR-050 six dictionaries, ADR-080/081, full ADR-090 ontology, ADR-093, Consensus Query 103+.
