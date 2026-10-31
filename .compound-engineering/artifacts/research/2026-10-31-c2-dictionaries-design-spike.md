# C2 dictionaries design spike (ADR-050)

- **Status:** design spike, read-only. Not an ADR, not honeycomb Acceptance, not a delivery authorization.
- **Date:** 2026-10-31. Repo `/root/kutha-graph`, branch `work/main-line`, HEAD `2680cd9`.
- **Scope:** how the six ADR-050 dictionary kinds become log entities, with the semantics, admission/query surface, non-goals, and an acceptance matrix a product lease could execute.
- **Lease:** none (`Active Milestone: None`). C2 is *not* leased by this document. Execution needs two owner actions listed in §7.1: thaw `SIX-01` and name a milestone.
- **Budget:** one file. No tracked file was edited, no cargo run, no `graph index`.

Every code claim below carries `file:line` as read at HEAD `2680cd9`. Every test claim names the test file and `fn`. Nothing here was verified by running cargo; §9 lists exactly what a lease must verify first.

---

## 0. Evidence base

| Source | What was taken |
|---|---|
| `docs/ADR/ADR-050-meta-prompt-dictionaries.md` | six kinds, D050-1…D050-6, the 2026-10-01 and 2026-10-03 clarifications, the explicit "three distinct nouns" note |
| `docs/ADR/ADR-011-lean-event-schema-lineage.md` | D011-1…D011-4 and the 2026-10-01 clarification "dictionary entries are ordinary bi-temporal facts in a reserved namespace with content-addressed blobs" |
| `docs/ADR/ADR-013-bitemporal-facts-invalidation.md` | D013-1…D013-4: VT×TT shape, typed invalidation, "silent overwrite is forbidden" |
| `docs/ADR/ADR-021-pack-plugin-lifecycle.md` | D021-1…D021-3: a pack = operators + optional dictionary facets + leases; activate appends events |
| `docs/ADR/ADR-051-capability-security.md` | D051-1…D051-3 and the 2026-10-01 clarification: admission must evaluate the pinned policy body; capabilities are out of C2 |
| `crates/kutha-common/src/event.rs` | all 15 `Op` variants, `KUTHA_RESERVED_RELS`, `digest_bytes`, the hash functions |
| `crates/kutha-runtime/src/{admission,allow,commit,fold,fold_query,pipeline,justification,pack_manifest,pack_lifecycle,quantum,store,wal,segment,gpml}.rs` | the shipped write path, fold, leases, pack lifecycle, wire-read behaviour |
| `.kutha/dictionaries/*.yaml` | harness intake (relations/checks/invariants/bridges/fsm/honeycomb) — **used only as a fence**, never as the product six kinds |
| `docs/architecture/semantic-gap-review.md`, `docs/architecture/gate-gc.md`, `docs/process/pin-compat.md` | F9/F10/F11 findings, the passed Gate GC probe, the pin-scheme compatibility policy |

No `dictionary.rs` exists in `crates/kutha-runtime/src/` (directory listing at HEAD). The `dictionary` word in the codebase currently means either the intern map (`TermDictionary`, `crates/kutha-common/src/intern.rs:6`) or the `kutha:` reserved fact relations (`event.rs:149-161`). That naming collision is itself part of the problem this spike has to settle (§3.6).

---

## 1. Requirement restatement

### 1.1 The six kinds and what each is for

ADR-050 D050-1 names six kinds. Their roles, with the honest current P0 state:

| # | Kind | ADR-050 role | What exists at HEAD | What does **not** exist |
|---|---|---|---|---|
| 1 | Controlled vocabulary | legal tokens / labels @T | nothing (an assertion subject is any interned term) | dated token meanings, token classes, closedness |
| 2 | Action | what an agent may attempt | `kutha:action` **instance** records (`justification.rs:143-187`) pinning one attempted action to an admission | a dictionary of *allowed action kinds* |
| 3 | Relation | legal edge types / behaviors | the FF6 allowlist: `kutha:allows` facts + `GraphFold::relation_allowed_at` (`fold_query.rs:559-585`) | edge schema, subject/object kind constraints, relation behaviors |
| 4 | State / mode | modes from which the FSM is *derived* | nothing | states, transitions, the derived FSM read model |
| 5 | Policy | tool and conflict policy | `kutha:policy` pins, opaque body blob, a literal `"deny"` special case (`admission.rs:58-75`) | evaluation of the pinned body (F9, ADR-051 open question) |
| 6 | Domain ontology / schema | TBox/schema, evolves via SMO/KGCL | nothing | any schema object, any SMO |

Three of the six kinds are therefore represented today only as *facts about* the engine, not as dictionaries: `kutha:allows`, `kutha:policy`, `kutha:action`. That is exactly the distinction ADR-050 records at 2026-10-03: "(1) five reserved facts `kutha:allows` | `kutha:rule` | `kutha:policy` | `kutha:admission` | `kutha:action` … (2) D050-1 six dictionary kinds … (3) harness dictionaries under `.kutha/dictionaries`. They are not the same set."

### 1.2 P0 vs later, inside ADR-050's own vocabulary

ADR-050 marks its own P0 explicitly: "P0 FF6 stub: `load_allowed_names` reads `crates/kutha-runtime/dictionaries/relations.yaml` only when bootstrapping a **new** log". So **inside ADR-050, P0 is the relation allowlist only**; the six-kind model is target contract, and `honeycomb.yaml:495-504` records `delivery: frozen`, `freeze_as: "ADR-050 six dictionaries"`, `capability: none`.

Mapping the decisions to C2:

| ADR-050 item | C2 status | Note |
|---|---|---|
| D050-1 six kinds | **in scope**, one kind per slice | this spike |
| D050-2 fail-closed propose→validate→log | **partly in scope**: the engine supplies `validate`; the *cycle* (load → propose → execute) is caller-side and out of scope | do not build an agent loop |
| D050-3 intern map ≠ control dictionaries | **fence, untouched** | entries reuse the existing intern; no second intern design |
| D050-4 compiler-not-executor, routing is a table | **out of scope** | needs a routing table and an execution receipt; not a dictionary representation question |
| D050-5 ontology workbench is a pack | **out of scope** | needs SMO; `SchemaModify` does not exist on the log |
| D050-6 security named, not specified | **out of scope**; facets must not carry capability fields | keeps ADR-051 unopened |
| Clarification F5 (admission status is a bitemporal fact) | shipped for admission/action | `record_justification` / `record_action` |
| Clarification F9/F10 (executable policy body; genesis instead of a file) | **partly**: genesis allowlist shipped (`quantum.rs:424-438`); policy *evaluation* still open | do not claim F9 closed |
| Clarification F4 (rule registry whose version equals the definition hash) | shipped for rules (`event.rs:662-680`) | `kutha:rule` + `rule_definition_hash` |
| `MetaPrompt` entity | **out of scope** | needs receipts binding versions (ADR-014) |

