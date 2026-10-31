# C2 vocabulary-first milestone sketch (ADR-050 / SIX-01)

- **Status:** planning sketch, read-only. Not a lease, not a GSD artifact, not an ADR, not honeycomb Acceptance.
- **Date:** 2026-10-31. Repo `/root/kutha-graph`, branch `work/main-line`, HEAD `2680cd9`.
- **Inputs:** `.compound-engineering/artifacts/research/2026-10-31-c2-dictionaries-design-spike.md` (read in full),
  `.compound-engineering/artifacts/research/2026-10-31-architecture-gap-register.md`,
  `.cursor/gsd-core/workflows/new-milestone.md`, `.cursor/gsd-core/workflows/plan-phase.md`,
  `.cursor/gsd-core/templates/phase-prompt.md`, `AGENTS.md`, `.kutha/STATE.md`.
- **Lease:** none (`Active Milestone: None`, `.kutha/STATE.md:7-9`). Nothing here is authorized. Two owner actions are
  still required: name the milestone in the lease, and name `SIX-01` in the slice (thaw-map gate, `.kutha/STATE.md:39`).

## 0. Evidence rule

Every code claim carries `file:line` read at HEAD; every test claim names the test file and `fn`. **T1–T12** are the
spike's proposed oracles (§6 of the spike); **T13–T18** are new here and marked NEW. §9 lists what was not verified.

## 1. Milestone identity and the lease text

