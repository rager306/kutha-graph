# Rust review: pack lifecycle, manifest, registry, enrichment, and admission (kutha-runtime)

Static, read-only advisory review of the newest pack/enrichment stack in
`crates/kutha-runtime`: `pack_lifecycle.rs`, `pack_manifest.rs`, `pack_registry.rs`,
`enrichment.rs`, `admission.rs`, the `Pack*` / `EnrichmentProposal` variants and their
digest handling in `crates/kutha-common/src/event.rs`, and `security.rs`. The code is a
**D-01 meta plane**: pack install/activate/retire and enrichment proposals are log events,
validated by a fail-closed admission gate and re-validated by replay, with read models
(lifecycle, operator registry, enrichment ledger) derived from the log and never entering
the graph fold. `kutha-runtime` is an **internal library** (`publish = false` in the
workspace `Cargo.toml`, edition 2024, `rust-version = 1.99`, research stage), so this
review applies **simplicity mode**, not public-API ceremony.

**Verdict:** the stack is disciplined. It has no `unsafe`, no reachable panic, no
production `unwrap`, typed fail-closed error enums with value-carrying variants, and it
deliberately shares one lifecycle rule between admission and replay. The most important
defect is not a crash: it is a **trust-flag asymmetry** — `hydrate_from_log` turns three
degraded conditions into `meta_leases_trusted = false`, while the incremental commit path
(`pipeline.rs:304-320`) silently discards the same three results, so the same log can
produce a fail-closed runtime on open and a fail-open one in a live process. Clippy,
rustfmt, and the test suite were **not** run: the task contract forbids a workspace build
(disk/time). `lsp diagnostics` was run and returned 0 diagnostics for
`pack_manifest.rs`, `enrichment.rs`, and `admission.rs`. Commands and exit statuses are in
§ Command log.

Priority order below is blocking correctness, then API design, then idiom, then docs.

## Should fix

### 1. `meta_leases_trusted` is not maintained on the commit path, so a live runtime can be more trusting than the same log on open (`crates/kutha-runtime/src/pipeline.rs:304-320`)

**Rule:** "Make invalid states unrepresentable. Push invariants into types: enums instead
of bool flags and 'must be X if Y' comments" (BeautifulRust, § 1 Mindset) — here the pair
`(lifecycle, meta_leases_trusted)` is a "must be consistent" invariant held by hand.

**Problem:** `hydrate.rs:29-71` treats three degraded results as trust signals: a failed
`lifecycle.replay` (`hydrate.rs:34-36`), a failed `declared_operators_for` (the `Err(_)`
arm at `hydrate.rs:61`), and a failed `operators.declare` (`hydrate.rs:57-59`) all set
`trusted = false`, which `admission.rs:112-126` then uses to reject **every** pack write.
`commit_event` performs the same three operations and ignores all three results
(`pipeline.rs:305` `let _ = self.lifecycle.replay(...)`, `314-318`
`if let Ok(names) = ... { let _ = self.operators.declare(...) }`). If any of them ever
fails at commit time, the event is appended anyway (`self.append_indexed(event)` at
`pipeline.rs:323`), the lease stays short, and `meta_leases_trusted` stays `true` — so a
later pack install is admitted against a lease that a fresh `open()` of the identical log
would refuse to trust.
The three `commit_event` call sites are admit-gated today (`pipeline.rs:453, 501, 562`),
so this is latent rather than live; `commit_event` is `pub(crate)` and is already called
directly from `quantum_invariants.rs:619-620`, which is exactly the kind of caller that
can feed it an unvalidated op.

**Fix** (minimal, mirrors the existing hydrate decisions):

