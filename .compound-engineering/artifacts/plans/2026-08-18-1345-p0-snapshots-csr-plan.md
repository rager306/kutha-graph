---
title: P0 Snapshots CSR Persistence Spike
type: research
date: 2026-08-18
artifact_contract: ce-unified-plan/v1
artifact_readiness: implementation-ready
product_contract_source: ce-plan-bootstrap
execution: code
---

# P0 Snapshots, CSR Lease, File Log

## Goal Capsule

Close ADR-000’s remaining snapshot-story checkbox and extend the heartbeat spike: snapshots as droppable leases, file-backed **semantic** log (not Rocks-as-SoT), CSR adjacency lease + seek, fork-at-offset. Then graduate ADR-000 Research → **Proposed**.

## Key Decisions

- KD1. Semantic log on disk is JSONL of events. Rocks WAL remains the crash *cousin* (not this unit).
- KD2. Snapshot = fold + dict + log offset. Replay = snapshot + tail. Snapshot is not SoT.
- KD3. CSR is a rebuilt lease of live facts; droppable; sorted neighbors + `seek` (LFTJ cousin, not a new WCOJ).
- KD4. No HNSW, Cypher, Rocks, ABAC, commits unless asked.

## Units

U1. serde + snapshot + store + CSR + fork in `kutha-common` / `kutha-runtime`.
U2. Tests: snapshot+tail equals full replay; persist/open round-trip; CSR drop+rebuild; fork isolation; seek.
U3. ADR-000 checklist + status Proposed; README spine row.

## DoD

`cargo test --workspace` green. ADR-000 Proposed.
