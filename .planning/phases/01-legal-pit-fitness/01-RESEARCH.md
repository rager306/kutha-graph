# Phase 1: Legal PIT fitness - Research

**Researched:** 2026-09-29
**Domain:** Brownfield product fitness verification (Rust `cargo test` / FF5–FF6 / M010–M011 / H2–H4)
**Confidence:** HIGH (in-repo evidence); MEDIUM on Cargo Book offline edge cases

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- **D-01:** Phase 1 hard gate is `cargo test --workspace --offline` (same args as harness `observe.py`). — **Reversibility:** reversible — command string only
- **D-02:** Named FIT tests are an **evidence map** in SUMMARY/VERIFICATION (test fn → FIT-id), not a cargo `--test` / `--exact` CLI filter. — **Reversibility:** reversible
- **D-03:** `uv run kutha-gov ci` and `precommit` are **not** Phase 1 must_haves. Full governor quantum belongs to Phase 2 (GOV-*). — **Reversibility:** reversible
- **D-04:** Phase evidence SoT is `*-VERIFICATION.md` / plan SUMMARY with the FIT→test map. Do not require mid-execute edits to REQUIREMENTS checkboxes. — **Reversibility:** reversible
- **D-05:** When Phase 1 verification passes, update REQUIREMENTS.md FIT-01…05 to `[x]` in one batch (verify/complete step), plus STATE progress — not piecemeal during tasks. — **Reversibility:** reversible
- **D-06:** Any failing hard-gate FIT blocks phase completion (hard gate). No “green with caveat” for FIT-01…05. — **Reversibility:** reversible — policy only
- **D-07:** If red: stop, record blocker in STATE with failing test name; do not quarantine or skip without an explicit human lease outside this CONTEXT. — **Reversibility:** reversible
- **D-08:** FIT-05 is satisfied only by existing tests in `crates/kutha-runtime/tests/h2_harness_tenant.rs` and `h4_process_allows.rs` (plus workspace run). No new harness assertions, no ADR-090 ontology, no legal corpus. — **Reversibility:** reversible
- **D-09:** Do not add product-crate features to “help” H2/H4; Active Slice remains None. — **Reversibility:** reversible

### Claude's Discretion
User deferred (“you decide”) on verification surface Q1–Q3 and asked Claude to decide the three follow-on gray areas reasonably. Locked defaults above: cargo-offline hard gate; evidence in VERIFICATION; hard-fail on red FIT; H2/H4 = existing tests only.

### Deferred Ideas (OUT OF SCOPE)
- Full `uv run kutha-gov ci` as hard gate — Phase 2
- REQUIREMENTS checkbox mid-task updates — deferred; batch at verify
- New H2/H4 fixtures or ADR-090 work — freeze / Phase 3 lease
- File-filtered or `--exact` cargo invocation — not chosen; may revisit if workspace runtime becomes painful
- None else — discussion stayed within phase scope
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| FIT-01 | On statute-shaped fixture, `as_of(2015)` ≠ `as_of(2021)` live triples (FF5) | Evidence: `ff5_as_of_t1_differs_from_as_of_t2_on_statute_log` in `ff5_legal_pit.rs`; hard gate includes it via full workspace |
| FIT-02 | Unknown product relation does not append (FF6) | Evidence: `ff6_unknown_relation_does_not_append` in `ff6_allowlist.rs` |
| FIT-03 | Open without `snapshot.json` restores intern meanings; Define-op recovery; live `Op::Define` | Evidence: three fns in `m010_semantic_open.rs` (all in `fsm.yaml` observe_cargo.required) |
| FIT-04 | Claim/support identity; unknown claim / dangling `caused_by` fail-closed; thin P→Q eligibility | Evidence: four fns in `m011_claim_supports.rs` |
| FIT-05 | H2 status AS OF cuts + same-second emitted cut; H4 prior membership after later edition drops it | Evidence: named H2/H4 fns in existing files only (D-08); no new assertions |
</phase_requirements>

## Summary

