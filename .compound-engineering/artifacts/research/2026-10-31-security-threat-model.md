# Security threat model — milestone input for ADR-080 / ADR-081 / ADR-051 (C3)

**Date:** 2026-10-31
**Scope:** authorization, capability and isolation threat model for a future *security* milestone. Read-only research; no product change, no lease claimed.
**Repository state:** branch `work/main-line`, HEAD `d430a14` (the task brief named `2680cd9`; the tree moved, all citations below are against `d430a14`).
**Lease:** none (`Active Milestone: None`, `.kutha/STATE.md`). ADR-080/081 thaw gate: "a security milestone is leased (after M003)" (`.kutha/STATE.md` § Thaw map). 081 and 080 are `delivery: frozen`, `capability: none` in `.kutha/dictionaries/honeycomb.yaml` (rows at lines 639–661).

**Relation to M182.** `269-SECURITY.md` (M182, `.planning/milestones/v1.75-phases/269-meta-surface-threat-audit-and-adversarial-tests/269-SECURITY.md`) closed 10 threats over the *meta* surfaces (manifest, lifecycle, enrichment, replay reader gate) with `threats_open: 0`. This document does not restate those ten. It takes them as **already mitigated** and extends them along four axes M182 deliberately left open: (a) there is no principal, (b) there is no read-side enforcement, (c) blobs and receipts are untyped/undecomposed, (d) tenancy and sandbox do not exist. Every M182 threat that must be re-opened by this milestone is called out explicitly as `re-opens T-269-NN`.

**Evidence rule.** Every claim about code carries `file:line` at `d430a14`; every claim about a test names the test file and `fn`. Where a claim is an inference rather than an observation it is marked *(inference)*.

---

## 1. Assets and trust boundaries

### 1.1 Assets

| # | Asset | Where it lives today | Classification | Notes |
|---|-------|----------------------|----------------|-------|
| A1 | Event log (SoT truth) | `crates/kutha-runtime/src/quantum.rs:314` (`pub(crate) log: EventLog`), persisted by `store::persist` / `segment::persist_log` | integrity-critical, append-only | D1: the log *is* the truth; every other asset is a fold of it (ADR-000 D1, line 90) |
| A2 | Fold + leases (CSR, typed CSR, hot maps) | `crates/kutha-runtime/src/fold.rs`, `csr.rs`, rebuilt from the log | droppable | a wrong lease is a correctness bug, not truth loss |
| A3 | Pack declarations (identity, operator names, manifest pin) | `Op::PackInstall` + `StoreBlob` manifest, read models `pack_lifecycle.rs`, `pack_registry.rs` | integrity-critical | declares authority *names*; no code |
| A4 | Enrichment proposals (derived content) | `Op::EnrichmentProposal` + payload blob, ledger `enrichment.rs:164` | derived, non-truth | never creates/supersedes a kernel fact (`enrichment.rs:1-7`) |
| A5 | Receipts (quantum evidence) | `Op::QuantumOutcome` events (`pipeline.rs:432-454`), `Op::Checkpoint` receipts (`provenance.rs:227`) | audit | the only evidence a caller can be shown for "this quantum happened" |
| A6 | Blobs (policy bodies, rule bodies, manifests, enrichment payloads, checkpoints) | one untyped map `fold.blobs: HashMap<String, Vec<u8>>` (`fold.rs:576-581`) | integrity-critical | one namespace, three accepted digest schemes (§1.4) |
| A7 | Tenant data | **does not exist as a first-class asset** | — | isolation today is "one store directory per tenant"; see T-07 |
| A8 | Dictionary / intern map | `TermDictionary` + `Op::Define` events | integrity + bounded resource | ADR-011 owns the intern map; no cap exists (`quantum.rs:468-482`) |
| A9 | Harness plane state (`.kutha/events.jsonl`, `.kutha/STATE.md`, dictionaries) | outside `crates/` | process evidence, not product truth | gitignored observation log |

### 1.2 Trust boundaries as drawn today

```
  untrusted-ish                                trusted                                trusted
  ─────────────                                ───────                                ───────
  caller of Runtime::emit / assert  ──►  admission (admission.rs)  ──►  commit + fold  ──►  store dir
  caller of Runtime::emit_allow /               │                          │
  emit_policy / emit_pack_install  ────────────┘                          └──► QuantumOutcome receipt
        (NO gate at all — trusted constructor)

  .kutha/events.jsonl  ──kutha-tenant bin──►  Runtime::default()  ──►  .kutha/tenant store
        (agent-writable, unauthenticated)          (trusts the file)

  store dir on disk  ──open/replay_check──►  fold          (tamper detection: fingerprint,
                                                            checkpoint receipts, blob digests,
                                                            meta replay, tt anchors)
  reader via as_of / live_at / query_ir::execute  ──►  rows      (NO policy gate at all)
```

The dashed asymmetry is the whole finding: **the write path has gates, the read path has none, and the gate set has no notion of *who*.**

### 1.3 Who is trusted today, exactly

| Trust decision | Code | Consequence |
|---|---|---|
| Every caller of the public Rust API is the same principal | `pipeline.rs:931-934` `emit`, `pipeline.rs:790` `emit_allow`, `:871` `emit_policy`, `:834` `emit_rule_ast`, `:682/:692/:741/:756` pack lifecycle, `:771` `emit_tt_anchor`, `:785` `emit_retract_dict`, `:618` `emit_enrichment_proposal` | a *dictionary-constructor* call is indistinguishable from a product write. There is no capability parameter, no principal argument, no `Session` type |
| User writes are rejected on reserved names | `pipeline.rs:338-361` `reject_forged_meta` (meta ops + `kutha:admission`/`kutha:action` Assert) and `admission.rs:180-197` (`ReceiptMode::User` reserved-relation rejection) | defence in depth by two independent gates; tested at `crates/kutha-runtime/tests/m013_write_path.rs::public_emit_assert_kutha_allows_is_refused`, `::public_emit_behavior_kutha_relation_is_refused`, `::public_emit_retract_kutha_allow_is_refused`, `::public_emit_correct_kutha_allow_is_refused` |
| Any relation named `kutha:*` skips the relation allowlist | `admission.rs:213-215` (`if name.starts_with("kutha:") { return Ok(()) }`) | reachable only through the trusted constructors above (User mode rejects the name first at `admission.rs:182-183`); still a *structural* bypass — see T-02 |
| Product relation names must be allowed by a live `kutha:allows` fact | `admission.rs:209-230` → `fold_query.rs:559-584` | the only temporal (VT×TT) authorization in the system |
| Those allow facts come from a YAML file + env var on a *fresh* log | `quantum.rs:426-438` (genesis `emit_allow` over `load_allowed_names`), `allow.rs:49-59` (`KUTHA_RELATIONS_PATH`), `allow.rs:11-47` (`.env` discovery from `CARGO_MANIFEST_DIR` up to 5 levels, `set_var` of every unset key) | bootstrap policy is **process configuration**, not log truth; after genesis it is log facts |
| Pack lifecycle ops are admitted only when the meta leases are trusted | `admission.rs:112-126` gated on `meta_leases_trusted` (`quantum.rs:379`), set true at construction (`quantum.rs:418`) and recomputed on hydrate (`hydrate.rs:21-70`) | protects against opening a hand-edited store, not against a hostile in-process caller |
| The harness JSONL may be ingested into a product runtime | `crates/kutha-runtime/src/bin/kutha-tenant.rs:19-20` (`KUTHA_HARNESS_LOG` defaults `.kutha/events.jsonl`, `KUTHA_TENANT_DIR` defaults `.kutha/tenant`), mapping at `tenant.rs:71-104` | the harness plane is an *unauthenticated input* to a product runtime instance; the product core never reads `.kutha/` otherwise (grep over `crates/*/src`: only `tenant.rs`, `bin/kutha-tenant.rs` mention harness) |
| Opening a store directory trusts the filesystem | `store.rs:64-84` advisory `File::try_lock` on `kutha.lock`, `store.rs:35` | single-writer is advisory and same-host only (ADR-000 D8.2 pointer, line 211); NFS/cross-host uncovered |

