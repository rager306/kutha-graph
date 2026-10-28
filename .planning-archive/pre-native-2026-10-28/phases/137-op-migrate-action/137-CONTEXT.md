---
phase: 137-op-migrate-action
---

# Phase 137 — Context

M065 remapped RecordAdmission. RecordAction mirrors `record_action`: single `kutha:action` Assert with subject=jid, object=target_claim string, delivery_key=jid, polarity from admitted. Cite fields accepted for shape only. Empty jid/claim fail-closed. Serde aliases stay forbidden. Directory rewrite stays deferred.