Phase 1 is **verification-first brownfield**: the steel-thread fitness already lives in `crates/kutha-runtime/tests/`. Planning must wire a hard gate (`cargo test --workspace --offline`), an evidence map (FIT-id → file → fn → pass/fail), and a hard stop on red — **not** new product features, legal packs, Rocks/Cypher/HNSW, or freeze thaw. Harness `observe.py` already defaults to the same cargo args; Phase 1 reuses that command string without requiring full `kutha-gov ci` (Phase 2).

Baseline this session: all six FIT integration binaries are green under cargo; full `cargo test --workspace --offline -q` exited 0 in ~0.5s on a warm target. CBM indexes most FIT symbols; `m011_claim_supports.rs` and `fsm.yaml` report `metadata_changed` — planners must treat graph hits for M011 as approximate and prefer Read/`cargo test -- --list` for the evidence map (especially `derived_q_loses_eligibility_when_last_premise_support_withdrawn`, absent from the current graph).

**Primary recommendation:** Plan a short verify-only wave: run the hard-gate command, fill VERIFICATION evidence table from the map below, hard-stop on any red named FIT, batch REQUIREMENTS checkboxes only at phase complete — zero product/harness code edits unless a FIT is already red (then stop per D-07).

## Architectural Responsibility Map

| Capability | Primary Tier | Secondary Tier | Rationale |
|------------|-------------|----------------|-----------|
| Legal PIT / allowlist / claim / semantic fitness | Database / Storage (in-process Runtime fold+log) | — | Product truth is event log + fold; tests assert cuts, not an HTTP API |
| H2/H4 tenant JSONL AS OF dogfood | Database / Storage (Runtime + ingest) | — | Process facts on product log; not ADR-090 legal pack |
| Hard-gate execution | Developer workstation / CI shell | — | `cargo test --workspace --offline` is the Phase 1 gate |
| FIT evidence map / VERIFICATION | GSD planning artifacts | Harness dictionaries (Phase 2) | D-02/D-04: map lives in SUMMARY/VERIFICATION; governor observe is Phase 2 |
| Freeze / Active Slice None | Process lease (`.kutha/STATE.md`) | — | Blocks product expansion; Phase 1 must not thaw |
| Governor quantum honesty | Harness (`kutha-gov ci`) | — | Explicitly **not** Phase 1 must_have (D-03) |

## Standard Stack

### Core

| Library / Tool | Version | Purpose | Why Standard |
|----------------|---------|---------|--------------|
| `cargo test` | Cargo 1.98.1 / rustc 1.98.1 `[VERIFIED: shell cargo --version]` | Product unit + integration fitness | Workspace SoT for L_capability; observe defaults match |
| Rust edition workspace | `kutha-common`, `kutha-runtime` | Hosts FF5–H4 tests | Already in crates; no new crates |
| Integration tests under `crates/kutha-runtime/tests/*.rs` | (in-tree) | FIT evidence targets | Named fns cited by REQUIREMENTS + `fsm.yaml` |

### Supporting

| Library / Tool | Version | Purpose | When to Use |
|----------------|---------|---------|-------------|
| `uv` / `kutha-gov` | uv 0.12.13 `[VERIFIED: shell uv --version]` | Full FSM observe (Phase 2) | Optional local smoke only in Phase 1; not must_have |
| `.kutha/dictionaries/fsm.yaml` `observe_cargo.required` | (in-tree) | Named evidence list for governor | Align VERIFICATION map; do not edit for Phase 1 success |
| `scripts/kutha_gov/observe.py` | (in-tree) | Parses `test <name> ... ok` | Documents hard-gate arg parity; Phase 2 |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Full workspace hard gate | `cargo test --test ff5_…` filters | Faster, but **false green** risk vs D-01/D-02; deferred |
| Evidence map in VERIFICATION | Mid-task REQUIREMENTS `[x]` | Deferred (D-04/D-05) |
| Phase 1 = `kutha-gov ci` | cargo-only | Deferred to Phase 2 (D-03) |

**Installation:** None — no new crates/PyPI packages for Phase 1.

**Version verification:** `cargo 1.98.1`, `rustc 1.98.1`, `uv 0.12.13` probed this session. Cargo Book documents `--workspace` and `--offline` on `cargo-test` `[CITED: https://doc.rust-lang.org/cargo/commands/cargo-test.html]`.

## Package Legitimacy Audit

> Phase installs **no** external packages.

