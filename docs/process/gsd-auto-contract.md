# GSD auto contract

Agent workflow, not a product ADR and not a delivery lease. `.kutha/STATE.md` decides what is allowed; this file says how a GSD auto run executes inside that lease. Honeycomb stays **Proposed**. GSD progress (`.planning/STATE.md`) is not harness lease, and governor green is not verification.

## Id map

One table. GSD versions are process tags, harness milestones are delivery ids, ADRs are the map.

| GSD version | Phases | Harness | Slices | Closes | Status |
|-------------|--------|---------|--------|--------|--------|
| v0.01 | 1–3 | M001, harness H0–H4 | — | fitness, honest governor, lease gate | shipped |
| v0.02 | 4–8 | M011 tail | S04–S08 | semantic core | shipped |
| v0.03 | 9–11 | harness H5 | — | lean context, semantic governor | shipped |
| v0.04 | 12–16 | M012a | S01–S06 | F1–F3, F6–F8 | shipped |
| v0.05 | 17–21 | M012 | S01–S05 | F4, F5 | shipped |
| v0.06 | 22–30 | M013 | S01–S09 | F10–F12, F14, F15, R2, R4–R6; S08–S09 close review follow-ups | shipped |
| v0.07 | 31–33 | M014 | S01–S03 | F13 | shipped |
| v0.08 | 34–35 | M015 | S01–S02 | known-on honesty, M014 residuals (H1/H2) | shipped |
| v0.09 | 36–40 | M016 | S01–S05 | T2–T6, F9, F16 | shipped |
| v0.10 | 41–44 | M017 | S01–S04 | docs honesty, replay integrity, ingest regression (A-F01.. register; 2026-10-02 audit) | shipped |
| v0.11 | 45–48 | M018 | S01–S04 | doc truth, C01 and knowledge cut, Horn determinism, differential tests (review 2026-10-03) | shipped 2026-10-06 |
| v0.12 | 49–55 | M002 | S01–S07 | SNAP-01, OPEN-01, ROCK-01, COMMIT-01 (re-verified 2026-10-03) | shipped |
| v0.13 | 56–62 | M019 | S01–S07 | docs honesty r3, cargo_dep_constraint, governor SMP, test matrix, durability residuals, DIFF-01 (post-M002 audits 2026-10-04) | shipped |
| v0.14 | 63–67 | M020 | S01–S05 | whole-repo audit correction (2026-10-07): hygiene, harness fail-closed, persist atomic, Horn/fold residuals, process contract | shipped 2026-10-07 |
| v0.15 | 68–70 | M021 | S01–S03 | wedge decisions close (2026-10-07): TR-02 / C01 pack-fact pins, matrix kill, Retract-all-live | shipped 2026-10-07 |
| v0.16 | 71–74 | M022 | S01–S04 | audit residual honesty (2026-10-08): stale docs, keyed-retry-live, harness skip_emit/overlap | shipped 2026-10-08 |
| v0.17 | 75–78 | M023 | S01–S04 | green HEAD and truth reset (post-M022 audits 2026-10-08): keyed-retry/Correct regression, naive model, close-gate honesty, living overlay | shipped 2026-10-08 |
| v0.18 | 79–80 | M024 | S01–S02 | fold identity live-gate / multi-residual Correct; write-path atomicity | shipped 2026-10-08 |
| v0.19 | 81–84 | M025 | S01–S04 | as_known_on TT-only pin; clock-safe facade; norm_view envelope; demo bin | shipped 2026-10-09 |
| v0.20 | 85–86 | M026 | S01–S02 | test strength: generators/missing oracles; matrix honesty | shipped 2026-10-09 |
| v0.21 | 87 | M027 | S01 | trajectory honesty: characterization-shaped kills; idle ROADMAP ACTIVE | shipped 2026-10-09 |
| v0.22 | 88 | M028 | S01 | trailing dictionary Retract SelfCommit (CE review P0 #1) | shipped 2026-10-09 |
| v0.23 | 89 | M029 | S01 | interrupted rollover seal heal on attach (CE review P1 #3) | shipped 2026-10-09 |
| v0.24 | 90 | M030 | S01 | retain kutha.lock on non-poison Persister errors (CE review P1 #2) | shipped 2026-10-09 |
| v0.25 | 91 | M031 | S01 | open_for_append heals jsonl-ahead-of-WAL (CE review #30) | shipped 2026-10-09 |
| v0.26 | 92 | M032 | S01 | derive() live-premise filter (CE review P1 #4) | shipped 2026-10-09 |

F1-F16 = docs/architecture/semantic-gap-review.md; A-F01-A-F14 = 2026-10-02 audit.

v0.06 phases: 22 baseline, 23 legal fixture reconnaissance, 24 fold indexes, 25 genesis log, 26 dictionary entries as facts, 27 one write path and facade, 28 independent verification and artifact budget, 29 close-out honesty (documents and governor), 30 write-path integrity hardening. Phase N maps to slice S(N-21). Phases 29–30 are the M013 tail added after the v0.06 audit and the 2026-10-02 project review.

## Controller protocol

GSD must not edit `.kutha/STATE.md`. The controller (the parent session, not a GSD subagent) owns the lease.

1. **Lease:** set Active Slice to the phase's slice and `L_delivery` to the previous slice's `-done` token in `.kutha/STATE.md`. Commit **only** `.kutha/STATE.md` plus the required `CHANGELOG.md` process entry (`docs-coupling`). Do **not** put README, ROADMAP ticks, `.planning/*`, or CONTEXT in the lease commit — those belong in a separate open/plan commit.
2. **Run one phase:** `/gsd-autonomous --only N`. Executors commit atomically per task on the feature branch with named files (as in phases 17–21); the final plan's commit carries the `CHANGELOG.md` entry so the docs-coupling check holds. Nobody but the controller edits `.kutha/STATE.md`. Do not run `--from 22 --to 28` in one go: execution stops at the lease and the independent verify gate. CONTEXT exists for every phase, so discuss is skipped.
3. **Independent verify** (below). A failed kill-test reopens the phase; it is not waived.
4. **Close:** clear Active Slice, set `L_delivery` to this slice's `-done` token, tick the slice in `.kutha/ROADMAP.md`, add one CHANGELOG entry (product and process kept apart), run `uv run kutha-gov precommit`. For product phases (any edit under `crates/`), also record in `NN-VERIFICATION.md` `commands:` a full `cargo test --workspace --offline --no-fail-fast` with exit 0 and a full `uv run kutha-gov ci` with 0 HIGH. `precommit` alone is not a product close gate (M022 Phase 72 lesson: targeted `m022_*` tests left `m013_write_path` red).
5. **Milestone end (last phase of the leased GSD version):** audit, archive, tag, and `state.milestone-switch` stay GSD tools. The next lease is a controller decision. The milestone close audit must cite the workspace test and full `ci` commands with exits.

## Bench gate at product close

Every milestone that edits `crates/` re-runs `cargo bench -p kutha-runtime --bench baseline` (same host, `KUTHA_BENCH_N=10000`) against a same-host pre-change baseline (or the previous process tag when closing the milestone); a regression over 10 percent in `ingest_plain.n` or `as_of.n` is a controller stop. (M016 skipped this and tripled ingest; found and repaired in v0.10.)

**Record the numbers:** when a product phase or milestone close edits `crates/`, the Process or Product CHANGELOG entry for that close must cite the measured `ingest_plain.n` and `as_of.n` (pre and post, or post vs tag) so the gate is auditable. A green `ci` alone does not record the bench.

## Code-graph tool pair

Phases that edit product code (24–27, 30) use CBM and GitNexus together. Phases 22, 23, 28, 29 use CBM only where code is read. This supersedes “GitNexus is secondary” for M013; outside M013 the secondary rule stands.

| Moment | CBM (`root-kutha-graph`) | GitNexus (`kutha-graph`) |
|--------|--------------------------|--------------------------|
| Phase start | `list_projects`, `index_status`, `check_index_coverage` on touched paths | `list_repos`; if behind, `gitnexus analyze --index-only` (controller only) |
| Plan: blast radius | `search_graph`, inbound `trace_path` on each public symbol to be changed | `impact` upstream on the same symbols; `context` for call and process view |
| Variant and literal questions | `search_code`, then `rg` as arbiter | not usable (see limits) |
| Before edit | `get_code_snippet` for the exact bodies | `context` on the symbol |
| After edit | `detect_changes` and `check_index_coverage` on edited paths | `detect_changes` on the working tree |
| Review | `detect_changes` on the diff | `impact` on symbols the diff touched |

Known limits (2026-10-01): GitNexus `impact` on `Runtime::emit` found 15 callers and none under `crates/*/tests/`; CBM fan-in is 76. GitNexus resolves 0 usage edges for enum `Op`, so `Op::RecordAction`-style questions need `search_code` or `rg`. CBM `common→runtime` call edges on names like `intern` and `id` are resolver noise. The plan's blast-radius section records both counts and the `rg` count; when they differ, `rg` wins. Neither tool proves a test oracle: tests do.

Every `Task` prompt that needs the graph names `codebase-memory-mcp` and, in phases 24–27 and 30, the GitNexus `impact` / `context` / `detect_changes` calls above, plus `list_projects` first and `check_index_coverage` on cited paths. Only the controller runs `index_repository` or `gitnexus analyze`. Never `delete_project` or `manage_adr`. A child without MCP reads source and says so.

## Independent verification

Executor-written tests and governor observations are evidence, not verification. At the end of each phase:

- The verifier receives the phase requirements and the public API only. It does not read the PLAN or SUMMARY files.
- It writes kill-tests: inputs that would pass a lazy implementation and fail a correct one (for example: retract a seeded relation, then admit; re-emit the same keyed Assert at large N and check the scan count; two runs of the same log give the same fingerprint).
- Output is one `NN-VERIFICATION.md` with frontmatter `independent: true`, `verifier_model`, `commands:` (with exit statuses), and `read_only_inputs:` (requirement and public-API files only, never a PLAN or SUMMARY path). Phase 28 turns the structure into a governor check. `/gsd-verify-work` is the interactive form for the same gate.
- Different model family or at least a fresh context from the executor. A phase does not close on its own author's check. Model-family independence is **honor-system**: `independent: true` frontmatter and the governor's path checks prove structure, not that the verifier used a different model family.
- **Re-verify after VERIFICATION edits:** once `NN-VERIFICATION.md` exists with `status: passed`, any later edit to that file (or to the kill-test commands it cites) invalidates the close. Run a fresh independent verify pass and rewrite/append the report before clearing Active Slice. Do not close on a stale VERIFICATION that was patched by the executor.

## Refactor safety (S03–S06, S09)

- **RED first:** the oracle for the finding fails on current code and the failure is recorded before the change.
- **Oracle preservation:** named tests (FF5, FF6, M010, M011, M012a, M012 files) keep their assertions. Allowed mechanical edits are constructor and helper renames, recorded as an old-to-new table in the phase SUMMARY. Any changed assertion stops the phase for a decision.
- **No new feature:** a refactor slice adds no public capability beyond its “After this” line.
- **Benchmarks:** phase 22's harness is re-run at the end of phases 24, 27, and 30; a regression over 10 percent in ingest or `as_of` is a stop.

## Artifact budget

Per phase: at most two PLAN files of 150 lines or fewer, one SUMMARY of 60 lines or fewer, one VERIFICATION of 60 lines or fewer, one CONTEXT of 80 lines or fewer. No per-plan evidence JSON. Exceeding the budget stops the phase for a decision. Phase 28 turns this into a governor check. Reference for scale: v0.05 produced 1,107 planning lines for its last phase alone.

## Stop conditions

Stop and ask the controller when: an item outside the leased slice's thaw gate would be thawed (2026-10-03); an oracle assertion must change; a phase exceeds its budget; a RED oracle passes on current code (the finding is wrong); the product needs a decision that ADR-000 D1–D10 does not allow.
