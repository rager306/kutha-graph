---
phase: 134-op-migrate-rule
---

# Phase 134 — Context

M062 remapped AllowRelation to a single Assert. RegisterRule is two modern ops (`StoreBlob` then `kutha:rule` Assert) via `RuleAst::named` + body hash — same as runtime `emit_rule`. Invalid idents fail closed (`InvalidRuleDefinition`). Serde aliases stay forbidden.