| Package | Registry | Age | Downloads | Source Repo | Verdict | Disposition |
|---------|----------|-----|-----------|-------------|---------|-------------|
| — | — | — | — | — | N/A | No install |

**Packages removed due to [SLOP] verdict:** none
**Packages flagged as suspicious [SUS]:** none

## Architecture Patterns

### System Architecture Diagram

```text
Developer / executor
    │
    ▼
cargo test --workspace --offline     ◄── Phase 1 HARD GATE (D-01)
    │
    ├─► kutha-common unit tests
    └─► kutha-runtime
          ├─ unit #[cfg(test)]
          └─ integration tests/*.rs
                ├─ ff5_legal_pit.rs      → FIT-01
                ├─ ff6_allowlist.rs      → FIT-02
                ├─ m010_semantic_open.rs → FIT-03
                ├─ m011_claim_supports.rs→ FIT-04
                ├─ h2_harness_tenant.rs  → FIT-05 (H2)
                └─ h4_process_allows.rs  → FIT-05 (H4)
    │
    ▼
Exit 0?
    │ yes                         │ no
    ▼                             ▼
Fill VERIFICATION evidence map   STOP (D-06/D-07):
FIT-id | file | fn | pass         record failing fn in STATE
    │
    ▼
Phase complete → batch REQUIREMENTS FIT-01…05 [x] (D-05)

(Out of Phase 1 must_have — Phase 2:)
uv run kutha-gov ci → observe.py interpret_cargo_output(required[])
```

### Recommended Project Structure (unchanged)

```text
crates/kutha-runtime/tests/
├── ff5_legal_pit.rs
├── ff6_allowlist.rs
├── m010_semantic_open.rs
├── m011_claim_supports.rs
├── h2_harness_tenant.rs
├── h4_process_allows.rs
└── fixtures/
    ├── legal_pit.md              # docs of statute shape — not loaded by test
    └── harness_process.jsonl     # H2 ingest fixture
.planning/phases/01-legal-pit-fitness/
├── 01-CONTEXT.md
├── 01-RESEARCH.md                # this file
└── 01-VERIFICATION.md            # to be written at verify (evidence SoT)
```

### Pattern 1: Evidence map (not CLI filter)

**What:** Record each FIT against concrete `fn` names; run full workspace; mark pass/fail in VERIFICATION.
**When to use:** Always for Phase 1 (D-02).
**Example evidence rows (canonical names):**

| FIT | File | Test fn |
|-----|------|---------|
| FIT-01 | `crates/kutha-runtime/tests/ff5_legal_pit.rs` | `ff5_as_of_t1_differs_from_as_of_t2_on_statute_log` |
| FIT-02 | `crates/kutha-runtime/tests/ff6_allowlist.rs` | `ff6_unknown_relation_does_not_append` |
| FIT-03 | `…/m010_semantic_open.rs` | `open_without_snapshot_recovers_intern_meanings` |
| FIT-03 | `…/m010_semantic_open.rs` | `open_without_snapshot_or_terms_file_recovers_from_define_ops` |
| FIT-03 | `…/m010_semantic_open.rs` | `intern_appends_define_for_new_terms_only` |
| FIT-04 | `…/m011_claim_supports.rs` | `retracting_one_support_leaves_claim_supported` |
| FIT-04 | `…/m011_claim_supports.rs` | `unknown_claim_does_not_append` |
| FIT-04 | `…/m011_claim_supports.rs` | `replay_rejects_behavior_without_prior_cause` |
| FIT-04 | `…/m011_claim_supports.rs` | `derived_q_loses_eligibility_when_last_premise_support_withdrawn` |
| FIT-05 | `…/h2_harness_tenant.rs` | `h2_harness_status_as_of_t1_differs_from_as_of_t2` |
| FIT-05 | `…/h2_harness_tenant.rs` | `h2_same_second_status_as_of_uses_emitted_cut` |
| FIT-05 | `…/h4_process_allows.rs` | `h4_prior_cut_keeps_status_membership_after_later_edition_drops_it` |

These twelve names are exactly the Phase 1 evidence set. They also appear in `.kutha/dictionaries/fsm.yaml` `states.observe_cargo.required` `[VERIFIED: .kutha/dictionaries/fsm.yaml:31-43]` — quote:

