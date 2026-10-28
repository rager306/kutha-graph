---
phase: 149-migrate-cli
---

# Phase 149 — kutha-migrate operator CLI

After M076, `migrate_compat` is library-only. Operators lack a bin entry (unlike
`kutha-tenant`). Thin slice: `kutha-migrate` calls existing
`store::migrate_compat` on a directory path or `KUTHA_MIGRATE_DIR`. Honeycomb
stays Proposed.