### 1.3 The three nouns must stay separate

```text
harness intake (.kutha/dictionaries/*.yaml)   schema: kutha-harness-relations/v1  (.kutha/dictionaries/relations.yaml:1)
product FF6 seed (crates/.../relations.yaml)  schema: kutha-relations/v1          (crates/kutha-runtime/dictionaries/relations.yaml:1)
product six kinds (ADR-050 D050-1)            C2 target, nothing shipped
intern map (ADR-011 D011-2)                   TermDictionary, a droppable lease
```

C2 must not read `.kutha/dictionaries/`, must not change either YAML schema, and must not treat `kutha:allows` (the FF6 allowlist) as the Relation *kind*. The harness plane's H2 tenant rows `runStatus` / `observed` are delivery facts on the allowlist (`ADR-050` 2026-08-18 note), not a seventh kind.

### 1.4 The delta C2 has to add

`docs/architecture/gate-gc.md` records Gate GC **passed** on 2026-10-31 (M179-S01, probe `crates/kutha-runtime/tests/m179_gate_gc.rs`, test `toy_pack_installs_queries_retires_and_replays_without_core_edits`). The same document says what GC does **not** prove: "No dictionary facets, no schema SMO, no lease rebuild…". So C2 = **the dictionary facet a pack can carry**, on top of an already-green pack lifecycle. That is a narrow, well-bounded delta, which is why the shape below reuses C1's proven mechanisms instead of inventing new ones.

---

## 2. Representation options

Three shapes are on the table. All three are log-only; none of them creates a sidecar. They differ in *what kind of log record* carries a dictionary entry.

### 2.1 Option A — new `Op` variants (`DictionaryDefine` / `DictionaryRetire`)

Precedent: `PackInstall` / `PackActivate` / `PackRetire` are three `Op` variants added for C1 (`event.rs:251-267`), and `EnrichmentProposal` for C5 (`event.rs:271-279`).

What the change costs, measured at HEAD:

- `Op` has 15 variants (`event.rs:165-295`) and **four verified exhaustive matches with no wildcard**:
  - `Event::new` `object_ids` (`event.rs:308-333`),
  - `Event::digest_bytes` (`event.rs:416-633`),
  - `commit::op_commit_class` (`commit.rs:17-35`, commented "Exhaustive match (no `_`)" at `commit.rs:13`),
  - `GraphFold::apply` (`fold.rs:893-1077`).
- plus the wire-migration surface: `crates/kutha-common/src/op_migrate.rs` (`match op` at 611/647/709), `store::migrate_op_wire` (`store.rs:290-324`), and the recorded invariant `I-op-compat-policy` (`.kutha/dictionaries/invariants.yaml:498-502`) which says "Removed dictionary meta-ops fail deserialize on open (jsonl, WAL, and sealed segment frames)".
- and the *replay* surface: a new variant would sit outside `facts` (like `PackLifecycleTracker`, `pack_lifecycle.rs:88-110`), so `replay_check` (`provenance.rs:47-90`) would need a new equality leg, repeating the C1 pattern, because `fingerprint` only walks `facts` (`fold_query.rs:260-288`).

**This is the shape the project already decided against.** `docs/architecture/semantic-gap-review.md:120` (F11) states the mechanism argument verbatim: "Every new dictionary kind adds an `Op` variant, a side table, and a copy of `is_live_at` (five today in `fold.rs`)". And the amended target contract (`semantic-gap-review.md:145`, ADR-011 row) is: "Dictionary entries are bi-temporal facts in a reserved namespace with content-addressed bodies". The harness records the same rule as an invariant: `I-F11-one-write-path` — "Every record kind enters through one propose-admit-commit-receipt path; dictionary entries are facts, **not parallel Op kinds with parallel side tables**" (`.kutha/dictionaries/invariants.yaml:230-234`).

### 2.2 Option B — reuse `Op::Assert` + typed facet in a reserved namespace, body in `StoreBlob`

Shape:

```text
Op::StoreBlob { digest = sha256_hex(facet_bytes), bytes = facet_bytes }      // one per facet revision
Op::Assert {
    subject      = intern(entry name)          // the token; shares the product id space (D050-3)
    relation     = intern("kutha:dict:<kind>") // the kind, reserved namespace
    object       = intern(facet_digest_hex)    // the body address
    valid_from / valid_to                      // the entry's VT window
    claim        = None
    delivery_key = None                        // see §3.3: VT is the revision axis, not a retry key
    polarity     = None
}
```

Existing machinery this reuses without change:

- admission auto-allows any `kutha:`-prefixed relation (`admission.rs:213-215`), so no allowlist row is needed for a kind;
- user writes to those relations are already refused: `relation_name_reserved` treats any name starting with `kutha:` as reserved (`admission.rs:233-237`), applied in `ReceiptMode::User` (`admission.rs:180-197`), oracles `public_emit_assert_kutha_allows_is_refused` and three siblings (`crates/kutha-runtime/tests/m013_write_path.rs:46,72,114,133`);
- the fold already classifies `kutha:*` relations as dictionary and hides them from product queries (`fold.rs:540-546`, `fold.rs:569-574`, `LiveFactFilter` at `fold.rs:228-244`; used by `as_of`/`known_at` at `fold_query.rs:41,59,489,500`);
- the commit classifier already treats `kutha:*` asserts as `SelfCommit` (`commit.rs:37-41`), so a dictionary write needs no trailing `QuantumOutcome` — the same shape `ReceiptMode::Dictionary` gives the existing constructors (`pipeline.rs:37-44`);
- dictionary facts land in `facts`, therefore inside `fingerprint` (`fold_query.rs:262-286`) and therefore inside `replay_check` with zero new machinery;
- content-addressed bodies are verified already: `blob_digest_matches` (`event.rs:683-698`) at `admission.rs:104-111` and again on replay (`provenance.rs:80-87`);
- `StoreBlob`-then-`Assert` ordering is already the shipped pattern for rules and policies (`pipeline.rs:844-867`, `pipeline.rs:882-904`);
- generation is already a log fact, not a file read: genesis `kutha:allows` asserts (`quantum.rs:430-435`), oracle `genesis_allow_visible_on_as_of` (`crates/kutha-runtime/tests/m013_dictionary_facts.rs:103`).