```text
required:
  - ff5_as_of_t1_differs_from_as_of_t2_on_statute_log
  - ff6_unknown_relation_does_not_append
  - h2_harness_status_as_of_t1_differs_from_as_of_t2
  - h2_same_second_status_as_of_uses_emitted_cut
  - h4_prior_cut_keeps_status_membership_after_later_edition_drops_it
  - open_without_snapshot_recovers_intern_meanings
  - open_without_snapshot_or_terms_file_recovers_from_define_ops
  - intern_appends_define_for_new_terms_only
  - retracting_one_support_leaves_claim_supported
  - unknown_claim_does_not_append
  - replay_rejects_behavior_without_prior_cause
  - derived_q_loses_eligibility_when_last_premise_support_withdrawn
```

Extra tests in the same files (`ff3_csr_lease_drop_…`, `ff6_allowlisted_…`, `h2_unmapped_…`, `h2_tenant_persist_…`, `h4_same_second_…`) ride along under the workspace gate but are **not** separate FIT checkboxes.

### Pattern 2: Hard gate arg parity with observe

**What:** Use the same args observe defaults to.
**When to use:** Phase 1 must_have command.
**Verified default** `[VERIFIED: scripts/kutha_gov/observe.py:133-141]`:

```python
default_args=["test", "--workspace", "--offline"],
```

FSM also hardcodes `[VERIFIED: .kutha/dictionaries/fsm.yaml:23-26]`:

```yaml
args: ["test", "--workspace", "--offline"]
```

### Anti-Patterns to Avoid

- **Treating filtered cargo as the gate:** `--test ff5_legal_pit` or name substrings ≠ D-01 workspace observation `[CITED: cargo-test book — --test selects integration target; TESTNAME filters libtest]`.
- **“Green with caveat”:** Forbidden for FIT-01…05 (D-06).
- **Implementing to “fix” fitness:** Active Slice is None; no product features (D-09); if red → stop (D-07).
- **Starting legal pack / Rocks / Cypher / HNSW** to “strengthen” PIT: freeze + out of scope.
- **Collapsing L_capability into governor green:** Phase 1 proves cargo fitness; Phase 2 owns CI honesty.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Phase 1 fitness proof | New legal corpus / ADR-090 pack | Existing FF5/H2/H4 tests | D-08; freeze |
| Named FIT selection | Custom cargo `--exact` wrapper | Evidence map + full workspace | D-02 |
| Observe parity | Reimplement cargo parser | Existing `observe.py` (Phase 2) | Already interprets required names |
| Requirement checkbox hygiene | Mid-task REQUIREMENTS edits | Batch at verify (D-04/D-05) | Avoid false progress |
| Red FIT | Quarantine/`#[ignore]` | Stop + STATE blocker (D-07) | Hard gate integrity |

**Key insight:** Phase 1 value is **honest observation of existing falsifiers**, not new capability.

## Common Pitfalls

### Pitfall 1: False green — workspace ok, FIT deleted/renamed

**What goes wrong:** `cargo test --workspace --offline` exits 0 after a FIT fn is renamed/removed; remaining tests still pass.
**Why it happens:** Cargo does not know about `fsm.yaml` required names; Phase 1 skips `kutha-gov ci` (D-03).
**How to avoid:** VERIFICATION must include the twelve-row evidence map; confirm each name appears in `cargo test --workspace --offline -- --list` and was not `FAILED` / filtered out. Static bridges (`observe-required-fn`, `m011-claim-supports`, …) catch renames under precommit/`ci` — Phase 2 — so Phase 1 must not rely on them alone.
**Warning signs:** Evidence map row missing from `--list`; CBM `search_graph` miss for a name that still exists in source (stale index).

### Pitfall 2: False green — filtered smoke mistaken for gate

**What goes wrong:** Executor runs `cargo test --test m011_claim_supports` only, marks Phase 1 done.
**Why it happens:** Faster iteration; book documents `--test` as valid target selection `[CITED: doc.rust-lang.org/cargo/commands/cargo-test.html]`.
**How to avoid:** Plans must list the **exact** hard-gate string `cargo test --workspace --offline` as must_have; filtered runs are optional diagnostics only.
**Warning signs:** Plan tasks that use `--test` / name filters as acceptance.

