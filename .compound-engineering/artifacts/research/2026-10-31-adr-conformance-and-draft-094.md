# ADR conformance audit — ADR-011 / 012 / 014 / 020 / 022, and draft ADR-094

Date: 2026-10-31
Branch: `work/main-line`, HEAD `2680cd9`
Scope: read-only audit of five honeycomb cells against the shipped tree, plus a **draft** (not applied) ADR-094.
Constraints honoured: no tracked file edited, no commit, no `.kutha/STATE.md`, no `docs/ADR/`, no `.planning/`, no `crates/`, no `scripts/`, no `honeycomb.yaml` edit; no `graph index`; no `cargo`. Lease: **Active = None** (`.kutha/STATE.md`: "**Active Milestone:** None"), so no product change was attempted or proposed as a leased action.
Method: `rg` for literal/absence questions, direct source reads for shape questions, line-numbered citations for every claim. Test claims name the test file and `fn`.
Prior audits read (not re-derived): `.compound-engineering/artifacts/research/2026-10-08-audit-honeycomb-cells-vs-repo.md:13` (cell 020), `:15` (cell 022), `.compound-engineering/artifacts/research/2026-10-04-post-m002-audit-adr.md:43` (cells with no falsifiable product-cell test).

YAML state for the audited cells (`.kutha/dictionaries/honeycomb.yaml`): ADR-011 `map: Proposed / delivery: spike / capability: named / evidence: 37`; ADR-012 `spike / none / 17`; ADR-014 `spike / none / 16`; ADR-020 `spike / none / 0`; ADR-022 `map-only / none / 0`. Every cell stays **Proposed**; nothing in this report accepts a cell.

---

## 1. Shipped workspace shape (measurements)

| Measure | Value | Evidence |
|---|---|---|
| Workspace members | exactly 2 | `Cargo.toml:3` — `members = ["crates/kutha-common", "crates/kutha-runtime"]` |
| Crates on disk | exactly 2 directories | `crates/` listing (`ls crates/`) |
| `app-shell` crate | absent | `Cargo.toml:3`; no `crates/app-shell` |
| `modules/<slice>` directory | absent | `find crates -type d`: only `crates/kutha-common{/src,/tests,/tests/ui}`, `crates/kutha-runtime{/src,/src/bin,/tests,/benches,...}` |
| `domain/` / `ports/` / `adapters/` dirs | absent | same `find`; also confirmed by prior audit `2026-10-08-audit-honeycomb-cells-vs-repo.md:13` |
| Binaries | 2, both **inside** the runtime package | `crates/kutha-runtime/src/bin/kutha-migrate.rs`, `crates/kutha-runtime/src/bin/kutha-tenant.rs` |
| `kutha-common` src | 6 files, 2,160 lines, 74 `pub` items, 0 `pub(crate)`, 48 re-exported names | `crates/kutha-common/src/lib.rs:10-26` (5 private `mod` at `:4-8`); measurement over `crates/kutha-common/src` |
| `kutha-runtime` src | 34 `.rs` files (33 modules + `lib.rs`) plus 2 bins; 13,928 lines incl. a 750-line `#[cfg(test)]` module (engine ≈13,178); 339 `pub` items, 179 `pub(crate)`, 69 re-exported names, 32 `#[doc(hidden)]` | `crates/kutha-runtime/src/lib.rs:39-70`, `quantum.rs:867-869`; measurement over `crates/kutha-runtime/src` |
| Module declarations | 32 in `lib.rs`: 29 private `mod`, `pub mod store`, 2 feature-gated `pub mod`; plus `quantum_invariants.rs` included as `#[cfg(test)] mod tests` | `crates/kutha-runtime/src/lib.rs:3-37`; `crates/kutha-runtime/src/quantum.rs:867-869` |
| `kutha-runtime` non-src (tests/benches) | 46,087 lines — 3.3× the engine source | measurement over `crates/kutha-runtime/{tests,benches}` |
| Largest engine modules | `store.rs` 1409, `fold.rs` 1108, `segment.rs` 1008, `persist_append.rs` 939, `pipeline.rs` 935, `eval.rs` 872 — 6 files ≈ 45% of engine source | per-file line counts |
| Dependency direction | one-way, `kutha-runtime` → `kutha-common`; `kutha-common` has no dep on runtime | `crates/kutha-runtime/Cargo.toml:15`; `crates/kutha-common/Cargo.toml` (deps: `uuid`, `sha2`, `serde`, `serde_json`) |
| Traits in the engine (candidate Ports) | 2 total: `Materializer` and `VocabBinder` | `crates/kutha-runtime/src/materializer.rs:6`; `crates/kutha-runtime/src/gpml.rs:37` |
| Visibility model | 29 private `mod` + `pub mod store` + 2 feature-gated `pub mod`, then 69 curated re-exports | `crates/kutha-runtime/src/lib.rs:3-70` |