### 2.3 Option C — `StoreBlob` manifest, pack-shaped

Shape: one JSON manifest per dictionary revision (list of entries), pinned by digest; the *only* per-entry log trace is inside the blob. Precedent: `PackManifest` (`pack_manifest.rs:12-24`), digest = `sha256_hex(canonical_bytes)` (`pack_manifest.rs:93-101`), strict raw-SHA addressing on load (`pack_manifest.rs:164-183`), install-time validation against the folded blob map (`pack_manifest.rs:189-216`, called from `admission.rs:160-177`).

Its structural weakness for dictionaries is that a blob is not a fact:

- no per-entry VT window: one blob is one instant, so "token X meant A until 2021 and B after" needs either two blobs plus a fact to say when each is live, or a VT list inside JSON that the fold cannot answer without parsing;
- `dictionary_as_of` / `is_live_at` cannot see it: `fold.blobs` is explicit `#[serde(skip)]` and documented "not fingerprint input" (`fold.rs:330-332`);
- every per-entry question becomes a blob parse, so the cheap fold-cut lease pattern (`relation_allowed_at`, `fold_query.rs:559-585`) has no analog;
- it is the right shape for *manifest-level* declarations (operators, pack identity), which is why C1b used it and why §3.4 keeps it for the pack-to-entry *reference list* only.

### 2.4 Comparison

| Criterion | A: new `Op` variants | B: Assert + typed facet | C: StoreBlob manifest |
|---|---|---|---|
| **Replay determinism** | new state outside `facts`; `replay_check` needs a new equality leg (`provenance.rs:47-90`), as C1 did | entries *are* facts → inside `fingerprint` (`fold_query.rs:262-286`) and `replay_check` already; zero new machinery | blob is skipped by fingerprint (`fold.rs:330-332`); determinism rests on `blob_digest_matches` (`provenance.rs:80-87`) plus a canonical-JSON contract that nothing enforces today |
| **Digest stability** | each new variant adds `digest_bytes` arms (`event.rs:416-633`); old events keep their digests, but every new arm is a new permanent wire contract | no wire change at all: `Assert`/`StoreBlob`/`Define` digests unchanged; facet identity = raw SHA-256 of the bytes as stored, and the engine never re-serializes them | same as B for the blob, but one canonicalization drift invalidates a whole revision, and `PackManifest` does **not** set `deny_unknown_fields` (`pack_manifest.rs:17-24`), so a typo'd field is silently ignored |
| **Admission ordering** | self-contained: no ordering constraint | `StoreBlob` must be folded before the entry `Assert` that pins it — already enforced by construction in `emit_rule_ast`/`emit_policy` (`pipeline.rs:844-852`, `pipeline.rs:882-889`) | same ordering constraint, plus `validate_install_indexed` needs the blob map populated (`admission.rs:143,160-177`) |
| **Forward compat (old binary reads new log)** | **hard break, three ways**: legacy jsonl `serde_json::from_str(...)?` errors (`store.rs:1008-1010`); WAL frame that is valid JSON but not an `Event` is `InvalidData` (`wal.rs:142-171`); an **unsealed** segment frame that does not decode is silently dropped as a torn tail (`segment.rs:547-559`) — one silent-loss mode | old binary folds the events as ordinary facts and `StoreBlob`s; it does not interpret the facet. Because `kutha:` relations are flagged as dictionary (`fold.rs:541-546`), an old binary even excludes them from product queries correctly | old binary folds the blob and (for C-shaped installs) tolerates unknown manifest fields; it never sees per-entry truth |
| **"No second SoT" (D1)** | satisfied — but contradicts `I-F11-one-write-path` (`.kutha/dictionaries/invariants.yaml:230-234`) and F11 | satisfied and aligned with the amended ADR-011 contract (`semantic-gap-review.md:145`) | satisfied *if* the manifest stays a log blob; the standing risk is a producer keeping a `.json` as the working truth (ADR-021 D021-1 forbids that) |

### 2.5 Recommendation

**Option B for entries, Option C only for the pack's reference list.**

Reasons, in order of weight:

1. It is the option the project already recorded as the target contract (ADR-011 2026-10-01 clarification; `semantic-gap-review.md:145`) and the shape the harness invariant `I-F11-one-write-path` requires. Choosing A would require rewriting a recorded invariant; choosing C would leave per-entry AS OF unanswered.
2. Determinism and replay come free: dictionary entries are `Fact`s, so `fingerprint` and `replay_check` already cover them.
3. Forward compatibility is not a trade-off but a fact of the wire code: A breaks jsonl/WAL and *silently drops* an unsealed segment frame (`segment.rs:553-558`); B is readable by an old binary as plain data.
4. The expensive part of A is not the enum edit; it is `I-op-compat-policy`'s migration chain (`.kutha/dictionaries/invariants.yaml:498-502`, `store.rs:290-324`, plus the `kutha-migrate` CLI documented in `docs/process/pin-compat.md`). B pays none of it.
5. C keeps its proper job: the pack manifest names which dictionary entries a pack owns, exactly as it names operators today (`pack_manifest.rs:17-24`).

The one thing B does **not** give for free is one fold lease instead of six side tables — that is a deliberate C2 deliverable, specified in §4.6.

---

## 3. Semantics

### 3.1 An entry is a bitemporal fact

An entry is one `Fact` row (`fold.rs:115-137`). Its four clocks are the shipped ones, with the shipped liveness rule:

```text
is_live_at(tt, vt)  (fold.rs:163-179)
  tt < ingested_at            -> dead        (not yet known)
  invalidated_at <= tt        -> dead        (retracted/corrected at or before the cut)
  vt < valid_from             -> dead        (not yet in force)
  valid_to <= vt              -> dead        (half-open VT window)
```

So: **VT** is when the entry's meaning holds in the world (a norm's validity window, a mode's lifetime); **TT** is when the engine learned it. VT is half-open, TT is inclusive-closed — that asymmetry is already fixed by `is_live_at` and must not be restated differently in dictionary code.

An entry's identity is `(kind, name)`; its *content* is the facet digest. The `subject` TermId is the token and is the same id space as product terms (this is the D050-3 bridge: CSR stores interned ids, so a token the dictionary knows is the id CSR already uses). Interning is permanent, so a retired and re-added token gets the same TermId — identity is by name, not by id.

### 3.2 Resolving a token AS OF

