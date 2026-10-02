# Kutha harness state

**Lease** of process intent + last fold — not process SoT. Process SoT (H0) is `.kutha/events.jsonl` (gitignored observations). Product SoT is `crates/kutha-runtime`. Not an ADR.

**STCA plane:** Space = check slices; Time = harness JSONL; Composition = `--budget`.

**Active Milestone:** M013
**Active Slice:** S08
**Phase:** H5

## Lifecycles (do not collapse)

```text
L_map=honeycomb-proposed
L_delivery=M013-S07-done
L_capability=ff5-green
```

| Lifecycle | Current | Must not read as |
|-----------|---------|------------------|
| L_map | ADR-000–002 and 010–093 are **Proposed** | Product ready / Accepted |
| L_delivery | M013 S01–S07 done; S08 active (close-out honesty: documents and governor); S09 unleased | Capability proven / Rocks started / M002 leased / rules or typed clocks started / ADR-050 six dictionaries |
| L_capability | FF5 green (`as_of(2015) ≠ as_of(2021)` on statute fixture) | Governor CI green |

## Next action

**M013 S08 leased** (`L_delivery=M013-S07-done`, Active Slice **S08**, 2026-10-02). GSD Phase 29 only: documents and governor follow-ups from the v0.06 audit and the project review (DOC-01..04, GOV-01..04); no `crates/` change. S09 (Phase 30, write-path hardening) stays unleased until S08 closes. Do **not** start M014/M015, ADR-050 six dictionaries, Cypher/HNSW, or M002 Rocks. Honeycomb stays Proposed (GATE-03). Controller protocol: `docs/process/gsd-auto-contract.md`. No SemVer tag.


## Freeze (until explicit M002 lease)

RocksDB crate, Cypher/GPML parser, HNSW, ADR-050 six dictionaries, ADR-080/081, full ADR-090 ontology, ADR-093, Consensus Query 103+.
