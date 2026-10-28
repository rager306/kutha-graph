---
phase: 160-check-reject-hint
---

# Phase 160 — --check reject redirect hint

After M088, CLI rejects `--check` outside json|precommit|selftest (exit 2), but
stderr only names the rule. Agents still guess the next command. Thin slice:
one-line redirect hint on the reject path. Honeycomb stays Proposed.
