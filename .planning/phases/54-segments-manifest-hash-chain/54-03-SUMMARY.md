# 54-03 SUMMARY — Migrate, export, sealed tamper

Executor Phase 54 / M002 S06 plan 54-03 only. Not verification. Honeycomb stays Proposed. Did not edit STATE/README/REQUIREMENTS/ROADMAP or write VERIFICATION. Deviation: rolled 54-01/54-02 SUMMARYs into `54-01-SUMMARY.md` so this file fits artifact-budget n≤2.

## Commits

- `c79927d` test(54-03): RED oracles for migrate, export and sealed tamper
- `4810eaf` feat(54-03): migrate committed prefix to segments and export jsonl
- `e695d98` test(54-03): prove open does not rewrite a smashed jsonl export
- this commit: Evidence, bench table, CHANGELOG, SUMMARY, governor YAML

## RED (`m002_seg01`)

Failed 6: migrate_preserves_ids_and_fold_fingerprint, migrate_drops_uncommitted_suffix_and_reports, migrate_is_idempotent, migrate_crash_matrix_reopens_without_loss, append_after_migrate_resumes, export_jsonl_roundtrips_and_is_never_read_on_open. Writer tamper + mutate_jsonl + legacy_dir_without_migrate preserved (48 pass / 6 fail).

## Decision (D-07)

Migrate only the committed prefix; report `dropped_uncommitted`. Jsonl stays an export. WAL removed after verify-reopen. Idempotent if manifest exists (also drops leftover WAL).

## Blast radius (CBM `root-kutha-graph`; rg arbiter)

Coverage: `segment.rs` / `m002_seg01.rs` `not_tracked`; store `metadata_changed`. No `index_repository`. `detect_changes` since RED: `store.rs`, `m002_seg01.rs` (plus unrelated untracked CE artifacts). `persist` still never calls migrate.

## old → new

Workspace 292 tests (was 271; `m002_seg01` 54). `store::migrate` / `export_jsonl` added. Existing M010–M017 / snap/append/commit oracles unchanged.

## Bench (D-09)

`be57466` vs `e695d98`, 4 interleaved pairs, `tmp/p54-03-*-target`. ingest 108.7M → 116.4M (+7.1%); as_of 297164 → 322731 (+8.6%); open.n 2.759s → 2.767s (+0.3%; +1.5% vs 54-02 2.726s). persist.n −0.5%; persist_append.n −2.3%. **No ≥10% median regression.** Worktrees removed.

## Mutations

All nine kill-mutations failed an oracle (`git checkout --` restore; tracked tree clean): skip chain; CRC-only; auto-convert in persist; read jsonl on manifest; WAL before manifest flip; skip verify-reopen; export used by open; swap-detection removed; trailer bytes check removed.

## Controller must change (not edited here)

- `AGENTS.md` L20 product plane “event log = SoT”; L29 Formula; L99 integrator `.kutha/events.jsonl` (harness, not product encoding).
- `README.md` L3 “append-only event log is the source of truth”; L52 `.kutha/` gitignored `events.jsonl`.
- `.kutha/ROADMAP.md` L265 thaw “keeps `events.jsonl` as SoT”; L281–282 S06 “the jsonl export is no longer the SoT” (and S06 checkbox when closing).
- `.planning/REQUIREMENTS.md` L10 DOC-06 “jsonl SoT with covering WAL”.

## Residual

Typed/Dictionary group hole; no multi-writer lock; WAL duplication gone only for segmented dirs; `rocks_spike` still jsonl until S07. Harness `.kutha/events.jsonl` untouched.
