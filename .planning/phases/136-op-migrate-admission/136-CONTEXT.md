---
phase: 136-op-migrate-admission
---

# Phase 136 — Context

M064 remapped PinPolicy. RecordAdmission mirrors `record_justification`: single `kutha:admission` Assert with subject=justification_id, object true/false, delivery_key=policy_version, polarity Positive/Negative. Empty jid/pin fail-closed. Serde aliases stay forbidden. RecordAction stays deferred.
