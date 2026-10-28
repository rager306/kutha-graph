---
phase: 161-skip-cargo-reject-hint
---

# Phase 161 — --skip-cargo reject redirect hint

After M089, CLI rejects `--skip-cargo` outside `ci` (exit 2), but stderr only
names the rule. Agents still guess the next command. Thin slice: one-line
redirect hint on the reject path. Honeycomb stays Proposed.
