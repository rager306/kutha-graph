# Kutha harness state

**Lease** of process intent + last fold — not process SoT. Process SoT (H0) is `.kutha/events.jsonl` (gitignored observations). Product SoT is `crates/kutha-runtime`. Not an ADR.

**STCA plane:** Space = check slices; Time = harness JSONL; Composition = `--budget`.

**Active Milestone:** M013
**Active Slice:** None
**Phase:** H5

## Lifecycles (do not collapse)

```text
L_map=honeycomb-proposed
L_delivery=M013-leased
L_capability=ff5-green
```

| Lifecycle | Current | Must not read as |
|-----------|---------|------------------|
| L_map | ADR-000–002 and 010–093 are **Proposed** | Product ready / Accepted |
| L_delivery | M012 closed; M013 leased, no slice started (baseline, indexes, genesis, substrate, facade) | Capability proven / Rocks started / M002 leased / rules or typed clocks started / ADR-050 six dictionaries |
| L_capability | FF5 green (`as_of(2015) ≠ as_of(2021)` on statute fixture) | Governor CI green |

## Next action

**M012 closed; M013 leased** (`L_delivery=M013-leased`, 2026-10-01, after the post-M012 review in `docs/architecture/semantic-gap-review.md`). No slice is active: each GSD phase 22–28 leases its slice (S01–S07) before any code, and clears it when its verification passes. Thawed subset for M013 only: bench harness, fold indexes, genesis log, dictionary entries as facts, one write path and facade. Do **not** implement rules, typed clocks (M014/M015, need their own lease), ADR-050 six dictionaries, a legal pack, Cypher/HNSW, or M002 Rocks. Honeycomb stays Proposed (GATE-03). H5 dogfood remains the current harness rung. Controller protocol and tool contract: `docs/process/gsd-auto-contract.md`.


## Freeze (until explicit M002 lease)

RocksDB crate, Cypher/GPML parser, HNSW, ADR-050 six dictionaries, ADR-080/081, full ADR-090 ontology, ADR-093, Consensus Query 103+.
