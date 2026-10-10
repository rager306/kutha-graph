---
phase: 105-glob-absent-comment-policy
milestone: v0.39
harness: M045
slice: S01
ce_finding: "P2 #22"
---

# Phase 105 — CONTEXT

## Kill-claim

`glob_absent` exempts `#` comment lines only when `unless_line_contains` is set; without it, whole-file substring match HIGHs on comment-only needles. `//` comments are never exempt (CE review P2 #22).

## Goal

One per-line comment policy (`#` and `//`) for both branches. Oracles cover comment-only pass and code-line HIGH without `unless_line_contains`.

## Oracles

- `test_hash_comment_only_needle_passes_without_unless_line`
- `test_slash_comment_only_needle_passes_without_unless_line`
- `test_code_line_needle_high_without_unless_line`
- check `glob-absent-comment-policy`

## Out of scope

#6 decision-gate. Cypher/HNSW. Honeycomb Proposed.
