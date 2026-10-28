---
phase: 135-op-migrate-policy
---

# Phase 135 — Context

M063 remapped RegisterRule via `RuleAst::named`. PinPolicy mirrors `emit_policy`: UTF-8 body bytes + `policy_version_hash` digest, `StoreBlob` then `kutha:policy` Assert. Empty definition fail-closed. Serde aliases stay forbidden.
