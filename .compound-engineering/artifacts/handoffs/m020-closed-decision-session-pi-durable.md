---
artifact_contract: "ce-handoff/v1"
created_at: "2026-10-07T11:44:07Z"
title: "M020 closed — decision brief + Pi Durable harness patterns"
summary: "M020/v0.14 shipped; Phase 46 C01/Horn locked; TR-02 and CorrectInterval retract open; Pi Durable mapped to harness-only adoptions."
keywords: ["M020", "C01", "HORN-KC", "ASOF-ELIG", "TR-02", "Pi Durable", "harness", "handoff"]
cwd: "/root/kutha-graph"
resume_focus: "Controller lease choice: (1) doc/spike lease for TR-02 + C01 appliesTo pin, or (2) CorrectInterval retract fix, or (3) harness replay/request_id spike from Pi map — do not reopen Phase 46."
repository: "kutha-graph"
repo_root_sha: "1ee5942d3f12fafc1be6983f45d0713958047900"
branch: "chore/m013-roadmap-reset"
head: "23d1575c02a5f0931bfb02888b1324a278a24c63"
worktree_path: "/root/kutha-graph"
---

# Handoff — M020 closed, decision session, Pi Durable

## Objective / user intent

- M020 / GSD v0.14 (Phases 63–67) executed and closed with independent VERIFICATION.
- User asked for architecture capability summary, then: decide C01/Horn decision-session questions sensibly.
- Latest: **fix current state** + study Pi Durable patterns via subagent (`https://pi.dev/changelog` + user Russian Durable summary). No lease opened; no ADR/behavior edits from the decision brief.

Attribution: Phase 46 locks were controller+owner GSD auto (discuss skipped), not a fresh owner vote in this chat. Agent re-validated them and proposed TR-02 / retract / appliesTo spike — **user has not yet approved a lease**.

## Work completed (this arc)

- M020 CLOSED: hygiene, harness fail-closed, atomic persist (`replace_via_tmp`), Horn/fold residuals, process contract. `L_delivery=M020-closed`. Active Milestone/Slice **None**.
- Close commit: `23d1575` (`chore(close): M020 / v0.14 audit correction independently verified`).
- `/tmp/cursor-sandbox-cache` cleaned (~20G).
- Decision research: ADR-013 Verdict 2026-10-06 already chose C01 pack-stamp, HORN-KC ingest-at-head, ASOF support-live. DET-02 / HORN-CORR killed in Phase 47.
- Pi Durable: parent fetched `pi.dev/changelog` (mostly coding-agent 1.0.x releases); subagent grounded patterns against Earendil Durable post + Cloudflare PiHarness and mapped to Kutha (read-only).

## Authoritative state pointers

| What | Where / what matters |
|------|----------------------|
| Harness lease | `.kutha/STATE.md` — Active None; Next = controller decision; thaw map unchanged |
| GSD progress | `.planning/STATE.md` — v0.14 shipped |
| Milestone residuals | `.planning/milestones/v0.14-MILESTONE-AUDIT.md` |
| C01 / Horn / ASOF locks | `docs/ADR/ADR-013-bitemporal-facts-invalidation.md` L160–168 (Phase 46 Verdict); do not invert |
| Phase 46 context | `.planning/phases/46-c01-knowledge-cut/46-CONTEXT.md` Locked decisions |
| Matrix rows still `blocked` | `docs/architecture/test-matrix.yaml` SEM-c01-decision, SEM-horn-kc, SEM-asof-elig, SEM-tr02-legal (empty oracle) |
| Killed | SEM-det02-ids, SEM-horn-correct (M018-S03) |
| C01 oracle | `crates/kutha-runtime/tests/m016_golden_fixture.rs` `fixg01_c01_sunset_inexpressible_as_horn` |
| HORN-KC pin | `crates/kutha-runtime/tests/m017_known_on_horn.rs` `known04_*` |
| ASOF pin | `m011_claim_supports` `asof_elig01_as_of_contains_derived_when_ineligible` |
| Retract residual | `fold_query.rs` `fact_index_by_event_id` first-live; `fold.rs` `Op::Retract` single invalidate |
| Product SoT / commit | sealed KUTHASEG + manifest; `committed_prefix` / QuantumOutcome; `replace_via_tmp` |
| Harness plane | `docs/process/kutha-harness.md`; `scripts/kutha_gov`; `.kutha/events.jsonl` |
| Env for cargo/ci | `set -a && source .env && set +a && unset CARGO_TARGET_DIR` (+ ROCKSDB_*); mold rustflags in cargo config |