```rust
Op::PackInstall { .. } | Op::PackActivate { .. } | Op::PackRetire { .. } => {
    if self.lifecycle.replay(&event.op).is_err() {
        // Same signal hydrate derives from a full rebuild.
        self.meta_leases_trusted = false;
    }
    if let Op::PackRetire { pack_id, .. } = &event.op {
        self.operators.retire(pack_id);
    } else if let Op::PackInstall { pack_id, manifest_digest: Some(_), .. } = &event.op {
        match crate::pack_manifest::declared_operators_for(&self.fold.blobs, &event.op) {
            Ok(names) => {
                if self.operators.declare(pack_id, names).is_err() {
                    self.meta_leases_trusted = false;
                }
            }
            Err(_) => self.meta_leases_trusted = false,
        }
    }
}
```

A structural alternative, if the owner wants the invariant enforced by the compiler: move
the flag into the tracker (`PackLease { tracker: PackLifecycleTracker, trusted: bool }`
with `fn is_trusted(&self)`), so the two cannot be updated independently.

**Scope:** commit-path bookkeeping only. No event-log field, no fold input, no admission
rule changes; `meta_leases_trusted = false` can only reject more writes, never fewer.

### 2. The lifecycle state machine exists twice, and the read model can contradict the lease for the same log (`crates/kutha-runtime/src/pack_lifecycle.rs:37-86` vs `104-193`)

**Rule:** one rule, one implementation — the module states the contract itself:
"Keeping one implementation prevents the two gates from drifting apart"
(`pack_lifecycle.rs:88-92`); BeautifulRust § 11 rejects "mirrored branches", and the same
argument applies to production code.

**Problem:** `derive_pack_lifecycle` (`37-86`) re-implements the transitions by hand and
never validates them, while `PackLifecycleTracker::check`/`replay` (`116-193`) is the
validating implementation. `derive_pack_lifecycle` is what `Runtime::pack_lifecycle()`
(`pipeline.rs:586-589`), `derive_pack_operators` (`pack_registry.rs:25`), and
`derive_enrichment_ledger` (`enrichment.rs:165`) all report from. They agree on every log
admission can produce today, and diverge on any log replay rejects. Concrete case —
`[PackInstall(p, "A", "1"), PackActivate(p, "1"), PackInstall(p, "B", "2")]`: the tracker
rejects the third event (`pack_id_already_known`) and hydrate marks the lease untrusted
while keeping `Active v1`; `derive_pack_lifecycle` lets `records.insert` overwrite the row
and reports `Installed v2` (`pack_lifecycle.rs:49-63`). So for one and the same log,
"what the lease says" (`Active v1`) and "what the read model says" (`Installed v2`) differ,
and `derive_pack_operators` publishes rows for **both** installs, each carrying the one
overwritten state (`pack_registry.rs:41-53`).

**Fix** (project the read model through the tracker instead of re-deciding transitions;
needs an index from pack id to row so it stays linear):

```rust
pub fn derive_pack_lifecycle(events: &[Event]) -> Vec<PackLifecycle> {
    let mut tracker = PackLifecycleTracker::new();
    let mut rows: Vec<PackLifecycle> = Vec::new();
    let mut at: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
    for event in events {
        // Tolerant replay: an invalid transition leaves the row at the last
        // state the same check admission applies.
        if tracker.replay(&event.op).is_err() {
            continue;
        }
        match &event.op {
            Op::PackInstall { pack_id, name, version, .. } => {
                at.insert(pack_id.clone(), rows.len());
                rows.push(PackLifecycle {
                    pack_id: pack_id.clone(),
                    name: name.clone(),
                    version: version.clone(),
                    state: PackLifecycleState::Installed,
                    event_ids: vec![event.id],
                });
            }
            Op::PackActivate { pack_id, .. } | Op::PackRetire { pack_id, .. } => {
                let Some(&i) = at.get(pack_id) else { continue };
                let row = &mut rows[i];
                row.version = tracker.version(pack_id).unwrap_or_default().to_owned();
                row.state = tracker.state(pack_id).unwrap_or(PackLifecycleState::Installed);
                row.event_ids.push(event.id);
            }
            _ => {}
        }
    }
    rows
}
```