**Where the boundary really is.** Today the security boundary is the *process*, not a principal. Everything inside the process is fully authorized; anything that can call the crate is the operator. That is a legitimate P0 posture, but it means ADR-080/051 cannot be "added" as checks — they require introducing a principal, which is the first real work item (§4).

### 1.4 Blobs: three accepted digest schemes, one untyped namespace

`StoreBlob` admission accepts a pair if **any** of three hashes matches: raw SHA-256 of the bytes, the canonical rule-body hash, or the policy UTF-8 hash (`kutha-common/src/event.rs:682-698`, checked at `admission.rs:104-111` and again on replay at `provenance.rs:93-103`). Digests are plain `String` keys in one map, first-write-wins (`fold.rs:576-581`).

Consumers re-verify selectively:

| Consumer | Read | Re-verifies digest? |
|---|---|---|
| Pack manifest | `pack_manifest.rs:165-183` | **yes, raw SHA-256 only** (`:177-181`, comment at `:174-176`) — tested `m183_digest_falsifiers.rs::policy_hash_addressed_manifest_is_rejected` |
| Enrichment payload | `enrichment.rs:143-152` | **yes, raw SHA-256 only** — tested `m183_digest_falsifiers.rs::policy_hash_addressed_enrichment_payload_is_rejected` |
| Admission policy body | `admission.rs:58-75` | **no** — bytes are read by pin and only compared to the literal `"deny"` |
| Horn rule body | `fold.rs:735-745`, `eval.rs:191-196`, `pipeline.rs:132-138` | **no** — `RuleAst::from_canonical` parses and requires an empty tail (`kutha-common/src/rule.rs:145-149`); it does **not** compare `rule_body_hash(bytes)` to the pin |
| Checkpoint blob | `fold.rs:600-612` → `checkpoint_snapshot_if_receipt_ok` | yes (receipt match), also `provenance.rs:244-263` |

So the invariant "a blob is what its key says it is" is currently enforced by hash strength plus two conscientious call sites, not by the type system. That is the substance of T-04.

---

## 2. Principals and capabilities

### 2.1 Principals that exist, or are implied

| Principal | Exists? | Holds what today |
|---|---|---|
| Process / API caller | yes, unnamed | everything: inject (`emit`), administer (all `emit_*` constructors), read (`as_of`, `live_at`, `fold()`, `execute`), persist (`store::persist`) |
| Pack | **declaration only** | a name and a list of operator *strings* (`pack_manifest.rs:17-24`); it cannot act, because nothing resolves an operator to code — `derive_pack_operators` is a read model (`pack_registry.rs:1-6`), and T-269-10 accepts the absence of an executor (`269-SECURITY.md` R-269-02, with an explicit re-open trigger) |
| Enrichment pack | yes, narrow | may append a proposal while its lifecycle state is `Active` and its version matches (`enrichment.rs:113-135`) |
| LLM / agent | **no** | there is no agent identity anywhere in `crates/`; the LLM is outside the trust boundary by design (ADR-050 context) |
| Log reader (auditor) | no separate principal | same privileges as the writer |
| Harness (Python governor) | yes, out-of-process | writes `.kutha/events.jsonl`, sets the lease; never touches product truth |

The milestone's first deliverable is therefore not a check but a **principal**: a value that is carried on the write and read entry points and recorded on the log, so a policy can be *about* someone. Without it, grants are records about nobody and a path policy protects nobody.

### 2.2 Why a capability must be an unforgeable reference, not a role string

A role string is data. If authority is a string (`"role": "auditor"`), then anyone who can write facts can write that string, and every consumer must re-decide whether to believe it. The current system already demonstrates the failure mode in miniature: `kutha:allows` facts are *name grants*, and the gate at `admission.rs:209-230` believes the fact, not the writer — which is sound only because the writer is already fully trusted (§1.3).

ADR-051 D051-1 fixes the direction: authority is an unforgeable reference, and *passing the capability into a plugin is the Port*. Concretely for Kutha:

- A capability is a **runtime value** whose existence proves the grant was resolved: `Cap<Port>` created only by the admission/authorization component, non-`Clone` where it authorizes an effect, and never serializable to the log. It names `(principal, object-or-slice, operator-or-port, scope, valid-window)`.
- A **grant** is the durable, auditable record: a VT×TT fact (D080-2) that says the capability *should* exist for that window. A grant record never *is* the bearer, because a log row can be copied; a reference cannot (D051-3 wording: "recording a grant does not make an arbitrary event reference an unforgeable bearer capability").
- Therefore the two live in different places: grants in the fold (auditable, time-travel-able, replayable), capabilities in the process (unforgeable, droppable, rebuilt by resolving grants at the decision cut).

**What this rules out** (and the milestone must refuse): storing capabilities as a second ACL table that the LLM edits (D051-1); exposing a tool list and calling it authorization (D051-2, ScopeGate); binding an authority-bearing argument from retrieved text (D051-2 clarification: retrieved text may *propose* a value, never grant authority for it).

**How grants/revokes become VT×TT facts.** They are `Op::Assert`/`Op::Retract`-shaped rows like any other fact, evaluated at the invocation cut for *permission to act now* and at a historical cut only for *audit of what held then* (D080-2 clarification). The clarification is a hard test, not a comment: choosing `AS OF` before a revocation must not restore the caller's present access. Today's relation allowlist already has this shape and is the only place the property is exercised — `fold_query.rs:559-584` evaluates `kutha:allows` liveness at `(tt, vt)`, and `m012_allowlist_facts.rs` / `m013_genesis_log.rs` cover it.

**Why the abstraction is needed at all** (and not just more checks): the confused-deputy problem is structural. The runtime holds all authority; if a pack or a model can make the runtime act on its behalf without a distinguishable handle, the runtime is the deputy. A capability that can only be obtained from the authorizer, and that is consumed by the operation, is the only mechanism where "the pack had the grant" is a *type-level* fact rather than a log claim.

