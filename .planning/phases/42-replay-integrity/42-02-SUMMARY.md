---
phase: 42-replay-integrity
plan: 02
subsystem: runtime
tags: [REPL-01, REPL-02, REPL-03, REPL-04, KNOWN-03, D-04, GREEN]
requires: [{ phase: 42-replay-integrity, provides: RED oracles }]
provides: ["GREEN m017_replay_integrity; D-04 open policy"]
affects: [42-verify]
actuals: { tokens: 7262, tasks: 3, commits: 3 }
plan_head_before: 3c8b5d5924c8ddbfecfd89e7b58dc1851de007bc
plan_head_after: 855da9c6d996b34f210c014c2d614d23a4ccad24
tech-stack: { added: [], patterns: ["tmp+sync+rename snapshot/WAL; covering WAL; sidecar equal-or-absent"] }
key-files: { created: [], modified: [crates/kutha-runtime/src/store.rs, crates/kutha-runtime/src/fold.rs, crates/kutha-runtime/src/hydrate.rs, crates/kutha-runtime/src/provenance.rs, crates/kutha-runtime/src/wal.rs, crates/kutha-runtime/src/quantum.rs, crates/kutha-runtime/tests/m017_replay_integrity.rs, CHANGELOG.md, docs/ADR/ADR-010-event-log-runtime-quantum.md] }
key-decisions:
  - "D-04: prefer covering WAL; jsonl fallback; fail closed on id conflict"
  - "Persist outcomes from log rows, not Typed buffers (sidecar equality)"
requirements-completed: [REPL-01, REPL-02, REPL-03, REPL-04, KNOWN-03]
duration: 21min
completed: 2026-10-03
status: complete
---

# Phase 42 Plan 02: GREEN replay integrity

**Open fails closed on WAL/jsonl id conflict; snapshot-held TtAnchor walls fail replay; snapshot/WAL atomic replace; sidecar equals log or absent; FoldIndexMode pinned; as_known_on rustdoc is TT-only.**

D-04: jsonl SoT; WAL lease. Prefer WAL iff recover magic `KUTHAWAL`+v1 and ids cover jsonl in order (WAL may be longer). Header-only/magic-only/empty/short/recover-err + readable jsonl → jsonl. Neither covers → InvalidData. No jsonl + invalid WAL → InvalidData.

Workspace `cargo test --workspace --offline`: **166 passed / 0 failed** (50 suites). `kutha-gov ci` **exit 0**. `.kutha/STATE.md` byte-clean.

| key | Phase 30 | run a | run b | vs P30 run b |
|-----|----------|------:|------:|-------------:|
| ingest_plain.n | 126962516 | 303357414 | 280920210 | **+121% STOP** |
| as_of.n | 288999 | 322652 | 269763 | −6.7% (within 10%) |

D-09 ingest_plain vs last product close (Phase 30) is a controller STOP. as_of holds on run b. No ingest-path change intended this plan.

## Mechanical test edits (m017)

| old | new |
|-----|-----|
| `File::create(snapshot.json)` then drop | `store::abort_snapshot_replace_before_rename` |
| `File::create(events.wal)` then drop | `store::abort_wal_replace_before_rename` |

D-07: no FF5/FF6/M010–M016 assertion edits.

**Graph (CBM inbound / GN upstream / rg crates); rg wins:** persist 31/5/153; open 48/19/298; finish_open 1/—/5; recover_events 2/—/3; replay_check 76/0/73; deserialize 0/—/18; from_env 207/10/4; as_known_on 10/0/61; known_at 1/—/10. Did not `index_repository`.

## Deviations from Plan

**1. [Rule 2] Persist outcomes from log.** Typed `buffer_receipt` rows are not log-native; writing `outcome_records()` made honest persist+open fail sidecar equality. `hydrate.rs` `outcomes_from_events`. Commit `c4d031b`.

## Self-Check: PASSED
