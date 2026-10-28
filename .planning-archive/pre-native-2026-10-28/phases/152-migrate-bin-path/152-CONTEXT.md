---
phase: 152-migrate-bin-path
---

# Phase 152 — built migrate bin discoverability

After M080, observe builds `kutha-migrate`, but AGENTS still documents
`cargo run` (second compile). Thin slice: agent-surface prefers
`$CARGO_TARGET_DIR/debug/kutha-migrate`. Honeycomb stays Proposed.