```text
resolve(kind, name, tt, vt):
  rows = live facts with (relation == kind_rel, subject == intern(name)) at (tt, vt)
  rows = []                        -> UnknownToken        (fail closed)
  distinct facet digests in rows:
       1                          -> Ok(entry)
      >1                          -> AmbiguousToken      (fail closed; no silent pick)
```

Two things this deliberately refuses:

- **"Latest wins" over an ambiguous cut.** ADR-013 D013-3: "Equal endpoints or overlapping valid-time alone do not establish supersession… Silent overwrite is forbidden." If the log holds two live rows with different facets for one key at one cut, the read fails and names both event ids; it does not choose.
- **Ignoring TT.** A token asserted with `valid_from = 0` but ingested at TT 42 is unknown at TT 41. That is the shipped `as_known_on` honesty (ADR-013 evidence row `true_on_d_differs_from_known_on_d`), reused.

Ordering: `dictionary_entries_at` must sort at the API boundary. This is not cosmetic — the two fold paths return **different orders**: cold path iterates `facts` in seq order (`fold_query.rs:103-111`), warm path iterates `vt_by_from` (`fold_query.rs:113-121`), i.e. by `valid_from` then insertion. Without a sort, the same log yields two orders depending on `FoldIndexMode`. Sort key: `(name, valid_from, event_id)`.

### 3.3 Revisions of a dictionary

A revision is a new entry row with a new VT window. The rules:

| Situation | Result |
|---|---|
| same `(kind, name)`, same facet digest, same VT window, row already live | **no-op**, return the existing `EventId` (idempotent re-pin) |
| same `(kind, name)`, different facet, **overlapping** VT | **rejected** `DictionaryConflict` — the caller must retract the old row or close its VT window first |
| same `(kind, name)`, different facet, **disjoint** VT | append — this is the legitimate dated revision |
| retract | `Retract` of the entry's `EventId`; losers stay on the log (D013-2), `dictionary_as_of` stops returning it after the retract's TT |

Why **not** `delivery_key = entry name`: the retry gate classifies liveness as "not invalidated and same key" and is *not* VT-aware — `has_live_keyed_support` (`fold_query.rs:513-537`) checks `invalidated_at.is_none()` only, and `delivery_key_retry` consults it (`admission.rs:340-347`). A row whose VT window has closed is still "live" by that test, so a legitimate dated revision would be rejected as `DeliveryKeyConflict`. ADR-050's own 2026-10-10 clarification exists because this namespace is subtle (`emit_policy` repins). Keeping `delivery_key = None` for dictionary entries leaves the registry semantics of `kutha:allows` / `kutha:rule` / `kutha:policy` untouched, and puts idempotency at the constructor (§4.4), the pattern `Runtime::derive` already uses when it pre-checks `rule_hash_live_at` before `emit_rule` (`quantum.rs:710-714`).

A dictionary *revision* (as opposed to an entry revision) is then a derived value, not a stored object: the set of live entries at `(tt, vt)`, summarised by `dictionary_snapshot_id` (§4.4). That keeps one SoT.

### 3.4 What "dictionary facet of a pack" means (ADR-021 interplay)

ADR-021 D021-1: "A pack = operators + optional dictionary facets + lease registrations." Concretely, in the recommended shape:

- **Declaring:** `PackManifest` gains one `#[serde(default)]` field — `dictionaries: Vec<DictionaryRef>` where `DictionaryRef { kind, name, facet_digest }` — the same additive pattern `operators` already uses (`pack_manifest.rs:22-24`). The manifest stays the *reference list*; it is not the truth of the entries. Per ADR-021 D021-2 this is a dictionary change, not a view build and not an SMO.
- **Installing:** admission validates each ref: blob present, raw SHA-256 match (`pack_manifest.rs:164-183` shape), facet schema tag and `kind` field consistent, and no *active* pack already owning the same `(kind, name)` with a different facet. `validate_install_indexed` (`pack_manifest.rs:189-216`) is the template; the collision index is a second `OperatorIndex`-shaped map (`pack_manifest.rs:223-227`).
- **Activating:** entries become live. Recommended default: the entry's VT window comes from the facet/entry write, not from activation; activation only makes the *pack* the owner (so pack-scoped resolution includes them). If an entry must be world-valid only from activation, the pack supplies `valid_from = activation vt` explicitly — no implicit "now" (the codebase refuses implicit clocks; `csr_lease_at` is documented "callers must name valid-time (no silent now)", `quantum.rs:589-591`).
- **Retiring:** **graph-neutral**, by evidence. Gate GC proves `retire` does not move the fingerprint (`crates/kutha-runtime/tests/m179_gate_gc.rs:94-98`, "retire is graph-neutral") and that replay-after-retire equals reopen (`pack_retire_then_replay_is_parity_equal_to_reopen`, same file line 119). If retire retracted dictionary facts, that proven property would break. Recommended default: retire removes pack ownership (so pack-scoped resolution stops returning the entries) but leaves the facts; deleting entries is a separate explicit call (§4.4). This is owner question O6.

### 3.5 Fail-closed on an unknown token — and where the boundary is

The fail-closed point is the **dictionary-consuming surface**, not generic `emit(Assert)`. Evidence that widening it would break shipped behaviour: `retract`/`derive` already accept any allowlisted relation and any object term (`quantum.rs:644-740`), and FF5/FF6 oracles write `relatedTo` facts whose objects are arbitrary terms (`crates/kutha-runtime/tests/m013_dictionary_facts.rs:24-35`). Requiring vocabulary membership for every Assert would turn those into dictionary errors and would amount to the hard-FSM shape D3 rejects.

So:

- generic writes: unchanged (allowlist only);
- **binder / validate / pack install**: fail closed. `Runtime::dictionary_validate(kind, name, tt, vt)` returns `Err` for `UnknownToken`, `AmbiguousToken`, `KindMismatch`, `FacetMissing`, `FacetDigestMismatch`, and additionally for a *closed* vocabulary (a facet field) whose name is absent.
- The known F10 residual should be fixed in the same slice: `RuntimeBinder::relation_id` reads the YAML file at bind time (`gpml.rs:66` → `allow::load_allowed_names`, `allow.rs:54-60`). ADR-050's 2026-10-01 clarification says "the log owns dictionary truth from its first event… not a file read at admit time", and the harness records `I-F10-genesis` (`.kutha/dictionaries/invariants.yaml:224-228`). The binder must consult the fold (`relation_allowed_at`, `fold_query.rs:559-585`) instead. This is a two-line change with a named oracle (§6 T12), and it is the only place C2 touches an existing consumer.

