# Kutha harness state

**Lease** of process intent + last fold — not process SoT. Process SoT (H0) is `.kutha/events.jsonl` (gitignored observations). Product SoT is `crates/kutha-runtime`. Not an ADR.

**STCA plane:** Space = check slices; Time = harness JSONL; Composition = `--budget`.

**Active Milestone:** M013
**Active Slice:** S09
**Phase:** H5

## Lifecycles (do not collapse)

```text
L_map=honeycomb-proposed
L_delivery=M013-S08-done
L_capability=ff5-green
```

| Lifecycle | Current | Must not read as |
|-----------|---------|------------------|
| L_map | ADR-000–002 and 010–093 are **Proposed** | Product ready / Accepted |
| L_delivery | M013 S01–S08 done; S09 active (write-path integrity hardening) | Capability proven / Rocks started / M002 leased / rules or typed clocks started / ADR-050 six dictionaries |
| L_capability | FF5 green (`as_of(2015) ≠ as_of(2021)` on statute fixture) | Governor CI green |

## Next action

**M013 S09 leased** (`L_delivery=M013-S08-done`, Active Slice **S09**, 2026-10-02). GSD Phase 30 only: write-path integrity hardening from the project review (HARD-01..04, RED first; `crates/kutha-runtime`). Not retract-by-EventId, typed clocks, `as_of` perf index, WAL, rules, or `derive` auto-rule. Bench vs Phase 27 within 10%. Do **not** start M014/M015, ADR-050 six dictionaries, Cypher/HNSW, or M002 Rocks. Honeycomb stays Proposed (GATE-03). Controller protocol: `docs/process/gsd-auto-contract.md`. No SemVer tag.


## Freeze (until explicit M002 lease)

RocksDB crate, Cypher/GPML parser, HNSW, ADR-050 six dictionaries, ADR-080/081, full ADR-090 ontology, ADR-093, Consensus Query 103+.