**Scope:** read-model only. `pack_lifecycle()` output changes **only** for logs that
`replay_check` already rejects; the fold, the log, and every admitted log are unaffected.

### 3. A `StoreBlob` digest mismatch is reported as `UnknownRuleVersion` (`crates/kutha-runtime/src/admission.rs:104-110`)

**Rule:** "Two sound shapes: an enum with one variant per cause ... Either way, don't make
one crate-wide enum that lists every failure mode in the crate" (BeautifulRust, § 5
Errors and panics).

**Problem:** `UnknownRuleVersion { pin }` is documented at `quantum.rs` as "User Behavior
pin is not a live registry hash (RULE-02 / RULE-03)", but `admit` returns it when
`blob_digest_matches` fails for a `StoreBlob` (`105-109`). A caller that stored a blob with
a wrong digest gets `unknown rule version <64-hex>` — no mention of a blob, and callers
cannot separate the two causes. The tests already pay for this: `m182_meta_threats.rs:81`
and `:256` have to write `Err(RuntimeError::UnknownRuleVersion { .. }) | Err(...)` with a
comment explaining that "the generic StoreBlob digest check runs before meta validation";
`b8_blob_digest.rs:91` pins the wrong variant outright.

**Fix:**

```rust
// quantum.rs, in RuntimeError, next to UnknownPolicyVersion
    /// `StoreBlob` bytes do not hash to the pinned digest (D-02).
    BlobDigestRejected { digest: String },
// admission.rs
        if let Op::StoreBlob { digest, bytes } = op {
            if !blob_digest_matches(digest, bytes) {
                return Err(RuntimeError::BlobDigestRejected { digest: digest.clone() });
            }
            return Ok(());
        }
```

Five assertions move: `b8_blob_digest.rs:91`, `b8_blob_digest_kill.rs:87`, and
`m016_s02_kill.rs:19` (`store_blob_json_with_named_pin_does_not_append`) all exercise a
`StoreBlob` input, and the two disjunctions at `m182_meta_threats.rs:81,256` can then drop
their `UnknownRuleVersion` arm. Every genuine rule-pin site (`m016_rule_ast.rs:15,20`) is
unchanged.

**Scope:** `RuntimeError` is not serialized (no `serde` derive on the enum) and its
`Display` is diagnostics-only, so this is not a wire-format change and does not touch the
fold. It does change a public error contract inside the workspace — hence "should fix",
owner decides.

### 4. `assert_payload_matches` re-implements `Op`'s derived `PartialEq` and will drift (`crates/kutha-runtime/src/admission.rs:273-308`)

**Rule:** "Derive whenever every field supports it: `Debug` ... `PartialEq` / `Eq`"
(BeautifulRust, § 4 Derive and implement the standard traits eagerly).

**Problem:** `Op` derives `PartialEq` (`event.rs:164`), yet `assert_payload_matches`
spells out all eight `Op::Assert` fields by hand (`276-305`). Today it is exact. The risk
is the next field: a new `Assert` field that is not added here silently makes two
different payloads compare equal, and `delivery_key_retry` (`313-361`) would then return
the **original** event id and receipt for a payload that differs — a silent data drop,
not a fail-closed error. That is the one place in this stack where drift fails *open*.

**Fix:**

```rust
    fn assert_payload_matches(left: &Op, right: &Op) -> bool {
        matches!(left, Op::Assert { .. }) && left == right
    }
```

**Scope:** behavior-identical today, no wire or fold effect; removes ~35 lines.

### 5. The "one shared proposal check" is shared, but its glue is written twice (`crates/kutha-runtime/src/enrichment.rs:72-83`, `admission.rs:136-153`, `pipeline.rs:646-666`)

**Rule:** the module documents the contract: "One shared proposal check used by admission
and replay. ... so both gates apply the same rules without re-walking the log"
(`enrichment.rs:72-76`).