### 2.3 What `security.rs` does and does not do

`crates/kutha-runtime/src/security.rs` (158 lines) is a **closed-default compile-time path policy** over the existing `QueryIr` (only two variants: `Scan`, `Hops` — `query_ir.rs:24-42`).

**It does:**
- `PathPolicy::closed()` denies everything; `allow_relations` and `allow_scan` opt in (`security.rs:22-48`).
- `compile_path_policy` denies `Scan` unless `allow_scan` (`:94-102`), denies zero-hop or unlabeled walks (`:106-110`), and requires **every** relation in a hop chain to be allowed (`:111-124`), failing closed with `UnknownRelation` when a `TermId` has no name (`:112-116`).
- Denial is a typed error, never empty rows (`:133-144`; `execute_authorized` → `SecurityError::PolicyDenied`). Tested: `m175_security_policy.rs::labeled_allowed_hop_compiles_allowed`, `::denied_relation_compiles_denied_before_prepare`, `::scan_denies_by_default_and_allows_explicitly`, `::unlabeled_and_zero_hop_walks_deny_by_default`, `::multi_hop_requires_every_relation`; integration `m175_security_integration.rs::authorized_gpml_result_equals_direct_execution`, `::denied_gpml_returns_policy_error_not_empty_rows`, `::authorized_typed_two_hop_preserves_query`, `::closed_default_scan_and_unlabeled_are_denied`.
- It does **not** materialize unauthorized hops: the decision precedes `prepare` (`security.rs:129-131`), which is the D080-1 shape (rewrite/deny in the plan, not a post-filter).

**It does not:**
- **Run anywhere in production.** `compile_path_policy` / `execute_authorized` have no call site outside `tests/` (grep over `crates/ -g '!tests/*'`: zero hits; re-exported at `lib.rs:64-66`). Every read entry point is open: `Runtime::as_of` (`quantum.rs:747`), `Runtime::live_at` (`quantum.rs:846`), `Runtime::fold()` (`quantum.rs:572`), `QueryIr::execute` (`query_ir.rs:342-345`), CSR leases (`quantum.rs:591`, `:610`), `dictionary_live_at` (`fold_query.rs:249`).
- **Come from the log.** `PathPolicy` is a Rust struct built by the caller; it is not derived from `kutha:allows`, not versioned, not temporal, and carries no principal. Two independent, unrelated authorization surfaces now exist (write-time allowlist from the log; read-time allowlist from a Rust literal), with no cross-check.
- **Express rows or slices.** It constrains *relation names on a path*, never *which vertices* — so it cannot serve as a tenant slice, exactly as ADR-080 separates "may this hop return?" from "is this vertex in the slice?".
- **Cover anything but `QueryIr`.** GPML and any future surface must remember to call it, and nothing does today (first bullet above); there is no chokepoint, so "remember to call" is the enforcement mechanism.

---

## 3. Threats

Severity: **critical > high > medium > low**. "Existing mitigation" cites code and, where a test proves it, the test. "Delta" is what the milestone must add — no more.

### T-01 — Forged meta-ops through the public `emit` (STRIDE: Spoofing/Tampering) · **medium (residual), was high**

**Attack path.** A caller builds `Op::PackInstall` / `Op::EnrichmentProposal` / `Op::QuantumOutcome` / `Op::JustificationCite` / `Op::Checkpoint` and calls `emit`. Today these are refused by name (`pipeline.rs:338-350`). Two variants survive:

1. **`Op::Define` is not on the refuse list.** `emit(Op::Define { name })` passes `reject_forged_meta` (`:338-350`), passes `gate_user_op` (`:363-430` — no `Define` arm), reaches `admit` (`:209-211`, `op_relation(Define) == None` → `Ok`), and is committed. Effect: unbounded dictionary growth under caller control, and — because `intern_silent` calls `bind_reserved` for any new `kutha:`-prefixed string (`quantum.rs:484-490`) — a caller can flip `dictionary_rel_flags` (`fold.rs:540-556`) for an arbitrary name. Asserting a fact under that name is still refused (`admission.rs:182-183`), so this is a budget/integrity nuisance, not an escalation *(inference: no path found from Define to an admitted `kutha:`-named Assert)*.
2. **`Op::StoreBlob` and `Op::TtAnchor` are deliberately re-routed**, not refused: `pipeline.rs:470-472` sends both to `ReceiptMode::Dictionary`. So an arbitrary caller can store blobs (bounded only by the digest rule) and can mint anchors (anchor-freeze path, T-09) — with no principal on the record.

**Existing mitigation.** `pipeline.rs:338-361`; tests `m182_meta_threats.rs::user_forged_meta_ops_are_rejected`, `m177_pack_manifest.rs::user_forged_install_with_manifest_digest_is_rejected`, `m178_enrichment.rs::user_forged_proposal_is_rejected`. **Delta:** classify ops by authority required (`Inject`, `Administer`, `Propose`, `Read`), not by name; require the corresponding capability at `emit`, so the check is a type/capability gate rather than an enum list that a new `Op` variant can silently miss. The `Op` enum is `#[non_exhaustive]`-free and grows every milestone (m050–m076 are all Op migrations), so name-list maintenance is a real recurring risk.

### T-02 — `kutha:*` namespace bypass of the relation allowlist (STRIDE: Elevation of privilege) · **medium**

**Attack path.** `admission.rs:213-215` returns `Ok` for any relation whose interned name starts with `kutha:` — before the `relation_allowed_at` check at `:221`. The only reason this is not an escalation is that `ReceiptMode::User` rejects the same names earlier (`:180-197`), and every non-User entry point is a trusted constructor. So the control is *ordering*, and the two gates are maintained separately: `reject_forged_meta` names only `kutha:admission` and `kutha:action` (`pipeline.rs:351-358`), while `relation_name_reserved` covers all of `KUTHA_RESERVED_RELS` plus the prefix (`admission.rs:233-237`, constants `event.rs:148-161`). A future `Op` or a new non-User constructor that forgets the User-mode pre-check inherits an open door.

**Second-order effect.** `is_dictionary_rel` is derived from the *name prefix* (`fold.rs:540-556`), and it removes facts from product reads (`fold_query.rs:41`, CSR build filters at `quantum.rs:659` and `:726`). So "reserved" is a *visibility* property keyed on a name, not on how the fact was admitted. A product relation may never be named `kutha:*` (the allowlist never contains such a name — `crates/kutha-runtime/dictionaries/relations.yaml` lists only `knows/knownBy/relatedTo/inForceAs/runStatus/observed/processAllows`), so today this is clean; it becomes fragile as soon as dictionaries become user-extensible (ADR-050, 6 dictionary kinds).

**Existing mitigation.** The User-mode gate and its four tests (T-01 citation). **Delta:** make reservedness an admission-time *category* recorded on the event (or a closed `Op` variant), never a name test at read time; and assert in one place that every reserved relation is admitted only by a `Dictionary`/`Administer` capability.

