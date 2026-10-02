---
independent: true
verifier_model: cursor-grok-4.6-xhigh
status: passed
score: 8/8
commands:
  - cmd: "uv run pytest -q scripts/tests -k 'cui_budget or glob_max_lines or verification_independent or frontmatter_paths'"; exit: 0
  - cmd: "uv run python (checks=60 budget_default=64)"; exit: 0
  - cmd: "git diff --exit-code 685a8b3..HEAD -- crates .kutha/STATE.md"; exit: 0
  - cmd: "git diff --exit-code 153dbde..HEAD -- scripts/kutha_gov/__main__.py"; exit: 0
  - cmd: "scratch empty glob then precommit --check artifact-budget"; exit: 1
  - cmd: "scratch drop independent: true then precommit --check verification-independent"; exit: 1
  - cmd: "scratch missing read_only_inputs then precommit --check verification-independent"; exit: 1
  - cmd: "uv run kutha-gov selftest"; exit: 0
  - cmd: "uv run kutha-gov precommit"; exit: 0
  - cmd: "uv run kutha-gov py"; exit: 0
  - cmd: "rg -n 'fn <six observe_cargo names>' crates/kutha-runtime/tests"; exit: 0
  - cmd: "uv run kutha-gov precommit --check verification-independent"; exit: 0
  - cmd: "uv run kutha-gov precommit --check artifact-budget"; exit: 0
read_only_inputs:
  - .planning/REQUIREMENTS.md
  - .planning/ROADMAP.md
  - .planning/v0.06-MILESTONE-AUDIT.md
  - .planning/phases/29-close-out-honesty/29-CONTEXT.md
  - .planning/phases/28-independent-verification-and-artifact-budget/28-VERIFICATION.md
  - .planning/phases/23-legal-fixture-reconnaissance/23-VERIFICATION.md
  - docs/process/gsd-auto-contract.md
  - docs/process/kutha-harness.md
  - docs/architecture/semantic-gap-review.md
  - docs/ADR/README.md
  - .kutha/META.md
  - .kutha/dictionaries/checks.yaml
  - .kutha/dictionaries/invariants.yaml
  - .kutha/dictionaries/fsm.yaml
  - .kutha/dictionaries/honeycomb.yaml
  - scripts/kutha_gov/kinds.py
  - scripts/kutha_gov/fsm.py
  - crates/kutha-runtime/src/store.rs
  - crates/kutha-runtime/src/fold.rs
  - crates/kutha-runtime/src/fold_query.rs
  - crates/kutha-common/src/event.rs
---

# Phase 29 — independent verification

**ROADMAP SC:** (1) no leased-in-progress L_delivery; audit YAML+body `gaps_found`; 28 file passed; M013 still leased S08. (2) 2026-10-02 addendum: closed F10/F11/F12/F14/F15; open F1-encoding/F7/F8 remainder/F9/F13/F16; FF5 = two VT windows at TT max. (3) touched ADRs stay Proposed; notes ≤6 lines; D1–D10 diff empty. (4) scratch budget/empty glob/missing independent each HIGH.

| REQ | Status | Evidence |
|-----|--------|----------|
| GOV-01 | satisfied | `fsm.py:240` skipped>0 HIGH; pytest 18 passed; 60≤64; `__main__.py` unchanged vs 153dbde |
| GOV-02 | satisfied | `min_hits: 1` on four budget globs; scratch 0 hits HIGH×4; selftest artifact-budget mutations=4 |
| GOV-03 | satisfied | glob min_phase 22 hits 22–28 (7; this file makes 8); 23 dropped `/root/law-nexus`; 22–28 diffs that deletion only; honor-system `gsd-auto-contract.md:57` |
| GOV-04 | satisfied | six required fns present; 11 new `i-f*` checks with asserts; I-F7/8/9/13/16 still deferred; no check id deleted |
| DOC-01 | satisfied | BASE/REC `[x]`; audit cites 22/23 files + `legal-fixture-recon.md`; crates/STATE empty |
| DOC-02 | satisfied | addendum lists closed/open; FF5 matches `fold_query.rs:64` `live_at(u64::MAX, vt)` |
| DOC-03 | satisfied | `store.rs` jsonl rename vs `File::create` snapshot; `allows_by_name`; `KUTHA_RESERVED_RELS` len 5; honeycomb-ledger OK |
| DOC-04 | satisfied | harness `ci` = checks→selftest→cargo; `--skip-cargo` skips observe only; META `min_hits`; py green |

**Score:** 8/8. CBM stale (`metadata_changed`); `rg` arbiter. Scratch under `tmp/` deleted. The plans/the summaries were not used as inputs.