### 3.6 One more naming decision

The word `dictionary` currently means the intern map. C2 introduces a fifth meaning unless names are fixed now. Recommendation: reserve `dictionary` for ADR-011's intern map (keep `TermDictionary`, `Runtime::dictionary()` at `quantum.rs:576`), and call the new surface **catalogue** in prose but keep the ADR-050 word in the API (`DictionaryKind`, `emit_dictionary_entry`) — renaming the intern map is a wider refactor than C2 should carry. Alternatively rename the map later under its own slice. Either way, C2 must not silently introduce a second `dictionary()` accessor with a different meaning.

---

## 4. Admission + query API sketch

### 4.1 Validation rules for a dictionary write

Applied by `emit_dictionary_entry`, and (for external producers) mirrored in `admit` for `kutha:dict:*` relations:

1. **Kind is enumerated.** The relation must be one of the six reserved kind relations (§4.2). An arbitrary `kutha:*` relation is not a kind.
2. **Name**: non-empty; ≤ 255 bytes; must not start with `kutha:`; must not equal `true` / `false` (the admission/action truth sentinels interned at `quantum.rs:427-428` and used at `justification.rs:85`) nor any reserved relation name; no leading/trailing whitespace (exact spelling is identity — the rule `pack_manifest::valid_operator_name` already states, `pack_manifest.rs:155-162`).
3. **Facet digest is a raw SHA-256 address only.** Not the three-way `blob_digest_matches` (`event.rs:683-698`) that `StoreBlob` admission accepts today: that function also accepts a rule-body hash and a policy UTF-8 hash, which would let one blob be read under two identities. `pack_manifest::load_manifest` already makes exactly this argument (`pack_manifest.rs:174-181`); facets should copy it.
4. **Facet bytes must be present and match.** Missing blob → `FacetMissing`; `sha256_hex(bytes) != digest` → `FacetDigestMismatch`.
5. **Facet schema tag and kind field must agree.** `{"schema":"kutha-dict-facet/v1","kind":"vocabulary",…}`; unknown schema → reject (fail-closed, `pack_manifest.rs:112-117` pattern). The struct should set `deny_unknown_fields` — `PackManifest` does not (`pack_manifest.rs:17-24`) and that is a robustness gap worth not copying.
6. **Per-kind required fields** (§4.3).
7. **VT**: `valid_from` is mandatory (no implicit now); `valid_to` must be `> valid_from` when present.
8. **Conflict**: no live row with the same `(kind, name)` at overlapping VT and a different facet (§3.3). Identical facet + identical window → no-op returning the original `EventId`.
9. **Ordering**: the `StoreBlob` is committed before the entry `Assert` (same call, blob first — `pipeline.rs:844-852` pattern).
10. **Receipt mode**: `ReceiptMode::Dictionary` (admit + commit, no `QuantumOutcome`), consistent with `emit_allow` / `emit_rule_ast` / `emit_policy` (`pipeline.rs:789-905`).

### 4.2 Reserved names

| Name | Status | Should C2 touch it? |
|---|---|---|
| `kutha:allows` | shipped FF6 allowlist (`event.rs:149`) | no — stays the allowlist, not the Relation kind |
| `kutha:rule` | shipped rule registry (`event.rs:150`) | no |
| `kutha:policy` | shipped policy pin (`event.rs:151`) | no — C2 may add *entries* that reference a policy blob, but must not change the pin contract |
| `kutha:admission` | shipped admission status (`event.rs:152`) | no |
| `kutha:action` | shipped action **instance** records (`event.rs:153`, `justification.rs:163`) | no — reusing it for the Action *kind* would put instance records and kind entries on one relation |
| `kutha:dict:vocabulary` | new | yes |
| `kutha:dict:action` | new | yes |
| `kutha:dict:relation` | new | yes |
| `kutha:dict:mode` | new | yes |
| `kutha:dict:policy` | new | yes |
| `kutha:dict:ontology` | new | yes |

Why a separate `kutha:dict:` prefix rather than six more `KUTHA_RESERVED_RELS` entries: the five existing names are *facts about* the agent control plane (registry slot, pin, status, instance record). The six kinds are *the dictionaries themselves*. Merging them re-creates the noun collapse ADR-050's 2026-10-03 note forbids, and `kutha:action` in particular already carries instance rows whose `delivery_key` is a justification id (`justification.rs:179`).

Intern order constraint: append the six kind names at the **end** of `Runtime::new`'s interning (after the genesis rule at `quantum.rs:436-437`). Inserting them earlier shifts the ids of `true`/`false` and of every YAML-seeded relation, which changes the numeric ids inside snapshots and any test that names an id. Appending leaves every existing id untouched.

### 4.3 How behaviors and rules reference tokens

- **Behaviors** (ADR-010/ADR-013) carry interned `subject`/`relation`/`object` (`event.rs:202-214`). A token reference is simply that `TermId`; the fold never consults a dictionary when applying a Behavior. Dictionaries constrain through *validate* (D050-2), and the entry's identity is the same interned id, so nothing new is needed on the Behavior path.
- **Rules** (`kutha:rule` + `rule_definition_hash`, `event.rs:662-680`) reference idents by name; a Relation/Action kind can declare which idents are legal, checked at `emit_rule_ast` time, not at fold time.
- **State/mode → FSM is derived, never stored.** The derived FSM is a read model over live `kutha:dict:mode` + `kutha:dict:action` entries, exactly the relationship `PackLifecycleTracker`/`derive_pack_lifecycle` has to pack events (`pack_lifecycle.rs:36-110`). It must not become fold state and must not gate writes (D050-2: "FSM/statecharts are **derived** … not hard-coded per vertical as the sole control model").
- **Capability fields must not appear** in Action or Policy facets: ADR-051 D051-1 says capabilities are unforgeable references, not ACL rows; a string field named `capability` inside a dictionary facet would be precisely the second ACL table D051-1 forbids. If a facet needs to name a privilege, defer the field to C3.

### 4.4 Runtime surface (harness-facing)

Sketch, matching the existing naming and error style (`RuntimeError` variants are `Clone + PartialEq`, e.g. `admission.rs:106-109`):

