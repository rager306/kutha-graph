# ADR-014: Cascade Budgets & Quantum Receipts

## Status

**Proposed** (Time/Composition/Verify — bound the quantum; constant-size evidence per emit→idle)

### Evidence (2026-10-02; Proposed)

Historical (superseded by the 2026-10-04 blocks below): Same persist story as ADR-012: `events.jsonl` rename-atomic; `OUTCOMES_REL` / `JUSTIFICATIONS_REL` sidecars still written on `persist` (`store.rs`). Receipt stays on the in-log quantum path via `pipeline`. **2026-10-03 note:** snapshot/WAL in-place truncate is false; see Evidence 2026-10-03.

### Evidence (2026-10-03; Proposed)

Historical (superseded by the 2026-10-04 blocks below): Outcomes are `Op::QuantumOutcome` log records (`Runtime::persist_outcome_event` → `outcome_op` in `pipeline.rs`). `store::persist` still writes `OUTCOMES_REL` / `JUSTIFICATIONS_REL` sidecars as leases (`write_outcomes` / `write_justifications`: `File::create` in place, no file sync or parent-dir fsync — SNAP-01). **2026-10-03:** `snapshot.json` and `events.wal` use tmp+`sync_all`+rename (`replace_snapshot`, `replace_events_wal`); persist atomicity for those files closed in M017 S02. `replay_check` calls `check_tt_anchors`; `GraphFold::fingerprint` is facts-only; `provenance_fingerprint` is a separate hash. Commit-marker vs `Op::QuantumOutcome` encoding is COMMIT-01 (gap-review F11). M002 is thawed and planned as GSD v0.12 (slices S01–S07, `.kutha/ROADMAP.md` § M002): SNAP-01, parent-dir fsync, partial-sidecar-as-absent, `check_tt_anchors` log-ahead tolerance, and fail-closed WAL open are scheduled in S02; COMMIT-01 in S05 (decision gate). This subsection does not rewrite D014-1…D014-5.

### Evidence (2026-10-03, Phase 50 / M002 S02; Proposed)

SNAP-01 closed by Phase 50: outcome and justification sidecars are tmp+`sync_all`+rename with parent-dir fsync; partial/prefix/unparseable sidecar = absent. Historical (superseded by Phase 53 option A): COMMIT-01 (commit-marker vs `Op::QuantumOutcome`) remains S05. This subsection does not rewrite D014-1…D014-5. Status stays **Proposed**.

### Evidence (2026-10-03, Phase 52 / M002 S04; Proposed)

Between checkpoints, outcome and justification sidecars stay stale leases (Phase 50 prefix-absent on open). `Persister::checkpoint` / `close` refresh them through `replace_via_tmp`. Acknowledged-vs-unacknowledged is `append`'s `Ok`, not the sidecar. Historical (superseded by Phase 53): COMMIT-01 remains S05. This subsection does not rewrite D014-1…D014-5. Status stays **Proposed**.

### Evidence (2026-10-04, Phase 53 / M002 S05; Proposed)

COMMIT-01 option A: `Op::QuantumOutcome` remains the User-quantum commit marker. An uncommitted suffix (NeedsOutcome ops after the last outcome, or a product quantum with no outcome at all) is hidden on `open` via `commit::committed_prefix`. Self-commit set: `Define`, `TtAnchor`, `StoreBlob`, `JustificationCite`, `QuantumOutcome`, `kutha:*` Assert. F11 is closed by option A. Options B/C stay as an explicit residual (not shipped; S06 shipped segments, not commit frames). Residual: Typed/Dictionary multi-event groups have no outcome, so a crash inside such a group can still expose a partial group (`Define`-level atomicity only). This subsection does not rewrite D014-1…D014-5. Status stays **Proposed**.

### Current encoding (HEAD, 2026-10-04; Proposed)

F11 closed by option A (`Op::QuantumOutcome` + `commit::committed_prefix`). Cousin durability is `KUTHAWAL` on a legacy dir or segment `sync_data`; optional external Merkle/chain anchor is absent (`verify_sealed` is in-file SHA-256, prev seeded `[0u8;32]`). Residuals (2026-10-04): Typed/Dictionary group has no outcome; no multi-writer lock; `Persister` not wired into `kutha-tenant`. Pointers: `store::persist_until`, `segment::persist_log`, `Persister`, `rocks_index::SegmentIndex`.

