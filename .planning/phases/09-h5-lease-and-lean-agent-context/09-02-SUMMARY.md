---
phase: 09-h5-lease-and-lean-agent-context
plan: 02
subsystem: agent-context
tags: [H5, CTX-01, CTX-02, CTX-03, CTX-04, D-A1, D-A2, D-A3, D-A4, D-S1, AGENTS]

requires:
  - phase: 09-h5-lease-and-lean-agent-context
    provides: Operator Phase H5 leased; README Status twin; freeze byte-stable
provides:
  - AGENTS.md ≤110 lines with durable every-turn rules and no live-lease assignment tokens
  - P0 spike inventory, Leapfrog note, applicability Agent notes, README Layout rows
  - semantic-contract-validation.md header uses D-S1 snapshot wording
  - Diet ledger with grep evidence for every relocated AGENTS block
affects: [Phase 11 SEM-08 changelog narrative]

estimate:
  tokens: 36000
  tasks: 3

actuals:
  tokens: 6856
  tasks: 3
  commits: 3

plan_head_before: 7193347de576654426a817f548a9988161edd35e
plan_head_after: PENDING_TASK3
commits: 3

tech-stack:
  added: []
  patterns:
    - AGENTS.md holds durable rules; volatile facts live in owning docs
    - CHANGELOG Process pointer only; SEM-08 narrative reserved for Phase 11

key-files:
  created:
    - docs/architecture/p0-spike-inventory.md
    - .compound-engineering/artifacts/research/notes/leapfrog-triejoin.md
    - .planning/phases/09-h5-lease-and-lean-agent-context/09-02-SUMMARY.md
  modified:
    - AGENTS.md
    - README.md
    - CHANGELOG.md
    - .compound-engineering/artifacts/research/applicability/README.md
    - docs/architecture/semantic-contract-validation.md
    - docs/process/codex-subagents.md

key-decisions:
  - "D-A1/D-A2: AGENTS.md is 96 lines; live lease tokens removed; STATE-first freeze pointer"
  - "D-A3: diet ledger maps every removed block to a home with grep evidence"
  - "D-A4: language, planes, D1–D10, CE routing, commands, conventions 1–9, CBM forbids, Task map retained"
  - "D-S1: semantic-contract header is a 2026-09-13 M011 S03 snapshot; live lease is STATE"
  - "D-G2 as green-ci: CHANGELOG Process pointer only; SEM-08 not authored"

patterns-established:
  - "AGENTS.md heading Code graph (Cursor) is the Codex→Task map home"
  - "docs-coupling Process pointer is not SEM-08 authorship"

requirements-completed: [CTX-01, CTX-02, CTX-03, CTX-04]

coverage:
  - id: D1
    description: AGENTS.md ≤110 lines with no L_delivery= / L_map= / L_capability= / M011 is active and no card counts; STATE-first freeze pointer
    requirement: CTX-01
    verification:
      - kind: other
        ref: wc -l AGENTS.md (96); negative greps on four forbidden strings and 163
        status: pass
    human_judgment: false
  - id: D2
    description: P0 lists, Leapfrog, literature/traps/GTM, and repo tree live in D-A3 homes with AGENTS one-liners
    requirement: CTX-02
    verification:
      - kind: other
        ref: diet ledger greps in this SUMMARY
        status: pass
    human_judgment: false
  - id: D3
    description: Language policy, two planes, D1–D10, CE routing, commands, conventions 1–9, CBM forbids, GitNexus secondary, Task mapping table remain
    requirement: CTX-03
    verification:
      - kind: other
        ref: plan keep-list greps on AGENTS.md plus heading Code graph (Cursor)
        status: pass
    human_judgment: false
  - id: D4
    description: semantic-contract-validation.md header uses D-S1 snapshot wording; AGENTS has no stale S03 live lease
    requirement: CTX-04
    verification:
      - kind: other
        ref: grep as of 2026-09-13, M011 S03 and .kutha/STATE.md; L_delivery=M011-S03-done count 0
        status: pass
    human_judgment: false

duration: 7min
completed: 2026-09-30
status: complete
---

# Phase 9 Plan 02: AGENTS diet Summary

**AGENTS.md slimed to 96 durable operating-rule lines; volatile P0/literature/Leapfrog/layout/lease captions relocated with a grep-backed diet ledger**

## Performance

- **Duration:** 7min
- **Started:** 2026-09-30T05:59:28Z
- **Completed:** 2026-09-30T06:06:47Z
- **Tasks:** 3
- **Files modified:** 9

## Accomplishments