```rust
pub enum DictionaryKind { Vocabulary, Action, Relation, Mode, Policy, Ontology }

pub struct DictionaryEntry {
    pub kind: DictionaryKind,
    pub name: String,
    pub facet_digest: String,
    pub valid_from: u64,
    pub valid_to: Option<u64>,
    pub event_id: EventId,     // the support slot; Retract/Create target it (REF-01)
}

impl Runtime {
    // write
    pub fn emit_dictionary_entry(&mut self, kind: DictionaryKind, name: &str, facet: &[u8],
                                 valid_from: u64, valid_to: Option<u64>)
        -> Result<QuantumOutcome, RuntimeError>;      // blob first, then Assert; idempotent on identical (facet, window)
    pub fn retract_dictionary_entry(&mut self, kind: DictionaryKind, name: &str)
        -> Result<QuantumOutcome, RuntimeError>;      // retracts every live row of that key

    // read (bitemporal, no implicit now)
    pub fn dictionary_entry_at(&self, kind: DictionaryKind, name: &str,
                               tt: impl Into<TransactionTime>, vt: impl Into<ValidTime>)
        -> Result<Option<DictionaryEntry>, DictionaryError>;   // Err(Ambiguous) on >1 distinct facet
    pub fn dictionary_entries_at(&self, kind: DictionaryKind,
                                 tt: impl Into<TransactionTime>, vt: impl Into<ValidTime>)
        -> Vec<DictionaryEntry>;                      // sorted by (name, valid_from, event_id)
    pub fn dictionary_facet(&self, entry: &DictionaryEntry) -> Option<&[u8]>;

    // validate (the D050-2 hook; used by the binder and by pack install)
    pub fn dictionary_validate(&self, kind: DictionaryKind, name: &str,
                               tt: impl Into<TransactionTime>, vt: impl Into<ValidTime>)
        -> Result<(), DictionaryError>;

    // binding for ADR-014 receipts (ADR-050 open question 2); computed, not stored
    pub fn dictionary_snapshot_id(&self, tt: impl Into<TransactionTime>, vt: impl Into<ValidTime>)
        -> String;
}
```

`DictionaryError`: `UnknownToken`, `AmbiguousToken { event_ids }`, `KindMismatch { expected, found }`, `FacetMissing { digest }`, `FacetDigestMismatch { digest }`, `FacetSchema { found }`, `Conflict { live: EventId }`, `ReservedName`.

`dictionary_snapshot_id`: SHA-256 over domain `kutha-dict-snapshot/v1` then the sorted rows `kind|name|facet_digest|valid_from|valid_to|invalidated_at`. It is a *computed* label (like `fold.fingerprint()`), never stored and never a second identity.

### 4.5 What must be rejected

| Rejected | Mechanism that already exists / must be added |
|---|---|
| `rt.emit(Assert { relation: <kutha:dict:*> })` from user mode | exists: `MetaOpRejected`, `admission.rs:180-197,233-237`; oracle `public_emit_assert_kutha_allows_is_refused` (`tests/m013_write_path.rs:46`) |
| Retract/Correct of a `kutha:dict:*` fact from user mode | exists: `admission.rs:186-196`; oracles `public_emit_retract_kutha_allow_is_refused`, `public_emit_correct_kutha_allow_is_refused` |
| A free-form relation used as a kind | new: kind must be one of six constants |
| Entry name starting with `kutha:` / equal to `true`/`false` | new: `ReservedName` |
| Facet blob missing, or digest not raw SHA-256, or bytes mismatched | new: reuse `sha256_hex` (`event.rs:656-660`) + strict compare, not `blob_digest_matches` |
| Facet schema tag unknown, or `kind` field disagrees with the relation | new: `FacetSchema` / `KindMismatch` |
| Overlapping VT, same key, different facet | new: `Conflict` (D013-3: silent overwrite forbidden) |
| Pack facet whose refs name a blob that is not on the log | new: `validate_install_indexed` extension (`pack_manifest.rs:189-216`) |
| Pack facet colliding with an active pack on `(kind, name)` with a different facet | new: `OperatorIndex`-shaped collision map (`pack_manifest.rs:223-227`) |
| A product write that uses an unallowlisted relation | exists: `UnknownRelation`, `admission.rs:224-230`; oracle `ff6_unknown_relation_does_not_append` (`tests/ff6_allowlist.rs:8`) |

### 4.6 Code changes inside existing files (what a lease would touch)

| File | Change | Why |
|---|---|---|
| `crates/kutha-common/src/event.rs` | add six `pub const` kind names next to `KUTHA_RESERVED_RELS` (`:149-161`); **no new `Op` variant, no `digest_bytes` arm** | new reserved namespace |
| new `crates/kutha-runtime/src/dictionary.rs` | `DictionaryKind`, facet structs + schema constants, `DictionaryError`, validation | the kind logic has no home today |
| `crates/kutha-runtime/src/fold.rs` | one lease `dict_by_key: IngestMap<(TermId /*kind*/, TermId /*name*/), FactIdxList>`, indexed in the slot hook like `index_allows_slot` (`:697-712`), rebuilt in `bind_reserved` (`:540-567`) | one lease for all six kinds; maintained even under `FoldIndexMode::Lazy`, as `allows_by_name` is (`:286-292`) |
| `crates/kutha-runtime/src/admission.rs` | a `kutha:dict:` branch before the generic allowlist return (`:209-231`): facet presence/digest/schema/conflict | the six validation rules |
| `crates/kutha-runtime/src/quantum.rs` | intern the six kind names at the end of `Runtime::new` (`:436`) | ids must not shift |
| `crates/kutha-runtime/src/pipeline.rs` | `emit_dictionary_entry` / `retract_dictionary_entry` constructors | the write surface |
| `crates/kutha-runtime/src/gpml.rs` | `RuntimeBinder::relation_id` consults the fold instead of YAML (`:66`) | closes the F10 residual (`I-F10-genesis`) |
| `crates/kutha-runtime/src/pack_manifest.rs` | `#[serde(default)] dictionaries: Vec<DictionaryRef>` + validation | ADR-021 D021-1 facets |
| `crates/kutha-runtime/src/lib.rs` | re-exports | public surface |

Note what is **absent** from that table: `commit.rs`. No new `Op` means `op_commit_class` (`commit.rs:17-35`) and `class_with_dict` (`commit.rs:37-57`) are already correct for dictionary asserts, because they classify by the `kutha:` relation prefix, not by variant.

---

## 5. Explicit non-goals for C2

