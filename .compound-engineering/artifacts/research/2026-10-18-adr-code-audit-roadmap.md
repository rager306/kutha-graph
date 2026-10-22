# ADR / code audit + proposed GSD roadmap (2026-10-18)

Evidence: fresh CBM `kutha-graph` (14930n/35788e, 2026-10-18T16:21Z) and GitNexus `kutha-graph` (5044n/17842e, analyze --index-only). Subagents + parent MCP. Graph ≠ completeness; Rust `Self::` callers need `rg`.

Freeze unchanged: Cypher / HNSW / legal pack / SIX-01. Honeycomb stays Proposed. Harness `L_delivery=M104-closed`.

## Indexes

| Plane | Project | Nodes | Edges | Note |
|-------|---------|-------|-------|------|
| CBM | kutha-graph | 14930 | 35788 | full reindex; cited product paths metadata_match |
| GitNexus | kutha-graph | 5044 | 17842 | --index-only; skipAgentsMd/skipSkills |
| GitNexus explain/pdg | — | — | — | no PDG/taint layer |
| route_map / tool_map | — | 0 | — | N/A (Rust engine, no HTTP/MCP routes) |

## Proposed next GSD milestone (pending owner confirm)

**v0.99 — Residual correctness + hot-path honesty (harness M105–M107)**

Phases continue from 176. One steel thread per phase. Not Cypher/HNSW.

| Phase | Req | Slice intent | Class |
|-------|-----|--------------|-------|
| 176 | AUD-01 / OPT-PIN | `live_policy_pin_at` indexed election (no full policy scan) | optimize |
| 177 | AUD-02 / OPT-PREM | reverse premise-rel pointer; O(1) unindex half | optimize |
| 178 | AUD-03 / FIX-WIRE | mixed Define emit-once / TermId honesty | correct |
| 179 (optional) | AUD-05 / FIX-REPLAY | `horn_behavior_reproducible` matches `conjunct_vt` | correct |
| defer | AUD-04 | multi-combination Horn DFS | design |
| defer | AUD-07 | open path recover fold from replay | optimize (larger) |

Out of scope: thaw map items, honeycomb Accept, ADR-050 six dicts, kinds.py split.