- `AGENTS.md` is **96** lines (≤110). Product snapshot is four bullets plus a P0 pointer. Live lease assignment tokens are gone; agents read `.kutha/STATE.md` first and do not start work it does not name.
- P0 in/not-in lists, Leapfrog paper note, applicability Agent notes, and README Layout rows hold the former AGENTS facts. Two layout prohibitions stay in AGENTS.md.
- `## Code graph (Cursor)` remains the Codex-role → Cursor `Task` map (referenced by `docs/process/codex-subagents.md` and `.cursor/rules/code-graph-cbm.mdc`).
- semantic-contract header is a historical snapshot as of 2026-09-13, M011 S03; live lease is `.kutha/STATE.md`. Body below the header untouched.
- `uv run kutha-gov ci` HIGH 0 LOW 0. Docs-only wave: no graph verification; `git diff` against `crates/` is empty.

## Diet ledger

| Old AGENTS block | New home | Verification grep | Observed count |
|------------------|----------|-------------------|----------------|
| P0 in-spike / not-in-spike lists | `docs/architecture/p0-spike-inventory.md`; pointers in `AGENTS.md` and `README.md` | `Materializer`, `leapfrog intersect`, `ADR-050`, `RocksDB` in inventory; `p0-spike-inventory.md` in AGENTS and README; `Not in spike` in AGENTS | inventory 1/1/1/1; AGENTS pointer 1; README pointer 1; AGENTS `Not in spike` **0** |
| Literature bound / trap cluster / GTM | `.compound-engineering/artifacts/research/applicability/README.md` § Agent notes; AGENTS keeps `do not mint aggregator waves` without a count | `Agent notes`, `Graphiti`, `Dify`, `Harvey-as-SoT`, `LegalSearch-R1-as-engine`, `finance/clinical` in applicability README; `do not mint aggregator waves` in AGENTS; `163` in AGENTS | Agent notes 1; Graphiti 1; Dify 1; Harvey 1; LegalSearch 1; finance/clinical 1; waves 1; AGENTS `163` **0** |
| Leapfrog Triejoin note | `.compound-engineering/artifacts/research/notes/leapfrog-triejoin.md` | `1210.0481` in note; `1210.0481` in AGENTS | note 2; AGENTS **0** |
| Repo layout tree | `README.md` Layout table | `CLAUDE.md`, `Cargo.toml`, `config.json`, `codebase/`, `intel/`, `INGEST-CONFLICTS.md`, `dictionaries`, `events.jsonl`, `.cursor/`, `docs/ADR`, `docs/process`, `scripts/kutha_gov`, `.compound-engineering/artifacts` in README; `ports/` and `kutha-runtime` still in AGENTS | each README needle ≥1; AGENTS ports/ 1; kutha-runtime 1 |
| Long Codex subagent essay | `docs/process/codex-subagents.md` (gap-filled); AGENTS keeps pointer + Task map | `codebase-memory-mcp`, `do not inherit`, `does not lease` in codex-subagents.md; `codebase-memory-scout` and `Code graph (Cursor)` in AGENTS | mcp 1; inherit 1; lease 1; scout 1; heading 1 |
| Governor-check how-to paragraph | `.kutha/META.md` + `docs/process/governor-intake.md`; AGENTS one pointer | `How to add a check` in META; `invariants.yaml` in governor-intake.md; `.kutha/META.md` and `governor-intake.md` in AGENTS | META 1; intake invariants 4; AGENTS META 1; AGENTS intake 2 |
| GSD command table | Condensed in AGENTS: session planning lives in `.planning/` and must not grow a second living roadmap | `second living roadmap` in AGENTS; `.planning/STATE.md` in AGENTS | roadmap sentence 1; `.planning/STATE.md` 3 |
| Live-lease / trajectory assignment tokens | `.kutha/STATE.md` only (D-A2) | `L_delivery=`, `L_map=`, `L_capability=`, `M011 is active` in AGENTS (non-heading lines) | all **0** |
| semantic-contract live S03 caption | Header snapshot (D-S1); live pointer `.kutha/STATE.md` | `L_delivery=M011-S03-done` in that file; `as of 2026-09-13, M011 S03`; `.kutha/STATE.md` | S03 token **0**; snapshot 1; STATE 1 |

## Task Commits

1. **Task 1: End-to-end P0 inventory relocation with README pointer** - `a3bb326` (docs)
2. **Task 2: Relocate remaining volatile blocks and diet AGENTS.md** - `6f4b046` (docs)
3. **Task 3: Fix stale lease citations, diet ledger, and wave-close ci** - this SUMMARY (docs)

**Plan metadata:** recorded after state updates.

## Files Created/Modified

