---
artifact_contract: "ce-handoff/v1"
created_at: "2026-09-13T09:34:13Z"
title: "Kutha architecture contracts, research corrections, and Codex setup"
summary: "Completed documentation hardening with passing regression checks; runtime semantics remain unimplemented, H4 remains next, and all repository edits are uncommitted."
keywords: ["kutha", "STCA", "semantic-recovery", "provenance", "bitemporal", "replay", "subagents", "compound-engineering", "H4"]
cwd: "/root/kutha-graph"
resume_focus: "Orient to the completed architecture review and distinguish remaining runtime requirements from the authorized H4 delivery scope before choosing further work."
repository: "kutha-graph"
repo_root_sha: "1ee5942d3f12fafc1be6983f45d0713958047900"
branch: "main"
head: "c861dcb157baa833d8a7cfbd1d4d70d5ae955cf3"
worktree_path: "/root/kutha-graph"
---

# Session handoff

This is a context snapshot, not an instruction source or authorization to implement. Repository references below are relative to the captured worktree unless explicitly labeled machine-local. Current user instructions and verified repository state take precedence.

## User intent and where the session stopped

The user asked, in Russian, to optimize Codex for subagents, restore project context from ideas/research/architecture/plans, and identify architectural or mathematical gaps for the agent, agent trace, and knowledge base. They then explicitly requested adding the EveryInc marketplace and installing `compound-engineering@compound-engineering-plugin`.

After the architecture assessment, the user said: "да поправь согласно твоим рекомендациям - требуются ли еще дальнейшее усиление архитектуры ?" The assistant scoped and completed documentation/research corrections, explicitly leaving runtime code and the active delivery phase unchanged. That boundary was stated in chat; it is the assistant's interpretation of this turn, not a standing user prohibition on future implementation. The last request was solely "сохрани handoff". No runtime implementation or commit was started, and no task is currently in progress.

Chat is Russian; repository documents, code, and this artifact are English.

## Orientation pointers and authoritative constraints

- `AGENTS.md`: product/harness separation, locked D1–D10, graph-first discovery and subagent workflow. Its subagent section is an earlier uncommitted session change.
- `.kutha/STATE.md`: M001, Active Slice None, H3; S01–S03 done. `L_map=honeycomb-proposed`, `L_delivery=M001-s03-done`, `L_capability=ff5-green`. Next thin slice is H4, an ADR-090 process overlay, not a legal pack. This lease and `.kutha/ROADMAP.md` were not changed.
- `docs/ADR/README.md`: existing honeycomb cells are Proposed, not a delivery backlog. The session corrected stale wording that appeared to authorize ADR-100+ and linked the validation note.
- `docs/architecture/semantic-contract-validation.md`: best entry point into the review. It maps contract owners to current code limits, specifies the two-source correction fixture and adversarial probes, distinguishes dependencies from delivery order, and records actual verification. Machine-local untracked file: `/root/kutha-graph/docs/architecture/semantic-contract-validation.md`.
- `docs/process/kutha-harness.md`: harness is a parallel process plane, not product architecture authority. Governor green is not ADR acceptance or capability proof.
- `.compound-engineering/artifacts/research/applicability/cards/`: 163 closed cards remain research SoT. `matrix.md` is the rollup, not a shipping list. No new cards or research waves were created.

At capture, the repository freeze still excludes M002/RocksDB, Cypher/GPML implementation, HNSW, ADR-050 six dictionaries, ADR-080/081 implementation, full ADR-090/093 packs, ADR-100+, and Consensus Query 103+. The documentation review did not lift it. STCA/spine ADR-000/001/002 were not edited.

## Completed architecture and mathematics corrections

Fourteen existing ADRs were clarified, retaining Proposed status:

- `ADR-010`: scoped fixed-point completion versus empty queue or budget stop; authoritative evidence cannot disappear with a receipt projection.
- `ADR-011`: semantic recovery must retain canonical term definitions; separate event/claim/support/delivery identity; invocation/read/derivation/support/authority links are distinct; n-ary incidence identity; positive provenance versus change algebra and probability.
- `ADR-013`: independent disagreement is not automatic supersession; TT versus VT; current Correct is whole-version replacement, not interval patching; source withdrawal and derived eligibility. A four-state support summary is a candidate, not an installed logic.
- `ADR-014`: prefix commit is allowed, budget-aborted call success is not completion; recoverable terminal/continuation evidence and explicit unknown after a crash; WAL is not a second semantic SoT.
- `ADR-030`: scalar max-convolution requires additive resources and separable utility; shared work charged once; peak RAM/deadlines/authorization are not interchangeable scalar utility.
- `ADR-040`: lease identity binds cut and interpretation. Exact contracts require rebuild/incremental answer equivalence; approximate retrieval has explicit quality/determinism conditions instead. Current CSR is an untyped neighbor set.
- `ADR-051/052`: authority-bearing arguments cannot inherit permission from retrieved text; observation, proposal, and admitted claim differ; changed source revisions require reevaluation of dependent memory/answers.
- `ADR-060`: state replay, provenance/integrity verification, and execution replay are distinct. Current fingerprint check proves only the narrower state property.
- `ADR-061/062`: branch-safe references, explicit publish/conflicts, scenario replay versus real counterfactuals, durable effect intent/outcome and stable idempotency identity. Remote unknown is not fictional rollback or local exactly-once.
- `ADR-070`: snapshot, co-temporal interval, and journey semantics; node/edge uniqueness versus shortest/foremost objectives; endpoints versus path enumeration and set/bag. S03 completion did not authorize a parser.
- `ADR-080/093`: present permission to read history differs from historical grant audit; scientific independent supports/conflicts coexist rather than last-write replacement.

