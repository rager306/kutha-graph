---
phase: 122-append-tip-rollback
---

# Phase 122 — Context

CE whole-repo #6: post-write append `Other` must not leave a tip that same-object
retry duplicates EventIds. Decision: restore durable in-memory cursor on failure
(crash-shaped disk suffix); next `append` rewinds that suffix before write; keep
`Other` + `kutha.lock` (m030); poison only after a committed rollover seal.

Honeycomb stays Proposed. Not Cypher/HNSW.