Shape in one line: **`common` + one runtime crate (32 declared modules, 29 of them private) + two bins in that same package**. There are no slice crates and no composition-root crate.

---

## 2. ADR-020 — Cargo Workspace & Port Isolation

### 020-A. D020-1 target shape

**Claim** — `docs/ADR/ADR-020-cargo-workspace-port-isolation.md:33`:

> "Target shape (STCA guide): `app-shell` (sole composition root) + `common` + `modules/<slice>` with `domain/` / `internal/` / `adapters/` / Ports in `lib.rs`."

**Evidence** — the shipped workspace has 2 members and no `app-shell` or `modules/` (`Cargo.toml:3`; `find crates -type d`). The 32 engine modules sit flat in `crates/kutha-runtime/src/` (`lib.rs:3-37`). The two binaries live inside that package (`src/bin/*.rs`), so the "sole composition root" is a *target*, not a directory.

**Verdict**: **deviates** (shape), and the deviation is inside ADR-020's own P0 allowance:

> "- Over-slicing before P0 spike. P0 may be `common` + one runtime crate." — `ADR-020:66`
> "1. P0 crate split vs monolith-until-spike." — `ADR-020:83` (Open Research Question 1)

**Class**: **(b) acceptable-and-needs-a-clarification** — the ADR names the shipped shape as a legal P0 variant but never records that HEAD *is* that variant. That is exactly the gap a DRAFT ADR-094 closes.

### 020-B. D020-2 law 1 — domain ignorance

**Claim** — `ADR-020:37`: "**Domain ignorance** — `domain/` without sqlx/ORM/HTTP."

**Evidence** — there is no `domain/` directory, so the law as written has no subject. The checkable proxy at crate granularity holds:

- `crates/kutha-common/Cargo.toml` dependencies: `uuid`, `sha2`, `serde`, `serde_json` only (no I/O, no ORM, no HTTP).
- `rg "std::fs|std::net|std::process|std::env" crates/kutha-common/src/*.rs` → no matches.
- `kutha-runtime` carries storage I/O: `std::fs` in `store.rs`, `segment.rs`, `wal.rs`, `persist_append.rs`; optional `rocksdb` at `crates/kutha-runtime/Cargo.toml:23`.
- No mandatory network adapter anywhere: the full dependency list is `sha2`, `serde`, `serde_json`, `crc32fast`, `rustc-hash`, `smallvec`, optional `rocksdb` (`Cargo.toml:16-23`), which satisfies `ADR-020:33`'s "Hot path has **no mandatory network adapters**".

**Verdict**: **conforms at crate granularity; the law as literally written cannot be checked** (no `domain/`).

**Class**: **(a) documentation gap** — record the crate-level reading ("`kutha-common` stays I/O-free") in the clarification; a grep-based gate is available today.

### 020-C. D020-2 law 2 — private-by-default

**Claim** — `ADR-020:38`: "**Private-by-default** — only Ports leave the crate (`pub(crate)` otherwise)." Reinforced at `ADR-020:61`: "Agents cannot import materializer internals as 'just a helper.'"

**Evidence** — the first half is real: 29 of 32 module declarations are private `mod` (`lib.rs:3-32`), and 179 `pub(crate)` items exist. The second half does not hold:

- 69 names are re-exported from `lib.rs:39-70`, including engine internals: `GraphFold`/`Fact` (`lib.rs:41`), `CsrLease`/`TypedCsrLease` (`lib.rs:39`), `Prepared`/`QueryIr` (`lib.rs:59-62`), `EventLog` (`lib.rs:45`), `Snapshot` (`lib.rs:67`).
- `pub mod store` (`lib.rs:30`) exposes a whole module: 28 `pub fn` in `store.rs`, of which 10 are test instrumentation or failpoints — `persist_dir_fsync_count` (`store.rs:47`), `replace_via_tmp_file_sync_count` (`store.rs:53`), `reset_replace_via_tmp_file_sync_count` (`store.rs:59`), `persist_with_failpoint` (`store.rs:832`), `abort_events_jsonl_replace_before_rename` (`store.rs:942`), `abort_wal_replace_before_rename` (`store.rs:950`), `abort_snapshot_replace_before_rename` (`store.rs:955`), `migrate_with_failpoint` (`store.rs:694`), `migrate_with_verify_fault` (`store.rs:709`), `open_for_append_with_failpoint` (`store.rs:1140`).
- 32 `#[doc(hidden)]` items in `crates/kutha-runtime/src` — hidden from docs, still importable.

`#[doc(hidden)]` is not a boundary: nothing in the compiler stops a future pack crate from importing `kutha_runtime::store::abort_snapshot_replace_before_rename`. The Port-only export rule is **not enforced**; it is a convention over a curated list.