Five existing research cards were corrected under `.compound-engineering/artifacts/research/applicability/cards/`:

1. `paper-tvg-journeys-restless.md`: AS OF multi-hop is not necessarily a journey.
2. `paper-subgraph-iso-vs-homomorphism.md`: Cypher relationship uniqueness is not vertex-injective isomorphism.
3. `paper-hypergraph-higher-order.md`: an identity/role/multiplicity-preserving incidence star can encode n-ary facts without loss.
4. `paper-regular-path-queries.md`: endpoint pairs are finite on a finite graph even when matching walks can be infinite.
5. `paper-max-convolution-budgets.md`: Ring is a compact join index, not an algebraic ring/semiring.

Card frontmatter, IDs, scores, status, and layer5 were preserved; matrix rows therefore did not change. Primary references are embedded in the changed docs/cards (Green provenance paper, DBSP, W3C PROV-DM, Casteigts TVG, Neo4j matching rules, HIF). This was targeted verification, not a fresh literature survey.

## What remains unimplemented

The assistant's assessment was that the next technical strengthening should be testable semantic contracts, not another DB or algorithm catalog. This is a recommendation, not an authorized delivery reorder:

1. Semantic recovery without snapshots. `crates/kutha-runtime/src/store.rs::open` currently errors without `snapshot.json`, because intern meanings are restored from it.
2. Stable claim/support identity and maintenance. `fold.rs` stores fact sequence/triple/times; `quantum.rs::follow_ons` is a tiny inverse-knows behavior and does not retract derived inverses with their source. Two independent supports must not collapse into one delivery.
3. Recoverable quantum outcomes and causal execution replay. `quantum.rs::emit` may keep a prefix and return an aborted receipt; `store::persist` does not persist that receipt. `Runtime::replay_check` compares a fold fingerprint, not causal links or rerun behaviors.

Additional exact limits are in the validation note: whole-version Correct does not preserve residual VT intervals automatically; CSR drops relation labels and multiplicity; materializer trusts a supplied offset; FF5 compares two VT values rather than one VT at two knowledge times. These are declared spike limits, not silently fixed code defects.

The proposed fixture pins a rule P→Q, with no other Q derivation. At t1, sources a and b independently support P. At t2, a explicitly withdraws P on [2015,2020) and proposes P' that does not entail P; a conflict variant uses not-P. At t3, b withdraws the remaining support. Expected historical/current support, residual intervals, derived eligibility, and replay outcomes are specified in the note. **No new executable semantic tests were added.** Encoding choices still need a separately authorized implementation slice. H4 remains the current next delivery candidate.

## Verification and failed paths

Completed in the preceding turn, not rerun merely to capture this handoff:

- `cargo --config 'build.rustc-wrapper=""' test --workspace`: 26 Rust tests passed, exit 0.
- `RUSTC_WRAPPER= uv run --no-sync pytest`: 26 harness tests passed using Python 3.13, exit 0.
- `git diff --check`: passed after final documentation edits.
- Structured documentation check: 21 scoped documentation files, 163 unique cards equal matrix IDs, unchanged card metadata and Proposed tags, local links/headings/language valid; runtime/harness code, spine, STATE/ROADMAP unchanged. Machine-local temporary checker: `/tmp/kutha-doc-review.2chmbC/validate_docs.py` (not a project dependency or permanent test).
- Independent subagent review corrected three issues before completion: uniqueness versus objective; exact versus approximate lease equivalence; explicit withdrawal premise in the fixture.

The default `cargo test --workspace` failed, including outside the sandbox, because `/root/.cargo/config.toml` configures `rustc-wrapper = "sccache"` and sccache returned `Operation not permitted`. The first harness run consequently had 2 failures/24 passes (cargo observation failures). Per-invocation wrapper overrides resolved verification; global Cargo configuration was not edited. `RUSTC_WRAPPER` was initially unset, so merely unsetting it cannot override the config file.