### Pitfall 3: Rename breakage across three planes

**What goes wrong:** Rename `fn ff5_…` without updating `fsm.yaml` required, `bridges.yaml`/`checks.yaml` needles (`fn …`), and GSD evidence map.
**Why it happens:** Three consumers of the same string: Rust tests, harness dictionaries, GSD FIT text.
**How to avoid:** Phase 1 plans should **forbid** renames. If unavoidable, treat as multi-plane change + Phase 2 observe alignment — out of Phase 1 success path.
**Warning signs:** `observe-missing` HIGH under `ci`; `yaml_needles_in_glob` / `file_contains` check fails on `fn ` prefix.

### Pitfall 4: Freeze thaw under “verification”

**What goes wrong:** Adding RocksDB, Cypher, HNSW, ADR-050 dicts, or legal corpus “to make PIT real.”
**Why it happens:** Confusing statute-shaped **fixture** (`legal_pit.md` docs + in-code Asserts) with ADR-090 pack.
**How to avoid:** Cite `.kutha/STATE.md` freeze; FF5 builds fixture in-process (`T_OLD=2015`, `T_NEW=2021`) — `legal_pit.md` is documentation only `[VERIFIED: crates/kutha-runtime/tests/ff5_legal_pit.rs:7-11]` / fixture file header.
**Warning signs:** New crate deps; new `dictionaries/`; corpus loaders.

### Pitfall 5: `--offline` network surprise

**What goes wrong:** Cold machine / missing registry cache → cargo fails with network blocked.
**Why it happens:** `--offline` “Prevents Cargo from accessing the network” `[CITED: cargo-test Manifest Options --offline]`.
**How to avoid:** Ensure deps already fetched (`cargo fetch` once online) before gate; do not drop `--offline` from the hard gate to “make it work” without recording environment blocker.
**Warning signs:** Exit nonzero with offline/index errors before any test runs.

### Pitfall 6: CBM stale M011 symbol

**What goes wrong:** Planner assumes `derived_q_…` does not exist because `search_graph` returns 0.
**Why it happens:** `check_index_coverage` on `m011_claim_supports.rs` → `freshness: metadata_changed` this session; graph incomplete.
**How to avoid:** Evidence from Read + `cargo test -- --list`; do **not** call `index_repository` from a research/plan child unless parent leases it.
**Warning signs:** Graph miss + source/`--list` hit.

## Code Examples

### Hard gate + list named FIT fns

```bash
# Phase 1 hard gate (D-01) — must_have
cargo test --workspace --offline

# Diagnostic: prove evidence-map names are registered (not a substitute for the gate)
cargo test --workspace --offline -- --list | rg \
  'ff5_as_of_t1_differs_from_as_of_t2_on_statute_log|ff6_unknown_relation_does_not_append|open_without_snapshot_recovers_intern_meanings|open_without_snapshot_or_terms_file_recovers_from_define_ops|intern_appends_define_for_new_terms_only|retracting_one_support_leaves_claim_supported|unknown_claim_does_not_append|replay_rejects_behavior_without_prior_cause|derived_q_loses_eligibility_when_last_premise_support_withdrawn|h2_harness_status_as_of_t1_differs_from_as_of_t2|h2_same_second_status_as_of_uses_emitted_cut|h4_prior_cut_keeps_status_membership_after_later_edition_drops_it'
```

### FIT-01 invariant (verbatim excerpt)

`[VERIFIED: crates/kutha-runtime/tests/ff5_legal_pit.rs:7-11,42-47]`:

```rust
const T_OLD: u64 = 2015;
const T_NEW: u64 = 2021;
// ...
let old = rt.fold().as_of(T_OLD);
let new = rt.fold().as_of(T_NEW);
assert_ne!(
    old, new,
    "AS OF T1 must differ from AS OF T2 on the same log"
);
```

### FIT-02 fail-closed (verbatim excerpt)

`[VERIFIED: crates/kutha-runtime/tests/ff6_allowlist.rs:27-31]`:

```rust
assert!(
    matches!(err, RuntimeError::UnknownRelation { ref name } if name == "notALegalRelation"),
    "{err:?}"
);
assert_eq!(n, rt.log().len(), "fail-closed: log must not grow");
```