### T-03 — Operator-name collision and post-retire name reuse (STRIDE: Spoofing) · **medium**

**Attack path.** Operator names are a single global namespace. Collisions across packs are refused at admission and replay (`pack_manifest.rs:189-216`, `OperatorIndex::declare` at `:235-260`); tested `m182_meta_threats.rs::cross_pack_operator_collision_fails_on_replay`, `m177_pack_manifest.rs::cross_pack_operator_collision_fails_closed`. But `PackRetire` **releases** the names (`pack_manifest.rs:263-269`, `hydrate.rs:48`), tested `m177_pack_manifest.rs::retired_pack_releases_operator_names`. Consequences the milestone must handle:

1. A second pack may claim a retired pack's operator name. Any past record that cites "operator X" is then ambiguous between pack A (retired) and pack B (active), because the record carries only the name — `PackOperator` (`pack_registry.rs:13-18`) keeps `pack_id` + `version` for audit, but there is *no code digest anywhere* (manifest v1 has `schema/pack_id/name/version/operators` only, `pack_manifest.rs:17-24`).
2. A pack id cannot be reinstalled (`pack_lifecycle.rs:131-133`, `pack_id_already_known`), so there is no version drift under one identity — but also no legitimate re-install after retire *(availability, not security; note it)*.
3. Names are not tenant-scoped: two tenants sharing a store cannot both install `extract` even in principle.

**Delta:** add a content pin for behaviour (pack code/artifact digest) to the manifest so a name resolves to *identity*, not to a string; scope operator resolution by the principal/tenant slice; keep names globally unique only within a slice.

### T-04 — Blob namespace / three-way digest confusion (STRIDE: Tampering, Spoofing) · **high** (design debt with one concrete read-side hole)

**Where it is solid.** Substituting one byte string for another under a fixed key requires a SHA-256 preimage: all three accepted schemes are hashes of the bytes (`event.rs:682-698`), the map is first-write-wins (`fold.rs:576-581`), and the two consumers that matter for identity (manifest, enrichment) re-verify raw SHA-256 (`pack_manifest.rs:177-181`, `enrichment.rs:148-152`), with direct falsifiers `m183_digest_falsifiers.rs::policy_hash_addressed_manifest_is_rejected`, `::policy_hash_addressed_enrichment_payload_is_rejected`, and the positive control `::raw_sha256_addressing_still_passes_both_checks`. Re-opens/mirrors T-269-02 and T-269-07 as *type* rather than digest problems.

**Where it is not.** The read side trusts the key:
- Admission reads a policy body by pin and does **not** re-verify (`admission.rs:58-75`); its only interpretation is `s.trim() == "deny"` (`:70`). Any other bytes are *inert*, i.e. the policy silently passes. A caller who believes `kutha:policy` names a policy language gets a kill-switch.
- Horn evaluation reads a rule body by pin (`fold.rs:735-745`, `eval.rs:191-196`, `pipeline.rs:132-138`) and accepts whatever canonical rule the bytes encode; `RuleAst::from_canonical` (`rule.rs:145-149`) checks parseability, not `rule_body_hash`. Behaviour admission checks only the pin's liveness (`pipeline.rs:416-428`), not the body.
- Because keys are untyped strings, *one* byte string is legitimately reachable under three different digest values. Nothing in the code says which consumer may read which address form; the two "strict" consumers encode that policy as a comment (`pack_manifest.rs:174-176`).

**Concrete attack path (the one that must be fixed).** Any future consumer added without a re-verification line inherits an untrusted read. Today's cheapest example is the policy path: a caller with `emit_store_blob` power can ensure a blob exists whose key is accepted as a policy pin by storing bytes whose *policy* hash equals that key, and then assert `kutha:policy` naming it — legitimate, since both operations are the same trusted principal. The defect is not that it works; it is that the runtime has no way to reject a *mis-typed* pin, so the day a second principal can write blobs (T-05/T-06 direction), a rule-body blob becomes addressable as a policy pin and vice versa.

**Delta:** typed blob keys (`sha256:` / `rule:` / `policy:` / `manifest:` / `payload:` / `checkpoint:`) enforced at `StoreBlob` admission and at replay; a single `resolve_typed(key, expected_kind)` that re-verifies, replacing every raw `blobs.get` read (`admission.rs:64`, `:143`, `fold.rs:600`, `:735`, `pipeline.rs:132`, `eval.rs:191/231/271`, `provenance.rs:171/244`).

### T-05 — Enrichment proposal replay and cross-pack suppression (STRIDE: Repudiation, Denial) · **medium**

**Attack path.**
1. `cache_key` is a **global** set, not per pack (`enrichment.rs:155-159`; collected without pack qualification at `hydrate.rs:41-45` via `enrichment_cache_keys.insert(cache_key.clone())`). Pack B's proposal with a key Pack A already used is rejected as `DuplicateProposal`. This is intended dedupe per the ADR-052 replay story, but it is also a **cross-pack write suppression primitive**: whoever proposes first with a guessable key (model + prompt + source ids are all public inputs) permanently blocks another pack's proposal for that input.
2. Replay of a *proposal* is prevented by the same key check plus pack-state/version checks; admission and replay share the check (`admission.rs:127-154` and `provenance.rs:99-200` both call `check_proposal`), tested by `m185_enrichment_admission.rs::admission_and_replay_agree_on_proposals`, `::admission_rejects_duplicate_cache_key_without_log_scan`, `m178_enrichment.rs::unknown_source_and_duplicate_key_fail_closed`, `m180_replay_meta.rs::duplicate_enrichment_cache_key_fails_replay`.
3. **Source existence is a global oracle**: `source_known` is `self.log_ids.contains(&id)` (`admission.rs:142`, `pipeline.rs:655`), so a proposal reveals whether *any* event with that id exists — accepted risk R-269-01, explicitly deferred "when tenant slices land". With tenants, this becomes a cross-tenant existence oracle.
4. Attribution after retire: the ledger keys state by `pack_id` and takes the *final* lifecycle state (`enrichment.rs:164-198`), so every proposal of a retired pack is reported `Retired`; proposals are still listed (by design — audit). No proposal can be admitted for a retired pack (`:120-124`), so replay of an old proposal is bounded to the prefix that was legal at the time — which is correct, and must stay that way.

**Delta:** scope `cache_key` to `(pack_id, model, prompt, source-event set)`; make the source-existence predicate slice-scoped (fails the leak only *after* tenancy exists, so sequence it with T-07); record the admitting principal on the proposal.

### T-06 — Receipt tampering (STRIDE: Repudiation) · **high**