**Verdict**: **deviates**.
**Class**: **(b) acceptable-and-needs-a-clarification** today (there is no second consumer crate to import internals; the "engine API" is deliberately wide for the 46k lines of integration tests, e.g. `facade_statute.rs`, `m024_write_path.rs`). It becomes **(c) architecture risk** the moment a second vertical pack exists, because that is the exact "import internals as a helper" failure `ADR-020:61` forbids and there is no gate to stop it.

### 020-D. D020-2 law 3 — single composition root

**Claim** — `ADR-020:39`: "**Single composition root** — only `app-shell` wires concrete adapters."

**Evidence** — two binaries exist, both in the runtime package: `src/bin/kutha-migrate.rs` (operator migrate, `kutha-migrate.rs:1-12`) and `src/bin/kutha-tenant.rs` (H2 harness ingest, `kutha-tenant.rs:1-8`). Each is a separate Cargo target, so each sees only the library's public API — the *effect* of the law (no bin can reach internals) holds structurally. The *shape* (one root crate) does not. Concrete adapter wiring also exists **inside** the engine module layer: `rocksdb` behind `rocks-index` / `rocks-index-spike` features (`crates/kutha-runtime/Cargo.toml:10-12`, `lib.rs:34-37`), i.e. an optional locator adapter inside `kutha-runtime`, not in a root. That is permitted by the freeze rule ("optional dependencies only", `.kutha/STATE.md` thaw map) but it is not what D020-2 law 3 describes.

**Verdict**: **deviates** in shape; functionally benign today.
**Class**: **(b)** — two bins are an operator surface, not an adapter graph; record it. Note for the revisit trigger: the first *non-optional* adapter (a network or I/O port a pack must swap) is the point where law 3 starts to matter.

### 020-E. D020-3 hard separations

**Claim** — `ADR-020:50-54`: `Cargo crate ≠ Named graph / tenant`; `Port trait ≠ Event log`; `app-shell wiring ≠ Pack activate`; `Merkle world hash ≠ SoT`; `Hexagon-inside-slice ≠ Repo-wide ports/ folder`.

**Evidence** — no repo-wide `ports/` / `adapters/` / `domain/` tree exists (`find crates -type d`), so the rejected alternative was not taken. Tenancy is a fold slice via `ingest_harness_jsonl` (`crates/kutha-runtime/src/tenant.rs`, exported at `lib.rs:68`), not a crate — the separation holds.

**Verdict**: **conforms** (vacuously for the hexagon clause, positively for the "no repo-wide ports tree" clause).
**Class**: n/a.

---

## 3. ADR-022 — Vertical-Slice Module Conventions

### 022-A. D022-1 slice = own crate, hexagon inside

**Claim** — `docs/ADR/ADR-022-vertical-slice-module-conventions.md:30`: "Each vertical slice owns: domain types, relation behaviors, dictionary facets, optional materializers, Ports. Hexagon (ports/adapters) lives **inside** the slice, not as a repo-global layer."

**Evidence** — no slice crate exists (`Cargo.toml:3`). What does exist are cross-cutting capability modules inside `kutha-runtime`: pack lifecycle `pack_lifecycle.rs` (209 lines), `pack_registry.rs` (58), `pack_manifest.rs` (302), enrichment `enrichment.rs` (198), parser `gpml.rs` (501), path policy `security.rs` (158), harness tenant `tenant.rs` (141). None of them has `domain/` / `internal/` / `adapters/` submodules or slice-local Port traits; the only traits are `Materializer` (`materializer.rs:6-18`) and `VocabBinder` (`gpml.rs:37`), both engine-level, not slice-level.

**Verdict**: **unimplemented**. The ADR predicted this: "Conventions without a first slice remain slogans until 090/093 spike" (`ADR-022:57`), and honeycomb.yaml already records `delivery: map-only`.
**Class**: **(b)** — the cell is honest about being unimplemented; nothing to fix in the ADR beyond the ADR-094 pointer ("slices are modules until the first pack").

### 022-B. D022-2 slices communicate only via Ports + the log

**Claim** — `ADR-022:33`: "No slice instructs another except by logged events (ADR-010). Shared kernel types live in `common`."

**Evidence** — cross-crate, this holds by construction: `kutha-common` is the shared kernel and `kutha-runtime` depends on it one-way (`Cargo.toml:3`, `crates/kutha-runtime/Cargo.toml:15`). Inside the single crate, "slices" (the capability modules) call each other directly through `crate::` paths — e.g. `security.rs:9-10` imports `crate::quantum::Runtime` and `crate::query_ir::{QueryIr, QueryRows, execute}`; `enrichment.rs:10` imports `crate::pack_lifecycle::{PackLifecycleState, derive_pack_lifecycle}`; `admission.rs:1-8` imports `crate::pipeline::ReceiptMode`, `crate::quantum::{...}`, `crate::receipt::QuantumReceipt`. There is no Port trait between them and no compile-time barrier.