### Observe required-name matching (Phase 2 context)

`[VERIFIED: scripts/kutha_gov/observe.py:16-42]`:

```python
ok_pat = re.compile(rf"^test (?:[\w:]+::)?{escaped} \.\.\. ok\b", re.MULTILINE)
fail_pat = re.compile(rf"^test (?:[\w:]+::)?{escaped} \.\.\. FAILED\b", re.MULTILINE)
```

Rename without updating `required` → `observe-missing` under `ci`, while cargo alone can still be green.

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| “Build legal pack to prove PIT” | Statute-shaped in-crate FF5 fixture | Spike / FF5 | Phase 1 verifies fixture, does not start ADR-090 |
| Governor CI as sole capability proof | Separate L_capability (cargo fitness) vs L_map / L_delivery | Harness H4 | Phase 1 = capability evidence; Phase 2 = CI honesty |
| File-filtered cargo as acceptance | Full `--workspace --offline` + evidence map | CONTEXT 2026-09-29 | Prevents false green from partial runs |

**Deprecated/outdated for this phase:**
- Using `uv run kutha-gov ci` as Phase 1 must_have (deferred D-03)
- Mid-execute REQUIREMENTS checkbox flips (D-04)

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | Warm-target ~0.5s workspace runtime stays acceptable without `--test` filters | Pitfalls / Alternatives | If cold CI is painfully slow, may revisit deferred filter — needs human lease |
| A2 | No doctest/network dep will newly break `--offline` before Phase 1 executes | Environment | Gate fails for env reasons; record as blocker not FIT red |

**If empty rows needed:** Core FIT↔fn mapping claims are `[VERIFIED]` from source Read this session — not assumed.

## Open Questions

1. **Should Phase 1 VERIFICATION also paste `cargo test -- --list` excerpts?**
   - What we know: D-02 wants FIT→fn map with pass/fail.
   - What's unclear: Whether list output is required artifact or optional.
   - Recommendation: Require pass/fail column; optional `--list` snippet if any map name is disputed.

2. **If hard gate is green but a bridge needle is already stale (pre-existing)?**
   - What we know: Phase 1 does not must_have precommit/`ci`.
   - What's unclear: Whether discoverable dictionary drift blocks Phase 1.
   - Recommendation: Note drift in VERIFICATION notes; do not expand Phase 1 into GOV-*; fix under Phase 2 unless it proves a FIT fn is truly missing.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| `cargo` / `rustc` | Hard gate | ✓ | 1.98.1 | — |
| `Cargo.lock` + local registry cache | `--offline` | ✓ (lock present; warm target) | — | `cargo fetch` once online, then re-run offline |
| `uv` / `kutha-gov` | Optional smoke | ✓ | uv 0.12.13 | Not required Phase 1 |
| CBM index | Structural search | ✓ (stale on m011/fsm) | indexed_at 2026-09-16 | Read source; parent may reindex later |
| Rocks/Cypher/HNSW | — | N/A frozen | — | Must not install |

**Missing dependencies with no fallback:** none for Phase 1 path.

**Missing dependencies with fallback:** none blocking.

Step 2.6: env probed — cargo/uv present.

## Validation Architecture

### Test Framework

| Property | Value |
|----------|-------|
| Framework | Rust libtest via `cargo test` (edition 2021 workspace) |
| Config file | per-crate `Cargo.toml`; no custom harness crate |
| Quick run command | `cargo test --workspace --offline -q` |
| Full suite command | `cargo test --workspace --offline` |

### Phase Requirements → Test Map

| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| FIT-01 | as_of 2015 ≠ 2021 on statute log | integration | hard gate (workspace); evidence fn `ff5_as_of_t1_differs_from_as_of_t2_on_statute_log` | ✅ |
| FIT-02 | unknown relation no append | integration | hard gate; `ff6_unknown_relation_does_not_append` | ✅ |
| FIT-03 | snapshot-less open + Define recovery | integration | hard gate; three m010 fns | ✅ |
| FIT-04 | claim supports / lineage / P→Q | integration | hard gate; four m011 fns | ✅ |
| FIT-05 | H2/H4 AS OF dogfood | integration | hard gate; three named H2/H4 fns | ✅ |

