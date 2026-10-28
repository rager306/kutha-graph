---
phase: 146-migrate-pins
---

# Phase 146 — Context

After M071/M072, pin-wire and pin-cite are separate directory rewrites. Operators can run cite without wire (or forget order). Thin facade: `store::migrate_pins` = `migrate_pin_wire` then `migrate_pin_cite` (reuse; no new remap).
