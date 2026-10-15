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
| v0.27 | 93 | M033 | S01 | CI multi-residual admission VT (CE review P1 #8) | shipped 2026-10-09 |
| v0.28 | 94 | M034 | S01 | Correct Horn re-fire except-set (CE review P1 #9) | shipped 2026-10-09 |
| v0.29 | 95 | M035 | S01 | replay binds reserved dict (CE review P1 #10) | shipped 2026-10-10 |
| v0.30 | 96 | M036 | S01 | VT-aware Horn live-triple dedup (CE review P1 #11) | shipped 2026-10-10 |
| v0.31 | 97 | M037 | S01 | norm_view claim-lineage eligibility (CE review P1 #13) | shipped 2026-10-10 |
| v0.32 | 98 | M038 | S01 | policy delivery-key namespace (CE review P2 #29) | shipped 2026-10-10 |
| v0.33 | 99 | M039 | S01 | stub CI quantum honest name (CE review P1 #7) | shipped 2026-10-10 |
| v0.34 | 100 | M040 | S01 | vacuous planning gates (CE review P1 #16) | shipped 2026-10-10 |
| v0.35 | 101 | M041 | S01 | persist-trace identity + shrink ddmin (ADR-023) | shipped 2026-10-10 |
| v0.36 | 102 | M042 | S01 | m028×layouts + shrink corpus (ADR-023 / P0 #1) | shipped 2026-10-10 |
| v0.37 | 103 | M043 | S01 | FSM quantum honest name (CE review P1 #17) | shipped 2026-10-10 |
| v0.38 | 104 | M044 | S01 | nameless dict retain (CE review P2 #20) | shipped 2026-10-10 |
| v0.39 | 105 | M045 | S01 | glob_absent comment policy (CE review P2 #22) | shipped 2026-10-10 |
| v0.40 | 106 | M046 | S01 | parallel check-error parity (CE review P2 #23) | shipped 2026-10-10 |
| v0.41 | 107 | M047 | S01 | observe-skipped breadcrumb (CE review P2 #39) | shipped 2026-10-10 |
| v0.42 | 108 | M048 | S01 | parallel Context flags (CE review P2 #48) | shipped 2026-10-10 |
| v0.43 | 109 | M049 | S01 | skip-cargo log marker (CE review P2 #38) | shipped 2026-10-10 |
| v0.44 | 110 | M050 | S01 | Op compat documented break (CE review P1 #14) | shipped 2026-10-10 |
| v0.45 | 111 | M051 | S01 | pin scheme v1 stability vectors (CE review P1 #24) | shipped 2026-10-11 |
| v0.46 | 112–115 | M052 | S01–S04 | CE whole-repo wave 1 (advisor demotes #1; #8/#7/#11) | shipped 2026-10-11 |
| v0.47 | 116 | M053 | S01 | CE review regression enrolment + M054 gap map | shipped 2026-10-11 |
| v0.48 | 117–121 | M054 | S01–S05 | CE residual gaps (Horn → persist → governor → scaling → ID) | shipped 2026-10-11 |
| v0.49 | 122 | M055 | S01 | CE #6 append tip rollback (Other keep-lock; poison after rollover) | shipped 2026-10-11 |
| v0.50 | 123–127 | M056 | S01–S05 | advisory residuals (lease-bundle → persist #29/#30 → #26/#28 → hygiene) | shipped |
| v0.51 | 128 | M057 | S01 | docs-coupling STATE→ROADMAP (lease-bundle hole at git_path_implies) | shipped |
| v0.52 | 129 | M058 | S01 | disk_free preflight (YAML min_free_gib) + VERIFICATION bench markers off gitignored tmp | shipped |
| v0.53 | 130 | M059 | S01 | budget-template-align (.env.example ↔ fsm defaults.budget via cite_equals) | shipped |
| v0.54 | 131 | M060 | S01 | budget_covers_checks (fsm defaults.budget ≥ check count in precommit) | shipped |
| v0.55 | 132 | M061 | S01 | env_int_at_least (live KUTHA_GOV_BUDGET ≥ fsm defaults.budget when set) | shipped |
| v0.56 | 133 | M062 | S01 | Op migrate AllowRelation (explicit remap; open fail-open) | shipped |
| v0.57 | 134 | M063 | S01 | Op migrate RegisterRule (StoreBlob + kutha:rule Assert) | shipped |
| v0.58 | 135 | M064 | S01 | Op migrate PinPolicy (StoreBlob + kutha:policy Assert) | shipped |
| v0.59 | 136 | M065 | S01 | Op migrate RecordAdmission (kutha:admission Assert) | shipped |
| v0.60 | 137 | M066 | S01 | Op migrate RecordAction (kutha:action Assert) | shipped |
| v0.61 | 138 | M067 | S01 | Op wire directory migrate (migrate_op_wire + op-wire.v1) | shipped |
| v0.62 | 139 | M068 | S01 | Op wire WAL-only migrate (recover_json_values + fail-open frames) | shipped |
| v0.63 | 140 | M069 | S01 | Op wire segmented migrate (segment Values → demote jsonl + marker) | shipped |
| v0.64 | 141 | M070 | S01 | Op-wire demote orphan seg-*.log cleanup + G4 residual honesty | shipped |
| v0.65 | 142 | M071 | S01 | Pin-wire Behavior migrate (migrate_pin_wire + pin-wire.v1) | shipped |
| v0.66 | 143 | M072 | S01 | Pin-cite JustificationCite migrate (migrate_pin_cite + pin-cite.v1) | shipped |
| v0.67 | 144 | M073 | S01 | Git auto clean-tree merge-base + ci run_high/tip_high labels | shipped |
| v0.68 | 145 | M074 | S01 | Agent surface discoverability (CBM name + AGENTS CLI flags) | shipped |
| v0.69 | 146 | M075 | S01 | Pin migrate sequence facade (migrate_pins = wire then cite) | shipped |
| v0.70 | 147 | M076 | S01 | Compat migrate sequence facade (migrate_compat = op-wire then pins) | shipped |
| v0.71 | 148 | M077 | S01 | Multi-check CLI select (--check comma-separated ids) | shipped |
| v0.72 | 149 | M078 | S01 | Compat migrate operator CLI (kutha-migrate → migrate_compat) | shipped |
| v0.73 | 150 | M079 | S01 | Repeatable --check flags (argparse append → join) | shipped |
| v0.74 | 151 | M080 | S01 | Observe build both operator bins (tenant + migrate) | shipped |
| v0.75 | 152 | M081 | S01 | Built migrate bin discoverability (AGENTS path, not cargo run) | shipped |
| v0.76 | 153 | M082 | S01 | Process notes built migrate path (op-compat + pin-compat) | shipped |
| v0.77 | 154 | M083 | S01 | Harness CutPort migrate-bin (kutha-harness Space table) | shipped |
| v0.78 | 155 | M084 | S01 | .env.example built migrate path (debug/kutha-migrate) | shipped |
| v0.79 | 156 | M085 | S01 | json --check append oracle (parity with precommit) | shipped |
| v0.80 | 157 | M086 | S01 | selftest --check append oracle (parity with precommit/json) | shipped |
| v0.81 | 158 | M087 | S01 | Harness --check surface discoverability (json|precommit|selftest) | shipped |
| v0.82 | 159 | M088 | S01 | AGENTS --check not for ci (json|precommit|selftest only) | shipped |
| v0.83 | 160 | M089 | S01 | --check reject redirect hint (json|precommit|selftest) | shipped |

F1-F16 = docs/architecture/semantic-gap-review.md; A-F01-A-F14 = 2026-10-02 audit.

Whole-repo (2026-10-11) #1 is **not** fix-first — see
`.compound-engineering/artifacts/reviews/2026-10-11-ce-whole-repo/advisor-correction.md`.


v0.06 phases: 22 baseline, 23 legal fixture reconnaissance, 24 fold indexes, 25 genesis log, 26 dictionary entries as facts, 27 one write path and facade, 28 independent verification and artifact budget, 29 close-out honesty (documents and governor), 30 write-path integrity hardening. Phase N maps to slice S(N-21). Phases 29–30 are the M013 tail added after the v0.06 audit and the 2026-10-02 project review.

## Controller protocol

GSD must not edit `.kutha/STATE.md`. The controller (the parent session, not a GSD subagent) owns the lease.

1. **Lease (bundle):** set Active Milestone/Slice and `L_delivery` in `.kutha/STATE.md`. In the **same** lease commit include: `CHANGELOG.md` Trajectory entry (`docs-coupling`); `.kutha/ROADMAP.md` with an open `- [ ] **S##:` checkbox for the leased slice (and the M### section if new); `README.md` Status row naming that Active Milestone/Slice so `state-readme` stays green. Do **not** put `.planning/*`, CONTEXT, or product code in the lease commit — those belong in open/plan/execute commits. A lease that only touches STATE+CHANGELOG is a known false-start: `trajectory`/`state-readme` go HIGH until the bundle lands (M056 lesson). `docs-coupling` also HIGH when STATE is in the diff without ROADMAP (M057).
2. **Run one phase:** `/gsd-autonomous --only N`. Executors commit atomically per task on the feature branch with named files (as in phases 17–21); the final plan's commit carries the `CHANGELOG.md` entry so the docs-coupling check holds. Nobody but the controller edits `.kutha/STATE.md`. Do not run `--from 22 --to 28` in one go: execution stops at the lease and the independent verify gate. CONTEXT exists for every phase, so discuss is skipped.
3. **Independent verify** (below). A failed kill-test reopens the phase; it is not waived.
4. **Close:** clear Active Slice, set `L_delivery` to this slice's `-done` token, tick the slice in `.kutha/ROADMAP.md`, add one CHANGELOG entry (product and process kept apart), run `uv run kutha-gov precommit`. For product phases (any edit under `crates/`), also record in `NN-VERIFICATION.md` `commands:` a full `cargo test --workspace --offline --no-fail-fast` with exit 0 and a full `uv run kutha-gov ci` with 0 HIGH. `precommit` alone is not a product close gate (M022 Phase 72 lesson: targeted `m022_*` tests left `m013_write_path` red).
5. **Milestone end (last phase of the leased GSD version):** audit, archive, tag, and `state.milestone-switch` stay GSD tools. The next lease is a controller decision. The milestone close audit must cite the workspace test and full `ci` commands with exits.

## Bench gate at product close

Every milestone that edits `crates/` re-runs `cargo bench -p kutha-runtime --bench baseline` (same host, `KUTHA_BENCH_N=10000`) against a same-host pre-change baseline (or the previous process tag when closing the milestone); a regression over 10 percent in `ingest_plain.n` or `as_of.n` is a controller stop. (M016 skipped this and tripled ingest; found and repaired in v0.10.)

**Record the numbers:** when a product phase or milestone close edits `crates/`, the Process or Product CHANGELOG entry for that close must cite the measured `ingest_plain.n` and `as_of.n` (pre and post, or post vs tag) so the gate is auditable. A green `ci` alone does not record the bench.

## Code-graph tool pair

Phases that edit product code (24–27, 30) use CBM and GitNexus together. Phases 22, 23, 28, 29 use CBM only where code is read. This supersedes “GitNexus is secondary” for M013; outside M013 the secondary rule stands.

Canonical CBM project name is `kutha-graph` (historical alias `root-kutha-graph` on the same root — prefer `kutha-graph`; see `.cursor/skills/codebase-memory/SKILL.md`).

| Moment | CBM (`kutha-graph`) | GitNexus (`kutha-graph`) |
|--------|---------------------|--------------------------|
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
- Output is one `NN-VERIFICATION.md` with frontmatter `independent: true`, `verifier_model`, `commands:` (with exit statuses), and `read_only_inputs:` (requirement and public-API files only, never a PLAN or SUMMARY path). Do not list gitignored scratch (`tmp/**`) in `read_only_inputs:` — store durable bench/evidence under the phase directory (M058 lesson). Phase 28 turns the structure into a governor check. `/gsd-verify-work` is the interactive form for the same gate.
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
