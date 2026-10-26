---
artifact_contract: "ce-handoff/v1"
created_at: "2026-10-26T02:05:34Z"
title: "M161 leased, shared-head fix uncommitted"
summary: "M160 is closed on origin. M161 is leased locally. The cause-index fix is in the working tree and is not committed."
keywords: ["m161", "cause_of_behavior", "det-02", "gsd-auto", "uncommitted"]
cwd: "/root/kutha-graph"
resume_focus: "M161 S01 is leased and the product edit is uncommitted. Independent verification and the close bundle have not been done."
repository: "kutha-graph"
repo_root_sha: "1ee5942d3f12fafc1be6983f45d0713958047900"
branch: "work/main-line"
head: "d7a65c3bb6591920521d6d11c1f2a819da2417d3"
worktree_path: "/root/kutha-graph"
---

# Handoff — M161 leased, fix not committed

Superseded by `.compound-engineering/artifacts/handoffs/m161-closed-gsd-auto.md`. The product edit was committed after this note was written.

## Objective

The user repeatedly asked to continue the Arc B roadmap through GSD-auto subagents: fix semantic and cascade gaps, update docs, commit without asking, clean `tmp/` and this repo's `target/`, and keep cargo builds to one target directory. That wording is the user's, across several turns. This handoff does not authorize the next agent to act before the current user confirms.

## What is complete

- **M160 / GSD v1.54 / Phase 234** is closed and on `origin/main` and `origin/work/main-line` at `e9b79a8` (`docs/m160: pin handoff head after close`). Ordinary `StoreBlob` bytes are checked in `replay_check` (`UnknownRuleVersion`). A missing Horn blob stays `ExecutionReplayFailed` (M159). Release id is still `0.0.01` (`VERSION`). Cargo and `pyproject.toml` stay `0.0.1` because Cargo rejects a leading zero.
- **M161 lease** is local commit `d7a65c3` (`chore(m161): lease S01 every cause of a shared head`). That commit is **not pushed** (`work/main-line` was ahead of origin by 1 when this handoff was written). Lease text: `.kutha/STATE.md` Active Milestone M161 / S01, `L_delivery=M161-S01`. Plan: `.planning/phases/235-shared-cause/235-CONTEXT.md` and `235-01-PLAN.md`.

## What is in the working tree (not committed)

The M161 executor was interrupted before it committed. The tree still has its edit. `235-01-SUMMARY.md` (untracked) says the lib test and `uv run kutha-gov precommit` were 0. This handoff session did **not** re-run those commands.

Uncommitted paths:

- `crates/kutha-runtime/src/quantum.rs` — `cause_of_behavior` type. The summary says it is now `IngestMap<EventId, Vec<EventId>>`.
- `crates/kutha-runtime/src/pipeline.rs` — record every distinct `caused_by`; `unindex_behavior` walks all of them.
- `crates/kutha-runtime/src/quantum_invariants.rs` — `#[cfg(test)]` test `shared_head_unindex_clears_every_cause` (indexes are `pub(crate)`, so this is not an integration test).
- `CHANGELOG.md` — uncommitted `## 2026-10-25 — Product: shared head causes`.
- `.planning/phases/235-shared-cause/235-01-SUMMARY.md` — untracked. The summary claims RED exit 101 then GREEN, and `--lib` exit 0.

`by_event_id`, fingerprint, and `Op` were not meant to change. `VERSION` was not meant to change.

## Not started

- Independent verifier and kill-test outside the crate tests.
- Close bundle: ROADMAP still says M161 LEASED, no `235-VERIFICATION.md`, no id-map row, no M161 handoff-after-close.
- Push of `d7a65c3` and of the product edit.

## Constraints already in force

- Do not put `0.0.01` in `Cargo.toml`. Format of release ids is `N.N.NN` in `VERSION` only.
- Do not thaw Cypher (M003) or HNSW (M005). Competitor baseline still needs the owner's OK to use docker and to pause `fd_api` / `9router`. That pause was never given.
- Harness moratorium: no new governor check without a fired RED.
- Honeycomb stays **Proposed**.
- Cargo: one `target/` directory, filtered `-p kutha-runtime --lib` for this slice. A foreign `/root/prime-agent` cargo job shares the host.

## References

- Lease and the DET-02 reason: `.kutha/STATE.md` Next action, and `.kutha/ROADMAP.md` section `M161`.
- Why two rows share an id: `Event::stable_derived` in `crates/kutha-common/src/event.rs` hashes the head, not `caused_by`.
- The index that used to last-win: `cause_of_behavior.insert` used to be in `crates/kutha-runtime/src/pipeline.rs` (both `append_indexed` and `index_live_behavior_event`). `unindex_behavior` removed only one cause.
- Prior closed handoff: `.compound-engineering/artifacts/handoffs/m160-closed-gsd-auto.md` (`head: b063f20`).