1. **No LLM call, ever, on the dictionary path.** ADR-050 D050-4 and its non-goal "LLM proposes; dictionaries validate". No model invocation, no prompt, no cache key in C2.
2. **No capability system (ADR-051).** No unforgeable references, no grant/revoke events, no `capability` field in any facet. `validate(+security)` stays a named call site, not an implementation (D050-6).
3. **No pack execution (ADR-081).** C2 does not run an operator, does not load code, does not register a WASM module. Gate GC already proved install/activate/retire with no operator execution (`docs/architecture/gate-gc.md`).
4. **No Cypher binder (M003b).** The `VocabBinder` trait stays as is (`gpml.rs:37-40`); a dictionary-backed binder is a later *second implementation* gated on C2 (`.kutha/STATE.md:37`; `2026-10-30-gate-gb-decision.md:24`). C2 only fixes the YAML read at `gpml.rs:66`.
5. **No schema SMO.** No `SchemaModify`, no KGCL payload, no ALTER. The ontology kind may be *declared* as an entry shape but its evolution semantics are not in C2 (D050-5).
6. **No `MetaPrompt` entity.** ADR-050 D050-1 names it, but it needs receipt binding (ADR-014) and a version identity contract; C2 ships dictionaries only.
7. **No policy evaluation.** The pinned policy body stays opaque, with the existing `"deny"` special case (`admission.rs:70`). F9 stays open and `I-F9-policy-semantics` (`.kutha/dictionaries/invariants.yaml:218-222`) stays a check on a claim that C2 does not close.
8. **No routing table, no topic router, no FSM executor.** Derived FSM is a read model (§4.3).
9. **No multi-tenant dictionary scoping.** ADR-080 stays unopened; all entries are in the single log namespace.
10. **No `.kutha/dictionaries/` change and no harness-plane governor check.** A new CI phase would need a governor-intake row (AGENTS.md); C2's tests live in `crates/kutha-runtime/tests/` and add no governor row.

---

## 6. Acceptance test matrix

Twelve tests in `crates/kutha-runtime/tests/c2_dictionaries.rs` (plus one pack test beside the Gate GC probe). "Falsifier" = a mutation to the named mechanism must flip it; "structural" = proves shape, not behaviour.

| ID | Test | What it proves | Failure it catches | Kind |
|---|---|---|---|---|
| T1 | `dictionary_write_uses_only_preexisting_ops` | a dictionary write appends only `Define` / `StoreBlob` / `Assert` | a later refactor smuggling in a `DictionaryDefine` variant (the option-A drift) | structural |
| T2 | `dictionary_entry_resolves_at_vt_not_latest` | entry A 2015→2021, entry B 2021→; resolve(2018)=A, resolve(2022)=B | resolving the newest row regardless of VT | **falsifier** |
| T3 | `dictionary_entry_unknown_before_its_tt` | entry ingested at TT=K is `UnknownToken` at TT=K−1 and Ok at TT=K | leaking future knowledge into a cut (the `as_known_on` class of defect) | **falsifier** |
| T4 | `dictionary_retract_removes_token_only_after_cut` | after `retract_dictionary_entry`, resolution fails at a later TT and still succeeds at an earlier TT | treating retract as delete (D013-2: losers remain) | **falsifier** |
| T5 | `dictionary_entries_order_independent_of_index_mode` | same log under `FoldIndexMode::Eager` and `Lazy` returns identical ordered vectors | the cold/warm order split (cold: `facts` order `fold_query.rs:103-111`; warm: `vt_by_from` order `:113-121`) | **falsifier** |
| T6 | `dictionary_kind_relation_excluded_from_product_as_of` | a `kutha:dict:*` fact is absent from `as_of` and present in `dictionary_as_of` | a dictionary entry leaking into the product picture (the `dictionary_rel_flags` contract, `fold.rs:541-546`) | structural |
| T7 | `dictionary_identical_entry_is_idempotent` | re-emitting the same `(kind,name,facet,vt)` returns the original `EventId` and grows the log by zero facts | a duplicate live row making later resolution ambiguous | **falsifier** |
| T8 | `dictionary_conflicting_entry_overlapping_vt_fails_closed` | same key, different facet, overlapping VT → named `Conflict` error, log unchanged | silent overwrite (D013-3) or a last-writer-wins fold | **falsifier** |
| T9 | `dictionary_facet_digest_mismatch_fails_closed` | entry pinning digest D with bytes of a different body → rejected, no append | an opaque body accepted by identity alone (the F9 shape) | **falsifier** |
| T10 | `dictionary_facet_of_wrong_kind_fails_closed` | a `vocabulary` facet written under `kutha:dict:mode` → `KindMismatch` | one blob read under two identities | structural |
| T11 | `pack_dictionary_facet_installs_without_going_live` | pack installed with a vocabulary ref: `dictionary_validate` fails before activate, succeeds after; `retire` leaves facts and fingerprint unchanged | ownership and liveness conflated; retire mutating the graph (would break `m179_gate_gc.rs:94-98`) | **falsifier** |
| T12 | `binder_relation_gate_reads_fold_not_yaml` | a relation retracted from the log no longer binds, and an allowlisted-but-unwritten name still binds, with `KUTHA_RELATIONS_PATH` pointing at an empty file | the residual F10 YAML root at `gpml.rs:66` | **falsifier** |

Two mechanical notes for the lease:

- **Mutation mechanics.** The harness falsifier driver is `uv run kutha-gov selftest` (mutates a copy; an unproven row fails). Only rows declared in `.kutha/dictionaries/checks.yaml` participate; C2 adds **no** governor row (§5.10), so "falsifier" here means "the reviewer must run the named mutation by hand and record the flip", not "CI proves it".
- **T12 environment hazard.** `KUTHA_RELATIONS_PATH` is process-global (`allow.rs:43-52`) and `apply_dotenv` writes into the process environment once (`allow.rs:9-41`). A test that sets it races any parallel test. Either give `RuntimeBinder` an explicit allowlist source, or mark the test `#[serial]`-equivalent (no such crate is a dependency; prefer the injected source).

---

## 7. Risks and open questions for the owner

Each with a recommended default. O-numbers are cited by §3/§4.

### 7.1 Blockers before any execution

