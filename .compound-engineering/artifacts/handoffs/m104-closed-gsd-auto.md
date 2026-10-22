---
artifact_contract: "ce-handoff/v1"
created_at: "2026-10-18T07:27:44Z"
title: "M104 closed; next GSD-auto lease is open"
summary: "GSD-auto waves M100 through M104 are committed on chore/m013-roadmap-reset; L_delivery is M104-closed and no slice is leased."
keywords: ["gsd-auto", "M104", "kutha", "ce-review-2026-10-15", "lease"]
cwd: "/root/kutha-graph"
resume_focus: "Lease the next thin residual only after the user asks to continue; do not start product edits before the lease commit."
repository: "kutha-graph"
repo_root_sha: "1ee5942d3f12fafc1be6983f45d0713958047900"
branch: "chore/m013-roadmap-reset"
head: "5ac95b05806d9af3913885b1e356f25865beb25c"
worktree_path: "/root/kutha-graph"
---

# Handoff — M104 closed, next lease open

## Objective

The user repeatedly asked to move the repo with research, plan, and execute via GSD auto (`gsd-autonomous`), one steel thread at a time. This is the controller's reading of that intent, not a new user directive inside this handoff. Honeycomb stays Proposed. Cypher, HNSW, and a legal pack stay frozen.

## Current state

Complete and committed. `L_delivery=M104-closed`. Active Milestone and Active Slice are None. `.kutha/STATE.md` next action says the controller chooses the next lease.

| Wave | Lease | Close | What landed |
|------|-------|-------|-------------|
| M100 / v0.94 / phase 171 | `a8060ec` | `3c1a7d2` | Correct and CorrectInterval unindex a Behavior |
| M101 / v0.95 / phase 172 | `a97c129` | `dc5a6a3` | Rule pin overlaps the trigger window |
| M102 / v0.96 / phase 173 | `19e0563` | `ef9bcb8` | Failed migrate verify drops `seg-*.log` |
| M103 / v0.97 / phase 174 | `269706f` | `f3575e4` | `from_strings` keeps the first TermId |
| M104 / v0.98 / phase 175 | `9266d42` | `5ac95b0` | `cause_of_behavior` unindexes one cause bucket |

HEAD is `5ac95b0` `fix(175-01): close M104 with O(1) unindex via cause pointer`. Id map row is `docs/process/gsd-auto-contract.md` v0.98 / 175 / M104.

## Decisions (controller calls, not new user orders)

- One phase per user turn. Discuss skipped because the controller wrote CONTEXT before execute. GSD subagents did not edit `.kutha/STATE.md`.
- Lease commit is only `.kutha/STATE.md`, `.kutha/ROADMAP.md`, `CHANGELOG.md`, `README.md`. Close commit carries product, tests, matrix, planning packets, and the closed lease.
- M104 oracle counts cause-bucket visits. RED on a full scan was `[32, 128]`. Premise-rel buckets still scan all values; that was left out of the slice on purpose.
- `from_strings` keeps vector positions and only the map is keep-first. Op-wire emit-once for mixed Defines was not done.

## Verification last recorded

M104 close: `cargo test --workspace --offline --no-fail-fast` exit 0; quiet bench N=10000 `ingest_plain.n` 112401628 ns, `as_of.n` 321088 ns (prior band about 101–131M ingest and 241–342k as_of); `uv run kutha-gov ci` `run_high=0` `tip_high=0` cargo=ok tenant=ok. Numbers live in `.planning/phases/175-unindex-cause-pointer/175-VERIFICATION.md`.

After the user asked to delete tmp and target, the first `--offline` cargo failed until `cargo fetch --locked` restored crates. That fetch is done; later M103 and M104 gates used `--offline`.

## Untracked and not in commits

Leave these unstaged. They were already dirty before these waves:

- Modified historical review `.compound-engineering/artifacts/reviews/2026-10-11-ce-whole-repo/report.md`
- Untracked `.compound-engineering/artifacts/handoffs/m089-closed-gsd-auto-check-debt.md`
- Untracked review tree `.compound-engineering/artifacts/reviews/2026-10-15-ce-whole-repo-current-head/`

`tmp/` was emptied and recreated. Repo `target/` and the machine-local cargo dir `/tmp/cursor-sandbox-cache/a43da72db70e2acc74455aa660335cf6/cargo-target` were removed; later builds recreated the cargo dir.

## Residuals still open

Named in the 2026-10-15 review residual list, not leased. The controller's preferred next thin slices, in no user-ordered sequence: policy-pin election full scan (`live_policy_pin_at` in `crates/kutha-runtime/src/fold_query.rs`); premise-rel half of unindex (still walks every `behaviors_by_premise_rel` bucket in `crates/kutha-runtime/src/pipeline.rs`); op-wire emit-once for mixed Defines (`rewrite_op_wire_values` in `crates/kutha-runtime/src/store.rs`). Single-combination DFS was deferred as a larger design. Do not thaw honeycomb.

## How this repo runs the next wave

Read `.kutha/STATE.md` then `docs/process/gsd-auto-contract.md` controller protocol. Lease before any product edit. Chat with the user is Russian; docs, code, and commits stay English. Commits use a message file and named paths. Do not force-push or skip hooks.