**Problem:** `check_proposal` is shared, but everything around it is copy-pasted: the
pack-state lookup with `unwrap_or("")` and a `to_string()` allocation
(`admission.rs:136-139`, `pipeline.rs:651-654`), the `source_known` closure
(`admission.rs:142`, `pipeline.rs:655`), the payload lookup, the call, and the
`map_err(|err| RuntimeError::EnrichmentRejected { reason: err.to_string() })`. A fix to
the version lookup or the payload source applied in one gate silently diverges the other —
which is precisely the drift the doc claims is prevented. `pipeline.rs:647-650` also needs
`let ... else { unreachable!() }` only because it must re-destructure the op it just built.

**Fix** (one helper on `Runtime`; both call sites become one line, and the `unreachable!`
disappears):

```rust
// admission.rs, in impl Runtime
pub(crate) fn check_enrichment(
    &self,
    op: &Op,
    payload: Option<&[u8]>,
) -> Result<(), crate::enrichment::EnrichmentError> {
    let Op::EnrichmentProposal { pack_id, blob_digest, .. } = op else {
        return Ok(());
    };
    let state = self
        .lifecycle
        .state(pack_id)
        .map(|s| (s, self.lifecycle.version(pack_id).unwrap_or("")));
    crate::enrichment::check_proposal(
        op,
        state,
        |id| self.log_ids.contains(&id),
        payload.or_else(|| self.fold.blobs.get(blob_digest).map(Vec::as_slice)),
        &self.enrichment_cache_keys,
    )
}
```

(the `Option<(state, &str)>` and `impl Fn` follow from findings 7 and the `String` clone
noted here; keep `map_err(... EnrichmentRejected)` at the two call sites.)

**Scope:** glue only; identical rules, identical rejection reasons.

### 6. A rejected pack install leaves an orphan `StoreBlob` in the log, one per retry (`crates/kutha-runtime/src/pipeline.rs:702-737`)

**Rule:** the sibling path states the pattern: "Validate before the blob append so a
rejected proposal leaves no orphan `StoreBlob` behind" (`pipeline.rs:643-645`).

**Problem:** `emit_pack_install_with_manifest` validates the manifest shape/identity, then
**commits** the manifest blob (`716-724`, `pipeline_kind` → `pipeline_events` →
`commit_event`), and only afterwards gates the `PackInstall` (`729-737`). So
`emit_pack_install_with_manifest` with an already-known `pack_id`, or with an operator
name owned by another pack (`m177_pack_manifest.rs:116-125`), returns `Err` while the blob
is already on the log. Admission has no dedupe for `StoreBlob` (`admission.rs:104-111`), so
a client retry loop appends one identical blob event — and one `tt` — per attempt, and the
log is the SoT with no vacuum for meta rows.

**Fix** (validate the cheap, non-blob-dependent failures before the append; keep the
post-blob admission check as the authority):

```rust
let bytes = manifest.canonical_bytes();
let digest = kutha_common::sha256_hex(&bytes);
let install = Op::PackInstall { pack_id, name, version, manifest_digest: Some(digest.clone()) };
// Reject the cheap failures before the blob append: same rules admission applies.
self.lifecycle.check(&install)?;
if let Op::PackInstall { pack_id, .. } = &install {
    for operator in &manifest.operators {
        if self.operators.owner_of(operator).is_some_and(|owner| owner != pack_id) {
            return Err(RuntimeError::PackLifecycleRejected {
                pack_id: pack_id.clone(),
                reason: format!("manifest: operator {operator:?} is already declared by another pack"),
            });
        }
    }
}
self.pipeline_kind(Op::StoreBlob { digest, bytes }, ReceiptMode::Dictionary)?;
self.pipeline_kind(install, ReceiptMode::Dictionary)
```

**Scope:** no new rejection rule — it moves the same rejections ahead of the append. The
log stays byte-identical for accepted installs. (A rejected install now also does not
consume a `tt`, which is a behavior improvement, not a determinism change.)

## Consider