**Attack path.** `QuantumReceipt.digest` is SHA-256 over the concatenated event digests plus the aborted flag (`receipt.rs:18-29`), and `quantum_id` is that digest's hex (`pipeline.rs:438-447`). The receipt is persisted as a `QuantumOutcome` event carrying `event_ids`, `receipt_digest_hex`, `disposition`, `resume_of` (`quantum.rs:219-256`). **`replay_check` never recomputes it.** Its checks are: fold fingerprint (`provenance.rs:49-55`), checkpoint receipts (`:56`, implementation `:227-267`), tt anchors (`:57`), Behavior lineage (`:58-80`), StoreBlob digests (`:93-103`), meta replay (`:104`). Grep for `receipt_digest_hex` over `src/` returns only creation (`pipeline.rs:438`) and copies (`hydrate.rs:95-105`) — no verification.

So an edited or foreign-written log can carry a receipt whose `event_ids` and `receipt_digest_hex` do not correspond to the events in its quantum (add events, drop events, swap a quantum id, or point `resume_of` at any quantum) and still open. The consequence is exactly what ADR-014 exists to prevent: evidence that is *shown* as "this quantum happened" and cannot be contradicted. Note the pre-existing narrowing: M182's checkpoint verification covers `Op::Checkpoint` receipts only.

**Aggravating surface.** `Runtime::attach_outcomes(&mut self, rows)` is public (`quantum.rs:586-589`) and replaces the in-memory outcome buffer wholesale; `outcome_records()` then reports the injected rows (used legitimately by the sidecar loader, `store.rs` OUTCOMES lease). Within one process an attacker with API access can thus present forged receipts without touching the log at all *(inference: `attach_outcomes` has no provenance check; intended caller is the loader)*.

**Delta:** recompute `QuantumReceipt::from_events(event_ids, digests, aborted)` for every `QuantumOutcome` on open and compare `receipt_digest_hex`; make `attach_outcomes` `pub(crate)` or require a `Verified` token; collapse `quantum_id` derivation into the verification so a receipt can never exist without being derivable.

### T-07 — Tenant crossover (STRIDE: Information disclosure, Elevation) · **high** (no mechanism exists)

**Facts.** There is no tenant, slice, or named-graph field on any `Op` (`kutha-common/src/event.rs:165-295`) and none on `Fact`; `tenant.rs` is *not* a tenancy mechanism — it maps harness JSONL rows onto product `Op::Assert`s (`tenant.rs:1-4`, `:71-104`) and `IngestReport` is a counter bundle (`:46-52`). Isolation today = one store directory per tenant, and even that is only as strong as the advisory writer lock (`store.rs:35`, `:64-84`) — cross-host/NFS is uncovered (ADR-000 D8.2 pointer, line 211).

**Attack paths.**
1. **Read crossover, by design.** Any caller with `as_of`/`live_at`/`execute` reads every vertex in the fold; `PathPolicy` constrains relation names only (§2.3). A tenant-scoped deployment must therefore run *separate processes and separate directories*, or accept full crossover.
2. **Write crossover.** `kutha:allows` facts are global; a product relation name allowed once is writable by every writer in that store. There is no per-principal allowlist.
3. **Existence oracles.** Enrichment `source_event_ids` (T-05 item 3) and `kutha:allows`/`kutha:policy`/`kutha:rule` registry reads are global; `dictionary_live_at` (`fold_query.rs:249`) exposes dictionary facts to anyone who reaches it.
4. **Shared blob namespace.** Digest keys are global and content-addressed, so identical payload bytes (e.g. a published statute) are deduplicated across tenants — benign for confidentiality (hashes are preimage-resistant) but it makes "this tenant stored this payload" observable to the other tenant through the enrichment/cache-key oracles *(inference: requires an oracle that distinguishes "already present"; `StoreBlob` de-duplication is silent at `fold.rs:576-581`, so the signal is timing/size, not a return value)*.
5. **The harness→product bridge is a crossover channel.** `bin/kutha-tenant.rs:19-20` ingests `.kutha/events.jsonl` into `.kutha/tenant`; `emit_chained` invents monotonic times by clamping each row to `max(valid_from, cursor)` and stepping the cursor by one second (`tenant.rs:108-141`), so the product fold's temporal truth deliberately differs from the harness row's own timestamp. Not a tenancy bug — a provenance-honesty fact the milestone must not lose.

**Delta:** a `SliceId` on every product fact and on the read entry point; closed default (absent slice grant → no rows, D080-5); per-slice quotas (D080-3); `source_known` and cache keys scoped by slice. This is the largest schema change in the milestone and the one with migration cost (the `m062`–`m076` op-wire migration family is the precedent).

### T-08 — Sandbox escape via pack UDF (STRIDE: Elevation) · **high when the executor lands; not present today**

**Today.** There is no UDF executor. `derive_pack_operators` is a read model (`pack_registry.rs:1-6`), manifests carry strings only (`pack_manifest.rs:17-24`), and nothing resolves a name to a callable. T-269-10 accepted this with a written re-open trigger (`269-SECURITY.md` R-269-02: "any code path that resolves a manifest operator to a callable, or any new `Op` that carries executable payload").

