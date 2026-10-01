---
artifact_contract: "ce-handoff/v1"
created_at: "2026-10-01T10:47:19Z"
title: "M013 S03 + process productivity + D-06 write-path"
summary: "S03 leased with fold indexes shipped; D-06 ingest gate still open; process --skip-cargo/GSD config uncommitted; MCP project jina/exa/keenable added."
keywords: ["M013", "S03", "Phase-24", "D-06", "skip-cargo", "FxHashMap", "MCP", "kutha-gov"]
cwd: "/root/kutha-graph"
resume_focus: "Controller D-06 decision (accept write-cost / thin indexes / soften gate) or FxHashMap spike; optionally commit process wave via ce-commit"
repository: "kutha-graph"
repo_root_sha: "1ee5942d3f12fafc1be6983f45d0713958047900"
branch: "chore/m013-roadmap-reset"
head: "199807d487314596d15a9bf5483b1b7e30d88b26"
---

# Handoff: M013 S03 continuity

## Objective / user intent

User (controller) drove M013 GSD auto through Phases 22–23 closed, Phase 24 indexes implemented, then process-productivity audit/opts, D-06 ingest regression investigation, Exa research on write-path options, and MCP wiring (jina/exa/keenable into project config). Chat language: Russian; docs/code: English.

## Lease (authoritative)

Read `.kutha/STATE.md` first.

- Active Milestone **M013**, Active Slice **S03**, `L_delivery=M013-S02-done`
- Product HEAD already has fold ingest indexes: commit `199807d` (`feat(24): fold ingest indexes with constant visit oracle`)
- Do **not** start S04–S07, M014/M015, Rocks/M002, legal pack, Cypher/HNSW
- Honeycomb stays **Proposed**; `.kutha/STATE.md` is controller-only

## Completed this session (relative to HEAD + working tree)

### Product (committed)

- Phase 22 baseline + Phase 23 legal fixture recon closed earlier on this branch
- Phase 24 indexes + visit oracle green in `199807d`
- D-06 absolute ingest vs Phase 22 **not met** (+17…+28% before reserve; +21…+28% after reserve spike)

### Product (uncommitted spike)

- `Runtime::reserve_ingest_indexes` + `Vec::with_capacity(1)` on fact/claim buckets; bench pre-sizes before timer
- Files: `crates/kutha-runtime/src/{fold,quantum}.rs`, `benches/baseline.rs`, `docs/architecture/benchmark-baseline.md` (reserve spike section), CHANGELOG note
- Visit oracle still green; reserve alone does **not** clear ≤10% D-06

### Process (uncommitted)

Plan: `.compound-engineering/artifacts/plans/2026-10-01-process-productivity-from-log-audit.md`

- `kutha-gov ci --skip-cargo` + fail-fast after HIGH (skip cargo/tenant only; emit_log kept); `phase_ms` stdout
- Tests updated; pytest green when run
- `.planning/config.json` quick wins: `skip_discuss`, no worktrees/Nyquist/UI/AI-phase, `create_tag=false`; keep `plan_check`+`verifier`
- `docs/process/gsd-auto-contract.md` gate stacking + reindex discipline; `kutha-harness.md` skip-cargo line
- `.planning/WINDOWS.md` waive 1/3/5/6; fixed 8; open 2/4/7/9
- `23-VERIFICATION.md` trimmed ≤60 lines
- `AGENTS.md` rg vs ast-grep note (uncommitted)

### MCP / tooling (machine-local)

- Project `.cursor/mcp.json` now: CBM + jina + exa + keenable (keenable shape: url + `X-API-Key` via `${env:KEENABLE_API_KEY}`, no `type`)
- Approvals refreshed under machine-local `/root/.cursor/projects/root-kutha-graph/mcp-approvals.json`
- Composio/X/Gmail disabled for this project via `mcp-disabled.json` (plugins may still appear in catalog until new session)
- Keys live in machine-local `/root/.bashrc` (`EXA_API_KEY`, `KEENABLE_API_KEY`, `JINA_API_KEY`) — **do not copy secrets into handoffs or commits**
- This chat's dynamic tool catalog may still lack jina/exa/keenable until a **new session**; Exa was used via HTTP API when MCP namespace missing