### 7. `check_proposal` takes `&mut dyn FnMut` for a predicate it never mutates (`crates/kutha-runtime/src/enrichment.rs:77-83`)

**Rule:** "Closures: use the most general bound that works (`FnOnce` if called once,
`FnMut` if it mutates, else `Fn`)" (BeautifulRust, § 4 Traits and generics).

**Problem:** `source_known: &mut dyn FnMut(EventId) -> bool` is called but never mutated,
forces both call sites to declare `let mut` (`admission.rs:142`, `pipeline.rs:655`), and
reads like the function has an effect on the caller. The `dyn` also defeats inlining of
`HashSet::contains` on the admission hot path for no code-size benefit that matters here.

**Fix:** `source_known: impl Fn(EventId) -> bool`, and drop `mut` at both call sites (see
finding 5's sketch).

**Scope:** `pub(crate)`, so nothing outside the crate is affected.

### 8. `PackManifestError::Malformed` throws away the serde error (`crates/kutha-runtime/src/pack_manifest.rs:104-109`)

**Rule:** "Errors that lose information: `map_err(|_| ...)`, no `source()`, or messages
without the path, line, or value that failed" (BeautifulRust, § 5 Errors and panics).

**Problem:** a manifest is JSON that a human writes by hand. `map_err(|_| Malformed)`
(`106`) discards line/column, and `Display` then says only "pack manifest is not valid
JSON". Combined with the missing `deny_unknown_fields` (see § Out of scope, item C), a
typo is hard to diagnose from the runtime error alone.

**Fix:**

```rust
    Malformed { detail: String },
...
        serde_json::from_slice(bytes).map_err(|e| PackManifestError::Malformed { detail: e.to_string() })?;
...
            Self::Malformed { detail } => write!(f, "pack manifest is not valid JSON: {detail}"),
```

`m177_pack_manifest.rs:109-112` needs `Malformed { .. }`.

**Scope:** diagnostics of a rejected input; no accepted byte changes.

### 9. `EnrichmentError::EmptyField` conflates "empty" and "padded", while the sibling gate distinguishes them (`crates/kutha-runtime/src/enrichment.rs:33,104`)

**Rule:** "An enum with one variant per cause" (BeautifulRust, § 5 Errors and panics).

**Problem:** `enrichment.rs:104` rejects `value.trim().is_empty() || value != value.trim()`
and always reports `EmptyField { field }`, whose `Display` is "enrichment {field} is
empty". A `pack_id` of `" core "` is rejected with a message saying it is empty, which
sends the reader looking for the wrong bug. `pack_lifecycle.rs:126-129` performs the same
two checks and correctly separates `empty_identifier` from `padded_identifier`; two gates
for the same fields should not have different vocabularies.

**Fix:**

```rust
    PaddedField { field: &'static str },
...
        if value.trim().is_empty() {
            return Err(EnrichmentError::EmptyField { field });
        }
        if value != value.trim() {
            return Err(EnrichmentError::PaddedField { field });
        }
```

**Scope:** message quality only; same accept/reject set.

### 10. One needless `String` per write, and one allocation forced by the map shape (`crates/kutha-runtime/src/admission.rs:212`, `329-332`)

**Rule:** "No `.clone()` whose only job is to satisfy the borrow checker" (BeautifulRust,
§ 13 Final checklist).

**Problem:** `admit` runs for every write. `let name = self.dict.lookup(rel).unwrap_or("").to_string();`
(`212`) allocates a `String` only to compare it, and the empty case then has to be
special-cased at `221`. `delivery_key_retry` builds `key.to_string()` (`331`) only to index
`by_delivery_key: IngestMap<(TermId, String), EventId>`; `get` cannot borrow a `(TermId,
&str)` tuple from an owned key, so that allocation is *forced by the map shape*, not by
the borrow checker — worth stating so a later reader does not "optimize" it into a broken
borrow.

**Fix** for the first one (second is a note, not a change):

```rust
        let Some(name) = self.dict.lookup(rel) else {
            return Err(RuntimeError::UnknownRelation { name: format!("#{rel}") });
        };
        if name.starts_with("kutha:") {
            return Ok(());
        }
        if self.fold.relation_allowed_at(name, tt, vt) {
            return Ok(());
        }
        Err(RuntimeError::UnknownRelation { name: name.to_owned() })
```

If the second allocation ever matters, the clean shape is a nested
`IngestMap<TermId, IngestMap<String, EventId>>` (borrow-lookup per level); that is a
bigger change than this finding justifies today.

**Scope:** behavior-identical; no measurement was taken, so this is an allocation count,
not a wall-clock claim.

### 11. Path-policy errors name a relation by raw `TermId` and copy the name per hop (`crates/kutha-runtime/src/security.rs:111-117`)

**Rule:** "Errors that lose information: ... messages without the ... value that failed"
(BeautifulRust, § 5) plus the crate's own convention: unknown relation ids are printed as
`#<id>` (`admission.rs:226`).

**Problem:** when the dictionary has no name, the error reports
`name: rel.to_string()` (`113-115`), so `Display` says "unknown relation 7 in path policy"
in a codebase where `7` is a `TermId` and every other site writes `#7`. Second, `117`
(`let name = name.to_string();`) is a needless copy: `relation_allowed(&str)` and
`format!` both borrow, and the `rt` borrow does not conflict with anything.

**Fix:**

```rust
            for rel in rels {
                let Some(name) = rt.dictionary().lookup(*rel) else {
                    return Err(SecurityError::UnknownRelation { name: format!("#{rel}") });
                };
                if !policy.relation_allowed(name) {
                    return Ok(AuthorizedQuery::Denied {
                        reason: format!("relation {name} is not allowed"),
                    });
                }
            }
```

(Test `m175_security_policy.rs:152` asserts a resolvable name, so it is unaffected.)

**Scope:** message text only; no policy decision changes.

### 12. Stale doc on `check_pack_lifecycle_live` describes a replay it does not do (`crates/kutha-runtime/src/admission.rs:239-250`)

**Rule:** "`//` comments explain why, not what: ... Document the end state"
(BeautifulRust, § 10 Documentation and comments).

**Problem:** two doc blocks were merged and the first sentence is now false:
"Replay D021 lifecycle facts from the existing log and validate the next transition"
(`239-240`) — the function is a one-line delegation to `self.lifecycle.check(next)`
(`248-250`) and replays nothing. The last paragraph ("O(1) in log length ...", `246-247`)
is the accurate one. The wrapper itself also adds nothing but a doc site.

**Fix:** keep one block and, if the wrapper is kept for the doc, say what it is:

```rust
    /// Validate one lifecycle op against the lease built on hydrate and updated on
    /// commit. O(1) in log length: a write does not replay the log.
    fn check_pack_lifecycle_live(&self, next: &Op) -> Result<(), RuntimeError> {
```
(or delete the wrapper and call `self.lifecycle.check(op)` at `159`).

**Scope:** docs only.

## Rejected as ceremony

These were considered against the rules and **not** reported as findings; the rule that
made them cost more than they are worth is given for each.

- **`PackManifest::new(pack_id, name, version, operators)` with three `impl Into<String>`
  and four loose arguments** (§ 4 "A constructor with four or more loose arguments should
  group them into semantic types"). One production call shape, one crate, `publish = false`
  — a builder or a `PackIdentity` struct is ceremony here.
- **`#[non_exhaustive]` on `PackManifestError`, `EnrichmentError`, `PackLifecycleState`,
  `PackOperator`, `EnrichmentRecord`** (§ 4 "Mark public enums ... `#[non_exhaustive]`
  unless the set is a stable ontology"). No consumer outside the workspace; it would only
  add wildcard arms to every test match. Explicitly excluded by the Allspark simplicity
  policy for internal crates.
- **Private fields + accessors on `PackManifest`, `PackOperator`, `EnrichmentRecord`,
  `PackLifecycle`** (§ 4 "Fields stay private unless the struct is passive C-style data").
  They *are* passive data rows; accessors would be pure ceremony.
- **`PackLifecycleState::as_str(&self)` → `as_str(self)`** (clippy
  `trivially_copy_pass_by_ref`, pedantic only). The lint is not enabled, and `&self` is
  the conventional accessor shape.
- **`derive_pack_operators`' `(pack_id.clone(), operator.clone())` seen-key clones**
  (§ 3 "Don't clone to make the borrow checker stop complaining"). A `BTreeSet<(&str,
  &str)>` cannot outlive the `out.push` that moves `operator`, so the clone is the honest
  fix; a borrowed-key set would not compile.
- **`OperatorIndex::declare(impl IntoIterator<Item = String>)` → `&[String]`**. It moves
  names into owned sets; the generic is both cheaper and more permissive.
- **`compile_path_policy(rt, q: &QueryIr, policy)` → `q: QueryIr`** (§ 3 "Take ownership
  only when you need it"). It would remove `q.clone()` at `security.rs:96,124`, but all
  seven call sites in `m175_security_policy.rs` and `execute_authorized` reuse `q` after
  the call, so ownership would just move the clone into every caller.
- **`PartialOrd`/`Ord` on `PackOperator`/`EnrichmentRecord`** (§ 4). No total order is
  used; the vector order *is* the log order, and making it `Ord` invites sorting it.
- **`#[must_use]` on `derive_pack_lifecycle`, `derive_enrichment_ledger`,
  `PackManifest::digest`**. Every call site consumes the value; the attribute is noise.
- **`PackLifecycleTracker::check` followed by `replay`** looks like check-then-unwrap, but
  it is deliberate: admission must validate without mutating, and `replay` re-runs the
  same check before applying. Keep.
- **`admission.rs:127-135` `matches!` + `unreachable!("guarded by matches!")`** — not
  reported separately because it disappears in finding 5's fix; the panic is proven
  unreachable by the preceding guard and carries a message, which the rules allow.

## Out of scope by contract (report only, no recommendation)

- **The admission policy body is allow-by-default for anything but the exact token
  `deny`** (`admission.rs:70`: `from_utf8(bytes).is_ok_and(|s| s.trim() == "deny")`). A
  whitespace-only or misspelled policy body silently *allows*. Changing the default to
  deny-unless-"allow" changes ADM-02 product semantics → needs an ADR. Not recommended
  here.
- **`PackInstall`/`PackActivate`/`PackRetire` have no retry key** (unlike `Op::Assert`'s
  `delivery_key`), so an idempotent client retry fails with `pack_id_already_known` /
  `activate_requires_same_installed_version` (`pack_lifecycle.rs:131-133, 144-163`).
  Adding one changes which logs are admitted → owner decision, not a review fix.
- **`#[serde(deny_unknown_fields)]` on `PackManifest`** would catch a typo'd
  `"operators"` key (today `#[serde(default)]` turns it into zero operators and the
  manifest still validates), but it would also make **already-logged** manifests with an
  extra field fail `parse` during replay → replay-compat break, needs an ADR.
- **Any change to the `Op::Pack*` / `Op::EnrichmentProposal` fields, to `PackManifest`'s
  canonical field order (it *is* the digest pre-image, `pack_manifest.rs:93-101`), or to
  `Op::digest_bytes` encoding:** wire format / address scheme. Out of scope.
- **Making pack state a graph `Fact` or moving the lifecycle check into the fold:** D1–D10
  (event log = SoT, graph = deterministic fold of graph ops; pack rows are meta-only).
  Out of scope.

## What's already good

- **Fail-closed typed errors with useful payloads.** `PackManifestError` and
  `EnrichmentError` each carry the failing field, pack, digest, or operator name, and both
  implement `Display` + `std::error::Error` (`pack_manifest.rs:26-74`,
  `enrichment.rs:26-70`). In the six reviewed files there is no `unsafe` and no production
  `unwrap`; the only `expect` is `pack_manifest.rs:95` with a proof ("PackManifest is
  always serializable"), and the two `unreachable!` (`admission.rs:134`,
  `pipeline.rs:649`) each sit behind a `matches!` that proves the arm. (`pipeline.rs`'s two
  other `expect`s, at `451` and `537`, are receipt/rollback code outside this review.)
- **Admission/replay parity is designed rather than incidental.** One tracker with
  `check` then `replay` (`pack_lifecycle.rs:112-193`) serves both gates; `check_proposal`
  receives the caller's leases so replay and admission cannot apply different rules
  (`enrichment.rs:72-83`); the blob-before-install ordering is documented at the point
  where it matters (`pipeline.rs:643-645`).
- **Bounded, allocation-light validation.** `valid_operator_name` is a byte scan with
  `kutha:` reserved (`pack_manifest.rs:156-162`), `validate_shape` uses a borrowed-key
  `BTreeSet` (`118-131`), and `OperatorIndex` replaces a per-install scan of every
  operator set with one lookup while documenting the complexity (`219-283`).
- **Deterministic addressing and ordering.** `canonical_bytes` pins serde field order,
  `load_manifest` re-hashes on read so one blob cannot be read under two identities
  (`pack_manifest.rs:174-183`), and both read models walk the log rather than a `HashMap`,
  so iteration order never reaches output.

## Command log

Static review only: no `cargo build`, no `cargo clippy`, no `cargo fmt`, no `cargo test`
(contract: disk/time), and no repository writes other than this report.

| # | Command | Exit |
|---|---------|------|
| 1 | `df -h /root` | 0 |
| 2 | `ls -la /root/kutha-graph/.compound-engineering/artifacts/reviews/` | 0 |
| 3 | `rg -n "check_proposal\|derive_pack_lifecycle\|from_log\|enrichment_cache_keys\|meta_leases_trusted\|operators\b\|lifecycle\b" --glob '!target' crates/ scripts/` | 0 |
| 4 | `rg -n "log_ids" --glob '!target' -g 'crates/kutha-runtime/src/**'` | 0 |
| 5 | `rg -n "commit_event\|fn append_indexed\|fn rebuild_log_indexes\|by_delivery_key" --glob '!target' crates/kutha-runtime/src` | 0 |
| 6 | `rg -n "UnknownRuleVersion" --glob '!target' crates/` | 0 |
| 7 | `rg -n "pack_lifecycle\(\)\|derive_pack_lifecycle\|Malformed\|unreachable!\|meta_leases_trusted" crates/kutha-runtime/tests/{m176,m177,m180,m182,m184}*` | 0 |
| 8 | `rg -n "unsafe\|unwrap(\|expect(\|panic!\|unreachable!\|todo!\|unimplemented!" <six target files + hydrate.rs + pipeline.rs>` | 0 (0 `unsafe`, 0 `unwrap(`, 3 `expect`, 2 `unreachable!`) |
| 9 | `rg -n "compile_path_policy\|execute_authorized\|QueryIr::Hops" --glob '!target' crates/` | 0 |
| 10 | `rg -n 'format!\("#\{rel\}"\)\|#\{rel\}' --glob '!target' crates/kutha-runtime/src` | 0 |
| 11 | `git status --porcelain` (clean before this review; afterwards only untracked review reports, this one and a concurrent sibling task's), `git log --oneline -3` (`6be40f2`), `cat VERSION` (`0.0.01`) | 0 |
| 12 | `lsp diagnostics` on `pack_manifest.rs`, `enrichment.rs`, `admission.rs` | 0 diagnostics each |

Read-only evidence: `crates/kutha-runtime/Cargo.toml` (`publish.workspace = true`),
root `Cargo.toml:8-11` (`edition = "2024"`, `rust-version = "1.99"`,
`publish = false`).