| Item | Value | Basis |
|---|---|---|
| Harness milestone | **M187** (`S01`) — *identity depends on ordering*: if the test-honesty milestone goes first (recommended in `2026-10-31-m187-test-honesty-sketch.md`, because its defect 4 breaks this sketch's Phase 276 gate under concurrency), this becomes **M188 / v1.81 / phases 277–279** and every `DICT-27x-01` id shifts by two. | last closed is M186 (`.kutha/STATE.md` § Next action) |
| GSD milestone | **v1.80**, name `C2 vocabulary dictionaries` | `init new-milestone`: `latest_completed_milestone` = `v1.79`, `phase_archive_path` = `.planning/milestones/v1.79-phases` |
| GSD phases | **275, 276, 277** | last roadmap phase is 274 (`.planning/ROADMAP.md` § Phases). Keep default numbering; do **not** pass `--reset-phase-numbers` |
| Requirement IDs | `DICT-275-01` … `DICT-277-01` | format precedent `PERF-274-01` (`.planning/REQUIREMENTS.md:8`) |

**Goal (one sentence):** prove the ADR-050 dictionary substrate with the Controlled-vocabulary kind only — a
bi-temporal entry in a reserved namespace, one fold lease, fail-closed conflict rules, and a pack-carried facet —
without a new `Op`, an LLM, capabilities, a binder, or an SMO.

### 1.1 Exact `.kutha/STATE.md` edits the owner pastes (parent controller, never a GSD run)

`Phase: H5` is the harness process phase and does not change; the GSD phases are 275–277.

```text
**Active Milestone:** M187
**Active Slice:** S01
**Phase:** H5

L_delivery=M187-S01
| L_delivery | M187 S01 leased (v1.80) | ADR-050 Accepted / six kinds delivered |
```

`## Next action` first line:

```text
**Active Milestone:** M187 · **Active Slice:** S01 — SIX-01, vocabulary kind first: a dictionary entry is a
bi-temporal fact in the reserved `kutha:dict:` namespace with a droppable fold lease, fail-closed conflict rules,
and a pack-carried facet. No new `Op`, no LLM, no capabilities, no binder, no SMO. Honeycomb stays Proposed;
`I-legal-pack-not-next` stays; not a hard FSM (ADR-000 D3).
```

Thaw-map row (the only thaw taken):

```text
| ADR-050 six dictionaries | vocabulary kind leased as M187-S01; the other five kinds keep this gate |
```

## 2. Phase breakdown

Deliberate deviation from the spike's slice table: spike **S0 (substrate)** and **S1 (vocabulary kind)** are **merged
into Phase 275**, because S0 alone exposes a read API no external test can seed (the write path `pipeline_kind` is
`pub(crate)`, `pipeline.rs:461`) and splitting validation from the constructor leaves a half-validated writer on the
tree. Spike S2 (binder/F10), S3 (Action+Mode), S5 (Relation conformance) are deferred (§4, §8).

### Phase 275 — C2 substrate + Controlled-vocabulary kind (spike S0+S1)

**Goal:** a vocabulary entry is appended as `StoreBlob` + `Assert` in `kutha:dict:vocabulary`, resolves by
`(kind, name)` at an explicit `(tt, vt)`, fails closed on unknown/ambiguous/conflicting/mismatched facets, and is
invisible to product `as_of`.

**Files owned (one worker, serial by nature):**

| File | Change |
|---|---|
| `crates/kutha-common/src/event.rs` | six `pub const` kind names next to `KUTHA_RESERVED_RELS` (`:149-161`); **no `Op` variant, no `digest_bytes` arm** (`:416-633` untouched) |
| `crates/kutha-runtime/src/dictionary.rs` *(new)* | `DictionaryKind`, `DictionaryEntry`, `DictionaryError`, facet structs with `deny_unknown_fields`, schema tag `kutha-dict-facet/v1`, validation, `impl Runtime` read/validate/snapshot-id methods |
| `crates/kutha-runtime/src/fold.rs` | one lease `dict_by_key: IngestMap<(TermId, TermId), FactIdxList>` beside `allows_by_name` (`:286-292`), slot hook like `index_allows_slot` (`:697-712`), rebuild in `bind_reserved` (`:540-567`) |
| `crates/kutha-runtime/src/quantum.rs` | intern the six kind names **at the end** of `Runtime::new`, after the genesis rule (`:436-437`) |
| `crates/kutha-runtime/src/admission.rs` | a `kutha:dict:` branch before the generic allowlist return (`:209-231`); the `kutha:` bypass at `:213-215` must not be widened |
| `crates/kutha-runtime/src/pipeline.rs` | `emit_dictionary_entry` / `retract_dictionary_entry`, blob first, `ReceiptMode::Dictionary` (`:37-44`), pattern `emit_rule_ast` (`:834-868`) |
| `crates/kutha-runtime/src/lib.rs` | re-exports |
| `crates/kutha-runtime/tests/c2_dictionaries.rs` *(new)* | T1–T10, T13–T16 |

**Acceptance tests** (all in `crates/kutha-runtime/tests/c2_dictionaries.rs` unless stated):

| ID | `fn` name | Kind | Source |
|---|---|---|---|
| T1 | `dictionary_write_uses_only_preexisting_ops` | structural | spike |
| T2 | `dictionary_entry_resolves_at_vt_not_latest` | falsifier | spike |
| T3 | `dictionary_entry_unknown_before_its_tt` | falsifier | spike |
| T4 | `dictionary_retract_removes_token_only_after_cut` | falsifier | spike |
| T5 | `dictionary_entries_order_independent_of_index_mode` | falsifier | spike (`fold_query.rs:103-121` is the divergence) |
| T6 | `dictionary_kind_relation_excluded_from_product_as_of` | structural | spike (`fold.rs:540-546`, `dictionary_as_of` at `fold_query.rs:245`) |
| T7 | `dictionary_identical_entry_is_idempotent` | falsifier | spike |
| T8 | `dictionary_conflicting_entry_overlapping_vt_fails_closed` | falsifier | spike |
| T9 | `dictionary_facet_digest_mismatch_fails_closed` | falsifier | spike |
| T10 | `dictionary_facet_of_wrong_kind_fails_closed` | structural | spike |
| T13 | `dictionary_entry_rejects_reserved_and_malformed_names` | falsifier | **NEW** — rule 2 of spike §4.1 (`kutha:` prefix, `true`/`false`, empty, >255 bytes, edge whitespace) has no oracle in T1–T12 |
| T14 | `dictionary_facet_over_size_cap_fails_closed` | falsifier | **NEW** — spike §7.3 says the facet cap "should be stated in the lease"; a stated cap needs a test |
| T15 | `dictionary_entries_survive_persist_reopen_with_same_snapshot_id` | falsifier | **NEW** — T5 covers index mode in one runtime; nothing covers persist/reopen (pattern: `tests/m179_gate_gc.rs:104-117`) |
| T16 | `public_emit_assert_kutha_dict_vocabulary_is_refused` | structural | **NEW** — existing oracles name only `kutha:allows` (`tests/m013_write_path.rs:46,114,133`); the new prefix has no oracle |

**Commands and expected exit codes:** `cargo test -p kutha-runtime --test c2_dictionaries` → 0 (every listed `fn`);
`cargo test --workspace` → 0; `uv run kutha-gov ci --jobs 1` → 0 with `run_high=0`;
`KUTHA_BENCH_N=10000 cargo bench -p kutha-runtime --offline --bench baseline` → 0, before and after.

**Verification mode:** independent verifier, fresh context. Each **falsifier** row needs the named mutation applied by
hand (drop the VT comparison for T2; use `self.facts` order instead of the sorted vector for T5) with pre/post results
recorded. `uv run kutha-gov selftest` is **not** the mechanism — C2 adds no governor row (§4.10), so mutation is manual.

### Phase 276 — Pack-carried vocabulary facet (spike S4, narrowed to the vocabulary kind)

**Goal:** a pack names vocabulary entries it owns; install validates the references (blob present, raw SHA-256 match,
schema/kind agreement, no collision with an active pack), activation makes the pack the owner without making the
entry live, and retire is graph-neutral.

**Files owned:** `pack_manifest.rs` (`#[serde(default)] dictionaries: Vec<DictionaryRef>` beside `operators`,
`:22-24`; install validation in the `validate_install_indexed` shape, `:189-216`), `admission.rs` (ownership handed
over from Phase 275 at the wave boundary — never concurrent), `crates/kutha-runtime/tests/c2_pack_facets.rs` *(new)*.

**Acceptance tests** (in `crates/kutha-runtime/tests/c2_pack_facets.rs`):

| ID | `fn` name | Kind | Source |
|---|---|---|---|
| T11 | `pack_dictionary_facet_installs_without_going_live` | falsifier | spike |
| T17 | `pack_scope_does_not_leak_into_host_dictionary_validate` | falsifier | **NEW** — spike §7.3 risk "a pack facet makes a third-party vocabulary authoritative" has no oracle |

**Commands:** `cargo test -p kutha-runtime --test c2_pack_facets` → 0; `cargo test -p kutha-runtime --test m179_gate_gc`
→ 0 (retire-neutrality must still hold, `tests/m179_gate_gc.rs:94-98`); `cargo test -p kutha-runtime --test ff6_allowlist`
→ 0; `cargo test --workspace` → 0; `uv run kutha-gov ci --jobs 1` → 0 with `run_high=0`; bench pre/post → 0.

**Verification mode:** independent verifier; T11 and T17 are falsifiers (mutations: make activation set
`valid_from`, and make pack scope visible to host `dictionary_validate`).

### Phase 277 — Close evidence and rollback rehearsal

**Goal:** the milestone closes on measured and rehearsed evidence, not on claims. **Files owned:** no `crates/` source;
bench samples, the rollback receipt, `NN-VERIFICATION.md`, the close entry.

1. **Bench receipt.** Interleaved same-host A/B against `2680cd9`, ≥6 samples each, medians from **sorted** samples
   (the M186 post-close audit found unsorted positional medians; `.kutha/STATE.md` § Next action, per-sample record
   `.compound-engineering/artifacts/research/2026-10-31-m186-ab-samples.md`). Record `ingest_plain.n`, `as_of.n`,
   `as_of.2n`, `ingest_keyed.n`; outliers named, not dropped.
2. **Rollback rehearsal.** Persist a store holding vocabulary facts, open it with a binary built at `2680cd9`
   (`git worktree add tmp/c2-rollback/base 2680cd9`): open exit 0, product `as_of` unchanged, fingerprint unchanged.
   Record both commands and exit codes.
3. **Honesty check.** `honeycomb.yaml` ADR-050 row stays `delivery: frozen`, `capability: none` (`:509-516`) and
   `docs/ADR/ADR-050` stays Proposed. Changing either is not part of this milestone.

**Commands:** `cargo test --workspace` → 0; `uv run kutha-gov ci --jobs 1` → 0 with `run_high=0`; both rehearsal
commands → 0. `uv run pytest` is out of scope: C2 changes no file under `scripts/` (§4.10).

**Verification mode:** independent verifier signs `NN-VERIFICATION.md` with `independent: true`, commands with exit
statuses, and `status` from the closed set (`passed | gaps_found | human_needed`).

## 3. Wave plan

| Wave | Worker | Files owned (exclusive) | Depends on |
|---|---|---|---|
| 1 | W1 `c2-substrate` | `event.rs`, `dictionary.rs`, `fold.rs`, `quantum.rs`, `admission.rs`, `pipeline.rs`, `lib.rs` | — (critical path) |
| 1 | W2 `c2-oracles` | `tests/c2_dictionaries.rs` | API contract fixed in the PLAN (`DictionaryKind`, `emit_dictionary_entry`, `dictionary_entry_at`, `dictionary_entries_at`, `dictionary_validate`, `dictionary_snapshot_id`); compiles only after W1 lands, runs post-merge |
| 2 | W3 `c2-pack` | `pack_manifest.rs`, `admission.rs` (handed over by W1 at the wave boundary) | wave 1 merged |
| 2 | W4 `c2-pack-oracles` | `tests/c2_pack_facets.rs` | contract from wave 1 |
| 3 | W5 `c2-evidence` | no repository source; bench samples and rehearsal receipt under `.compound-engineering/artifacts/research/` | waves 1–2 merged |

No second source worker in wave 1: `dictionary.rs` types feed `fold.rs`, `fold.rs` feeds `admission.rs`, which feeds
`pipeline.rs`; a parallel source worker would create a compile-order loop, not speed.

**Parent integration points (parent only, never a child):** `lib.rs` export merge if W1 and W3 both need a line;
`admission.rs` handoff; `.kutha/STATE.md` lease and close; `.planning/` artifacts; bench record; `graph index` after
the merge; `cargo test --workspace` + `uv run kutha-gov ci --jobs 1`; `CHANGELOG.md`; final Russian report.

## 4. Non-goals (spike §5, plus two fences for this milestone)

1. **No LLM call** on the dictionary path (D050-4: "LLM proposes; dictionaries validate"). No prompt, no cache key.
2. **No capabilities (ADR-051).** No unforgeable references, no grant/revoke, no `capability` field in a facet.
3. **No pack execution (ADR-081).** Install/activate/retire are log transitions; no operator load, no WASM.
4. **No Cypher binder (M003b), no dictionary-backed `VocabBinder`.** The sketch's contract excludes binder work, so
   the spike's **C2-S2 / T12** (`binder_relation_gate_reads_fold_not_yaml`) is **out of this milestone** and the F10
   residual at `gpml.rs:66` (`load_allowed_names()`) stays open, named as a residual in the close entry.
5. **No SMO.** No `SchemaModify`, no KGCL payload, no ontology evolution; `Ontology` is not a declared kind here.
6. **No `MetaPrompt` entity** (needs ADR-014 receipt binding).
7. **No policy evaluation.** The pinned body stays opaque, the `"deny"` special case stays (`admission.rs:58-75`).
8. **No routing table, no FSM executor, no derived-FSM read model** (spike S3, deferred).
9. **No multi-tenant scoping** (ADR-080 stays unopened); a single log namespace.
10. **No `.kutha/dictionaries/` change, no new governor row or CI phase**, hence no `uv run pytest` obligation.
11. **No new `Op` variant** (spike option A rejected): no `digest_bytes` arm, no `op_commit_class` arm, no
    `I-op-compat-policy` migration.

## 5. Bench gate

**Yes — required.** Phase 275 adds one fold lease maintained on every fact push (`allows_by_name` pattern,
`fold.rs:286-292`, maintained even under `FoldIndexMode::Lazy`) plus one admission branch: write-path cost. AGENTS.md:
"every product milestone that edits `crates/` records pre/post `ingest_plain.n` and `as_of.n` in the close entry. A
regression over 10% is a stop."

| Metric | Role | Threshold |
|---|---|---|
| `ingest_plain.n` | gate | >10% slower → **stop**, do not close |
| `as_of.n` | gate | >10% slower → **stop** |
| `as_of.2n` | secondary | >10% → stop and explain; not an automatic revert (no mechanism unless the lease reaches the warm path, which it should not) |
| `ingest_keyed.n` | secondary | record; M186 shows it can pass the line without being a gate failure |

Procedure: `KUTHA_BENCH_N=10000`, `KUTHA_FOLD_INDEX_MODE` unset (Eager), same host, separate target dirs, base commit
`2680cd9`, interleaved samples, ≥6 each, medians from sorted samples.

## 6. Rollback

Reverting C2 does **not** delete anything: entries are ordinary `Assert` + `StoreBlob` facts in a reserved namespace
(spike §2.2/§2.5). Concretely:

1. **Facts stay.** A pre-C2 binary folds `kutha:dict:*` facts as plain facts; `bind_reserved` flags **every**
   `kutha:`-prefixed relation (`fold.rs:541-546`), so they stay out of product `as_of` and remain reachable through
   `dictionary_as_of` (`fold_query.rs:245`). Nothing needs migrating.
2. **No wire break.** `Assert` and `StoreBlob` are pre-existing ops, so the jsonl/WAL/segment readers are untouched
   — this is exactly why option B was chosen over a new `Op` (spike §2.4). There is no `I-op-compat-policy` step.
3. **Ids stay stable.** The six names are interned *after* the genesis rule (`quantum.rs:436-437`), so no existing
   `TermId` moves and `fingerprint` (`fold_query.rs:260-288`, which hashes ids) is unchanged for pre-C2 logs. On reopen
   ids come from `snap.dict_strings` (`hydrate.rs:129`, `store.rs:264`) or log `Define`s, so a C2 store reopens with the
   same ids and fingerprint under a pre-C2 binary. Phase 277 rehearses this.
4. **Cost that remains:** the facts keep occupying log and fold memory; there is no vacuum (gap register Part 4 #7),
   so a reverted engine can never reclaim them. That is a recorded residual, not a correctness problem.
5. **The one-way door is semantic.** After a revert the engine silently stops validating any `kutha:dict:*` fact a
   producer writes and ignores an ambiguity C2 rejects. Mitigation: state in the lease that in M187 the only producers
   are tests and fixtures — there is no external producer yet.
6. **Cost in code:** a `git revert` of the merge; no store migration, no log edit, no snapshot rewrite.

## 7. Risks and owner decisions (spike O1–O9 + this sketch's O10–O14)

| # | Question | Recommended default | Risk if the owner chooses otherwise |
|---|---|---|---|
| O1 | Lease / thaw | lease one kind: M187-S01, vocabulary; lease text names `SIX-01` | C2 cannot start; honeycomb stays `frozen` (`honeycomb.yaml:509-516`) and the thaw-map gate at `.kutha/STATE.md:39` stays unmet |
| O2 | First kind | Controlled vocabulary, then Action+Mode, then Relation (no admit-result change), then Policy (shape only), then Ontology | Relation first proves nothing new (the FF6 allowlist already works) |
| O3 | Namespace | new `kutha:dict:<kind>` prefix; the five reserved relations stay facts *about* the plane | merging re-collides with `kutha:action` instance rows (`justification.rs:143-187`) |
| O4 | Delivery key | `None` for entries; idempotency at the constructor | dated revisions become `DeliveryKeyConflict`, because `has_live_keyed_support` is not VT-aware (`fold_query.rs:513-537`) |
| O5 | Ambiguity | fail closed at admission *and* at read | a legacy log with an ambiguity becomes a silent pick, contradicting D013-3 |
| O6 | Pack retire | hide from pack scope, keep facts | a retracting retire breaks the proven graph-neutral retire (`tests/m179_gate_gc.rs:94-98`) |
| O7 | Facet strictness | `deny_unknown_fields` + raw-SHA-only addressing | one blob readable under two identities (`blob_digest_matches`, `event.rs:683-698`) |
| O8 | Snapshot id | compute and expose; do not change receipt shape | ADR-014 binding done twice, in two shapes |
| O9 | Naming | `dictionary` keeps meaning the intern map (`Runtime::dictionary`, `quantum.rs:576`); new API keeps the ADR word | a second `dictionary()` with a different meaning |
| **O10** | Phase split | merge spike S0+S1 into Phase 275 | a read API no external test can seed, and a half-validated writer in the tree |
| **O11** | Missing oracles | adopt T13–T17 | five stated rules ship without an oracle |
| **O12** | Binder / F10 | defer (this milestone forbids binder work); record as residual | T12 lands with no phase that owns it |
| **O13** | Facet size cap | state 64 KiB in the lease and enforce it (T14) | unbounded fold-resident growth (spike §7.3) |
| **O14** | Ids | M187 / GSD v1.80 / phases 275–277 | GSD phase numbering drifts from the roadmap |

## 8. Deferred out of M187 (each needs its own lease or a later milestone)

- **S2 / T12** — `RuntimeBinder::relation_id` reads the fold instead of YAML (`gpml.rs:63-70`); residual F10.
- **S3** — Action + Mode kinds and the derived-FSM read model; its structural oracle would be **T18 (NEW)**
  `derived_fsm_read_does_not_change_fingerprint`.
- **S5** — move FF6 onto the entry shape without changing admit results.
- **Policy kind** (shape only), **Ontology kind** (needs SMO), **`MetaPrompt`** (needs ADR-014), **`TermDictionary`
  rename** (O9).

## 9. What this sketch verified, and what it inherits

Verified by reading at HEAD `2680cd9`: `event.rs:148-161` (five reserved names, comment "Reserved dictionary relation
names (D-01)"); `admission.rs:209-237` (`kutha:` auto-allow `:213-215`, prefix refusal `:233-237`); `fold.rs:286-292`,
`:540-546`, `:697-712`; `fold_query.rs:103-121` (cold `facts` vs warm `vt_by_from` order), `:245` (`dictionary_as_of`),
`:260-288` (fingerprint hashes `TermId`s); `gpml.rs:63-70`; `pack_manifest.rs:14-24`; `pipeline.rs:37-44`, `:789-905`;
`quantum.rs:424-438`; `store.rs:264` + `hydrate.rs:129` (`dict_strings` on open); `tests/m013_write_path.rs:46,114,133`,
`tests/ff6_allowlist.rs:8,42`, `tests/m179_gate_gc.rs:68,94-98,119`; `.planning/ROADMAP.md` (last phase 274);
`init new-milestone` (`latest_completed_milestone` `v1.79`); `honeycomb.yaml:509-516`.

Inherited from the spike without re-reading: every other `file:line` in spike §2–§4, including the `commit.rs:17-57`
classifier claim and the `blob_digest_matches` three-way argument. T1–T17 are **proposed** oracles: none written, none
run, no cargo command executed for this sketch, no performance claim made (§5 is the gate that will make one).
