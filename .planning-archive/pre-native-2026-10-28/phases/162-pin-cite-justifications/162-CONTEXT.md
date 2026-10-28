---
phase: 162-pin-cite-justifications
---

# Phase 162 — migrate_pin_cite drops stale justifications sidecar

CE whole-repo review at HEAD `5dd5338` finding #1: `migrate_pin_cite`
remaps JustificationCite pins but leaves `justifications.jsonl`, so
`sidecar_or_stale_prefix` bricks reopen for stores persisted with cites.
Thin slice: remove the regenerable lease alongside `snapshot.json`; RED
oracle with non-empty sidecar. Honeycomb stays Proposed.
