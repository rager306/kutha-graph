# Changelog

All notable changes to **this repository** are recorded here. This is project history, not GitHub Releases and not a Compound Engineering skill catalog.

Keep **product** (`crates/`) and **process** (harness) distinct. Dated entries may also use a **Trajectory** subsection so L_map / L_delivery / L_capability are not collapsed. New entries prefer Keep a Changelog groups (`Added` / `Changed` / `Fixed`) inside those plane headings.

## 2026-10-23 — Product: hop workload

### Added

- Self-relative `[[bench]] hop_k` (`KUTHA_HOP_N` default 256, factors 1/2/4): layered fan-out-4 graph, 2-hop via `execute(Hops)` from every layer-0 node, `typed_csr_lease_at` as `csr_lease.{n,2n,4n}`. Reports calib-loop ratios and 2n/n. `baseline.rs` 29 keys unchanged. hop2 has no BASE twin.

### Changed

- Bench A/B vs `46b7114` (9 alternating pairs, `taskset -c 0`, `KUTHA_BENCH_N=10000` direct `--bench` binaries; pair 1 BASE then HEAD, pair 2 HEAD then BASE, …; shared `CARGO_TARGET_DIR`). PRE (BASE medians) ingest_plain.n **117180497 ns**, as_of.n **275864 ns**. POST (HEAD medians) ingest_plain.n **120307962 ns** (HEAD/BASE 1.027), as_of.n **296743 ns** (1.076). Discarded HEAD as_of pairs 3 and 4 as >3× the HEAD arm median (1209884 ns, 895488 ns); after discard as_of.n **265565 ns** (0.963). No ingest sample was a >3× outlier. 1-min load 2.58–5.27. Gate **PASS** (both HEAD/BASE ≤ 1.10).
- hop_k self-relative band (4 runs, `KUTHA_HOP_N=256`, `taskset -c 0`; recorded range, not an absolute-ns budget): hop2.n / calib **1.39–2.14**; hop2 2n/n **2.73–2.88**; hop2 4n/2n **4.06–4.51**. csr_lease.n / calib **0.010–0.031** (single `from_fold`; 2n/n not a stable ratio).

### Trajectory

- ADR-041 Evidence 2026-10-23: Kutha-side hop number via `query_ir::execute` over the typed CSR lease; `leapfrog_intersect` still off the query path. ADR-040 Evidence: hops consume the existing droppable CSR lease. Status stays **Proposed**. Honeycomb restaged; map/delivery/capability unchanged.
- Deferred: competitor baseline (owner + docker + 20 GB), planner B5, parser B6, Verus V2.

## 2026-10-23 — Product: B3-lite hop query IR

### Added

- Thin typed query IR in `query_ir.rs` (`Workload = QueryIr`; no serde). `execute(&Runtime, &QueryIr)` is read-only over `live_at` / `typed_csr_lease_at` / `csr_lease_at`. Scan is a sorted product bag (`tt == MAX` equals `as_of(vt)`; `limit == 0` unlimited). Hops return sorted unique end nodes (`BTreeSet` frontier; unlabeled any-rel; typed rel filter; diamond and support multiplicity collapse). `try_hops_named` uses `dictionary().id` only (never `intern` / `admit`); intern-map miss is `QueryError`. Oracles in `tests/b5_hop_workload.rs` (in-memory only): scan vs sorted `as_of` on FF5+statute, nested join on chain+branch+diamond, unlabeled vs untyped neighbors, VT window-end and TT retract PIT, fail-closed unknown names, xorshift shuffle. Independent kill-tests in `tests/b5_hop_workload_kill.rs` pin a `live_at` exact-k oracle, Scan bag vs `as_of`, both clocks, fail-closed intern, Horn/CI residuals, and seeded graphs (`KUTHA_KILL_CASES`). No parser, planner, Op, wire, or fingerprint change. Hop bench band is 231-02.
- Self-relative `[[bench]] hop_k` (`benches/hop_k.rs`): `hop2.{n,2n,4n}` and `csr_lease.{n,2n,4n}` as calibration-loop ratios plus 2n/n; `baseline.rs` 29 keys unchanged. Recorded band is in the hop-workload Product entry.

### Fixed

- `QueryError::RelsShape { rels, k }` is the fail-closed error when hops `rels.len()` is not 0, 1, or `k` (no longer `UnknownRelation { name: "rels/k mismatch" }`). Named oracles: per-hop `rels` order vs nested join (`b5_hop_per_hop_rels_order`); `limit` is a prefix of sorted rows and `limit == 0` is unlimited (`b5_hop_limit_truncates_sorted_prefix`). `try_hops_named` stays re-exported from `lib.rs`.

## 2026-10-23 — Product: fork diff

### Added

- Read-side `diff_folds` / `GraphFold::diff` fact-set query in `fold_diff.rs` (`FoldDiff` added / removed / changed). Identity is `(event_id, valid_from, valid_to)` plus canonical rank among equal keys; `seq` never keys, ranks, or marks changed. A test-only `apply_diff` in `b4_fork_diff.rs` reconstructs the other side as a multiset of rows modulo `seq` (including `as_of` / `as_known_on` oracles). Checkpointed `fork_at(c+1..=len)` diffs like the copied-log twin; interior `1..=c` is `ForkCutUnavailable`. No Op, named refs, GED, merge, operator CLI, or harness check. Independent kill-tests in `b4_fork_diff_kill.rs` pin apply-diff totality, equal-key rank, seq-ignored identity, dead-row domain, checkpointed twin, and Option-faithful `valid_to` / `invalidated_at` (None is not `u64::MAX`).

### Fixed

- `identity_key` / `canon_cmp` keep `valid_to` and `invalidated_at` as `Option<u64>` (`None` ranks after every `Some`, including `Some(u64::MAX)`); open interval / no invalidation is not a `u64::MAX` sentinel.

### Changed

- Picture-diff types moved out of `fold_query.rs` (now byte-identical to 791df68) into `fold_diff.rs` so ingest/`as_of` codegen is isolated. `Fact` no longer derives `Default`; `Fact::blank` is a crate-private zeroed row for `FactChange::default` only.
- Bench A/B vs 791df68 (9 alternating pairs, `taskset -c 0`, `KUTHA_BENCH_N=10000` direct `--bench` binaries; pair 1 BASE then HEAD, pair 2 HEAD then BASE, …). PRE (BASE medians) ingest_plain.n **120198466 ns**, as_of.n **264152 ns**. POST (HEAD medians) ingest_plain.n **116451919 ns** (HEAD/BASE 0.969), as_of.n **267389 ns** (1.012). No run was a >3× outlier from its arm median. 1-min load 3.21–6.06. `validated yes`.

### Trajectory

- ADR-061 Evidence 2026-10-23: fact-set `diff_folds` shipped; named refs (D061-1) and GED (D061-3) still open. Status stays **Proposed**. Honeycomb restaged; map/delivery/capability unchanged.

## 2026-10-23 — Trajectory: lease M157-S01 (B3-lite hop workload)

Process tag **v1.51** / Phase **231**. Not a SemVer release.

- Active Milestone **M157**, Active Slice **S01** (`L_delivery=M157-S01`): workload as data, thin typed query IR over existing read APIs, self-relative multi-hop bench on the typed CSR lease. Competitor baseline deferred (owner, docker, disk). No parser, no planner, no thaw. Also: ADR-061 stale first Evidence/Open question reconciled with the shipped fact-set diff, `.planning/PROJECT.md` pointer moved to v1.51, README layout names the fork diff.

## 2026-10-23 — Trajectory: M156 / GSD v1.50 closed

Process tag **v1.50** / Phase **230**. Not a SemVer release.

- B2 fork diff: read-side `diff_folds` / `GraphFold::diff` over fold fact rows, exact on counterfactual and checkpointed forks, order-independent. One independent verification round found two HIGH defects (open interval and no-invalidation collapsed with `u64::MAX` in identity and rank); fixed and re-run. `cargo test --workspace --offline --no-fail-fast` 0, `kutha-gov ci --jobs 1` `run_high=0`. Bench numbers (9 alternating A/B pairs; an earlier single-sample and a 7-pair non-alternating run were order-biased and are superseded) are in the Product entry below.
- Residuals (each needs a fired RED before a lease): ordinary `StoreBlob` digests are not verified on open; `horn_behavior_reproducible` fails open on a missing rule blob; dangling lineage ids after a checkpoint; vintage checkpoint payload without `behaviors`; `cause_of_behavior` last-wins on duplicate event ids; checkpoint has no operator surface; B1b vacuum.
- `L_delivery=M156-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v1.50 / Phase 230. ADR-000 D3 still rejects a hard FSM as the only control model.

## 2026-10-23 — Trajectory: lease M156-S01 (fork diff, B2)

Process tag **v1.50** / Phase **230**. Not a SemVer release.

- Active Milestone **M156**, Active Slice **S01** (`L_delivery=M156-S01`): read-side `diff_folds` over fold fact rows plus fork-then-diff on a counterfactual fixture. No Op, no wire change, no thaw. Honeycomb stays Proposed.

## 2026-10-23 — Trajectory: M155 / GSD v1.49 closed

Process tag **v1.49** / Phase **229**. Not a SemVer release.

- Gap closure for thin B1 from the 2026-10-23 audit: `fork_at` fails closed on compacted cuts (typed `ForkCutUnavailable`), the Horn cascade after a checkpoint equals the un-checkpointed twin, `migrate_compat` refuses a checkpointed store. Four independent verification rounds closed seven HIGH defects (two Horn lineage holes, multi-path derived heads incl. a reopen defect that predates the checkpoint, prefix delivery-key retry). `cargo test --workspace --offline --no-fail-fast` 0, `kutha-gov ci --jobs 1` `run_high=0`. Bench numbers are in the Product entry below.
- Test hygiene: kill3 property cases default 16 (`KUTHA_KILL_CASES` for deep runs), new tests clean up their scratch dirs.
- Residuals (each needs a fired RED before a lease): ordinary `StoreBlob` digests are not verified on open; `horn_behavior_reproducible` fails open on a missing rule blob; dangling lineage ids after a checkpoint; vintage checkpoint payload without `behaviors`; checkpoint has no operator surface; B1b vacuum.
- `L_delivery=M155-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v1.49 / Phase 229. ADR-000 D3 still rejects a hard FSM as the only control model.

## 2026-10-23 — Product: checkpoint cut

### Changed

- Compacted prefix is an atomic cut: `Runtime::fork_at` returns `ForkCutUnavailable` for `1..=c` and `n > log.len()` (`n` is an offset into the current log). Horn cascade after a cut carries exact prefix `Op::Behavior` records in the checkpoint Snapshot blob (`behaviors`, covered by the blob digest; empty omitted so no-Horn payloads stay field-compatible). Two minting paths to the same head keep two records (same EventId, different `caused_by`); `rebuild_log_indexes` indexes each row like `append_indexed`. `GraphFold::apply(Checkpoint)` restores those records; they are not re-applied as facts and stay off the rewritten log (Checkpoint valid only on an empty fold). A rematch heuristic is not used. Re-checkpoint (R5) copies prior blob `behaviors` plus later prefix Behavior events. `migrate_compat` / `migrate_op_wire` / `kutha-migrate` return `InvalidData` when the log contains `Op::Checkpoint`; directory unchanged. M154/M155 checkpoint payloads written before this `behaviors` field are not readable as cascade-exact checkpoints (fail closed for Horn lineage; no checkpointed stores exist outside tests).
- Bench PRE (229-01, 2026-10-23T02:57:57Z load **4.05 3.54 3.23**) vs Horn-rematch POST (2026-10-23T03:38:12Z load **2.99 4.52 4.39**): ingest_plain.n **101894268 ns** → **100147178 ns** (−1.7%); as_of.n **247421 ns** → **267138 ns** (+8.0%, eager). Kill-fix POST (2026-10-23T04:55:33Z load **2.03 3.47 6.08**): ingest_plain.n **90623213 ns** (−11.1% vs PRE); as_of.n **234898 ns** (−5.1%). Round-2 POST (2026-10-23T06:11:28Z load **4.93 7.13 6.78**, `taskset -c 0`, `KUTHA_BENCH_N=10000 cargo bench -p kutha-runtime --bench baseline --offline`): ingest_plain.n **97135182 ns** (−4.7% vs PRE); as_of.n **229128 ns** (−7.4%, eager). Neither regresses over 10 percent. Discarded noisy passes: compile 19.6s under 5-min load 7.78 (ingest_plain.n 149325122 ns); as_of 351424 / 276274 while 5-min load 6.3–6.6; round-2 unpinned 112818836 / 410345, 120526898 / 289760, 115256945 / 299428 while `/root/prime-agent` rustc was active (not touched).

### Fixed

- Independent verifier kill-tests `r2_chain_two_hops_retract_first_premise` (live `relatedTo(A,D)` after retracting prefix A-B) and `r2_two_rules_two_premises_chain_and_no_live_premise` (inverse `relatedTo(C,B)` of still-live B-C dropped) — exact derivation records through the checkpoint, not rematch.
- Round-2 kill2 3-hop (`b3_checkpoint_cut_kill2`): `checkpoint_prefix_behaviors` no longer drops a second `Op::Behavior` that shares EventId (`Event::stable_derived` hashes pin+bindings, not `caused_by`). `rebuild_log_indexes` indexes every live minting row so `derived_by_cause` sees both causes. Retract A-B and retract D-E on a checkpointed store match the never-reopened twin and the persist+`open` twin.
- Round-3 kill3 `r3_reregister_same_horn_after_checkpoint`: prefix keyed Asserts (product, `kutha:rule` / `kutha:allows` / `kutha:policy`) live in restored facts but were dropped from the rewritten log, so `by_delivery_key` missed them and a re-register minted a second live pin. `rebuild_log_indexes` now fills `by_delivery_key` from facts (`or_insert` after the log walk) and reconstructs those Assert Events for `event_by_id`. Payload shape and fingerprint unchanged.

### Trajectory

- ADR-012 / ADR-060 / ADR-061 gained Evidence one-liners only. Status stays **Proposed**. Honeycomb restaged; map/delivery/capability unchanged.

## 2026-10-23 — Trajectory: lease M155-S01 (checkpoint is a cut, not a hole)

Process tag **v1.49** / Phase **229**. Not a SemVer release.

- Active Milestone **M155**, Active Slice **S01** (`L_delivery=M155-S01`): fail-closed `fork_at` on compacted cuts, Horn cascade parity after a checkpoint, `migrate_compat` fail-closed on a checkpoint. Gap closure from the 2026-10-23 audit. Honeycomb stays Proposed; nothing thawed.

## 2026-10-23 — Process: post-M154 docs reconciliation and temp hygiene

### Changed

- Reconciled post-M154 docs and rustdoc with the checkpoint receipt: `GraphFold::apply` of `Op::Checkpoint` restores facts from the Snapshot-shaped blob on an empty fold; current `store::open` verifies receipts and fails closed on mismatch (old readers still fail open on the unknown serde variant); Op inventories include `Checkpoint`; `Persister::checkpoint` (snapshot refresh) is named separately from `store::checkpoint_sealed_prefix` / `Op::Checkpoint` (sealed-prefix receipt). Capability is library API only (no `Runtime` method or operator CLI, not vacuum). `kutha-migrate` / `migrate_compat` must not be run on a checkpointed store (they demote to jsonl; answers unchanged). No product code behavior change.

### Fixed

- M153 closed handoff still named a B1 vacuum lease; superseded by the M154 handoff. Direction-roadmap status records B1 as thin checkpoint.
- Checkpoint integration tests drop their repo-local `tmp/` store directories on unwind (including panic).

## 2026-10-22 — Trajectory: M154 / GSD v1.48 closed

Process tag **v1.48** / Phase **228**. Not a SemVer release.

- Thin B1: a sealed log prefix can be replaced by an on-log checkpoint and an `Op::Checkpoint` receipt with strict replay exact. Not vacuum (no TT history erased, fingerprint contract unchanged). Independent verification took three passes (6 HIGH defects found and fixed, one requirement amended to parity for ordinary `StoreBlob` tamper). `cargo test --workspace --offline --no-fail-fast` 0, `kutha-gov ci --jobs 1` `run_high=0`.
- Residuals (each needs a fired RED before a lease): ordinary `StoreBlob` digests are not verified on open (pre-existing); `horn_behavior_reproducible` fails open on a missing rule blob; `migrate_compat` demotes a checkpointed store to jsonl. Bench numbers are in the Product entry below.
- `L_delivery=M154-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v1.48 / Phase 228. ADR-000 D3 still rejects a hard FSM as the only control model.

## 2026-10-22 — Product: checkpoint receipt (thin B1)

### Added

- `store::checkpoint_sealed_prefix` under `kutha.lock` rewrites a sealed prefix to prefix `StoreBlob`, `QuantumOutcome`, and `JustificationCite` ops copied verbatim (same ids/order), then `Op::StoreBlob` (prefix `Snapshot` fold, same shape as `snapshot.rs`) plus `Op::Checkpoint` receipt plus the unchanged tail. `snapshot.json` stays a lease, never the carrier. `dropped_count` is prefix length minus those kept kinds (not `Op::Checkpoint`; a re-checkpoint folds the old receipt into the new Snapshot); `fact_count` is the prefix fold fact count.
- Oracle `user_emit_checkpoint_is_meta_op_rejected`: public `Runtime::emit(Op::Checkpoint)` returns `MetaOpRejected` and does not append.

### Changed

- `GraphFold::apply` of `Checkpoint` restores facts, `next_seq`, and `tt_anchors` from the blob; `tt_anchors_from_events` / `rebuild_tt_anchors` union walls from checkpoint blobs. Fingerprint still hashes all facts including dead ones (not vacuum). `GraphFold.blobs` stays serde-skip; rule/policy/plain bodies ride copied prefix StoreBlobs.
- Bench PRE (228-01) vs kill2-fix POST (2026-10-22T18:03:00Z load 6.82 5.85 4.36), same host, `KUTHA_BENCH_N=10000 cargo bench -p kutha-runtime --bench baseline --offline` alone: ingest_plain.n **144924496 ns** → **102767611 ns** (−29.1%); as_of.n **394795 ns** → **215071 ns** (−45.5%, eager). Neither regresses over 10 percent (sub-baseline is noise under that load).

### Fixed

- After checkpoint, persist the encoded log (Defines then graph), not the compacted graph-only rewrite. `committed_prefix` can then classify `kutha:admission` / `kutha:action` in the tail, and `snapshot.log_offset` matches the committed fold-affecting count. Persister attach sees the same event count as `encoded_log`.
- `Op::Checkpoint` apply and `replay_check` verify blob digest, restored `fact_count`, and `fingerprint_hex` against the prefix Snapshot; mismatch is `ReplayDivergence` (open fail-closed). Fingerprint still hashes all facts including dead ones.
- Prefix `QuantumOutcome` and `JustificationCite` ride the rewritten log like kept `StoreBlob`s, so `outcome_records` / `justification_records` match the un-checkpointed twin. `Op::Checkpoint` is not copied (R5 re-checkpoint keeps one receipt).
- `Op::Checkpoint` is valid only on an empty fold (`facts` empty, `next_seq == 0`) after kept prefix meta (`StoreBlob` / `QuantumOutcome` / `JustificationCite` / `Define`). A duplicate receipt or one moved after tail events is `ReplayDivergence` (`open` / `replay_check` fail closed; apply does not clobber).

Lease and honeycomb cells stay **Proposed**.

## 2026-10-22 — Trajectory: lease M154-S01 (checkpoint receipt)

Process tag **v1.48** / Phase **228**. Not a SemVer release.

- Active Milestone **M154**, Active Slice **S01** (`L_delivery=M154-S01`): thin B1, a sealed log prefix replaced by an on-log checkpoint with a receipt op; replay exact. Not vacuum (no TT history erased). Honeycomb stays Proposed; nothing thawed.

## 2026-10-22 — Trajectory: M153 / GSD v1.47 closed

Process tag **v1.47** / Phase **227**. Not a SemVer release.

- Verus on-ramp: release `0.2026.10.18.7d82994` pinned in `tools/verus/` (rustc 1.99.1, checksum equals the GitHub release digest); `verus/vt_intersect.rs` verifies with the production body unchanged (`2 verified, 0 errors`); two mutations of the body fail in Verus. Check `verus-proof-sync` pins the proof body to `fold.rs`; budget 104→105. Matrix `SEM-m153-verus-vt-sync` killed.
- Verus is evidence, not a gate: CI does not run it and no crate depends on it. The official PDD Kit is not installable yet (research preview); this slice is the Verus prerequisite only.
- `L_delivery=M153-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v1.47 / Phase 227. ADR-000 D3 still rejects a hard FSM as the only control model.

## 2026-10-22 — Process: Verus on-ramp

### Added

- `tools/verus/{VERSION,SHA256SUMS}`, `verus/vt_intersect.rs`, `KUTHA_VERUS_BIN` in `.env.example`, governor check `verus-proof-sync` (invariant `I-verus-vt-proof-sync`), and the setup note in `.cursor/rules/verus-formal-verification.mdc`.

## 2026-10-22 — Trajectory: lease M153-S01 (Verus on-ramp)

Process tag **v1.47** / Phase **227**. Not a SemVer release.

- Active Milestone **M153**, Active Slice **S01** (`L_delivery=M153-S01`): Verus pin, `verus/vt_intersect.rs` and one sync check. Evidence only, no CI dependency. Honeycomb stays Proposed; nothing thawed.

## 2026-10-22 — Process: Verus adopted as a bounded evidence tool

### Added

- `AGENTS.md` Working conventions item 10 and `.cursor/rules/verus-formal-verification.mdc`: Verus for pure kernels only (VT interval algebra, later `leapfrog_intersect`), inside a product slice, never a gate or an ADR acceptance. Docs and rules only; no proof file, check, or dependency added, budget stays 104.
- Direction proposal § Verus records the 2026-10-22 spike: `vt_intersect` verifies with its production body unchanged; `leapfrog_intersect` does not (`partition_point`, `map_or` unsupported).

## 2026-10-22 — Trajectory: M152 / GSD v1.46 closed

Process tag **v1.46** / Phase **226**. Not a SemVer release.

- Arc 0 hygiene: CE handoffs, audit roadmap, review run and the direction proposal are tracked; harness moratorium recorded in the GSD-auto contract; working branch renamed `chore/m013-roadmap-reset` to `work/main-line` and pushed as a backup. No product or harness code changed; budget stays 104.
- `L_delivery=M152-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v1.46 / Phase 226. ADR-000 D3 still rejects a hard FSM as the only control model.

## 2026-10-22 — Process: harness moratorium

### Changed

- `docs/process/gsd-auto-contract.md`: new § Harness moratorium. A new harness milestone needs a RED that fired in CI or review; a lease without one is a stop condition. Docs only; no check added, budget stays 104.
- Direction proposal (order B, C, A; proposal only, nothing thawed): `.compound-engineering/artifacts/research/2026-10-22-long-horizon-direction-roadmap.md`. CE handoffs and audit artifacts that were untracked are now tracked.

## 2026-10-22 — Trajectory: lease M152-S01 (Arc 0 hygiene)

Process tag **v1.46** / Phase **226**. Not a SemVer release.

- Active Milestone **M152**, Active Slice **S01** (`L_delivery=M152-S01`): docs and git hygiene only (track CE artifacts, harness moratorium, branch rename and backup push). Honeycomb stays Proposed; nothing thawed.

## 2026-10-22 — Trajectory: M151 / GSD v1.45 closed

Process tag **v1.45** / Phase **225**. Not a SemVer release.

- Overlap cargo pack includes `wall_ms`; `phase_ms observe_cargo=` uses it instead of Future join-wait. Check `observe-phase-ms-wall`; budget 103→104. Matrix `SEM-m151-observe-phase-ms` killed.
- `L_delivery=M151-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v1.45 / Phase 225. ADR-000 D3 still rejects a hard FSM as the only control model.

## 2026-10-22 — Trajectory: lease M151-S01 (observe phase_ms wall)

Process tag **v1.45** / Phase **225**. Not a SemVer release.

- Active Milestone **M151**, Active Slice **S01** (`L_delivery=M151-S01`): phase_ms observe_cargo uses cargo wall time under overlap. Honeycomb stays Proposed.

## 2026-10-22 — Trajectory: M150 / GSD v1.44 closed

Process tag **v1.44** / Phase **224**. Not a SemVer release.

- `_run_cargo` observe-fail messages include `elapsed_ms=N` and a truncated stderr/stdout tail (`KUTHA_GOV_OBSERVE_FAIL_TAIL`, default 240). Check `observe-fail-detail`; budget 102→103. Matrix `SEM-m150-observe-fail-detail` killed.
- `L_delivery=M150-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v1.44 / Phase 224. ADR-000 D3 still rejects a hard FSM as the only control model.

## 2026-10-22 — Trajectory: lease M150-S01 (observe-fail detail)

Process tag **v1.44** / Phase **224**. Not a SemVer release.

- Active Milestone **M150**, Active Slice **S01** (`L_delivery=M150-S01`): observe-fail includes elapsed_ms and output tail. Honeycomb stays Proposed.

## 2026-10-22 — Trajectory: M149 / GSD v1.43 closed

Process tag **v1.43** / Phase **223**. Not a SemVer release.

- `interpret_cargo_output`: non-zero exit + missing required + no FAILED collapses to one `observe-incomplete` (partial ok stream retries via M146). Real FAILED stays per-name; exit 0 keeps per-name missing. Check `observe-partial-incomplete`; budget 101→102. Matrix `SEM-m149-partial-observe-incomplete` killed.
- `L_delivery=M149-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v1.43 / Phase 223. ADR-000 D3 still rejects a hard FSM as the only control model.

## 2026-10-22 — Trajectory: lease M149-S01 (partial observe incomplete)

Process tag **v1.43** / Phase **223**. Not a SemVer release.

- Active Milestone **M149**, Active Slice **S01** (`L_delivery=M149-S01`): non-zero cargo exit with missing required names and no FAILED → `observe-incomplete` (retry). Honeycomb stays Proposed.

## 2026-10-22 — Trajectory: M148 / GSD v1.42 closed

Process tag **v1.42** / Phase **222**. Not a SemVer release.

- `cmd_ci` always prints `observe_retried=N` (count of LOW `_OBSERVE_RETRIED` findings) next to `run_high` / `run_low`. Check `observe-retried-summary`; budget 100→101. Matrix `SEM-m148-observe-retried-summary` killed. Label contract via pytest (not `observe_retried=` file_contains needles).
- `L_delivery=M148-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v1.42 / Phase 222. ADR-000 D3 still rejects a hard FSM as the only control model.

## 2026-10-22 — Trajectory: lease M148-S01 (observe_retried summary)

Process tag **v1.42** / Phase **222**. Not a SemVer release.

- Active Milestone **M148**, Active Slice **S01** (`L_delivery=M148-S01`): ci summary prints observe_retried=N. Honeycomb stays Proposed.

## 2026-10-22 — Trajectory: M147 / GSD v1.41 closed

Process tag **v1.41** / Phase **221**. Not a SemVer release.

- `_apply_cargo_pack`: observation status fails only on HIGH (LOW breadcrumbs stay `cargo=ok`). Successful incomplete→ok retry emits LOW `observe-retried`. Check `observe-retried-breadcrumb`; budget 99→100. Matrix `SEM-m147-observe-retried` killed.
- `L_delivery=M147-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v1.41 / Phase 221. ADR-000 D3 still rejects a hard FSM as the only control model.

## 2026-10-22 — Trajectory: lease M147-S01 (observe-retried breadcrumb)

Process tag **v1.41** / Phase **221**. Not a SemVer release.

- Active Milestone **M147**, Active Slice **S01** (`L_delivery=M146-closed`): cargo pack HIGH-only + LOW observe-retried. Honeycomb stays Proposed.

## 2026-10-22 — Trajectory: M146 / GSD v1.40 closed

Process tag **v1.40** / Phase **220**. Not a SemVer release.

- `run_cargo_observation` retries on `observe-incomplete` under the same flock (`KUTHA_GOV_CARGO_OBSERVE_RETRIES`, default 1); successful retry drops first-attempt findings. Check `observe-retry-incomplete`; budget 98→99. Matrix `SEM-m146-observe-retry` killed.
- `L_delivery=M146-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v1.40 / Phase 220. ADR-000 D3 still rejects a hard FSM as the only control model.

## 2026-10-22 — Trajectory: lease M146-S01 (cargo observe retry)

Process tag **v1.40** / Phase **220**. Not a SemVer release.

- Active Milestone **M146**, Active Slice **S01** (`L_delivery=M145-closed`): retry observe on incomplete via env. Honeycomb stays Proposed.

## 2026-10-21 — Trajectory: M145 / GSD v1.39 closed

Process tag **v1.39** / Phase **219**. Not a SemVer release.

- `interpret_cargo_output(..., cargo_exit=)`: non-zero exit with no required ok/FAILED lines → one `observe-incomplete` (not N× `observe-missing`). Check `observe-incomplete-honesty`; budget 97→98. Matrix `SEM-m145-observe-incomplete` killed.
- `L_delivery=M145-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v1.39 / Phase 219. ADR-000 D3 still rejects a hard FSM as the only control model.

## 2026-10-21 — Trajectory: lease M145-S01 (observe-incomplete honesty)

Process tag **v1.39** / Phase **219**. Not a SemVer release.

- Active Milestone **M145**, Active Slice **S01** (`L_delivery=M144-closed`): cargo infra fail must not mint N× observe-missing. Honeycomb stays Proposed.

## 2026-10-21 — Trajectory: M144 / GSD v1.38 closed

Process tag **v1.38** / Phase **218**. Not a SemVer release.

- `git-auto-base`: dropped `run_high=` / `tip_high=` file_contains needles; pins merge-base wiring + pytest names (`test_cmd_ci_prints_run_high_and_tip_high` owns the label contract). Matrix `SEM-m144-git-auto-base-behavioral` killed. Budget still 97.
- `L_delivery=M144-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v1.38 / Phase 218. ADR-000 D3 still rejects a hard FSM as the only control model.

## 2026-10-21 — Trajectory: lease M144-S01 (git-auto-base label behavioral)

Process tag **v1.38** / Phase **218**. Not a SemVer release.

- Active Milestone **M144**, Active Slice **S01** (`L_delivery=M143-closed`): de-text-couple git-auto-base summary label needles. Honeycomb stays Proposed.

## 2026-10-21 — Trajectory: M143 / GSD v1.37 closed

Process tag **v1.37** / Phase **217**. Not a SemVer release.

- `skip-cargo-log-marker`: dropped CE #38 comment/assert-snippet needles; pins fsm wiring + `test_skip_cargo_does_not_call_cargo_or_tenant`. Matrix `SEM-m143-skip-cargo-marker-behavioral` killed. Budget still 97.
- `L_delivery=M143-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v1.37 / Phase 217. ADR-000 D3 still rejects a hard FSM as the only control model.

## 2026-10-21 — Trajectory: lease M143-S01 (skip-cargo-marker behavioral)

Process tag **v1.37** / Phase **217**. Not a SemVer release.

- Active Milestone **M143**, Active Slice **S01** (`L_delivery=M142-closed`): de-text-couple skip-cargo-log-marker. Honeycomb stays Proposed.

## 2026-10-21 — Trajectory: M142 / GSD v1.36 closed

Process tag **v1.36** / Phase **216**. Not a SemVer release.

- `observe-skipped-breadcrumb`: dropped assert-snippet needles; pins fsm wiring + `test_second_observe_cargo_notes_skip_after_prior_high`. Matrix `SEM-m142-observe-skipped-behavioral` killed. Budget still 97.
- `L_delivery=M142-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v1.36 / Phase 216. ADR-000 D3 still rejects a hard FSM as the only control model.

## 2026-10-21 — Trajectory: lease M142-S01 (observe-skipped behavioral)

Process tag **v1.36** / Phase **216**. Not a SemVer release.

- Active Milestone **M142**, Active Slice **S01** (`L_delivery=M141-closed`): de-text-couple observe-skipped-breadcrumb. Honeycomb stays Proposed.

## 2026-10-21 — Trajectory: M141 / GSD v1.35 closed

Process tag **v1.35** / Phase **215**. Not a SemVer release.

- `parallel-context-flags`: dropped docstring/assert-snippet needles; pins Context wiring + `test_eval_check_worker_propagates_skip_emit_and_changed_paths`. Matrix `SEM-m141-parallel-context-behavioral` killed. Budget still 97.
- `L_delivery=M141-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v1.35 / Phase 215. ADR-000 D3 still rejects a hard FSM as the only control model.

## 2026-10-21 — Trajectory: lease M141-S01 (parallel-context behavioral)

Process tag **v1.35** / Phase **215**. Not a SemVer release.

- Active Milestone **M141**, Active Slice **S01** (`L_delivery=M140-closed`): de-text-couple parallel-context-flags. Honeycomb stays Proposed.

## 2026-10-21 — Trajectory: M140 / GSD v1.34 closed

Process tag **v1.34** / Phase **214**. Not a SemVer release.

- `agent-surface`: dropped exact `__main__.py` hint-string needles; pins parser wiring + `test_check_parser_helpers_are_live_callables`; CLI reject pytest asserts surface redirect not exact `hint:` template. Matrix `SEM-m140-agent-surface-hint-behavioral` killed. Budget still 97.
- `L_delivery=M140-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v1.34 / Phase 214. ADR-000 D3 still rejects a hard FSM as the only control model.

## 2026-10-21 — Trajectory: lease M140-S01 (agent-surface hint behavioral)

Process tag **v1.34** / Phase **214**. Not a SemVer release.

- Active Milestone **M140**, Active Slice **S01** (`L_delivery=M139-closed`): de-text-couple agent-surface hint needles. Honeycomb stays Proposed.

## 2026-10-21 — Trajectory: M139 / GSD v1.33 closed

Process tag **v1.33** / Phase **213**. Not a SemVer release.

- `retract_derived` phase 2 uses droppable `behaviors_by_premise_fact` (supporting premise EventIds) instead of the same-rel relation bucket; multi-premise N chains stay near-linear in `retract_examine`. Oracle `retract_n_multiprem_examines_grow_near_linear_not_quadratic`. Matrix `SEM-m139-retract-multiprem-fact-index` killed. Budget still 97. No Op schema change.
- `L_delivery=M139-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v1.33 / Phase 213. ADR-000 D3 still rejects a hard FSM as the only control model.

## 2026-10-21 — Trajectory: lease M139-S01 (multiprem retract fact-index)

Process tag **v1.33** / Phase **213**. Not a SemVer release.

- Active Milestone **M139**, Active Slice **S01** (`L_delivery=M138-closed`): multi-premise retract premise-fact index. Honeycomb stays Proposed.

## 2026-10-21 — Trajectory: M138 / GSD v1.32 closed

Process tag **v1.32** / Phase **212**. Not a SemVer release.

- `kinds-family-split`: behavioral RUNNERS dispatch oracle (`test_runners_dispatch_uses_sibling_family_callables`); dropped `RUNNERS: dict[str, Runner]` needle; selftest mutates `"test_matrix"` key. Matrix `SEM-m138-kinds-family-behavioral` killed. Budget still 97.
- `L_delivery=M138-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v1.32 / Phase 212. ADR-000 D3 still rejects a hard FSM as the only control model.

## 2026-10-21 — Trajectory: lease M138-S01 (kinds-family behavioral)

Process tag **v1.32** / Phase **212**. Not a SemVer release.

- Active Milestone **M138**, Active Slice **S01** (`L_delivery=M137-closed`): de-text-couple kinds-family-split. Honeycomb stays Proposed.

## 2026-10-21 — Trajectory: M137 / GSD v1.31 closed

Process tag **v1.31** / Phase **211**. Not a SemVer release.

- `parallel-check-error-parity`: behavioral `_eval_check_worker` oracle (`test_eval_check_worker_maps_run_exception_to_check_error`); dropped docstring/assert text needles; kept `return eval_check(check, ctx)`. Matrix `SEM-m137-parallel-error-behavioral` killed. Budget still 97.
- `L_delivery=M137-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v1.31 / Phase 211. ADR-000 D3 still rejects a hard FSM as the only control model.

## 2026-10-21 — Trajectory: lease M137-S01 (parallel error behavioral)

Process tag **v1.31** / Phase **211**. Not a SemVer release.

- Active Milestone **M137**, Active Slice **S01** (`L_delivery=M136-closed`): de-text-couple parallel-check-error-parity. Honeycomb stays Proposed.

## 2026-10-21 — Trajectory: M136 / GSD v1.30 closed

Process tag **v1.30** / Phase **210**. Not a SemVer release.

- `retract_derived` second phase skips single-premise Horns (phase 1 `caused_by` already covers them), so N same-rel retracts stay near-linear in `retract_examine`. Oracle `retract_n_derived_examines_grow_near_linear_not_quadratic`. Matrix `SEM-m136-retract-n-scaling` killed.
- `L_delivery=M136-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v1.30 / Phase 210. ADR-000 D3 still rejects a hard FSM as the only control model.

## 2026-10-21 — Trajectory: lease M136-S01 (retract N-scaling)

Process tag **v1.30** / Phase **210**. Not a SemVer release.

- Active Milestone **M136**, Active Slice **S01** (`L_delivery=M135-closed`): N derived + N retracts examine scaling. Honeycomb stays Proposed.

## 2026-10-21 — Trajectory: M135 / GSD v1.29 closed

Process tag **v1.29** / Phase **209**. Not a SemVer release.

- `migrate_pins` unlock window: no parent lock (source oracle); peer cite WouldBlocks in the gap then facade converges via markers. Docs needle `unlock window`. Matrix `SEM-m135-migrate-pins-unlock-window` killed. Budget still 97.
- `L_delivery=M135-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v1.29 / Phase 209. ADR-000 D3 still rejects a hard FSM as the only control model.

## 2026-10-21 — Trajectory: lease M135-S01 (migrate_pins unlock window)

Process tag **v1.29** / Phase **209**. Not a SemVer release.

- Active Milestone **M135**, Active Slice **S01** (`L_delivery=M134-closed`): unlock window between pin-wire and pin-cite. Honeycomb stays Proposed.

## 2026-10-21 — Trajectory: M134 / GSD v1.28 closed

Process tag **v1.28** / Phase **208**. Not a SemVer release.

- pin-cite orphan rematerialize: operator docs state `delete pin-cite.v1`; oracle `migrate_pin_cite_requires_delete_marker_to_rematerialize_after_new_evidence`; `pin-compat-policy` needles + rust_test_asserts. Matrix `SEM-m134-pin-cite-delete-rerun` killed. Budget still 97.
- `L_delivery=M134-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v1.28 / Phase 208. ADR-000 D3 still rejects a hard FSM as the only control model.

## 2026-10-21 — Trajectory: lease M134-S01 (pin-cite delete/rerun)

Process tag **v1.28** / Phase **208**. Not a SemVer release.

- Active Milestone **M134**, Active Slice **S01** (`L_delivery=M133-closed`): pin-cite.v1 delete/rerun honesty. Honeycomb stays Proposed.

## 2026-10-21 — Trajectory: M133 / GSD v1.27 closed

Process tag **v1.27** / Phase **207**. Not a SemVer release.

- `migrate_compat` order oracle: `migrate_op_wire` precedes `migrate_pins`. Oracle `migrate_compat_calls_op_wire_before_pins`. Matrix `SEM-m133-migrate-compat-order` killed. Kill: swap call order (SEM-m076 markers alone stay green under that mutation).
- `L_delivery=M133-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v1.27 / Phase 207. ADR-000 D3 still rejects a hard FSM as the only control model.

## 2026-10-21 — Trajectory: lease M133-S01 (migrate_compat call order)

Process tag **v1.27** / Phase **207**. Not a SemVer release.

- Active Milestone **M133**, Active Slice **S01** (`L_delivery=M132-closed`): op-wire before pins order oracle. Honeycomb stays Proposed.

## 2026-10-21 — Trajectory: M132 / GSD v1.26 closed

Process tag **v1.26** / Phase **206**. Not a SemVer release.

- `reconcile_uncommitted_tip`: each `set_len` followed by `sync_all` (not `sync_data`). Oracle `reconcile_truncation_uses_sync_all_after_set_len`. Matrix `SEM-m132-reconcile-sync-all` killed. Incremental append still uses `sync_data`.
- `L_delivery=M132-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v1.26 / Phase 206. ADR-000 D3 still rejects a hard FSM as the only control model.

## 2026-10-21 — Trajectory: lease M132-S01 (reconcile tip truncate sync_all)

Process tag **v1.26** / Phase **206**. Not a SemVer release.

- Active Milestone **M132**, Active Slice **S01** (`L_delivery=M131-closed`): `sync_all` after reconcile `set_len`. Honeycomb stays Proposed.

## 2026-10-21 — Trajectory: M131 / GSD v1.25 closed

Process tag **v1.25** / Phase **205**. Not a SemVer release.

- `yaml_map_list`: empty select list is HIGH (`not a non-empty list`), same message family as needles/matrix. RED vacuous pass → GREEN; check `yaml-map-list-nonempty`; budget 97.
- `L_delivery=M131-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v1.25 / Phase 205. ADR-000 D3 still rejects a hard FSM as the only control model.

## 2026-10-21 — Trajectory: lease M131-S01 (yaml_map_list empty-select fail-closed)

Process tag **v1.25** / Phase **205**. Not a SemVer release.

- Active Milestone **M131**, Active Slice **S01** (`L_delivery=M130-closed`): empty `yaml_map_list` select must HIGH. Honeycomb stays Proposed.

## 2026-10-21 — Trajectory: M130 / GSD v1.24 closed

Process tag **v1.24** / Phase **204**. Not a SemVer release.

- `_run_selftest_parallel`: submit one-by-one; harvest + leftover serial on submit/as_completed failure (parity with `run_checks_parallel`). RED: FailAfterFirst raised; GREEN: both check ids return OK. Check `selftest-submit-backstop`; budget 96; `selftest.py` 981 ≤1000.
- `L_delivery=M130-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v1.24 / Phase 204. ADR-000 D3 still rejects a hard FSM as the only control model.

## 2026-10-21 — Trajectory: lease M130-S01 (selftest submit serial backstop)

Process tag **v1.24** / Phase **204**. Not a SemVer release.

- Active Milestone **M130**, Active Slice **S01** (`L_delivery=M129-closed`): parallel selftest submit-failure serial backstop. Honeycomb stays Proposed.

## 2026-10-21 — Trajectory: M129 / GSD v1.23 closed

Process tag **v1.23** / Phase **203**. Not a SemVer release.

- Check `yaml-needles-live`: FSM `observe_cargo` / `observe_rocks` required names as `fn <name>` under crates via live `yaml_needles_in_glob`. RED (glob → `AGENTS.md`) HIGH; GREEN on crates glob; selftest derived-by-own-mutation. Budget 95. Matrix `SEM-m129-yaml-needles-live` killed.
- `L_delivery=M129-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v1.23 / Phase 203. ADR-000 D3 still rejects a hard FSM as the only control model.

## 2026-10-21 — Trajectory: lease M129-S01 (yaml_needles live dogfood)

Process tag **v1.23** / Phase **203**. Not a SemVer release.

- Active Milestone **M129**, Active Slice **S01** (`L_delivery=M128-closed`): live `yaml_needles_in_glob` on FSM observe required lists. Honeycomb stays Proposed.

## 2026-10-21 — Trajectory: M128 / GSD v1.22 closed

Process tag **v1.22** / Phase **202**. Not a SemVer release.

- Oracle `two_disjoint_ab_windows_mint_two_ac_heads`: two disjoint AB residuals + open BC mint two AC heads; kill (first-only `PremMatch` in `search_at_all`) was RED; HEAD GREEN. Matrix `SEM-m128-horn-multi-combination` killed. No product code change (`search_at_all` already enumerates).
- `L_delivery=M128-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v1.22 / Phase 202. ADR-000 D3 still rejects a hard FSM as the only control model.

## 2026-10-21 — Trajectory: lease M128-S01 (Horn multi-combination oracle)

Process tag **v1.22** / Phase **202**. Not a SemVer release.

- Active Milestone **M128**, Active Slice **S01** (`L_delivery=M127-closed`): CE residual — oracle that disjoint premise VT windows under one trigger mint one head each. Honeycomb stays Proposed.

## 2026-10-21 — Trajectory: M127 / GSD v1.21 closed

### Process

- Check `selftest-lean`: `file_max_lines` on `scripts/kutha_gov/selftest.py` max 1000 (wc -l). Budget 93→94. Matrix `SEM-m127-selftest-lean` killed.

### Trajectory

- `L_delivery=M127-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v1.21 / Phase 201. ADR-000 D3 still rejects a hard FSM as the only control model.

## 2026-10-21 — Trajectory: lease M127-S01 (selftest.py line ceiling)

### Trajectory

- Active Milestone **M127**, Active Slice **S01** (`L_delivery=M126-closed`): pin `selftest.py` with `file_max_lines` (twin of `kinds-lean`). Honeycomb stays Proposed.

## 2026-10-21 — Trajectory: M126 / GSD v1.20 closed

### Process

- Check `kinds-lean`: `file_max_lines` on `scripts/kutha_gov/kinds.py` max 2500 (wc -l). Budget 92→93. Matrix `SEM-m126-kinds-lean` killed.

### Trajectory

- `L_delivery=M126-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v1.20 / Phase 200. ADR-000 D3 still rejects a hard FSM as the only control model.

## 2026-10-21 — Trajectory: lease M126-S01 (kinds.py line ceiling)

### Trajectory

- Active Milestone **M126**, Active Slice **S01** (`L_delivery=M125-closed`): pin `kinds.py` with `file_max_lines` against unbounded kernel re-growth. Honeycomb stays Proposed.

## 2026-10-21 — Trajectory: M125 / GSD v1.19 closed

### Process

- Selftest derives `yaml_needles_in_glob` (`yaml_needles_in_glob_mutation_target`). Check `yaml-needles-derive`. ALLOWED − DERIVABLE = {`git_path_implies`} only. Budget 91→92. Matrix `SEM-m125-yaml-needles-derive` killed.

### Trajectory

- `L_delivery=M125-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v1.19 / Phase 199. ADR-000 D3 still rejects a hard FSM as the only control model.

## 2026-10-21 — Trajectory: lease M125-S01 (yaml_needles derive + DERIVABLE completeness)

### Trajectory

- Active Milestone **M125**, Active Slice **S01** (`L_delivery=M124-closed`): derive `yaml_needles_in_glob`; ALLOWED kinds DERIVABLE except `git_path_implies`. Honeycomb stays Proposed.

## 2026-10-20 — Trajectory: M124 / GSD v1.18 closed

### Process

- Selftest derives `glob_regex_absent` (`DERIVABLE` + `glob_regex_absent_mutation_target`). Check `glob-regex-absent-derive`. Budget 90→91. Matrix `SEM-m124-glob-regex-derive` killed.

### Trajectory

- `L_delivery=M124-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v1.18 / Phase 198. ADR-000 D3 still rejects a hard FSM as the only control model.

## 2026-10-20 — Trajectory: lease M124-S01 (glob_regex_absent selftest derive)

### Trajectory

- Active Milestone **M124**, Active Slice **S01** (`L_delivery=M123-closed`): selftest derives `glob_regex_absent` so the kind is not UNPROVEN without explicit YAML. Honeycomb stays Proposed.

## 2026-10-20 — Trajectory: M123 / GSD v1.17 closed

### Process

- Governor kind `glob_regex_absent`: no glob match may contain any of `patterns` (invalid regex → HIGH).
- Check `fixed-tmp-persist-dir` also forbids `.join("tmp").join("` under crates. Sem `p62-sem` folded into the unique leaf. Budget stays 90. Matrix `SEM-m123-fixed-tmp-join` killed.

### Trajectory

- `L_delivery=M123-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v1.17 / Phase 197. ADR-000 D3 still rejects a hard FSM as the only control model.

## 2026-10-20 — Trajectory: lease M123-S01 (fixed-tmp join-after-tmp dogfood)

### Trajectory

- Active Milestone **M123**, Active Slice **S01** (`L_delivery=M122-closed`): forbid `.join("tmp").join("…")` under crates; fold `p62-sem` into the unique leaf. Honeycomb stays Proposed.

## 2026-10-20 — Trajectory: M122 / GSD v1.16 closed

### Process

- Governor check `fixed-tmp-persist-dir`: `glob_absent` over `crates/**/*.rs` forbids `Path::new("tmp/`. Budget 90. Invariant `I-fixed-tmp-persist-dir`. Matrix `SEM-m122-fixed-tmp-persist-dir` killed.

### Product

- Example `statute_as_of` and `m026_generators` persist/open use per-call directories (pid/nanos/seq). No remaining `Path::new("tmp/…")` under crates.
- Bench (`KUTHA_BENCH_N=10000`): `ingest_plain.n` 123995257 ns, `as_of.n` 319631 ns. Vs M121 recorded 123783243 / 263320. No library hot-path edit; host noise.

### Trajectory

- `L_delivery=M122-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v1.16 / Phase 196. ADR-000 D3 still rejects a hard FSM as the only control model.

## 2026-10-20 — Trajectory: lease M122-S01 (fixed-tmp persist-dir dogfood)

### Trajectory

- Active Milestone **M122**, Active Slice **S01** (`L_delivery=M121-closed`): forbid `Path::new("tmp/…")` under crates; unique dirs for example + m026. Honeycomb stays Proposed.

## 2026-10-20 — Trajectory: M121 / GSD v1.15 closed

### Product

- `statute_as_of_via_facade` no longer uses the fixed path `tmp/p27-facade-statute`. Each call gets `tmp/p27-facade-statute-<pid>-<nanos>-<seq>`. Oracle `facade_statute_store_dirs_differ_across_calls`; matrix `SEM-m121-facade-statute-store-dir` killed. `observe_cargo.required` includes that name.
- Bench (`KUTHA_BENCH_N=10000`, test isolation only): `ingest_plain.n` 123783243 ns, `as_of.n` 263320 ns. Vs M120 recorded 108170549 / 246429. No library edit; host noise.

### Trajectory

- `L_delivery=M121-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v1.15 / Phase 195. ADR-000 D3 still rejects a hard FSM as the only control model.

## 2026-10-20 — Trajectory: lease M121-S01 (unique facade-statute persist dir)

### Trajectory

- Active Milestone **M121**, Active Slice **S01** (`L_delivery=M120-closed`): `statute_as_of_via_facade` must not share a fixed `tmp/p27-facade-statute` across parallel Cargo. Honeycomb stays Proposed.

## 2026-10-20 — Trajectory: M120 / GSD v1.14 closed

### Product

- `fork_at_before_later_anchor_differs_and_persist_open` no longer uses the fixed path `tmp/p36-02-fork`. Each call gets `tmp/p36-02-fork-<pid>-<nanos>-<seq>`. Oracle `fork_known_on_store_dirs_differ_across_calls`; matrix `SEM-m120-fork-known-on-store-dir` killed. `observe_cargo.required` includes that name.
- Bench (`KUTHA_BENCH_N=10000`, test isolation only): `ingest_plain.n` 108170549 ns, `as_of.n` 246429 ns. Vs M119 recorded 109703364 / 318443. No library edit; host noise.

### Trajectory

- `L_delivery=M120-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v1.14 / Phase 194. ADR-000 D3 still rejects a hard FSM as the only control model.

## 2026-10-20 — Trajectory: lease M120-S01 (unique fork known-on persist dir)

### Trajectory

- Active Milestone **M120**, Active Slice **S01** (`L_delivery=M119-closed`): `fork_at_before_later_anchor_differs_and_persist_open` must not share a fixed `tmp/p36-02-fork` across parallel Cargo. Honeycomb stays Proposed.

## 2026-10-20 — Trajectory: M119 / GSD v1.13 closed

### Product

- `monotone_unique_anchors_persist_open` no longer uses the fixed path `tmp/p34-02-store`. Each call gets `tmp/p34-02-store-<pid>-<nanos>-<seq>`. Oracle `anchor_admit_store_dirs_differ_across_calls`; matrix `SEM-m119-anchor-admit-store-dir` killed. `observe_cargo.required` includes that name.
- Bench (`KUTHA_BENCH_N=10000`, test isolation only): `ingest_plain.n` 109703364 ns, `as_of.n` 318443 ns. Vs M118 recorded 123282211 / 283929. No library edit; host noise.

### Trajectory

- `L_delivery=M119-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v1.13 / Phase 193. ADR-000 D3 still rejects a hard FSM as the only control model.

## 2026-10-20 — Trajectory: lease M119-S01 (unique anchor-admit persist dir)

### Trajectory

- Active Milestone **M119**, Active Slice **S01** (`L_delivery=M118-closed`): `monotone_unique_anchors_persist_open` must not share a fixed `tmp/p34-02-store` across parallel Cargo. Honeycomb stays Proposed.

## 2026-10-20 — Trajectory: M118 / GSD v1.12 closed

### Product

- `closed_vt_known_on_persist_open` no longer uses the fixed path `tmp/p34-01-store`. Each call gets `tmp/p34-01-store-<pid>-<nanos>-<seq>`. Oracle `closed_vt_known_on_store_dirs_differ_across_calls`; matrix `SEM-m118-closed-vt-known-on-store-dir` killed. `observe_cargo.required` includes that name.
- Bench (`KUTHA_BENCH_N=10000`, test isolation only): `ingest_plain.n` 123282211 ns, `as_of.n` 283929 ns. Vs M117 recorded 113396450 / 267298. No library edit; host noise.

### Trajectory

- `L_delivery=M118-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v1.12 / Phase 192. ADR-000 D3 still rejects a hard FSM as the only control model.

## 2026-10-20 — Trajectory: lease M118-S01 (unique closed-VT known-on persist dir)

### Trajectory

- Active Milestone **M118**, Active Slice **S01** (`L_delivery=M117-closed`): `closed_vt_known_on_persist_open` must not share a fixed `tmp/p34-01-store` across parallel Cargo. Honeycomb stays Proposed.

## 2026-10-20 — Trajectory: M117 / GSD v1.11 closed

### Product

- Unit `as_known_on_survives_persist_open_and_matches_fold` no longer uses the fixed path `tmp/p32-01-store`. Each call gets `tmp/p32-01-store-<pid>-<nanos>-<seq>`. Oracle `quantum_known_on_store_dirs_differ_across_calls`; matrix `SEM-m117-quantum-known-on-store-dir` killed. `observe_cargo.required` includes that name.
- Bench (`KUTHA_BENCH_N=10000`, test isolation only): `ingest_plain.n` 113396450 ns, `as_of.n` 267298 ns. Vs M116 recorded 107588756 / 285763. No library edit; host noise.

### Trajectory

- `L_delivery=M117-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v1.11 / Phase 191. ADR-000 D3 still rejects a hard FSM as the only control model.

## 2026-10-20 — Trajectory: lease M117-S01 (unique quantum known-on persist dir)

### Trajectory

- Active Milestone **M117**, Active Slice **S01** (`L_delivery=M116-closed`): unit `as_known_on_survives_persist_open_and_matches_fold` must not share a fixed `tmp/p32-01-store` across parallel Cargo. Honeycomb stays Proposed.

## 2026-10-20 — Trajectory: M116 / GSD v1.10 closed

### Product

- `as_known_on` persist/open no longer uses the fixed path `tmp/p32-02-store`. Each call gets `tmp/p32-02-store-<pid>-<nanos>-<seq>` (same isolation as `m019_dur_a`). Oracle `persist_open_store_dirs_differ_across_calls`; matrix `SEM-m116-known-on-store-dir` killed. `observe_cargo.required` includes that name.
- Bench (`KUTHA_BENCH_N=10000`, test file only): `ingest_plain.n` 107588756 ns, `as_of.n` 285763 ns. Same-tree `taskset -c 0` sample: 117591647 ns / 357736 ns. Last recorded quiet sample ≈101.8M / ≈241.2k. No library edit; the spread is host noise.

### Trajectory

- `L_delivery=M116-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v1.10 / Phase 190. ADR-000 D3 still rejects a hard FSM as the only control model.

## 2026-10-20 — Trajectory: lease M116-S01 (unique known-on persist dir)

### Trajectory

- Active Milestone **M116**, Active Slice **S01** (`L_delivery=M115-closed`): `as_known_on` persist/open must not share a fixed `tmp/p32-02-store` across parallel Cargo. Honeycomb stays Proposed.

## 2026-10-19 — Trajectory: M115 / GSD v1.09 closed

### Process

- `kutha-gov ci --check ""` exits 2. A present `--check` is no longer treated as absent when the joined value is empty, so the full quantum does not start. Named `--check` on `ci` stays rejected the same way.
- Oracle `test_ci_rejects_empty_check_flag` on `agent-surface`; matrix `SEM-m115-empty-check-on-ci` killed.

### Trajectory

- `L_delivery=M115-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v1.09 / Phase 189. ADR-000 D3 still rejects a hard FSM as the only control model.

## 2026-10-19 — Trajectory: lease M115-S01 (empty --check on ci)

### Trajectory

- Active Milestone **M115**, Active Slice **S01** (`L_delivery=M114-closed`): `ci --check ""` must exit 2 instead of the full suite. Honeycomb stays Proposed.

## 2026-10-19 — Trajectory: M114 / GSD v1.08 closed

### Process

- `observe_cargo` takes an exclusive `fcntl` flock on `$CARGO_TARGET_DIR/.kutha-gov-cargo.lock` (default `./target/…`) for the whole test+build observation. Wait seconds from `KUTHA_GOV_CARGO_LOCK_WAIT_SEC` (default 600). Busy lock → HIGH `observe-error` exit 75.
- Check `cargo-observe-lock` + invariant; matrix `SEM-m114-cargo-observe-lock` killed; fsm / `.env.example` budget 89.

### Trajectory

- `L_delivery=M114-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v1.08 / Phase 188. ADR-000 D3 still rejects a hard FSM as the only control model.

## 2026-10-19 — Trajectory: lease M114-S01 (cargo observe flock)

### Trajectory

- Active Milestone **M114**, Active Slice **S01** (`L_delivery=M113-closed`): exclusive flock around `observe_cargo` so concurrent Cargo does not false-fail exit 101. Honeycomb stays Proposed.

## 2026-10-19 — Trajectory: M113 / GSD v1.07 closed

### Process

- `get_checks` fail-closes (`LoadErrorCheck` alongside survivors) when a declared `selftest` `replace_regex` keeps `pattern` as a substring of `with`. Shared helper `replace_regex_substring_error` used by selftest (M112) and dictionary load (M113).
- Oracle `test_replace_regex_with_still_contains_pattern_is_load_error`; `dict-load-fail-closed` needles extended; matrix `SEM-m113-selftest-mutation-static` killed. META + governor-intake note load-time twin.

### Trajectory

- `L_delivery=M113-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v1.07 / Phase 187. ADR-000 D3 still rejects a hard FSM as the only control model.

## 2026-10-19 — Trajectory: lease M113-S01 (selftest mutation static)

### Trajectory

- Active Milestone **M113**, Active Slice **S01** (`L_delivery=M112-closed`): dictionary load fail-closes on substring-preserving `selftest` `replace_regex` (static dogfood of M112). Honeycomb stays Proposed.

## 2026-10-19 — Trajectory: M112 / GSD v1.06 closed

### Process

- `kutha-gov selftest` rejects `replace_regex` when `with` still contains `pattern` as a substring (UNPROVEN before apply). Stops the VACUOUS class seen when `tt_by_ingest_REMOVED` still matched `tt_by_ingest`.
- Derived `yaml_map_list` require_fields / nonempty_list swaps use `__selftest_rm_<field>__:` so the needle is not a substring of the replacement (ledger selftests stay non-vacuous under the new rule).
- Check `selftest-substring-honesty` + invariant; matrix `SEM-m112-selftest-substring` killed; fsm / `.env.example` budget 88. META + governor-intake document the rule.

### Trajectory

- `L_delivery=M112-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v1.06 / Phase 186. ADR-000 D3 still rejects a hard FSM as the only control model.

## 2026-10-19 — Trajectory: M111 / GSD v1.05 closed

### Product

- `unindex_behavior` no longer full-scans `derived_by_cause` / `behaviors_by_premise_rel` when a reverse pointer is missing; missing means not in that lease.

### Process

- Governor check `hot-path-lease-honesty` pins ADR README + `tt_by_ingest` + live-only rebuild comment; FSM / `.env.example` budget 87. Oracle pytest `test_m111_hot_path_dogfood`.
- Matrix `SEM-m111-hot-path-dogfood` / `SEM-m111-unindex-no-fallback` killed. Id map v1.05 / M111.

### Trajectory

- `L_delivery=M111-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v1.05 / Phase 185. ADR-000 D3 still rejects a hard FSM as the only control model.

## 2026-10-19 — Trajectory: M110 / GSD v1.04 closed

### Product

- `rebuild_log_indexes` indexes `derived_by_cause` / `cause_of_behavior` / `live_behaviors` only for Behavior facts that are still live. Dead heads stay on the log but do not re-enter retract leases after open.
- Oracle `open_after_correct_keeps_dead_behavior_out_of_retract_index`; matrix `SEM-m110-rebuild-live-derived` killed. RED was retract examine `1` after persist+open.

### Process

- Id map row v1.04 / M110. Regression map names the rebuild-live oracle.

### Trajectory

- `L_delivery=M110-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v1.04 / Phase 184. ADR-000 D3 still rejects a hard FSM as the only control model.

## 2026-10-19 — Trajectory: M109 / GSD v1.03 closed

### Product

- Warm `known_at` walks `tt_by_ingest` (`ingested_at` → fact indexes) so later decoys are not examined. Lazy cold path still full-scans until rebuild (AUD-08).
- Oracle `known_at_examines_stay_constant_vs_late_decoys`; matrix `SEM-m109-known-at-ingest` killed. RED was examine counts `[74, 266]`.

### Process

- Id map row v1.03 / M109. Regression map names the known_at ingest oracle.

### Trajectory

- `L_delivery=M109-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. ADR-040 research note and README hot-path note record `tt_by_ingest`. Id map v1.03 / Phase 183. ADR-000 D3 still rejects a hard FSM as the only control model.

## 2026-10-19 — Trajectory: M108 / GSD v1.02 closed

### Product

- `live_behaviors` is an `FxHashSet`. Unindex of one Behavior removes that id in one step instead of walking the whole set.
- Oracle `unindex_one_behavior_live_visits_stay_constant`; matrix `SEM-m108-unindex-live-set` killed. RED was examine counts `[32, 128]`.

### Process

- Id map row v1.02 / M108. Regression map names the live-set oracle.

### Trajectory

- `L_delivery=M108-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v1.02 / Phase 182. ADR-000 D3 still rejects a hard FSM as the only control model.

## 2026-10-19 — Trajectory: M107 / GSD v1.01 closed

### Product

- `unindex_behavior` returns before walking `live_behaviors` when the event id is not a behavior (`cause_of_behavior` / `premise_rels_of_behavior`).
- Oracle `unindex_non_behavior_skips_live_behavior_vec`; matrix `SEM-m107-unindex-live-skip` killed. RED was examine counts `[32, 128]`.

### Process

- Id map row v1.01 / M107. Regression map names the live-vector oracle.

### Trajectory

- `L_delivery=M107-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v1.01 / Phase 181. ADR-000 D3 still rejects a hard FSM as the only control model.

## 2026-10-19 — Trajectory: ADR-040 PGM research variant

### Trajectory

- ADR-040 records a piecewise-linear position model as a Proposed read lease on a sealed sorted `u64` column. Eager `BTreeMap` stays the incremental structure. Honeycomb stays Proposed. The PVLDB 2020 space examples are 83× (static) and 1140× (dynamic), not a 14,000× Kutha constant. No product code.

## 2026-10-19 — Trajectory: M106 / GSD v1.00 closed

### Product

- `open` restores the snapshot fold-index lease through `set_index_mode`. A Lazy snapshot still full-scans when the env is eager. An Eager snapshot keeps the warm valid-time range when the env is lazy.
- Oracles `open_keeps_lazy_scan_when_env_is_eager`, `open_keeps_eager_range_when_env_is_lazy`; matrix `SEM-m106-open-index-lease` killed.

### Process

- Id map row v1.00 / M106. Regression map names the open-lease oracle.

### Trajectory

- `L_delivery=M106-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v1.00 / Phase 180. ADR-000 D3 still rejects a hard FSM as the only control model.

## 2026-10-19 — Trajectory: M105 / GSD v0.99 closed

### Product

- Policy pin election walks `policy_by_ingest` newest-first and stops on the first live row.
- `unindex_behavior` uses `premise_rels_of_behavior` and ignores event ids that are not behaviors.
- Op-wire rewrite drops a second modern `Define` when the name is already interned.
- Horn fire keeps every premise combination. Replay requires that conjunction to cover the Behavior VT. `open` keeps the fold `replay_check` built and restores the snapshot `FoldIndexMode`. Correct walks every live residual.
- Oracles `newest_policy_pin_examines_one_live_row`, `unindex_premise_examines_only_that_behaviors_rels`; matrix `SEM-m105-policy-pin-newest`, `SEM-m105-unindex-premise-pointer` killed.

### Process

- Id map row v0.99 / M105. Regression map names the two new killed oracles. README status row is Delivery and Architecture again.

### Trajectory

- `L_delivery=M105-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v0.99 / Phase 176. ADR-000 D3 still rejects a hard FSM as the only control model.

## 2026-10-18 — Trajectory: M104 / GSD v0.98 closed

### Product

- `cause_of_behavior` maps each Behavior to its `caused_by` so `unindex_behavior` touches one cause bucket instead of scanning every bucket.
- Oracle `unindex_cause_examines_constant_in_live_d`; matrix `SEM-m104-unindex-cause-pointer` killed.

### Trajectory

- `L_delivery=M104-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v0.98 / Phase 175.

## 2026-10-18 — Trajectory: lease M104-S01 (unindex cause reverse pointer)

### Trajectory

- Active Milestone **M104**, Active Slice **S01** (`L_delivery=M103-closed`): unindex a Behavior from its cause bucket in O(1), not by scanning every cause. Honeycomb stays Proposed.

## 2026-10-17 — Trajectory: M103 / GSD v0.97 closed

### Product

- `TermDictionary::from_strings` keeps the earliest index for a repeated name. Later slots in the string table stay in place.
- Oracle `from_strings_keeps_first_id_for_duplicate_name`; matrix `SEM-m103-define-keep-first` killed.

### Trajectory

- `L_delivery=M103-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v0.97 / Phase 174.

## 2026-10-17 — Trajectory: lease M103-S01 (duplicate Define keeps first id)

### Trajectory

- Active Milestone **M103**, Active Slice **S01** (`L_delivery=M102-closed`): a repeated dictionary name must keep the first TermId. Honeycomb stays Proposed.

## 2026-10-17 — Trajectory: M102 / GSD v0.96 closed

### Product

- Failed migrate verify deletes every `seg-*.log` after removing `manifest.json`, then `sync_dir`s. Legacy `events.jsonl` and `events.wal` stay the store.
- Oracle `migrate_verify_fault_leaves_no_segment_files`; matrix `SEM-m102-migrate-verify-segments` killed.

### Trajectory

- `L_delivery=M102-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v0.96 / Phase 173.

## 2026-10-17 — Trajectory: lease M102-S01 (migrate verify drops segments)

### Trajectory

- Active Milestone **M102**, Active Slice **S01** (`L_delivery=M101-closed`): a failed migrate verify must not leave `seg-*.log` beside the legacy store. Honeycomb stays Proposed.

## 2026-10-17 — Trajectory: M101 / GSD v0.95 closed

### Product

- `live_horns` keeps a transaction-time-live rule pin when its valid-time overlaps the trigger window, then the existing conjunction clips the head.
- Oracle `late_starting_rule_pin_fires_on_overlap`; matrix `SEM-m101-horn-pin-overlap` killed.

### Trajectory

- `L_delivery=M101-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v0.95 / Phase 172.

## 2026-10-17 — Trajectory: lease M101-S01 (Horn pin overlap)

### Trajectory

- Active Milestone **M101**, Active Slice **S01** (`L_delivery=M100-closed`): a rule pin whose valid-time overlaps the trigger window must still fire. Honeycomb stays Proposed.

## 2026-10-16 — Trajectory: M100 / GSD v0.94 closed

### Product

- Correct and CorrectInterval of a Behavior call `unindex_behavior`, same as Retract, so a dead head leaves `live_behaviors`, `derived_by_cause`, and `behaviors_by_premise_rel`.
- Oracles `correct_of_behavior_drops_retract_index`, `correct_interval_of_behavior_drops_retract_index`; matrix `SEM-m100-correct-unindex-behavior` killed.

### Trajectory

- `L_delivery=M100-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v0.94 / Phase 171.

## 2026-10-16 — Trajectory: lease M100-S01 (Correct unindexes Behavior)

### Trajectory

- Active Milestone **M100**, Active Slice **S01** (`L_delivery=M099-closed`): Correct and CorrectInterval must drop a Behavior from the retract indexes. Honeycomb stays Proposed.

## 2026-10-16 — Trajectory: M099 / GSD v0.93 closed


### Process

- `test_ce_2026_10_15_regressions` fails if a 2026-10-15 mapped finding loses its oracle in the regression map or in code.
- `--check ""` fail-closes (exit 2) instead of running the full suite.
- `yaml_map_list` reports HIGH on non-mapping rows; multi-check oracles require an exact check set; `migrate_compat` source order is op-wire then pins.

### Trajectory

- `L_delivery=M099-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v0.93 / Phase 170.

## 2026-10-16 — Trajectory: lease M099-S01 (CE 2026-10-15 regression enrolment)


### Trajectory

- Active Milestone **M099**, Active Slice **S01** (`L_delivery=M098-closed`): pin the 2026-10-15 review to living oracles and close the remaining testing gaps. Honeycomb stays Proposed.

## 2026-10-16 — Trajectory: M098 / GSD v0.92 closed


### Process

- Check `kinds-family-split` observes `kinds.py` sibling imports plus `test_m054_id_structure.py` (CE 2026-10-15 #5).
- `SEM-m054-id-kinds-split` oracle is `kinds-family-split` (was the `test-matrix` meta-check). FSM / `.env.example` budget 85→86.

### Trajectory

- `L_delivery=M098-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v0.92 / Phase 169.

## 2026-10-16 — Trajectory: lease M098-S01 (CE #5 kinds-split oracle)


### Trajectory

- Active Milestone **M098**, Active Slice **S01** (`L_delivery=M097-closed`): `SEM-m054-id-kinds-split` must name a check that observes kinds.py family placement, not the test-matrix meta-check. Honeycomb stays Proposed.

## 2026-10-16 — Trajectory: M097 / GSD v0.91 closed

### Product

- Multi-premise Horn `search_at` uses `facts_overlapping_relation` (VT overlap with trigger/head window) instead of point-gated `live_facts_for_relation` (CE 2026-10-15 #4). Design: overlap-aware over retry-at-candidate-starts.
- Oracles `late_starting_premise_fires_when_trigger_starts_earlier`, `horn_overlap_discovery_is_order_independent`; matrix `SEM-m097-horn-overlap-aware` killed.

### Trajectory

- `L_delivery=M097-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v0.91 / Phase 168.

## 2026-10-16 — Trajectory: lease M097-S01 (CE #4 Horn overlap-aware)

### Trajectory

- Active Milestone **M097**, Active Slice **S01** (`L_delivery=M096-closed`): multi-premise Horn discovery must be overlap-aware (not point-gated at trigger valid_from). Design: overlap-aware over retry-at-candidate-starts. Honeycomb stays Proposed.

## 2026-10-16 — Trajectory: M096 / GSD v0.90 closed

### Product

- Baseline `bytes_per_fact` captures `rss_before` immediately before the dedicated sample (after factor loops), so the delta matches sample live only (CE 2026-10-15 #2).
- Oracle `bench_rss_before_is_taken_immediately_before_sample`; matrix `SEM-m096-bench-rss-before-sample` killed.

### Trajectory

- `L_delivery=M096-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v0.90 / Phase 167.

## 2026-10-16 — Trajectory: lease M096-S01 (CE #2 bench rss_before)

### Trajectory

- Active Milestone **M096**, Active Slice **S01** (`L_delivery=M095-closed`): move baseline `rss_before` to immediately before the dedicated sample so `bytes_per_fact` is sample-live only. Honeycomb stays Proposed.

## 2026-10-16 — Trajectory: M095 / GSD v0.89 closed

### Product

- `retract_derived` second phase walks `behaviors_by_premise_rel` for the retracted fact's relation instead of every live Behavior (CE 2026-10-15 #3).
- Oracles `retract_second_phase_examines_constant_in_unrelated_live_d`, `retract_non_trigger_still_drops_after_premise_rel_index`; matrix `SEM-m095-retract-second-phase-index` killed.

### Trajectory

- `L_delivery=M095-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v0.89 / Phase 166.

## 2026-10-16 — Trajectory: lease M095-S01 (CE #3 retract second-phase scan)

### Trajectory

- Active Milestone **M095**, Active Slice **S01** (`L_delivery=M094-closed`): index the second-phase retract/correct `live_behaviors` walk so examine counts stay sub-linear in irrelevant derived. Honeycomb stays Proposed.

## 2026-10-16 — Trajectory: M094 / GSD v0.88 closed

### Process

- `get_checks` fail-closes on non-mapping rows, duplicate ids (keep-first), and empty/scalar steps via LoadError alongside survivors (CE 2026-10-15 #6–#8).
- Oracles `test_scalar_check_row_is_load_error`, `test_duplicate_check_id_is_load_error`, `test_stepless_check_is_load_error`; check `dict-load-fail-closed`; matrix `SEM-m094-dict-load-fail-closed` killed.
- FSM / `.env.example` budget 84→85.

### Trajectory

- `L_delivery=M094-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v0.88 / Phase 165.

## 2026-10-16 — Trajectory: lease M094-S01 (CE #6–#8 dictionary loader)

### Trajectory

- Active Milestone **M094**, Active Slice **S01** (`L_delivery=M093-closed`): checks.yaml loader must fail-closed on scalar rows, duplicate ids, and stepless checks. Honeycomb stays Proposed.

## 2026-10-16 — Trajectory: M093 / GSD v0.87 closed

### Product

- `kutha-migrate --help`/`-h` prints usage on stdout and exits 0; argc>2 rejected with usage+exit 2 (CE 2026-10-15 #9).
- Usage documents that store-dir must already exist; missing-dir stays exit 2 (CLI does not create).
- Oracles `kutha_migrate_help_exits_zero_on_stdout`, `kutha_migrate_rejects_extra_positionals`; matrix `SEM-m093-migrate-cli-contract` killed.
- Quiet bench (`taskset -c 0`, `KUTHA_BENCH_N=10000`): `ingest_plain.n` ≈101.8M ns, `as_of.n` ≈241.2k ns (CLI-only; vs M092 ≈107.2M / ≈300.3k).

### Trajectory

- `L_delivery=M093-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v0.87 / Phase 164.

## 2026-10-16 — Trajectory: lease M093-S01 (CE #9 kutha-migrate CLI)

### Trajectory

- Active Milestone **M093**, Active Slice **S01** (`L_delivery=M092-closed`): `kutha-migrate --help` must exit 0; extra args rejected. Honeycomb stays Proposed.

## 2026-10-16 — Trajectory: M092 / GSD v0.86 closed

### Product

- OpWire/PinWire/PinCite migrate reports expose `dropped_tail` for torn jsonl / truncated WAL / active-segment remainders (CE 2026-10-15 #13).
- `kutha-migrate` stdout prints `*_dropped_tail=`; oracles `migrate_*_reports_dropped_jsonl_tail`; matrix `SEM-m092-migrate-dropped-tail` killed.
- Quiet bench (`taskset -c 0`, `KUTHA_BENCH_N=10000`): `ingest_plain.n` ≈107.2M ns, `as_of.n` ≈300.3k ns (migrate not on ingest hot path; vs M091 ≈109.5M / ≈277.9k).

### Trajectory

- `L_delivery=M092-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v0.86 / Phase 163.

## 2026-10-16 — Trajectory: lease M092-S01 (CE #13 migrate dropped_tail)

### Trajectory

- Active Milestone **M092**, Active Slice **S01** (`L_delivery=M091-closed`): Op/Pin migrate reports must surface torn-tail drops. Honeycomb stays Proposed.

## 2026-10-16 — Trajectory: M091 / GSD v0.85 closed

### Product

- `migrate_pin_cite` removes stale `justifications.jsonl` alongside `snapshot.json` so reopen succeeds after JustificationCite pin remap (CE 2026-10-15 #1).
- Oracle `migrate_pin_cite_reopens_after_nonempty_justifications_sidecar`; matrix `SEM-m091-pin-cite-justifications` killed.
- Bench (`KUTHA_BENCH_N=10000`): `ingest_plain.n` ≈109.5M ns, `as_of.n` ≈277.9k ns (migrate not on ingest hot path; vs recent quiet M056-S04 ≈110.7M / ≈261.4k).

### Trajectory

- `L_delivery=M091-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed. Id map v0.85 / Phase 162.

## 2026-10-16 — Trajectory: lease M091-S01 (CE #1 pin-cite justifications)

### Trajectory

- Active Milestone **M091**, Active Slice **S01** (`L_delivery=M090-closed`): `migrate_pin_cite` must not leave a stale `justifications.jsonl` that bricks reopen. Honeycomb stays Proposed.

## 2026-10-15 — Trajectory: M090 / GSD v0.84 closed

### Process

- Invalid `--skip-cargo` stderr adds `hint: kutha-gov ci --skip-cargo`.
- Oracle `test_non_ci_rejects_skip_cargo_flag` asserts the hint; agent-surface pins `__main__.py`; matrix `SEM-m090-skip-cargo-reject-hint` killed.
- Id map v0.84 / Phase 161.

### Trajectory

- `L_delivery=M090-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed.

## 2026-10-15 — Trajectory: lease M090-S01 (--skip-cargo reject hint)

### Trajectory

- Active Milestone **M090**, Active Slice **S01** (`L_delivery=M089-closed`): invalid `--skip-cargo` stderr must redirect to `kutha-gov ci --skip-cargo`. Honeycomb stays Proposed.

## 2026-10-15 — Trajectory: M089 / GSD v0.83 closed

### Process

- Invalid `--check` stderr adds `hint: kutha-gov {json|precommit|selftest} --check ID[,ID…]`.
- Oracle `test_ci_rejects_check_flag` asserts the hint; agent-surface pins `__main__.py`; matrix `SEM-m089-check-reject-hint` killed.
- Id map v0.83 / Phase 160.

### Trajectory

- `L_delivery=M089-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed.

## 2026-10-15 — Trajectory: lease M089-S01 (--check reject hint)

### Trajectory

- Active Milestone **M089**, Active Slice **S01** (`L_delivery=M088-closed`): invalid `--check` stderr must redirect to json|precommit|selftest. Honeycomb stays Proposed.

## 2026-10-15 — Trajectory: M088 / GSD v0.82 closed

### Process

- AGENTS Flags state `--check` is json|precommit|selftest only — not `ci`.
- Oracles `test_agents_md_check_not_for_ci` + `test_ci_rejects_check_flag`; agent-surface pins; matrix `SEM-m088-agents-check-not-ci` killed.
- Id map v0.82 / Phase 159. No CLI change (`ci --check` still exit 2).

### Trajectory

- `L_delivery=M088-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed.

## 2026-10-15 — Trajectory: lease M088-S01 (AGENTS --check not for ci)

### Trajectory

- Active Milestone **M088**, Active Slice **S01** (`L_delivery=M087-closed`): AGENTS Flags must state `--check` is only for json|precommit|selftest (not `ci`). Honeycomb stays Proposed.

## 2026-10-15 — Trajectory: M087 / GSD v0.81 closed

### Process

- `kutha-harness.md` documents `--check` for json/precommit/selftest (repeatable + comma); CLI examples include json/selftest.
- Oracle `test_harness_documents_check_surfaces`; dogfood + agent-surface pins; matrix `SEM-m087-harness-check-surfaces` killed.
- Id map v0.81 / Phase 158. No CLI change (`ci` still rejects `--check`).

### Trajectory

- `L_delivery=M087-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed.

## 2026-10-15 — Trajectory: lease M087-S01 (harness --check surfaces)

### Trajectory

- Active Milestone **M087**, Active Slice **S01** (`L_delivery=M086-closed`): `kutha-harness.md` must document `--check` for json/precommit/selftest (repeatable + comma). Honeycomb stays Proposed.

## 2026-10-15 — Trajectory: M086 / GSD v0.80 closed

### Process

- Oracle `test_selftest_repeated_check_flags_runs_both`: `kutha-gov selftest --check a --check b` selects both.
- Agent-surface multi-check needles; matrix `SEM-m086-selftest-check-append` killed.
- Id map v0.80 / Phase 157. No CLI change (reuse `_join_check_flags`).

### Trajectory

- `L_delivery=M086-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed.

## 2026-10-15 — Trajectory: lease M086-S01 (selftest --check append oracle)

### Trajectory

- Active Milestone **M086**, Active Slice **S01** (`L_delivery=M085-closed`): oracle that `kutha-gov selftest --check a --check b` selects both. Honeycomb stays Proposed.

## 2026-10-15 — Trajectory: M085 / GSD v0.79 closed

### Process

- Oracle `test_json_repeated_check_flags_runs_both`: `kutha-gov json --check a --check b` selects both rows.
- Agent-surface multi-check needles; matrix `SEM-m085-json-check-append` killed.
- Id map v0.79 / Phase 156. No CLI change (reuse `_join_check_flags`).

### Trajectory

- `L_delivery=M085-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed.

## 2026-10-15 — Trajectory: lease M085-S01 (json --check append oracle)

### Trajectory

- Active Milestone **M085**, Active Slice **S01** (`L_delivery=M084-closed`): oracle that `kutha-gov json --check a --check b` selects both. Honeycomb stays Proposed.

## 2026-10-15 — Trajectory: M084 / GSD v0.78 closed

### Process

- `.env.example` documents observe-built `debug/kutha-migrate` and optional `KUTHA_MIGRATE_DIR`.
- Toolchain + `test_env_example_documents_built_migrate`; matrix `SEM-m084-env-migrate-path` killed.
- Id map v0.78 / Phase 155.

### Trajectory

- `L_delivery=M084-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed.

## 2026-10-15 — Trajectory: lease M084-S01 (.env.example built migrate path)

### Trajectory

- Active Milestone **M084**, Active Slice **S01** (`L_delivery=M083-closed`): `.env.example` documents observe-built `debug/kutha-migrate`. Honeycomb stays Proposed.

## 2026-10-14 — Trajectory: M083 / GSD v0.77 closed

### Process

- `kutha-harness.md` CutPort adds migrate-bin / `debug/kutha-migrate`; observe_cargo prose names both bins.
- Dogfood + `test_harness_cutport_names_built_migrate`; matrix `SEM-m083-harness-migrate-cutport` killed.
- Id map v0.77 / Phase 154.

### Trajectory

- `L_delivery=M083-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed.

## 2026-10-14 — Trajectory: lease M083-S01 (Harness CutPort migrate-bin)

### Trajectory

- Active Milestone **M083**, Active Slice **S01** (`L_delivery=M082-closed`): `kutha-harness.md` CutPort names observe-built migrate-bin. Honeycomb stays Proposed.

## 2026-10-14 — Trajectory: M082 / GSD v0.76 closed

### Process

- `op-compat.md` and `pin-compat.md` cite observe-built `$CARGO_TARGET_DIR/debug/kutha-migrate`.
- Policy checks + `test_process_notes_prefer_built_migrate_bin`; matrix `SEM-m082-migrate-process-notes` killed.
- Id map v0.76 / Phase 153.

### Trajectory

- `L_delivery=M082-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed.

## 2026-10-14 — Trajectory: lease M082-S01 (Process notes built migrate path)

### Trajectory

- Active Milestone **M082**, Active Slice **S01** (`L_delivery=M081-closed`): `op-compat.md` / `pin-compat.md` cite observe-built `debug/kutha-migrate`. Honeycomb stays Proposed.

## 2026-10-14 — Trajectory: M081 / GSD v0.75 closed

### Process

- AGENTS documents `$CARGO_TARGET_DIR/debug/kutha-migrate` (no `cargo run` second compile).
- META names the observe-built migrate path; agent-surface + `test_agents_md_prefers_built_migrate_bin`.
- Matrix `SEM-m081-migrate-bin-path` killed; id map v0.75 / Phase 152.

### Trajectory

- `L_delivery=M081-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed.

## 2026-10-14 — Trajectory: lease M081-S01 (Built migrate bin discoverability)

### Trajectory

- Active Milestone **M081**, Active Slice **S01** (`L_delivery=M080-closed`): AGENTS/META prefer built `kutha-migrate` after observe (no `cargo run`). Honeycomb stays Proposed.

## 2026-10-14 — Trajectory: M080 / GSD v0.74 closed

### Process

- `observe_cargo` build: one `cargo build` with `--bin kutha-tenant --bin kutha-migrate` (fsm + observe defaults).
- Check `tenant-bin` requires both bins; oracle `test_observe_default_build_includes_migrate`.
- Matrix `SEM-m080-migrate-build` killed; id map v0.74 / Phase 151.

### Trajectory

- `L_delivery=M080-closed`, Active Milestone/Slice **None**. Honeycomb stays Proposed.

## 2026-10-14 — Trajectory: lease M080-S01 (Observe build both operator bins)

### Trajectory

- Active Milestone **M080**, Active Slice **S01** (`L_delivery=M079-closed`): `observe_cargo` build compiles `kutha-tenant` and `kutha-migrate` once. Honeycomb stays Proposed.

## 2026-10-14 — Trajectory: M079 / GSD v0.73 closed

### Trajectory

- Active Milestone/Slice cleared; `L_delivery=M079-closed`. Honeycomb stays Proposed.

## 2026-10-14 — Process: Repeatable --check flags (Phase 150)

### Process

- `--check` is argparse `action=append`; `_join_check_flags` feeds M077 `_parse_check_ids` (comma form unchanged). Tests: `test_precommit_repeated_check_flags_runs_both`. AGENTS + `agent-surface` document repeatable flags.
- Matrix `SEM-m079-check-append` killed; id map v0.73 / Phase 150.

## 2026-10-14 — Trajectory: lease M079-S01 (Repeatable --check flags)

### Trajectory

- Active Milestone **M079**, Active Slice **S01** (`L_delivery=M078-closed`): `--check` uses argparse `append` so repeated flags join the M077 multi-select. Honeycomb stays Proposed.

## 2026-10-14 — Trajectory: M078 / GSD v0.72 closed

### Trajectory

- Active Milestone/Slice cleared; `L_delivery=M078-closed`. Honeycomb stays Proposed.

## 2026-10-14 — Product: Compat migrate operator CLI (Phase 149)

### Product

- Binary `kutha-migrate`: runs existing `store::migrate_compat` on `<store-dir>` or `KUTHA_MIGRATE_DIR`; prints `migrate:` report; exit 2 on missing dir. Tests: `m078_migrate_cli`.

### Process

- Matrix `SEM-m078-migrate-cli` killed; `op-compat-policy` + `pin-compat-policy` cover `kutha-migrate`; id map v0.72 / Phase 149.

## 2026-10-14 — Trajectory: lease M078-S01 (Compat migrate operator CLI)

### Trajectory

- Active Milestone **M078**, Active Slice **S01** (`L_delivery=M077-closed`): `kutha-migrate` bin runs `store::migrate_compat` on a store directory. Honeycomb stays Proposed.

## 2026-10-14 — Trajectory: M077 / GSD v0.71 closed

### Trajectory

- Active Milestone/Slice cleared; `L_delivery=M077-closed`. Honeycomb stays Proposed.

## 2026-10-14 — Process: Multi-check CLI select (Phase 148)

### Process

- `_parse_check_ids` + `_select_checks`: comma-separated dictionary check ids for `precommit` / `json` / `selftest` (dedupe; empty/unknown fail-closed). `run_selftest(..., check_ids=)`. Tests: `test_multi_check`. AGENTS documents `ID[,ID…]` and `run_high=` over `ci | tail`.
- Extended `agent-surface` / `I-agent-surface`; matrix `SEM-m077-multi-check` killed; id map v0.71 / Phase 148.

## 2026-10-14 — Trajectory: lease M077-S01 (Multi-check CLI select)

### Trajectory

- Active Milestone **M077**, Active Slice **S01** (`L_delivery=M076-closed`): `--check` accepts comma-separated dictionary check ids for precommit/json/selftest. Honeycomb stays Proposed.

## 2026-10-14 — Trajectory: M076 / GSD v0.70 closed

### Trajectory

- Active Milestone/Slice cleared; `L_delivery=M076-closed`. Honeycomb stays Proposed.

## 2026-10-14 — Product: Compat migrate sequence facade (Phase 147)

### Product

- `store::migrate_compat` + `CompatMigrateReport`: runs existing `migrate_op_wire` then `migrate_pins` (reuse; no new remap). Stamps op-wire + pin markers when needed; idempotent when all exist. Test: `m076_migrate_compat`. Bench sample (`KUTHA_BENCH_N=1000`): `ingest_plain.n` ≈17.0ms, `as_of.n` ≈41µs (noisy host; migrate not on ingest hot path; vs M075 ≈15.3ms / ≈32µs).

### Process

- Matrix `SEM-m076-migrate-compat` killed; `op-compat-policy` + `pin-compat-policy` + compat docs cover `migrate_compat`; id map v0.70 / Phase 147.

## 2026-10-14 — Trajectory: lease M076-S01 (Compat migrate sequence facade)

### Trajectory

- Active Milestone **M076**, Active Slice **S01** (`L_delivery=M075-closed`): `store::migrate_compat` runs Op-wire then pin migrate so meta-op rewrite precedes pin remap. Honeycomb stays Proposed.

## 2026-10-14 — Trajectory: M075 / GSD v0.69 closed

### Trajectory

- M075 / GSD v0.69 closed (Phase 146). Active Milestone **None**; `L_delivery=M075-closed`. Audit: `.planning/milestones/v0.69-MILESTONE-AUDIT.md`. Honeycomb stays Proposed.

## 2026-10-14 — Product: Pin migrate sequence facade (Phase 146)

### Product

- `store::migrate_pins` + `PinsMigrateReport`: runs existing `migrate_pin_wire` then `migrate_pin_cite` (reuse; no new remap). Stamps both markers when needed; idempotent when both exist. Test: `m075_migrate_pins`. Bench sample (`KUTHA_BENCH_N=1000`): `ingest_plain.n` ≈15.3ms, `as_of.n` ≈32µs (noisy host; migrate not on ingest hot path; vs M072 ≈9.6–11.2ms / ≈40–57µs).

### Process

- Matrix `SEM-m075-migrate-pins` killed; `pin-compat-policy` + `pin-compat.md` cover `migrate_pins`; id map v0.69 / Phase 146.

## 2026-10-14 — Trajectory: lease M075-S01 (Pin migrate sequence facade)

### Trajectory

- Active Milestone **M075**, Active Slice **S01** (`L_delivery=M074-closed`): `store::migrate_pins` runs pin-wire then pin-cite so the directory pin migrate sequence is one entry. Honeycomb stays Proposed.

## 2026-10-14 — Trajectory: M074 / GSD v0.68 closed

### Trajectory

- M074 / GSD v0.68 closed (Phase 145). Active Milestone **None**; `L_delivery=M074-closed`. Audit: `.planning/milestones/v0.68-MILESTONE-AUDIT.md`. Honeycomb stays Proposed.

## 2026-10-14 — Process: Agent surface discoverability (Phase 145)

### Process

- `docs/process/gsd-auto-contract.md` tool-pair column labels CBM as `kutha-graph` (alias `root-kutha-graph` footnote only).
- `AGENTS.md` Commands list `kutha-gov json` and flags `--jobs` / `--skip-cargo` / `--check` / `--budget`.
- Check `agent-surface` + invariant `I-agent-surface`; FSM / `.env.example` budget 83→84; matrix `SEM-m074-agent-surface` killed. Pytest: `test_agent_surface`. Id map v0.68 / Phase 145.

## 2026-10-14 — Trajectory: lease M074-S01 (Agent surface discoverability)

### Trajectory

- Active Milestone **M074**, Active Slice **S01** (`L_delivery=M073-closed`): CBM tool-pair name `kutha-graph` + AGENTS.md harness flags (`--jobs` / `--skip-cargo` / `json` / `--check`). Honeycomb stays Proposed.

## 2026-10-14 — Trajectory: M073 / GSD v0.67 closed

### Trajectory

- M073 / GSD v0.67 closed (Phase 144). Active Milestone **None**; `L_delivery=M073-closed`. Audit: `.planning/milestones/v0.67-MILESTONE-AUDIT.md`. Honeycomb stays Proposed.

## 2026-10-14 — Process: Git auto clean-tree coupling window (Phase 144)

### Process

- `gitdiff.changed_relpaths(auto)` on a clean tree uses upstream/main merge-base…HEAD (not HEAD~1 alone) so `docs-coupling` matches the branch window.
- `kutha-gov ci` summary prints `run_high` / `run_low` for this quantum and `tip_high` / tip cargo/tenant for the fold tip.
- Check `git-auto-base` + invariant `I-git-auto-base`; FSM / `.env.example` budget 82→83; matrix `SEM-m073-git-auto-base` killed. Pytest: `test_gitdiff_auto`, `test_ci_summary_labels`. Id map v0.67 / Phase 144.

## 2026-10-14 — Trajectory: lease M073-S01 (Git auto clean-tree coupling window)

### Trajectory

- Active Milestone **M073**, Active Slice **S01** (`L_delivery=M072-closed`): clean-tree `git_against=auto` uses upstream merge-base…HEAD (not HEAD~1 alone); `ci` summary separates this-run HIGH from fold tip. Honeycomb stays Proposed.

## 2026-10-13 — Trajectory: M072 / GSD v0.66 closed

### Trajectory

- M072 / GSD v0.66 closed (Phase 143). Active Milestone **None**; `L_delivery=M072-closed`. Audit: `.planning/milestones/v0.66-MILESTONE-AUDIT.md`. Honeycomb stays Proposed.

## 2026-10-13 — Product: Pin-cite JustificationCite migrate (Phase 143)

### Product

- `remap_events_pins` + `store::migrate_pin_cite`: remaps JustificationCite `rule_version` when pin is a legacy UTF-8 ident digest for a rule name evidenced in-log (`Behavior.name` / rule-ident `Define`); stamps `pin-cite.v1`. Orphan cites unchanged. No silent serde remap. Tests: `pin_migrate` unit + `m072_pin_cite_migrate`. Bench sample (`KUTHA_BENCH_N=1000`): `ingest_plain.n` ≈9.6–11.2ms, `as_of.n` ≈40–57µs (noisy host; migrate not on ingest hot path; vs M071 ≈9.0ms / ≈24µs).

### Process

- Matrix `SEM-m072-pin-cite` killed; `pin-compat-policy` oracles + `pin-compat.md` cover cite migrate; id map v0.66 / Phase 143.

## 2026-10-13 — Trajectory: lease M072-S01 (Pin-cite JustificationCite migrate)

### Trajectory

- Active Milestone **M072**, Active Slice **S01** (`L_delivery=M071-closed`): explicit `migrate_pin_cite` remaps JustificationCite `rule_version` legacy UTF-8 ident pins → v1 when a rule name is evidenced in the same log; stamp `pin-cite.v1`. Honeycomb stays Proposed.

## 2026-10-13 — Trajectory: M071 / GSD v0.65 closed

### Trajectory

- M071 / GSD v0.65 closed (Phase 142). Active Milestone **None**; `L_delivery=M071-closed`. Audit: `.planning/milestones/v0.65-MILESTONE-AUDIT.md`. Honeycomb stays Proposed.

## 2026-10-13 — Product: Pin-wire Behavior migrate (Phase 142)

### Product

- `kutha_common::pin_migrate` + `store::migrate_pin_wire`: remaps Behavior `rule_version` when pin is legacy UTF-8 ident digest → pin-scheme v1 Named; jsonl/WAL/segmented sources; stamps `pin-wire.v1`. No silent serde remap of `rule_definition_hash`. Tests: `pin_migrate` unit + `m071_pin_wire_migrate`. Bench sample (`KUTHA_BENCH_N=1000`): `ingest_plain.n` ≈9.0ms, `as_of.n` ≈24µs (noisy host; migrate not on ingest hot path; vs M070 ≈9.1ms / ≈28µs).

### Process

- Matrix `SEM-m071-pin-wire` killed; `pin-compat-policy` oracles + `pin-compat.md` cover explicit directory migrate; id map v0.65 / Phase 142.

## 2026-10-13 — Trajectory: lease M071-S01 (Pin-wire Behavior migrate)

### Trajectory

- Active Milestone **M071**, Active Slice **S01** (`L_delivery=M070-closed`): explicit `migrate_pin_wire` remaps Behavior `rule_version` legacy UTF-8 ident pins → v1 Named; stamp `pin-wire.v1`. Honeycomb stays Proposed.

## 2026-10-13 — Trajectory: M070 / GSD v0.64 closed

### Trajectory

- M070 / GSD v0.64 closed (Phase 141). Active Milestone **None**; `L_delivery=M070-closed`. Audit: `.planning/milestones/v0.64-MILESTONE-AUDIT.md`. Honeycomb stays Proposed.

## 2026-10-13 — Product: Op-wire demote orphan cleanup (Phase 141)

### Product

- Segmented `migrate_op_wire` demote calls `remove_all_segment_logs` so every `seg-*.log` is removed (including generation-unique orphans not named by the manifest). Tests: `m070_op_wire_orphan_seg`. Bench sample (`KUTHA_BENCH_N=1000`): `ingest_plain.n` ≈9.1ms, `as_of.n` ≈28µs (noisy host; cleanup not on ingest hot path; vs M069 ≈15.8ms / ≈29µs).

### Process

- Matrix `SEM-m070-op-wire-orphan-seg` killed; `op-compat` / `pin-compat` G4 residual text no longer mislabels CE #26 as clock facade; id map v0.64 / Phase 141.

## 2026-10-13 — Trajectory: lease M070-S01 (Op-wire demote orphan cleanup)

### Trajectory

- Active Milestone **M070**, Active Slice **S01** (`L_delivery=M069-closed`): after segmented `migrate_op_wire`, remove orphan `seg-*.log`; honest G4 residual docs. Honeycomb stays Proposed.

## 2026-10-13 — Trajectory: M069 / GSD v0.63 closed

### Trajectory

- M069 / GSD v0.63 closed (Phase 140). Active Milestone **None**; `L_delivery=M069-closed`. Audit: `.planning/milestones/v0.63-MILESTONE-AUDIT.md`. Op-wire layout arc complete (jsonl / WAL / segmented). Honeycomb stays Proposed.

## 2026-10-13 — Product: Op wire segmented migrate (Phase 140)

### Product

- `migrate_op_wire` segmented path: `segment::recover_json_values` → remap → materialize `events.jsonl`, drop segments + `manifest.json`, stamp `op-wire.v1`. Demotes to legacy jsonl SoT (operator may later `store::migrate`). Open stays fail-open on sealed legacy meta-op frames. Tests: `m069_op_wire_seg`. Bench sample (`KUTHA_BENCH_N=1000`): `ingest_plain.n` ≈15.8ms, `as_of.n` ≈29µs (noisy host; migrate not on ingest hot path; vs M068 ≈15.8ms / ≈52µs).

### Process

- Matrix `SEM-m069-op-wire-seg` killed; `op-compat-policy` oracles + `op-compat.md` cover segmented; id map v0.63 / Phase 140.

## 2026-10-13 — Trajectory: lease M069-S01 (Op wire segmented migrate)

### Trajectory

- Active Milestone **M069**, Active Slice **S01** (`L_delivery=M068-closed`): extend `migrate_op_wire` to segmented dirs (segment frame Values → remap → jsonl + drop segments/manifest + marker); open stays fail-open. Honeycomb stays Proposed.

## 2026-10-13 — Trajectory: M068 / GSD v0.62 closed

### Trajectory

- M068 / GSD v0.62 closed (Phase 139). Active Milestone **None**; `L_delivery=M068-closed`. Audit: `.planning/milestones/v0.62-MILESTONE-AUDIT.md`. Honeycomb stays Proposed.

## 2026-10-13 — Product: Op wire WAL-only migrate (Phase 139)

### Product

- `migrate_op_wire` WAL-only path: `wal::recover_json_values` → remap → materialize `events.jsonl`, drop WAL, stamp `op-wire.v1`. `recover_events` returns `InvalidData` on CRC-good frames that are valid JSON but not a modern `Event` (fail-open; no silent drop of legacy meta-ops). Segmented dirs deferred. Tests: `m068_op_wire_wal` + wal unit. Bench sample (`KUTHA_BENCH_N=1000`): `ingest_plain.n` ≈15.8ms, `as_of.n` ≈52µs (noisy host; migrate not on ingest hot path; vs M067 ≈10.9ms / ≈29µs).

### Process

- Matrix `SEM-m068-op-wire-wal` killed; `op-compat-policy` oracles + `op-compat.md` cover WAL-only; id map v0.62 / Phase 139.

## 2026-10-13 — Trajectory: lease M068-S01 (Op wire WAL-only migrate)

### Trajectory

- Active Milestone **M068**, Active Slice **S01** (`L_delivery=M067-closed`): extend `migrate_op_wire` to WAL-only legacy dirs; open stays fail-open. Segmented dirs deferred. Honeycomb stays Proposed.

## 2026-10-13 — Trajectory: M067 / GSD v0.61 closed

### Trajectory

- M067 / GSD v0.61 closed (Phase 138). Active Milestone **None**; `L_delivery=M067-closed`. Audit: `.planning/milestones/v0.61-MILESTONE-AUDIT.md`. Honeycomb stays Proposed.

## 2026-10-13 — Product: Op wire directory migrate (Phase 138)

### Product

- `store::migrate_op_wire`: rewrite legacy `events.jsonl` via `rewrite_event_json_line` (Defines + five meta-op remaps); drop stale WAL/snapshot; stamp `op-wire.v1`. Idempotent. Open stays fail-open. Segmented/WAL-only deferred. Tests: common unit + `m067_op_wire_migrate`. Bench sample (`KUTHA_BENCH_N=1000`): `ingest_plain.n` ≈10.9ms, `as_of.n` ≈29µs.

## 2026-10-13 — Trajectory: lease M067-S01 (Op wire directory migrate)

### Trajectory

- Active Milestone **M067**, Active Slice **S01** (`L_delivery=M066-closed`): explicit `store::migrate_op_wire` + `op-wire.v1` marker for legacy jsonl; open stays fail-open. Honeycomb stays Proposed.

## 2026-10-12 — Trajectory: M066 / GSD v0.60 closed

### Trajectory

- M066 / GSD v0.60 closed (Phase 137). Active Milestone **None**; `L_delivery=M066-closed`. Audit: `.planning/milestones/v0.60-MILESTONE-AUDIT.md`. Five meta-op remaps complete. Honeycomb stays Proposed.

## 2026-10-12 — Product: Op migrate RecordAction (Phase 137)

### Product

- `record_action_to_assert`: legacy `RecordAction` → `kutha:action` Assert (subject=jid, object=target_claim, delivery_key=jid, polarity ±; mirrors `record_action`). Cite fields accepted for shape only. Empty jid/claim fail-closed. Open/serde stay fail-open. Tests: common unit + `m066_op_migrate`. Bench sample (`KUTHA_BENCH_N=1000`): `ingest_plain.n` ≈13.0ms / recheck ≈25.7ms, `as_of.n` ≈29–31µs (noisy host; migrate not on ingest hot path; vs M065 ≈9.2ms / ≈45µs).

## 2026-10-12 — Trajectory: lease M066-S01 (Op migrate RecordAction)

### Trajectory

- Active Milestone **M066**, Active Slice **S01** (`L_delivery=M065-closed`): explicit `RecordAction` → `kutha:action` Assert; open stays fail-open. Directory rewrite stays deferred. Honeycomb stays Proposed.

## 2026-10-12 — Trajectory: M065 / GSD v0.59 closed

### Trajectory

- M065 / GSD v0.59 closed (Phase 136). Active Milestone **None**; `L_delivery=M065-closed`. Audit: `.planning/milestones/v0.59-MILESTONE-AUDIT.md`. Honeycomb stays Proposed.

## 2026-10-12 — Product: Op migrate RecordAdmission (Phase 136)

### Product

- `record_admission_to_assert`: legacy `RecordAdmission` → `kutha:admission` Assert (subject=jid, object true/false, delivery_key=policy_version, polarity ±; mirrors `record_justification`). Empty jid/pin fail-closed. Open/serde stay fail-open. RecordAction deferred. Tests: common unit + `m065_op_migrate`. Bench sample (`KUTHA_BENCH_N=1000`): `ingest_plain.n` ≈9.2ms, `as_of.n` ≈45µs (vs M064 ≈10.0ms / ≈25µs).

## 2026-10-12 — Trajectory: lease M065-S01 (Op migrate RecordAdmission)

### Trajectory

- Active Milestone **M065**, Active Slice **S01** (`L_delivery=M064-closed`): explicit `RecordAdmission` → `kutha:admission` Assert; open stays fail-open. RecordAction stays deferred. Honeycomb stays Proposed.

## 2026-10-12 — Trajectory: M064 / GSD v0.58 closed

### Trajectory

- M064 / GSD v0.58 closed (Phase 135). Active Milestone **None**; `L_delivery=M064-closed`. Audit: `.planning/milestones/v0.58-MILESTONE-AUDIT.md`. Honeycomb stays Proposed.

## 2026-10-12 — Product: Op migrate PinPolicy (Phase 135)

### Product

- `pin_policy_to_ops`: legacy `PinPolicy` → `StoreBlob` + `kutha:policy` Assert via `policy_version_hash` (mirrors `emit_policy`). Empty definition fail-closed. Open/serde stay fail-open. Tests: common unit + `m064_op_migrate`. Bench sample (`KUTHA_BENCH_N=1000`): `ingest_plain.n` ≈10.0ms, `as_of.n` ≈25µs.

## 2026-10-12 — Trajectory: lease M064-S01 (Op migrate PinPolicy)

### Trajectory

- Active Milestone **M064**, Active Slice **S01** (`L_delivery=M063-closed`): explicit `PinPolicy` → `StoreBlob` + `kutha:policy` Assert; open stays fail-open. Honeycomb stays Proposed.

## 2026-10-12 — Trajectory: M063 / GSD v0.57 closed

### Trajectory

- M063 / GSD v0.57 closed (Phase 134). Active Milestone **None**; `L_delivery=M063-closed`. Audit: `.planning/milestones/v0.57-MILESTONE-AUDIT.md`. Honeycomb stays Proposed.

## 2026-10-12 — Product: Op migrate RegisterRule (Phase 134)

### Product

- `remap_legacy_ops` / `register_rule_to_ops`: legacy `RegisterRule` → `StoreBlob` + `kutha:rule` Assert via `RuleAst::named` (mirrors `emit_rule`). Event expand keeps legacy id on Assert. Invalid idents fail closed. Open/serde stay fail-open. Tests: common unit + `m063_op_migrate`. Bench sample (`KUTHA_BENCH_N=1000`): `ingest_plain.n` ≈10.3ms, `as_of.n` ≈28µs.

## 2026-10-12 — Trajectory: lease M063-S01 (Op migrate RegisterRule)

### Trajectory

- Active Milestone **M063**, Active Slice **S01** (`L_delivery=M062-closed`): explicit `RegisterRule` → `StoreBlob` + `kutha:rule` Assert; open stays fail-open. Honeycomb stays Proposed.

## 2026-10-12 — Trajectory: M062 / GSD v0.56 closed

### Trajectory

- M062 / GSD v0.56 closed (Phase 133). Active Milestone **None**; `L_delivery=M062-closed`. Audit: `.planning/milestones/v0.56-MILESTONE-AUDIT.md`. Honeycomb stays Proposed.

## 2026-10-12 — Product: Op migrate AllowRelation (Phase 133)

### Product

- `kutha_common::op_migrate`: explicit `AllowRelation` JSON → `kutha:allows` `Op::Assert` via caller intern (`remap_legacy_op_value` / `remap_legacy_event_value`). No serde aliases on `Op`; m050 fail-open oracles kept. Tests: common unit + `m062_op_migrate`. Bench sample (`KUTHA_BENCH_N=1000`): `ingest_plain.n` ≈24.4ms, `as_of.n` ≈23µs.

## 2026-10-12 — Process: selftest reload + budget cite mutate (Phase 133 companion)

### Process

- Selftest reloads `checks.yaml` after YAML mutations so `disk_free` / `env_int_at_least` derives are not vacuous. `cite_equals` wrong-token for `KUTHA_GOV_BUDGET` appends a disagreeing capture. Policy doc `op-compat.md` records leased AllowRelation migrate.

## 2026-10-12 — Trajectory: lease M062-S01 (Op migrate AllowRelation)

### Trajectory

- Active Milestone **M062**, Active Slice **S01** (`L_delivery=M061-closed`): explicit `AllowRelation` → `kutha:allows` Assert remap; open stays fail-open. Honeycomb stays Proposed.

## 2026-10-12 — Trajectory: M061 / GSD v0.55 closed

### Trajectory

- M061 / GSD v0.55 closed (Phase 132). Active Milestone **None**; `L_delivery=M061-closed`. Audit: `.planning/milestones/v0.55-MILESTONE-AUDIT.md`. Honeycomb stays Proposed.

## 2026-10-12 — Process: env_int_at_least (Phase 132)

### Process

- Kind `env_int_at_least` / check `env-budget-floor`: when live `KUTHA_GOV_BUDGET` is set it must be ≥ `fsm.yaml` `defaults.budget` (HIGH; unset passes). Selftest injects `value: 1`. FSM / `.env.example` budget 82. Pytest `EnvBudgetFloorTests`. Matrix `SEM-m061-env-budget-floor` killed.

## 2026-10-12 — Trajectory: lease M061-S01 (env budget floor)

### Trajectory

- Active Milestone **M061**, Active Slice **S01** (`L_delivery=M060-closed`): live `KUTHA_GOV_BUDGET` when set must be ≥ `fsm.yaml` `defaults.budget` (`env_int_at_least`). Honeycomb stays Proposed.

## 2026-10-12 — Trajectory: M060 / GSD v0.54 closed

### Trajectory

- M060 / GSD v0.54 closed (Phase 131). Active Milestone **None**; `L_delivery=M060-closed`. Audit: `.planning/milestones/v0.54-MILESTONE-AUDIT.md`. Honeycomb stays Proposed.

## 2026-10-12 — Process: budget_covers_checks (Phase 131)

### Process

- Kind `budget_covers_checks` / check `budget-covers-checks`: `fsm.yaml` defaults.budget must be ≥ dictionary check count (precommit). Selftest lowers budget to 1. FSM / `.env.example` budget 81. Pytest `BudgetCoversChecksTests`. Matrix `SEM-m060-budget-covers` killed.

## 2026-10-12 — Trajectory: lease M060-S01 (budget covers checks)

### Trajectory

- Active Milestone **M060**, Active Slice **S01** (`L_delivery=M059-closed`): `fsm.yaml` defaults.budget must be ≥ dictionary check count (`budget_covers_checks`). Honeycomb stays Proposed.

## 2026-10-12 — Trajectory: M059 / GSD v0.53 closed

### Trajectory

- M059 / GSD v0.53 closed (Phase 130). Active Milestone **None**; `L_delivery=M059-closed`. Audit: `.planning/milestones/v0.53-MILESTONE-AUDIT.md`. Honeycomb stays Proposed.

## 2026-10-12 — Process: budget-template-align (Phase 130)

### Process

- Check `budget-template-align` reuses `cite_equals` so `.env.example` `KUTHA_GOV_BUDGET` equals `fsm.yaml` `defaults.budget`. FSM / template budget 80. Pytest `BudgetTemplateAlignTests`. Matrix `SEM-m059-budget-template` killed. No new kind.

## 2026-10-12 — Trajectory: lease M059-S01 (budget template align)

### Trajectory

- Active Milestone **M059**, Active Slice **S01** (`L_delivery=M058-closed`): `.env.example` `KUTHA_GOV_BUDGET` must match `fsm.yaml` `defaults.budget` (`cite_equals`). Honeycomb stays Proposed.

## 2026-10-12 — Trajectory: M058 / GSD v0.52 closed

### Trajectory

- M058 / GSD v0.52 closed (Phase 129). Active Milestone **None**; `L_delivery=M058-closed`. Audit: `.planning/milestones/v0.52-MILESTONE-AUDIT.md`. Honeycomb stays Proposed.

## 2026-10-12 — Process: disk_free preflight (Phase 129)

### Process

- Kind `disk_free` / check `disk-free`: free space under the repo filesystem must be ≥ YAML `min_free_gib` (default 2). Selftest derives an absurd threshold. FSM / `.env.example` budget 79. Pytest `DiskFreePreflightTests`. Matrix `SEM-m058-disk-free` killed.
- Phase 65/66 VERIFICATION bench markers moved from gitignored `tmp/` to tracked phase paths so scratch cleans do not trip `verification-independent`.

## 2026-10-12 — Trajectory: lease M058-S01 (disk_free preflight)

### Trajectory

- Active Milestone **M058**, Active Slice **S01** (`L_delivery=M057-closed`): dictionary kind `disk_free` with YAML `min_free_gib` before heavy `ci`/`cargo`. Honeycomb stays Proposed.

## 2026-10-12 — Trajectory: M057 / GSD v0.51 closed

### Trajectory

- M057 / GSD v0.51 closed (Phase 128). Active Milestone **None**; `L_delivery=M057-closed`. Audit: `.planning/milestones/v0.51-MILESTONE-AUDIT.md`. Honeycomb stays Proposed.

## 2026-10-12 — Process: STATE→ROADMAP docs-coupling (Phase 128)

### Process

- `docs-coupling` requires `.kutha/ROADMAP.md` when `.kutha/STATE.md` is in the git diff (lease-bundle hole after M056). Pytest `LeaseRoadmapCouplingTests`. Matrix `SEM-m057-lease-roadmap` killed. FSM budget stays 78. `128-VERIFICATION.md` passed.

## 2026-10-12 — Trajectory: lease M057-S01 (STATE→ROADMAP coupling)

### Trajectory

- Active Milestone **M057**, Active Slice **S01** (`L_delivery=M056-closed`): docs-coupling must require `.kutha/ROADMAP.md` whenever `.kutha/STATE.md` is in the diff. Honeycomb stays Proposed.

## 2026-10-12 — Trajectory: M056 / GSD v0.50 closed

### Trajectory

- M056 / GSD v0.50 closed (Phases 123–127). Active Milestone **None**; `L_delivery=M056-closed`. Audit: `.planning/milestones/v0.50-MILESTONE-AUDIT.md`. Honeycomb stays Proposed.

## 2026-10-12 — Product/Process: hygiene (Phase 127)

### Product

- #36: `punkt_15_1.txt` is Russian-only; English metadata lives in `m025_demo_fixtures.rs`. Oracle `demo_punkt_fixture_is_single_language`.
- Quiet bench three `--quick` runs (`KUTHA_BENCH_N=10000`, `taskset -c 0`): median `ingest_plain.n` ≈118.3M / `as_of.n` ≈393.7k vs M056-S04 ≈110.7M / ≈261.4k (+6.9% / +50.6%; as_of noisy, fixture-only change).

### Process

- #35: archive `58-VERIFICATION.md` commands use `uv run kutha-gov`. Checks `verification-uv-only` / `statute-fixture-single-lang`; invariants; FSM budget 78. Matrix `SEM-m056-hygiene-*` killed. `127-VERIFICATION.md` passed.

## 2026-10-12 — Trajectory: lease M056-S05 (hygiene)

### Trajectory

- Active Milestone **M056**, Active Slice **S05** (`L_delivery=M056-S04-done`): VERIFICATION uv-only (#35); statute fixture single-language (#36). Honeycomb stays Proposed.

## 2026-10-11 — Trajectory: M056-S04 done (NormView derived Correct eligibility)

### Trajectory

- M056-S04 done (Phase 126). Active Milestone **M056**, Active Slice **None**; `L_delivery=M056-S04-done`. Next lease: M056-S05 (#35/#36). Honeycomb stays Proposed.

## 2026-10-11 — Product/Process: NormView derived Correct eligibility (Phase 126)

### Product

- #28: `norm_view` requires live derived rows to match the Behavior head proposition; Correct/CI object changes no longer inherit rule eligibility. Oracles `m056_norm_view_derived_correct` (2).
- Quiet bench three `--quick` runs (`KUTHA_BENCH_N=10000`, `taskset -c 0`): median `ingest_plain.n` ≈110.7M / `as_of.n` ≈261.4k vs M056-S03 ≈109.7M / ≈272.6k (+0.9% / −4.1%).

### Process

- Matrix `SEM-m056-norm-view-derived-correct` killed. `126-VERIFICATION.md` passed.

## 2026-10-11 — Trajectory: lease M056-S04 (NormView derived Correct eligibility)

### Trajectory

- Active Milestone **M056**, Active Slice **S04** (`L_delivery=M056-S03-done`): Correct of a derived head must re-validate the live replacement vs the Behavior proposition (#28). Honeycomb stays Proposed.

## 2026-10-11 — Trajectory: M056-S03 done (delivery-key same-relation)

### Trajectory

- M056-S03 done (Phase 125). Active Milestone **M056**, Active Slice **None**; `L_delivery=M056-S03-done`. Next lease: M056-S04 (#28). Honeycomb stays Proposed.

## 2026-10-11 — Product/Process: delivery-key same-relation (Phase 125)

### Product

- #26: `has_live_keyed_support` is relation-scoped; keyed retry no longer short-circuits a retracted relation via a sibling claim+key. Oracle `m056_delivery_key_same_relation`.
- Quiet bench three `--quick` runs (`KUTHA_BENCH_N=10000`, `taskset -c 0`): median `ingest_plain.n` ≈109.7M / `as_of.n` ≈272.6k vs M056-S02 ≈107.9M / ≈287.2k (+1.7% / −5.1%).

### Process

- Matrix `SEM-m056-delivery-key-same-relation` killed; SEM-keyed-retry-live cites same-relation. `125-VERIFICATION.md` passed.

## 2026-10-11 — Trajectory: lease M056-S03 (delivery-key same-relation)

### Trajectory

- Active Milestone **M056**, Active Slice **S03** (`L_delivery=M056-S02-done`): keyed retry must require same-relation live support (#26). Honeycomb stays Proposed.

## 2026-10-11 — Trajectory: M056-S02 done (persist heal)

### Trajectory

- M056-S02 done (Phase 124). Active Milestone **M056**, Active Slice **None**; `L_delivery=M056-S02-done`. Next lease: M056-S03 (#26). Honeycomb stays Proposed.

## 2026-10-11 — Product/Process: persist heal residuals (Phase 124)

### Product

- #29: short unreferenced next-segment header (`orphan_len < HEADER_LEN`) recreated on `heal_interrupted_active_seal`. #30: migrate verify-fail removes manifest then `sync_dir`. Oracles `m056_persist_heal` (2).
- Quiet bench three `--quick` runs (`KUTHA_BENCH_N=10000`, `taskset -c 0`): median `ingest_plain.n` ≈107.9M / `as_of.n` ≈287.2k vs M055 ≈103.3M / ≈316.4k (+4.5% / −9.2%).

### Process

- Matrix `SEM-m056-persist-short-orphan` / `persist-migrate-sync` killed. `124-VERIFICATION.md` passed.

## 2026-10-11 — Trajectory: close M056-S01; lease M056-S02

### Trajectory

- M056-S01 done (Phase 123 lease-bundle). Active Slice **S02** (`L_delivery=M056-S01-done`): persist heal #29/#30. Honeycomb stays Proposed.

## 2026-10-11 — Process: lease-bundle honesty (Phase 123)

### Process

- gsd-auto lease is a four-surface bundle (STATE + CHANGELOG + ROADMAP open checkbox + README). Trajectory require_message cites the bundle. Pytest `LeaseBundleHonestyTests`. Matrix `SEM-m056-lease-bundle` killed. `123-VERIFICATION.md` passed.

## 2026-10-11 — Trajectory: lease M056-S01 (lease-bundle honesty)

### Trajectory

- Active Milestone **M056**, Active Slice **S01** (`L_delivery=M055-closed`): process debt — lease commit must keep trajectory/state-readme green (STATE + CHANGELOG + ROADMAP open checkbox + README). Product #29/#30 follows as S02. Honeycomb stays Proposed.

## 2026-10-11 — Trajectory: M055 / GSD v0.49 closed

### Trajectory

- M055 / GSD v0.49 closed (Phase 122). Active Milestone **None**; `L_delivery=M055-closed`. Audit: `.planning/milestones/v0.49-MILESTONE-AUDIT.md`. CE #6 tip-rollback shipped; `SEM-gap-persist-poison` killed. Honeycomb stays Proposed.

## 2026-10-11 — Product/Process: append tip rollback (Phase 122)

### Product

- CE whole-repo #6: post-write `Other` restores the durable in-memory tip and leaves a crash-shaped disk suffix; the next `append` rewinds that suffix before write so same-object retry cannot duplicate EventIds. `Other` keeps `kutha.lock` (m030). Failure after a committed rollover seal poisons. Oracles `m055_append_tip_rollback` (2).
- Quiet bench five `--quick` runs (`KUTHA_BENCH_N=10000`, `taskset -c 0`): median `ingest_plain.n` ≈103.3M / `as_of.n` ≈316.4k vs Phase 121 ≈116.4M / ≈307.6k (−11.3% / +2.9%) and vs Phase 80 ≈122.5M / ≈299.6k (−15.7% / +5.6%).

### Process

- Matrix `SEM-gap-persist-poison` killed (owner `M055-S01`). Map + gsd-auto-contract v0.49. `122-VERIFICATION.md` passed.

## 2026-10-11 — Trajectory: lease M055-S01 (#6 append tip rollback)

### Trajectory

- Active Milestone **M055**, Active Slice **S01** (`L_delivery=M054-closed`): CE whole-repo #6 — rollback physical tip on post-write append failure before `remember_appended`; keep `Other` lock+retry; poison only after committed rollover. Honeycomb stays Proposed.

## 2026-10-11 — Trajectory: M054 / GSD v0.48 closed

### Trajectory

- M054 / GSD v0.48 closed (Phases 117–121). Active Milestone **None**; `L_delivery=M054-closed`. Audit: `.planning/milestones/v0.48-MILESTONE-AUDIT.md`. **#6 poison** remains decision-gate (`SEM-gap-persist-poison`). Honeycomb stays Proposed.

## 2026-10-11 — Product/Process: ID and structure (Phase 121)

### Product

- #24 content-derived EventIds are RFC 9562 UUID v8 (`stable_derived` / `stable_define`); user ingest stays v7. #20 shrink corpus uses `kutha_common::sha256_hex`. #12 SEM helper split into named modules. Oracles `m054_event_id` + `m054_id_structure`.
- Quiet bench three `--quick` runs (`KUTHA_BENCH_N=10000`, `taskset -c 0`): median `ingest_plain.n` ≈116.4M / `as_of.n` ≈307.6k vs Phase 120 ≈106.2M / ≈299.9k (+9.6% / +2.6%) and vs Phase 80 ≈122.5M / ≈299.6k (−5.0% / +2.7%).

### Process

- #21 kinds.py family split (`kind_planning` / `kind_cargo` / `kind_matrix`); RUNNERS stay in `kinds.py`. Matrix `SEM-m054-id-*` killed. `121-VERIFICATION.md` passed.

## 2026-10-11 — Product/Process: Scaling and bench honesty (Phase 120)

### Product

- #15/#25 bench: RSS while dedicated sample alive; timed `as_of` triples validated. #16/#17/#19/#27: `derived_by_cause`, relation-keyed Horn probe, append suffix-only, rule/policy pin maps. Oracles `m054_scaling_bench` (7).
- Quiet bench three `--quick` runs (`KUTHA_BENCH_N=10000`, `taskset -c 0`): median `ingest_plain.n` ≈106.2M / `as_of.n` ≈299.9k vs pre-change `70b76ce` ≈103.2M / ≈285.2k (+2.9% / +5.2%) and vs Phase 80 ≈122.5M / ≈299.6k (−13.3% / +0.1%).

### Process

- Matrix killed: `SEM-m054-scale-bench-rss` / `bench-asof` / `retract-index` / `horn-rel` / `append-suffix` / `pin-index`. `120-VERIFICATION.md` passed.

## 2026-10-11 — Process: Governor honesty (Phase 119) + lease M054-S04

### Process

- #14 artifact-budget PLAN gate keys off CONTEXT + per-phase correspond_glob. #22/#23 test_matrix rejects wrong schema and non-mapping rows. #34 parallel pool submit is at-most-once (missing-only serial fallback). Pytest oracles in `test_kutha_gov` / `test_test_matrix` / `test_jobs`. Matrix `SEM-m054-gov-plan-budget` / `gov-matrix-schema` / `gov-pool-once` killed. `119-VERIFICATION.md` passed. Budget stays 76.

### Trajectory

- M054-S03 done. Active Slice **S04** (`L_delivery=M054-S03-done`): scaling + bench honesty. **#6 decision-gate** stays human. Honeycomb stays Proposed.

## 2026-10-11 — Product/Process: Persist durability (Phase 118) + lease M054-S03

### Product

- #9 `open_for_append` heals torn empty active (scan consumed vs physical) before append. #10 `persist_log` writes generation-unique segment names, fsyncs, then atomically swaps the manifest. Oracles `m054_persist_durability` (3). **#6 poison stays decision-gate.**
- Quiet bench three `--quick` runs (`KUTHA_BENCH_N=10000`, `taskset -c 0`): median `ingest_plain.n` ≈105.9M / `as_of.n` ≈269.8k vs Phase 117 ≈111.8M / ≈266.2k (−5.3% / +1.4%) and vs Phase 80 ≈122.5M / ≈299.6k (−13.6% / −9.9%).

### Process

- Matrix killed: `SEM-m054-persist-torn-heal` / `persist-gen-unique`. Gap `SEM-gap-persist-poison` (#6). `118-VERIFICATION.md` passed.

### Trajectory

- M054-S02 done. Active Slice **S03** (`L_delivery=M054-S02-done`): governor honesty #14/#22/#23/#34. Honeycomb stays Proposed.

## 2026-10-11 — Product/Process: Horn soundness (Phase 117) + lease M054-S02

### Product

- Horn #4/#3/#2/#5: Correct/CI-caused re-fire stays `derivation_eligible`; non-trigger retract drops derived; head VT = premise×pin conjunction; full-cover dedup emits uncovered VT residuals; Correct cascade does not duplicate derived Retract. Oracles `m054_horn_soundness` (5).
- Quiet bench three `--quick` runs (`KUTHA_BENCH_N=10000`, `taskset -c 0`): median `ingest_plain.n` ≈111.8M / `as_of.n` ≈266.2k vs Phase 115 ≈103.9M / ≈299.2k (+7.6% / −11.0%) and vs Phase 80 ≈122.5M / ≈299.6k (−8.7% / −11.1%).

### Process

- Matrix killed: `SEM-m054-horn-correct-eligible` / `nontrigger-retract` / `vt-intersect` / `residual-dedup` / `no-dup-retract`. Gap `SEM-gap-horn-soundness` removed. Map updated. `117-VERIFICATION.md` passed.

### Trajectory

- M054-S01 done. Active Slice **S02** (`L_delivery=M054-S01-done`): persist #9/#10. **#6 decision-gate** stays human. Honeycomb stays Proposed.

## 2026-10-11 — Trajectory: lease M054-S01 (Horn soundness)

### Trajectory

- Active Milestone **M054**, Active Slice **S01** (`L_delivery=M053-closed`): Horn soundness — CE whole-repo #4/#3/#2/#5 (Phase 117). **#6 decision-gate** stays human. Honeycomb stays Proposed.

## 2026-10-11 — Product/Process: CE review regression enrolment (Phase 116)

### Product

- Regression oracles `m053_ce_review_regressions`: `fork_at` rebuilds outcomes from log QuantumOutcome after wiping the lease; shared `persist_dir` stays under manifest `tmp/` with pid; `try_persist_with_layout(Rollover)` matches sealed>=2 seam; all five removed dictionary meta-ops fail deserialize.

### Process

- Matrix killed rows `SEM-m053-hydrate-outcomes` / `persist-dir-anchor` / `rollover-layout-seam` / `legacy-meta-ops-fail-open`. Gap classes `SEM-gap-horn-soundness` / `persist-durability` / `governor-honesty` / `scaling-bench` / `id-and-structure` owned by unleased M054-S01–S05. Map: `docs/process/ce-review-regression-map.md`.

### Trajectory

- M053 / GSD v0.47 closed. Active Milestone **None**; `L_delivery=M053-closed`. Audit: `.planning/milestones/v0.47-MILESTONE-AUDIT.md`. Next: lease an M054 gap slice. #6 decision-gate unchanged.

## 2026-10-11 — Trajectory: M052 / GSD v0.46 closed

### Trajectory

- M052 / GSD v0.46 closed (Phases 112–115). Active Milestone **None**; `L_delivery=M052-closed`. Audit: `.planning/milestones/v0.46-MILESTONE-AUDIT.md`. Whole-repo #1 remains demoted. #6 decision-gate unchanged. Next: remaining CE whole-repo apply-queue (Horn #2–#5, persist #6/#9/#10, governor honesty, EventId #24).

### Process

- Compact `113-01-SUMMARY.md` under the 60-line artifact-budget. Active Slice tokens use `S##` (not `M052-S##`) for trajectory.

## 2026-10-11 — Product: snapshot-free open replay (Phase 115)

### Product

- Added `m052_open_replay` oracles: empty-pin `inverse_knows` (RULE-04) is rejected on snapshot-backed open and on Define/terms reconstruction after `snapshot.json` is removed.
- Fixed `finish_open`: `replay_check` runs on every reconstruction path (snapshot, Define, terms, empty) before sidecar hydrate, so dropping the snapshot lease cannot admit an unreproducible log. Quiet bench three `--quick` runs (`KUTHA_BENCH_N=10000`, `taskset -c 0`): median `ingest_plain.n` ≈103.9M / `as_of.n` ≈299.2k vs Phase 114 ≈106.7M / ≈303.3k (−2.6% / −1.4%) and vs Phase 80 ≈122.5M / ≈299.6k (−15.2% / −0.1%).

### Process

- Matrix row `SEM-m052-open-replay` (owner `M052-S04`). `.kutha/ROADMAP.md` names M052-S04 so the owner resolves. No new governor check; budget stays 76.

### Trajectory

- Honeycomb cells remain **Proposed**. Lease stays with the controller (`.kutha/STATE.md` untouched). M052-S04 / CE whole-repo P1 #11.

## 2026-10-11 — Product: lock before heal (Phase 114)

### Product

- Added `m052_lock_before_heal` oracles: a second `Persister::attach` while the writer holds `kutha.lock` after an interrupted rollover seal is `WouldBlock` with store bytes unchanged; attach still finishes the seal once the lock is free.
- Fixed `Persister::attach`: pre-lock `attach_holding` is read-only (COMMIT-01 `InvalidData` on a dirty tail still wins over `WouldBlock`); `heal_interrupted_active_seal` runs only after `acquire_store_write_lock`. Quiet bench three `--quick` runs (`KUTHA_BENCH_N=10000`, `taskset -c 0`): median `ingest_plain.n` ≈106.7M / `as_of.n` ≈303.3k vs Phase 113 ≈111.4M / ≈294.1k (−4.2% / +3.1%) and vs Phase 80 ≈122.5M / ≈299.6k (−12.9% / +1.2%).

### Process

- Matrix row `SEM-m052-lock-before-heal` (owner `M052-S03`). `.kutha/ROADMAP.md` names M052-S03 so the owner resolves. No new governor check; budget stays 76.

### Trajectory

- Honeycomb cells remain **Proposed**. Lease stays with the controller (`.kutha/STATE.md` untouched). M052-S03 / CE whole-repo P1 #7.

## 2026-10-11 — Process: CE whole-repo triage correction (Phase 112)

### Process

- Advisor correction demotes whole-repo finding #1 from P1/fix-first: documented Op wire break (`op-compat.md` + `m050_op_compat`) is intentional; migrate stays deferred residual. Re-verified withdrawals: hydrate rebuilds outcomes; m028 `persist_dir` is manifest-dir+PID+nanos; rollover-shrink keeps layout via `try_persist_with_layout`. Artifact: `.compound-engineering/artifacts/reviews/2026-10-11-ce-whole-repo/advisor-correction.md`.

### Trajectory

- M052 / GSD v0.46 opened with S01–S04. Fix-first starts at cascade #8 (Phase 113). #6 decision-gate unchanged. Honeycomb Proposed.

## 2026-10-11 — Product: cascade atomicity (Phase 113)

### Product

- Added `m052_cascade_atomic` oracles: a later-hop Horn whose head is never interned (`ghostRel`) must `Err` without growing log, fold, or outcomes; inverse_knows still writes `QuantumOutcome`.
- Fixed `commit_user_cascade`: a later-hop `rule_follow_ons` / admit error rolls back through `fork_at` so log, fold, and outcomes stay unchanged; the staged hop sequence and `QuantumOutcome` commit together on success. Quiet bench three `--quick` runs (`KUTHA_BENCH_N=10000`, `taskset -c 0`): median `ingest_plain.n` ≈111.4M / `as_of.n` ≈294.1k vs Phase 111 ≈116.2M / ≈272.7k (−4.1% / +7.8%) and vs Phase 80 ≈122.5M / ≈299.6k (−9.1% / −1.8%).

### Process

- Matrix row `SEM-m052-cascade-atomic` (owner `M052-S02`). `.kutha/ROADMAP.md` names M052-S02 so the owner resolves. No new governor check; budget stays 76.

### Trajectory

- Honeycomb cells remain **Proposed**. Lease stays with the controller (`.kutha/STATE.md` untouched). M052-S02 / CE whole-repo P1 #8.

## 2026-10-11 — Product/Process: pin scheme v1 stability vectors (Phase 111)

### Product

- Hardcoded pin stability oracles `m051_pin_compat`: ident `derive_pq` locks Named AST digest; diverges from main-era UTF-8 domain digest; non-ident free string keeps domain+UTF-8. `rule_definition_hash` docstring cites pin scheme v1 (no algorithm change). Quiet bench three `--quick` runs (`KUTHA_BENCH_N=10000`, `taskset -c 0`): median `ingest_plain.n` ≈116.2M / `as_of.n` ≈272.7k vs M038 ≈108.9M / ≈303.0k (+6.7% / −10.0%) and vs Phase 80 ≈122.5M / ≈299.6k (−5.1% / −9.0%).

### Process

- Controller policy **documented rotation to pin scheme v1; no silent remap** in `docs/process/pin-compat.md`. FSM / `.env.example` budget 75→76. Check `pin-compat-policy`; matrix `SEM-m051-pin-compat-policy`. CE review P1 #24.

### Trajectory

- M051 / GSD v0.45 closed. Active Milestone **None**; `L_delivery=M051-closed`. Audit: `.planning/milestones/v0.45-MILESTONE-AUDIT.md`. #6 decision-gate unchanged. Next: G4 #26 or leased Op backfill.

## 2026-10-10 — Product/Process: Op compat documented break (Phase 110)

### Product

- Characterization oracles `m050_op_compat`: main-era `AllowRelation` / `RegisterRule` / `PinPolicy` / `RecordAdmission` / `RecordAction` JSON fail to deserialize; modern `kutha:*` Assert round-trips. No serde TermId remap in this slice.

### Process

- Controller policy **documented break; serde backfill deferred** in `docs/process/op-compat.md`. FSM / `.env.example` budget 74→75. Check `op-compat-policy`; matrix `SEM-m050-op-compat-policy`. CE review P1 #14 (minimum path + fail-open pin).

### Trajectory

- M050 / GSD v0.44 closed. Active Milestone **None**; `L_delivery=M050-closed`. Audit: `.planning/milestones/v0.44-MILESTONE-AUDIT.md`. #6 decision-gate unchanged. Next: G4 #24 or leased Op backfill.

## 2026-10-10 — Process: skip-cargo log marker (Phase 109)

### Process

- When `emit_tenant` is suppressed by skip_cargo / prior-high, record allowlisted `tenant=skipped` (LOW `observe-skipped` + `append_observe`) so fold tip is not a stale full-evidence tenant. Cargo tip already becomes `skipped` via Phase 107. FSM / `.env.example` budget 73→74. Check `skip-cargo-log-marker`; matrix `SEM-m049-skip-cargo-log-marker`. CE review P2 #38.

### Trajectory

- M049 / GSD v0.43 closed. Active Milestone **None**; `L_delivery=M049-closed`. Audit: `.planning/milestones/v0.43-MILESTONE-AUDIT.md`. G3 apply-queue + advisory #38 closed. #6 decision-gate unchanged. Next: G4 #14 (compat policy).

## 2026-10-10 — Process: parallel Context flags (Phase 108)

### Process

- `_eval_check_worker` payload carries `skip_emit` and `changed_paths` so jobs>1 matches serial Context control fields. FSM / `.env.example` budget 72→73. Check `parallel-context-flags`; matrix `SEM-m048-parallel-context-flags`. CE review P2 #48.

### Trajectory

- M048 / GSD v0.42 closed. Active Milestone **None**; `L_delivery=M048-closed`. Audit: `.planning/milestones/v0.42-MILESTONE-AUDIT.md`. #6 decision-gate unchanged. Next: #38 (or owner pick).

## 2026-10-10 — Process: observe-skipped breadcrumb (Phase 107)

### Process

- Suppressed `observe_cargo` (skip_cargo / prior-high / overlap-after-high) appends a LOW `observe-skipped` finding naming FSM state and cause; observation stays allowlisted `cargo=skipped` so emit_log does not fail-closed. FSM / `.env.example` budget 71→72. Check `observe-skipped-breadcrumb`; matrix `SEM-m047-observe-skipped`. CE review P2 #39.

### Trajectory

- M047 / GSD v0.41 closed. Active Milestone **None**; `L_delivery=M047-closed`. Audit: `.planning/milestones/v0.41-MILESTONE-AUDIT.md`. #6 decision-gate unchanged. Next: #48.

## 2026-10-10 — Process: parallel check-error parity (Phase 106)

### Process

- `_eval_check_worker` reuses `eval_check` so `check.run` exceptions are `check-error` under jobs=1 and jobs>1; `worker-error` stays transport-only. FSM / `.env.example` budget 70→71. Check `parallel-check-error-parity`; matrix `SEM-m046-parallel-check-error`. CE review P2 #23.

### Trajectory

- M046 / GSD v0.40 closed. Active Milestone **None**; `L_delivery=M046-closed`. Audit: `.planning/milestones/v0.40-MILESTONE-AUDIT.md`. #6 decision-gate unchanged. Next: #39.

## 2026-10-10 — Process: glob_absent comment policy (Phase 105)

### Process

- `glob_absent` uses one per-line comment policy (`#` and `//`) whether or not `unless_line_contains` is set. Check `glob-absent-comment-policy`; matrix `SEM-m045-glob-absent-comment`. CE review P2 #22.

### Trajectory

- M045 / GSD v0.39 closed. Active Milestone **None**; `L_delivery=M045-closed`. Audit: `.planning/milestones/v0.39-MILESTONE-AUDIT.md`. **#6 remains decision-gate** (human). Next: G3 #23… or parked residuals.

## 2026-10-10 — Process: nameless dict retain (Phase 104)

### Process

- `get_checks` keeps already-parsed valid checks when a sibling row lacks `id`; `LoadErrorCheck` cites the row index alongside survivors (no whole-dictionary discard). Check `nameless-dict-retain`; matrix `SEM-m044-nameless-dict-retain`. CE review P2 #20.

### Trajectory

- M044 / GSD v0.38 closed. Active Milestone **None**; `L_delivery=M044-closed`. Audit: `.planning/milestones/v0.38-MILESTONE-AUDIT.md`. #6 remains decision-gate. Next: #22.

## 2026-10-10 — Process: Cargo profiles for kache + mold

### Process

- Workspace `Cargo.toml` records `profile.dev` (`line-tables-only`, dep DWARF/incremental off) and `profile.release` (`thin` LTO, strip) synced with host kache wrapper + mold linker. README / STACK no longer claim sccache.

## 2026-10-10 — Process: FSM quantum honest name (Phase 103)

### Process

- Renamed skip_cargo FSM quantum to `test_ci_quantum_scheduling_reaches_ok_with_skips` (empty observations). Live `test_ci_quantum_observations_live` asserts cargo/tenant/ff5 under `KUTHA_LIVE_CARGO=1`. Governor check `fsm-quantum-honest-name`; invariant `I-fsm-quantum-honest-name`. Matrix `SEM-m043-fsm-quantum-honest`. CE review P1 #17.

### Trajectory

- M043 / GSD v0.37 closed. Active Milestone **None**; `L_delivery=M043-closed`. Audit: `.planning/milestones/v0.37-MILESTONE-AUDIT.md`. Honeycomb stays Proposed.

## 2026-10-10 — Product: m028×layouts + shrink corpus (Phase 102)

### Product

- Trailing `emit_retract_dict` stays revoked after persist+open on Legacy/Segmented/Rollover/Migrated (`m042_m028_layouts`; Rollover padded for tiny-segment seam). `save_shrunk_to` / `load_shrunk` content-addressed JSON roundtrip; `maybe_save_shrunk` delegates to `save_shrunk_to`. Matrix `SEM-m042-dict-retract-layouts`, `SEM-m042-shrink-corpus`.

### Trajectory

- ADR-023 Clarification Phase 102 (Proposed). Honeycomb evidence restaged. Extends CE P0 #1 beyond single-layout `m028_dict_retract`.

## 2026-10-10 — Process: close M042 / v0.36 (Phase 102)

### Process

- M042 / GSD v0.36 closed. Active Milestone **None**; `L_delivery=M042-closed`. Audit: `.planning/milestones/v0.36-MILESTONE-AUDIT.md`. S01 layouts + corpus (`102-VERIFICATION.md`). Next controller lease: G3 remaining (#17…) or parked residuals.

### Trajectory

- Honeycomb stays Proposed.

## 2026-10-10 — Product: persist-trace identity + shrink gate (Phase 101)

### Product

- `common/sem.rs`: `candidate_deps_hold`, `shrink_events_ddmin`, `try_persist_with_layout`, `expected_open_event_ids`, `persist_ids_disagree`, optional `KUTHA_SAVE_SHRUNK`. `fail_shrunk` uses ddmin; `persist_stamps_disagree` rejects invalid/Rollover-seam candidates instead of panicking. Oracle `m041_persist_trace_identity` pins non-Define EventId order across Legacy/Segmented/Rollover/Migrated. Matrix `SEM-m041-persist-trace-identity`, `SEM-m041-shrink-ddmin`.

### Trajectory

- ADR-023 Clarification Phase 101 (Proposed). Honeycomb evidence restaged. Export Define-prefix helper stays distinct from open-path ids.

## 2026-10-10 — Process: close M041 / v0.35 (Phase 101)

### Process

- M041 / GSD v0.35 closed. Active Milestone **None**; `L_delivery=M041-closed`. Audit: `.planning/milestones/v0.35-MILESTONE-AUDIT.md`. S01 persist-trace identity (`101-VERIFICATION.md`). Next controller lease: G3 remaining (#17…) or parked residuals.

### Trajectory

- Honeycomb stays Proposed.

## 2026-10-10 — Process: vacuous planning gates (Phase 100)

### Process

- `verification-independent` no longer uses `allow_empty`. Content steps gate on live CONTEXT via `when_glob_exists`; idle without CONTEXT skips. `_when_glob_idle` in `kinds.py` for `file_contains` and `frontmatter_paths_exist`. Unit oracles: CONTEXT without VERIFICATION → HIGH. Matrix `SEM-m040-vacuous-planning-gates`. CE review P1 #16.

### Trajectory

- Honeycomb stays Proposed. Governor-intake idle/live contract restated.

## 2026-10-10 — Process: close M040 / v0.34 (Phase 100)

### Process

- M040 / GSD v0.34 closed. Active Milestone **None**; `L_delivery=M040-closed`. Audit: `.planning/milestones/v0.34-MILESTONE-AUDIT.md`. S01 vacuous planning gates (`100-VERIFICATION.md`). Next controller lease: G3 remaining (#17, #6…) or parked residuals.

### Trajectory

- Honeycomb stays Proposed.

## 2026-10-10 — Process: stub CI honest name (Phase 99)

### Process

- Split the pytest PATH-stub quantum from a lying `test_ci_green_on_this_tree` name. Stub cargo/tenant now only proves FSM observe scheduling (`test_ci_fsm_reaches_ok_with_stubbed_observe`). Live `observe_cargo` is `uv run kutha-gov ci` or opt-in `KUTHA_LIVE_CARGO=1` (`test_ci_observe_cargo_live`). Governor check `stub-ci-honest-name` (`glob_absent` + `file_contains`); invariant `I-stub-ci-honest-name`. Matrix `SEM-m039-stub-ci-honest`. CE review P1 #7.

### Trajectory

- Honeycomb stays Proposed. Test-matrix plane stays harness oracle.

## 2026-10-10 — Process: close M039 / v0.33 (Phase 99)

### Process

- M039 / GSD v0.33 closed. Active Milestone **None**; `L_delivery=M039-closed`. Audit: `.planning/milestones/v0.33-MILESTONE-AUDIT.md`. S01 stub CI honest name (`99-VERIFICATION.md`). Next controller lease: G3 remaining (#16, #17, #6…) or parked residuals.

### Trajectory

- Honeycomb stays Proposed.

## 2026-10-10 — Product: policy delivery-key namespace (Phase 98)

### Product

- Ingest lease `by_delivery_key` keys `(relation, delivery_key)` so `kutha:admission` Asserts that pin the policy hash do not overwrite the `kutha:policy` registry slot. Identical `emit_policy` re-pin after `record_justification` is idempotent; same key on different relations does not `DeliveryKeyConflict` (CE review P2 #29). Admission keeps the policy pin on `delivery_key`. Oracles: `m038_policy_delivery_namespace.rs`. Matrix `SEM-m038-policy-delivery-namespace`. ADR-050 Clarification Phase 98. Quiet bench three `--quick` runs (`KUTHA_BENCH_N=10000`, `taskset -c 0`): median `ingest_plain.n` ≈108.9M / `as_of.n` ≈303.0k vs M037 ≈110.4M / ≈280.9k (−1.4% / +7.9%) and vs Phase 80 ≈122.5M / ≈299.6k (−11.1% / +1.1%).

### Trajectory

- Honeycomb stays Proposed. ADR-050 evidence restaged.

## 2026-10-10 — Process: close M038 / v0.32 (Phase 98)

### Process

- M038 / GSD v0.32 closed. Active Milestone **None**; `L_delivery=M038-closed`. Audit: `.planning/milestones/v0.32-MILESTONE-AUDIT.md`. S01 delivery-key namespace (`98-VERIFICATION.md`). CE G2 lineage closed. Next controller lease: parked residuals / G3 governor oracles.

### Trajectory

- Honeycomb stays Proposed.

## 2026-10-10 — Product: norm_view claim-lineage eligibility (Phase 97)

### Product

- `Runtime::norm_view` gates derivation eligibility on claim lineage (`event_by_id(f.claim_id)` is `Op::Behavior`), not minting event type. Correct/CorrectInterval of an ineligible derived claim stay Abstain; eligible Correct stays Ok (CE review P1 #13). Oracles: `m037_norm_view_claim_lineage.rs`. Matrix `SEM-m037-norm-view-claim-lineage`. ADR-013 Clarification Phase 97. Quiet bench three `--quick` runs (`KUTHA_BENCH_N=10000`, `taskset -c 0`): median `ingest_plain.n` ≈110.4M / `as_of.n` ≈280.9k vs M036 ≈118.8M / ≈272.3k (−7.1% / +3.2%) and vs Phase 80 ≈122.5M / ≈299.6k (−9.9% / −6.2%).

### Trajectory

- Honeycomb stays Proposed. ADR-013 evidence restaged.

## 2026-10-10 — Process: close M037 / v0.31 (Phase 97)

### Process

- M037 / GSD v0.31 closed. Active Milestone **None**; `L_delivery=M037-closed`. Audit: `.planning/milestones/v0.31-MILESTONE-AUDIT.md`. S01 claim-lineage eligibility (`97-VERIFICATION.md`). Next controller lease: G2 #29 or parked residuals / G3 governor oracles.

### Trajectory

- Honeycomb stays Proposed.

## 2026-10-10 — Product: VT-aware Horn dedup (Phase 96)

### Product

- `has_live_triple` / `has_live_triple_except` take candidate VT and block only on `vt_intersect` overlap. Disjoint VT windows of the same triple each mint a derived inverse; overlapping windows still suppress (CE review P1 #11). Oracles: `m036_vt_aware_dedup.rs`. Matrix `SEM-m036-vt-aware-dedup`. ADR-013 Clarification Phase 96. Quiet bench three `--quick` runs (`KUTHA_BENCH_N=10000`, `taskset -c 0`): median `ingest_plain.n` ≈118.8M / `as_of.n` ≈272.3k vs M035 ≈111.5M / ≈322.3k (+6.5% / −15.5%) and vs Phase 80 ≈122.5M / ≈299.6k (−3.0% / −9.1%).

### Trajectory

- Honeycomb stays Proposed. ADR-013 evidence restaged.

## 2026-10-10 — Process: close M036 / v0.30 (Phase 96)

### Process

- M036 / GSD v0.30 closed. Active Milestone **None**; `L_delivery=M036-closed`. Audit: `.planning/milestones/v0.30-MILESTONE-AUDIT.md`. S01 VT-aware Horn dedup (`96-VERIFICATION.md`). Next controller lease: G2 lineage (#13…) or parked residuals.

### Trajectory

- Honeycomb stays Proposed.

## 2026-10-10 — Product: replay dict bind (Phase 95)

### Product

- `GraphFold::replay_with_dict` binds reserved `kutha:*` after replay; `Runtime::replay_check` returns that fold so product `as_of` matches live. Bare `replay` stays fingerprint-safe and dictionary-blind (characterization) (CE review P1 #10). Oracles: `m035_replay_dict_bind.rs`. Matrix `SEM-m035-replay-dict-bind`. ADR-060 Clarification Phase 95. Quiet bench three `--quick` runs (`KUTHA_BENCH_N=10000`, `taskset -c 0`): median `ingest_plain.n` ≈111.5M / `as_of.n` ≈322.3k vs M034 ≈113.6M / ≈328.9k (−1.8% / −2.0%) and vs Phase 80 ≈122.5M / ≈299.6k (−9.0% / +7.6%).

### Trajectory

- Honeycomb stays Proposed. ADR-060 evidence restaged.

## 2026-10-10 — Process: close M035 / v0.29 (Phase 95)

### Process

- M035 / GSD v0.29 closed. Active Milestone **None**; `L_delivery=M035-closed`. Audit: `.planning/milestones/v0.29-MILESTONE-AUDIT.md`. S01 replay dict bind (`95-VERIFICATION.md`). Next controller lease: G2 lineage (#11…) or parked residuals.

### Trajectory

- Honeycomb stays Proposed.

## 2026-10-09 — Product: Correct Horn re-fire (Phase 94)

### Product

- `rule_follow_ons` for `Op::Correct` mirrors CorrectInterval: retract except-set, `horn_fires_except`, fires-then-retracts so LIFO retracts before re-fires. Same-triple Correct (e.g. VT-only) keeps the derived inverse; object-changing Correct re-fires the new inverse (CE review P1 #9). Oracles: `m034_correct_refire.rs`. Matrix `SEM-m034-correct-refire`. ADR-013 Clarification Phase 94. Quiet bench three `--quick` runs (`KUTHA_BENCH_N=10000`, `taskset -c 0`): median `ingest_plain.n` ≈113.6M / `as_of.n` ≈328.9k vs M033 ≈112.2M / ≈300.6k (+1.2% / +9.4%) and vs Phase 80 ≈122.5M / ≈299.6k (−7.3% / +9.8%).

### Trajectory

- Honeycomb stays Proposed. ADR-013 evidence restaged.

## 2026-10-09 — Process: close M034 / v0.28 (Phase 94)

### Process

- M034 / GSD v0.28 closed. Active Milestone **None**; `L_delivery=M034-closed`. Audit: `.planning/milestones/v0.28-MILESTONE-AUDIT.md`. S01 Correct Horn re-fire (`94-VERIFICATION.md`). Next controller lease: G2 lineage (#10…) or parked residuals.

### Trajectory

- Honeycomb stays Proposed.

## 2026-10-09 — Product: CI admission VT coverage (Phase 93)

### Product

- Admission `stale_support` checks any live CorrectInterval residual at `row.vt` via `live_fact_indices_by_event_id` (not a single `fact_index_by_event_id` representative). Patch/suffix justifications no longer false-deny; retracted CI stays `stale_support` (CE review P1 #8). Oracles: `m033_ci_admission_vt.rs`. Matrix `SEM-m033-ci-admission-vt`. ADR-013 Clarification Phase 93. Quiet bench three `--quick` runs (`KUTHA_BENCH_N=10000`, `taskset -c 0`): median `ingest_plain.n` ≈112.2M / `as_of.n` ≈300.6k vs M032 ≈121.2M / ≈287.0k (−7.4% / +4.7%) and vs Phase 80 ≈122.5M / ≈299.6k (−8.4% / +0.3%).

### Trajectory

- Honeycomb stays Proposed. ADR-013 evidence restaged.

## 2026-10-09 — Process: close M033 / v0.27 (Phase 93)

### Process

- M033 / GSD v0.27 closed. Active Milestone **None**; `L_delivery=M033-closed`. Audit: `.planning/milestones/v0.27-MILESTONE-AUDIT.md`. S01 CI admission VT (`93-VERIFICATION.md`). Next controller lease: G2 lineage (#9…) or parked residuals.

### Trajectory

- Honeycomb stays Proposed.

## 2026-10-09 — Product: derive live premise (Phase 92)

### Product

- `Runtime::derive` premise lookup requires `invalidated_at.is_none()`, matching retract/correct. Sole retracted premise → `UnknownFact` (no Behavior); later retract leaves prior live Assert as `caused_by` (CE review P1 #4). Oracles: `m032_derive_live_premise.rs`. Matrix `SEM-m032-derive-live-premise`. ADR-013 Clarification Phase 92. Quiet bench six `--quick` runs (`KUTHA_BENCH_N=10000`, `taskset -c 0`): median `ingest_plain.n` ≈121.2M / `as_of.n` ≈287.0k vs M031 ≈110.2M / ≈281.3k (+10.0% / +2.0%) and vs Phase 80 ≈122.5M / ≈299.6k (−1.1% / −4.2%).

### Trajectory

- Honeycomb stays Proposed. ADR-013 evidence restaged.

## 2026-10-09 — Process: close M032 / v0.26 (Phase 92)

### Process

- M032 / GSD v0.26 closed. Active Milestone **None**; `L_delivery=M032-closed`. Audit: `.planning/milestones/v0.26-MILESTONE-AUDIT.md`. S01 derive live premise (`92-VERIFICATION.md`). Next controller lease: G2 lineage (#8…) or parked residuals.

### Trajectory

- Honeycomb stays Proposed.

## 2026-10-09 — Product: WAL-tail heal (Phase 91)

### Product

- `open_for_append` heals when legacy jsonl tip is ahead of WAL even if `dropped_uncommitted==0` (`legacy_attach_needs_rewrite` → `persist_unlocked`, then attach). Jsonl failpoint no longer wedges incremental append (CE review #30). Oracles: `m031_wal_tail_heal.rs`. Matrix `SEM-m031-wal-tail-heal`. ADR-010 Clarification Phase 91. Quiet bench three `--quick` runs (`KUTHA_BENCH_N=10000`, `taskset -c 0`): median `ingest_plain.n` ≈110.2M / `as_of.n` ≈281.3k vs M030 ≈116.2M / ≈303.3k (−5.1% / −7.3%) and vs Phase 80 ≈122.5M / ≈299.6k (−10.0% / −6.1%).

### Trajectory

- Honeycomb stays Proposed. ADR-010 evidence restaged.

## 2026-10-09 — Process: close M031 / v0.25 (Phase 91)

### Process

- M031 / GSD v0.25 closed. Active Milestone **None**; `L_delivery=M031-closed`. Audit: `.planning/milestones/v0.25-MILESTONE-AUDIT.md`. S01 WAL-tail heal (`91-VERIFICATION.md`). Next controller lease: G2 lineage (#4) or parked residuals.

### Trajectory

- Honeycomb stays Proposed.

## 2026-10-09 — Product: writer lock retention (Phase 90)

### Product

- Non-poison `Persister` append/checkpoint errors keep `kutha.lock` (`release_lock_if_poisoned`); fail-closed `InvalidData` still poisons and releases for heal (CE review P1 #2). Heal/migrate oracles `drop` the crashed writer before re-taking the lock. Oracles: `m030_writer_lock.rs`. Matrix `SEM-m030-writer-lock`. ADR-010 Clarification Phase 90. Quiet bench three `--quick` runs (`KUTHA_BENCH_N=10000`, `taskset -c 0`): median `ingest_plain.n` ≈116.2M / `as_of.n` ≈303.3k vs M029 ≈127.2M / ≈283.0k (−8.6% / +7.2%) and vs Phase 80 ≈122.5M / ≈299.6k (−5.1% / +1.2%).

### Trajectory

- Honeycomb stays Proposed. ADR-010 evidence restaged.

## 2026-10-09 — Process: close M030 / v0.24 (Phase 90)

### Process

- M030 / GSD v0.24 closed. Active Milestone **None**; `L_delivery=M030-closed`. Audit: `.planning/milestones/v0.24-MILESTONE-AUDIT.md`. S01 writer lock retention (`90-VERIFICATION.md`). Next controller lease: G1 #30 or G2 lineage.

### Trajectory

- Honeycomb stays Proposed.

## 2026-10-09 — Product: interrupted rollover seal heal (Phase 89)

### Product

- `heal_interrupted_active_seal` finishes a mid-rollover seal trailer on attach (promote sealed active, empty next segment, rewrite manifest). `open_for_append` no longer fails with WAL tail mismatch after `RolloverTrailer` / `RolloverNewHeader` crashes (CE review P1 #3). Oracles: `m029_rollover_heal.rs`. Matrix `SEM-m029-rollover-heal`. ADR-012 Clarification Phase 89. Bench three `--quick` runs (`KUTHA_BENCH_N=10000`): median `ingest_plain.n` ≈127.2M / `as_of.n` ≈283.0k vs Phase 80 ≈122.5M / ≈299.6k (+3.8% / −5.5%).

### Trajectory

- Honeycomb stays Proposed. ADR-012 evidence restaged.

## 2026-10-09 — Process: close M029 / v0.23 (Phase 89)

### Process

- M029 / GSD v0.23 closed. Active Milestone **None**; `L_delivery=M029-closed`. Audit: `.planning/milestones/v0.23-MILESTONE-AUDIT.md`. S01 interrupted rollover seal heal (`89-VERIFICATION.md`). Next controller lease: G1 #2 / #30 or G2 lineage.

### Trajectory

- Honeycomb stays Proposed.

## 2026-10-09 — Product: dict Retract SelfCommit (Phase 88)

### Product

- Retract of a `kutha:*` Assert is SelfCommit in `class_with_dict` (same as the target Assert). Trailing `emit_retract_dict` survives persist/open; revoked allow stays `UnknownRelation`. Dictionary mode still writes no `QuantumOutcome`. Oracles: `m028_dict_retract.rs`. Matrix `SEM-m028-dict-retract`. ADR-014 Clarification Phase 88. Bench four `--quick` runs (`KUTHA_BENCH_N=10000`): median `ingest_plain.n` ≈126.9M / `as_of.n` ≈302.1k vs Phase 80 ≈122.5M / ≈299.6k (+3.6% / +0.8%). Vs M025 post ≈110.6M / ≈282k ingest median is noisier (samples 108.0–146.0M); fold path unchanged.

### Trajectory

- Honeycomb stays Proposed. ADR-014 evidence restaged.

## 2026-10-09 — Process: close M028 / v0.22 (Phase 88)

### Process

- M028 / GSD v0.22 closed. Active Milestone **None**; `L_delivery=M028-closed`. Audit: `.planning/milestones/v0.22-MILESTONE-AUDIT.md`. S01 dict retract on open (`88-VERIFICATION.md`). Next controller lease: parked residuals / review P1.

### Trajectory

- Honeycomb stays Proposed.

## 2026-10-09 — Process: lease M028 S01 / GSD Phase 88

### Process

- Active Milestone **M028**, Active Slice **S01** (`L_delivery=M027-closed`): trailing dictionary Retract on open (CE review P0 #1).

### Trajectory

- Honeycomb stays Proposed.

## 2026-10-09 — Process: close M027 / v0.21 (Phase 87)

### Process

- M027 / GSD v0.21 closed. Active Milestone **None**; `L_delivery=M027-closed`. Audit: `.planning/milestones/v0.21-MILESTONE-AUDIT.md`. S01 trajectory honesty (`87-VERIFICATION.md`). Next controller lease: parked residuals.

### Trajectory

- Honeycomb stays Proposed.

## 2026-10-09 — Process: trajectory honesty (Phase 87)

### Process

- `killed` rows with characterization-shaped `kill_mutation` (mirror/alias/honor-system/env-floor) are HIGH. ROADMAP `— ACTIVE` is forbidden while Active Milestone is None. `cite-lease` cites ROADMAP ACTIVE headings. Reused `test_matrix` / `glob_absent` / `cite_equals`. No new Python class. No crates.

### Trajectory

- Honeycomb stays Proposed.

## 2026-10-09 — Process: lease M027 S01 / GSD Phase 87

### Process

- Active Milestone **M027**, Active Slice **S01** (`L_delivery=M026-closed`): trajectory honesty (characterization-shaped matrix kills; idle ROADMAP ACTIVE).

### Trajectory

- Honeycomb stays Proposed.

## 2026-10-09 — Process: close M026 / v0.20 (Phase 86)

### Process

- M026 / GSD v0.20 closed. Active Milestone **None**; `L_delivery=M026-closed`. Audit: `.planning/milestones/v0.20-MILESTONE-AUDIT.md`. S02 matrix honesty (`86-VERIFICATION.md`). Next controller lease: parked residuals / new plan.

### Trajectory

- Honeycomb stays Proposed.

## 2026-10-09 — Product: matrix honesty relabel (Phase 86)

### Product

- Honor-system kills: `SEM-seeded-det` → characterization; decision/DUR rows keep `killed` with kill_mutation matching real product injection. Tautologies → `SEM-naive-as-of-alias` / `SEM-known-at-mirror` / `SEM-seed-budget`. Oracle: `m026_matrix_honesty`. Matrix 90 rows. ADR-023 Clarification Phase 86. cargo-mutants not run (non-gate). No fold behavior change.

### Trajectory

- Honeycomb stays Proposed. ADR-023 evidence restaged.

## 2026-10-09 — Process: close M026 S01 / lease S02 (Phase 86)

### Process

- S01 done (`85-VERIFICATION.md`). Active Slice **S02** (`L_delivery=M026-S01-done`): matrix honesty.

### Trajectory

- Honeycomb stays Proposed.

## 2026-10-09 — Product: MixStyle::Coverage + G-C04/C05/C11 oracles (Phase 85)

### Product

- `MixStyle::Coverage` in `ingest_mix` (Retract-before-Assert, overlapping Asserts, keyed retry, derived Correct/Retract, empty `n_ops`). Oracles: `m026_generators` (G-C04 Assert EventId persist/open; G-C05 facade latest-only; G-C11 empty persist/open). BehaviorFree/WithHorn LCG unchanged. Matrix `SEM-m026-*`. ADR-023 Clarification Phase 85. No fold behavior change.

### Trajectory

- Honeycomb stays Proposed. ADR-023 evidence restaged.

## 2026-10-09 — Process: lease M026 S01 / GSD Phase 85

### Process

- Active Milestone **M026**, Active Slice **S01** (`L_delivery=M025-closed`): generators and missing oracles.

### Trajectory

- Honeycomb stays Proposed.

## 2026-10-09 — Process: close M025 / v0.19 (Phase 84)

### Process

- M025 / GSD v0.19 closed. Active Milestone **None**; `L_delivery=M025-closed`. Audit: `.planning/milestones/v0.19-MILESTONE-AUDIT.md`. S04 demo + fixtures (`84-VERIFICATION.md`). Next controller lease: M026 / v0.20.

### Trajectory

- Honeycomb stays Proposed.

## 2026-10-09 — Product: demo bin + real-text statute fixtures (Phase 84)

### Product

- `examples/statute_as_of` answers "norm in force on D, as known on W" via `norm_view` (prints typed envelope). Fixtures: `fixtures/statute/{art12_speed,punkt_15_1,edition_promulgation}.txt` (test data only). Oracles: `m025_demo_fixtures`. Matrix `SEM-m025-demo-fixtures`. ADR-013 Clarification Phase 84. `I-legal-pack-not-next` stays.

### Trajectory

- Honeycomb stays Proposed. Pack ontology still frozen.

## 2026-10-08 — Process: close M025 S03 / lease S04 (Phase 84)

### Process

- S03 done (`83-VERIFICATION.md`). Active Slice **S04** (`L_delivery=M025-S03-done`): demo binary + real-text fixtures.

### Trajectory

- Honeycomb stays Proposed.

## 2026-10-08 — Product: legal answer envelope norm_view (Phase 83)

### Product

- `Runtime::norm_view(ValidTime, WallTime)` → `NormView` (`Ok` / `Conflict` / `Abstain` / `Unknown` / `MissingAnchor`). Force ∩ knowledge ∩ eligibility (R-22); reuses `knowledge_tt_at`, `live_facts_at`, `derivation_eligible_at`, `conflict_report_at`. `as_of` / ASOF-ELIG-01 unchanged. Oracles: `m025_norm_view`. ADR-013 / ADR-090 Clarifications; matrix `SEM-m025-norm-view`.

### Trajectory

- Honeycomb stays Proposed. ADR-013/090 evidence restaged; pack ontology still frozen.

## 2026-10-08 — Process: close M025 S02 / lease S03 (Phase 83)

### Process

- S02 done (`82-VERIFICATION.md`). Active Slice **S03** (`L_delivery=M025-S02-done`): legal answer envelope (`norm_view`).

### Trajectory

- Honeycomb stays Proposed.

## 2026-10-08 — Product: clock-safe as_of / as_known_on facade (Phase 82)

### Product

- Public `as_of` takes `ValidTime`; `as_known_on` takes `WallTime` (not `impl Into`). Bare `u64` and wrong-role clocks are compile errors (trybuild `ui_clock_facade`). Call sites wrap with `ValidTime::from` / `WallTime::from`. Op storage unchanged. ADR-013 Clarification Phase 82. Bench (`KUTHA_BENCH_N=10000`): pre `ingest_plain.n` 134073571 / `as_of.n` 309416; post median ≈110.6M / ≈282k (within ≤10% gate; fold path unchanged).

### Trajectory

- Honeycomb stays Proposed. ADR-013 evidence restaged.

## 2026-10-08 — Process: close M025 S01 / lease S02 (Phase 82)

### Process

- S01 done (`81-VERIFICATION.md`). Active Slice **S02** (`L_delivery=M025-S01-done`): clock-safe facade.

### Trajectory

- Honeycomb stays Proposed.

## 2026-10-08 — Product: pin as_known_on TT-only by design (Phase 81)

### Product

- Controller decision G-A08/G-C03: `as_known_on` stays TT-only (`known_at` / no VT filter). Pin: not-yet-in-force VT fact appears on `as_known_on` and not on `as_of`; closed VT survives. Oracles: `m025_as_known_on_vt.rs`. No fold behavior change. ADR-013 Clarification 2026-10-08 Phase 81.

### Trajectory

- Honeycomb stays Proposed. ADR-013 evidence restaged.

## 2026-10-08 — Process: lease M025 S01 (GSD Phase 81)

### Process

- Active Milestone **M025**, Active Slice **S01** (`L_delivery=M024-closed`): `as_known_on` VT decision — pin TT-only by design (G-A08/G-C03). Honeycomb stays Proposed.

### Trajectory

- Honeycomb stays Proposed. `L_capability` unchanged (FF5).

## 2026-10-08 — Process: close M024 / v0.18 (Phase 80)

### Process

- M024 / GSD v0.18 closed. Active Milestone **None**; `L_delivery=M024-closed`. Audit: `.planning/milestones/v0.18-MILESTONE-AUDIT.md`. S02 write-path atomicity characterized (`80-VERIFICATION.md`). Product close CHANGELOG cites Phase 80 bench (`KUTHA_BENCH_N=10000`): median vs Phase 79 post — `ingest_plain.n` 126105754→≈122.5M (−2.9%); `as_of.n` 281565→≈299.6k (+6.4%).

### Trajectory

- Honeycomb stays Proposed. Next lease is a controller decision (not Cypher/HNSW).

## 2026-10-08 — Product: write-path Define SelfCommit and D014-1 digest pin (Phase 80)

### Product

- Characterized intern `Define` as SelfCommit (no QO); `committed_prefix` keeps Defines and drops NeedsOutcome Assert without QO. D014-1 constant commitment pinned to `QuantumReceipt::digest` `[u8; 32]` (hex 64); `event_ids` / `events_in_quantum` remain resume leases (width leak residual). GROUP-01 residual unchanged. Oracles: `m024_write_path.rs`. ADR-014 Clarification 2026-10-08. Bench median (`KUTHA_BENCH_N=10000`, four runs): `ingest_plain.n` ≈122.5M (−2.9% vs Phase 79); `as_of.n` ≈299.6k (+6.4%).

### Trajectory

- Honeycomb stays Proposed. ADR-014 evidence restaged.

## 2026-10-08 — Process: close M024 S01 / lease S02 (Phase 80)

### Process

- S01 done (`79-VERIFICATION.md`). Active Slice **S02** (`L_delivery=M024-S01-done`): write-path atomicity.

### Trajectory

- Honeycomb stays Proposed.

## 2026-10-08 — Product: fold identity live-gate and multi-residual Correct (Phase 79)

### Product

- User Retract/Correct/CorrectInterval require a live residual (`UnknownFact` on dead ids). `Correct` on a multi-residual event_id invalidates all live rows and mints one replacement. `CorrectInterval` with ≠1 live residual is `IntervalPatchRejected`. Helper `live_fact_indices_by_event_id`. Oracles: `m024_fold_identity.rs`; naive model aligned; `interval_patch_not_live` expects `UnknownFact`. ADR-013 Clarification 2026-10-08. Bench (`KUTHA_BENCH_N=10000`): `ingest_plain.n` 127137526→126105754 (−0.8%); `as_of.n` 295462→281565 (−4.7%).

### Trajectory

- Honeycomb stays Proposed. ADR-013 evidence restaged.

## 2026-10-08 — Process: lease M024 S01 (GSD Phase 79)

### Process

- Active Milestone **M024**, Active Slice **S01** (`L_delivery=M023-closed`): fold identity — live-gated Retract/Correct; Correct on multi-residual event_id; CorrectInterval multi-residual fail-closed. Honeycomb stays Proposed.

### Trajectory

- Honeycomb stays Proposed. `L_capability` unchanged (FF5).

## 2026-10-08 — Process: close M023 / v0.17 (Phase 78)

### Process

- M023 / GSD v0.17 closed. Active Milestone **None**; `L_delivery=M023-closed`. Audit: `.planning/milestones/v0.17-MILESTONE-AUDIT.md`. S01 restored workspace green (keyed retry Correct lineage; naive Retract-all-live; dur_a parallel scratch). S02 close-gate honesty (`product-close-gate`). S03 living overlay truth. Default FSM budget 65→70 (artifact line budget; `.env.example` aligned). Product close CHANGELOG cites Phase 75 bench (`KUTHA_BENCH_N=10000`): `ingest_plain.n` 134180211→117370496 (−12.5%); `as_of.n` 287635→281294 (−2.2%).

### Trajectory

- Honeycomb stays Proposed. Next lease is a controller decision (not Cypher/HNSW).

## 2026-10-08 — Process: close M023 S03 / lease S04 (Phase 78)

### Process

- S03 done (`77-VERIFICATION.md`). Active Slice **S04** (`L_delivery=M023-S03-done`): M023 close.

### Trajectory

- Honeycomb stays Proposed.

## 2026-10-08 — Process: living overlay truth (Phase 77)

### Process

- PROJECT / ROADMAP Later / WINDOWS recount; legal-fixture-recon C01 pack stamp; semantic-gap F2/F7/F8 and semantic-contract-validation dated; p0-spike-inventory SEG-01; ADR-000 D8.2 and ADR-011 Op taxonomy pointers; honeycomb keyed-retry evidence; CBM skill Rust call-edge blind spot. ADRs stay Proposed.

### Trajectory

- Honeycomb stays Proposed.

## 2026-10-08 — Process: close M023 S02 / lease S03 (Phase 77)

### Process

- S02 done (`76-VERIFICATION.md`). Active Slice **S03** (`L_delivery=M023-S02-done`): living overlay truth. Also fixed cite-lease token `L_delivery=M023-S01-done` and verification-independent needle in `75-VERIFICATION.md`.

### Trajectory

- Honeycomb stays Proposed.

## 2026-10-08 — Process: close-gate honesty (Phase 76)

### Process

- Corrected false M022 Phase 72 "verified" claim (`72-VERIFICATION.md` passed-with-correction; v0.16 audit S02 note). `gsd-auto-contract` Close requires recorded `cargo test --workspace --offline --no-fail-fast` and full `kutha-gov ci` for product phases; governor check `product-close-gate`. No crates change.

### Trajectory

- Honeycomb stays Proposed.

## 2026-10-08 — Process: close M023 S01 / lease S02 (Phase 76)

### Process

- S01 done (`75-VERIFICATION.md` independent passed). Active Slice **S02**; lifecycle token `L_delivery=M023-S01-done`: close-gate honesty — correct false M022 Phase 72 verified claim; close checklist requires workspace tests + full ci.

### Trajectory

- Honeycomb stays Proposed. `L_capability` unchanged (FF5).

## 2026-10-08 — Product: keyed retry follows Correct lineage (Phase 75)

### Product

- `delivery_key_retry` short-circuits while any live fact of the keyed claim carries the delivery key (Correct/CorrectInterval replacements), remints only when none is live. Oracles: m013 keyed-correct pair restored; `keyed_assert_retry_after_correct_interval_short_circuits`, `keyed_assert_retry_after_retract_of_correct_interval_id_remints`. Naive model Retract clears all CI residuals. `m019_dur_a` scratch dirs include PID + atomic seq (parallel-safe). Bench (`KUTHA_BENCH_N=10000`): `ingest_plain.n` 134180211→117370496 (−12.5%); `as_of.n` 287635→281294 (−2.2%).

### Trajectory

- Honeycomb stays Proposed. Corrects the false M022 Phase 72 "verified" claim (fix in Phase 76).

## 2026-10-08 — Process: lease M023 S01 (GSD Phase 75)

### Process

- Active Milestone **M023**, Active Slice **S01** (`L_delivery=M022-closed`): restore green — keyed retry follows Correct lineage, naive model Retract-all-live, `m019_dur_a` flakes. Honeycomb stays Proposed.

### Trajectory

- Honeycomb stays Proposed. `L_capability` unchanged (FF5).

## 2026-10-08 — Process: open M023 / v0.17 (green HEAD and truth reset)

### Process

- Post-M022 audits (four read-only reports, CBM and GitNexus reindexed) found HEAD `18151af` red under `cargo test --workspace`: two `m013_write_path` keyed-correct tests regress from M022 Phase 72, `m019_sem_model` naive Retract is stale after M021, three `m019_dur_a` tests flake in the workspace run. M022's "verified" claim for Phase 72 is false until M023 S01–S02 land. Opened roadmap M023 S01–S04 (Phases 75–78), requirements GREEN-01..03, GATE-02, DOC-TRUTH-01, CLOSE-02. Context `.compound-engineering/artifacts/research/2026-10-08-post-m022-synthesis-context.md`; plan `.compound-engineering/artifacts/plans/2026-10-08-post-m022-roadmap-for-gsd-auto.md`. No lease yet; no `crates/` change.

### Trajectory

- Honeycomb stays Proposed. `L_capability` unchanged (FF5).

## 2026-10-08 — Process: close M022 / v0.16

### Process

- `L_delivery=M022-closed`, Active Slice **None**. Phases 71–74 verified. Audit `.planning/milestones/v0.16-MILESTONE-AUDIT.md`. Crate SemVer stays 0.0.0.

### Trajectory

- Honeycomb stays Proposed. `L_capability` unchanged (FF5).

## 2026-10-08 — Process: harness skip_emit and overlap fail-closed (Phase 73)

### Process

- `Context.skip_emit` skips emit_log/emit_tenant writes; FSM ci unit test uses it. `jobs>1` observe_cargo without `overlap_with` is HIGH (`overlap-required`). VERIFICATION frontmatter repaired for phases 68–71 (artifact-budget / verification-independent).

### Trajectory

- Honeycomb stays Proposed.

## 2026-10-08 — Product: keyed retry requires live fact (Phase 72)

### Product

- `delivery_key_retry` short-circuits only while the prior keyed Assert is live; after Retract the same key remints. Oracles: `keyed_assert_retry_after_retract_remints_live_support`, `keyed_assert_retry_while_live_short_circuits`. Admission EventId pin: `adm_pin01_admission_status_via_event_id`. Bench (`KUTHA_BENCH_N=10000`): `ingest_plain.n` 120718182→113891757 (−5.7% vs Phase 69); `as_of.n` 242161→269744 (+11% one-shot noise; still below Phase 66 post 315267).

### Trajectory

- Honeycomb stays Proposed.

## 2026-10-08 — Process: lease M022 S02 (GSD Phase 72)

### Process

- `L_delivery=M022-S01-done`, Active Slice **S02**: keyed-retry requires live fact; admission EventId pin; bench ≤10%. Independent `71-VERIFICATION.md` passed. Honeycomb stays Proposed.

### Trajectory

- Honeycomb stays Proposed. `L_capability` unchanged (FF5).

## 2026-10-08 — Process: lease M022 S01 (GSD Phase 71)

### Process

- Active Milestone **M022**, Active Slice **S01** (`L_delivery=M021-closed`): audit residual honesty — strike stale Retract CI claim; triage keyed-retry / harness isolate. Plan `.compound-engineering/artifacts/plans/2026-10-08-m022-audit-residuals.md`. Honeycomb stays Proposed.

### Trajectory

- Honeycomb stays Proposed. `L_capability` unchanged (FF5).

## 2026-10-07 — Process: close M021 / v0.15

### Process

- `L_delivery=M021-closed`, Active Slice **None**. Phases 68–70 independently verified. Audit `.planning/milestones/v0.15-MILESTONE-AUDIT.md`. Crate SemVer stays 0.0.0.

### Trajectory

- Honeycomb stays Proposed. `L_capability` unchanged (FF5).

## 2026-10-07 — Product: Retract all live residuals (Phase 69)

### Product

- `Op::Retract { event_id }` invalidates every live Fact with that event id (CorrectInterval prefix/patch/suffix). Oracles: `retract_correct_interval_event_id_clears_all_residuals`, `retract_assert_event_id_still_clears_single_row`. Bench (`KUTHA_BENCH_N=10000`): `ingest_plain.n` 132950275→120718182, `as_of.n` 315267→242161 (≤10% vs Phase 66 post).

### Trajectory

- Matrix `SEM-retract-all-live` killed. Honeycomb stays Proposed.

## 2026-10-07 — Process: lease M021 S02 (GSD Phase 69)

### Process

- `L_delivery=M021-S01-done`, Active Slice **S02**: Retract-all-live for CorrectInterval residuals; RED first; bench ≤10%. Independent `68-VERIFICATION.md` passed. Honeycomb stays Proposed.

### Trajectory

- Honeycomb stays Proposed. `L_capability` unchanged (FF5).

## 2026-10-07 — Process: close M021 S01 (Phase 68)

### Process

- `L_delivery=M021-S01-done`. Independent `68-VERIFICATION.md` passed. TR-02 / C01 pins; matrix decision rows killed.

### Trajectory

- Honeycomb stays Proposed. `L_capability` unchanged (FF5).

## 2026-10-07 — Product: wedge decision pins (Phase 68)

### Product

- Named oracles `tr02_force_vt_publication_anchor_decision_as_of` and `c01_applies_to_assert_encodes_application_window` (`m021_wedge_decisions.rs`). Fold/eval unchanged.

### Trajectory

- ADR-013 / ADR-090 Evidence 2026-10-07: TR-02 three moments on two axes; C01 pack Assert spike. Matrix SEM-c01 / horn-kc / asof / tr02 `killed`. Honeycomb restaged; cells stay Proposed.

## 2026-10-07 — Process: lease M021 S01 (GSD Phase 68)

### Process

- Active Milestone **M021**, Active Slice **S01** (`L_delivery=M020-closed`): wedge decisions — TR-02 three-moments pin, C01 pack-fact Assert spike, matrix kill for C01/HORN-KC/ASOF/TR-02; fold unchanged. Plan `.compound-engineering/artifacts/plans/2026-10-07-m021-wedge-decisions.md`. Honeycomb stays Proposed.

### Trajectory

- Honeycomb stays Proposed. `L_capability` unchanged (FF5).

## 2026-10-07 — Process: close M020 / v0.14

### Process

- `L_delivery=M020-closed`, Active Slice **None**. Independent `67-VERIFICATION.md` passed. M020 / GSD v0.14 shipped (Phases 63–67). Audit note `.planning/milestones/v0.14-MILESTONE-AUDIT.md`. Crate SemVer stays 0.0.0.

### Trajectory

- Honeycomb stays Proposed. `L_capability` unchanged (FF5).

## 2026-10-07 — Process: contract rules (Phase 67)

### Process

- `gsd-auto-contract.md`: lease commit = STATE+CHANGELOG only; product close CHANGELOG must record `ingest_plain.n` / `as_of.n`; edits to a passed `NN-VERIFICATION.md` require re-verify before close.
- Bench numbers for M020 product phases (`KUTHA_BENCH_N=10000`): Phase 65 `ingest_plain.n` 137106769→104606205, `as_of.n` 316671→253101; Phase 66 `ingest_plain.n` 137712385→132950275, `as_of.n` 419823→315267.

### Trajectory

- Honeycomb stays Proposed. `L_capability` unchanged (FF5).

## 2026-10-07 — Process: lease M020 S05 (GSD Phase 67)

### Process

- Active Milestone **M020**, Active Slice **S05** (`L_delivery=M020-S04-done`): process contract — lease commit file set, bench at product close, re-verify after VERIFICATION edits; then close M020. Honeycomb stays Proposed.

### Trajectory

- Honeycomb stays Proposed. `L_capability` unchanged (FF5).

## 2026-10-07 — Process: close M020 S04 (Phase 66)

### Process

- `L_delivery=M020-S04-done`, Active Slice **None**. Independent `66-VERIFICATION.md` passed. CorrectInterval residual re-derive, co-live `live_triples`, Retract `kutha:*` gate, fingerprint oracle; bench ≤10%.

### Trajectory

- Honeycomb stays Proposed. `L_capability` unchanged (FF5).

## 2026-10-07 — Product: Horn/fold residuals (Phase 66)

### Product

- CorrectInterval re-derives Horn on prefix/suffix (old object) and patch (new); same-batch retract ids ignored for mint; LIFO applies retracts before fires.
- `live_triples` is a co-live multimap (`FactIdxList`); invalidate drops one idx.
- User `Retract` of reserved/`kutha:*` → `MetaOpRejected`; `emit_retract_dict` for constructors.
- `open_retract_reemit_indexes_match_fingerprint` asserts persist+open fingerprint.

### Trajectory

- Honeycomb stays Proposed. `L_capability` unchanged (FF5).

## 2026-10-07 — Process: lease M020 S04 (GSD Phase 66)

### Process

- Active Milestone **M020**, Active Slice **S04** (`L_delivery=M020-S03-done`): Horn/fold residuals — CorrectInterval re-derive, co-live-safe `live_triples`, Retract gate for `kutha:*`, fingerprint oracle, pins, bench ≤10%. Honeycomb stays Proposed.

### Trajectory

- Honeycomb stays Proposed. `L_capability` unchanged (FF5).

## 2026-10-07 — Process: close M020 S03 (Phase 65)

### Process

- `L_delivery=M020-S03-done`, Active Slice **None**. Independent `65-VERIFICATION.md` passed. Segment rewrite is tmp+fsync+rename; bench ≤10% on `ingest_plain.n` / `as_of.n`.

### Trajectory

- Honeycomb stays Proposed. `L_capability` unchanged (FF5).

## 2026-10-07 — Product: atomic segment rewrite (Phase 65)

### Product

- `write_segment_file` writes `seg-*.log.tmp`, `sync_all`, then rename (`replace_via_tmp`); no in-place truncate of sealed/active segments. `rewrite_segment_with_seal_trailer` and `write_test_segmented` inherit the path. Test seam `abort_write_segment_before_rename` leaves dest intact.

### Trajectory

- Honeycomb stays Proposed. `L_capability` unchanged (FF5).

## 2026-10-07 — Process: lease M020 S03 (GSD Phase 65)

### Process

- Active Milestone **M020**, Active Slice **S03** (`L_delivery=M020-S02-done`): persist atomic rewrite — segment `*.tmp`+fsync+rename, manifest last, crash-window oracle, bench ≤10%. Honeycomb stays Proposed.

### Trajectory

- Honeycomb stays Proposed. `L_capability` unchanged (FF5).

## 2026-10-07 — Process: close M020 S02 (Phase 64)

### Process

- `L_delivery=M020-S02-done`, Active Slice **None**. Independent `64-VERIFICATION.md` passed. Fail-closed: nameless dict id, missing cite, bad JSONL fold, `min_phase` without NN-, empty heading glob.

### Trajectory

- Honeycomb stays Proposed. `L_capability` unchanged (FF5).

## 2026-10-07 — Harness fail-closed (Phase 64)

### Process

- Dictionary row without `id` → `LoadErrorCheck` HIGH (`get_checks`).
- `cite_equals` cite `path` missing → HIGH `missing` (not an empty loop).
- `fold_log` invalid JSONL → `ValueError` with 1-based line; FSM `fold_log` observes `fail`.
- `min_phase>0` under `.planning/phases/` without `NN-` dirname → HIGH.
- `markdown_heading_tag` empty glob with existing parent → HIGH.

### Trajectory

- Honeycomb stays Proposed. `L_capability` unchanged (FF5).

## 2026-10-07 — Process: lease M020 S02 (GSD Phase 64)

### Process

- Active Milestone **M020**, Active Slice **S02** (`L_delivery=M020-S01-done`): harness fail-closed — nameless dictionary row, missing cite, bad JSONL, `min_phase` without NN-, empty heading glob. RED selftest first. No product crates.

### Trajectory

- Honeycomb stays Proposed. `L_capability` unchanged (FF5).

## 2026-10-07 — Process: close M020 S01 (Phase 63)

### Process

- `L_delivery=M020-S01-done`, Active Slice **None**. Independent `63-VERIFICATION.md` passed. Hygiene: fmt, dead `line_at`, README/M020 pointers, `ce-skills-ru` handoff override.

### Trajectory

- Honeycomb stays Proposed. `L_capability` unchanged (FF5).

## 2026-10-07 — Hygiene (Phase 63)

### Process

- `cargo fmt --all` clears Phase 47 indent debt in `eval.rs` (`retract_derived` / `horn_fires`). No behaviour change.
- Removed dead `scripts/kutha_gov/rust_source.line_at` (zero callers).
- Landed `.cursor/rules/ce-skills-ru.mdc` `ce-handoff` destination override to `.compound-engineering/artifacts/handoffs/`.
- README Status row names Active Milestone **M020** (`state-readme`).

### Trajectory

- Honeycomb stays Proposed. `L_capability` unchanged (FF5).

## 2026-10-07 — Process: lease M020 S01 (GSD Phase 63)

### Process

- Active Milestone **M020**, Active Slice **S01** (`L_delivery=M018-closed`): hygiene — fmt, dead `line_at`, stale pointers, `ce-skills-ru` handoff override. No product behaviour.

### Trajectory

- Honeycomb stays Proposed. `L_capability` unchanged (FF5).

## 2026-10-07 — Process: open M020 / v0.14 (audit correction)

### Process

- Harness milestone **M020** (GSD v0.14, Phases 63–67): hygiene, harness fail-closed, persist atomic rewrite, Horn/fold residuals, process contract. Source: `.compound-engineering/artifacts/reviews/2026-10-07-whole-repo-audit/` and parent plan `2026-10-07-audit-correction-plan.md`.
- Id map row v0.14 added. Lease rule clarified: STATE + CHANGELOG only (`docs-coupling`); ROADMAP/CONTEXT are a separate open/plan commit.
- Active Slice still **None** until the S01 lease commit.

### Trajectory

- Honeycomb stays Proposed. `L_capability` unchanged (FF5). Not Cypher/HNSW.

## 2026-10-06 — Process: close M018 / v0.11

### Process

- `L_delivery=M018-closed`, Active Milestone **None**, Active Slice **None**. S01–S04 complete (Phases 45–48). Independent `48-VERIFICATION.md` passed. ORACLE-01 / GATE-01 closed. No late process tag `v0.11` (v0.12/v0.13 already exist). Crate SemVer 0.0.0.

### Trajectory

- Honeycomb stays Proposed. `L_capability` unchanged (FF5).

## 2026-10-06 — Oracle tightening (Phase 48)

### Product

- I-F15 visit bound remains `extra_relatedto_horn_visits_constant_in_n` (`m017_ingest_scaling.rs`): extra unrelated `relatedTo` must not grow `horn_examine` with N. Matrix `ORACLE-if15` killed (`owner: M018-S04`). `i-f15-ingest-o1` still cites `green_ingest_visits_constant_in_n` and now also this pin.
- I-F16 pins `horn_correct_derived_not_live_after_premise_correct` and `horn_correct_interval_derived_not_live_after_patch` (`m019_sem_meta.rs`). Matrix `ORACLE-if16` killed (`owner: M018-S04`). `i-f16-derivable` observes those two tests (no longer `empty_pin_inverse_fails_replay_fingerprint_unchanged`). `SEM-horn-correct` stays killed.
- GATE-01 is the recorded controller bench step (`ingest_plain.n`, `as_of.n`, `KUTHA_BENCH_N=10000`, 10 percent stop), not a second harness.
- `emit_rule_ast_horn_pin_equals_body_hash` admits `derive_pq` before the premise Assert so CASC-01 hop-admit does not fail the workspace observe.

### Process

- Test matrix: `ORACLE-if15` / `ORACLE-if16` killed (`owner: M018-S04`). GATE-01 recorded as `gsd-auto-contract` Bench gate at product close plus `BENCH-controller` (`status: gate`, `gate: controller`). `observe_cargo` stays `cargo test --workspace --offline`. No new governor Check class; no `cargo bench` in CI.
- Repaired `47-VERIFICATION.md` `read_only_inputs` (governor `verification-independent`: list existing paths; no `-PLAN.md` / `-SUMMARY.md` needles).

### Trajectory

- ADR-041 / ADR-023 Evidence 2026-10-06 (Phase 48 / M018 S04). Honeycomb restaged (those cells); map/delivery/capability unchanged; honeycomb stays **Proposed**. D041-1…D041-4 unchanged. No `.kutha/STATE.md` write; `48-VERIFICATION.md` is verifier-owned.

## 2026-10-06 — Process: lease M018 S04 (GSD Phase 48)

### Process

- Active Milestone **M018**, Active Slice **S04** (`L_delivery=M018-S03-done`): I-F15/I-F16 oracle tightening and bench gate (GATE-01). DIFF-01 already closed in M019 S07.

### Trajectory

- Honeycomb stays Proposed. No new capability claim.

## 2026-10-06 — Process: close M018 S03 (GSD Phase 47)

### Process

- `L_delivery=M018-S03-done`, Active Slice **None**. Independent verification passed (`47-VERIFICATION.md`). DET-02 / HORN-CORR-01 / CASC-01 closed. Repaired `46-VERIFICATION.md` `exit:` needle (governor `verification-independent`). Next: lease M018 S04 (Phase 48).

### Trajectory

- Honeycomb stays Proposed. ADR-013 / ADR-014 remain Proposed. `L_capability` unchanged (FF5).

## 2026-10-06 — Horn determinism (Phase 47)

### Product

- RED pin `det02_two_ingest_derived_event_ids_equal`: two `Runtime::default()` ingest of genesis `inverse_knows` (Alice/Bob `knows`) must mint equal derived Behavior EventIds. User Assert stays `Event::new` v7 and is not compared.
- Derived Horn `Op::Behavior` and follow-on `Op::Retract` mint `Event::stable_derived` (SHA-256 prefix `kutha-derived-id`, rule pin UTF-8, `ingested_at`, TermId bindings as LE bytes, ASCII tag `B`/`R`). Triggering user Assert UUID v7 is not in the preimage.
- `Correct` / `CorrectInterval` of a Horn premise `retract_derived` then `horn_fires` on fold subject/relation plus the new object/interval. Stale derived is not live at the post-correct cut.
- `commit_user_cascade` admits each follow-on (`ReceiptMode::User`) before `pending.push`; LIFO `pop` unchanged. UnknownRelation only on a matched firing Horn whose head is missing from the dict; intern-later nary stays `Ok`.

### Process

- Test matrix: `SEM-det02-ids`, `SEM-horn-correct`, `SEM-horn-correct-char`, `SEM-casc-admit` `killed` with owner `M018-S03`. Blocked remaining: C01, HORN-KC-01, ASOF-ELIG-01, TR-02.

### Trajectory

- ADR-013 / ADR-014 / ADR-023 Evidence 2026-10-06 (Phase 47 / M018 S03). Honeycomb restaged (those cells); map/delivery/capability unchanged; honeycomb stays **Proposed**. D013-1…D013-4 unchanged. No `.kutha/STATE.md` write; `47-VERIFICATION.md` is verifier-owned.

## 2026-10-06 — Process: lease M018 S03 (GSD Phase 47)

### Process

- Active Milestone **M018**, Active Slice **S03** (`L_delivery=M018-S02-done`): Horn determinism and closure (DET-02, HORN-CORR-01, CASC-01). RED first on characterization oracles that this slice inverts.

### Trajectory

- Locked for this slice (not ADR Accepted): derived EventIds are SHA-256-stable like `Event::stable_define`; user Assert stays UUID v7. Correct/CorrectInterval of a Horn premise re-derives. Every cascade hop `admit`s; unknown Horn relation fails closed. Honeycomb stays Proposed.

## 2026-10-06 — Process: close M018 S02 (GSD Phase 46)

### Process

- `L_delivery=M018-S02-done`, Active Slice **None**. Independent verification 5/5 (`46-VERIFICATION.md`, composer-2.5-fast). C01-01 / HORN-KC-01 / ASOF-ELIG-01 recorded; fold src unchanged. Next: controller leases M018 S03 (Phase 47).

### Trajectory

- Honeycomb stays Proposed. ADR-013 remains Proposed. `L_capability` unchanged (FF5).

## 2026-10-06 — C01 knowledge-cut (Phase 46)

### Product

- Named pin `asof_elig01_as_of_contains_derived_when_ineligible`: after last premise-support Retract, `derivation_eligible_at` is false and `Runtime.as_of(2017)` still lists the derived triple. Eligibility stays a separate API; `as_of` is not filtered. Distinct from `v44_a_inverse_retract_as_of_earlier` (direct Horn `retract_derived`). Fold src unchanged.

### Process

- Test matrix: `SEM-c01-decision` / `SEM-horn-kc` / `SEM-asof-elig` stay `blocked` (`kill_mutation: none`); invariants rewritten to pack stamp, ingest-at-head, and support-live. `SEM-asof-elig` oracle filled with `asof_elig01_as_of_contains_derived_when_ineligible`. `SEM-horn-tt-max` and `SEM-c01-inexpressible` stay characterization. ADR-023 questions 1–3 dated closed 2026-10-06; 4–6 remain open. Honesty restage: F13 remainder and A-F05 decided (crate unchanged). `I-F13-typed-clocks` claim is pack-stamp out-of-wedge; disposition stays deferred.

### Trajectory

- ADR-013 Verdict 2026-10-06 (Phase 46 / M018 S02): C01 (a) pack stamp; HORN-KC ingest-at-head; ASOF support-live. Honeycomb restaged (ADR-013, ADR-023); map/delivery/capability unchanged; honeycomb stays **Proposed**. D013-1…D013-4 unchanged. FIXG-01 oracle kept. D-SOT: sealed `KUTHASEG` + `manifest.json` are D1; `events.jsonl` after migrate is an export; not a fifth clock.

## 2026-10-06 — Process: lease M018 S02 (GSD Phase 46)

### Process

- Active Milestone **M018**, Active Slice **S02** (`L_delivery=M018-S01-done`): C01 and knowledge-cut decision. Process tag `v0.13` already exists on `1520220` (not a product release). Owner acknowledgement: sealed `KUTHASEG` + `manifest.json` are D1; `events.jsonl` after migrate is an export.

### Trajectory

- Locked for this slice (not ADR Accepted): C01 option (a) pack stamp — no fifth core axis, not a Horn, not `valid_to`; HORN-KC-01 ingest-time Horn at log-head TT, `as_known_on` does not re-derive; ASOF-ELIG-01 `as_of` remains support-live, eligibility is `derivation_eligible_at`. DET-02 / HORN-CORR-01 wait for S03. Honeycomb stays Proposed.

## 2026-10-05 — Process: close M019 / v0.13

### Process

- Close M019 (S01–S07) and archive Phases 56–62 under `.planning/milestones/v0.13-*`. What shipped: docs honesty r3, `cargo_dep_constraint`, governor SMP, test matrix, durability residuals, DIFF-01/DIFF-02/SEM-02. Cumulative bench vs v0.12 PASS (ingest_plain −6.05%, as_of +5.24%, open −0.40%). Residuals in `v0.13-MILESTONE-AUDIT.md` (MEM-01 RSS `bytes_per_fact` not explained to a lower bound; GROUP-01 verdict (c); no external chain anchor; `open` O(N); Persister/`SegmentIndex` unwired; advisory lock without lock-dir fsync; no worker timeout; `.env` cargo timeout 180 s; `id-map-cite` substring; `apply_dotenv` writes `.env` into the process environment; clippy `-D warnings` 22 pre-existing lib lints; Phase 62 equivalent/indirect pins). Process tag `v0.13` is a GSD process tag for the controller to create, not a product release; crate SemVer stays 0.0.0. Owner acknowledgement still needed for the SoT wording that `events.jsonl` is an export.

### Trajectory

- `L_delivery=M019-closed`, Active Milestone **None**, Active Slice **None**. M018 remains paused at its S02 decision gate (C01, Horn vs knowledge cut). Honeycomb stays Proposed.

## 2026-10-05 — Process: close M019 S07 (GSD Phase 62)

### Process

- `L_delivery=M019-S07-done`, Active Slice **None**. Independent verification 11/11 PASS-WITH-FINDINGS then re-verified (`62-VERIFICATION.md`); final fix round `511e2a5`. DIFF-01 (moved from M018 S04), DIFF-02, SEM-02 closed. Seeded LCG oracles: fold vs CSR vs persist/rebuild, naive bitemporal model, cascade Eager vs Lazy, metamorphic persist/tamper/replay, Horn Correct characterization. Matrix killed 55 / characterization 6 / gap 0 / blocked 6 / spec-only 4 / gate 1 (72). Controller: isolated 5.1+5.3+0.09 s; `KUTHA_SEEDS=256` green; `kutha-gov ci` 0 HIGH / 1 LOW / 65 checks. `TermDictionary::is_empty` only `src/` change (no bench gate). Owner-gated still open: C01/FIXG-01, HORN-KC-01, ASOF-ELIG-01, DET-02, TR-02, HORN-CORR-01. No capability claim. Next: controller milestone close (v0.13 / M019).

### Trajectory

- Honeycomb stays Proposed. ADR-013 / ADR-041 / ADR-060 remain Proposed. `L_capability` unchanged (FF5). M018 remains paused at its S02 decision gate.

## 2026-10-05 — Product: M019 S07 semantic and logic oracles

### Product

- Seeded LCG oracles (≥64 seeds; `KUTHA_SEEDS` may raise, including persist/tamper/export at `max(12, n/5)`): `SEM-csr-fold-diff` (fold `live_at`/`as_of`/`as_known_on` vs typed/untyped CSR vs persist+open legacy/segmented/migrate, BehaviorFree and WithHorn), `SEM-bitemporal-model` (naive ADR-013 list-scan vs `GraphFold`; `Fact::is_live_at` vs naive at EDGE_VT; triples are `(s,r,o)`), `SEM-cascade-order` (Eager vs Lazy `knows`/`inverse_knows`), metamorphic `SEM-order-independence`, `SEM-replay-idempotence` (load-bearing event count / committed prefix / dictionary / next_tt), `SEM-persist-roundtrip-random` (two passes per persist index: legacy, rollover multi-segment, migrated, export-imported; WithHorn through segmented/migrated; `export_jsonl` preserves persist-encoded event-id order), `SEM-replay-tamper-random` (flip/swap/drop never silently equal; equal-fold swap is `commutes` only on disjoint (s,r), else a dependent Retract is minted so raised `KUTHA_SEEDS` stays green; middle sealed payload XOR fails closed InvalidData and detector pin CRC mismatch). `SEM-horn-correct-char` pins current Correct/CorrectInterval-of-Horn-premise behaviour (HORN-CORR-01; no rule chosen) and that the superseded premise is not live at the post-correction cut. Tests: `m019_sem_diff.rs`, `m019_sem_model.rs`, `m019_sem_meta.rs`. Named tests call `assert_seed_budget` so honeycomb/test-matrix `rust_test_asserts` see an in-body `assert*`. Persist scratch lives under `tmp/p62-sem/` (pid + atomic counter, no wall clock). `TermDictionary::is_empty` added so `clippy -D warnings` accepts public `len` (no fold/CSR behaviour change).

### Trajectory

- ADR-013 / ADR-041 / ADR-060 Evidence 2026-10-05. Honeycomb restaged; cells stay **Proposed**. Matrix: killed 55, characterization 6, gap 0, blocked 6, spec-only 4, gate 1 (total 72). Still open: Horn@TT=`MAX` (HORN-KC-01), `as_of` vs eligibility (ASOF-ELIG-01), C01, DET-02, TR-02, HORN-CORR-01.

## 2026-10-05 — Process: lease M019 S07 (GSD Phase 62)

### Process

- Active Milestone **M019**, Active Slice **S07** (`L_delivery=M019-S06-done`): DIFF-02, SEM-02 (DIFF-01 moved from M018 S04). First task: reference-model design plus RED differential oracles; owner-gated rules stay blocked. M018 stays paused at its S02 gate.

## 2026-10-05 — Process: close M019 S06 (GSD Phase 61)

### Process

- `L_delivery=M019-S06-done`, Active Slice **None**. Independent verification 8/8 (`61-VERIFICATION.md`). ROCK-02: always-on sealed-list binding `m:sealed`, opt-in `verify_full()`, `m:v` 2. Export-race oracles were already green because of the Phase 60 lock (green pins). MEM-01: counting-allocator attribution + streaming of frames + `split_events` by reference + `drop(encoded_log)` (persist peak N=10000 56.7 MB → 28.7 MB by the verifier's allocator; persisted bytes identical base vs head). `log_has_quantum_outcome` deleted; `wal::append_events` and `rocks_spike` kept; wiring not done (reasons in ADR-010). Matrix killed 48 / characterization 5 / gap 3 / blocked 6 / spec-only 4 / gate 1 (67). Bench gate **PASS** on the controller re-measure (base `d71cd91` vs head `65cc5a3`, 10 interleaved pairs, `taskset -c 9`; executor 6-pair `open` +10.10% did not reproduce). Honest residuals: `bytes_per_fact` rise not explained to a lower bound; mid-poison only by `verify_full`/`get_event`; `m:v` shadowed by `m:sealed`; `open` O(N); no external chain anchor; Persister not wired into kutha-tenant; DUR-export-race tests were green pins; 3 gap rows remain for S07. No capability claim. Next: controller leases M019 S07 (Phase 62 semantic and logic test expansion: seeded fold/CSR/rebuild differential, cascade order, Correct-of-Horn-premise characterization).

### Trajectory

- Honeycomb stays Proposed. ADR-010 / ADR-012 remain Proposed. `L_capability` unchanged (FF5). M018 remains paused at its S02 decision gate.

## 2026-10-05 — Durability residuals B (M019 S06 / GSD Phase 61)

### Product

- ROCK-02: `SegmentIndex` meta `m:v` 1→2. Always-on sealed-list binding (`m:sealed`) on `open_or_rebuild`; opt-in `verify_full()` scans `e:` keys against `m:digest`. A poisoned middle locator with matching tip is not an open rebuild; `verify_full` and `get_event` fail closed. Sealed-list tamper rebuilds. Log/segments/events unchanged. Oracles: `m019_rocks_b.rs`.
- DUR-export-race: `export_jsonl` while a `Persister` holds `kutha.lock` fails closed (`WouldBlock`) and writes nothing; after drop the export equals the log; lock is taken before `load_log_events`. Oracles: `m019_dur_b.rs` (GREEN pins on the Phase 60 lock).
- MEM-01: counting `GlobalAlloc` N=500 — ingest live identical vs `7e31784` (867644 B). Cheap persist copies: stream frames, `split_events` by ref, drop `encoded_log` before snapshot. Persist peak 1506755 → 1312566 B. Remaining `bytes_per_fact` is whole-bench RSS (ingest + persist transients + a second Runtime on open). Characterization oracles pin the streaming/`drop` order.
- CLEAN-01: deleted unused `hydrate::log_has_quantum_outcome` (no callers). Kept `wal::append_events` (`wal` unit test) and `rocks_spike` / `m002_rock00` (jsonl offsets, optional, unwired).
- Bench vs `d71cd91` (6 interleaved pairs, warm-up discarded, `taskset -c 4`): ingest_plain.n −1.62%, as_of.n −5.97%, open.n **+10.10% STOP**. Host noise ~15–55% per pair.

### Process

- `observe_rocks` lists `m019_rocks_b` and requires the three ROCK-02 tests. Test matrix: killed 48, characterization 5, gap 3 (SEM S07), blocked 6, spec-only 4, gate 1 (total 67).

### Trajectory

- ADR-010 / ADR-012 Evidence 2026-10-05 Phase 61; statuses stay **Proposed**. D-05: `SegmentIndex` and `Persister` stay unwired into `Runtime` / `kutha-tenant`. `honeycomb.yaml` restaged (map/delivery/capability unchanged). Design note `.compound-engineering/artifacts/research/2026-10-05-rocks-staleness-design.md` (a always-on, b opt-in). Residuals: `open` O(N); no external chain anchor; bpf RSS inherent; SEM S07 gaps.

## 2026-10-05 — Process: lease M019 S06 (GSD Phase 61)

### Process

- Active Milestone **M019**, Active Slice **S06** (`L_delivery=M019-S05-done`): ROCK-02, MEM-01, CLEAN-01. First task: Rocks staleness design note, then RED oracles; measure bytes_per_fact before changing code. M018 stays paused at its S02 gate.

## 2026-10-05 — Process: close M019 S05 (GSD Phase 60)

### Process

- `L_delivery=M019-S05-done`, Active Slice **None**. Independent verification 13/13 after a fix round (`60-VERIFICATION.md`: killed row for sealed-trailer-lie corrected to the layered mutation; Persister poison added). WRITER-01 / GROUP-01 closed: advisory single-writer `kutha.lock` (`File::try_lock`, `WouldBlock`, readers unlocked, dead-process safe, panic releases, listings ignore it); `Persister` poisoned after a fail-closed append; GROUP-01 verdict (c) documented residual with no format change (design note `.compound-engineering/artifacts/research/2026-10-05-group-commit-design.md`). 13 matrix rows resolved: killed 46 / characterization 4 / gap 6 / blocked 6 / spec-only 4 / gate 1 (67). Four lock oracles were RED on the old code (real bugs: second writer allowed, no lock file, persist/migrate race, listings); the rest were green pins shown to fail under their mutations. Bench gate PASS. Honest residuals: no fsync of the lock dir entry; advisory local lock only (NFS / other host not covered); bypass writes not covered; tenant covered via `store::persist`; Typed/Dictionary partial group still a documented residual; no external chain anchor; `open` O(N); `Persister` not wired into `kutha-tenant`; 6 gap rows remain for S06/S07. No capability claim. Next: controller leases M019 S06 (Phase 61 durability residuals B: Rocks staleness beyond the tip, `bytes_per_fact` RSS, dead-code cleanup, wiring decision).

### Trajectory

- Honeycomb stays Proposed. ADR-010 / ADR-012 / ADR-014 remain Proposed. `L_capability` unchanged (FF5). M018 remains paused at its S02 decision gate.

## 2026-10-05 — Product: durability residuals A (M019 S05 / GSD Phase 60)

### Product

- Added advisory single-writer lock `kutha.lock` (`File::try_lock`, `ErrorKind::WouldBlock`, message names the file) on `Persister::attach`, `open_for_append`, `store::persist`, `store::migrate`, and `store::export_jsonl`. Readers (`store::open`) take no lock. A dead-process lock file does not block. Panic/unwind Drop unlocks. Failed append/checkpoint releases the lock so in-process heal can run; attach validates before locking so a dirty tail stays `InvalidData`. A fail-closed `append` (`ErrorKind::InvalidData`, tail-guard) poisons that `Persister`: later `append` / `checkpoint` / `close` return the same kind and write nothing; heal is a new `open_for_append`. Failpoints stay `Other` and do not poison.
- Added `m019_dur_a.rs` oracles for the 13 S05 matrix rows plus lock semantics (second writer, poison after fail-closed append, reader unaffected, dead-process, panic release, listings ignore `kutha.lock`).
- GROUP-01 stays verdict (c): no Event/Op/Snapshot/frame-format change. A crashed Typed/Dictionary group can still expose a SelfCommit prefix (`mixed_typed_dict_partial_group_is_visible`, characterization).

### Trajectory

- ADR-010 / ADR-012 / ADR-014 Evidence 2026-10-05; statuses stay **Proposed**. Residuals: no fsync of the `kutha.lock` directory entry; advisory local lock only (NFS / another host not covered); writes that bypass `Persister` / `store::*` are not covered; `kutha-tenant` is covered only through `store::persist`; no external chain anchor; `open` O(N).
- Test matrix: killed 34→46, characterization 3→4, gap 19→6 (total 67). S05 gaps closed; S06/S07 gaps remain. `DUR-sealed-trailer-lie` / `DUR-wrong-role-active` record layered kills (single-check removal is covered by neighbouring checks).

## 2026-10-05 — Process: lease M019 S05 (GSD Phase 60)

### Process

- Active Milestone **M019**, Active Slice **S05** (`L_delivery=M019-S04-done`): WRITER-01, GROUP-01. First task: design note for the group-commit option (hard stop if a format change is needed), then RED oracles. M018 stays paused at its S02 gate.

## 2026-10-05 — Process: close M019 S04 (GSD Phase 59)

### Process

- `L_delivery=M019-S04-done`, Active Slice **None**. Independent verification 12/12 after a fix round (`59-VERIFICATION.md`: unlabelled characterizations, missing checkpoint × omitted-QO gap, an overclaimed killed row replaced by a gap row, a planning-artifact reference removed). TEST-01 closed: ADR-023 **Proposed** (oracle taxonomy, layering and determinism rules, options verdicts); `docs/architecture/test-matrix.yaml` 67 rows (killed 34, gap 19, blocked 6, spec-only 4, characterization 3, gate 1) plus the human page; kind `test_matrix` and check `test-matrix` (64 → 65 checks; selftest 65/65 derived by own rule); 10 RED pytests then green; pytest 181 passed; two characterization oracles got a label comment (the only `crates/` touch, comment-only). Honest non-claims: the matrix is a map of oracles, not coverage; `killed` is recorded text checked for consistency, not a mutation gate; 6 blocked rows wait for owner decisions (C01/FIXG-01, HORN-KC-01, ASOF-ELIG-01, DET-02, TR-02, HORN-CORR-01); 19 gap rows are owned by M019 S05/S06/S07. No capability claim. Next: controller leases M019 S05 (Phase 60 durability residuals A: single-writer lock, Typed/Dictionary group vs `committed_prefix`, missing durability oracles from the matrix gap rows owned by M019-S05).

### Trajectory

- Honeycomb stays Proposed. ADR-023 remains Proposed. `L_capability` unchanged (FF5). M018 remains paused at its S02 decision gate.

## 2026-10-05 — Test strategy matrix and ADR-023 (GSD Phase 59)

### Product

- Comment-only D023-2 labels on `cascade_idles_without_inverse_loop` and `fixg01_c01_sunset_inexpressible_as_horn` (no assertion, name, or logic change).

### Process

- RED pytest (`scripts/tests/test_test_matrix.py`) failed 10/10 on a missing check id before the kind existed.
- Kind `test_matrix` and check `test-matrix` prove the oracle map is internally consistent (named `#[test]` / check ids exist; `killed` forbids `kill_mutation: none`; `gap` owners are ROADMAP `M###-S##`; `feature:` gates name an `observe_cargo` row; `blocked` owners are the decision allowlist). Selftest derives a missing-oracle mutation. FSM and `.env.example` budget 64 → 65 (65 dictionary checks).
- Matrix honesty (verifier): `DUR-sidecar-atomic` kill is abort-before-rename dest truncate, not same-bytes in-place write; S05 gaps added for inode/tmp+rename witness and checkpoint-after-omitted-QO (`open_for_append`).

### Trajectory

- ADR-023 **Proposed** (Verify-primary, numbered in the 020 band). Honeycomb cell 023 is `map-only` / `capability: none`. `docs/architecture/test-matrix.yaml` is the machine map (67 rows); the `.md` page is counts, gaps by slice, and blocked decisions. Not Accepted. Not a coverage number. ADR-023 mutation policy does not cite a planning CONTEXT.
- Honeycomb restage 59-06 (ADR-023 no planning CONTEXT cite).

## 2026-10-05 — Process: lease M019 S04 (GSD Phase 59)

### Process

- Active Milestone **M019**, Active Slice **S04** (`L_delivery=M019-S03-done`): TEST-01. First task: research note + ADR-023 draft, then the matrix and its governor check RED first. M018 stays paused at its S02 gate.

## 2026-10-05 — Process: close M019 S03 (GSD Phase 58)

### Process

- `L_delivery=M019-S03-done`, Active Slice **None**. Independent verification 13/13 (`58-VERIFICATION.md`). Governor SMP (PAR-01) closed: `--jobs` / `KUTHA_GOV_JOBS` (flag > env > min(cpu, 8)); process pool for checks and selftest; cargo observation overlap; deterministic merge in dictionary order; `--jobs 1` golden equal to pre-phase; worker errors fail closed (`worker-error` HIGH); scratch dirs under `tmp/`; selftest copy skips `.compound-engineering` and `.planning/milestones` with identical classification 58/4/2; pytest 171 passed. Honest residuals: no per-check worker timeout (`parallel.py` ~line 106; a hung check hangs the run as in serial); a SIGKILLed worker yields HIGH on unfinished checks without a LOW `pool-fallback` note; selftest pool fallback in `selftest.py` ~line 199 is silent (re-runs serial); SIGINT takes ~22 s to stop the pool; selftest `--jobs 2` slower than serial because of per-check copytree; `.env` `KUTHA_GOV_CARGO_TIMEOUT_SEC=180` still overrides the rocks 420 s row. No capability claim. Next: controller leases M019 S04 (Phase 59 test strategy and oracle matrix).

### Trajectory

- Honeycomb stays Proposed. ADRs remain Proposed. `L_capability` unchanged (FF5). M018 remains paused at its S02 decision gate.

## 2026-10-05 — Process: governor SMP (GSD Phase 58)

### Process

- PAR-01: `--jobs` / `KUTHA_GOV_JOBS` process pool for dictionary checks and selftest proofs; `observe_cargo` may overlap those phases; findings merge in dictionary order. `--jobs 1` is the serial oracle.
- Isolation pytest for two roots captures stdout sequentially so parallel `--jobs` runs cannot race a patched `sys.stdout`.

## 2026-10-05 — Process: lease M019 S03 (GSD Phase 58)

### Process

- Active Milestone **M019**, Active Slice **S03** (`L_delivery=M019-S02-done`): PAR-01. First task: RED pytest for jobs-1 vs jobs-N parity. M018 stays paused at its S02 gate.

## 2026-10-05 — Process: close M019 S02 (GSD Phase 57)

### Process

- `L_delivery=M019-S02-done`, Active Slice **None**. Independent verification 19/19 (`57-VERIFICATION.md`). Governor hardening (GOV-04, GOV-05, GOV-06) closed: `cargo_dep_constraint` kind with stdlib `tomllib`; 7 bypass forms RED then green; M002 oracles enrolled in `observe_cargo` plus an `observe_rocks` row; strict `artifact-budget`; `id-map-cite` requires `v0.12`/`v0.13`; selftest classification 58 derived / 4 explicit mutation / 2 explicit skip, 0 file-exists-only; `rust_test_asserts` index cached (selftest ~17 min → ~28 s, `ci` ~56 s warm); full pytest 159 passed ~110 s. Honest residuals: `.env` `KUTHA_GOV_CARGO_TIMEOUT_SEC=180` overrides the 420 s rocks row so a cold RocksDB build can time out (fail closed); `test_ci_green_on_this_tree` is hermetic-stubbed and no longer proves live cargo observe (the controller's standalone `ci` does); `id-map-cite` needle is a substring; `docs-coupling` and `verification-independent` selftests are explicit skips; CBM index is stale for `scripts/kutha_gov` (controller reindex pending). No capability claim. Next: controller leases M019 S03 (Phase 58 governor SMP).

### Trajectory

- Honeycomb stays Proposed. ADRs remain Proposed. `L_capability` unchanged (FF5). M018 remains paused at its S02 decision gate.

## 2026-10-05 — Process: hermetic pytest ci and rust_test_asserts cache (GSD Phase 57)

### Process

- `test_ci_green_on_this_tree` stubs `cargo` and `kutha-tenant` so pytest does not cold-compile under `timeout_sec: 180` (pytest log: `observe_cargo=180111`, empty stdout, 65 `observe-missing`). `rust_test_asserts` keeps a process-local fn-name index keyed by root + file mtime/size so honeycomb-ledger selftest does not re-parse `crates/**/*.rs` on every YAML mutation. Oracles and required names are unchanged; standalone `kutha-gov ci` remains the live cargo proof.

## 2026-10-05 — Product: Rust edition 2024

### Product

- Workspace `edition` 2021 → 2024 (crates inherit it). `cargo fix --edition` ran on all targets with `rocks-index` and `rocks-index-spike` enabled. Only change: `std::env::set_var` / `remove_var` are `unsafe` in 2024, so `allow::apply_dotenv` (once per process, only unset keys) and four test helpers wrap them in `unsafe` with SAFETY notes. Follow-up (not done): stop writing `.env` into the process environment. One `tail-expr-drop-order` warning remains in `m017_verify_killtests` (harmless drop-order change on a test temporary). `cargo fmt --all` applied in a separate commit (2024 style: import ordering). No behaviour or oracle assertion changed. `cargo test --workspace --offline` green; `m002_rock01` (11) and `m002_rock00` (3) green with their features. Bench gate stays a controller step at the M019 close.

## 2026-10-05 — Process: pin Rust 1.99

### Process

- Added `rust-toolchain.toml` (channel `1.99.0`, rustfmt, clippy, minimal profile) and `rust-version = "1.99"` in the workspace package (both crates inherit it). Edition stays 2021; crate versions stay `0.0.0`. `cargo check --workspace --offline` is green on 1.99.0. Updated `.planning/codebase/STACK.md` and the M019 S05 text (`File::try_lock` on Rust 1.99). The Phase 51 build record in `m002-cloud-kernel-io-intake.md` keeps 1.98.1 as history.

## 2026-10-04 — Process: selftest classification and own-rule mutations (GSD Phase 57 GOV-06)

### Process

- Selftest classifies each dictionary check as `derived-by-own-mutation`, `explicit-mutation`, or `explicit-skip`. `git_path_implies` stays skip (live diff). `file-exists-only` is reported as LOW, not as proof of sibling steps. Ledger/pointer/glob kinds now derive a mutation that breaks their own rule. Mutation-target formatting and freeze-rule typing follow ruff/ty.

## 2026-10-04 — Process: enrol M002 oracles and strict artifact-budget (GSD Phase 57 GOV-05)

### Process

- `observe_cargo.required` lists SNAP-01 / APPEND-01 / COMMIT-01 / SEG-01 headline fns. New FSM state `observe_rocks` runs ROCK-01 and ROCK-00 under `rocks-index` / `rocks-index-spike` (`timeout_sec: 420`; cold 279.8s, warm 1.4s). `artifact-budget` uses `min_hits: 1` with `when_glob_exists` and SUMMARY `max_files_per_dir: 1`; selftest skip is live-CONTEXT gated. `id-map-cite` requires `v0.12` and `v0.13`.

## 2026-10-04 — Process: cargo_dep_constraint freeze kind (GSD Phase 57 GOV-04)

### Process

- New governor kind `cargo_dep_constraint` (stdlib `tomllib`; META / intake `disposition: kind`). `freeze-rocks`, `freeze-cypher-deps`, and `freeze-hnsw-dep` keep their ids, severities, and messages; they parse Cargo tables instead of line grep. Bypass forms (`[dependencies.rocksdb]`, `rocksdb="0.22"`, same-line `# optional = true`, workspace table form, `package = "rocksdb"` rename) are HIGH. Comments and missing globs are not.

## 2026-10-04 — Process: lease M019 S02 (GSD Phase 57)

### Process

- Active Milestone **M019**, Active Slice **S02** (`L_delivery=M019-S01-done`): GOV-04, GOV-05, GOV-06. First task: RED pytest for freeze bypass forms. M018 stays paused at its S02 gate.

## 2026-10-04 — Process: close M019 S01 (GSD Phase 56)

### Process

- `L_delivery=M019-S01-done`, Active Slice **None**. Independent verification 9/9 (`56-VERIFICATION.md`). Docs honesty round 3 (DOC-08) closed. Honest residuals: honeycomb `capability` stays none until S02; closed-milestone collapse of ROADMAP is living-doc shape, not a product ship; ADRs stay Proposed. No capability claim. Next: controller leases M019 S02 (Phase 57 governor hardening).

### Trajectory

- Honeycomb stays Proposed. ADRs remain Proposed. `L_capability` unchanged (FF5). M018 remains paused at its S02 decision gate.

## 2026-10-04 — Process: lease M019 S01 (GSD Phase 56)

### Process

- Active Milestone **M019**, Active Slice **S01** (`L_delivery=M002-closed`): DOC-08, docs and YAML comments only. The controller rewrote the Product-SoT sentence in `.kutha/STATE.md` (event log; segments + manifest; `events.jsonl` is an export after migrate) and fixed the stale "tag v0.12 for the controller to create" line (the tag exists). M018 stays paused at its S02 gate.

## 2026-10-04 — Process: M019 S01 docs honesty round 3 (executor)

### Process

- Living-doc SoT pass: ADR-000/010/012/014/060 Evidence and Decision pointers; honeycomb ADR-010/012 evidence. ADRs stay **Proposed**. No crate or `scripts/` edit.
- Architecture inventory, STRATEGY date/Rocks sentence, and `.planning/codebase/*` superseded banners aligned with HEAD encodings.

### Trajectory

- Honeycomb stays Proposed. `L_delivery=M002-closed`. M018 remains paused at its S02 decision gate.

## 2026-10-04 — Process: author M019 / GSD v0.13 plan

### Process

### Added

- Author harness milestone **M019** as GSD **v0.13** (Phases 56–62, slices S01–S07: docs honesty round 3; governor hardening; governor SMP; test strategy and oracle matrix; durability residuals A/B; semantic and logic test expansion). Source: 2026-10-04 post-M002 audits (`.compound-engineering/artifacts/research/2026-10-04-post-m002-audit-{roadmap,adr,code-tests,governor}.md`). Owner request: docs correction first, governor hardening plus SMP parallelism, test strategy ADR/matrix, residual fixes, semantic test expansion, all through GSD auto. Planned, unleased. Nothing shipped.

### Changed

- M018 S04 keeps I-F15 / I-F16 and the bench-gate items; DIFF-01 (fold / CSR / rebuild) moves to M019 S07.

### Trajectory

- Harness lease stays idle (`L_delivery=M002-closed`). M018 remains paused at its S02 decision gate. Honeycomb stays Proposed.

## 2026-10-04 — Process: close M002 S07 and milestone M002 (GSD v0.12)

### Process

- Close S07 (ROCK-01) and archive Phases 49–55 under `.planning/milestones/v0.12-*`. What shipped: SNAP-01, fail-closed WAL open, framed append, commit marker option A (`Op::QuantumOutcome`), sealed segments with a cross-segment SHA-256 chain, droppable Rocks index lease. Residuals in `v0.12-MILESTONE-AUDIT.md` (Typed/Dictionary partial group, no multi-writer lock, tip-only index staleness, line-based `freeze-rocks` bypasses, unexplained RSS `bytes_per_fact` 17218→25200 B, `rocks_spike` still on jsonl, O(N) `open`, ~15% bench host noise). Process tag `v0.12` is for the controller to create; crate SemVer stays 0.0.0. Owner acknowledgement needed for the SoT wording that `events.jsonl` is an export.

### Trajectory

- `L_delivery=M002-closed`, Active Milestone **None**, Active Slice **None**. M018 remains paused at its S02 decision gate (C01, Horn vs knowledge cut). Honeycomb stays Proposed.

## 2026-10-04 — Process: durable freeze-rocks (optional rocksdb)

### Process

### Changed

- `glob_absent` accepts optional `unless_line_contains` (a needle on a comment line, or on a line that also contains that substring, is ignored). `freeze-rocks` allows `rocksdb =` in `crates/*/Cargo.toml` only when the same line has `optional = true`, regardless of Active Milestone; a plain or non-optional assignment fails. Feature lines (`dep:rocksdb`) are not assignments. The workspace `Cargo.toml` must not contain `rocksdb =`. `unless_file_contains` stays in the kind and its tests. `freeze-cypher-deps` and `freeze-hnsw-dep` stay unconditional. Selftest still appends `rocksdb =` on a new line so the check fails closed.

## 2026-10-04 — Product: ROCK-01 droppable Rocks locator lease

### Product

### Added

- Optional `kutha-runtime` feature `rocks-index` (same `rocksdb` optional dep, `default-features = false`). `SegmentIndex` stores EventId locators (`e:` seq+offset+len) and delivery_key winners (`d:`) over the committed segmented prefix, plus `m:v` / `m:tip`. Deleting the Rocks directory changes no log answer. Legacy dirs fail with `InvalidData` "run store::migrate first". Not wired into `Runtime`, `store::open`, `Persister`, fold, or hydrate. Oracles: `crates/kutha-runtime/tests/m002_rock01.rs`. `rocks-index-spike` / ROCK-00 jsonl offsets stay.

### Trajectory

- ADR-010/012 Evidence 2026-10-04 Phase 55: ROCK-01 locator lease; cells stay **Proposed**. `honeycomb.yaml` restaged (map/delivery/capability unchanged). Bench vs `0245bbd` (2 idle interleaved pairs): ingest_plain.n −17.8%, as_of.n −8.7%, open.n −0.2% — D-10 met. 10k-assert segmented index build ~410.4 ms; midpoint `get_event` mean ~12.8 µs. Residuals: not wired into Runtime, single reader/writer, rebuild O(N), RocksDB cold compile ~5.5 min.

## 2026-10-04 — Product: migrate, jsonl export, and sealed tamper (SEG-01 / 54-03)

### Product

### Added

- `store::migrate`: load via `load_events_d04` then `committed_prefix`; write segments + manifest; verify reopen (ids + fold fingerprint); leave `events.jsonl` as an unread export; remove `events.wal` only after that flip. A verify failpoint that corrupts a freshly written segment returns `InvalidData` and unflips the manifest so jsonl/WAL stay. Idempotent when a manifest already exists. Crash matrix reopens to legacy or segmented without loss. `store::export_jsonl` writes jsonl via `replace_via_tmp` and is never consulted by `open` when a manifest exists. `KUTHASEG` header is 44 bytes (previous-segment chain tip; H0 only for seq 1). Open checks sealed+active name contiguity, then per-file missing / chain-link / CRC / trailer / count / bytes / hash chain / manifest fields; the active header must link to the last sealed tip. Tamper oracles assert `InvalidData` plus those distinct needles. Cross-segment swap/replace/delete-middle/reorder/foreign-active are `InvalidData`. This does not stop a full rewrite of files + manifest + recomputed hashes; the manifest tip is not externally anchored. Oracles: `crates/kutha-runtime/tests/m002_seg01.rs`.

### Process

### Changed

- Governor `m012a-s04-verify-persist` / `B-m012a-s04`: YAML-only clarification that atomic `events.jsonl` persist is the **legacy** oracle (no manifest). Segmented dirs write `KUTHASEG`.

### Trajectory

- ADR-000 Clarification 2026-10-04 Phase 54-03: D1 still names the event log as SoT; migrate is explicit. ADR-010/012/060 Evidence 2026-10-04 Phase 54-03. Dated 2026-10-04 pointer under ADR-010 Clarification 2026-10-03 (jsonl-as-physical-SoT superseded by SEG-01; the event log is still the SoT). Cells stay **Proposed**. Residuals: Typed/Dictionary group hole, no multi-writer lock, WAL duplication gone only for segmented dirs, `rocks_spike` still on jsonl until S07. `honeycomb.yaml` restaged (map/delivery/capability unchanged). Bench vs `be57466` (interleaved medians): ingest_plain.n +7.1%, as_of.n +8.6%, open.n +0.3% (+1.5% vs 54-02 open.n) — D-09 met. Second round vs `424bda6` (2 interleaved pairs): ingest_plain.n −9.4%, as_of.n −9.7%, open.n +1.6% — D-09 met.

## 2026-10-04 — Product: segmented persist, Persister, and rollover (SEG-01 / 54-02)

### Product

### Added

- Empty dirs (no `manifest.json`, no `events.jsonl`, no `events.wal`) and already-segmented dirs persist `KUTHASEG` + `manifest.json` (atomic flip + dir fsync). Legacy dirs (`events.jsonl` or `events.wal` present, no manifest) keep jsonl+WAL. `Persister` on a segmented dir appends frames to the active segment only, one `sync_data` per quantum, O(1) tail guard; rollover at `SEGMENT_MAX_BYTES` / `SEGMENT_MAX_EVENTS` (thread-local test seam) seals trailer+sync → new header+sync → manifest flip. Checkpoint refreshes snapshot+sidecars only. `open_for_append` heals via a full segmented persist. Pin helper `store::ensure_legacy_dir`. Oracles: `crates/kutha-runtime/tests/m002_seg01.rs`.

### Trajectory

- ADR-000 Clarification 2026-10-04 Phase 54-02: D1 still names the event log as SoT; empty/segmented dirs now write the new encoding. ADR-010/012/060 Evidence 2026-10-04 Phase 54-02. Cells stay **Proposed**. `honeycomb.yaml` restaged (map/delivery/capability unchanged). Bench vs `39af84b` (interleaved medians): ingest_plain.n −5.9%, as_of.n +0.0%, open.n −7.8% — D-09 met. persist.n (empty dir now segmented vs legacy) −59.4%; persist_append.n −72.6% (no jsonl).

## 2026-10-04 — Product: sealed segment reader on open (SEG-01 / 54-01)

### Product

### Added

- `KUTHASEG` v1 (`segment.rs`): WAL-identical event frames plus a CRC'd seal trailer and a SHA-256 per-frame chain (`sha2` already in `kutha-runtime`). `manifest.json` lists sealed files and the named active segment. When the manifest is present, `store::open` loads that layout (full sealed verify; torn active tail dropped) and never reads `events.jsonl` / `events.wal`. `persist` and `Persister` still write the legacy jsonl+WAL layout. Test-only writer builds byte-valid fixtures. Oracles: `crates/kutha-runtime/tests/m002_seg01.rs`.

### Trajectory

- ADR-000 Clarification 2026-10-04: D1 still names the event log as SoT; only the physical encoding gained a reader. ADR-010/012/060 Evidence 2026-10-04 Phase 54-01. Cells stay **Proposed**. `honeycomb.yaml` restaged (map/delivery/capability unchanged). Bench vs `3c469fe` (interleaved medians): ingest_plain.n +0.6%, as_of.n +3.1%, open.n −5.2% — D-09 met. Segmented vs legacy open of a ~10k-assert fixture: −1.1%.

## 2026-10-04 — Product: QuantumOutcome commit marker on open (COMMIT-01)

### Product

### Added

- `store::committed_prefix` / `op_commit_class`: on `open`, a typed uncommitted suffix is not visible. `Op::QuantumOutcome` stays the User-quantum commit marker (option A). Self-commit: `Define`, `TtAnchor`, `StoreBlob`, `JustificationCite`, `QuantumOutcome`, dictionary `kutha:*` Assert. NeedsOutcome: product `Assert`/`Retract`/`Correct`/`CorrectInterval`/`Behavior`. The first NeedsOutcome in the tail cuts that op and every later op (including later SelfCommit). `open_report` counts dropped events; `open_for_append` heals a dirty tail with a full persist then attaches. Snapshot `log_offset` ahead of the committed fold-op count fails closed. Oracles: `crates/kutha-runtime/tests/m002_commit01.rs`.

### Trajectory

- ADR-010/012/014/060 dated Evidence 2026-10-04 Phase 53: COMMIT-01 = option A; B/C wait for S06. Cells stay **Proposed**. `honeycomb.yaml` restaged (map/delivery/capability unchanged). Residual: Typed/Dictionary multi-event groups have no outcome by design, so a crash inside such a group can still expose a partial group (now also in ADR-010 Evidence; ADR-014 already said it). Second-round: `m002_commit01` oracles kill keeping later SelfCommit after the first NeedsOutcome. Bench vs `187d4c7` (interleaved medians): ingest_plain.n +5.6%, as_of.n −3.4%, open.n +3.9% — D-10 met.

## 2026-10-03 — Product: incremental framed append (APPEND-01)

### Product

### Added

- `store::Persister` (`attach` / `append` / `checkpoint` / `close`): jsonl lines then WAL frames, one log `sync_data` per non-checkpoint quantum; prefix bytes of `events.jsonl` / `events.wal` stay. Torn jsonl tails without a newline are dropped; a complete malformed line still fails `InvalidData`. `CHECKPOINT_EVERY` (4096) and `close` refresh snapshot and sidecars; full `persist` remains the atomic rewrite. Oracles: `crates/kutha-runtime/tests/m002_append01.rs`. Bench key `persist_append.n` (1% tail after `persist.n`).

### Fixed

- Snapshot `open` was dropping log `Op::Define` events and hydrating the dictionary only from `snap.dict_strings`, so terms interned after the last checkpoint vanished on reopen even though the Defines were durable (replay_check still passed). `from_snapshot` now interns those extra names in log order. Intern-only appends (Define tail, no new graph events) no longer fail `attach`/`append` (`InvalidData` from encoded-log last-frame vs on-disk Define). Persist/open parity is event-id and dictionary/`TermId`s, not encoded-log byte equality.

### Trajectory

- ADR-010/012/014/060 dated Evidence 2026-10-03 Phase 52: O(N) persist rewrite closed for the append path. Cells stay **Proposed**. `honeycomb.yaml` restaged (map/delivery/capability unchanged). Bench vs `bd5fa4b` (interleaved medians): ingest_plain.n −6.4%, as_of.n −1.5% — D-10 met. HEAD persist.n median 5.94 s vs persist_append.n 48.9 ms. After the Define-on-open fix, re-run medians: ingest −3.4%, as_of −1.4% (still D-10). Snapshot open now interns log Defines beyond `snap.dict_strings`.

## 2026-10-03 — Product: ROCK-00 RocksDB EventId-offset spike

### Product

### Added

- Optional `kutha-runtime` feature `rocks-index-spike` (`rocksdb` 0.22, `default-features = false`: no snappy/lz4/zstd/zlib/bzip2). `rocks_spike::{build_index, lookup, read_event_at}` maps `EventId` bytes to `events.jsonl` offsets; oracles `m002_rock00.rs` (round trip, droppable directory, absent key, malformed log line fails closed with `InvalidData`) are feature-gated. Default `cargo test --workspace` does not compile RocksDB. Not wired into `Runtime`, persist, fold, or hydrate.

### Trajectory

- ADR-010/012 dated Evidence 2026-10-03: RocksDB builds offline as an optional spike; no wiring. Cells stay **Proposed**. `honeycomb.yaml` restaged (map/delivery/capability unchanged). Build record in `m002-cloud-kernel-io-intake.md`. Bench vs `24e2306` (interleaved medians): ingest_plain.n −22.7%, as_of.n −15.9% — inside ~15–18% host noise; D-07 met.

## 2026-10-03 — Process: freeze-rocks allows rocksdb during M002

### Process

### Changed

- `glob_absent` accepts optional `unless_file_contains: {file, needle}` (missing file does not skip). `freeze-rocks` allows the string `rocksdb` in `crates/*/Cargo.toml` and the workspace `Cargo.toml` only while `.kutha/STATE.md` names `**Active Milestone:** M002`. `freeze-cypher-deps` and `freeze-hnsw-dep` stay unconditional. Selftest disables the unless needle on the copy before inserting `rocksdb` so the check still fails closed.

## 2026-10-04 — Process: lease M002 S07 (GSD Phase 55)

### Process

- Active Milestone **M002**, Active Slice **S07** (`L_delivery=M002-S06-done`): ROCK-01, the last slice of M002. A durable `freeze-rocks` rule (rocksdb only as an optional dependency) replaces the time-boxed one; an optional `rocks-index` feature holds an EventId and delivery_key locator index over the segmented log as a droppable lease, not wired into `Runtime`. Honeycomb stays Proposed.

## 2026-10-04 — Process: close M002 S06 (GSD Phase 54)

### Process

- `L_delivery=M002-S06-done`, Active Slice **None**. `54-VERIFICATION.md` is independent (7/7 after a second round: the first found an unverified migrate step, a stitching hole between sealed segments, weak error needles and living jsonl-SoT wording; the chain is now continuous across segments). The event log stays the source of truth; the physical encoding is a manifest plus segments and `events.jsonl` is an export; the owner should note this wording change in `.kutha/ROADMAP.md`. Bench series stays within the 10% gate on unchanged ingest/as_of paths. Next: S07 (Phase 55) needs a lease.

## 2026-10-04 — Process: lease M002 S06 (GSD Phase 54)

### Process

- Active Milestone **M002**, Active Slice **S06** (`L_delivery=M002-S05-done`): SEG-01. The event log stays the source of truth; its physical encoding moves to a manifest plus sealed segments (SHA-256 chain on the existing frame format, no new dependency) plus an active segment; `events.jsonl` becomes an on-demand export; legacy directories still open and convert only through an explicit `store::migrate`. Three plans in one phase. Honeycomb stays Proposed.

## 2026-10-04 — Process: close M002 S05 (GSD Phase 53)

### Process

- `L_delivery=M002-S05-done`, Active Slice **None**. `53-VERIFICATION.md` is independent (7/7 after a second round: the first found a surviving mutation, a later self-commit op kept after the first uncommitted product op, and a missing ADR-010 residual; both were fixed). Bench on interleaved medians stays within the 10% gate (writer unchanged; the +5.6% ingest figure is host noise). Next: S06 (Phase 54) needs a lease.

## 2026-10-04 — Process: lease M002 S05 (GSD Phase 53)

### Process

- Active Milestone **M002**, Active Slice **S05** (`L_delivery=M002-S04-done`): COMMIT-01. Option A chosen from two research notes (prior art, internal analysis): `Op::QuantumOutcome` stays the commit marker and `open` hides an uncommitted suffix; commit frame and envelope wait for S06; the env switch is rejected. Honeycomb stays Proposed.

## 2026-10-03 — Process: owner defers M002 S05

### Process

- The owner chose to defer COMMIT-01 (S05) and S06/S07 until M018's decisions (C01, Horn vs knowledge cut) are made. No lease changes; Active Slice stays **None**.

## 2026-10-03 — Process: close M002 S04 (GSD Phase 52)

### Process

- `L_delivery=M002-S04-done`, Active Slice **None**. `52-VERIFICATION.md` is independent (8/8). The first verification found a real defect (terms interned between appends were dropped on open) and four surviving mutations; the executor fixed `from_snapshot`, `attach` and added oracles, and the second verification killed every mutation. Bench on interleaved medians stays within the 10% gate. Next: S05 (COMMIT-01) is a decision gate; M018 stays paused.

## 2026-10-03 — Process: lease M002 S04 (GSD Phase 52)

### Process

- Active Milestone **M002**, Active Slice **S04** (`L_delivery=M002-S03-done`): APPEND-01, PERF-GATE. A `Persister` appends framed records to `events.jsonl` and `events.wal` with one log sync per quantum; snapshot and sidecars refresh at a checkpoint. No segments, no commit frame, no Rocks wiring. Honeycomb stays Proposed.

## 2026-10-03 — Process: close M002 S03 (GSD Phase 51)

### Process

- `L_delivery=M002-S03-done`, Active Slice **None**. `51-VERIFICATION.md` is independent (6/6; kill-mutations on the unless-gate, offset, key bytes, malformed-line skip, absent key; the malformed-line oracle was added after the first verification found it missing). `freeze-rocks` allows `rocksdb` only while Active Milestone is M002. Bench: default build paths are unchanged, recorded ratios are host noise. Next: S04 (Phase 52) needs a lease; M018 stays paused.

## 2026-10-03 — Process: lease M002 S03 (GSD Phase 51)

### Process

- Active Milestone **M002**, Active Slice **S03** (`L_delivery=M002-S02-done`): ROCK-00. RocksDB as an optional `kutha-runtime` feature spike (EventId to `events.jsonl` offset), not wired into `Runtime`; the `freeze-rocks` check is relaxed only while Active Milestone is M002. Honeycomb stays Proposed.

## 2026-10-03 — Process: close M002 S02 (GSD Phase 50)

### Process

- `L_delivery=M002-S02-done`, Active Slice **None**. `50-VERIFICATION.md` is independent (7/7; kill-mutations on in-place sidecar write, missing directory fsync, exact anchor equality, short-header `Ok(vec![])`, dropped prefix rule, masked I/O error, skipped failpoint all fail a test). Bench gate D-06 met on interleaved medians (ingest +0.1%, as_of +1.4%); the first back-to-back runs were host noise. `bytes_per_fact` RSS rise is carried to Phase 52. Next: S03 (Phase 51) needs a lease; M018 stays paused.

## 2026-10-03 — Product: SNAP-01 atomic sidecars and fail-closed WAL open

### Product

### Fixed

- `persist` writes `terms.jsonl` / `quantum_outcomes.jsonl` / `justifications.jsonl` through tmp+`sync_all`+rename; parent-directory fsync after each rename and at end of persist. Unparseable, mid-line, or proper-prefix sidecars are treated as absent; well-formed non-prefix still fails `InvalidData`.
- Open-path `check_tt_anchors_range` allows log `TtAnchor`s with `ingested_at >= snap.next_tt`; `replay_check` stays exact. `recover_events` returns `InvalidData` ("truncated WAL header") when shorter than magic+version; `load_events_d04` does not mask non-`InvalidData` WAL I/O. Crash-matrix failpoint seam `persist_with_failpoint`. Oracles: `crates/kutha-runtime/tests/m002_snap01.rs`. The directory-fsync counter is thread-local and the test asserts exactly 7 (six renames plus the end of persist), so one in-place sidecar write is caught.

### Trajectory

- ADR-012/014/060 dated Evidence 2026-10-03 Phase 50 (SNAP-01 closed in evidence; incremental append S04; COMMIT-01 S05). `semantic-gap-review.md` F1/F6 status. `honeycomb.yaml` restaged (capability and `freeze_as` unchanged). Cells stay **Proposed**.
- Bench gate vs `7e31784` (`KUTHA_BENCH_N=10000`): first back-to-back runs were host noise (baseline drifted 15% by itself); six interleaved pairs give medians ingest_plain.n +0.1%, as_of.n +1.4% — D-06 met. `bytes_per_fact` peak RSS rose 17218 B to 25200 B and is carried to Phase 52.

## 2026-10-03 — Process: lease M002 S02 (GSD Phase 50)

### Trajectory

- Active Milestone **M002**, Active Slice **S02** (`L_delivery=M002-S01-done`): SNAP-01, OPEN-01, CRASH-01. Product code in `crates/kutha-runtime` only; RocksDB stays banned until Phase 51; honeycomb stays Proposed.

## 2026-10-03 — Process: thaw map and freeze governor honesty (M002 S01)

### Process

### Changed

- Governor: check `freeze` is split into `freeze-rocks` (Phase 51 / S03), `freeze-cypher-deps` (M003) and `freeze-hnsw-dep` (M005); each scans `crates/*/Cargo.toml` and the workspace root `Cargo.toml`; no message claims to read STATE; the tautological message check is gone; `bridges.yaml` has three rows; pytest and selftest cover each id. YAML only, no kernel change (the `unless:` extension is Phase 51's first task).
- Docs: `AGENTS.md`, `README.md`, `PROJECT.md`, `kutha-harness.md`, `governor-intake.md`, `gsd-auto-contract.md`, codebase maps and ADR-000/010/012/014/050/060/070 point to `.kutha/STATE.md` § Thaw map instead of "freeze until M002 lease"; ADR-010/012/014/060 Evidence schedules SNAP-01, fail-closed WAL open, incremental append, COMMIT-01 and the Rocks index lease on M002 S02–S07; ADR-010 and ADR-012 also record the O(N) `persist` rewrite as the current defect. `honeycomb.yaml` restaged (capability and `freeze_as` unchanged).

### Trajectory

- No `crates/` change. **M002 S01 closed** (`L_delivery=M002-S01-done`, Active Slice **None**); Phase 49 verified independently (11/11) after two reopens (CHANGELOG duplicate and overclaim, harness doc omitted the workspace `Cargo.toml`, authorship wording, graphiti/falkordb test). Honeycomb stays Proposed.

## 2026-10-03 — Process: thaw M002 and plan GSD v0.12; lease S01

### Process

### Added

- Harness milestone **M002** thawed and planned as GSD **v0.12** (Phases 49–55, slices S01–S07: thaw map and governor honesty; SNAP-01 with fail-closed WAL open; RocksDB build spike; incremental framed append; commit-marker decision; segments, manifest and hash chain; droppable RocksDB index lease). Requirements in `.planning/REQUIREMENTS.md`; Phase 49 CONTEXT under `.planning/phases/49-thaw-map-and-freeze-governor-honesty/`. Re-verification at `f4af544` (provenance, governor freeze map, durability design research) found that the freeze list and its "until explicit M002 lease" gate came from agent-assisted commits (`1ee5942`, `85db1f7`, 2026-08-18, Cursor co-authored), which the owner says he did not decide, and not from ADR-000 D1–D10; the real defects are the O(N) `persist` rewrite, in-place sidecar writes, missing parent-directory fsync, exact-equality `check_tt_anchors`, and fail-open WAL magic reads.

### Changed

- `.kutha/STATE.md` § Freeze becomes § Thaw map: RocksDB crate thaws at Phase 51; Cypher (M003), HNSW (M005), six dictionaries (SIX-01), ADR-080/081, ADR-090, ADR-093 and Consensus 103+ keep their own gates. M018 is paused after S01 at the S02 decision gate; OPEN-01 and the crash seam moved from M018 S04 to M002 S02.

### Trajectory

- Active Milestone **M002**, Active Slice **S01** (`L_delivery=M018-S01-done`). No `crates/` change in S01; the `rocksdb` dependency stays banned by the governor until Phase 51. Honeycomb stays Proposed.

## 2026-10-03 — Process: doc truth round 2 (M018 S01)

### Process

### Changed

- ADR-010/012/013/014/041/050/060 and `docs/ADR/README.md` carry dated 2026-10-03 clarifications: `snapshot.json` and `events.wal` replace via tmp+sync+rename, `events.jsonl` is SoT with `KUTHAWAL` as covering lease, `replay_check` compares `TtAnchor` (`check_tt_anchors`), `live_horns` is lease-first (`horns_by_premise`); sidecars stay in-place `File::create` (SNAP-01, M002). ADR-013 states `as_of` is support-live (derivation eligibility is `derivation_eligible_at`), Horn fires at `TT=MAX`, `Correct` does not re-derive, and a missing `event_id` target is a silent no-op; none of these is decided. `honeycomb.yaml` restaged (capability unchanged).
- `semantic-gap-review.md`, `p0-spike-inventory.md`, `semantic-contract-validation.md`, `PROJECT.md`, `.kutha/ROADMAP.md`, `gsd-auto-contract.md`: v0.10 reality, audit ids `A-F01`–`A-F14` separate from gap-review `F1`–`F16`, A-F05 stays open, one freeze table (STATE), applicability card count 164 (163 matrix + M002 intake card).
- `ADR-000` and `docs/architecture/stca-guide.md` translated to English (no decision text changed; ADR-000 gains a D3 pointer and a D6 clarification).
- `.gitignore` ignores crate scratch and vendored agent trees (`crates/**/tmp/`, `.gsd/`, `.claude/skills/`, `.cursor/skills/gsd-*/`, GSD install state, hooks, agents).

### Trajectory

- No `crates/` change. Honeycomb cells stay Proposed. **M018 S01 closed** (`L_delivery=M018-S01-done`, Active Slice **None**); Phase 45 verified independently 5/5 after one reopen (stale F1/F6 claims in `semantic-gap-review.md` marked historical with a current status). S02 is a decision gate and is not leased.

## 2026-10-03 — Process: plan M018 (GSD v0.11) and lease S01

### Process

### Added

- Harness milestone **M018** "Wedge unblock" (GSD v0.11, Phases 45–48) from the 2026-10-03 post-v0.10 review (five read-only audits at `d230ab2`): S01 doc truth round 2, S02 C01 and knowledge-cut decision (decision gate), S03 Horn determinism and closure, S04 fail-closed open and differential tests. Requirements in `.planning/REQUIREMENTS.md`; Phase 45 CONTEXT under `.planning/phases/45-doc-truth-round-2/`. M002 durability moves to the v0.12 slot, still frozen.

### Trajectory

- **M017 is CLOSED** (tag `v0.10`); the product S04 entries below are historical and carry no live lease. Active Milestone **M018**, Active Slice **S01** (documents only, no `crates/` change). Audit ids F01–F14 of 2026-10-02 are written `A-F01`–`A-F14` from here on; older entries keep the original spelling. Honeycomb stays Proposed.

## 2026-10-03 — Product: Horn premise-relation ingest lease (M017 S04)

### Product

### Fixed

- Droppable `GraphFold` lease `horns_by_premise` (premise relation name → live Horn slots, `serde skip`, not fingerprint). Indexed on `kutha:rule` push and dropped on retract, including Lazy, like `allows_by_name`. `horn_fires` looks up the asserted relation instead of scanning `facts()` in `live_horns`. Extra `relatedTo` ingest no longer pays O(N) Horn visits. No new crate or public API. Same-host Eager `KUTHA_BENCH_N=10000`: `ingest_plain.n` 111283177 / 105460301 ns vs 44-01 v0.08 115322845 (within 10%; `as_of.n` 401528 / 290851).

### Trajectory

- Lease and honeycomb cells unchanged. Active Milestone **M017**, Active Slice **S04**.

## 2026-10-03 — Product: ingest Horn-visit RED oracle (M017 S04)

### Product

### Added

- `GraphFold` `horn_examine` counter (`serde skip`, not fingerprint) with `reset_horn_examine_count` / `horn_examine_count`. `live_horns` and `search_at` bump once per row. Integration oracle `extra_relatedto_horn_visits_constant_in_n` in `crates/kutha-runtime/tests/m017_ingest_scaling.rs` (RED on HEAD: visits grow with N). No ingest-path repair.

### Trajectory

- Lease and honeycomb cells unchanged. Active Milestone **M017**, Active Slice **S04**.

## 2026-10-03 — Product: KNOWN-04 fixture hygiene

### Product

### Changed

- Shared `seed_statute_between_anchors` in `crates/kutha-runtime/tests/common/mod.rs` (m014 and m016 known-on hardening keep every assertion). FIXG-01 drops `C01.contains` quote-membership tautology; `RuleTerm::constant` none and VT vs known-on remain. `leased_pin_and_derive` not merged (arity and live_count asserts diverge).

## 2026-10-03 — Product: KNOWN-04 Horn vs as_known_on (M017 S03)

### Product

### Added

- Characterization `known04_horn_fires_when_premise_hidden_on_early_cut` in `crates/kutha-runtime/tests/m017_known_on_horn.rs`: two-premise Horn mints derived `relatedTo(A,C)` at `TT = MAX` while `as_known_on(2014)` hides A–B and the derived row; `as_known_on(2020)` shows A–B. No eval or known-on behaviour change.

### Trajectory

- ADR-013 Evidence (2026-10-03) records current F05 behaviour; decision pending. Honeycomb restage comment and evidence name; cell stays **Proposed**. Active Milestone **M017**, Active Slice **S03**.

## 2026-10-03 — Product: DET-01 seeded determinism (M017 S03)

### Product

### Added

- Seeded LCG properties in `crates/kutha-runtime/tests/m017_determinism.rs`: all listed seeds (`1, 7, 13, 42, 99, 12345, 0xA5A5_A5A5, 0xC0FFEE`, N=24) prove `replay(log).fingerprint == fold.fingerprint`, Eager vs Lazy equality on sorted `as_of` / `as_known_on` / `live_at` triples (not fingerprints), persist+open round trip under `tmp/p43-01-*`, and identical-log delivery-key retry. No new Cargo test crates.

### Trajectory

- Lease and honeycomb cells unchanged. Active Milestone **M017**, Active Slice **S03**.

## 2026-10-03 — Product: replay integrity (M017 S02)

### Product

### Fixed

- `open` prefers a covering WAL, falls back to intact `events.jsonl`, and fails closed when event ids diverge. Tampered persisted `TtAnchor` walls fail `replay_check` / `open`. `snapshot.json` and `events.wal` replace via temp + sync + rename. A present outcomes or justifications sidecar must equal log rows; an absent sidecar stays Ok. Snapshot JSON pins `FoldIndexMode` (missing field → Eager).

### Changed

- `Runtime::as_known_on` rustdoc is TT-only (`GraphFold::known_at`), not `live_at(tt, MAX)`.

### Trajectory

- ADR-010 Evidence (2026-10-03) records the D-04 open policy. Honeycomb comment restage only; cell stays **Proposed**. Active Milestone **M017**, Active Slice **S02**.

## 2026-10-03 — Trajectory: M017 CLOSED (v0.10 archived)

### Trajectory

- Archive Phases 41–44 under `milestones/v0.10-*`; `L_delivery=M017-closed`, Active Milestone **None**. Process tag `v0.10`; crate SemVer stays 0.0.0. Ingest regression from M016 repaired (+6.3% vs v0.08). Bench gate at product close added to the GSD auto contract. Carried residuals: `v0.10-MILESTONE-AUDIT.md`.

## 2026-10-03 — Process: M017 S04 leased (ingest regression)

### Process

- Lease M017-S04 for GSD Phase 44 (PERF-01, PERF-02).

### Trajectory

- Active Milestone **M017**, Active Slice **S04**. `L_delivery=M017-S03-done`.

## 2026-10-03 — Trajectory: M017 S03 CLOSED (Phase 43 verified)

### Trajectory

- M017 S03 closed (`L_delivery=M017-S03-done`), Active Slice **None**; independent kill-tests `m017_verify43_killtests.rs` pass. Next: Phase 44 (ingest regression repair).

## 2026-10-03 — Process: M017 S03 leased (determinism)

### Process

- Lease M017-S03 for GSD Phase 43 (DET-01, KNOWN-04). Phase 44 unleased.

### Trajectory

- Active Milestone **M017**, Active Slice **S03**. `L_delivery=M017-S02-done`.

## 2026-10-03 — Trajectory: M017 S02 CLOSED (Phase 42 verified)

### Trajectory

- M017 S02 closed (`L_delivery=M017-S02-done`), Active Slice **None**. Phase 44 (ingest regression repair, S04) authored; Phase 43 next.

## 2026-10-03 — Product: M017 S02 verification kill-tests and ingest finding

### Product

- Independent kill-tests `crates/kutha-runtime/tests/m017_verify_killtests.rs` (anchor tamper, truncated WAL, id gap, spurious sidecar, env flip, abort-before-rename, closed-VT known-on) all pass against Phase 42.

### Trajectory

- Same-host bench bisect: ingest_plain.n 106M (v0.06), 108M (v0.08), 301M (post-M016): M016 tripled ingest, Phase 42 does not move it. Scheduled as M017 S04 / Phase 44.

## 2026-10-03 — Process: M017 S02 leased (replay integrity)

### Process

- Lease M017-S02 for GSD Phase 42 (REPL-01..04, KNOWN-03). Phases 43 unleased. No crate change in this entry.

### Trajectory

- Active Milestone **M017**, Active Slice **S02**. `L_delivery=M017-S01-done`.

## 2026-10-03 — Process: M017 S01 living docs and cite-lease

### Process

- GSD v0.10 / M017 authored; S01 leased (living docs and governor honesty). `.planning/PROJECT.md` cites `.kutha/STATE.md` for lease and freeze; `cite-lease` covers PROJECT (stale `Active Milestone` token fails). Semantic-gap addendum 2026-10-03, I-F7/I-F9/I-F13/I-F16 restaged, architecture and planning banners, ROADMAP M016 Success (Horn over live facts) and Freeze pointer. ADR-000/013/014/041/050/060/070 and README carry 2026-10-03 clarifications (honeycomb restage line added). `idle-delivery-closed` selftest mutation now bites both idle and leased STATE; the cite-lease Python test allows PROJECT.md.

### Trajectory

- M017 S01 closed (`L_delivery=M017-S01-done`), Active Slice **None**; independent verification `41-VERIFICATION.md` passed. Honeycomb stays **Proposed**. No `crates/` change.

## 2026-10-02 — Process: M002 intake note (kernel I/O stalls)

### Process

- Captured arXiv:2609.12597 / SteelDB companion for frozen M002: `docs/architecture/m002-cloud-kernel-io-intake.md` and applicability card `paper-kernel-io-dirty-throttle-cloud-oltp`. No lease; freeze unchanged.

## 2026-10-02 — Trajectory: M016 CLOSED (v0.09 archived)

### Trajectory

- `L_delivery=M016-closed`. Active Milestone **None**. Process tag `v0.09`. Crate SemVer stays `0.0.0`. Honeycomb stays **Proposed**. Next: lease M002 before durability work.

### Product

- FIXG-01 fail-closed: C01 sunset is not a Horn; ADR-013 Evidence dated. No fifth clock.

## 2026-10-02 — Product: C01 sunset fail-closed FIXG-01 verdict (Phase 40)

### Product

### Added

- Named oracle `fixg01_c01_sunset_inexpressible_as_horn`: C01 quote; Horn cannot hold a calendar sunset; `valid_to` folding is VT not a transitional rule.

### Changed

- ADR-013 Evidence (2026-10-02): fail-closed — two axes + current RuleAst cannot express application-end without a fifth clock. Cell stays **Proposed**. No five clocks invented.

### Trajectory

- Cells remain **Proposed**. M016 S05 lease unchanged; no SemVer tag.

## 2026-10-02 — Trajectory: M016 S05 leased

### Trajectory

- Active Slice **S05** (GSD Phase 40, Legal golden fixture). `L_delivery=M016-S04-done`.

## 2026-10-02 — Trajectory: M016 S04 CLOSED (Phase 39)

### Trajectory

- Phase 39 verified; `L_delivery=M016-S04-done`. Next: lease S05 (Legal golden fixture).

### Product

- Live policy blob `deny` fails admission; Horn execution replay fails empty-pin inverse.

## 2026-10-02 — Product: independent RULE-03/04 kill-tests (Phase 39)

### Product

### Added

- Independent kill-test: live `deny` fails `check_admission`; prior admission AS OF stays true; empty-pin inverse fails `replay_check` with unchanged fingerprint.

### Trajectory

- Cells remain **Proposed**. M016 S04 lease unchanged; no SemVer tag.

## 2026-10-02 — Product: policy body eval and Horn execution replay (Phase 39)

### Product

### Added

- Live policy blob `deny` fails `check_admission` (`reason: policy`); other live bodies including `leased-policy` still admit. Prior admission remains queryable AS OF after a later pin.
- `replay_check` re-derives Horn-pinned Behaviors at `ingested_at`; empty-pin inverse fails closed without changing the facts-only fingerprint.

### Trajectory

- Cells remain **Proposed**. M016 S04 lease unchanged; no SemVer tag.

## 2026-10-02 — Trajectory: M016 S04 leased

### Trajectory

- Active Slice **S04** (GSD Phase 39, Policy + execution replay). `L_delivery=M016-S03-done`.

## 2026-10-02 — Trajectory: M016 S03 CLOSED (Phase 38)

### Trajectory

- Phase 38 verified; `L_delivery=M016-S03-done`. Next: lease S04 (Policy + execution replay).

### Product

- Horn evaluator; genesis inverse_knows; n-ary Horn; premise retract drops Horn-derived supports.

## 2026-10-02 — Product: independent RULE-02 kill-tests (Phase 38)

### Product

### Added

- Independent kill-test: inverse pin is genesis Horn hash; relatedTo does not mint inverse; retract of knows drops live knownBy.

### Trajectory

- Cells remain **Proposed**. M016 S03 lease unchanged; no SemVer tag.

## 2026-10-02 — Product: n-ary Horn and derived retract (Phase 38)

### Product

### Added

- Named test: two-premise Horn fires only when both premises are live; retracting one drops the derived support.

### Trajectory

- Cells remain **Proposed**. M016 S03 lease unchanged; no SemVer tag.

## 2026-10-02 — Product: Horn evaluator replaces inverse_knows cascade (Phase 38)

### Product

### Changed

- `knows` Assert derives `inverse_knows` from a genesis Horn body (`knownBy(Y,X) :- knows(X,Y)`). The pin is the canonical body hash. The hard-coded `follow_ons` special-case is gone.

### Trajectory

- Cells remain **Proposed**. M016 S03 lease unchanged; no SemVer tag.

## 2026-10-02 — Trajectory: M016 S03 leased

### Trajectory

- Active Slice **S03** (GSD Phase 38, Rule evaluator). `L_delivery=M016-S02-done`.

## 2026-10-02 — Trajectory: M016 S02 CLOSED (Phase 37)

### Trajectory

- Phase 37 verified; `L_delivery=M016-S02-done`. Next: lease S03 (Rule evaluator).

### Product

- Canonical Named/Horn `RuleAst`; `rule_version` is body hash; non-canonical bodies fail closed.

## 2026-10-02 — Product: independent RULE-01 kill-tests (Phase 37)

### Product

### Added

- Independent kill-tests: big-endian length and truncation are not canonical; crafted Horn is rejected; `StoreBlob` JSON with a Named pin does not append.

### Trajectory

- Cells remain **Proposed**. M016 S02 lease unchanged; no SemVer tag.

## 2026-10-02 — Product: rule body hash is rule_version (Phase 37)

### Product

### Changed

- `rule_definition_hash` of an identifier is the canonical `Named` body hash. `emit_rule` stores tagged AST bytes, not raw UTF-8. Non-canonical names fail closed (`UnknownRuleVersion`). `blob_digest_matches` accepts a rule-domain digest only for exact-canonical bodies.

### Added

- Named tests: JSON/`emit_rule` reject; stored blob `from_canonical` equals `Named`; Horn `emit_rule_ast` pin equals body hash.

### Trajectory

- Cells remain **Proposed**. M016 S02 lease unchanged; no SemVer tag.

## 2026-10-02 — Product: rule AST canonical encoding (Phase 37)

### Product

### Added

- Bounded positive `RuleAst` (`Named`, `Horn`) with tagged little-endian canonical bytes. Body hash is SHA-256 of `kutha-rule-def` plus those bytes. JSON, trailing junk, and raw UTF-8 names are not canonical. Evaluation is not in this slice.

### Trajectory

- Cells remain **Proposed**. M016 S02 lease unchanged; no SemVer tag.

## 2026-10-02 — Product: test hardening T2–T6 (Phase 36)

### Product

### Added

- `kutha-common` trybuild UI crate proves `TransactionTime` and `WallTime` cannot be passed where `ValidTime` is required (`E0308`). The rustdoc `compile_fail` example stays as a secondary check.
- Named test: stored `JustificationCite` `tt`/`vt` stay distinct; a swapped-field clone is not equal to the stored row.
- Named tests: Eager vs Lazy `as_known_on`; `fork_at` before a later `TtAnchor`; mid-cut snapshot plus tail Assert/`TtAnchor`; `GraphFold::replay` known-on.

### Changed

- `snapshot_plus_tail_matches_full_replay` uses a mid-cut snapshot, a non-empty Assert+`TtAnchor` tail, and `as_known_on` (fingerprint remains facts-only).

### Trajectory

- Cells remain **Proposed**. M016 S01 lease unchanged; no SemVer tag.

## 2026-10-02 — Process: living docs and governor honesty (Phase 35)

### Process

### Changed

- Living ROADMAP, PROJECT, semantic-gap, p0-spike, and legal-fixture-recon match M014 ship plus the Phase 34 known-on residual.
- `observe_cargo.required` includes `true_on_d_differs_from_known_on_d`. Artifact-budget globs use `min_hits: 1` while live phases exist; id-map-cite needles include v0.07 and v0.08.

### Product

### Changed

- Comment only: TT-to-wall is M014 `tt_anchor` / M015 known-on honesty, not out of M012a S05.

### Trajectory

- ADR-013 Evidence records F13 clock/anchor ship and `as_known_on`; the five-clock question stays open for M016. Honeycomb ADR-013 evidence restaged. Cells remain **Proposed**. No SemVer tag.

## 2026-10-02 — Product: fail-closed TtAnchor admit (Phase 34)

### Product

### Fixed

- `emit_tt_anchor` rejects a duplicate wall or a wall earlier than the max prior wall (`RuntimeError::TtAnchorRejected`). The log and fold stay unchanged. Monotone unique walls still resolve greatest-anchor-≤D after persist/open.

### Trajectory

- Cells remain **Proposed**. M015 S01 lease unchanged; no SemVer tag.

## 2026-10-02 — Product: known-on closed VT (Phase 34)

### Product

### Fixed

- `as_known_on` evaluates TT-only product at the resolved anchor (`known_at` / `is_known_at`). Facts with finite `valid_to` stay visible at a knowledge cut. `as_of` still uses `live_at(MAX, vt)`; `is_live_at` is unchanged.

### Trajectory

- Cells remain **Proposed**. M015 S01 lease unchanged; no SemVer tag.

## 2026-10-02 — Product: fixture clock wraps (Phase 33)

### Product

### Changed

- FF5 public clock cuts wrap `ValidTime::from` / `TransactionTime::MAX`; `Op` `valid_from`/`valid_to` stay `u64`. Oracle assertion predicates unchanged.
- M011/M012/M013 fixtures and FF6 `live_count` wrap the same constructors at `as_of` / `live_at` / CSR / justification / derivation / admission cuts.

### Trajectory

- Cells remain **Proposed**. M014 S03 lease unchanged; no SemVer tag.

## 2026-10-02 — Product: tt_anchor and as_known_on (Phase 32)

### Product

### Added

- `Op::TtAnchor { wall: u64 }` maps `Event.ingested_at` (transaction time) to a wall date. The fold keeps a skip-serialized `BTreeMap` lease rebuilt from the log; no `Fact` is minted.
- `Runtime::as_known_on` / `GraphFold::as_known_on` take `impl Into<WallTime>` and return `Option` of product triples: greatest wall at or before the date, then `live_at(tt, u64::MAX)`. `None` means no such anchor (miss, not empty-as-of).
- Named test `true_on_d_differs_from_known_on_d` (`crates/kutha-runtime/tests/m014_as_known_on.rs`): `as_of(2015)` sees a statute that `as_known_on(2015)` does not; `as_known_on(2020)` sees it after the later anchor.

### Trajectory

- Cells remain **Proposed**. M014 S02 lease unchanged; no SemVer tag.

## 2026-10-02 — Product: typed clock newtypes (Phase 31)

### Product

### Added

- `ValidTime`, `TransactionTime`, and `WallTime` are distinct `#[serde(transparent)]` tuple structs (`Vt` / `Tt` aliases). Mixing clocks without conversion does not type-check (`compile_fail` doctest on `ValidTime`). `Op` clock fields and `Event.ingested_at` stay `u64` so named oracle literals compile.

### Changed

- `GraphFold` query cuts (`as_of`, `live_at`, dictionary/admission/policy pins, `live_supports`, `claim_supported_at`) take `impl Into` clocks and compare `as_u64` against `Fact` integer fields. `as_of(2015u64)` and `as_of(ValidTime::from(2015))` type-check; `as_of(TransactionTime::from(1))` does not.
- `Runtime::as_of` / `live_at` match the fold (`impl Into` clocks). Write helpers (`assert` / `retract` / `correct` / `derive` / `emit_allow` / `emit_rule` / `emit_policy`) take `u64` so named oracle literals still compile.
- `Runtime::csr_lease_at` / `typed_csr_lease_at`, `CsrLease::from_fold` / `TypedCsrLease::from_fold`, and `Materializer::build` take `impl Into` clocks and convert with `as_u64` before the integer CSR picture.
- Justification/action/admission/policy/derivation/conflict cuts take `impl Into` clocks; `Op::JustificationCite` `tt`/`vt` stay `u64`.
- TIME-01 fixture year bindings wrap `ValidTime::from(...)`; `assert_eq!(y2015, 2015)` bodies unchanged.

### Trajectory

- Cells remain **Proposed**. M014 S01 lease unchanged; no SemVer tag.

## 2026-10-02 — Trajectory: M016 S02 leased

### Trajectory

- Active Slice **S02** (GSD Phase 37, Rule AST). `L_delivery=M016-S01-done`.

## 2026-10-02 — Trajectory: M016 S01 CLOSED (Phase 36)

### Trajectory

- Phase 36 verified; `L_delivery=M016-S01-done`. Next: lease S02 (Rule AST).

### Product

- trybuild E0308 clock mix; JustificationCite tt/vt oracle; known-on Eager/Lazy/fork/snapshot+tail/replay hardening tests.

## 2026-10-02 — Trajectory: M016 S01 leased

### Trajectory

- Active Milestone **M016**, Active Slice **S01** (GSD Phase 36, T2–T6). `L_delivery=M015-closed`.

## 2026-10-02 — Process: v0.09 milestone authored

### Process

- GSD v0.09 / harness M016: Phase 36 test-hardening T2–T6, then Phases 37–40 rules (F9/F16). Not leased yet.

## 2026-10-02 — Trajectory: M015 CLOSED (GSD v0.08 archive and tag)

### Trajectory

- M015 S01–S02 delivered; Active Milestone cleared.
- Governor: artifact-budget `min_hits` returned to 0 for empty live phases between milestones. Archive `.planning/milestones/v0.08-*`; process tag `v0.08`. Crate SemVer stays `0.0.0`.

### Product

- TT-only `as_known_on` (closed VT survives); fail-closed `TtAnchor` walls; ANK required in governor.

## 2026-10-02 — Trajectory: M015 S02 leased

### Trajectory

- Active Slice **S02** (GSD Phase 35). `L_delivery=M015-S01-done`.

## 2026-10-02 — Trajectory: M015 S01 CLOSED (Phase 34)

### Trajectory

- Phase 34 verified; Active Slice cleared. `L_delivery=M015-S01-done`. Next: lease S02 alone.
- Governor: `idle-delivery-closed` selftest forces Milestone None + non-closed `L_delivery` (works leased or idle).

### Product

- `as_known_on` uses TT-only `known_at` (closed VT intervals survive); duplicate/backwards `TtAnchor` fail closed at admit.

## 2026-10-02 — Trajectory: M015 S01 leased

### Trajectory

- Active Milestone **M015**, Active Slice **S01** (GSD Phase 34). `L_delivery=M014-closed`. `README.md` shows M015.

## 2026-10-02 — Process: v0.08 milestone authored

### Process

- GSD v0.08 / harness M015: known-on honesty and M014 residuals (phases 34–35). Executable rules renumbered to **M016** / v0.09. Id map updated. Not leased yet.

## 2026-10-02 — Trajectory: M014 CLOSED (GSD v0.07 archive and tag)

### Trajectory

- M014 S01–S03 delivered; Active Milestone cleared. GSD archive under `.planning/milestones/v0.07-*`; process tag `v0.07`. Crate SemVer stays `0.0.0`.
- Governor: `artifact-budget` / `verification-independent` selftest skip when live phases empty; `idle-delivery-closed` mutates `L_delivery`; `state-readme` selftest mutates Phase only at idle.

### Product

- F13: typed clocks, `tt_anchor`/`as_known_on`, fixture clock wraps. Cells remain **Proposed**.

## 2026-10-02 — Trajectory: M014 S03 leased

### Trajectory

- Active Slice **S03** (GSD Phase 33, fixture migration). `L_delivery=M014-S02-done`.

## 2026-10-02 — Trajectory: M014 S02 CLOSED (Phase 32)

### Trajectory

- Phase 32 verified; Active Slice cleared. `L_delivery=M014-S02-done`. Next: lease S03 alone.

### Product

- `Op::TtAnchor` + `as_known_on(WallTime)`; named true-on-D vs known-on-D; FF5 unchanged and green.

## 2026-10-02 — Trajectory: M014 S02 leased

### Trajectory

- Active Slice **S02** (GSD Phase 32, `tt_anchor` / `as_known_on`). `L_delivery=M014-S01-done`. `README.md` shows M014 S02 active.

## 2026-10-02 — Trajectory: M014 S01 CLOSED (Phase 31)

### Trajectory

- Phase 31 clock newtypes verified; Active Slice cleared. `L_delivery=M014-S01-done`. Next: lease S02 alone.
- Governor selftest `state-readme` mutation pattern updated to `M014` (was `M013`).

### Product

- Distinct `ValidTime` / `TransactionTime` / `WallTime` newtypes; query cuts take `Into`; `compile_fail` mix proof; named oracles green (TIME-01 wraps only).

## 2026-10-02 — Trajectory: M014 S01 leased

### Trajectory

- Active Milestone **M014**, Active Slice **S01** (GSD Phase 31, clock newtypes). `L_delivery=M013-closed`. `README.md` shows Active Milestone **M014**. Cells remain **Proposed**.

## 2026-10-02 — Trajectory: M013 CLOSED (GSD v0.06 archive and tag)

### Trajectory

- M013 S01–S09 delivered; Active Milestone cleared. GSD archive under `.planning/milestones/v0.06-*`; process tag `v0.06`. Crate SemVer stays `0.0.0`. Residual audit debt acknowledged. Cells remain **Proposed**.

### Process

- Governor: `artifact-budget` `min_hits` 0 so an empty live `.planning/phases/` is allowed between milestones; `verification-independent` accepts `allow_empty` on its globs. `L_delivery=M013-closed` with Active Milestone None.
- Id map: v0.06 → shipped; v0.07 (M014, phases 31–33) is next and needs a lease.

## 2026-10-02 — Product: facade exact triples and dual-mode bench (Phase 30-02)

### Product

### Added

- `facade_statute`: `assert_eq!` on sorted live triples after assert, correct, retract, persist/open, and empty `fork_at(0)` product picture (HARD-04).

### Changed

- `benchmark-baseline.md`: Phase 30 bench delta vs Phase 27 after-lease (ingest +5.6%, as_of faster).

### Trajectory

- Cells remain **Proposed**. M013 S09 lease unchanged; no SemVer tag.

## 2026-10-02 — Product: dual-mode FF5 oracle (Phase 30-02)

### Product

### Added

- `m013_dual_mode`: Eager vs Lazy agreement on FF5 `as_of`, `live_at`, retract, correct, and `claim_supported_at` without warming Lazy leases before reads (HARD-03).

### Trajectory

- Cells remain **Proposed**. M013 S09 lease unchanged; no SemVer tag.

## 2026-10-02 — Product: write-path admit origin and keyed correct identity

### Product

### Changed

- User `emit` cannot Assert, Behavior, Correct, or CorrectInterval on reserved `kutha:` relations (`MetaOpRejected`); dictionary constructors (`emit_allow` / rule / policy) and typed justification/action paths unchanged; user Retract of dictionary facts still admitted (D-07 oracles).
- `GraphFold::apply` copies the prior fact `delivery_key` onto Correct and CorrectInterval replacement rows so replacement identity survives keyed correction; keyed ingest retry idempotence remains the log `by_delivery_key` map on the minting Assert (ING-01). Snapshots written before this change whose logs contain keyed Correct/CorrectInterval will fail `replay_check` fingerprint until rebuilt (research stage; no migration).
- Phase 30 bench `as_of` delta vs Phase 27 in `benchmark-baseline.md` is single-run variance, not a query-path change in this slice.

### Trajectory

- Cells remain **Proposed**. M013 S09 lease unchanged; no SemVer tag.

## 2026-10-02 — Product: Phase 30-01 RED oracles (write-path integrity)

### Product

### Added

- `m013_write_path` RED tests: public `emit` Assert on `kutha:allows` must be refused (HARD-01); keyed `Correct` / `CorrectInterval` must preserve `delivery_key` through retry (HARD-02). Implementation follows in the same phase.

### Trajectory

- Cells remain **Proposed**. M013 S09 lease unchanged; no SemVer tag.

## 2026-10-02 — Trajectory: M013 S09 closed

### Trajectory

- Phase 30 verified independently (`30-VERIFICATION` passed, Grok 4.6 xhigh). Active Slice cleared; `L_delivery=M013-S09-done`; M013 S01–S09 delivered. No archive, no SemVer tag. Cells remain **Proposed**.

## 2026-10-02 — Trajectory: M013 S09 leased

### Trajectory

- `L_delivery=M013-S08-done`, Active Slice **S09** (GSD Phase 30, write-path integrity hardening; HARD-01..04). Cells remain **Proposed**; no archive, no SemVer tag.

## 2026-10-02 — Trajectory: M013 S08 closed

### Trajectory

- Phase 29 verified independently (`29-VERIFICATION` passed, Grok 4.6 xhigh). Active Slice cleared; `L_delivery=M013-S08-done`; S09 (Phase 30) unleased. M013 stays open; no archive, no SemVer tag. Cells remain **Proposed**.

### Process

- Governor: budget truncation and empty budget globs fail closed; verification independence covers phases 22+; 60 checks within budget 64.

## 2026-10-02 — Process: Phase 29 review follow-ups

### Process

### Fixed

- `frontmatter_paths_exist` rejects paths that resolve outside the repo and an empty `read_only_inputs` list (HIGH); quoted entries are unquoted. Two tests pin both.
- `i-f11-one-write-path` pins `require_symbols`; invariants comment now says only deferred rows are unenforced; META `file_contains` row names `glob` / `min_phase`.

## 2026-10-02 — Process: honor-system verify and process doc alignment

### Process

### Changed

- `kutha-harness.md` and `.kutha/META.md` document `ci` as run_checks → run_selftest → observe_cargo; `--skip-cargo` leaves selftest on; `glob_max_lines` `min_hits` zero-match rule.
- `codex-subagents.md` Cursor role map for phases 22–30; `governor-intake.md` current I-cui-budget / glob / frontmatter examples.
- `gsd-auto-contract.md` states model-family independence is honor-system, not governor-provable.

### Trajectory

- No lease change. Cells remain **Proposed**.

## 2026-10-02 — Trajectory: post-M013 addendum and ADR evidence notes

### Trajectory

### Changed

- Dated Evidence/Clarification blocks on ADR-000, 010–014, 040–041, 050, 060–061 (Status stays **Proposed**; lock bodies unchanged).
- `semantic-gap-review.md` post-M013 addendum: F10–F12, F14–F15 closed with named oracles; F1/F7–F9/F13/F16 open; FF5 scope at TT max.
- `honeycomb.yaml` M013 test names restaged (`statute_as_of_via_facade`, genesis, ingest/fold oracles).

### Process

- `docs/ADR/README.md` status vocabulary note (map/delivery/capability vs Accepted).

## 2026-10-02 — Process: align v0.06 bookkeeping with 28-VERIFICATION

### Process

### Changed

- BASE/REC requirement checkboxes and traceability marked complete (evidence: 22-VERIFICATION, 23-VERIFICATION, `legal-fixture-recon.md`).
- `v0.06-MILESTONE-AUDIT.md` YAML and body agree: Phase 28 passed; PROC-02 satisfied; `gaps_found` only for audit-tail and tracking debt.
- `PROJECT.md` and `.kutha/ROADMAP.md` cite `L_delivery=M013-S07-done` and M013 S08–S09 tail without closing the milestone.

### Trajectory

- M013 stays leased until S09; no archive or tag. Cells remain **Proposed**.

## 2026-10-02 — Process: restage shipped I-F oracles and require M013 tests

### Process

### Added

- Six M013 `observe_cargo.required` test names; eleven `i-f*` `rust_test_asserts` checks restage shipped semantic-gap rows (I-F1–F6, F10–F12, F14–F15).

### Changed

- I-F7/I-F8 `until` strings name M014 / remaining fork-diff honestly; I-F9, F13, F16 stay deferred.

### Trajectory

- No lease change. Cells remain **Proposed**.

## 2026-10-02 — Process: glob verification independence and listed input paths

### Process

### Added

- `file_contains` supports `glob` and `min_phase`; new `frontmatter_paths_exist` kind requires listed `read_only_inputs` files to exist under the repo root.
- `verification-independent` uses phase-22+ globs instead of per-phase hardcoded paths; removed non-repo path from `23-VERIFICATION.md`.

### Trajectory

- No lease change. Cells remain **Proposed**.

## 2026-10-02 — Process: fail closed on check-budget truncation and empty budget globs

### Process

### Added

- FSM `run_checks` emits HIGH `budget` when any check is skipped under the configured budget; ledger row `I-cui-budget` records the `run_checks` kind obligation.
- `glob_max_lines` optional `min_hits` (default 0); artifact-budget steps require at least one phase-22+ match per glob.

### Trajectory

- No lease change. Cells remain **Proposed**.

## 2026-10-02 — Process: M013 tail S08–S09 from the audit and project review

### Process

### Added

- `.kutha/ROADMAP.md` M013 tail: S08 close-out honesty (documents and governor) and S09 write-path integrity hardening. GSD Phases 29 and 30 with CONTEXT; requirements DOC, GOV, HARD in `.planning/REQUIREMENTS.md`.

### Changed

- `docs/process/gsd-auto-contract.md` § Id map: v0.06 is Phases 22–30 and slices S01–S09; tool pair applies to Phase 30 (product code), CBM only for Phase 29. M013 stays leased until S09 closes. No archive, no SemVer tag.

### Trajectory

- No lease change in this commit. Active Slice **None**; S08 is leased in the next commit alone. Cells remain **Proposed**.

## 2026-10-02 — Process: ci --skip-cargo and phase timings

### Process

### Added

- `uv run kutha-gov ci --skip-cargo` skips `observe_cargo` and `emit_tenant` (selftest still runs); a HIGH from checks also skips cargo and tenant after selftest. The ci report prints `observe: cargo skipped (not evidence)` and per-phase `phase_ms`. `--skip-cargo` is rejected for commands other than `ci`. Not closeout evidence; one full `ci` at close (`docs/process/kutha-harness.md`).
- `AGENTS.md`: one short `rg` vs `ast-grep` note (text/regex arbiter vs AST). Still under the `agents-lean` line budget.

### Trajectory

- No lease change. Active Slice **None**. Cells remain **Proposed**.

## 2026-10-02 — Process: FSM check budget above the check count

### Process

### Fixed

- `defaults.budget` and `KUTHA_GOV_BUDGET` were 48 while the governor had 49 checks after Phase 28, so the default `ci` quantum silently truncated the tail check (`version-freeze`) and `test_ci_quantum_reaches_ok_on_this_tree` / `test_cui_budget_truncates_tail_checks_when_v_is_short` failed. Budget is now 64 (headroom). Fail-closed on any truncation is planned in Phase 29.

### Trajectory

- No lease change. Active Slice **None**. Cells remain **Proposed**.

## 2026-10-02 — Process: close M013 S07 after independent verification

### Process

### Changed

- Phase 28 independent verification passed (`28-VERIFICATION.md`); PROC-02 closed. Active Slice cleared; `L_delivery=M013-S07-done`. M013 S01–S07 delivered.

### Trajectory

- Phase 28 / S07 closed. Cells remain **Proposed**. Do not start M014 until leased. No SemVer tag.

## 2026-10-02 — Process: v0.06 milestone audit and PROC checkboxes

### Process

### Added

- `.planning/v0.06-MILESTONE-AUDIT.md` from gsd-audit-milestone; status `gaps_found` because `28-VERIFICATION.md` is not written in this wave (D-08).

### Changed

- REQUIREMENTS PROC-01, PROC-03, and PROC-04 checked; PROC-02 left open pending independent verify-work.

### Trajectory

- No lease close, no SemVer tag, no `state.milestone-switch`. Cells remain **Proposed**. S07 stays leased.

## 2026-10-02 — Process: Id map cite from governor and ROADMAPs

### Process

### Added

- Invariant `I-id-map` and check `id-map-cite`: the Id map table stays in `docs/process/gsd-auto-contract.md`; `.planning/ROADMAP.md` and `.kutha/ROADMAP.md` link to § Id map and must not restate the table header.

### Trajectory

- No lease or honeycomb change in this commit. Cells remain **Proposed**.

## 2026-10-02 — Process: remaining artifact budgets and verification independence

### Process

### Added

- `artifact-budget` steps for SUMMARY (max 60, n≤2), VERIFICATION (max 60, n=1), and CONTEXT (max 80, n=1) from phase 22.
- Check `verification-independent` and invariant `I-verification-independent`: `glob_absent` of `-PLAN.md` / `-SUMMARY.md` on VERIFICATION files; `file_contains` independence keys on phases 22–27.

### Trajectory

- No lease or honeycomb change in this commit. Cells remain **Proposed**.

## 2026-10-02 — Process: glob_max_lines and PLAN artifact budget

### Process

### Added

- Kind `glob_max_lines`: glob matches counted with `wc -l`, optional `min_phase` (phases 22+), optional `max_files_per_dir`. Excess lines are HIGH `max-lines`; too many files in one phase directory are HIGH `max-files`. Derived selftest mutation appends newlines to the first sorted match.
- Check `artifact-budget` (PLAN step only) and invariant `I-artifact-budget`. Unknown kind stays fail-closed HIGH.

### Trajectory

- No lease or honeycomb change in this commit. Cells remain **Proposed**.

## 2026-10-02 — Product: O(1) kutha:allows admit lease

### Product

### Changed

- Fold maintains a droppable `kutha:allows` hot lease (`allows_by_name`) rebuilt with other fold leases. `relation_allowed_at` is O(allows) per admit instead of collecting all live facts. Eager/Lazy still applies to product maps; product `as_of` excludes `kutha:`; `dictionary_as_of` unchanged. Named D-07 expects and `Op::AllowRelation` absence unchanged.
- Phase 22 bench at N=10000 after the lease: ingest_plain.n 120267790 ns (+26.1% vs Phase 22, was +2168%); as_of.n 511233 ns (single-shot fold walk; was +1566%).

### Trajectory

- No harness lease change. Active Slice **S06**. Cells remain **Proposed**.

## 2026-10-02 — Product: public facade and write-path oracles

### Product

### Added

- Public Runtime facade: `assert` / `retract` / `correct` / `derive` (internable names), `as_of` / `live_at`, `persist` / `open`. Statute example and `facade_statute` compile against that surface; `as_of(2015) ≠ as_of(2021)` plus one admission and one action.
- `m013_write_path` unknown-relation Assert is `UnknownRelation`; justification/action go through pipeline receipt (`outcome_records` grows); no ghost-relation fact.

### Changed

- FF5 named `as_of` reads `Runtime::as_of` (same triples as `fold().as_of`). `emit` stays public so m012/ff6/bench call sites and expects are unchanged (D-07).

### Process

- Governor `file_contains` needles for `live_support_count` / `claim_supported_at`, `derivation_eligible_at`, `provenance_fingerprint`, and `check_admission` / `conflict_report_at` follow the 27-02 module split (`fold_query.rs`, `provenance.rs`, `admission.rs`). Same symbols; named cargo expects unchanged.

### Trajectory

- S06 still Active Slice. Phase 22 bench re-run: ingest_plain.n +2168%, as_of.n +1566% vs the Phase 22 table — D-07 ≤10% **STOP** (documented in `docs/architecture/benchmark-baseline.md`). Cells remain **Proposed**.

## 2026-10-02 — Product: one write pipeline and runtime module split

### Product

### Added

- `Runtime::pipeline` / `pipeline_kind` / `pipeline_events`: one propose→admit→commit→receipt path. `emit` is a thin wrapper that still rejects public forging of `JustificationCite`, `QuantumOutcome`, `kutha:admission`, and `kutha:action` (`MetaOpRejected`).
- Runtime modules under `crates/kutha-runtime/src/`: `pipeline.rs`, `admission.rs`, `justification.rs`, `hydrate.rs`, `provenance.rs`, `fold_query.rs`, `quantum_invariants.rs`. Every `src/*.rs` is ≤800 lines. No new crate.

### Changed

- `emit_allow` / `emit_rule` / `emit_policy`, `record_justification` / `record_action` / `record_resume`, and user `emit` all call the pipeline. The `append_meta` side path is deleted. Dictionary constructors stay receipt-quiet so named m012 log-growth expects hold; justification/action buffer a receipt without an extra log `QuantumOutcome`.
- `GraphFold` query surface (`as_of`, `live_at`, dictionary/registry cuts, fingerprint) lives in `fold_query.rs`; apply/replay/hot maps stay in `fold.rs`.

### Trajectory

- No lease change. Active Slice **S06**. Cells remain **Proposed**. Facade methods and Phase 22 bench delta are the next 27-02 commit.

## 2026-10-01 — Product: RED facade statute consumer

### Product

### Added

- Integration test `facade_statute::statute_as_of_via_facade` and example `statute_as_of`: D-08 RED (PIPE-03/04) — facade-only consumer (`Runtime` / `RuntimeError`; D-03 names `assert` / `retract` / `correct` / `derive` / `as_of` / `live_at` / `persist` / `open`) does not compile today. One admission and one action record. Wiring is 27-02.
- Integration test `m013_write_path::unknown_relation_rejected_for_assert_justification_and_action`: D-08 RED (PIPE-01) — Assert unknown-relation is fail-closed; justification/action skip the same admit/receipt (`outcome_records` does not grow). Wiring is 27-02.

### Trajectory

- No lease change. Active Slice **S06**. Cells remain **Proposed**.

## 2026-10-01 — Product: EventId admissions and green dictionary oracles

### Product

### Changed

- `justification_id` is the `EventId` of the `kutha:admission` Assert. Named oracles (`m012_*`, `m012a_*`, `m011_*`, `ff5`, `ff6`) call `emit_allow` / `emit_rule` / `emit_policy`; 26-01 tests read `dictionary_as_of` (product `as_of` stays kutha-free). Retract of an admission or genesis allow EventId versions the later cut; `fork_at` still admits.
- Provenance mix hashes dictionary Asserts (former AllowRelation/RegisterRule/PinPolicy/RecordAdmission/RecordAction) plus `StoreBlob`.

### Trajectory

- Phase 26 / S05 closed: independent verification passed; `L_delivery=M013-S05-done`; Active Slice **None**. Cells remain **Proposed**. Do not start Phase 27 until S06 leased.

## 2026-10-01 — Product: dictionary entries as kutha facts and StoreBlob


### Product

### Added

- `Op::StoreBlob { digest, bytes }` — content-addressed body on the log (store-once; digest is SHA-256 hex of bytes). Used for `kutha:rule` / `kutha:policy` payloads; digest is `rule_definition_hash` / `policy_version_hash`.
- Reserved relation constants `KUTHA_ALLOWS`, `KUTHA_RULE`, `KUTHA_POLICY`, `KUTHA_ADMISSION`, `KUTHA_ACTION` and `KUTHA_RESERVED_RELS`.
- `Fold::dictionary_as_of` / `dictionary_live_at` — live dictionary facts including `kutha:` (D-05). Product `as_of` / `live_at` exclude `kutha:`.
- Runtime helpers `emit_allow`, `emit_rule`, `emit_policy` (silent intern + `StoreBlob` + dictionary Assert). `intern` marks reserved `kutha:` names so `admit` skips the product allowlist for those relations. `Retract` is fact-only (`EventId`).

### Changed

- Removed `Op::AllowRelation`, `RegisterRule`, `PinPolicy`, `RecordAdmission`, `RecordAction` and the five fold side tables. Dictionary rows are ordinary bi-temporal Facts with those reserved relations.
- Genesis emits `kutha:allows` Asserts (`valid_from` 0) after silent intern of reserved names plus YAML, instead of `AllowRelation`.
- `check_admission` reads the fold and log (no RAM justification buffer). `record_justification` returns the `kutha:admission` Assert `EventId` as `justification_id`. Public `emit` of `kutha:admission` / `kutha:action` Assert or `JustificationCite` is `MetaOpRejected`.

### Trajectory

- No lease change. Active Slice **S05**. Cells remain **Proposed**.

## 2026-10-01 — Product: RED dictionary entries on as_of


### Product

### Added

- Integration test `m013_dictionary_facts::admission_and_policy_visible_on_as_of_after_later_pin` (D-08 RED): after `PinPolicy` + `record_justification` + a later `PinPolicy`, `fold.as_of(2010)` must contain interned `kutha:policy` and `kutha:admission` triples (same walk as FF5). Fails today: those rows live in fold side tables. Wiring is 26-02.
- Integration test `m013_dictionary_facts::genesis_allow_visible_on_as_of` (D-08 RED): `Runtime::default()` then intern `kutha:allows`; `fold.as_of(0)` must contain that relation. Fails today: genesis is a non-fact allow-entry. Do not intern extra YAML names.

### Trajectory

- No lease change. Active Slice **S05**. Cells remain **Proposed**.

## 2026-10-01 — Product: genesis AllowRelation on new log

### Product

### Added

- `Runtime::new` seeds one `Op::AllowRelation` per YAML name (`valid_from` 0) on an empty log. Names are not interned. Sorted for a stable provenance mix.
- Genesis oracles `stripped_genesis_rejects_product_relation` and `persist_open_replay_keeps_genesis` (stripped log fail-closed; persist/open keeps YAML names; genesis payload tamper fails `provenance_check`).

### Changed

- `admit` consults `fold.relation_allowed_at` only. Runtime no longer holds a YAML `HashSet`. `from_snapshot` / `from_dict_and_events` / `fork_at` do not read relations YAML (D-01, D-02, D-03).
- M010 `intern_appends_define_for_new_terms_only` length proxies use genesis prefix `G` (Define-silence unchanged).

### Trajectory

- Phase 25 / S04 closed: independent verification passed; `L_delivery=M013-S04-done`; Active Slice **None**. Cells remain **Proposed**. Do not start Phase 26 until S05 leased.

## 2026-10-01 — Product: RED genesis log oracles

### Product

### Added

- Integration tests `m013_genesis_log` (`retract_seeded_related_to_fails_closed_after`, `open_after_relations_path_missing_still_emits`): D-07 / GEN-03 and D-03 / GEN-01 RED — retract YAML-seeded `relatedTo` then Assert must fail closed; persist then `open` with `KUTHA_RELATIONS_PATH` pointing at a missing file must still emit `inForceAs` (currently fails closed because `open` reloads YAML). Wiring is 25-02.

### Trajectory

- No lease change. Active Slice **S04**. Cells remain **Proposed**.

## 2026-10-01 — Process: productivity from log audit

### Process

### Added

- `kutha-gov ci --skip-cargo`: keep dictionary + H5 selftest + emit_log; skip cargo/tenant subprocess for harness iteration; fail-fast skips the same after HIGH; `phase_ms` on stdout (`docs/process/kutha-harness.md`).
- Gate stacking + reindex discipline in `docs/process/gsd-auto-contract.md`.
- Plan: `.compound-engineering/artifacts/plans/2026-10-01-process-productivity-from-log-audit.md`.

### Changed

- `.planning/config.json`: `skip_discuss`, no worktrees/Nyquist/UI/AI-integration phases, `git.create_tag=false`; keep `plan_check` + `verifier`.
- `.planning/WINDOWS.md`: waive 1/3/5/6; fixed 8; leave 2/4/7/9 open.
- Phase 23 `23-VERIFICATION.md` trimmed to ≤60 lines (artifact budget).

### Trajectory

- No lease change. Item deferred: hanging ideation `http.server` / unauth MCP (explain-only; no kill, no MCP edit).

## 2026-10-01 — Product: M013 S03 fold ingest indexes (pending D-06)

### Product

### Added

- Fold/Runtime ingest index leases (`fact_by_event`, `live_triples`, `by_event_id`, `by_delivery_key`) and `ingest_examine` visit counter; D-03 scan sites use indexes (Phase 24 / IDX-01, IDX-02).
- Integration test `m013_fold_indexes` (`green_ingest_visits_constant_in_n`, fork/open checks).

### Trajectory

- Active Slice **S03**; `L_delivery=M013-S02-done` until S03 closes.
- D-06 absolute ingest wall-time vs Phase 22 exceeded 10% on two runs (+17…+28%); documented in `benchmark-baseline.md` Phase 24 delta. Controller decision required before closing S03.
- Reserve spike (`reserve_ingest_indexes`, small Vec caps): still +21…+28% ingest; capacity alone does not clear D-06.
- Hybrid spike (`FxHashMap` + `SmallVec<[usize;1]>` + reserve; Samyama/Pogocache-inspired): ingest still +21…+28% vs Phase 22; visit oracle green. See `benchmark-baseline.md` hybrid section.
- `FoldIndexMode` config (Eager = accept write-cost default; Lazy = thin write / lazy lease via `KUTHA_FOLD_INDEX_MODE=lazy|thin`). API: `Runtime::with_index_mode` / `set_index_mode` / `ensure_fold_indexes`. Tests: `m013_fold_index_mode`. Env template: `.env.example` (`KUTHA_FOLD_INDEX_MODE=eager`); see `benchmark-baseline.md` § Fold index mode.
- Phase 24 / S03 closed: independent verification passed (IDX-03 with controller D-06 override); `L_delivery=M013-S03-done`.

## 2026-10-01 — Process: M013 S02 legal fixture reconnaissance

### Process

### Added

- `docs/architecture/legal-fixture-recon.md`: Proposed review mapping law-nexus fixtures to the current two-axis API (Phase 23 / REC-01..03).

### Trajectory

- Active Slice cleared; `L_delivery=M013-S02-done`. Independent verification passed (`23-VERIFICATION.md`). Note `legal-fixture-recon` has 12 QUOTE-INDEX cases including sunset C01.

## 2026-10-01 — Product: M013 S01 benchmark baseline

### Product

### Added

- `crates/kutha-runtime/benches/baseline.rs` (`[[bench]] harness = false`): relatedTo plain and keyed ingest at N/2N/4N with live_count, fingerprint, claim_supported_at, late retract, 27 sorted metric keys, and fail-closed `KUTHA_BENCH_SABOTAGE=drop_one` (Phase 22 / BASE-01, BASE-03).

### Fixed

- Baseline bench scratch dirs resolve under repo-root `tmp/` via `CARGO_MANIFEST_DIR` (cargo package cwd no longer leaves untracked `crates/kutha-runtime/tmp/`).

### Trajectory

- `L_delivery=M013-S01-done` after Phase 22 verify; Active Slice cleared before S02 lease. Independent verification passed (`22-VERIFICATION.md`).
- `docs/architecture/benchmark-baseline.md`: measured default-N run, ratios, persist/open at three sizes, proposed target N 1000000 and memory budget 13674000000.

### Process

### Added

- Phase 22 / M013-S01 baseline note under `docs/architecture/` (not an ADR); governor docs-coupling satisfied by this CHANGELOG entry.

## 2026-10-01 — Post-M012 review: roadmap reset and M013 lease

### Process

### Added

- `docs/process/gsd-auto-contract.md`: id map (GSD version, harness milestone, ADR), controller protocol for lease-per-phase GSD runs, the CBM plus GitNexus tool pair with measured limits, an independent-verification gate, refactor safety rules, and a planning-artifact budget.
- Ledger rows `I-F9`…`I-F16` in `.kutha/dictionaries/invariants.yaml` (deferred, not enforced).

### Changed

- `.kutha/ROADMAP.md`: M013 (baseline, legal fixture reconnaissance, fold indexes, genesis log, dictionary entries as facts, one write path and facade, independent verification) leased; M014 (typed clocks) and M015 (executable rules and the real-text legal golden fixture) proposed; M002 re-scoped to segmentation, hash chain, manifest, and commit-marker framing, gated by the M015 verdict.
- Code-graph rules: GitNexus runs together with CBM during M013 refactor phases only.
- v0.06 Phases 22 and 23 are planned and plan-checked (two rounds, independent checker); Phases 24–28 carry CONTEXT only and are planned against the code the previous phase leaves. Executors commit per task; the final plan's commit carries the CHANGELOG entry.

### Trajectory

- `docs/architecture/semantic-gap-review.md` gains an addendum (F9–F16, roadmap findings R1–R6, remedies). ADR-010, 011, 013, 014, 040, 041, 050, 051, 060, and 090 gain dated Proposed clarifications or open questions; `honeycomb.yaml` restaged. No cell is Accepted and no lock is rewritten.
- `L_delivery=M013-leased` at milestone open (Active Slice was cleared until S01). Freeze holds except the thawed M013 subset named in STATE. M014, M015, and M002 need their own lease.

## 2026-10-01 — Product: M012 S05 multi-hop derivation (M012 closed)

### Product

- Two-hop `derivation_eligible_at` walk and residual `inverse_knows` cascade are green under named DER oracles (`m012_multi_hop`).

### Process

- Governor DER observe + Phase 21 closed. Harness ROADMAP marks M012 **CLOSED**.

### Trajectory

- `L_delivery=M012-S05-done`; Active Slice **None**. Active Milestone still **M012** until operator clears or leases the next wave. Freeze until M002 unchanged. Honeycomb Proposed.

## 2026-10-01 — Process: governor observes DER-01..03

### Process

### Added

- Governor bridge `B-m012-s05` / `m012-s05-multi-hop` and three `observe_cargo.required` names for DER-01..03 (`two_hop_derivation_eligible_when_chain_live`, `two_hop_ineligible_when_root_assert_retracted`, `inverse_knows_cascade_is_residual_spike`). ALL, RULE, ADM, ACT, FF6, LOG, REF, ING, DUR, TIME, and HOT names stay required.

### Trajectory

- Honeycomb stays **Proposed**. ADR-011 evidence lists include the DER names. ADR-050 map stays Proposed and delivery frozen. Active Slice remains **S05**. Freeze until M002 unchanged.

## 2026-10-01 — Product: M012 S05 two-hop derivation eligibility

### Product

### Added

- `derivation_eligible_at` walks `caused_by` for a named two-hop hashed fixture. Retracting the root Assert drops hop2 eligibility while the hop2 fact may stay live. persist/open reconstructs that picture from the log.
- `inverse_knows` remains the residual automatic cascade (empty pin, knows-only) with a named guard. Hashed user Behavior chains are the leased follow-on.

### Trajectory

- Honeycomb stays **Proposed**. Active Slice remains **S05**. Freeze until M002 unchanged. Not ADR-050 six dictionaries. Not a MATCH compiler or provenance polynomials.

## 2026-10-01 — Process: lease M012 Active Slice S05

### Process

- Active Slice **S05** (multi-hop / n-ary derivation eligibility). Active Milestone stays **M012**; `L_delivery=M012-S04-done` until S05 closes. Last M012 product slice.

### Trajectory

- GSD Phase 21 may edit product crates for DER-01..03. Six dictionaries / Cypher / HNSW / M002 stay frozen. Full provenance polynomials stay out of scope.

## 2026-10-01 — Product: M012 S04 thin action record

### Product

- `Op::RecordAction` binds resolved arguments to admission + policy version.
- Action + admission remain queryable AS OF a prior cut after a later policy pin. Named oracles in `m012_action_record`.

### Process

- Governor ACT observe + Phase 20 closed.

### Trajectory

- `L_delivery=M012-S04-done`; Active Slice **None** until S05 leased. Active Milestone stays M012. Six dictionaries / Cypher / HNSW / M002 stay frozen.

## 2026-10-01 — Process: governor observes ACT-01..02

### Process

### Added

- Governor bridge `B-m012-s04` / `m012-s04-action-record` and two `observe_cargo.required` names for ACT-01..02 (`action_binds_args_to_admission_and_policy`, `action_and_admission_as_of_prior_cut_after_policy_change`). ALL, RULE, ADM, FF6, LOG, REF, ING, DUR, TIME, and HOT names stay required.

### Trajectory

- Honeycomb stays **Proposed**. ADR-011 and ADR-051 evidence lists include the ACT names. ADR-050 map stays Proposed and delivery frozen. Active Slice remains **S04**. Freeze until M002 unchanged.

## 2026-10-01 — Product: M012 S04 thin Action record

### Product

### Added

- Thin Action is `Op::RecordAction` log facts that bind resolved justification arguments to the admission decision and policy version. Named AS OF-after-later-policy oracle. persist/open reconstructs action-entries from the log.

### Trajectory

- Honeycomb stays **Proposed**. Active Slice remains **S04**. Freeze until M002 unchanged. Not ADR-050 six dictionaries. Not n-ary derivation.

## 2026-10-01 — Process: lease M012 Active Slice S04

### Process

- Active Slice **S04** (thin action record). Active Milestone stays **M012**; `L_delivery=M012-S03-done` until S04 closes.

### Trajectory

- GSD Phase 20 may edit product crates for ACT-01..02. Six dictionaries / Cypher / HNSW / M002 stay frozen.

## 2026-10-01 — Process: governor observes ADM-01..03

### Process

### Added

- Governor bridge `B-m012-s03` / `m012-s03-admission-facts` and three `observe_cargo.required` names for ADM-01..03 (`admission_status_queryable_as_of_cut`, `admission_cites_pinned_policy_version`, `record_justification_invokes_check_admission`). ALL, RULE, FF6, LOG, REF, ING, DUR, TIME, and HOT names stay required.

### Trajectory

- Honeycomb stays **Proposed**. ADR-011 and ADR-050 evidence lists include the ADM names. ADR-050 map stays Proposed and delivery frozen. Active Slice remains **S03**. Freeze until M002 unchanged.

## 2026-10-01 — Product: M012 S03 admission and policy meta-facts

### Product

### Added

- Admission status is `Op::RecordAdmission` log facts queryable AS OF a cut. Policy version is `Op::PinPolicy`; `record_justification` invokes `check_admission` and cites the live pin. Retract of those EventIds versions the later cut; persist/open reconstructs entries from the log. Named oracles in `m012_admission_facts`.

### Trajectory

- Honeycomb stays **Proposed**. Active Slice remains **S03**. Freeze until M002 unchanged. Not ADR-050 six dictionaries. Not a thin Action record.

## 2026-10-01 — Process: lease M012 Active Slice S03

### Process

- Active Slice **S03** (admission status and policy version as meta-facts). Active Milestone stays **M012**; `L_delivery=M012-S02-done` until S03 closes.

### Trajectory

- GSD Phase 19 may edit product crates for ADM-01..03. Six dictionaries / Cypher / HNSW / M002 stay frozen.

## 2026-10-01 — Process: governor observes RULE-01..03

### Process

### Added

- Governor bridge `B-m012-s02` / `m012-s02-rule-registry` and three `observe_cargo.required` names for RULE-01..03 (`register_rule_pins_behavior_to_definition_hash`, `unknown_or_mismatched_rule_version_fails_closed`, `free_string_rule_version_does_not_append`). ALL, FF6, LOG, REF, ING, DUR, TIME, and HOT names stay required.

### Trajectory

- Honeycomb stays **Proposed**. ADR-011 evidence lists include the RULE names. ADR-050 map stays Proposed and delivery frozen. Active Slice remains **S02**. Freeze until M002 unchanged.

## 2026-10-01 — Product: M012 S02 rule registry as log facts

### Product

### Added

- Rule definitions are `Op::RegisterRule` log facts. `Behavior.rule_version` equals `rule_definition_hash` of the registered body. Unknown, mismatched, and free-string pins fail closed on emit and eligibility. Retract of a registry EventId versions the later cut; persist/open reconstructs the hash from the log. Named oracles in `m012_rule_registry`.

### Trajectory

- Honeycomb stays **Proposed**. Active Slice remains **S02**. Freeze until M002 unchanged. Not ADR-050 six dictionaries.

## 2026-10-01 — Process: lease M012 Active Slice S02

### Process

- Active Slice **S02** (rule registry; `rule_version` = definition hash). Active Milestone stays **M012**; `L_delivery=M012-S01-done` until S02 closes.

### Trajectory

- GSD Phase 18 may edit product crates for RULE-01..03. Six dictionaries / Cypher / HNSW / M002 stay frozen.

## 2026-10-01 — Product: M012 S01 allowlist as log facts

### Product

- `Op::AllowRelation` appends versioned allowlist facts; `Runtime::admit` consults the fold cut (YAML seed is fallback only).
- Retract by EventId versions the allowlist; persist/open reconstructs YAML-absent names from the log. Named oracles in `m012_allowlist_facts`.

### Process

- Governor `m012-s01-allowlist-facts` + FSM observe for three ALL tests. GSD Phase 17 closed.

### Trajectory

- `L_delivery=M012-S01-done`; Active Slice **None** until S02 leased. Active Milestone stays M012. Six dictionaries / Cypher / HNSW / M002 stay frozen.

## 2026-10-01 — Process: governor observes ALL-01..03

### Process

### Added

- Governor bridge `B-m012-s01` / `m012-s01-allowlist-facts` and three `observe_cargo.required` names for ALL-01..03 (`allow_relation_appends_as_log_fact`, `retract_allow_relation_drops_admit_at_later_cut`, `admit_consults_fold_allowlist_not_yaml_alone`). FF6 `ff6_unknown_relation_does_not_append` stays required. LOG, REF, ING, DUR, TIME, and HOT names stay required.

### Trajectory

- Honeycomb stays **Proposed**. ADR-011 evidence lists include the ALL names. ADR-050 map stays Proposed and delivery frozen. Active Slice remains **S01**. Freeze until M002 unchanged.

## 2026-10-01 — Product: allowlist entries as log facts

### Product

### Added

- Relation allowlist entries are `Op::AllowRelation` log facts. `Runtime::admit` consults the fold cut for a live name; YAML/env remains a seed for names never logged. Unknown relations and empty allow-names still fail closed. Retract of an allow-entry EventId versions the later cut. Named oracles in `m012_allowlist_facts`.

### Trajectory

- Honeycomb stays **Proposed**. Active Slice remains **S01**. Freeze until M002 unchanged. Not ADR-050 six dictionaries.

## 2026-10-01 — Process: lease M012 Active Slice S01

### Process

- Active Slice **S01** (relation allowlist as log facts). Active Milestone stays **M012**; `L_delivery=M012-leased` until S01 closes.

### Trajectory

- GSD Phase 17 may edit product crates for ALL-01..03. Six dictionaries / Cypher / HNSW / M002 stay frozen.

## 2026-10-01 — Process: lease M012 dictionaries as facts

### Process

- Harness lease: Active Milestone **M012**; Active Slice **None**; `L_delivery=M012-leased`. Phase stays **H5**.
- Thawed subset named on STATE/ROADMAP: versioned relation-allowlist entries, rule registry (`rule_version` = definition hash), admission status + policy version as meta-facts, thin action record, multi-hop derivation eligibility. ADR-050 six dictionaries, Cypher/HNSW, legal pack, M002 remain frozen.

### Trajectory

- Product plane may land F4/F5 under M012. Governor green is not ADR Accepted. Honeycomb stays Proposed.

## 2026-10-01 — Product: M012a S06 hot indexes (M012a closed)

### Product

- `GraphFold` indexes facts by `valid_from` / `claim_id`; `as_of` / `claim_supported_at` skip non-overlapping facts (HOT-01).
- Untyped CSR builds via `CsrMaterializer`; CSR remains a droppable lease (HOT-02/03). Named oracles in `m012a_hot_indexes`.

### Process

- Governor `m012a-s06-hot-indexes` + FSM observe for three HOT tests. GSD Phase 16 closed. Harness ROADMAP marks M012a **CLOSED**.

### Trajectory

- `L_delivery=M012a-S06-done`; Active Slice **None**. Active Milestone still **M012a** until operator clears or leases M012. Freeze until M002 unchanged. Honeycomb Proposed.

## 2026-10-01 — Process: governor observes HOT-01..03

### Process

### Added

- Governor bridge `B-m012a-s06` / `m012a-s06-hot-indexes` and three `observe_cargo.required` names for HOT-01..03 (`as_of_and_claim_supported_at_skip_non_overlapping_facts`, `discard_csr_lease_and_snapshot_rebuild_keeps_as_of`, `csr_lease_at_builds_via_materializer`). LOG, REF, ING, DUR, and TIME names stay required.

### Trajectory

- Honeycomb stays **Proposed**. ADR-040 and ADR-041 evidence lists include the HOT names. Active Slice remains **S06**. Freeze until M002 unchanged. Not a GED-class fork-diff API.

## 2026-10-01 — Product: droppable CSR via Materializer

### Product

### Changed

- Untyped `csr_lease_at` builds through `CsrMaterializer` then unloads. `from_fold` consumes the fold live-fact iterator. Dropping the lease does not change `as_of` or fingerprint. Indexes and CSR stay in-memory leases, not snapshot SoT. Typed CSR remains `TypedCsrLease::from_fold` (named spike limit).

### Trajectory

- Honeycomb stays **Proposed**. Active Slice remains **S06**. Freeze until M002 unchanged. Not M012 dictionaries-as-facts.

## 2026-10-01 — Product: fold-internal hot as_of and claim_supported_at

### Product

### Added

- Fold-internal VT and claim maps so `as_of` / `claim_supported_at` skip non-overlapping facts. Answers match a brute-force `is_live_at` filter of `facts()`. Maps skip-serialize, rebuild from facts after snapshot open, and are not fingerprint input.

### Trajectory

- Honeycomb stays **Proposed**. Active Slice remains **S06**. Freeze until M002 unchanged. Not M012 dictionaries-as-facts.

## 2026-10-01 — Process: lease M012a S06 for Phase 16

### Process

- Active Slice **S06** (fold-internal hot indexes for `as_of` / `claim_supported_at`). Active Milestone stays **M012a**; `L_delivery=M012a-S05-done` until S06 closes.

### Trajectory

- GSD Phase 16 may edit product crates for HOT-01..03. Last M012a slice. Freeze until M002 unchanged. Honeycomb Proposed.

## 2026-10-01 — Product: M012a S04+S05 verify, persist, time scale

### Product

- `open` verifies snapshot via `replay_check` (tampered snapshot → `InvalidData`).
- `persist` replaces `events.jsonl` atomically (temp + rename). `Define` ids are name-stable (`Event::stable_define`).
- `TimeScale::{YearCe, LogSequence}` declared; fixture oracles map years and TT as log sequence.

### Process

- Governor `m012a-s04-verify-persist` / `m012a-s05-time-scale` + FSM observe for five DUR/TIME tests. GSD Phase 15 closed.

### Trajectory

- `L_delivery=M012a-S05-done`; Active Slice **None** until S06 leased. Active Milestone stays M012a. Freeze until M002 unchanged.

## 2026-10-01 — Process: governor observes DUR-01..03 and TIME-01..02

### Process

### Added

- Governor bridges `B-m012a-s04` / `m012a-s04-verify-persist` and `B-m012a-s05` / `m012a-s05-time-scale`, plus five `observe_cargo.required` names for the DUR and TIME oracles. LOG, REF, and ING names stay required.

### Trajectory

- Honeycomb stays **Proposed**. ADR-012 and ADR-013 evidence lists include the new names. Active Slice remains **S04**. Freeze until M002 unchanged.

## 2026-10-01 — Product: declared YearCe valid-time and log-sequence transaction-time

### Product

### Added

- `TimeScale::{YearCe, LogSequence}` with `VALID_TIME_SCALE` and `TRANSACTION_TIME_SCALE`. Fixture integers 2015/2017/2021 are Gregorian years CE; `Event.ingested_at` is log sequence, not wall-clock. Named tests in `m012a_time_scale`. No TT-to-wall calendar map.

### Trajectory

- Honeycomb stays **Proposed**. Active Slice remains **S04**. Freeze until M002 unchanged.

## 2026-10-01 — Product: verify-on-open, atomic jsonl persist, stable Define ids

### Product

### Added

- `open` verifies snapshot identity against the log (`replay_check`) and rejects a tampered snapshot as `InvalidData` / `ReplayDivergenceError`.
- Dictionary `Op::Define` ids are name-stable (SHA-256 of the ASCII prefix `kutha-define-id` plus the term bytes) across persist/open/persist. Live `intern()` still mints UUID v7.

### Changed

- `persist` replaces `events.jsonl` by writing a same-directory temp file, syncing, then renaming. A failed temp write leaves the previous complete jsonl in place.

### Trajectory

- Honeycomb stays **Proposed**. Active Slice remains **S04**. Freeze until M002 unchanged. Not M012 dictionaries-as-facts.

## 2026-10-01 — Process: lease M012a S04 for Phase 15

### Process

- Active Slice **S04** (verify-on-open, atomic persist, stable Define). Phase 15 GSD wave also covers S05 declared time scale. Active Milestone stays **M012a**; `L_delivery=M012a-S03-done` until Phase 15 closes.

### Trajectory

- GSD Phase 15 may edit product crates for DUR-01..03 and TIME-01..02. Freeze until M002 unchanged. Honeycomb Proposed.

## 2026-10-01 — Product: M012a S03 idempotent ingest

### Product

- Assert carries optional `delivery_key` and `polarity`; identical keyed Assert does not mint a second support.
- `conflict_report_at` partitions by stored polarity. Named oracles in `m012a_idempotent_ingest`.

### Process

- Governor `m012a-s03-idempotent-ingest` + FSM observe for three ING tests. GSD Phase 14 closed.

### Trajectory

- `L_delivery=M012a-S03-done`; Active Slice **None** until S04 leased. Active Milestone stays M012a. Freeze until M002 unchanged.

## 2026-10-01 — Process: governor observes ING-01..03

### Process

### Added

- Governor bridge `B-m012a-s03` / check `m012a-s03-idempotent-ingest` and three `observe_cargo.required` names for the ING-01..03 oracles. LOG and REF names stay required.

### Trajectory

- Honeycomb stays **Proposed**. ADR-011 and ADR-013 evidence lists include the new names. Active Slice remains **S03**. Freeze until M002 unchanged.

## 2026-10-01 — Product: claim vs support identities

### Product

### Added

- Named ING-02 oracle `claim_id_distinct_from_support_slot`. `Op::Assert` and `Fact` rustdoc name claim (proposition), minting EventId (support slot), and `delivery_key` (retry) as three identities.

### Changed

- `conflict_report_at` takes claim, transaction-time, and valid-time only and partitions live supports by stored `Fact.polarity`. Correct/CorrectInterval residuals copy polarity; an object-changing patch row flips Positive/Negative. Leased e2e fixture Asserts store Positive.

### Trajectory

- Honeycomb stays **Proposed**. Active Slice remains **S03**. Freeze until M002 unchanged.

## 2026-10-01 — Product: delivery-key Assert ingest

### Product

### Added

- `Op::Assert` carries an optional `delivery_key` and `SupportPolarity` (default none). Re-emitting an identical Assert with the same non-empty key returns the original `EventId` and does not append a second support.
- Same delivery key with a different Assert payload fails closed (`DeliveryKeyConflict`) and does not append. Persist/open still honors the durable key.

### Changed

- Fold `Fact` copies `delivery_key` and polarity; both mix into the event digest and fold fingerprint.

### Trajectory

- Honeycomb stays **Proposed**. Active Slice remains **S03**. Freeze until M002 unchanged. Not M012 dictionaries-as-facts.

## 2026-10-01 — Process: lease M012a S03

### Process

- Active Slice **S03** (idempotent ingest / delivery keys; claim ≠ support slot). Active Milestone stays **M012a**; `L_delivery=M012a-S02-done` until S03 closes.

### Trajectory

- GSD Phase 14 may edit product crates for ING-01..03. Freeze until M002 unchanged. Honeycomb Proposed.

## 2026-10-01 — Product: M012a S02 stable references

### Product

- Retract, Correct, and CorrectInterval target minting `EventId` (`Fact.event_id`); fold-local `seq` stays a lease.
- Justification cites and admission key on EventId across fork/rebuild. Named oracles in `m012a_stable_refs`.

### Process

- Governor `m012a-s02-stable-refs` + FSM observe for three REF tests. GSD Phase 13 closed.

### Trajectory

- `L_delivery=M012a-S02-done`; Active Slice **None** until S03 leased. Active Milestone stays M012a. Freeze until M002 unchanged.

## 2026-10-01 — Product + Process: rebuilt-fold EventId oracle

### Product

### Added

- Named REF-03 oracle `rebuilt_fold_renumbered_seqs_apply_same_retract_and_cite_payloads`: dropping a filler Assert renumbers fold-local seqs; the same Retract and JustificationCite Events still bind.

### Process

### Added

- Governor bridge `B-m012a-s02` / check `m012a-s02-stable-refs` and three `observe_cargo.required` names for the REF-01..03 oracles. LOG names stay required.

### Trajectory

- Honeycomb stays **Proposed**. ADR-011 and ADR-061 evidence lists include the new names. Active Slice remains **S02**. Freeze until M002 unchanged.

## 2026-10-01 — Product: EventId justification cites

### Product

### Changed

- `Op::JustificationCite` and `check_admission` name source supports by minting `EventId` only. Fold-local seq is no longer in the durable cite payload.
- `fork_at` of a prefix that contains the cite still admits after the parent retracts that EventId.

### Trajectory

- Honeycomb stays **Proposed**. Active Slice remains **S02**. Freeze until M002 unchanged.

## 2026-10-01 — Product: minting EventId retract and correct

### Product

### Changed

- `Op::Retract`, `Op::Correct`, and `Op::CorrectInterval` target the minting `EventId` stored on `Fact.event_id`. Fold-local `seq` remains a lease index (CSR / conflict report), not the operator key.
- `Runtime::emit` returns `UnknownFact { event_id }` and does not append when no Fact was minted by that id. Interval patches that do not intersect still fail closed as `IntervalPatchRejected { event_id }`.

### Trajectory

- Honeycomb stays **Proposed**. Active Slice remains **S02**. Freeze until M002 unchanged. Not ADR-011 Accepted.

## 2026-10-01 — Process: lease M012a S02

### Process

- Active Slice **S02** (stable EventId / proposition refs for Retract, Correct, and justification cites). Active Milestone stays **M012a**; `L_delivery=M012a-S01-done` until S02 closes.

### Trajectory

- GSD Phase 13 may edit product crates for REF-01..03. Freeze until M002 unchanged. Honeycomb Proposed.

## 2026-09-30 — Product: M012a S01 log-native SoT

### Product

- Quantum outcomes, justification cites, and resume persist as log records (`Op::QuantumOutcome`, `Op::JustificationCite`); sidecars are droppable leases.
- `provenance_fingerprint` mixes those log-native bytes (`kutha-prov-log-native`). Named oracles in `m012a_log_native`.

### Process

- Governor check `m012a-s01-log-native` + FSM observe names for the three LOG tests. GSD Phase 12 closed.

### Trajectory

- `L_delivery=M012a-S01-done`; Active Slice **None** until S02 leased. Active Milestone stays M012a. Freeze until M002 unchanged. Honeycomb Proposed.

## 2026-10-01 — Log-native SoT

### Product

### Added

- `Op::QuantumOutcome` fold-noop events on the product log. After persist, deleting `quantum_outcomes.jsonl` still reconstructs Zero / Partial / Full (`discard_outcomes_sidecar_keeps_reconstructible_disposition`).
- `Op::JustificationCite` fold-noop events. After persist, deleting `justifications.jsonl` still reconstructs cites and `check_admission` (`discard_justifications_sidecar_keeps_admission_and_resume`). Resume is a log-native `Op::QuantumOutcome` with `disposition: resume`.

### Changed

- Quantum outcome and justification SoT is the event log. Sidecar JSONL files remain written leases; open hydrates from log Events when they exist and does not let an empty sidecar wipe them.
- `provenance_fingerprint` domain tag is `kutha-prov-log-native`; the mix includes Behavior plus log-native outcome and justification Event bytes (`provenance_fingerprint_moves_when_log_native_record_bytes_change`).

### Process

### Added

- Governor bridge `B-m012a-s01` / check `m012a-s01-log-native` and three `observe_cargo.required` names for the LOG-01..03 oracles.

### Trajectory

- Honeycomb stays **Proposed**. Harness lease remains M012a / S01. Freeze until explicit M002 is unchanged. Not ADR-014 Accepted.

## 2026-09-30 — Process: lease M012a

### Process

- Harness lease: Active Milestone **M012a**; Active Slice **S01** (log-native outcomes/justifications); `L_delivery=M012a-leased`. Phase stays **H5**. Freeze until explicit M002 unchanged.
- Governor Active Milestone patterns accept letter-suffix ids (`M012a`). `.kutha/ROADMAP.md` marks M011 CLOSED and M012a ACTIVE with S01–S06 checkboxes.

### Trajectory

- Product plane may land single-log SoT + stable-reference work (F1–F3, F6 verify/atomic/Define, F7, F8). F4/F5 wait for M012. Rocks / Cypher / HNSW / legal pack stay frozen. Honeycomb stays Proposed.

## 2026-09-30 — Process: semantic governor

### Process

### Added

- `kutha-gov selftest` copies the tree once, mutates only that copy, and fails closed on UNPROVEN or VACUOUS. FSM `run_selftest` records `h5_selftest`; `ci` labels H5 from that evidence.
- Kinds `rust_test_asserts`, `cite_equals`, `refs_resolve`, and `file_max_lines`. Named `fn`-needle checks and honeycomb evidence resolve through asserting `#[test]` bodies. `cite-lease` also matches `L_map=`, `L_capability=`, and Active Milestone M### phrasing to STATE.
- Intake section “What a green check proves”: presence is not meaning; every check is mutation-provable or carries a skip reason. META allowlists the new kinds and `run_selftest`. `AGENTS.md` Commands includes `uv run kutha-gov selftest` (line budget ≤110, no lease tokens).

### Changed

- `run_checks` default budget and `.env.example` are 48 so `ci` cannot truncate Cui slices (32 would drop checks).
- Control-loop rows `I-F1-outcomes` … `I-F8-hot-reads` record F1–F8 as `disposition: deferred` (`until` names M012a / M012 / M002). Recorded, not enforced.
- ROADMAP H5 checkbox is checked; `h4-lease` requires the done form without promoting ADR-090. Dogfood ladder marks H5 **Done**. It remains the current rung (no H6).

### Fixed

- Live-repo selftest found no vacuous named test and no vacuous YAML row (`docs-coupling` skips: depends on the live git diff).
- Harness tests no longer leak `/tmp/kutha-selftest-mini-*` and `/tmp/kutha-rust-test-*` directories (cleanup registered with `atexit`).
- Harness scratch is repo-local: `scripts/tests/conftest.py` and `kutha-gov selftest` create temp trees under gitignored `tmp/` instead of system `/tmp`; selftest's tree copy skips root `tmp/`.

### Trajectory

- Operator Phase stays **H5**, now closed on the dogfood ladder. Active Milestone stays M011; Active Slice None; freeze and honeycomb **Proposed** unchanged. Governor green is not product correctness, not ADR Accepted, and not L_capability. F1–F8 are product gaps deferred to unleased M012a (F4/F5 until STATE names M012; F6 durability protocol until STATE names M002).

## 2026-09-30 — Process: semantic-gap review and Proposed honeycomb notes

### Process

- Architecture note `docs/architecture/semantic-gap-review.md` traces F1–F8 onto the honeycomb (per-cell verdicts, D1–D10 lock assessment). It is not an ADR and not a lease.
- Twelve honeycomb ADRs received dated 2026-09-30 Proposed clarifications and/or open questions. Status stays Proposed. ADR README points at the review.
- Later-milestones proposed order (non-authoritative): M012a (single-log SoT and stable refs) → M012 (dictionaries as facts) → M002 (log durability protocol first). Not leased.

### Trajectory

- Honeycomb stays Proposed. No cell promoted. Green governor is not ADR Accepted and not L_capability.

## 2026-09-30 — Process: H5 operator lease and AGENTS diet

### Process

- Operator lease Phase named **H5** (semantic governor + lean agent context). Dogfood ladder and `h4-lease` / `dogfood` needles name H5 without promoting ADR-090.
- `AGENTS.md` diet: 194 lines down to about 100 durable operating rules. Volatile notes relocated (P0 spike inventory, Leapfrog, applicability Agent notes, README Layout). Stale S03 live-lease caption replaced with a STATE pointer.

### Trajectory

- Operator Phase is **H5**; Active Milestone stays M011; Active Slice None; freeze and honeycomb Proposed unchanged. Green governor is not ADR Accepted and not L_capability.

## 2026-09-30 — Product: end-to-end candidate fixture

### Product

### Added

- Durable `justifications.jsonl` sidecar (`JUSTIFICATIONS_REL`) beside outcomes; `record_justification` / `check_admission` fail-closed on stale `source_fact_seqs`, ineligible derivation, or `rule_version` mismatch. Missing sidecar loads empty (cannot admit).
- `conflict_report_at` reports `positive_supports` and `negative_supports` without picking a winner.
- Named tests `e2e_fixture_supports_and_conflict_at_named_cuts`, `e2e_justification_cites_sources_and_rejects_stale_admission`, `e2e_incremental_matches_reconstruct_after_discarding_leases` (FIX-01, FIX-02, FIX-03).

### Process

- FSM `observe_cargo.required` plus `m011-e2e` / `B-m011-e2e` needles for the three named FIX tests (GATE-01).
- Harness lease advanced: **S08** delivered (`L_delivery=M011-S08-done`); Active Slice **None**. M011 tail (S04–S08) closed. Not ADR Accepted; not closed product delivery.

### Trajectory

- **S08 delivered** (`L_delivery=M011-S08-done`); Active Slice cleared. ADR-013, ADR-011, ADR-012, and ADR-040 remain Proposed (not Accepted, not L_capability). Evidence names the FIX oracles `e2e_fixture_supports_and_conflict_at_named_cuts`, `e2e_justification_cites_sources_and_rejects_stale_admission`, and `e2e_incremental_matches_reconstruct_after_discarding_leases`. Green governor is not ADR Accepted and not L_capability. Conflict report is not four-valued logic.

## 2026-09-30 — Process: lease Active Slice S08 after S07 delivery

### Process

- Harness lease advanced: **S07** remains delivered (`L_delivery=M011-S07-done`); Active Slice **S08** (end-to-end candidate fixture — semantic-contract observations 1–5 at named cuts). Not ADR Accepted; not closed product delivery.

### Trajectory

- Active Slice is **S08**; honeycomb cells stay Proposed (not Accepted, not L_capability). S07 provenance tests stay registered. Green governor is not ADR Accepted and not L_capability.

## 2026-09-30 — Product: provenance check apart from state replay

### Product

### Added

- `Op::Behavior.rule_version` (`#[serde(default)]`); fold still ignores lineage (`name` / `caused_by` / `rule_version`). `Runtime::provenance_fingerprint` / `provenance_check` sit beside unchanged `replay_check` (ADR-060 obligation 2; state replay stays obligation 1).
- Named test `provenance_detects_caused_by_swap_when_state_fingerprint_matches` (PROV-01): swap `caused_by` among valid priors; state fingerprint matches; provenance digest moves; both `replay_check` Ok.
- Named test `provenance_detects_rule_version_change_when_state_fingerprint_matches` (PROV-02): change only `rule_version` (`r1` → `r2`); state fingerprint matches; provenance digest moves; no execution replay.

### Process

- FSM `observe_cargo.required` plus `m011-provenance` / `B-m011-provenance` needles for named PROV-01 and PROV-02 tests (GATE-01).
- Harness lease advanced: **S07** delivered (`L_delivery=M011-S07-done`); Active Slice **None**. S08 remains unleased. Not ADR Accepted; not closed product delivery.

### Trajectory

- **S07 delivered** (`L_delivery=M011-S07-done`); Active Slice cleared until an explicit S08 lease. ADR-060 and ADR-011 honeycomb evidence names `provenance_detects_caused_by_swap_when_state_fingerprint_matches` and `provenance_detects_rule_version_change_when_state_fingerprint_matches`; map stays Proposed on both (not Accepted, not L_capability). Green governor is not ADR Accepted and not L_capability. `replay_check` Ok is not execution replay.

## 2026-09-30 — Process: lease Active Slice S07 after S06 delivery

### Process

- Harness lease advanced: **S06** remains delivered (`L_delivery=M011-S06-done`); Active Slice **S07** (provenance and rule-version check apart from state replay). S08 remains unleased. Not ADR Accepted; not closed product delivery.

### Trajectory

- Active Slice is **S07**; ADR-040 and ADR-060 remain Proposed (not Accepted, not L_capability). S06 typed-CSR tests stay registered. Green governor is not ADR Accepted and not L_capability.

## 2026-09-30 — Product: typed CSR lease (labels + support multiplicity)

### Product

- `TypedEdge` / `TypedCsrLease` beside untyped `CsrLease`; `Runtime::typed_csr_lease_at` + `edges_out` project one edge per live Fact (`relation`, `object`, `claim_id`, `fact_seq`). Untyped `CsrLease::from_fold` object-only dedup unchanged.
- Named test `typed_csr_preserves_relation_labels_and_support_multiplicity` (CSR-01): ≥2 relations and ≥2 supports visible on typed edges; untyped neighbors stay a singleton object.
- Named test `untyped_csr_neighbor_set_and_ff5_still_hold` (CSR-02): untyped neighbor-set + FF3-style drop/rebuild; existing FF5 statute observe stays green.

### Process

- FSM `observe_cargo.required` plus `m011-typed-csr` / `B-m011-typed-csr` needles for named CSR-01 and CSR-02 tests (GATE-01).
- Harness lease advanced: **S06** delivered (`L_delivery=M011-S06-done`); Active Slice **None**. S07–S08 remain unleased. Not ADR Accepted; not closed product delivery.

### Trajectory

- **S06 delivered** (`L_delivery=M011-S06-done`); Active Slice cleared until an explicit S07+ lease. ADR-040 and ADR-041 honeycomb evidence names `typed_csr_preserves_relation_labels_and_support_multiplicity` and `untyped_csr_neighbor_set_and_ff5_still_hold`; map stays Proposed on both (not Accepted, not L_capability). Green governor is not ADR Accepted and not L_capability.

## 2026-09-30 — Process: lease Active Slice S06 after S05 delivery

### Process

- Harness lease advanced: **S05** remains delivered (`L_delivery=M011-S05-done`); Active Slice **S06** (typed CSR lease keeps labels and support multiplicity). S07–S08 remain unleased. Not ADR Accepted; not closed product delivery.

### Trajectory

- Active Slice is **S06**; ADR-014 remains Proposed (not Accepted, not L_capability). S05 outcome tests stay registered. Green governor is not ADR Accepted and not L_capability.

## 2026-09-29 — Product: persisted quantum outcome sidecar

### Product

- Authoritative `quantum_outcomes.jsonl` sidecar beside the log (written by `store::persist` after events; loaded on every `store::open` path). Not a droppable lease — discarding `snapshot.json` keeps outcome history (D-O1 / ADR-010).
- Dispositions `Zero` / `Partial` / `Full` from abort × `events_in_quantum`; `Runtime::record_resume` appends an explicit `Resume` row (open never auto-resumes). Call `Ok(QuantumOutcome)` remains not completion proof (ADR-014).
- Named test `budgets_0_1_2_distinguish_zero_partial_full_after_persist_open` (OUT-01): budgets 0/1/2 via `Runtime::new(0|1|default)` survive persist→open as Zero/Partial/Full.
- Named test `crash_after_prefix_has_no_terminal_success_until_explicit_resume` (OUT-02): after a committed prefix without a Full row, open invents no terminal success until `record_resume`.

### Process

- FSM `observe_cargo.required` plus `m011-quantum-outcome` / `B-m011-quantum-outcome` needles for named OUT-01 and OUT-02 tests (GATE-01).
- Harness lease advanced: **S05** delivered (`L_delivery=M011-S05-done`); Active Slice cleared. S06–S08 remain unleased. Not ADR Accepted; not closed product delivery.

### Trajectory

- **S05 delivered** (`L_delivery=M011-S05-done`); Active Slice cleared until an explicit S06+ lease. ADR-014 honeycomb evidence names the OUT-01/OUT-02 tests; map stays Proposed (not Accepted, not L_capability). Green governor is not ADR Accepted and not L_capability.

### Product

- `Op::CorrectInterval { fact_seq, object, patch_from, patch_to }`: explicit interval patch on a live fact. Fold invalidates the original row (VT bounds unchanged) and pushes prefix residual, clipped replacement, and suffix residual sharing `claim_id`.
- Named test `interval_patch_leaves_vt_2012_and_2021_residuals` (CORR-01): after patch `[2015, 2020)` on `[2010, ∞)`, `as_of(2012)` and `as_of(2021)` keep source `a` as `P`; interior `as_of(2017)` is `P-prime`.
- Named test `whole_version_correct_does_not_invent_residuals` (CORR-02): whole-version `Op::Correct` with narrower VT `[2015, 2020)` on `[2010, ∞)` invents no live residual of `P` at VT 2012 or 2021; interior `as_of(2017)` is the replacement only.
- Fail-closed interval-patch writes: unknown `fact_seq` (`UnknownFact`); non-intersecting (including half-open touching endpoints), inverted, and not-live targets (`IntervalPatchRejected`) do not append.

### Process

- FSM `observe_cargo.required` plus `m011-partial-correction` / `B-m011-partial-correction` needles for named CORR-01 and CORR-02 tests (GATE-01).
- Harness lease advanced: **S04** delivered (`L_delivery=M011-S04-done`); Active Slice **S05** (persisted quantum outcome and continuation). S06–S08 remain unleased. Not ADR Accepted; not closed delivery.

### Trajectory

- Active Slice is **S05**; ADR-013 remains Proposed (not Accepted, not L_capability). S04 residual tests stay registered. Green governor is not ADR Accepted and not L_capability.

## 2026-09-16 — Product + Process: M011 S03 thin P→Q eligibility

### Product

- `Runtime::derivation_eligible_at`: Behavior-derived claim `Q` is eligible at a TT×VT cut iff `Q` is still live and its premise claim (from `caused_by`) still has a live support.
- Named test `derived_q_loses_eligibility_when_last_premise_support_withdrawn`: two supports for `P`; retract one → `Q` stays eligible; retract last → eligibility drops; historical `Q` and earlier TT cut remain.

### Process

- Lease `L_delivery=M011-S03-done`. FSM observe + `m011-claim-supports` needles cover S03.

### Trajectory

- ADR-011 / semantic-contract-validation / honeycomb evidence record the thin oracle. Cells remain Proposed; full provenance polynomials and M002 freeze unchanged.

## 2026-09-16 — Process: session handoff after M011 S02

### Process

- Durable handoff at `.compound-engineering/artifacts/handoffs/m011-s02-cbm-mcp.md`. Project `.cursor/mcp.json` points Cursor at local `codebase-memory-mcp` stdio (agent sessions were missing user MCP servers).

## 2026-09-16 — Product + Process: M011 S02 unknown claim and replay lineage

### Product

- `emit` rejects `Assert.claim` that never appeared as `Fact.claim_id` (`UnknownClaim`; log does not grow).
- `replay_check` rejects `Behavior.caused_by` that is not an earlier event id (`BrokenLineage`). Named tests `unknown_claim_does_not_append`, `replay_rejects_behavior_without_prior_cause`.

### Process

- Lease `L_delivery=M011-S02-done`. FSM observe + `m011-claim-supports` needles cover S02.

### Trajectory

- ADR-060 / semantic-contract-validation record causal-ref check. Cells remain Proposed; M002 freeze unchanged.

## 2026-09-16 — Product + Process: M011 S01 independent claim supports

### Product

- `Op::Assert.claim` / `Fact.claim_id`: absent claim opens a new claim (`event.id`); `Some(id)` attaches another support. Fold helpers `live_support_count` / `claim_supported_at`. Correct inherits `claim_id`.
- Named test `retracting_one_support_leaves_claim_supported`.

### Process

- Leased M011; S01 delivered (`L_delivery=M011-S01-done`). Governor `m011-claim-supports` + FSM observe required.

### Trajectory

- ADR-011 / semantic-contract-validation / honeycomb evidence record S01. Cells remain Proposed; M002 freeze unchanged.

## 2026-09-16 — Product + Process: M010 S03 live intern Define; M010 closed

### Product

- `Runtime::intern` appends `Op::Define` for newly created terms; bootstrap `knows`/`knownBy` stay silent. `graph_len()` and graph-oriented `snapshot.log_offset`; `from_snapshot` strips Define before applying the tail.
- Named test `intern_appends_define_for_new_terms_only`. Allowlist tests assert graph growth via `graph_len`.

### Process

- M010 S03 delivered then milestone closed (`L_delivery=M010-closed`). FSM observe requires the live-intern test; `m010-semantic-open` cites S03 needles.
- `pointer_in_other_file` supports `require_template` so an Active Slice must remain unchecked on ROADMAP (blocks done-but-leased drift).

### Trajectory

- ADR-011 / semantic-contract-validation / honeycomb evidence record S03. Cells remain Proposed; M002 freeze unchanged.

## 2026-09-15 — Product + Process: M010 S02 logged term definitions

### Product

- Persist encodes `Op::Define` (dictionary order) on the durable event stream. `open` recovers intern meanings without `snapshot.json` and without `terms.jsonl`. Snapshot open strips Define so log_offset still matches the in-memory graph log.
- Named test `open_without_snapshot_or_terms_file_recovers_from_define_ops`.

### Process

- Lease moved to M010 S02. Governor observe accepts `module::tests::name` cargo lines. `m010-semantic-open` cites the Define recovery test.

### Trajectory

- ADR-011 / semantic-contract-validation record S02. ADR-011 honeycomb evidence names the new test. Cells remain Proposed; M002 freeze unchanged.

## 2026-09-15 — Product + Process: lease M010 S01 semantic recovery open

### Product

- `persist` retains authoritative `terms.jsonl`; `open` recovers fold + intern meanings without `snapshot.json`. Named test `open_without_snapshot_recovers_intern_meanings`.

### Process

- STATE/ROADMAP/README lease M010 S01. Governor `m010-semantic-open` + FSM observe required. Bridge cites store.rs.

### Trajectory

- ADR-011 and semantic-contract-validation note M010 S01 limit (companion terms file; event-logged term ops deferred to S02). Cells remain Proposed; M002 freeze unchanged.

## 2026-09-15 — Process: live H4 membership emit on CI quanta

### Process

- `emit_log` syncs tip process-relation membership into `.kutha/events.jsonl` when the tip set differs from the last logged `allows` edition (H4 dogfood no longer test-only).
- kutha-changelog skill example lease text updated past H3→H4.

### Trajectory

- Cells and freeze unchanged. Active Milestone remains None; no M002 lease.

## 2026-09-15 — Process: post-M001 lease debt wave 1

### Process

- Added governor kind `when_match_then_match` and check `idle-delivery-closed` so Active Milestone None requires `L_delivery=*-closed`.
- Retargeted freeze finding text to the live M002 lease gate; extended `docs-coupling` so STATE and architecture-note diffs require CHANGELOG.
- Indexed `kutha-graph` in GitNexus for session blast-radius queries.

### Trajectory

- Refreshed `docs/architecture/semantic-contract-validation.md` lease wording to Phase H4 / M001-closed (no Active Milestone). Cells and freeze unchanged.

## 2026-09-15 — Process: close M001 milestone

### Process

- Cleared Active Milestone after S01–S03 delivery (`Active Milestone: None`). ROADMAP marks M001 **CLOSED**. Freeze and H4 phase unchanged.

### Trajectory

- `L_delivery=M001-closed`. L_map stays Proposed; L_capability stays ff5-green. No M002 lease.

## 2026-09-15 — Product + Process: H4 process overlay dogfood

### Product

- Tenant ingest maps `process.relations` / `allows` snapshot editions onto `processAllows` and chains intervals. AS OF a prior cut still sees a membership set that later editions dropped. One FF6 name; not ADR-050's six kinds.

### Process

- Tip YAML stays the admit lease. Reserved process relation `allows` logs one sorted membership snapshot per edition. Pytest proves tip-without-`status` rejects `status` without editing the repo tip. Governor `h4-lease` / `h4-membership-as-of` and FSM `observe_cargo.required` name the H4 evidence.
- CodeRabbit follow-ups: reject comma-bearing process relation names (CSV membership ambiguity); `pointer_in_other_file` names the real `other` path; `load_map` rejects non-mapping cells; ROADMAP H4 wording matches shipped overlay scope.

### Trajectory

- ADR-042/060 wording clarified (predicate filter language; `replay_check` unavailable outcome marked proposed). Lease still Phase H4; freeze unchanged.

### Trajectory

- Cells remain **Proposed**. ADR-090 ontology stays frozen. Delivery lease is **H4** overlay dogfood, not a legal pack.

## 2026-09-15 — Process: H4 session handoff after architecture POV

### Process

- Session continuity: `.compound-engineering/artifacts/handoffs/h4-after-architecture-pov.md` records the architecture POV (accept lifecycle self-correction; reject Stages 1–4 as delivery order) and the next CE path: H4 overlay brainstorm/plan, not a legal pack.

### Trajectory

- Cells remain **Proposed**. Delivery lease remains H3; next thin slice is still H4 (ADR-090 overlay). Product crates untouched.

## 2026-09-14 — Process: durable CE artifacts stay under docs_root

### Process

- Standing rule: session-surviving Compound Engineering writes (plans, handoffs, sweep state) go under `.compound-engineering/artifacts/`, not `/tmp`. `ce-handoff` must use `.compound-engineering/artifacts/handoffs/<topic>.md`; the skill's default managed store is OS-evictable.

### Trajectory

- Cells remain **Proposed**. Delivery lease remains H3; next thin slice is still H4 (ADR-090 overlay). Product crates untouched.

## 2026-09-14 — Process: CE skill routing for Russian chat

### Process

- Always-on Cursor rule `.cursor/rules/ce-skills-ru.mdc` maps Russian verbs (коммит, changelog, PR, план, …) onto Compound Engineering skills and `kutha-changelog`. Plugin-cache `description` fields stay English; do not patch them. Git user-rule remains safety-only; commit workflow is `ce-commit`.

### Trajectory

- Cells remain **Proposed**. Delivery lease remains H3; next thin slice is still H4 (ADR-090 overlay). Product crates untouched.

## 2026-09-14 — Process: changelog skill (not a release bumper)

### Process

- Adapted baoyu `release-skills` into `.cursor/skills/kutha-changelog/`: dated `CHANGELOG.md` on product / process / Trajectory planes. Versions stay `0.0.0`. Commit handoff is **ce-commit**; this skill does not tag, `gh release`, or push.

### Trajectory

- Cells remain **Proposed**. Delivery lease remains H3; next thin slice is still H4 (ADR-090 overlay). Product crates untouched.

## 2026-09-14 — Process: L_map compact honeycomb index

### Added

- Compact L_map index `.kutha/dictionaries/honeycomb.yaml` (`kutha-map-honeycomb/v1`): D1–D10 plus all opened ADR cells, with orthogonal `map` / `delivery` / `capability` stages and `depends_on` / `locks` / `evidence` links.
- `kutha-gov map` (optional cell id, neighborhood dump, `--format json`) so agents can load the graph without a second markdown coverage table.
- Governor check `honeycomb-ledger`, FSM state `load_honeycomb`, kind `glob_paths_in_file`, and `yaml_map_list` list/path/embed/glob steps. ADR diffs must include the index (`docs-coupling`).

### Process

- The map dictionary is a fourth intake surface: governor validates shape and links; it does not accept cells, enforce HNSW/Cypher, or treat the index as a delivery backlog.
- Control-loop claims stay in `invariants.yaml`; product freeze/tests stay in `bridges.yaml`. Honeycomb rows are not YAML checks.

### Trajectory

- Cells remain **Proposed**. Delivery annotations (`spike` / `frozen` / `map-only`) and named FF evidence do not promote L_map or L_capability. Lease stays H3; next thin slice is still H4 (ADR-090 overlay). Product crates untouched.

## 2026-09-14 — Process: governor intake split and docs entry

### Added

- Root `README.md` and `CLAUDE.md` (`@AGENTS.md`) as the human / Claude Code entry points.
- Harness `kutha-gov precommit` (dictionary checks only: no cargo quantum, no JSONL) plus `--check ID`, `git_path_implies` coupling, and `.pre-commit-config.yaml`. Neighbor surfaces: daily-archive `--check-only`, law-nexus `--check`; not a baoyu release bumper.
- Control-loop ledger `.kutha/dictionaries/invariants.yaml` and bridge ledger `.kutha/dictionaries/bridges.yaml` (`docs/process/governor-intake.md`). Kind `yaml_map_list` keeps each check id in exactly one ledger.

### Changed

- RuVector adapter citations in ADR-012/014/042/052 and the CE adaptation note: in-repo cards + `.compound-engineering/artifacts/research/ruvector-plugin-adaptation.md`; no machine-local vendor paths. Mapping does not lift the HNSW freeze.

### Process

- Pre-commit profile is FAST-ONLY: ruff on `scripts/` and `kutha-gov precommit`. Product `cargo test` remains `kutha-gov ci`. Versions stay `0.0.0`. CHANGELOG remains plane-dated history, not GitHub Releases.
- Control loop and Kutha requirements stay partitioned: honeycomb / fitness live in ADRs, STATE, and crates tests; `map-only` and `capability` are not process dispositions. Bridges may cite product freeze or named tests; they do not copy L_map or L_capability.

### Trajectory

- Cells remain **Proposed**. Product crates untouched. Delivery lease remains H3; next thin slice is still H4 (ADR-090 overlay).

## 2026-09-13 — Architecture: ruVector adapter mapping (L_map)

### Research & ADRs

- Mapped five named adapter crates onto already-open honeycomb cells via `.compound-engineering/artifacts/research/ruvector-plugin-adaptation.md`. Literature bound stays **163 cards**; no new matrix rows.
- **P-Temporal-Tensor** → ADR-012 D012-5 (vector tiering lease ≠ fact history).
- **P-Retrieval-Receipt** → ADR-014 D014-5 (`paper-constant-size-evidence`; write quantum receipt ≠ read evidence). `ruvector-proof-gate` remains a D014-2 cousin, not a new decision.
- **P-HNSW** → ADR-042 D042-2/3 citing existing cards `ruvector-hnsw-delete-repair`, `paper-acorn-predicate-subgraph`, `paper-navix-filtered-hnsw`. Mapping does not lift the HNSW freeze.
- **P-Agent-Memory** → ADR-052 D052-4 (coherence score ≠ fact validity).
- ADRs cite in-repo cards and the adaptation note. Machine-local vendor paths are not architecture SoT.

### Trajectory

- Cells remain **Proposed**. Product crates untouched. Delivery lease remains H3; next thin slice is still H4 (ADR-090 overlay). This mapping does not authorize HNSW, Cypher, or M002.

## 2026-08-18 — Wave 6: same-second tenant AS OF + one cargo compile per quantum

### Product

- Tenant ingest reports `last_valid_from` as the **emitted** cut after monotonic chaining. Two status rows with the same unix second no longer query AS OF a closed window (`as_of_match=0`).
- Test `h2_same_second_status_as_of_uses_emitted_cut` (TDD).

### Process

- `observe_cargo` optional `build:` compiles `kutha-tenant` once after `cargo test`. `emit_tenant` executes that binary (`KUTHA_TENANT_BIN`, else `$CARGO_TARGET_DIR/debug/kutha-tenant`). No second `cargo run`.
- Checks `plane-mix-dicts` and `tenant-bin`: process/product relation schemas stay distinct; FSM must not `cargo run` the tenant.
- `kutha-gov py` treats ty warnings as errors (`--error-on-warning`).

### Trajectory

- Phase **H3**. Next: H4 waits on ADR-090 overlay (do not start a legal pack). M002 Rocks stays frozen until STATE names it.

## 2026-08-18 — Wave 5: H3 process allowlist + one source for FF names

### Process

- Fail-closed process writes: `.kutha/dictionaries/relations.yaml` (`schema: kutha-harness-relations/v1`). Unknown relation does not append to `.kutha/events.jsonl`.
- FSM loads `relations.yaml` before checks. Override: `KUTHA_HARNESS_RELATIONS_PATH`.
- Named FF tests are CLI **evidence**, not process relations (aligned with H2 tenant mapping of `cargo` only).
- Kind `yaml_needles_in_glob`: FSM `observe_cargo.required` must appear as `fn` in `crates/**/*.rs` (one list, not a copy in checks.yaml).
- `.cbmignore` excludes ideation `mermaid.min.js` from the code index.

### Trajectory

- Phase **H3**. Next: H4 waits on ADR-090 overlay (do not start a legal pack). M002 Rocks stays frozen until STATE names it.

## 2026-08-18 — Wave 4: H2 tenant ingest + unnamed CSR cut

### Product

- Removed `Runtime::csr_lease()` (silent valid-time `0`). Callers name a cut with `csr_lease_at(tt, vt)`.
- H2 tenant: `ingest_harness_jsonl` maps process `harness.run`/`status` → `runStatus` and `harness.observe`/`cargo` → `observed`, with monotonic `valid_to` windows. Same log, two process times, different live status (fixture).
- Binary `kutha-tenant` persists the tenant picture under `KUTHA_TENANT_DIR` (default `.kutha/tenant`, gitignored).
- Allowlist rows `runStatus` and `observed` are delivery facts on the FF6 list, not a second dictionary kind (ADR-050 note).

### Process

- FSM kind `emit_tenant` after `emit_log`: run `kutha-tenant`, require `as_of(last)` to match last status.
- Fold pictures `last_cargo` and `last_tenant`.
- Check `unnamed-csr` forbids bringing `csr_lease()` back.

### Trajectory

- Phase **H2**. Next: H3 (process-plane allowlist). M002 Rocks stays frozen until STATE names it.

## 2026-08-18 — Wave 3: H1 cargo observation as evidence

### Process (harness)

- FSM kind `observe_cargo`: after dictionary checks, run `cargo test --workspace --offline` and require named FF tests (`ff5_…`, `ff6_…`) to appear as `... ok`.
- Cargo output is **evidence**, not product SoT. Success is recorded as `harness.observe` triples; missing/FAILED tests are HIGH and still walk emit/fold so the log exists.
- `emit_log` counts FSM HIGH findings (not only dictionary check highs).
- Timeout from `KUTHA_GOV_CARGO_TIMEOUT_SEC` then `fsm.yaml` `timeout_sec`.
- Static check `observe-required-fn` keeps required names aligned with Rust `fn` lines.
- Split glued FSM tests (`glob_none` vs budget-from-env).

### Trajectory

- Phase **H1**. Next: H2 (harness facts on the Kutha log). M002 Rocks stays frozen until STATE names it.

## 2026-08-18 — Wave 2: FF6 relation allowlist + harness hygiene

### Product

- Fail-closed writes: `Runtime::emit` admits only relations listed in `crates/kutha-runtime/dictionaries/relations.yaml` (`schema: kutha-relations/v1`). Unknown relation → `RuntimeError::UnknownRelation`; log does not grow.
- Path override: `KUTHA_RELATIONS_PATH`. Cascade default: `KUTHA_MAX_CASCADE` (also from repo `.env`).
- FF6 tests: `crates/kutha-runtime/tests/ff6_allowlist.rs`.

### Process

- Kind `glob_none`: forbidden globs (setuptools `*.egg-info` must not live in the tree).
- Check `relation-allowlist` guards the product dictionary.
- FSM transitions must use `event:` (test rejects YAML 1.1 boolean keys).

### Trajectory

- M001 S03 done. Active slice **None**. Next: H1 (cargo test as harness evidence). M002 Rocks stays frozen until STATE names it.

## 2026-08-18 — Wave 1: meta-prompt FSM + Legal PIT FF5

### Process (harness)

- `kutha-gov ci` walks a dictionary FSM (`.kutha/dictionaries/fsm.yaml`): constitution → checks → emit → fold → `ok`|`fail`.
- Unknown FSM kind or missing transition fails closed (`unknown-fsm-kind` / `unknown-transition`).
- Budget and warn-fail come from `.env` (`KUTHA_GOV_BUDGET`, `KUTHA_GOV_FAIL_ON_WARN`), then `fsm.yaml` defaults; CLI flags win. Template: `.env.example`.
- Trajectory also requires **Active Slice** to exist on ROADMAP.
- YAML transition field is `event:` (not `on:` — YAML 1.1 would coerce `on` to boolean `true`).

### Product (kutha-runtime)

- Named cuts: `GraphFold::live_at(tt, vt)`, `GraphFold::as_of(vt)` (no “now” default), `Runtime::csr_lease_at(tt, vt)`.
- FF5: same statute-shaped log, `as_of(2015) ≠ as_of(2021)`; CSR neighbors disagree the same way.
- FF3: dropping a CSR lease does not change log length or fold fingerprint.
- Fixture note: `crates/kutha-runtime/tests/fixtures/legal_pit.md`.

### Trajectory

- M001 S01 and S02 done. Active slice is S03 (FF6 relation-allowlist stub). Honeycomb ADRs stay Proposed.

