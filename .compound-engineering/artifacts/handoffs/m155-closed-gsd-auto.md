---
ce-handoff/v1: true
title: "M155 closed (checkpoint is a cut) — B2 fork diff next"
date: 2026-10-23
branch: work/main-line
head: a2f7b4b
resume_focus: "After the user asks to continue: lease B2 (ADR-061 fork diff over the now-honest fork_at) with a fresh RED, bundle STATE+ROADMAP+CHANGELOG+README, run through subagents under the GSD-auto controller protocol. Standing owner orders: do not ask permission to commit; clean temp files and target after each cycle; update docs; look for semantic gaps and glue results into one workflow. Harness moratorium applies."
---

# Handoff — M155 closed, B2 next

## Closed

- **M155 / GSD v1.49 / Phase 229:** `Runtime::fork_at -> Result` (`ForkCutUnavailable { cut, log_len }`), Horn cascade parity after a checkpoint (exact derivation records in the checkpoint payload; delivery-key and event-id indexes rebuilt from restored facts), `migrate_compat` refuses `Op::Checkpoint`. Also closed: pre-existing reopen defect for multi-path derived heads (one EventId, two causes).
- Flow: audit (read-only subagent) found F1-F11 -> docs/hygiene commit -> lease M155 -> plan -> two executor plans -> four independent verification rounds (each fresh or re-run verifier; 7 HIGH found and fixed). Tests: `b3_checkpoint_cut.rs`, `b3_checkpoint_cut_kill{,2,3}.rs` (+ M154 `b1_*`). `KUTHA_KILL_CASES` deepens the kill3 property (default 16).
- Closeout: workspace tests 0, `kutha-gov ci --jobs 1` `run_high=0`, bench no regression (see CHANGELOG Product entry).

## Next

1. **B2** ADR-061 fork diff: pure `diff_folds` keyed by (event id, valid from, valid to), sorted, no new Op; falsifier 'fork diff exact on a counterfactual fixture'; scout report conclusions are in the M155 chat; re-scout quickly before lease.
2. Residuals needing a fired RED: StoreBlob digest on open; `horn_behavior_reproducible` fail-open; dangling lineage ids; vintage payload; `cause_of_behavior` last-wins; checkpoint operator surface; B1b vacuum.
3. Before B3: Samyama and FalkorDB baseline environment with the owner (pausing `fd_api` / `9router` needs the owner's OK).

## Notes

- A foreign project (`/root/prime-agent`) runs cargo on this host and perturbs benches; use `taskset -c 0` and record load.
- rustfmt follows `mod` declarations: subagents must format only files without `mod` children or restore stray files.
