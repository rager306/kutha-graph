---
phase: 163-migrate-dropped-tail
---

# Phase 163 — migrate reports dropped_tail

CE whole-repo review finding #13: Op-wire / pin-wire / pin-cite migrate drop
torn jsonl/WAL/segment tails with no count. Thin slice: `dropped_tail` on the
three reports (+ jsonl oracle). Honeycomb stays Proposed.