**Pre-registered attack paths** (so they are designed out, not discovered):
1. **Capability leak through the host API.** If the UDF is handed `&mut Runtime` or any handle with `emit`/`emit_allow`, the sandbox is decorative. D081-1 requires the UDF to reach the log only through TOKI/receipt paths, and D051-1 requires the *Port* to be the capability.
2. **WASI as kernel isolation.** A WASM module with WASI file/network access can read the store directory and write outside it (D081 hard separation: "WASI syscalls ≠ kernel isolation"). The grantable surface must be enumerated as capabilities, not as syscalls.
3. **Shared-view aliasing.** D081-2 needs shared Arrow/CSR views to be competitive; a writable shared view into fold memory is an escape by construction (one memcpy of a `Vec<Fact>` and the log's truth is lies). Views must be read-only, and the sandbox must not receive a mutable borrow of any fold structure.
4. **Resource exhaustion from inside the sandbox** — a UDF that allocates or loops forever is the sandbox's own DoS surface, distinct from T-09, and needs instruction/memory fuel, not the cascade budget.
5. **Determinism as a replay bomb.** Any UDF whose output feeds the fold makes replay non-deterministic; `replay_check` recomputes Behaviors from pinned rule blobs (`provenance.rs:63-80`) and would diverge. Sandboxed UDF output must land as *proposals* (A4), never as fold-affecting Behaviour.

**Delta (and the ordering consequence, §4):** the executor arrives *with* the capability set, the read-only view contract, and fuel accounting in the same slice — never before them.

### T-09 — Denial via budget / resource exhaustion (STRIDE: Denial) · **medium, several concrete paths**

| Path | Code | Effect |
|---|---|---|
| Cascade budget is a per-process env constant with a silent abort | `quantum.rs:297-303` (`KUTHA_MAX_CASCADE`, default 32), abort at `pipeline.rs:550-555` with `aborted_on_budget` in the receipt | an attacker who can write facts that fire Horn rules converts one `emit` into up to `max_cascade` events, repeatedly; the abort leaves a partial quantum (rollback exists: `pipeline.rs:519-540`, tested `m052_cascade_atomic.rs`) |
| Unbounded dictionary growth | `quantum.rs:468-482` (`intern` appends a `Define` per name), `pipeline.rs:338-361` (Define not refused) | unbounded log growth and unbounded `dictionary_rel_flags` growth, at one event per unique string |
| Unbounded blob growth | `pipeline.rs:470-472` + `admission.rs:104-111` | any caller may store blobs of any size whose key is a hash of the bytes; no quota, no owner |
| Permanent anchor freeze | `admission.rs:198-207` requires each `TtAnchor.wall` to be strictly greater than the live maximum; `emit_tt_anchor` (`pipeline.rs:771`) and `emit(Op::TtAnchor)` (`:470-472`) are both reachable | one anchor at `u64::MAX` (accepted, since `MAX > max`) makes every later anchor `TtAnchorRejected`. Known-on/`as_known_on` anchoring is then frozen for the life of the log |
| Enrichment cache-key land-grab | `enrichment.rs:155-159` | cheap, permanent suppression of a competitor's proposal (T-05) |
| Replay cost | `replay_check` rebuilds the fold and walks metadata (`provenance.rs:47-107`); `m182_meta_threats.rs::replay_meta_is_not_quadratic` bounds the meta walk only | open-time DoS grows with log size; acceptable at current scale, must be revisited with tenants |

**Delta:** per-principal quotas on (log bytes, event count, blob bytes, interned names, cascade events per window) held in the store, not the environment; a bounded anchor policy (`wall <= now + skew`, reject-and-report rather than permanent freeze); cascade budget per principal *and* per call.

### T-10 — No authorization on the read path (STRIDE: Information disclosure) · **high**

Covered as a finding in §2.3; registered here so severity is explicit. **Attack path:** any code holding `&Runtime` reads the whole store: `as_of` (`quantum.rs:747`), `live_at` (`:846`), `fold()` (`:572`), `execute` (`query_ir.rs:342-345`), CSR leases (`quantum.rs:591`, `:610`), `dictionary_live_at` (`fold_query.rs:249`). `security.rs` is opt-in and unused outside tests. **Existing mitigation:** none on the read path. **Delta:** one chokepoint; `Runtime` read APIs become `pub(crate)`; a `Session`/`View` type is the only public read surface and it carries the principal and the policy.

### T-11 — Bootstrap configuration is authority (STRIDE: Tampering) · **low–medium**

`allow.rs:11-47` walks up to five parent directories from `CARGO_MANIFEST_DIR` looking for `.env` and calls `std::env::set_var` for every unset key; `allow.rs:49-59` lets `KUTHA_RELATIONS_PATH` replace the allowlist file; `quantum.rs:297-303` reads `KUTHA_MAX_CASCADE`. On a *fresh* log, `Runtime::new` seeds `kutha:allows` from that file (`quantum.rs:426-438`). Therefore: whoever controls a `.env` or the environment in a parent directory controls which relation names exist in a newly created store and how large a cascade may be. After genesis the facts are log truth, so the window is "store creation" — but that window is unbounded if a process creates stores. *(Also note the `unsafe { set_var }` comment at `allow.rs:34-36` already flags a race.)* **Delta:** refuse `.env` discovery outside `cfg(debug_assertions)` or an explicit opt-in flag; record the bootstrap source (file digest + env var names) as a log event so "why did this store allow X" is answerable.

### T-12 — Cross-plane confusion: harness truth read as product truth (STRIDE: Spoofing) · **medium** (process risk, not code)

The planes are separate by contract (AGENTS.md "Two planes"; `.kutha/STATE.md` header). The code risk is narrow but real: `bin/kutha-tenant.rs` ingests a gitignored, agent-writable file into a product runtime, and `tenant.rs` re-times rows (T-07 item 5). Nothing in the ingest authenticates the rows or proves the harness wrote them. Consequence: a product fold can be made to contain "harness asserts X" facts whose provenance is unverifiable, while `AGENTS.md` says governor green ≠ capability and `.planning/STATE.md` ≠ `.kutha/STATE.md`. **Delta:** an ingest report that names the source file digest and is itself recorded as an event; refusal of rows that the harness schema does not name (`tenant.rs:81-90` already whitelists three subject/relation pairs — make that a closed schema check with a failure, not a silent skip at `:81-83`).

### 3.1 Threats M182 closed that this milestone must re-open

| M182 ID | Why it re-opens |
|---|---|
| T-269-01 (replay DoS) | per-tenant/per-principal replay cost becomes an attack surface once one process serves many slices |
| T-269-06 (forged meta) | T-01: the refuse-list is by name; the milestone replaces it with a capability gate |
| T-269-09 (enrichment source ids) | R-269-01's own revisit trigger: "when tenant slices land" — that is this milestone |
| T-269-10 (operator execution) | R-269-02's re-open trigger: any executor, or any `Op` carrying executable payload |

### 3.2 Threat summary

| ID | Category | Severity | Ships-without-WASM fix |
|---|---|---|---|
| T-01 | Spoofing/Tampering | medium | yes |
| T-02 | Elevation | medium | yes |
| T-03 | Spoofing | medium | yes |
| T-04 | Tampering | high | yes |
| T-05 | Repudiation/Denial | medium | yes |
| T-06 | Repudiation | high | yes |
| T-07 | Info disclosure / Elevation | high | yes |
| T-08 | Elevation | high (when executor) | n/a — arrives with executor |
| T-09 | Denial | medium | yes |
| T-10 | Info disclosure | high | yes |
| T-11 | Tampering | low–medium | yes |
| T-12 | Spoofing | medium | yes |

---

## 4. Ordering

**Required order: (0) enforcement point + principal → (1) grants as VT×TT facts → (2) capabilities → (3) path-ABAC compiled from grants → (4) sandbox.** Steps (2) and (3) may interleave; the two inversions that matter are that the **enforcement point precedes every policy** (a policy nothing consults is the current state, §2.3) and the **sandbox follows everything** (there is nothing to sandbox yet, T-08). Rationale, shortest form:

1. **Enforcement point first, before grants.** A grant is a record about a principal, evaluated at a chokepoint. Today there is neither. Landing grants first produces facts that nothing consults — the failure mode `security.rs` already demonstrates (a correct policy with no enforced call site, §2.3). The enforcement point is cheap, schema-free and also closes T-10, the largest live hole. It must be *closed by default* (D080-5: no policy compiled → no data for regulated packs).
2. **Grants before the path rewrite.** D080-1 requires policy to be compiled *into the plan* rather than post-filtered. Compiling requires a policy source. If the rewrite lands first, its input is a Rust literal (today's `PathPolicy`), creating a second source of truth next to `kutha:allows` — exactly the duplication AGENTS.md forbids. With grants on the log, the rewrite's input is a fold cut, and the D080-2 clarification (present authority is checked now, historical cut only for audit) becomes testable.
3. **Capabilities and the path rewrite may interleave; both after grants.** The two answer different questions (D080 "may this hop return?" vs D051 "may this holder invoke *this* operator *now*?"). Ordering constraint: capabilities need a principal (step 1) and a grant record to resolve from (step 2). They do not need the rewrite. Recommended: land capabilities immediately after grants, because the write path (T-01/T-02/T-03/T-12) is where the money is, and the read path (rewrite) is where the *data* is. Both are pre-WASM.
4. **Sandbox last, and only with the executor.** There is no code-execution surface today (T-08), so building WASM now buys nothing and blocks the cheap wins. D081's own open question 3 ("whether P0 forbids UDFs entirely until 081 spikes") should be answered **yes**: no UDF execution until the capability set, the read-only view contract, and fuel accounting exist in the same slice.

**What can ship without WASM at all:** T-01 … T-07, T-09, T-10, T-11, T-12 — i.e. the whole authorization, tenancy, receipt-integrity and quota program. Only T-08 requires a sandbox, and only because it requires an executor.

**Suggested slice shape for the lease** (five slices, each independently verifiable):

- **S1 — Enforcement point + principal.** `Session { principal, policy, rt }`; `Runtime` read/write constructors become `pub(crate)`; closed default; op→authority classification replacing the name list. Closes T-01 (partial), T-10, T-11.
- **S2 — Grants as facts + typed blobs + receipt verification.** Grant/revoke VT×TT facts and their decision-cut evaluation; typed blob keys with a single verifying resolver; `QuantumOutcome` receipt recomputation on open. Closes T-04, T-06, and the record half of T-02/T-03.
- **S3 — Tenant slices + quotas + path-ABAC compiled from grants.** `SliceId` on facts and reads; per-principal quotas; the rewrite consults grants at the invocation cut. Closes T-05, T-07, T-09, and re-opens/resolves T-269-09.
- **S4 — Capabilities at the Port boundary (pre-sandbox).** `Cap<Port>` issuance from grants, argument-provenance binding for authority-bearing arguments (D051-2), and the "no executor without capabilities" gate that makes S5 safe.
- **S5 — Sandbox (081).** Only with an executor; owns T-08.

---

## 5. Acceptance tests

Thirteen runtime tests plus one design/spec test. Each names the defect it catches; `[falsifier]` marks a test that **fails at `d430a14`** (it proves a current defect and must be written before the fix); `[gate]` marks a test that can only pass once the milestone's new surface exists.

| # | Test name (file) | Catches | Marker |
|---|---|---|---|
| 1 | `reader_without_policy_is_denied_by_default` (`crates/kutha-runtime/tests/m190_security_chokepoint.rs`) | T-10: `as_of`/`execute`/CSR reads ignore policy; `security.rs` is opt-in | [falsifier] (today: returns rows) |
| 2 | `emit_define_is_refused_without_dictionary_capability` (`m190_...`) | T-01 variant 1: `Op::Define` reaches the log through `emit` (`pipeline.rs:338-350`) | [falsifier] (today: appends) |
| 3 | `emit_store_blob_and_tt_anchor_require_their_capability` (`m190_...`) | T-01 variant 2: `pipeline.rs:470-472` silently downgrades to Dictionary mode | [falsifier] |
| 4 | `quantum_outcome_receipt_is_reverified_on_open` (`m191_receipt_integrity.rs`) | T-06: edit `receipt_digest_hex` / `event_ids` / `resume_of`, store, reopen → must fail | [falsifier] (today: opens) |
| 5 | `attached_outcomes_cannot_shadow_log_receipts` (`m191_...`) | T-06 aggravating surface: `attach_outcomes` (`quantum.rs:586-589`) | [falsifier] |
| 6 | `tt_anchor_far_future_does_not_freeze_anchor_path` (`m192_quota_bounds.rs`) | T-09 anchor freeze: `wall = u64::MAX` accepted once, all later anchors rejected (`admission.rs:198-207`) | [falsifier] |
| 7 | `enrichment_cache_key_is_scoped_per_pack` (`m193_enrichment_scope.rs`) | T-05: global cache-key set (`enrichment.rs:155-159`) lets one pack suppress another | [falsifier] |
| 8 | `blob_key_kind_is_enforced_at_read` (`m194_blob_typing.rs`) | T-04: a blob accepted via the policy hash must not be readable as a rule body, and vice versa; a `kutha:rule` pin whose bytes encode a *different* canonical rule must fail closed | [falsifier] (today: `from_canonical` accepts, `rule.rs:145-149`) |
| 9 | `policy_body_that_is_not_understood_is_refused` (`m194_...`) | T-04 second concrete hole: any body other than `"deny"` is inert (`admission.rs:70`) | [falsifier] |
| 10 | `grant_revocation_does_not_backdate_access` (`m195_grants_temporal.rs`) | D080-2 clarification: `AS OF` before a revoke must not restore present access; audited cut ≠ invoked cut | [gate] |
| 11 | `operator_name_reuse_after_retire_is_attributable` (`m195_...`) | T-03: retire releases the name (`pack_manifest.rs:263-269`) and a second pack may claim it with no code pin | [gate] |
| 12 | `tenant_slice_excludes_foreign_vertex_by_default` (`m196_tenant_slice.rs`) | T-07: two slices in one fold, closed default returns no foreign rows; enrichment `source_known` scoped | [gate] |
| 13 | `capability_is_unforgeable_and_not_a_role_string` (`m197_capability_port.rs`) | §2.2: `Cap<Port>` cannot be constructed outside the authorizer, is not serializable to the log, and a role string in a fact confers no authority | [gate] |
| 14 | `sandbox_grant_matrix_is_ports_only` (design/spec test, `docs/process/` + a `m198` harness once an executor exists) | T-08 items 1–4: the module sees exactly the granted ports, no mutable fold view, fuel is charged, and its output lands as a proposal only | [gate] (blocked until S5) |

Existing tests that must keep passing (regression fence, do not rewrite): `m182_meta_threats.rs` (all 10 `fn`), `m183_digest_falsifiers.rs` (5 `fn`), `m013_write_path.rs` (4 reserved-name refusals), `m177_pack_manifest.rs::cross_pack_operator_collision_fails_closed` and `::retired_pack_releases_operator_names`, `m185_enrichment_admission.rs::admission_and_replay_agree_on_proposals`, `m175_security_policy.rs` (5), `m175_security_integration.rs` (4), `b8_blob_digest.rs::tampered_storeblob_same_length_fails_open`, `m030_writer_lock.rs::failpoint_append_keeps_lock_and_writer_usable`, `m052_cascade_atomic.rs`.

---

## 6. Non-goals and open questions

### 6.1 Explicit non-goals

1. **Authentication / identity provisioning.** The milestone consumes a principal; how it is proven (mTLS, OIDC, operator config) is out of scope. Without a stated IdP the milestone must accept an operator-supplied principal and say so in every receipt.
2. **A policy language.** No XACML/Rego/DSL. Closed default + relation/slice grants + the existing `"deny"` kill-switch is the full v1 vocabulary. A language is a later cell (080 open question 1).
3. **WASM/CHERI/TEE.** No WAMR, no CHERI, no SGX, no PQC placeholders. D081 open question 1 stays open.
4. **Cryptographic token custody.** D051-3 stands: P0 is an operator allow-list; grants are records, not bearer tokens; no MAC-signed capability envelopes, no key management.
5. **Zanzibar/oxify as SoT.** A grant-graph overlay is allowed as a lease; the log owns truth (D080-2).
6. **Encryption at rest, CMK, air-gap, DP/k-anonymity/STE.** GTM/publish concerns, not this PEP (D080 hard separations).
7. **Multi-writer / cross-host stores.** The advisory lock stays advisory; NFS and multi-process writes stay uncovered and documented (ADR-000 D8.2 pointer).
8. **Deny edges / negative permissions.** Not in v1 (080 open question 3).
9. **Query-result redaction or aggregation.** The rewrite denies paths; it does not aggregate or perturb.
10. **Governor/harness changes.** No new `kutha-gov` check is proposed here; a new CI phase would need a governor-intake row anyway.

### 6.2 Open questions with recommended defaults

| # | Question | Recommended default | Why |
|---|---|---|---|
| Q1 | How is the principal carried? | Thread a `Principal` through the pipeline and record it on the action/admission/grant facts; do **not** add a field to `Op::Assert` until two consumers need it | an `Op` change is a wire migration (the `m062`–`m076` family); grants are meta facts and can carry identity first |
| Q2 | Where is the chokepoint? | A new `Session { rt, principal, policy }`; `Runtime`'s read/write constructors become `pub(crate)` | one enforceable place; the dozen public entry points in §1.3 cannot each be patched credibly |
| Q3 | Slice representation | explicit `SliceId` on every product fact and on the read entry | deriving it from a relation name repeats T-02's name-keyed-authority mistake |
| Q4 | Blob typing | prefixed key inside the existing map (`sha256:`/`rule:`/`policy:`/`manifest:`/`payload:`/`checkpoint:`) plus one verifying resolver | no new storage, no migration of existing bytes (prefix is computed on read) |
| Q5 | Receipt verification cost | recompute on open; if that regresses the open benchmark beyond the 10 % stop line, verify lazily on first `outcome_records()` read and record the choice | correctness first; the gate is measurable (AGENTS.md bench gate) |
| Q6 | Quota ownership | store-level config file, one row per principal, monotone defaults; not env | T-11: environment is authority today and must stop being authority |
| Q7 | Anchor bounds | accept `wall <= now + configured skew`; refuse otherwise with a typed error; keep monotonicity | removes the permanent-freeze primitive without losing the ordering guarantee |
| Q8 | P0 forbids UDFs? | **Yes** — no executor until S4/S5 exist (answers D081 open question 3) | there is no surface to protect yet; building the sandbox first spends the milestone on T-08 while T-06/T-07/T-10 stay open |
| Q9 | Does the write-time relation allowlist survive? | Yes, as the *default policy input* for the read-side rewrite, sourced from the log at the invocation cut — one source of truth | two policy surfaces is the current defect (§2.3) |
| Q10 | Cross-tenant payload de-duplication | keep it, but never expose presence; do not add a "was it already here" return value | content addressing is the only reason blobs are cheap |

### 6.3 Gaps in this report

- No code was executed (no `cargo`, per contract), so every severity is a static judgement, not a measured exploit. The nine `[falsifier]` tests in §5 are the cheapest way to convert them into facts.
- Not reviewed: the harness plane's own trust model (`scripts/kutha_gov`, `docs/process/kutha-harness.md`), the `kutha-migrate` CLI surface, GC/gate assertions (m174/m179 areas), and anything about a future network listener — there is none in the workspace (ADR-000 line 17 records that Tokio is absent).
- The mapping from threat to ADR decision is 1:1 only for T-04…T-07; T-01/T-02/T-10 are *preconditions* of ADR-051 that no ADR currently owns, which is itself a documentation gap the lease should record (051's open question 3 covers "where PACT stops", not "where enforcement lives").

---

## Appendix A — evidence index (all at `d430a14`)

Admission/authority: `crates/kutha-runtime/src/admission.rs:103-231` (`admit`), `:104-111` (`StoreBlob` digest), `:112-126` (meta-lease gate), `:127-154` (enrichment), `:155-179` (pack lifecycle + manifest), `:180-197` (User reserved rejection), `:198-207` (anchor monotonicity), `:209-215` (allowlist + `kutha:` bypass), `:233-237` (`relation_name_reserved`).
Pipeline: `crates/kutha-runtime/src/pipeline.rs:338-361` (`reject_forged_meta`), `:363-430` (`gate_user_op`), `:457-485` (`pipeline_kind`), `:432-454` (receipt buffer/persist), `:519-575` (cascade + budget abort), `:790-905` (dictionary constructors), `:931-934` (`emit`).
Read path: `crates/kutha-runtime/src/quantum.rs:572` (`fold`), `:591`/`:610` (CSR leases), `:747` (`as_of`), `:846` (`live_at`), `:586-589` (`attach_outcomes`); `fold_query.rs:41` (dictionary-rel filter), `:169-183`, `:249`, `:559-584` (`relation_allowed_at`); `query_ir.rs:24-42` (IR), `:300-345` (`prepare`/`execute`).
Policy/path: `crates/kutha-runtime/src/security.rs:16-48`, `:88-127`, `:133-144`; wiring evidence: no non-test call site.
Blobs: `crates/kutha-common/src/event.rs:682-698`; `fold.rs:576-581`, `:600-612`, `:735-745`; `pack_manifest.rs:165-183`; `enrichment.rs:143-152`; `kutha-common/src/rule.rs:145-149`; `eval.rs:191-196`.
Packs: `pack_manifest.rs:156-162` (operator names), `:189-216` (install validation), `:235-269` (`OperatorIndex` declare/retire); `pack_lifecycle.rs:116-168` (`check`), `:171-193` (`replay`); `pack_registry.rs:21-58`.
Enrichment: `enrichment.rs:77-161` (`check_proposal`), `:164-198` (ledger); `hydrate.rs:41-45` (global cache keys).
Receipts: `receipt.rs:18-29`; `pipeline.rs:438-447`; `provenance.rs:47-107` (`replay_check`), `:227-267` (checkpoint receipts only).
Meta leases: `quantum.rs:379`, `:418`; `hydrate.rs:21-70`.
Resources: `quantum.rs:297-303` (`KUTHA_MAX_CASCADE`), `pipeline.rs:550-555`; `quantum.rs:468-490` (`intern`/`intern_silent`).
Bootstrap: `allow.rs:11-59`; `quantum.rs:426-438`; `crates/kutha-runtime/dictionaries/relations.yaml`.
Tenancy/bridge: `tenant.rs:71-141`; `crates/kutha-runtime/src/bin/kutha-tenant.rs:19-20`; `store.rs:35`, `:64-84`.
