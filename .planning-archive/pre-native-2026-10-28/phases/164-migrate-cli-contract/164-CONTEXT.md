---
phase: 164-migrate-cli-contract
---

# Phase 164 — kutha-migrate CLI contract

CE whole-repo review finding #9: `--help` exits 2 on stderr; extra positionals
are ignored. Thin slice: help→stdout exit 0; argc>2 reject; document that
store-dir must already exist. Honeycomb stays Proposed.
