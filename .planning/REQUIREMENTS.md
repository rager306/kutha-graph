# Requirements: Kutha

**Milestones:** v0.39 / harness M045 shipped 2026-10-10. Next lease: controller decision. Lease: `.kutha/STATE.md`.

## v0.39 Requirements (M045) — complete

### glob_absent comment policy (Phase 105, S01)

- [x] **GLOB-CMT-01**: `#` comment-only needles pass without unless_line_contains
- [x] **GLOB-CMT-02**: `//` comment-only needles pass without unless_line_contains
- [x] **GLOB-CMT-03**: code-line needles still HIGH; governor `glob-absent-comment-policy`

## Out of Scope (this milestone)

| Feature | Reason |
|---------|--------|
| G3 #6 selftest unproven SKIP | **Decision-gate (human)** — no auto-fix |
| G3 #23, #38… | Later lease |
| Product fold / Cypher / HNSW | Thaw map |
