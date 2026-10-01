---
independent: true
verifier_model: composer-2.5
status: passed
score: 4/4
commands:
  - cmd: "uv run kutha-gov precommit --check artifact-budget"; exit: 0
  - cmd: "uv run kutha-gov precommit --check verification-independent"; exit: 0
  - cmd: "uv run kutha-gov precommit --check id-map-cite"; exit: 0
  - cmd: "uv run kutha-gov selftest --check artifact-budget"; exit: 0
  - cmd: "uv run kutha-gov selftest --check verification-independent"; exit: 0
  - cmd: "uv run kutha-gov selftest --check id-map-cite"; exit: 0
  - cmd: "200-line scratch plan artifact then precommit --check artifact-budget"; exit: 1
read_only_inputs:
  - .planning/REQUIREMENTS.md
  - .planning/ROADMAP.md
  - .planning/phases/28-independent-verification-and-artifact-budget/28-CONTEXT.md
  - .planning/v0.06-MILESTONE-AUDIT.md
  - docs/process/gsd-auto-contract.md
  - docs/process/governor-intake.md
  - .kutha/META.md
  - .kutha/dictionaries/checks.yaml
  - .kutha/dictionaries/invariants.yaml
  - scripts/kutha_gov/kinds.py
  - .planning/phases/22-benchmark-baseline-and-size-target/22-VERIFICATION.md
  - .planning/phases/23-legal-fixture-reconnaissance/23-VERIFICATION.md
  - .planning/phases/24-fold-indexes-for-ingest/24-VERIFICATION.md
  - .planning/phases/25-genesis-log/25-VERIFICATION.md
  - .planning/phases/26-dictionary-entries-as-facts/26-VERIFICATION.md
  - .planning/phases/27-one-write-path-and-facade/27-VERIFICATION.md
---

# Phase 28 — independent verification

**ROADMAP SC:** (1) budget fail-closed — 200-line scratch plan file → precommit HIGH (max 150, max 2 plans/dir); clean tree exit 0; selftest `mutations=4` OK. (2) Phases 22–27: independence frontmatter; glob_absent needles absent in all VERIFICATION bodies; precommit OK. (3) `## Id map` in contract; both ROADMAPs link, no GSD-version table row in planning ROADMAP; id-map-cite OK; selftest `mutations=5`. (4) milestone audit read — was gaps_found (missing this file); re-audit before archive (PROC-04).

**Budget wc (phases 22–28):** PLAN≤150, SUMMARY≤60, VERIFICATION≤60, CONTEXT≤80; ≤2 PLAN and ≤2 SUMMARY per phase — all within limits (28 has no VERIFICATION until this file).

| REQ | Status | Evidence |
|-----|--------|----------|
| PROC-01 | satisfied | artifact-budget + non-vacuous selftest |
| PROC-02 | satisfied | 22–27 structural frontmatter; this file (allowed inputs only) |
| PROC-03 | satisfied | id-map-cite precommit + selftest |
| PROC-04 | partial | audit artifact exists; rerun `/gsd-audit-milestone` after PROC-02 closure |

**Score:** 4/4 ROADMAP success criteria for enforceable gates; PROC-04 archive gate awaits audit refresh.
