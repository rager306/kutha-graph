---
artifact_contract: "ce-handoff/v1"
created_at: "2026-10-19T07:35:00Z"
title: "M114 closed; next GSD-auto lease is open"
summary: "observe_cargo exclusive flock on Cargo target dir; L_delivery=M114-closed."
keywords: ["gsd-auto", "M114", "observe_cargo", "flock", "kutha"]
cwd: "/root/kutha-graph"
resume_focus: "Lease the next thin residual only after the user asks to continue; do not start product edits before the lease commit."
repository: "kutha-graph"
branch: "chore/m013-roadmap-reset"
worktree_path: "/root/kutha-graph"
---

# Handoff — M114 closed, next lease open

## What landed

- Cross-process flock around `run_cargo_observation` (test + optional build).
- Env: `KUTHA_GOV_CARGO_LOCK_WAIT_SEC` (default 600). Budget 89.
- Oracles: lock timeout when held; concurrent processes serialize stub start/end.

## Resume

Controller chooses next lease. Do not thaw Cypher/HNSW/legal. Do not “fix” trajectory LOW or AUD-08 Lazy. Prefer serial `ci` after bench (flock waits; still cheaper than racing).
