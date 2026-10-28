---
phase: 147-migrate-compat
---

# Phase 147 — migrate_compat

After M075, Op-wire and pin migrate stay separate entrypoints. Operators can
run `migrate_pins` while meta-op envelopes remain. Thin facade:
`store::migrate_compat` = `migrate_op_wire` then `migrate_pins` (reuse; no new
remap). Honeycomb stays Proposed.
