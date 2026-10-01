---
phase: 23-legal-fixture-reconnaissance
verified: 2026-10-01T06:35:00Z
status: passed
score: 3/3 requirements verified
independent: true
verifier_model: composer-2.5-fast
behavior_unverified: 0
overrides_applied: 0
covered_files:
  - .planning/REQUIREMENTS.md
  - docs/architecture/legal-fixture-recon.md
covered_digest: "v2:sha256:bde9f3523e81a90a491648582c60f6fecb6d1c43f8b453d7a494903c1da9afc8"
read_only_inputs:
  - .planning/REQUIREMENTS.md
  - docs/architecture/legal-fixture-recon.md
  - crates/kutha-common/src/event.rs
  - crates/kutha-runtime/src/fold.rs
  - crates/kutha-runtime/src/quantum.rs
  - /root/law-nexus (QUOTE-INDEX sources)
commands:
  - cmd: "git status --porcelain -- crates/"
    exit: 0
  - cmd: "git diff --stat -- crates/"
    exit: 0
  - cmd: "quote-gate (12 rows, rg -F under LAW_NEXUS_ROOT)"
    exit: 0
    detail: "valid=12 fail=0"
  - cmd: "codebase-memory-mcp list_projects"
    exit: 0
---

# Phase 23: Legal fixture recon — Independent Verification

**Goal (REC-01..03):** Map ≥10 law-nexus texts to the two-axis model; mark expressibility; no `crates/` edits.

## Kill-tests

| # | Test | Result |
|---|------|--------|
| 1 | Re-classify 3 QUOTE-INDEX rows (fz44-001 **T**, syn-002 **E**, hostile **X**) vs `Op` + `GraphFold::as_of`/`live_at` | 3/3 agree with index |
| 2 | Quote gate (path, len≥12, E\|T\|R\|X, `rg -F`) | 12/12 |
| 3 | `crates/` porcelain + diff vs HEAD | empty |
| 4 | Note: `Clock conflation:`, five clocks, one `## M015 golden fixture proposal`, T+R in index | OK |

## Requirements

| ID | Status | Evidence |
|----|--------|----------|
| REC-01 | ✓ | 12 QUOTE-INDEX rows + C01–C10 prose in note |
| REC-02 | ✓ | Per-row E/T/R/X; **Inexpressible set** → M014/M015 |
| REC-03 | ✓ | Git kill-test #3 |

## Re-classification sample (kill-test 1)

- **T** (fz44-001): edition + application window — `valid_from`/`valid_to` alone do not separate edition vs known-on (matches note).
- **E** (syn-002): formula clause — storable as opaque Assert-shaped fact; no rule eval required (matches **E**).
- **X** (hostile-bounds): synthetic descending range — decode/hostile seed, not fold fact shape (matches **X**).

## Observable truths

| Truth | Status |
|-------|--------|
| ≥10 mapped real-text cases | ✓ VERIFIED |
| Inexpressible set named for M014/M015 | ✓ VERIFIED |
| No product crate changes | ✓ VERIFIED |

**Gaps:** none.

---
_Verifier: gsd-verifier (independent; no Phase 23 PLAN/SUMMARY read)_