### Evidence (2026-10-05, Phase 60 / M019 S05; Proposed)

WRITER-01 ships the advisory `kutha.lock` (local FS, `WouldBlock`, readers unlocked). GROUP-01 stays (c): F11 option A does not hide a complete mid-group Typed/Dictionary SelfCommit prefix; no format change. Residual wording: Typed/Dictionary group still has no outcome (characterization `mixed_typed_dict_partial_group_is_visible`); no fsync of the `kutha.lock` directory entry; advisory local lock only (NFS / another host not covered); writes that bypass `Persister` / `store::*` are not covered; `kutha-tenant` is covered only through `store::persist`; no external chain anchor; `open` O(N). Oracles: `m019_dur_a.rs`. This subsection does not rewrite D014-1…D014-5. Status stays **Proposed**.

### Current encoding (HEAD, 2026-10-05; Proposed)

F11 closed by option A (`Op::QuantumOutcome` + `commit::committed_prefix`). Writers take advisory `kutha.lock` (local filesystem only). Cousin durability is `KUTHAWAL` on a legacy dir or segment `sync_data`; optional external Merkle/chain anchor is absent (`verify_sealed` is in-file SHA-256, prev seeded `[0u8;32]`). Residuals: Typed/Dictionary group has no outcome (GROUP-01 (c)); no fsync of the `kutha.lock` directory entry; advisory local lock only (NFS / another host not covered); writes that bypass `Persister` / `store::*` are not covered; `kutha-tenant` is covered only through `store::persist`; no external chain anchor; `open` O(N). Pointers: `store::persist_until`, `segment::persist_log`, `Persister`, `rocks_index::SegmentIndex`.

### Evidence (2026-10-06; Phase 47 / M018 S03; Proposed)

`commit_user_cascade` admits each follow-on (`ReceiptMode::User`) before `pending.push`. LIFO `pending.pop` is unchanged. A hop that fails `admit` never appears in the log. UnknownRelation only when a matched firing Horn's head is missing from the dict. Oracles: `casc01_disallowed_second_hop_absent_from_log`, `casc01_unknown_head_on_matched_horn_is_err`. This subsection does not rewrite D014-1…D014-5. Status stays **Proposed**.

### Clarification (2026-10-08; Phase 80 / M024 S02; Proposed)

Write-path atomicity for intern Define and D014-1 receipt shape (G-A09 / G-A10):

1. **Define SelfCommit:** `Runtime::intern` appends `Op::Define` without a User quantum / `QuantumOutcome`. `store::committed_prefix` keeps a Define (and other SelfCommit) prefix with no trailing QO. A following product `Assert` (NeedsOutcome) without QO is dropped on open — Defines before that Assert remain.
2. **D014-1 constant commitment:** `QuantumReceipt::digest` is `[u8; 32]` (hex form 64 chars on the log outcome). Size does not depend on cascade width.
3. **Width leak residual:** `event_ids: Vec` / `events_in_quantum` remain resume leases that may expose quantum width. Stripping them is a later format bump, not this cell's Accepted gate.
4. **GROUP-01:** Typed/Dictionary multi-event groups still have no outcome; a mid-group crash can expose a partial SelfCommit prefix (unchanged residual (c)).

Oracles: `m024_write_path.rs`. This subsection does not rewrite D014-1…D014-5. Status stays **Proposed**.

### Clarification (2026-10-09; Phase 88 / M028 S01; Proposed)