### Sampling Rate

- **Per task commit:** `cargo test --workspace --offline -q` (if any commit occurs; Phase 1 may be docs-only)
- **Per wave merge:** `cargo test --workspace --offline`
- **Phase gate:** Full hard gate green + completed evidence map before `/gsd-verify-work`

### Wave 0 Gaps

None — existing integration tests cover FIT-01…05. No new test files required unless a FIT is red (then stop per D-07; do not invent replacement tests without lease).

## Security Domain

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|------------------|
| V2 Authentication | no | N/A — no auth surface in Phase 1 |
| V3 Session Management | no | N/A |
| V4 Access Control | no | N/A — single-process tests |
| V5 Input Validation | yes (narrow) | Harness JSONL ingest fail-closed on unmapped relations (`h2_unmapped_…`); product allowlist FF6 |
| V6 Cryptography | no | N/A — no new crypto |

### Known Threat Patterns for fitness verification

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Silent skip of required fitness | Tampering / repudiation of evidence | Evidence map + full workspace; Phase 2 observe-required |
| Freeze bypass via “test helper” deps | Elevation of privilege (process) | STATE freeze; refuse Rocks/Cypher/HNSW/legal pack |
| Quarantine/`#[ignore]` to force green | Tampering | D-06/D-07 hard stop |

## Project Constraints (from .cursor/rules/)

| Rule | Directive for Phase 1 plans |
|------|-----------------------------|
| `code-graph-cbm.mdc` | Prefer CBM for symbols; Grep for literals/YAML; never `delete_project`; never CBM `manage_adr`; `index_repository` only parent if missing/stale/asked; graph coverage ≠ governor green |
| `quiet-no-lifeos.mdc` | Do not invoke LifeOS/PAI/notify |
| `ce-skills-ru.mdc` | Russian chat routing only — not a Phase 1 product constraint |
| AGENTS.md (via always_applied) | Active Slice None; freeze until M002 lease; two planes; no Rocks/Cypher/HNSW/legal pack; chat RU / docs EN |

## Sources

### Primary (HIGH confidence)

- `crates/kutha-runtime/tests/ff5_legal_pit.rs` — FIT-01 fn + T_OLD/T_NEW
- `crates/kutha-runtime/tests/ff6_allowlist.rs` — FIT-02
- `crates/kutha-runtime/tests/m010_semantic_open.rs` — FIT-03 trio
- `crates/kutha-runtime/tests/m011_claim_supports.rs` — FIT-04 quartet (Read; CBM stale)
- `crates/kutha-runtime/tests/h2_harness_tenant.rs`, `h4_process_allows.rs` — FIT-05
- `.kutha/dictionaries/fsm.yaml:23-43` — observe args + required names
- `scripts/kutha_gov/observe.py:133-141`, `16-42` — default_args + interpret patterns
- `.kutha/dictionaries/checks.yaml` — `observe-required-fn`, `m010-semantic-open`, `m011-claim-supports`, `h4-membership-as-of` needles
- `.planning/phases/01-legal-pit-fitness/01-CONTEXT.md` — locked D-01…D-09
- CBM `list_projects` / `search_graph` / `check_index_coverage` (caveat: m011 + fsm `metadata_changed`)
- Shell: `cargo test --workspace --offline` green; FIT binaries green

### Secondary (MEDIUM confidence)

- [Cargo Book `cargo-test`](https://doc.rust-lang.org/cargo/commands/cargo-test.html) — `--workspace`, `--offline`, `--test`, filters `[CITED]`
- `.planning/codebase/TESTING.md` — tooling gate matrix

### Tertiary (LOW confidence)

- Cold-CI duration / offline cache miss behavior on other machines `[ASSUMED]` A1/A2

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — in-repo tools version-probed; no new packages
- Architecture: HIGH — verification flow constrained by CONTEXT; tests mapped to FIT
- Pitfalls: HIGH — false green / rename / freeze derived from observe + dictionaries + CONTEXT

**Research date:** 2026-09-29
**Valid until:** 2026-10-29 (stable brownfield; re-verify if FIT fns rename or STATE leases a slice)
