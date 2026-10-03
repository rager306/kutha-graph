---
phase: 42-replay-integrity
plan: 01
subsystem: runtime
tags: [REPL-01, REPL-02, REPL-03, REPL-04, D-04, RED]
requires: [{ phase: 41-living-docs-and-governor, provides: M017-S01 closed }]
provides: ["RED oracles in m017_replay_integrity.rs; tmp/p42-01-red.txt"]
affects: [42-02]
actuals: { tokens: 2551, tasks: 2, commits: 1 }
plan_head_before: 428f3ff7451b10e1f3245f75a5d13dc4669872c6
plan_head_after: 13d15507f5080c04f817ad29883b8721b0bb1df9
tech-stack: { added: [], patterns: ["D-04 covering WAL; jsonl fallback; persist abort via File::create"] }
key-files: { created: [crates/kutha-runtime/tests/m017_replay_integrity.rs], modified: [] }
key-decisions:
  - "D-01: 8 must-fail oracles are RED; control wal_valid_covering_preferred is GREEN"
  - "Hydrate trap: tampered jsonl TtAnchor still passes replay_check (fold rebuilt from log)"
requirements-completed: []
duration: 6min
completed: 2026-10-03
status: complete
---

# Phase 42 Plan 01: RED REPL-01..04 oracles

**Failing m017 oracles pin D-04 WAL policy, hydrate-trap anchors, in-place snapshot/WAL truncate, sidecar ignore, and env-stolen index mode. Control covering WAL is green. No product fix.**

Recorded: `tmp/p42-01-tracer.txt`, `tmp/p42-01-red.txt` (exit 101; 1 passed / 8 failed). `.kutha/STATE.md` byte-clean. KNOWN-03 deferred to 42-02.

| Test | Result | Why |
|------|--------|-----|
| wal_valid_covering_preferred | PASS | honest persist/open; WAL covers jsonl |
| wal_header_only_falls_back_to_jsonl | FAIL | 12-byte WAL → recover `[]` → `ReplayDivergenceError` (not jsonl ids) |
| wal_magic_only_falls_back_to_jsonl | FAIL | 8-byte magic → `UnexpectedEof` on version; no jsonl fallback |
| wal_jsonl_id_disagreement_fails_closed | FAIL | WAL preferred; open Ok despite diverged jsonl ids |
| tampered_tt_anchor_wall_fails_replay_check | FAIL | drop WAL, rewrite jsonl wall; `from_snapshot` hydrates anchors from tampered log; fingerprint matches |
| abort_snapshot_replace_keeps_previous_bytes | FAIL | `File::create` truncates dest (0 vs 2764 bytes) |
| abort_wal_replace_keeps_previous_bytes | FAIL | same truncate on `events.wal` (0 vs 5101) |
| mismatched_outcomes_sidecar_fails_open | FAIL | spoof sidecar; open Ok (log hydrate / attach). Absent sidecar Ok (m012a) |
| snapshot_index_mode_survives_env_flip | FAIL | deserialize `from_env` → Lazy, not snapshot Eager |

**Surprise (D-04):** header-only is not silent empty Runtime; snapshot vs empty WAL fails `replay_check`. Still RED vs required Ok+jsonl. Magic-only is EOF, not empty recover.

**Graph (CBM inbound / GN upstream impact / rg crates hits); rg wins:** persist 39/5/119; open 48/5lb/276; finish_open 45/6/5; append_events 41/38/3; replay_check 76/0/69; deserialize 0/0/17; from_env 207/134/5; as_known_on 10/9/56; known_at 1/1/5. New test `not_tracked` in CBM; GN `detect_changes` missed untracked m017. Did not `index_repository`.

**D-07:** no FF5/FF6/M010–M016 assertion edits (no rename table). Combined plan’s two test commits into `13d1550` per parent.

## Deviations from Plan

**1. [process] One test commit, not two.** Parent required `test(42-01): RED REPL-01..04 oracles`. Tracer still ran first (`wal_header_only` RED, control GREEN).

## Self-Check: PASSED