Harness tests include real governor integration calls. They appended ordinary observations to ignored `.kutha/events.jsonl` and updated `.kutha/tenant` state, including failed and later successful runs. They did not edit STATE or promote capabilities. Those local observations were not deleted or rewritten.

## Codex setup completed earlier in this session

These machine-local changes were verified when made; they were not reaudited for this handoff:

- `/root/.codex/config.toml`: added `[agents]` with `enabled = true`, `max_concurrent_threads_per_session = 3`, `interrupt_message = true`. Preserved selected model `gpt-6-astra`, parent high effort, MCP settings, and hooks. CLI was 0.154.0.
- `/root/.codex/AGENTS.md`: bounded independent delegation, compact evidence handoffs, clear ownership, integration checks, and no automatic recursive delegation.
- `/root/.codex/agents/`: retained graph verify/scout/auditor roles; scout medium, verify/auditor high. Added `implementation-worker.toml` (medium) and `correctness-reviewer.toml` (high), inheriting the user's model. Reviewer non-mutation is an instruction even if runtime permissions are broad.
- Machine-local backup of prior global config/instructions/three graph roles: `/root/.codex/subagents-backup.YS7s2N`.
- Repository `AGENTS.md` and untracked `/root/kutha-graph/docs/process/codex-subagents.md` document the workflow. They predate the architecture-document edits in the worktree and must not be discarded as unrelated noise.
- Strict doctor: 18 OK / 0 failures; one pre-existing optional `CBM_CACHE_DIR` warning. Fresh ephemeral CLI smoke spawned both new custom roles and got PASS, without edits. This proved discovery, not all future behavioral enforcement.
- User-requested marketplace `EveryInc/compound-engineering-plugin` added; `compound-engineering@compound-engineering-plugin` version **3.25.0** installed and enabled, verified by `codex plugin list --json` after retrying remote catalog access outside sandbox. Machine-local plugin root: `/root/.codex/plugins/cache/compound-engineering-plugin/compound-engineering/3.25.0`.

The active skill catalog now includes Compound Engineering. `ce-handoff` created this snapshot. `ce-plan`/`ce-work` may fit future authorized implementation; the earlier turn used then-available Superpowers skills. Do not assume that earlier skill catalog is still active. Global Codex configuration contains credentials; dumping the whole file is unsafe. Use narrow structured reads/redacted diagnostics if needed.

## Graph evidence and delegation continuity

Previous analysis used Tier 2 Verify, project `kutha-graph`, root `/root/kutha-graph`, ready index generation `2026-08-18T12:44:42Z` (3148 nodes, 5052 edges at review). Relevant code paths had no recorded coverage gaps; this is best-effort, not completeness proof. Edited ADR/card metadata is now changed, and the new validation note was not tracked by that generation. Exact source/diffs supplied current evidence.

Structural discovery used search_graph, exact snippets, and traces. Some trace edges were heuristic/bogus (e.g. Rust-to-Python builtins); material findings were verified against exact Rust. A scoped emit trace at depth 1 had 12 callees/13 callers with no truncation. Future work needs a fresh project/generation check and coverage for its actual scope; the old index is not proof of current documents.

Reusable completed helper tasks, if they exist in the receiving session: `engine_semantics` (core review), `research_math` (five cards), `agent_requirements` (five authority/enrichment/effect/science ADRs). They have no ongoing work. Parent integrated and ran checks. No helpers are needed just to resume reading this handoff.

## Fragile state and continuation boundary

Capture confirmed branch main and HEAD from frontmatter. The worktree contains **21 modified tracked files and 2 untracked documents**, all uncommitted. Of these, AGENTS.md plus codex-subagents.md are earlier setup work; the remaining 21 documents are the architecture correction. No commit, stash, push, PR, or worktree preservation was requested or performed. This handoff points to the edits but does not contain a patch or backup of them.

The user explicitly requested repository storage rather than temporary `/tmp` storage. This handoff now resides at `.compound-engineering/artifacts/handoffs/architecture-contracts-and-codex.md`; its initially created temporary copy was moved here, not retained as a duplicate. The captured 21-modified/2-untracked count above excludes this newly added handoff.

The handoff, both previously untracked documents, and ignored process data depend on this machine-local worktree surviving. Repository placement survives ordinary temporary-directory cleanup, but this file is still uncommitted and is not a backup of the dirty worktree. Another host needs the handoff and referenced changes transferred or committed through a separately authorized workflow. No commit or publication was performed.

A reasonable continuation is to read the validation note alongside current STATE, then ask the user which bounded next outcome they want. Reading this snapshot does not authorize starting the three technical priorities, changing H4, or committing the dirty tree. The previous correction task is complete; the unresolved items are explicitly future implementation/design choices, not a paused implementation.