- `docs/architecture/p0-spike-inventory.md` - crates P0 in/not-in lists; not Accepted product
- `AGENTS.md` - durable rules only (96 lines)
- `README.md` - P0 pointer; Layout rows for the former tree
- `.compound-engineering/artifacts/research/notes/leapfrog-triejoin.md` - Leapfrog paper note
- `.compound-engineering/artifacts/research/applicability/README.md` - Agent notes (bound, traps, GTM)
- `docs/architecture/semantic-contract-validation.md` - header-only D-S1 wording
- `docs/process/codex-subagents.md` - Cursor Task naming, inherit, integrator `ci`
- `CHANGELOG.md` - Process pointer; SEM-08 narrative reserved
- `.planning/phases/09-h5-lease-and-lean-agent-context/09-02-SUMMARY.md` - this file

## Decisions Made

Followed 09-CONTEXT.md D-A1…D-A4, D-S1, D-G1. CHANGELOG is a docs-coupling Process pointer only (not SEM-08). Heading `## Code graph (Cursor)` kept so `codex-subagents.md` still resolves.

## Deviations from Plan

Committed on `main` under `git.branching_strategy: none` (#3552 warning), same as 09-01. No `--no-verify`.

CONTEXT.md D-G2 (“Phase 9 does not write CHANGELOG”) yielded to live `docs-coupling` YAML when `docs/architecture/**` or `docs/process/**` changed; SEM-08 narrative not authored.

### Auto-fixed Issues

**1. [Rule 1 - Bug] Plan verify needle `do not mint aggregator waves` is case-sensitive**
- **Found during:** Task 2
- **Issue:** Sentence-case “Do not mint…” failed the plan’s lowercase grep.
- **Fix:** Wording “Agents do not mint aggregator waves.”
- **Files modified:** `AGENTS.md`
- **Verification:** `grep -q 'do not mint aggregator waves' AGENTS.md`
- **Committed in:** `6f4b046`

**Total deviations:** 1 auto-fixed (Rule 1)
**Impact on plan:** Verify green. Meaning unchanged.

## Issues Encountered

None. `RUSTC_WRAPPER=` was set for `kutha-gov ci`; cargo observe succeeded (HIGH 0). sccache was not required.

## Trajectory (D-10 / D-G1)

1. **Commands:** `uv run kutha-gov ci`; `uv run kutha-gov explain trajectory`.
2. **Outcome:** `ci` exit 0; **HIGH 0**, **LOW 0** (32 checks). `uv run kutha-gov precommit` exit 0. `uv run pytest -q scripts/tests` — 48 passed.
3. **Explain excerpt (≤8 lines):**
   ```
   check: trajectory
   purpose: Active Milestone/Slice are None or exist on ROADMAP
   authority: none — harness does not accept ADRs or claim product readiness
   source: .kutha/dictionaries/checks.yaml
   steps:
     - file_exists  .kutha/STATE.md
     - file_exists  .kutha/ROADMAP.md
     - pointer_in_other_file  .kutha/STATE.md
   ```
4. **Honesty:** Green governor is not ADR Accepted and not L_capability.

Cited: D-A1 (≤110), D-A2 (no live-lease tokens), D-A3 (ledger), D-A4 (meaning), D-S1 (header). Docs-only: codebase-memory-mcp was not queried; this wave does not claim graph verification. `git diff --exit-code -- crates` is clean.

## Facts not relocated (already absent or intentionally omitted)

- Card count `163` is not in `AGENTS.md` (D-A2/D-A3). It remains in README Where-to-read and the applicability `axis-layer-synthesis.md` line.
- Ken Ross hash-table attribution was already absent from AGENTS at diet start; the Leapfrog note copies the then-current AGENTS sentences.
- Per-command `/gsd-*` table rows condensed to the `.planning/` overlay sentence (D-A3 GSD table). GSD skills still own those commands.
- `gsd-*` Cursor Task row dropped from the mapping table; five Codex-style roles remain. GSD agent types stay under `.cursor/agents`.
- `STRATEGY.md` has no stale S03 live-lease caption (D-S1 search: 0 hits). Left unstaged.
- `docs/process/kutha-harness.md` has no leftover `L_delivery=M011-S03-done` after 09-01. Ladder wording not reopened.

## Threat Flags

None beyond the plan register (T-09-05…T-09-08, T-09-SC). No new network endpoints, packages, or crate schema.

## Known Stubs

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Ready for Phase 10 (ADR/roadmap later-milestones) or Phase 11 SEM-08 narrative. Do not flip the H5 ROADMAP checkbox (Phase 11). Do not author SEM-08 here. Do not edit `crates/`. Do not lease M012a.

## Self-Check: PENDING_COMMIT