## Decision brief (agent, pending owner lease)

Elegant principle already in Phase 46: **no fifth clock; express the rest as facts and queries.**

| Topic | Status | Agent note |
|-------|--------|------------|
| C01 | Locked (a) pack stamp | Prefer wording: core has nothing to add; pack records `appliesTo` / application stamps via Assert — not “out of wedge forever” |
| HORN-KC | Locked ingest-at-head | Derived EventIds are TT-hidden by `as_known_on` like Asserts |
| ASOF-ELIG | Locked support-live | Eligibility = `derivation_eligible_at`; do not hide ineligible from `as_of` |
| TR-02 | **Still open** | force=`valid_from`; publication=`TtAnchor`/`as_known_on`; decision clock=`as_of` arg — falsify with fixture, no newtypes |
| CorrectInterval Retract | **Defect** | Retract by event id should invalidate all live residuals for that id |
| Agent facade | Suggestion | Pack/facade compose live∩eligible; never change `as_of` |

Recording needs a leased slice (ADR Evidence + matrix + CHANGELOG; honeycomb stays Proposed). Behavior change → RED oracle first.

## Pi Durable → Kutha (subagent memo distilled)

Sources: user Durable summary; Earendil Pi Durable / Cloudflare PiHarness (changelog page alone is coding-agent noise). Full mapping in session transcript / subagent `e90edb0f-8043-4789-91aa-46de492f1a04`.

**Adopt on harness plane only (not Rust graph core):**

1. FSM kind `replay:` + interrupted marker (`fsm.yaml` / `kinds.py`) — unsafe: `observe_cargo`, `emit_tenant`.
2. Harness `request_id` on `append_run` (analog of product `delivery_key`, not a substitute).
3. Constitution hash at change-site in process JSONL.
4. Governor Partial/Full: `running` then terminal (mirror product crash-without-QO honesty).
5. Named Strict Replay conformance on `fold_log` (two folds byte-equal).

**Reject:** TS/Pi clone in `kutha-runtime`; transcript as product SoT; LLM compaction of the event log; Cloudflare DO/`pi_*` as Kutha store; cheap conversation-fork replacing `Runtime.fork_at`; auto-replay side-effect kinds; mid-quantum dictionary hot-swap; collapsing L_map/L_delivery/L_capability into one session doc.

Pi does **not** reopen C01/HORN-KC/ASOF; does **not** fix CorrectInterval retract; TR-02 stays pack stamps, not Chord documents.

## Uncommitted / local

- Branch ahead with M020 work at `23d1575`.
- Untracked CE artifacts only (old handoffs dirs, reviews, one plan) — not part of M020 lease commits.
- No Active Milestone — do not mutate `.kutha/STATE.md` without a new lease.

## Wrong paths to avoid

- Re-litigate Phase 46 by inverting `known04_*` or making `as_of` hide ineligible rows.
- Fifth bi-temporal axis for C01/TR-02.
- Treat `valid_to` as application-end (FIXG-01 pin).
- Commit / open PR / bump SemVer without user ask.
- Index rebuild CBM/`gitnexus analyze` rewriting AGENTS.md.

## Plausible next steps (exclusive forks — user picks)

1. **Lease A — decisions/docs:** TR-02 pin + C01 appliesTo spike (Assert-only) + ADR-013/matrix Evidence; no core algebra change.
2. **Lease B — fold defect:** RED then fix Retract-all-live-for-event-id (CorrectInterval residuals).
3. **Lease C — harness honesty:** Pi-inspired `replay:` / `request_id` / running→terminal on governor (H0 only).
4. **Idle:** leave lease empty; continue literature/controller planning only.

## Skills for resume

- `ce-plan` / `ce-work` if a lease is chosen; `ce-handoff` for resume; `kutha-changelog` if docs land; CBM per `.cursor/rules/code-graph-cbm.mdc`.
)