- **O1 (lease).** `SIX-01` is not leased: `.kutha/STATE.md:39` says "ADR-050 six dictionaries | SIX-01 is leased"; honeycomb has `delivery: frozen` for ADR-050 (`.kutha/dictionaries/honeycomb.yaml:495-504`). C2 cannot start as a product change until the owner names the milestone in `.kutha/STATE.md`. *Recommended default:* lease one kind per milestone (the roadmap's wording: "SIX-01 stays six, but is leased one at a time" — `.compound-engineering/artifacts/research/2026-10-22-long-horizon-direction-roadmap.md:118`), starting with §7.2 O2.
- **O2 (first kind).** Which kind first. *Recommended default:* **Controlled vocabulary**, then **Action + Mode** (the derived-FSM pair), then **Relation conformance** (move FF6 onto the same shape with no change to admit results), then **Policy** (shape only, no evaluation), then **Ontology** (last; needs SMO). Rationale: vocabulary is the only kind that adds a consumer surface (binder/pack) without touching admission results, execution, or capabilities — the cheapest honest proof of the substrate. Choosing Relation first would risk a rename that proves nothing new (the allowlist already works).

### 7.2 Design decisions

- **O3 (namespace).** New `kutha:dict:<kind>` prefix vs reusing the five reserved relations. *Default:* new prefix; the five stay facts *about* the plane (§4.2). Risk if merged: `kutha:action` instance rows and Action-kind entries share one relation and one `by_delivery_key` namespace (`pipeline.rs:71`).
- **O4 (delivery key).** *Default:* `None` for dictionary entries, idempotency at the constructor (§3.3). Risk if the owner prefers the registry-slot shape: dated revisions become `DeliveryKeyConflict` because the liveness test is not VT-aware (`fold_query.rs:513-537`).
- **O5 (ambiguity).** Reject at admission and fail closed at read vs newest-wins at read. *Default:* fail closed both places (§3.2); a legacy log that already holds an ambiguity is then a recorded defect, not a silent pick.
- **O6 (pack retire).** Retire hides entries from pack scope and leaves facts, or retracts them. *Default:* leave facts (`m179_gate_gc.rs:94-98` proves retire is graph-neutral; a retracting retire would contradict it). A separate explicit `retract_dictionary_entry` call deletes.
- **O7 (facet body strictness).** `deny_unknown_fields` on facet structs, and raw-SHA-only addressing. *Default:* yes to both, even though `PackManifest` did neither (`pack_manifest.rs:17-24,164-183`) — the dictionary facet is a long-lived audit object, and a silently-ignored field is worse there than in a manifest.
- **O8 (snapshot id).** Whether `dictionary_snapshot_id` goes onto receipts in C2. *Default:* compute and expose it, do not change receipt shape; ADR-014 binding is a separate slice (ADR-050 open question 2).
- **O9 (naming collision, §3.6).** `dictionary` keeps meaning the intern map; the new surface keeps the ADR word in API names. *Default:* as stated; a rename of `TermDictionary` is its own slice.

### 7.3 Risks a reviewer should weigh

| Risk | Evidence | Mitigation |
|---|---|---|
| **Six kinds become six side tables despite choosing B** | exactly what F11 describes (`semantic-gap-review.md:120`) | one lease keyed `(kind, name)` (§4.6); a reviewer check that `fold.rs` gained no `is_live_at` copy |
| **`kutha:*` bypass is a hole**: `admit` returns `Ok` for *any* `kutha:`-prefixed relation before the allowlist check (`admission.rs:213-215`) | read in source | C2's kind branch must reject unknown `kutha:dict:*` names, and must not extend the bypass by accident |
| **The `StoreBlob` three-way digest** lets one blob answer to a rule pin, a policy pin, and its own SHA (`event.rs:683-698`) | read in source | facets require raw SHA only (§4.1 rule 3) |
| **Intern-id shift** changes numeric ids inside snapshots and id-asserting tests | genesis intern order, `quantum.rs:424-437` | append the six names at the end (§4.2) |
| **Dictionary growth is unbounded** and lives in RAM with the fold | `GraphFold` is fully resident (`fold.rs:257-343`) | out of C2 scope, but the facet-size cap (e.g. 64 KiB, `StoreBlob` already carries bodies today) should be stated in the lease |
| **A pack facet makes a third-party vocabulary authoritative** for the host | ADR-021 D021-3 "packs register into Kutha; they do not vendor a second engine" | pack-scoped resolution only; host-level `dictionary_validate` does not read pack scope unless asked |
| **Two vocabularies for one token name**: the intern map and the vocabulary kind both claim "this string means X" | D050-3 fence | the vocabulary entry never rewrites the intern map; it constrains *legality at a cut*, not the string↔id mapping |

---

## 8. What a C2 lease would actually execute

Ordered slices, each independently closable. No slice is authorized by this document.

| Slice | Content | Exit evidence |
|---|---|---|
| C2-S0 | substrate only: `DictionaryKind`, reserved names interned at the end of genesis, one fold lease, `dictionary_entries_at` / `entry_at` / `facet` | T1, T5, T6; identity slice `dictionary_as_of` unchanged |
| C2-S1 | vocabulary kind: facet schema, `emit_dictionary_entry`, conflict + idempotency | T2, T3, T4, T7, T8, T9, T10 |
| C2-S2 | binder fix: `RuntimeBinder` consults the fold | T12; `ff6_allowlist` + `m179_gate_gc` still green |
| C2-S3 | Action + Mode kinds and the derived-FSM read model | new structural test that the FSM is not fold state (fingerprint unchanged by a derived-FSM read) |
| C2-S4 | pack facets: manifest refs, install validation, activate/retire semantics | T11 |
| C2-S5 | Relation conformance: move FF6 onto the entry shape without changing admit results | the whole existing `ff6_allowlist` and `m013_*` suites unchanged, plus `admit` result parity |

Gate for the close: `cargo test --workspace` exit 0 and `uv run kutha-gov ci --jobs 1` with `run_high=0` (AGENTS.md verification honesty), and the bench gate if `crates/` write-path code changes: `ingest_plain.n` and `as_of.n` within 10%.

---

## 9. What this spike did not verify

Read-only, no cargo, no `graph index`. Therefore:

1. **No compile-level claim.** "Four exhaustive matches must change under option A" is a reading of `event.rs:308-333`, `event.rs:416-633`, `commit.rs:17-35`, `fold.rs:893-1077`; the compiler is the authority.
2. **No behavioural claim is proven by execution.** T2–T12 are *proposed* oracles, not run oracles. In particular T5's order divergence is code reading (`fold_query.rs:103-121`), not an observed failure.
3. **Forward-compat claims are code reading of three read paths** (`store.rs:1008-1010`, `wal.rs:147-169`, `segment.rs:547-559`). I did not build an older binary to observe the failure modes.
4. **No performance claim.** Adding one fold lease and one admission branch has an unmeasured cost; the bench gate above is the guard.
5. **No `blob_digest_matches` exploit was demonstrated**; the "one blob, three identities" risk is a reading of `event.rs:683-698` combined with `admission.rs:104-111`.
6. **The `m179_gate_gc` retire-neutrality property is cited from the test body**, not re-run.

A lease should open with a RED oracle for whichever of these it relies on first.
