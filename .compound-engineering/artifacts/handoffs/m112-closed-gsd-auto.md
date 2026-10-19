---
artifact_contract: "ce-handoff/v1"
created_at: "2026-10-19T05:40:00Z"
title: "M112 closed; next GSD-auto lease is open"
summary: "Selftest rejects substring-preserving replace_regex; yaml_map_list derived swaps no longer keep the needle; L_delivery=M112-closed."
keywords: ["gsd-auto", "M112", "selftest", "substring", "kutha"]
cwd: "/root/kutha-graph"
resume_focus: "Lease the next thin residual only after the user asks to continue; do not start product edits before the lease commit."
repository: "kutha-graph"
branch: "chore/m013-roadmap-reset"
worktree_path: "/root/kutha-graph"
---

# Handoff — M112 closed, next lease open

## Objective

Standing GSD-auto execute loop: thin vertical slices, TDD, CBM, docs/changelog, atomic commit. Honeycomb stays Proposed. Cypher / HNSW / legal pack stay frozen. ADR-000 D3 still rejects a hard FSM as sole control.

## What landed (M112 / Phase 186)

- `selftest` classifies `replace_regex` as UNPROVEN when `with` still contains `pattern` as a substring.
- Derived `yaml_map_list` require_fields / nonempty_list swaps use `__selftest_rm_<field>__:` (was `selftest_removed_<field>:`, which still contained `id:` / `freeze_as:`).
- Check `selftest-substring-honesty` + invariant; matrix `SEM-m112-selftest-substring` killed; budget 88.

## Verification

- `uv run pytest scripts/tests/test_m112_selftest_substring.py` — 2 passed
- `cargo test --workspace` — EXIT:0
- quiet bench N=10000 — EXIT:0
- `uv run kutha-gov ci` — `run_high=0` `tip_high=0`, 88 checks (one expected LOW: trajectory / no Active Slice)

## Resume

Controller chooses next lease. Do not thaw Cypher/HNSW/legal. Do not “fix” trajectory LOW or AUD-08 Lazy. Product candidates deferred: delivery_key last-wins remints; facts_by_relation keep invalidated for historical TT.

## Untracked / leave out of M112 commit

- `.compound-engineering/artifacts/reviews/2026-10-15-ce-whole-repo-current-head/`
- older handoffs/research under `.compound-engineering/artifacts/` not authored this wave
- live `.env` (gitignored; budget bumped locally to 88)