**Verdict**: **unimplemented/deviates in spirit** — the law is unenforceable in a single crate; only the log and `kutha-common` are structural.
**Class**: **(b)** now, **(c) architecture risk** at the second pack: without slice crates, a legal pack can call `crate::fold` directly and the "slices communicate only via Ports" claim becomes false without any signal.

### 022-C. D022-3 default vs named graph is compile scope

**Claim** — `ADR-022:38`: "A slice typically owns one or more named-graph IRIs."

**Evidence** — no named-graph IRI exists in crates: `rg -ni "named.?graph|\biri\b" crates/` returns no matches, and the only `Port` match in crates is a comment about "Portable claim" (`crates/kutha-runtime/src/fold.rs:124`, unrelated); queries are Rust methods over the fold (`query_ir.rs`, exported `lib.rs:59-62`).

**Verdict**: **unimplemented** (not in this task's question set; one line for completeness).
**Class**: **(b)** — already `map-only` in YAML.

---

## 4. ADR-011 — lean event schema: the `Op` pointer

**Claim** — `docs/ADR/ADR-011-lean-event-schema-lineage.md:43` (D011-1):

> "**Pointer 2026-10-08:** shipped `Op` kinds are Assert / Retract / Correct / CorrectInterval / Behavior / Define / QuantumOutcome / JustificationCite / StoreBlob / TtAnchor (not the prose labels create/patch/invalidate/`llm.*` from ADR-000 R5)."

**Pointer clause that may cover extras** — `ADR-011:13` (Evidence 2026-10-23):

> "Shipped `Op` kinds also include `Checkpoint` (M154 sealed-prefix receipt via `store::checkpoint_sealed_prefix`; prefix `StoreBlob`, `QuantumOutcome`, `JustificationCite`, and D021 `PackInstall` / `PackActivate` / `PackRetire` kept verbatim). The 2026-10-08 D011-1 pointer listed kinds through `TtAnchor`. This note does not rewrite D011-1…D011-4."

**Shipped truth** — `crates/kutha-common/src/event.rs` `pub enum Op` (`event.rs:165`) has **15** variants, in this order:

| # | Variant | event.rs line | Covered by D011-1 pointer (line 43)? | Covered by Evidence (line 13)? |
|---|---|---|---|---|
| 1 | `Assert` | 172 | yes | — |
| 2 | `Retract` | 185 | yes | — |
| 3 | `Correct` | 188 | yes | — |
| 4 | `CorrectInterval` | 195 | yes | — |
| 5 | `Behavior` | 202 | yes | — |
| 6 | `Define` | 216 | yes | — |
| 7 | `QuantumOutcome` | 220 | yes | — |
| 8 | `JustificationCite` | 230 | yes | — |
| 9 | `StoreBlob` | 240 | yes | — |
| 10 | `TtAnchor` | 245 | yes | — |
| 11 | `PackInstall` | 251 | **no** | yes |
| 12 | `PackActivate` | 259 | **no** | yes |
| 13 | `PackRetire` | 264 | **no** | yes |
| 14 | `EnrichmentProposal` | 271 | **no** | **no** |
| 15 | `Checkpoint` | 287 | **no** | yes |

**Finding.** The ten names in D011-1 match the first ten variants **exactly and in the same order** — no drift inside the named subset, and the prose-label warning ("not create/patch/invalidate/`llm.*`") still holds. Four extras are covered by the line-13 Evidence clause (three `Pack*` + `Checkpoint`). `EnrichmentProposal` (`event.rs:268-279`, the ADR-052 record) is covered by **neither**: `grep -c EnrichmentProposal docs/ADR/ADR-011-lean-event-schema-lineage.md` → `0`. The honeycomb.yaml restage header acknowledges only "M177 C1b pack manifest: ADR-011/012 kept-prefix Pack* note" (`.kutha/dictionaries/honeycomb.yaml:4`); there is no corresponding ADR-011 note for the M178 enrichment record (yaml mentions enrichment only on the ADR-052 row, `honeycomb.yaml:527-528`).

**Verdict**: **deviates** — the pointer is one kind short (1 of 15 uncovered).
**Class**: **(a) documentation gap to fix** — add one Evidence line to ADR-011 naming `EnrichmentProposal` (`event.rs:271`, ADR-052, meta-only, `MetaOpRejected` for user emit at `pipeline.rs:338-350`). This must NOT rewrite D011-1…D011-4 and must not change the cell's Proposed status.

---

## 5. ADR-014 D014-1 — "constant-size receipt"

**Claim** — `docs/ADR/ADR-014-cascade-budgets-quantum-receipts.md:85`:

> "Each completed (or budget-aborted) emit→idle quantum emits a **constant-size** evidence item: commitments to admitted events, I/O, environment digest, link into the log's hash/Merkle structure, authenticator. Size does not leak payload."

Narrowing clarification, `ADR-014:48-49`:

> "**D014-1 constant commitment:** `QuantumReceipt::digest` is `[u8; 32]` (hex form 64 chars on the log outcome). Size does not depend on cascade width."
> "**Width leak residual:** `event_ids: Vec` / `events_in_quantum` remain resume leases that may expose quantum width. Stripping them is a later format bump, not this cell's Accepted gate."

**Evidence**

- The commitment: `pub digest: [u8; 32]` (`crates/kutha-runtime/src/receipt.rs:13`), computed as SHA-256 over the admitted event digests plus the abort flag (`receipt.rs:18-29`).
- The *record* is not constant: `QuantumReceipt` also holds `pub event_ids: Vec<EventId>` (`receipt.rs:12`), and the on-log form carries it: `Op::QuantumOutcome { quantum_id, disposition, aborted_on_budget, events_in_quantum: usize, event_ids: Vec<EventId>, receipt_digest_hex, resume_of }` (`crates/kutha-common/src/event.rs:220-228`), written by `pipeline::outcome_op` (`crates/kutha-runtime/src/pipeline.rs:326-336`, with `event_ids: row.event_ids.clone()` at `pipeline.rs:332`).
- Content is narrower than D014-1's list: the digest covers admitted event digests + abort flag only — no I/O, no environment digest, no log-hash link, no authenticator. `ADR-014:109` already states this ("The P0 receipt is not yet the full cryptographic contract above.").
- Test: `receipt_digest_len_constant_across_cascade_widths` (`crates/kutha-runtime/tests/m024_write_path.rs:83`) asserts `digest.len() == 32` for narrow and wide cascades (`:118-123`).

**Verdict**: **conforms on the commitment; deviates on the record and on receipt content.**
Size independence holds for `digest` per quantum; it does not hold for the persisted outcome row, whose size grows with quantum width: 16 bytes per admitted event in RAM (`pub type EventId = Uuid`, `crates/kutha-common/src/event.rs:10`) and 36 characters per event once serialized as JSON, since `Uuid` serializes as a hyphenated string. The ADR text that says "Size does not leak payload" is true of the commitment, false of the log record, and the ADR's own clarification says so.

**Class**: **(a) documentation gap** — the residual is already recorded (`ADR-014:49`), so the fix is one sentence in D014-1's vicinity scoping "constant-size" to the commitment. Not an architecture risk: the digest is what a verifier reads, and the width leak is a resume lease, not a payload body.

---

## 6. ADR-012 — snapshots, tiers, vacuum

### 012-A. D012-1 snapshots are leases

**Claim** — `ADR-012:105-107`: "Snapshots shorten Yankin reconstruction. They are not SoT. Placement is a Control/pack policy … Compaction that **reorganizes layout** is allowed. Compaction that **drops losers silently** is TOKI-illegal."

**Evidence**

- Lease object exists: `Snapshot { log_offset, next_tt, max_cascade, fold, dict_strings, knows, known_by, behaviors, behaviors_exact }` (`crates/kutha-runtime/src/snapshot.rs:5-26`), rebuilt from the runtime (`hydrate.rs:114-124`).
- Two refresh paths exist and are named separately: append-path `Persister::checkpoint` (`persist_append.rs:198-211`, doc comment explicitly says it is *not* the sealed-prefix receipt) and the sealed-prefix rewrite `store::checkpoint_sealed_prefix(dir)` (`store.rs:177`, holds `kutha.lock` at `store.rs:178`, requires `manifest.json` at `:179-183`, rewrites the sealed prefix into `StoreBlob` + `Op::Checkpoint`).
- Lease identity is verified, not trusted: `snap.log_offset > fold_affecting_count(&events)` fails closed with "snapshot log_offset is ahead of the committed prefix" (`store.rs:1185-1190`).
- Placement by query load: **absent**. No query-distribution input, no compaction knob, no cadence policy in crates (`rg -ni "vacuum|retention|legal.?hold"` over crates finds only two doc comments).

**Verdict**: lease semantics **conform**; the "placed by query load" half is **unimplemented**.
**Class**: **(b)** — placement is by design a Control/pack policy (`ADR-012:107`), so its absence is a deferral, not a violation. A one-line note ("P0 cadence is a fixed append-path checkpoint every `CHECKPOINT_EVERY = 4096` events, `persist_append.rs:39`") would make the deferral explicit.

### 012-B. D012-2 aging pipeline: mutable adj → frozen CSR

**Claim** — `ADR-012:109-111`: "Hot write-friendly adjacency at the top of the pipeline; AP-friendly CSR after aging (BACH). … LSM levels are not truth."

**Evidence** — what exists is a **rebuild**, not an aging pipeline:

- `CsrLease::from_fold(fold, tt, vt, vertex_count)` allocates one bucket per vertex, walks `fold.live_facts_at(tt, vt)`, sorts and dedups each row, and concatenates (`crates/kutha-runtime/src/csr.rs:13-38`). `TypedCsrLease::from_fold` does the same with `TypedEdge` (`csr.rs:79-108`).
- `CsrMaterializer::build` simply calls `CsrLease::from_fold` and stores the result (`materializer.rs:41-53`); `unload` drops it (`materializer.rs:55-58`).
- There is no mutable adjacency tier, no level/frozen state, no incremental `apply(delta)`. Nothing ages: every build is a full O(live facts) reconstruction. This matches the prior finding for ADR-040 ("No `apply(delta)`/`snapshot`/`rollback`", `2026-10-08-audit-honeycomb-cells-vs-repo.md`, cell 040 row).

**Verdict**: **deviates** — D012-2 is written as a decision ("Hot write-friendly adjacency at the top of the pipeline"), not as a deferral, and the shipped code has only the tail end (a frozen CSR picture) with no mutable head.
**Class**: **(a) documentation gap** — unlike D012-4 ("P0 may be append-only-forever"), D012-2 has no P0-deferral sentence, so today the tree contradicts a decision that reads as current. Add the P0 allowance ("CSR may be rebuilt from the fold per cut; the mutable adjacency tier arrives with a materializer pack").

### 012-C. D012-3 tiers (SEM edge shards, cold archive)

**Claim** — `ADR-012:113-115`.

**Evidence** — nothing: no SSD edge shard, no cold archive, no `verus`-style tier code (`rg -n "semi.external|edge shard"` over crates → none; the only storage tiers are `KUTHASEG` segments and an optional Rocks locator). Out of this task's question set; one line only.
**Verdict**: **unimplemented**. **Class**: **(b)**.

### 012-D. D012-4 vacuum + legal hold; operator CLI

**Claim** — `ADR-012:117-119`: "Vacuum physically removes expired transaction-time history under a **declared policy**, and logs that the drop happened. A **legal hold** is a grant-shaped fact: vacuum must fail closed on held ranges. … P0 may be append-only-forever; honeycomb owns vacuum+hold as a pack."

**Evidence**

- No vacuum: `rg -ni "vacuum" crates/` returns two **doc comments that exclude** it — `security.rs:6` ("does not implement grants-as-facts, tenant slices, vacuum authority, …") and `m175_security_policy.rs:3`. No vacuum code, no policy fact kind, no hold fact kind.
- No operator CLI: the only binaries are `kutha-migrate` (Op-wire / pin migration, `kutha-migrate.rs:1-12`) and `kutha-tenant` (harness ingest, `kutha-tenant.rs:1-8`). Neither exposes checkpoint, vacuum, hold, fork, or verify subcommands; `store::checkpoint_sealed_prefix` (`store.rs:177`) is reachable only as a library call (tests call it directly: `b1_checkpoint_receipt.rs:196`, `m178_enrichment.rs:243`).
- Checkpoint receipts do **not** vacuum: `GraphFold::fingerprint` hashes all facts "including dead ones (not vacuum, D012-4)" (`ADR-012:69`).

**Verdict**: **unimplemented** for vacuum/hold and for the operator surface, which is exactly what `ADR-012:119` permits at P0.
**Class**: **(b) acceptable-and-needs-a-clarification** — the P0 allowance is already written; what is missing is the operator-CLI statement ("no operator entry point beyond two bins; checkpoint is library-only"). That one is worth recording in the clarification because "operator CLI" is assumed by other cells (ADR-012:83 anticipates ADR-080 "who may vacuum").

---

## 7. Law-by-law summary

| Law | Holds? | How it can be checked today |
|---|---|---|
| D020-1 target shape `app-shell`+`common`+`modules/<slice>` | **no** (by ADR's own P0 variant) | `Cargo.toml:3` |
| D020-2.1 domain ignorance | **only at crate granularity** (`kutha-common` I/O-free) | dependency list + `rg std::fs` on `kutha-common/src` |
| D020-2.2 private-by-default (Port-only exports) | **no** | 69 re-exports + `pub mod store` + 32 `#[doc(hidden)]` |
| D020-2.3 single composition root | **no** (2 bins, same package) | `src/bin/` |
| D020 "no mandatory network adapters" | **yes** | `crates/kutha-runtime/Cargo.toml:14-23` |
| D020-3 `Cargo crate ≠ tenant` | **yes** | tenancy is `tenant.rs` + fold, no crate |
| D022-1 slice = own crate, hexagon inside | **no** (no slice crate) | `crates/` |
| D022-2 slices only via Ports + log | **no** (direct `crate::` calls; one-way crate dep only) | imports in `security.rs:9-10`, `enrichment.rs:10`, `admission.rs:1-8` |
| D011-1 `Op` pointer | **mostly** (10/10 named match; 1 of 15 kinds uncovered) | `event.rs:165-295` vs `ADR-011:43,13` |
| D014-1 constant-size receipt | **commitment yes, record no** | `receipt.rs:12-29`, `event.rs:220-228` |
| D012-1 snapshots are leases | **yes** | `snapshot.rs:5-26`, `store.rs:1185-1190` |
| D012-1 placement by query load | **no** | no cadence/placement code |
| D012-2 aging mutable adj → CSR | **no** (rebuild only) | `csr.rs:13-38`, `materializer.rs:41-53` |
| D012-4 vacuum + legal hold | **no** (P0 allowance) | `rg vacuum` → doc comments only |

Laws that **cannot be checked as written** because their subject does not exist: domain ignorance (`domain/`), private-by-default (`Ports`), single composition root (`app-shell`), slice hexagon (slice crates).

---

## 8. Deviation register

| # | Deviation | Class | Action |
|---|---|---|---|
| 1 | ADR-020 target shape not shipped: 2 crates, slices as modules, 2 bins in the runtime package | **(b)** — anticipated in `ADR-020:66`/`:83` but never recorded as HEAD | DRAFT ADR-094 records the shape and the revisit trigger |
| 2 | Port-only export rule not enforced; 69 exports + `pub mod store` + 32 `#[doc(hidden)]` seams | **(b)** now → **(c)** at the second pack | ADR-094 D094-2 states what is lost; the pack lease must include the split |
| 3 | Domain ignorance checkable only at crate granularity (`kutha-common` I/O-free) | **(a)** | one clarification line; gate available today |
| 4 | ADR-022 conventions unimplemented (no slice crate, no slice-local Ports, direct module calls) | **(b)** → **(c)** at the second pack | ADR-094 notes slices are modules; no ADR-022 edit needed |
| 5 | `Op` pointer misses `EnrichmentProposal` (`event.rs:271`) | **(a)** | add one ADR-011 Evidence line (Proposed stays) |
| 6 | D014-1 "size does not leak payload" true of digest, false of the log record | **(a)** | scope the sentence to the commitment; residual already at `ADR-014:49` |
| 7 | D012-2 describes an aging pipeline HEAD does not have and lacks a P0-deferral sentence | **(a)** | add the P0 allowance (CSR rebuilt per cut) |
| 8 | D012-1 placement policy and D012-4 vacuum/hold absent; no operator CLI beyond 2 bins | **(b)** | generic deferral already written for vacuum; add the placement + CLI note |

No item in this audit needs a product change, so no lease is requested. Items 1, 2, 3, 4 are the ones the DRAFT ADR-094 addresses.

---

## 9. DRAFT ADR-094 — Workspace shape as shipped

> **DRAFT — not applied.** This text is a proposal for `docs/ADR/ADR-094-workspace-shape-as-shipped.md`. It is not written to `docs/ADR/`, it does not change any cell's status, and it does not authorize a refactor. Reported here as requested.

```markdown
# ADR-094: Workspace Shape as Shipped

## Status

**Proposed** (Space — records the P0 workspace the tree actually has; does not
accept ADR-020 or ADR-022, and does not replace either)

## Date

2026-10-31

## Honeycomb coordinates

- Axes: **Space** (primary)
- Depends on: ADR-000 (D6), ADR-020, ADR-022
- Anticipates: the first vertical pack (ADR-090 / ADR-093)

## Context

ADR-020 D020-1 names the target shape `app-shell` + `common` + `modules/<slice>`
with `domain/` / `internal/` / `adapters/` and Ports in `lib.rs`. ADR-022
D022-1 names a slice as its own crate with a hexagon inside. Neither is what
HEAD contains.

HEAD (`Cargo.toml:3`) is a two-member workspace:

- `crates/kutha-common` — 6 modules, 2,160 lines, 48 re-exports. Pure encoding:
  `Event`, `EventId`, `Op`, interned terms, rule AST. Dependencies are `uuid`,
  `sha2`, `serde`, `serde_json`; no `std::fs`, no `std::net`, no `std::process`.
- `crates/kutha-runtime` — 32 declared modules (29 private, `pub mod store`,
  2 feature-gated), 13,928 lines under `src/` (≈13,178 engine plus one 750-line
  `#[cfg(test)]` module) and 46,087 lines of tests/benches, 69 re-exports.
  This crate holds the kernel (`log`, `fold`, `quantum`, `pipeline`, `store`,
  `segment`, `commit`, `snapshot`) and every capability that would otherwise be
  a slice (pack lifecycle, enrichment, parser, path policy, harness tenant).
- Two binaries inside the runtime package: `kutha-migrate` and `kutha-tenant`.

ADR-020 already allows this as a P0 variant ("P0 may be `common` + one runtime
crate", ADR-020:66; Open Question 1, ADR-020:83) but never records that the
variant is what shipped. This cell records it so the map and the tree agree, and
so the next agent does not re-derive the gap as a defect.

## Decision

### D094-1. The P0 workspace is `common` + one runtime crate

Compile-time Space is preserved **at crate granularity**: `kutha-runtime`
depends on `kutha-common` and nothing depends on `kutha-runtime`; `kutha-common`
stays I/O-free, so the pure kernel types cannot absorb storage or network code.
Vertical slices are **modules inside `kutha-runtime`**, not crates. This is the
narrowest shape that still has one real dependency edge and one enforced purity
boundary; it is not a claim that per-slice isolation exists.

### D094-2. What this shape does not give us

1. **No per-slice crate boundary.** A future pack crate would be the compiler
   barrier ADR-020 and ADR-022 rely on. Today a capability module can call
   `crate::fold` directly; only the log and `kutha-common` are structural.
2. **No private-by-default enforcement.** 69 names leave `kutha-runtime`, plus
   the whole `store` module, including `#[doc(hidden)]` failpoint and counter
   functions. "Only Ports leave the crate" is a convention over a curated list,
   not a gate. `#[doc(hidden)]` hides documentation, not imports.
3. **No composition-root crate.** Two binaries in the runtime package consume
   the same wide library API; an optional `rocksdb` adapter is wired by Cargo
   feature inside the engine crate (permitted by the freeze rule, not by
   D020-2 law 3).
4. **Slice communication is by direct module call plus the log**, not by Port
   traits. The only traits are `Materializer` and `VocabBinder`.

D094-1 and D094-2 together mean: the ADR-020 / ADR-022 target stays a target.
This cell does not degrade those cells and does not make their laws satisfied.

### D094-3. Trigger for revisiting

Split the workspace per ADR-020 / ADR-022 — `app-shell` + per-slice crates —
when **either** happens:

1. **A second vertical pack** is leased (a legal or science slice with its own
   dictionary facets, relation behaviors, materializers, and a reason not to see
   kernel internals), or
2. **A second binary needs isolation** from the engine API — i.e. a binary whose
   correctness argument depends on *not* being able to call `store::*` or
   `fold::*` — or the first **non-optional** adapter must be swappable by a pack.

Until one of those is true, growing modules inside `kutha-runtime` is the
cheaper and honest choice. At that trigger, the split is a lease-sized change,
not a rewrite: the module seams already exist.

**Hard separations:**

```
Two crates at HEAD        ≠  ADR-020 D020-1 satisfied
Module inside runtime     ≠  Vertical slice crate
`#[doc(hidden)]`          ≠  Private
Curated re-export list    ≠  Port-only export gate
```

## Consequences

### Positive

- The map stops implying that per-slice isolation, private-by-default, and a
  composition-root `app-shell` exist.
- The next pack lease has a written, testable trigger instead of an implicit
  "someday".
- No refactor is implied by this cell.

### Negative / risks

- A two-crate workspace makes it cheap to keep adding capability modules and
  expensive to notice when isolation was actually needed. The trigger in D094-3
  is the only thing preventing drift.
- `kutha-runtime`'s wide export surface is now an explicitly recorded property,
  so later shrinking it is a visible, reviewable change rather than noise.

### Non-goals

- Not accepting ADR-020 or ADR-022.
- Not authorizing a crate split, a workspace reshuffle, or a new binary.
- Not defining tenant isolation (ADR-080) or pack install (ADR-021).

## Alternatives Considered

| Alternative | Why not now |
|-------------|-------------|
| Split into `app-shell` + slice crates immediately | No second slice exists; ADR-022:57 says the conventions are slogans until one does |
| Leave the tree undocumented and keep ADR-020 as the only statement | The map then contradicts HEAD, and each audit rediscovers it (2026-10-04, 2026-10-08, this one) |
| Treat "two crates" as D020-1 conformance | False: no `app-shell`, no slice crates, no Port gate |

## Related Decisions

- ADR-020 — workspace target shape and isolation laws
- ADR-022 — slice conventions (unimplemented, `delivery: map-only`)
- ADR-011 — `kutha-common` is the shared encoding kernel
```

---

## 10. What this audit did not verify

- **No build, no test run** (contract: no `cargo`). Every test claim is a source claim about the test file and `fn`, not an observation of a pass.
- **No coverage tooling.** The "what exists" claims come from `crates/` listings, line counts, `rg`, and source reads; absence claims are `rg` over `crates/` only, not over `target/`, fixtures, or generated code.
- **Governor state not re-run.** I did not run `uv run kutha-gov`; the honesty claims about YAML (`delivery` / `capability`) are read from `.kutha/dictionaries/honeycomb.yaml` directly.
- **`docs/ADR/ADR-094-*.md` does not exist** and was not created; §9 is draft text inside this report only.
- Runtime/VM size of `QuantumReceipt` was reasoned from field types, not measured with `std::mem::size_of` (which would need a build).