**Retract of a `kutha:*` Assert is SelfCommit** (CE review P0 #1). `class_with_dict` already treated reserved Asserts as SelfCommit; Retract of those asserts stayed `NeedsOutcome`, so `committed_prefix` dropped a trailing `emit_retract_dict` and `open` resurrected the allow. Dictionary mode still writes no `QuantumOutcome`. Oracles: `trailing_emit_retract_dict_survives_persist_open`, `trailing_dict_retract_without_user_qo_survives_open`. This subsection does not rewrite D014-1…D014-5. Status stays **Proposed**.

## Date

2026-08-18

## Honeycomb coordinates

- Axes: **Time** (primary) · **Composition** · **Verify**
- Depends on: ADR-000 (D2, D3, D9), ADR-002 (Cui), ADR-010
- Anticipates: ADR-030 (budget traits), ADR-031 (scheduling under V — **not opened here**), ADR-050 (meta-prompt version on receipts), ADR-091 (portable witness)

## Context

ADR-010’s cascade can storm. STCA already names modules as generalized knapsack items whose feasible envelope is **max-convolution**. Separately, regulated workflows need **one evidence item per quantum**, not a syslog sidecar and not a public chain as SoT.

Grounding cards:

- `paper-max-convolution-budgets` — `.compound-engineering/artifacts/research/applicability/cards/paper-max-convolution-budgets.md`
- `paper-constant-size-evidence` — `.compound-engineering/artifacts/research/applicability/cards/paper-constant-size-evidence.md`
- `paper-query-admission-control` — `.compound-engineering/artifacts/research/applicability/cards/paper-query-admission-control.md`
- Adapter mapping (crate identifiers, not new cards): `.compound-engineering/artifacts/research/ruvector-plugin-adaptation.md` (P-Retrieval-Receipt; `ruvector-proof-gate` is a D014-2 cousin, not a D014-n)

Max-convolution / tropical `(max,+)` composes pack budgets: CSR rebuild vs HNSW vs agent vs materializer share envelope \(V\). Admission of *one* query is a different noun (gate vs envelope). Constant-size cryptographic evidence (Kao; Crosby–Wallach log ADS) is the literature shape of **quantum receipts**: fixed-size tuple, hash/Merkle link, optional epoch anchor. Graph ADS / blockchain papers are **anchors**, not SoT (`paper-blockchain-graph-ads` contrast).

## Decision

### D014-1. One receipt per quantum, not per vendor span

Each completed (or budget-aborted) emit→idle quantum emits a **constant-size** evidence item: commitments to admitted events, I/O, environment digest, link into the log’s hash/Merkle structure, authenticator. Size does not leak payload. This certifies **that the engine quantum existed**. It is not how-provenance (ADR-011) and not an LLM envelope certificate.

### D014-2. Budgets compose by max-convolution, not a workflow engine

Cascade/agent/materializer local functions \(h_i(v)\) compose by tropical max-convolution into a global envelope. The LLM does not pick \(v\). Hard FSM is not the allocator (D3). Pack scheduling *under* a fixed \(V\) (competition among materializers) is **ADR-031**, out of this cell.

### D014-3. Anchor ≠ SoT

Optional epoch root-of-roots (Merkle, transparency log, even a chain) may **anchor** receipts. The semantic event log remains SoT; Rocks WAL is its storage-durability cousin (ADR-010), not another source of semantic truth. Public blockchain as graph store is rejected.

### Clarification (2026-10-04, Phase 56 / M019 S01; Proposed)

The D014-3 "Rocks WAL is its storage-durability cousin" sentence is **superseded** as a current claim. Cousin at HEAD is `KUTHAWAL` on a legacy dir (`writes_legacy_layout`) **or** segment `sync_data` on `KUTHASEG`. Optional external Merkle/chain anchor is **absent**. This subsection does not rewrite D014-1…D014-5 and does not mark this cell Accepted.

### D014-4. Receipt may bind control-plane versions

A receipt may include `meta_prompt_version` and dictionary snapshot ids (ADR-050). This cell does not define those entities. Content-addressed LLM/tool cache keys (D6) belong in the quantum’s evidence so replay is cheap.

### D014-5. Read provenance receipts (Proposed adapter mapping)

D014-1 certifies write quanta (emit→idle). A separate noun is a tamper-evident **read** commitment: the evidence subset handed to an agent for one query. Shape is the constant-size / Merkle family already named by `paper-constant-size-evidence`. When a query surface exists (ADR-071) and STATE names that pack, the adapter identifier is `ruvector-retrieval-receipt` (P-Retrieval-Receipt). Read receipts must not become a second write SoT. Per-query admission remains D014-2 (`paper-query-admission-control`); do not collapse that gate into this receipt.

### Clarification (2026-09-13): partial progress and recoverable outcomes

The proposed contract permits prefix commit; it does not promise rollback on budget exhaustion. An outcome must distinguish completed, budget-stopped, and failed execution, and bind the input cut, committed range, rule/environment versions, consumed budget, and continuation disposition. A constant-size commitment may reference larger evidence; it does not make that evidence optional or constant-size. The P0 receipt is not yet the full cryptographic contract above.

Completion evidence and enough information to reconstruct or explicitly reject continuation must be authoritative, not available only in a dropped receipt/cache. If a crash leaves no terminal evidence, recovery reports incomplete/unknown, never inferred success. Resumption must identify the original quantum and prevent duplicate delivery/effects (ADR-011/062); exact record encoding remains a future implementation choice.

Current `Runtime::emit` returns `Ok(QuantumOutcome)` even when its receipt says budget-aborted and may retain a committed prefix. Call success is therefore not quantum completion. Historical (pre-M012a): `store::persist` does not persist that receipt — superseded by log-native `Op::QuantumOutcome` (see Evidence 2026-10-03). Future budget-0/1/2 and crash-boundary fixtures must still test this distinction before stronger guarantees are claimed.

### Clarification (2026-09-30; Proposed)

Target contract, from docs/architecture/semantic-gap-review.md (F1): authoritative completion records are log records; sidecars and caches are leases. Outcomes, justifications, and resume evidence needed after dropping a receipt must survive as log records, not as the only copy in a dropped sidecar. Exact encoding remains an implementation choice; the SoT kind is not. This subsection does not rewrite D014-1…D014-5 and does not mark this cell Accepted.

**Hard separations:**

```text
Quantum receipt (write) ≠  Retrieval receipt (read evidence)
Quantum receipt        ≠  How-polynomial (ADR-011)
Envelope V             ≠  Per-query admission gate
Max-convolution        ≠  Airflow / Dify DAG
Merkle / ADS anchor    ≠  Event-log SoT
Budget abort           ≠  Silent truncate of truth
Hard FSM allocator     ≠  Cui envelope
```

### Open question (2026-10-01)

Historical (superseded by Evidence 2026-10-04 Phase 53): the quantum receipt, the persisted outcome (`Zero` / `Partial` / `Full`), and the cascade budget are three records of one commit. F11 is **closed by option A** (`Op::QuantumOutcome` + `commit::committed_prefix`). Options B/C stay as an explicit residual. Typed/Dictionary groups still have no outcome. This question does not rewrite D014-1…D014-5 and does not mark this cell Accepted.

## Consequences

### Positive

- Replay/fork (D9) has a sealed per-quantum handle.
- Materializers (040) can be scored as knapsack items.
- Legal/clinical/finance Layer 5 on the evidence card is GTM color, not a requirement to implement 37 niches.

### Negative / risks

- Convolution is hard in general; P0 may use a concave/greedy envelope, not the full algorithm.
- Binding meta-prompt versions before 050 exists needs a placeholder field.

### Non-goals (this ADR)

- Implementing PQ signatures or TEE.
- Opening ADR-030/031 files.
- Blockchain as database.

## Alternatives Considered

| Alternative | Why not |
|-------------|---------|
| Blockchain graph ADS as SoT | Anchor only |
| Workflow-engine orchestration | Fights reversible packs (STRATEGY) |
| Syslog sidecar as proof | Not constant-size, not bound to log offset |
| LLM picks budgets | Violates dict-first / Cui |

## Open Research Questions

1. P0 receipt layout: which hashes are mandatory vs optional authenticator.
2. Greedy/concave envelope vs full max-convolution for P0.
3. How ADR-050 versions appear on the tuple without circular crate deps.
4. Relation to RVF WitnessChain (ADR-091) — packaging of the same hashes, not a second log.

## Related Decisions

- ADR-002 — Cui composition
- ADR-010 — quantum definition
- ADR-011 — polynomials are different evidence
- ADR-040 — materializers consume budget
- ADR-050 — versions bound into receipts
- ADR-071 — hybrid retrieve (read receipts attach there when implemented)