## Interim decision (2026-10-01, locked)

Durable note (do not lose): `.compound-engineering/artifacts/research/notes/2026-10-01-d06-index-lease-interim-decision.md`

- North star: indexes as ADR-040 leases (not SoT; not forever-eager on every assert).
- Optional cheap spike: FxHashMap; do not assume it clears ≤10% D-06.
- Else: thin/lazy write **or** process honesty (visit-oracle gate + write-cost tradeoff).
- RocksDB cited as LSM analogy only — M002 stays frozen.
- Rocks parallel: eager secondary (txn Get→SingleDelete→Put) vs deferred/lazy reconcile; BlobDB/lazy blob resolution = keep hot path thin.

## D-06 status (blocker for S03 close)

Authoritative numbers and interpretation: `docs/architecture/benchmark-baseline.md` § Phase 24 delta + Reserve spike.

Phase 22 ref `ingest_plain.n` ≈ 95.4 ms. After indexes: still ~+21…+28% with reserve. Visit-oracle (IDX-02) green. Controller must choose:

1. Accept write-cost tradeoff and close S03 (independent verify still required)
2. Thin index shapes / fewer maps on write
3. Soften D-06 to visit-oracle (process change)
4. Next cheap code spike from Exa research: **FxHashMap / rustc-hash** on EventId/TermId; then get_mut vs entry; longer-term delta+frozen CSR lease (Aletheia-style) aligned with ADR-040

## Authoritative references (what matters)

| Path | Why |
|------|-----|
| `.kutha/STATE.md` | Lease S03 |
| `docs/process/gsd-auto-contract.md` | Controller protocol + gate stacking (edited) |
| `docs/architecture/benchmark-baseline.md` | D-06 numbers + reserve spike |
| `.compound-engineering/artifacts/plans/2026-10-01-process-productivity-from-log-audit.md` | Process opt plan |
| `.planning/phases/24-fold-indexes-for-ingest/` | Phase 24 CONTEXT/PLANs/SUMMARYs; independent verify still pending if closing |
| `AGENTS.md` | Two planes, freeze, CBM primary, no /tmp for durable CE handoffs |
| `.cursor/mcp.json` | Project MCP set |

## Unfinished / fragile

- Large uncommitted mix: process harness + product reserve spike + AGENTS + planning; do **not** commit unless user asks (`ce-commit`); split product vs process if committing
- Many untracked `.cursor/skills/gsd-*` / `.gsd/` noise — do not add unless asked
- S03 not closed; no independent Phase 24 VERIFICATION yet for closeout
- Branch `chore/m013-roadmap-reset` unpushed relative to prior session notes
- `agent mcp list` from CLI may still report empty after config edits; IDE session reload is the real gate for MCP namespaces

## Abandoned / do not retry blindly

- Killing ideation `http.server` — process already dead; bind `0.0.0.0` intentional for `http://164.68.106.228/`
- Expecting `reserve` alone to pass D-06 ≤10%
- Disabling Composio/X via mcp-disabled does not fully remove plugin namespaces from an already-open session catalog

## Plausible next steps (user chooses)

1. **D-06 decision** → then independent verify + lease close S03, or FxHashMap spike first
2. **ce-commit** process wave (and/or reserve spike) with CHANGELOG already drafted
3. **New chat** to pick up jina/exa/keenable MCP namespaces, then continue Exa-guided write-path work

## Skills

- Commit: `ce-commit` (Russian: коммит)
- Changelog already touched; `kutha-changelog` if extending Process/Product entries
- Resume this handoff: `/ce-handoff resume` with the path below
