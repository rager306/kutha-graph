# Phase 8: End-to-end candidate fixture - Research

**Researched:** 2026-09-30
**Domain:** Kutha semantic-contract composition (supports, conflict report, justification sidecar, reconstruct vs incremental)
**Confidence:** HIGH (in-repo fold/store/quantum APIs, GATE trio, D-O1 sidecar); MEDIUM (exact `Justification` field set — locked below as recommendation)

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

- **D-F1:** Ship **one shared fixture builder** plus **three named oracles** (FIX-01 / FIX-02 / FIX-03) in a new integration file (suggested `crates/kutha-runtime/tests/m011_e2e_fixture.rs`). The builder encodes the semantic-contract `a`/`b`/`P`/`Q` narrative once (t1 → t2 interval-patch on `a` → t3 withdraw `b`; optional conflict branch with `P'=not-P`). Prefer **glue over reimplementation**: call existing APIs (`claim_supported_at`, `live_support_count`, `derivation_eligible_at`, `CorrectInterval`, `typed_csr_lease_at` / untyped CSR drop-rebuild, `provenance_fingerprint` / `rule_version`, persist/open) rather than copying prior test bodies. Do **not** invent a second SoT. Do **not** require re-running every prior `m011_*` file as the S08 gate — prior suites stay regression; S08 owns the composed narrative. — **Reversibility:** reversible — test layout

- **D-F2:** Persist **justification / admission citation** rows as an **authoritative durable sidecar** beside the log (same plane pattern as `quantum_outcomes.jsonl` — write on `store::persist`, load on `store::open`; **not** a droppable lease; discarding `snapshot.json` must not erase cites). Working names: file `justifications.jsonl`, row type `Justification` / `AdmissionCite`, API `record_justification` / `check_admission` (planner may refine). Each row must cite at least: derived/target claim or action id, **source claim_ids / event ids**, **`rule_version`**, and the **(TT, VT) cut** used. Do **not** add a fold-projected graph Fact for “summary text”. Do **not** put cites only in RAM or only in CSR. **Stale cannot renew admission:** `check_admission` (or equivalent) **fails closed** when cited supports are no longer live, `derivation_eligible_at` is false, or cited `rule_version` no longer matches the live Behavior pin — caller must **mint a new justification after reevaluation**; replaying an old row must not grant admission. Thin admission only — **no** ADR-051/080 ABAC pack. — **Reversibility:** costly — once store directories carry the sidecar, open/persist contracts depend on it (same class as D-O1)

- **D-F3:** **Preserve both sides** as independent supports/claims (no implicit supersession). Conflict variant: after `a` proposes `P'=not-P` (or opposite object) while `b` still supports `P` on the overlapping VT, both remain distinguishable at the named cut. Expose a **thin evidence/conflict report** for the fixture (working name `conflict_report_at` / evidence summary returning positive vs negative support sets, aligned with ADR-013’s proposed `(positive_supports, negative_supports)` view) — **report**, do not pick a winner. Do **not** install four-valued logic, ADR-093 ontology, or admission policy dictionaries. — **Reversibility:** reversible — query/report API; fold already keeps independent supports

- **D-F4:** At each named cut used by the fixture, assert agreement on: live **values** (`as_of` / `live_at` as appropriate), **active supports** (`claim_supported_at` / counts), **derivation eligibility** for `Q`, and **completeness** for this exact contract (no silent hole). Paths to compare: (1) incremental Runtime after the narrative mutations; (2) clean reconstruct via `GraphFold::replay(log)` and/or `store::persist` → `open` **without** snapshot; (3) after **discarding** CSR leases (typed + untyped) and rebuilding — answers unchanged. Do **not** require incremental IVM (DBSP) or GraphBLAS. Do **not** treat approximate HNSW quality as in-scope. — **Reversibility:** reversible — oracle scope

- **D-F5:** Inherit Phase 2 **D-G1…D-G3** and **D-10 / D-11 / D-15**, and Phase 4–7 wave-close: every execute wave runs `uv run kutha-gov ci` (HIGH stops) + `explain trajectory` + D-10 Trajectory in SUMMARY; cargo on crate-touching waves. — **Reversibility:** reversible

- **D-F6:** Named cargo tests registered in governor (FSM observe + check/bridge needles), GATE-01, `ci` HIGH 0. Active Slice remains **S08** for the phase (GATE-02 — primary owner). Freeze / Proposed honeycomb (GATE-03 — primary owner); do **not** mark honeycomb cells Accepted at close. Do not edit `.kutha/STATE.md` during delivery. — **Reversibility:** reversible

- **D-F7:** Do not change Assert/Retract/Correct/CorrectInterval fold semantics, quantum outcome disposition rules, typed/untyped CSR build rules, or `replay_check` / `provenance_fingerprint` mix — S08 **composes** them. — **Reversibility:** reversible

### Claude's Discretion

User said «реши эти вопросы разумно и при необходимости доисследуй» and required **codebase-memory-mcp** during execution → D-F1…D-F4 (plus D-F5…D-F7). Planner/researcher/executor MUST use CBM (`list_projects` first; `search_graph` / `trace_path` / `get_code_snippet`; `check_index_coverage` on touched paths — note `quantum.rs` / `event.rs` may show `metadata_changed` until parent reindexes; `detect_changes` after edits). Exact sidecar field names, conflict report shape, and fixture constants are discretionary; must not thaw freeze, Accept ADRs, or expand into Rocks/Cypher/HNSW/ABAC packs.

### Deferred Ideas (OUT OF SCOPE)

- Full ADR-051/080 ABAC / multi-tenant admission packs
- Four-valued logic kernel / ADR-093 ontology
- Execution replay / CA-cache (ADR-060 obligation 3)
- Provenance polynomials
- Vacuum + legal hold (ADR-012 product vacuum)
- Journey/waiting, n-ary incidence, shared CSR cost accounting (semantic-contract probes beyond S08 stop condition)
- Marking honeycomb cells Accepted (explicit milestone review only)
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| FIX-01 | Independent supports and last-support withdrawal hold at named (TT,VT) cuts; conflict variant preserves both sides and reports conflict instead of superseding | Shared builder: two Asserts share `claim_id` for `P` (S03 glue); t2 `Op::CorrectInterval` on `a`; t3 retract `b`. `conflict_report_at` partitions live Facts by interned `P` vs `not-P`. Do **not** change CorrectInterval claim_id copy (D-F7). Polarity-aware “last positive support gone” uses `live_supports` filtered by object `P` — see Pattern 4 |
| FIX-02 | Summary/action records cite exact source revisions and rule version; changed dependency forces reevaluation; stale cached output cannot renew admission | Sidecar `justifications.jsonl` (D-F2 / D-O1 clone): `record_justification` + `check_admission` fail-closed on dead `source_fact_seqs`, polarity/eligibility, or `rule_version` ≠ live `Op::Behavior.rule_version`. Persist/open like `OUTCOMES_REL`. No fold Fact for summary text |
| FIX-03 | Incremental maintenance and clean reconstruction agree on values, supports, completeness; discarding CSR and snapshots changes no answer | Compare incremental Runtime vs `GraphFold::replay` vs `persist`→delete `snapshot.json`→`open`; drop/rebuild `csr_lease_at` + `typed_csr_lease_at`. Answers = fold `live_at`/`as_of`, support counts, eligibility helper, `conflict_report_at`, justification rows |
| GATE-01 | Named cargo tests registered in governor; `uv run kutha-gov ci` stays 0 HIGH | Copy Phase 7 trio: `fsm.yaml` `observe_cargo.required` + `checks.yaml` `m011-e2e` + `bridges.yaml` `B-m011-e2e`; docs-coupling CHANGELOG (D-F5/D-F6) |
| GATE-02 | Execute only while Active Slice is S08 | `.kutha/STATE.md` already names **Active Slice: S08** — executable; do not edit STATE / do not check ROADMAP S08 during delivery |
| GATE-03 | Freeze unstarted; honeycomb stays Proposed | No Rocks/Cypher/HNSW/ADR-050 six dicts; ADR-013/011/012/040 `map: Proposed`; evidence list append only |
</phase_requirements>

## Summary

Phase 8 delivers M011 **S08** (Active Slice leased): **one composed candidate fixture** that makes semantic-contract observations 1–5 executable at named (TT, VT) cuts — independent supports + conflict report (FIX-01), durable justification cites that cannot renew stale admission (FIX-02), and incremental vs reconstruct agreement after discarding CSR/snapshot leases (FIX-03). Named cargo tests + governor registration (GATE-01). GATE-02 / GATE-03 are the close-discipline owners.

Prior slices already ship the pieces: S03 `claim_supported_at` / `derivation_eligible_at`, S04 `CorrectInterval` residuals, S05 `quantum_outcomes.jsonl`, S06 typed/untyped CSR drop-rebuild, S07 `rule_version` + `provenance_fingerprint`. S08 **must not reimplement** those tests; it glues them into one narrative. `derivation_eligible_at` keys on premise **claim_id liveness**, not object polarity. `CorrectInterval` **copies `claim_id`** onto prefix/replacement/suffix rows. After t2 replaces `a`’s object with `not-P`, the premise claim still has a live Fact — so `derivation_eligible_at` can stay true while **positive** support for `P` is gone. The fixture oracle therefore reports conflict by object and treats “Q loses eligibility through this derivation” as **empty positive-P supports**, without rewriting the S03 function (D-F7).

**Primary recommendation:** Add `justifications.jsonl` + `Justification` buffer on `Runtime` (clone the outcomes persist/open/attach path); add `conflict_report_at` returning `(positive_supports, negative_supports)` fact seqs; add `check_admission` / `record_justification`; ship `tests/m011_e2e_fixture.rs` with one builder and the three D-F1 names; register GATE-01 as `m011-e2e` / `B-m011-e2e`. No new crates. Do not edit `.kutha/STATE.md`. Do not flip honeycomb `map: Accepted`.

## Architectural Responsibility Map

| Capability | Primary Tier | Secondary Tier | Rationale |
|------------|-------------|----------------|-----------|
| Event log SoT (`Assert` / `CorrectInterval` / `Behavior`) | Database / Storage (`EventLog`) | — | D-F7: compose existing ops; no second SoT |
| State picture (`GraphFold` live Facts) | API / Backend (fold) | — | Supports, residuals, `as_of` / `live_at` |
| Thin P→Q (`derivation_eligible_at`) | API / Backend (`quantum.rs`) | — | Call as glue; do not change mix (D-F7) |
| Conflict evidence (`conflict_report_at`) | API / Backend | Test oracles | D-F3 report, not a winner / not four-valued logic |
| Justification sidecar | Database / Storage (`justifications.jsonl`) | RAM buffer on `Runtime` | D-F2: same class as D-O1 outcomes; not a lease |
| Admission check | API / Backend (`check_admission`) | — | Fail-closed Result; no ABAC pack |
| Snapshot / CSR | CDN / Static (droppable leases) | — | FIX-03: discard must not change fold answers |
| Quantum outcomes sidecar | API / Backend | — | Untouched (D-F7); pattern only |
| Named FIX oracles | Test runner (integration) | — | S08 gate; prior `m011_*` stay regression |
| GATE-01 registration | Harness dictionaries (YAML) | — | Observe names; bridge cites product tests |
| Freeze / honeycomb map | Harness + ADR status | — | GATE-02/03; map stays Proposed |

## Project Constraints (from .cursor/rules/)

- Chat with the user is Russian; this RESEARCH.md and all planning artifacts stay English.
- Do **not** load LifeOS / PAI / notify endpoints (`quiet-no-lifeos`).
- Structural search: `codebase-memory-mcp`; `list_projects` first; graph before Grep for symbols; `check_index_coverage` on cited paths; `detect_changes` after edits. **Do not** call `index_repository` (unless missing/stale/user-asked), `delete_project`, or `manage_adr` (`code-graph-cbm.mdc`).
- CBM this session: project `kutha-graph` present (`nodes=17834`, `index_mode: full`, `indexed_at: 2026-09-30T01:45:29Z`). Coverage: `fold.rs` / `store.rs` / `csr.rs` / `lib.rs` / `m011_partial_correction.rs` / `m011_quantum_outcome.rs` / `m011_typed_csr.rs` → `freshness: metadata_match`. `quantum.rs` / `event.rs` / `m011_claim_supports.rs` → `metadata_changed` (Read source; CBM `get_code_snippet` for `derivation_eligible_at` returned **stale** `provenance_fingerprint` body). `m011_provenance.rs` → `not_tracked`. Parent reindexes only if asked. Discrete values below cite **Read** line ranges.
- Graphify overlay is **disabled** (`gsd_run graphify status` → `"disabled": true`) — no `graphify query`.
- GitNexus is secondary — not queried for this phase.
- CE skill routing for Russian verbs stays on `.cursor/rules/ce-skills-ru.mdc` (коммит → `ce-commit`, changelog → `kutha-changelog`) at execute time — not a research deliverable.
- Product plane: `crates/kutha-*`. Harness: `scripts/kutha_gov`, `.kutha/`. No Python in `kutha-runtime`. No repo-root hexagon folders.
- Freeze until STATE names M002: no RocksDB, Cypher/GPML, HNSW, ADR-050 six dictionaries, legal pack.
- Honeycomb cells stay **Proposed**. Governor green ≠ ADR Accepted ≠ `L_capability`.
- New governor check = YAML row. Bridge cites product tests. Do not add `scripts/kutha_gov/checks/*.py`.
- `docs-coupling`: crate diffs and harness dictionary diffs need `CHANGELOG.md` (`kutha-changelog` skill). Crate → `### Product`; `.kutha/dictionaries` → `### Process`; honeycomb evidence append → `### Trajectory`.
- Do **not** edit `.kutha/STATE.md` during S08 delivery. ROADMAP S08 checkbox stays unchecked while leased.
- Active Slice **S08**; `L_delivery=M011-S07-done`. [VERIFIED: `.kutha/STATE.md:8-16`]

## Standard Stack

### Core

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| Rust edition | `2021` (workspace) | Language | Existing workspace |
| `kutha-common` | `0.0.0` path | `Event`, `Op::{Assert,CorrectInterval,Behavior}` | Log SoT; `rule_version` already on Behavior |
| `kutha-runtime` | `0.0.0` path | fold, quantum, store, CSR | Sole implementation surface |
| `serde` / `serde_json` | workspace `1` | JSONL sidecars | Same loop as `write_outcomes` |
| `sha2` | workspace `0.10` (lock `0.10.9`) | Existing fingerprints | Do not mix justification rows into `provenance_fingerprint` (D-F7) |
| `uuid` | workspace `1` (v7/std/serde) | `EventId` / claim ids | Cite existing ids; do not mint new Behavior ids to “refresh” admission |

### Supporting

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| cargo / uv / kutha-gov | local toolchain | Tests + governor | GATE-01 / D-F5 |
| `temp_dir` fixtures | std | persist/open oracles | Copy `m011_quantum_outcome.rs` / `m010_semantic_open.rs` |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| `justifications.jsonl` sidecar | Fold Fact / new `Op` for summary text | Forbidden by D-F2 |
| Cites only in `Snapshot` / CSR | Durable sidecar | Forbidden — leases are droppable (FIX-03) |
| Change `derivation_eligible_at` to object polarity | Additive `conflict_report_at` + fixture filter | Forbidden by D-F7 |
| Four-valued logic / ADR-093 | Thin `(positive, negative)` report | Forbidden by D-F3 |
| Re-run all `m011_*` as the S08 gate | Three named e2e oracles | Forbidden by D-F1 |
| ABAC / ADR-080 admission pack | `check_admission` Result | Frozen / deferred |
| DBSP IVM / GraphBLAS | `GraphFold::replay` + persist/open | Forbidden by D-F4 |
| Merge cites into `replay_check` | Separate RAM buffer + sidecar | Would conflate ADR-060 obligation 1 with FIX-02 |

**Installation:**

```bash
# No new crates. Workspace members stay kutha-common + kutha-runtime only.
cargo test -p kutha-runtime --offline --test m011_e2e_fixture
uv run kutha-gov ci
```

**Version verification:** Workspace `Cargo.toml` members `kutha-common`, `kutha-runtime`; `edition = "2021"`; `sha2 = "0.10"`. Lockfile `name = "sha2"` / `version = "0.10.9"`. No `cargo search` / PyPI installs. [VERIFIED: Cargo.toml:1-14] [VERIFIED: Cargo.lock:244-245]

## Package Legitimacy Audit

> Phase installs **no** external packages. Legitimacy gate N/A for new names. `gsd_run query package-legitimacy check` requires at least one package argument — not invoked.

| Package | Registry | Age | Downloads | Source Repo | Verdict | Disposition |
|---------|----------|-----|-----------|-------------|---------|-------------|
| — | — | — | — | — | N/A | No installs |

**Packages removed due to [SLOP] verdict:** none
**Packages flagged as suspicious [SUS]:** none

*Do not add ABAC, polynomial, Rocks, Cypher, or HNSW crates in this phase.*

## Architecture Patterns

### System Architecture Diagram

```text
  intern a,b,P,Q,relatedTo,true,not-P
              │
              ▼
  t1 Assert a→P (claim None) + Assert b→P (claim Some)
  t1 Behavior derive_pq caused_by claim_p, rule_version "r1"
              │
              ▼
  record_justification (target=Q, source fact_seqs, rule_version, cut)
              │
              ├──────────────────────────────────────────┐
              ▼                                          ▼
     EventLog (SoT)                          justifications.jsonl
     GraphFold::apply                        (D-F2 sidecar, not lease)
              │                                          │
              ▼                                          ▼
  t2 CorrectInterval on a's fact_seq         check_admission(row)
     object = not-P | P-prime                fail-closed if fact_seq dead
  t3 Retract b's fact_seq                    or rule_version ≠ live pin
              │
              ▼
  conflict_report_at(claim_p, tt, vt, P, not-P)
     positive_supports | negative_supports   (report only)
              │
              ▼
  FIX-03: incremental fold
       vs GraphFold::replay(log)
       vs persist → rm snapshot.json → open
       vs drop typed+untyped CSR → rebuild
     answers must match
```

### Recommended Project Structure

```text
crates/kutha-runtime/
├── src/
│   ├── quantum.rs           # + Justification buffer, record_justification,
│   │                        #   check_admission, conflict_report_at,
│   │                        #   AdmissionDenied; constructors init empty vec
│   ├── store.rs             # + JUSTIFICATIONS_REL; persist/open/load/write
│   ├── fold.rs              # unchanged apply (D-F7); conflict helper may live here
│   ├── csr.rs               # unchanged
│   └── lib.rs               # re-export Justification, ConflictReport
└── tests/
    └── m011_e2e_fixture.rs  # builder + three named oracles

.kutha/dictionaries/
├── fsm.yaml                 # + three observe_cargo.required names
├── checks.yaml              # + m011-e2e
├── bridges.yaml             # + B-m011-e2e
└── honeycomb.yaml           # ADR-013/011/012/040 evidence append; map Proposed
```

### Pattern 1: Outcomes sidecar clone (D-F2 / D-O1)

**What:** Authoritative JSONL beside the log; written by `persist`; loaded on every `open` path; missing file = empty vec; not stored in `Snapshot`.
**When to use:** Justifications. Do not invent a second persist pipeline.

```rust
// Source: crates/kutha-runtime/src/store.rs:10-31 [VERIFIED]
/// Authoritative quantum outcome sidecar (not a droppable lease — D-O1 / ADR-010).
pub const OUTCOMES_REL: &str = "quantum_outcomes.jsonl";
// ...
    write_outcomes(dir, runtime.outcome_records())?;
    Ok(())
```

```rust
// Source: crates/kutha-runtime/src/store.rs:54-63 [VERIFIED]
    let outcomes = load_outcomes(&dir.join(OUTCOMES_REL))?;
    if snap_path.exists() {
        // ...
        rt.attach_outcomes(outcomes);
        return Ok(rt);
    }
```

```rust
// Source: crates/kutha-runtime/src/store.rs:133-147 [VERIFIED]
fn load_outcomes(path: &Path) -> std::io::Result<Vec<PersistedQuantumOutcome>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    // serde_json::from_str per non-empty line
}
```

Recommended twins (names locked for planner):

```rust
pub const JUSTIFICATIONS_REL: &str = "justifications.jsonl";
// persist: write_justifications after write_outcomes
// open: load_justifications; attach_justifications on all four return paths
// missing file → Vec::new() (fail-closed empty cites, never invent admission)
```

Do **not** add fields to `Snapshot`. [VERIFIED: crates/kutha-runtime/src/snapshot.rs:6-14]

```rust
// Source: crates/kutha-runtime/src/snapshot.rs:6-14 [VERIFIED]
pub struct Snapshot {
    pub log_offset: usize,
    pub next_tt: u64,
    pub max_cascade: usize,
    pub fold: GraphFold,
    pub dict_strings: Vec<String>,
    pub knows: TermId,
    pub known_by: TermId,
}
```

### Pattern 2: RAM buffer + explicit record API (clone outcomes)

**What:** `Runtime` holds `outcomes: Vec<PersistedQuantumOutcome>`; `record_resume` is explicit; `open` never auto-resumes.
**When to use:** Justifications must be explicit `record_justification`, never inferred from emit Ok.

```rust
// Source: crates/kutha-runtime/src/quantum.rs:124-135 [VERIFIED]
pub struct Runtime {
    log: EventLog,
    fold: GraphFold,
    dict: TermDictionary,
    next_tt: u64,
    pub max_cascade: usize,
    knows: TermId,
    known_by: TermId,
    allowed: HashSet<String>,
    /// In-memory buffer of durable quantum outcomes (SoT is `quantum_outcomes.jsonl`).
    outcomes: Vec<PersistedQuantumOutcome>,
}
```

Constructors that today set `outcomes: Vec::new()` and **must** also set `justifications: Vec::new()`: `Runtime::new`, `from_snapshot`, `from_dict_and_events`, `fork_at`. [VERIFIED: quantum.rs:148-158, 259-269, 295-305, 317-327]

`open` inbound callers of `attach_outcomes` include M010 snapshot-discard tests, S05 outcome tests, H2 tenant, `kutha-tenant` — adding `attach_justifications` beside each `attach_outcomes` keeps those paths compiling and empty-safe. [CBM `trace_path` inbound `attach_outcomes`]

### Pattern 3: Shared fixture builder (D-F1)

**What:** One narrative function; three `#[test]` oracles.
**When to use:** Always for S08.

Encoding (glue, not a new SoT):

| Role | Existing pattern | Fixture choice |
|------|------------------|----------------|
| Two supports for `P` | `m011_claim_supports.rs` shared `claim_id` | Assert `a` `claim: None`; Assert `b` `claim: Some(claim_p)`; subject = source term, relation `relatedTo`, object = interned `P` |
| Residuals of `a` | `m011_partial_correction.rs` | t2 `CorrectInterval` `patch_from=2015` `patch_to=Some(2020)` |
| P→Q | `derived_q_loses_eligibility_when_last_premise_support_withdrawn` | `Op::Behavior { name: "derive_pq", caused_by: claim_p, rule_version: "r1", subject: Q, ... }` |
| t3 last support | Retract `b`’s `fact_seq` | Observation 3 only requires residuals of **`a`**, not `b` |
| Conflict | intern `not-P` as object | t2 object = `not-P` (conflict) or `P-prime` (non-entailing replacement). FIX-01 named oracle **must** run the conflict branch |

Allowlist already contains `relatedTo`. [VERIFIED: crates/kutha-runtime/dictionaries/relations.yaml:6-9]

Suggested GATE names (lock across fsm/checks; planner may shorten **once**):

- `e2e_fixture_supports_and_conflict_at_named_cuts`
- `e2e_justification_cites_sources_and_rejects_stale_admission`
- `e2e_incremental_matches_reconstruct_after_discarding_leases`

Cuts (year-like clocks, same as S04):

```text
VF_WIDE = 2010; PATCH [2015, 2020); VT_LEFT = 2012; VT_INTERIOR = 2017; VT_RIGHT = 2021
t1 = ingested_at after derive; t2 = after CorrectInterval; t3 = after Retract b
Named observations at (t1, 2017), (t2, 2017), (t3, 2017); residuals at (t2 or t3, 2012/2021)
```

### Pattern 4: Polarity vs `derivation_eligible_at` (compose, do not rewrite)

**What:** S03 eligibility is claim-liveness, not “object still P”.
**When to use:** FIX-01 observation 2 and FIX-02 `derivation_eligible_at` clause.

```rust
// Source: crates/kutha-runtime/src/quantum.rs:552-578 [VERIFIED]
    /// Thin P→Q oracle (M011 S03): a Behavior-derived claim is eligible at a cut
    /// iff the derived fact is still live and its premise claim still has a live support.
    pub fn derivation_eligible_at(&self, derived: EventId, tt: u64, vt: u64) -> bool {
        if !self.fold.claim_supported_at(derived, tt, vt) {
            return false;
        }
        // ... premise = Assert.claim.unwrap_or(cause.id) or Behavior id ...
        self.fold.claim_supported_at(premise, tt, vt)
    }
```

```rust
// Source: crates/kutha-runtime/src/fold.rs:243-282 [VERIFIED]
            Op::CorrectInterval { ... } => {
                // ...
                let claim_id = old.claim_id;
                // ...
                self.facts[idx].invalidated_at = Some(event.ingested_at);
                // prefix / replacement / suffix all push_row(..., claim_id)
```

After t2, `claim_supported_at(claim_p, t2, 2017)` is **true** (replacement Fact). Observation 2 still requires: last **positive** support for `P` gone at t3 ⇒ empty live Facts with `object == P`. Historical Q Fact stays live (`claim_supported_at(claim_q)` true). Withdrawal does not prove the opposite.

**Locked composition:** keep `derivation_eligible_at` byte-identical. FIX-01/03 oracles assert:

1. Call `derivation_eligible_at` at t1 — must be `true`.
2. `conflict_report_at` / `live_supports` filtered by interned `P` / `not-P` for observations 1–2.
3. At t3, `positive_supports` empty; Q Fact still live; do **not** require `derivation_eligible_at == false` if a `not-P` residue still holds `claim_p`.

FIX-02 `check_admission`: treat D-F2’s “`derivation_eligible_at` is false” as **one** fail-closed arm. Also fail if **cited `source_fact_seqs` are not live** (`Fact::is_live_at`) — this is what actually goes stale after CorrectInterval/Retract. Fail if `rule_version` ≠ live Behavior pin for `target`. All three arms; any one is enough to deny.

### Pattern 5: GATE-01 trio (Phase 7)

**What:** FSM observe names + `file_contains` check + bridge citing the test file.
**When to use:** Wave 2 after oracles exist (D-F6).

```yaml
# Source: .kutha/dictionaries/checks.yaml:351-373 [VERIFIED]
  - id: m011-provenance
    steps:
      - kind: file_contains
        path: crates/kutha-runtime/tests/m011_provenance.rs
        needles:
          - "fn provenance_detects_caused_by_swap_when_state_fingerprint_matches"
          - "fn provenance_detects_rule_version_change_when_state_fingerprint_matches"
```

Copy with `id: m011-e2e`, `path: crates/kutha-runtime/tests/m011_e2e_fixture.rs`, three `fn …` needles, `category: m011-s08`. Optional extra needles: `JUSTIFICATIONS_REL` in `store.rs`, `conflict_report_at` in `quantum.rs`.

Append the three fn names to `fsm.yaml` `observe_cargo.required` after the provenance pair. [VERIFIED: .kutha/dictionaries/fsm.yaml:50-51]

Honeycomb: append FIX oracle names to **ADR-013** (conflict / no supersession), **ADR-011** (supports / P→Q glue), **ADR-012** (snapshot discard; evidence list is currently empty), **ADR-040** (CSR drop-rebuild). Leave every `map: Proposed`. [VERIFIED: honeycomb.yaml:86-95, 97-106, 108-120, 190-201]

### Anti-Patterns to Avoid

- **Copy-pasting S03–S07 test bodies** as the S08 gate (D-F1).
- **Putting justifications in `Snapshot` or only in CSR** (D-F2 / FIX-03).
- **A fold Fact whose object is summary text** (D-F2).
- **Changing CorrectInterval to mint a new `claim_id` for `not-P`** (D-F7) — report polarity instead.
- **Using `derivation_eligible_at == false` as the sole t3 oracle** after interval-patch (Pattern 4).
- **Picking a conflict winner / ABAC allow** (D-F3).
- **Treating CSR neighbors as SoT** in FIX-03 — compare **fold** answers; leases must rebuild identically, not own truth.
- **`Event::new` to “refresh” a justification** — new UUID; cite stable ids.
- **Editing `.kutha/STATE.md` or checking ROADMAP S08 `[x]` while leased** (GATE-02).
- **`map: Accepted`** (GATE-03 / D-10 Trajectory).
- **Requiring every prior `m011_*` fn as S08 observe needles** — they already live in `observe_cargo.required`.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| JSONL sidecar codec | Custom line splitter / hand parser | `serde_json::to_writer` / `from_str` like `write_outcomes` / `load_outcomes` | Empty-line skip and missing-file semantics already exist |
| Support liveness | New index | `Fact::is_live_at` + `live_supports` | Fold already implements VT×TT |
| Independent supports | New claim algebra | `Assert.claim: Some(claim_p)` | S01/S03 shipped |
| Interval residuals | Reinterpret `Correct` | `Op::CorrectInterval` | S04; D-F7 |
| Reconstruct picture | DBSP | `GraphFold::replay` + `store::open` without snapshot | M010 + fold `replay` |
| CSR identity | Persist CSR | `csr_lease_at` / `typed_csr_lease_at` rebuild | Droppable (ADR-040) |
| Rule pin | Duplicate field on Justification-only | Cite `Op::Behavior.rule_version`; compare live pin | S07 |
| GATE registration | Python Check class | YAML trio | Harness constitution |
| Admission policy engine | ADR-080 pack | `check_admission` Result | Thin FIX-02 only |

**Key insight:** The hard part is **not** writing JSONL. It is keeping three consumers honest on one log: history (residuals + losers), current evidence (`conflict_report_at` / live supports), and allowed action (`check_admission` cannot be renewed by replaying a stale row). `CorrectInterval` preserving `claim_id` is a feature for residuals and a trap if eligibility is mistaken for polarity.

## Runtime State Inventory

> New durable sidecar beside existing store directories (same class as D-O1), not a string rename.

| Category | Items Found | Action Required |
|----------|-------------|------------------|
| Stored data | `store::persist` writes `events.wal`, `events.jsonl`, `terms.jsonl`, `snapshot.json`, `quantum_outcomes.jsonl`. `open` loads outcomes if present. **No** `justifications.jsonl` today | Code: add sidecar. Old dirs without the file must `open` with empty justifications (`load_outcomes` pattern). No migration rewrite |
| Live service config | None — verified: product has no n8n/Datadog UI for store files | none |
| OS-registered state | None — no systemd unit bound to store filenames | none |
| Secrets/env vars | `KUTHA_MAX_CASCADE` / `KUTHA_RELATIONS_PATH` unrelated to justifications | none |
| Build artifacts | Cargo target rebuilds after `Runtime` field add | Re-run `cargo test --workspace --offline` |

**Nothing found in category:** Live service / OS / secrets — in-process store files only.

Temp fixture dirs used by S05/M010 (`std::env::temp_dir()`) are test-only; they do not require data migration.

## Common Pitfalls

### Pitfall 1: Treating `derivation_eligible_at` as polarity
**What goes wrong:** t3 oracle expects `false` after CorrectInterval left a live `not-P` Fact on `claim_p`; test fights D-F7 or “fixes” fold.
**Why it happens:** S03 docstring says “premise claim still has a live support”.
**How to avoid:** Pattern 4. Assert empty positive-P supports + live historical Q.
**Warning signs:** PR changes `derivation_eligible_at` match arms.

### Pitfall 2: Citing `claim_id` only for “supports still live”
**What goes wrong:** After CorrectInterval, `claim_supported_at` stays true; stale t1 justification still admits.
**Why it happens:** D-F2 lists claim_ids; claim identity survives the patch.
**How to avoid:** Store **`source_fact_seqs`** (and claim/event ids). `check_admission` requires each cited seq live at the **row’s** `(tt, vt)`.
**Warning signs:** FIX-02 passes without retract/patch.

### Pitfall 3: Justifications only in RAM or snapshot
**What goes wrong:** `rm snapshot.json` or process restart loses cites, or worse invents admission from fold.
**Why it happens:** Easy to hang a Vec on Runtime and forget persist.
**How to avoid:** Mirror outcomes: persist last, load on all `open` returns, FIX-03 deletes snapshot and still sees rows.
**Warning signs:** `justification_records()` empty after M010-style open.

### Pitfall 4: Exhaustive `Runtime` / `RuntimeError` matches miss new fields
**What goes wrong:** rustc E0063 on `Runtime { ... }` literals; Display match non-exhaustive.
**Why it happens:** Four constructors plus tests construct `Runtime` only via `new` / helpers today — still add `justifications` in each struct literal in `quantum.rs`.
**How to avoid:** Grep `outcomes: Vec::new()` and pair a justifications field; add `AdmissionDenied` Display arm.
**Warning signs:** E0027 / E0004 after the field add.

### Pitfall 5: GATE needle / fn name drift
**What goes wrong:** Test renamed; `observe_cargo.required` or `file_contains` stale → `ci` HIGH (`observe-required-fn`).
**How to avoid:** Lock the three CONTEXT names; Wave 2 after Wave 1 fns exist.
**Warning signs:** `FSM required names missing as fn in crates`.

### Pitfall 6: Persist order / crash orphan
**What goes wrong:** Cites written before events; crash leaves justifications without matching log (false admission after open).
**Why it happens:** Copy-paste without the S05 comment.
**How to avoid:** Write events (and WAL) first, then terms/snapshot/outcomes, **then** justifications. Missing justifications file → empty vec (cannot admit). Do not infer cites from the log.
**Warning signs:** `check_admission` Ok on a store dir with no `justifications.jsonl` but a live Q Fact.

### Pitfall 7: Using `knows` for the P narrative
**What goes wrong:** Inverse-`knownBy` cascade changes support counts and TT.
**How to avoid:** `relatedTo` like S03/S04/S07 (no follow-on).
**Warning signs:** `events_in_quantum > 1` on the P Asserts.

### Pitfall 8: Weakening freeze / Accepted
**What goes wrong:** Rocks crate, ABAC, `map: Accepted`, STATE edit.
**How to avoid:** D-F6/D-F7; Trajectory text; GATE-02/03 as close owners.
**Warning signs:** STATE diff; honeycomb `map: Accepted`.

## Code Examples

### `claim_supported_at` / counts (FIX-01 glue)

```rust
// Source: crates/kutha-runtime/src/fold.rs:145-159 [VERIFIED]
    pub fn live_supports(&self, claim: EventId, tt: TransactionTime, vt: ValidTime) -> Vec<&Fact> {
        self.facts
            .iter()
            .filter(|f| f.claim_id == claim && f.is_live_at(tt, vt))
            .collect()
    }

    pub fn live_support_count(&self, claim: EventId, tt: TransactionTime, vt: ValidTime) -> usize {
        self.live_supports(claim, tt, vt).len()
    }

    pub fn claim_supported_at(&self, claim: EventId, tt: TransactionTime, vt: ValidTime) -> bool {
        self.live_support_count(claim, tt, vt) > 0
    }
```

### Recommended `ConflictReport` + `conflict_report_at`

```rust
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConflictReport {
    pub positive_supports: Vec<u64>, // live fact_seq with object == positive
    pub negative_supports: Vec<u64>, // live fact_seq with object == negative
}

impl Runtime {
    pub fn conflict_report_at(
        &self,
        claim: EventId,
        tt: u64,
        vt: u64,
        positive: TermId,
        negative: TermId,
    ) -> ConflictReport {
        let mut positive_supports = Vec::new();
        let mut negative_supports = Vec::new();
        for f in self.fold.live_supports(claim, tt, vt) {
            if f.object() == positive {
                positive_supports.push(f.seq);
            } else if f.object() == negative {
                negative_supports.push(f.seq);
            }
        }
        ConflictReport {
            positive_supports,
            negative_supports,
        }
    }
}
```

Report only. Both non-empty = conflict at that cut. Do not return a winner enum.

### Recommended `Justification` row (discretion locked)

```rust
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Justification {
    pub justification_id: String,
    pub target_claim: EventId,
    pub source_claim_ids: Vec<EventId>,
    pub source_event_ids: Vec<EventId>,
    pub source_fact_seqs: Vec<u64>,
    pub rule_version: String,
    pub tt: u64,
    pub vt: u64,
}

#[derive(Debug)]
pub enum RuntimeError {
    // existing variants unchanged...
    AdmissionDenied {
        justification_id: String,
        reason: &'static str,
    },
}
```

`reason` strings (for tests `matches!`, not Display-as-oracle): `"stale_support"`, `"ineligible"`, `"rule_version"`. Display can be `"AdmissionDeniedError"`.

`record_justification` appends to the RAM buffer (like `record_resume`). `check_admission(&self, justification_id: &str)` does not append. Fail-closed if id missing.

Live Behavior pin: find log event `id == target_claim` (Q’s Behavior id) and compare `rule_version`. [Behavior `claim_id` is `event.id` — VERIFIED: fold.rs:186-205]

### `check_admission` sketch

```rust
pub fn check_admission(&self, justification_id: &str) -> Result<(), RuntimeError> {
    let row = self
        .justifications
        .iter()
        .find(|j| j.justification_id == justification_id)
        .ok_or(RuntimeError::AdmissionDenied {
            justification_id: justification_id.into(),
            reason: "unknown_justification",
        })?;
    for seq in &row.source_fact_seqs {
        let live = self.fold.facts().iter().any(|f| f.seq == *seq && f.is_live_at(row.tt, row.vt));
        if !live {
            return Err(RuntimeError::AdmissionDenied {
                justification_id: row.justification_id.clone(),
                reason: "stale_support",
            });
        }
    }
    if !self.derivation_eligible_at(row.target_claim, row.tt, row.vt)
        && /* polarity: */ self
            .fold
            .live_supports(
                /* premise claim of target Behavior — resolve like derivation_eligible_at */,
                row.tt,
                row.vt,
            )
            .iter()
            .all(|f| f.object() != /* interned P held in row or resolved from fixture */)
    {
        // Keep this arm simple in implementation: fail if cited fact_seqs dead OR
        // live Behavior.rule_version != row.rule_version OR
        // derivation_eligible_at is false.
        // Polarity is FIX-01's job; FIX-02 stale is fact_seq + rule_version first.
    }
    let pin = match self.log.iter().find(|e| e.id == row.target_claim) {
        Some(e) => match &e.op {
            Op::Behavior { rule_version, .. } => rule_version.as_str(),
            _ => "",
        },
        None => "",
    };
    if pin != row.rule_version.as_str() {
        return Err(RuntimeError::AdmissionDenied {
            justification_id: row.justification_id.clone(),
            reason: "rule_version",
        });
    }
    Ok(())
}
```

Implement the ineligible arm as a straight `if !self.derivation_eligible_at(...)`. Do not block Wave 1 on polarity inside `check_admission`; FIX-01 already covers polarity. After t2, t1 rows fail on **`stale_support`** (invalidated fact_seq). That is the FIX-02 proof.

### Persist/open without snapshot (FIX-03)

```rust
// Source: crates/kutha-runtime/tests/m010_semantic_open.rs:40-44 [VERIFIED]
    store::persist(&rt, &dir).unwrap();
    std::fs::remove_file(dir.join("snapshot.json")).unwrap();
    let opened = store::open(&dir).unwrap();
    assert_eq!(opened.fold().fingerprint(), rt.fold().fingerprint());
```

S05 also asserts outcomes survive snapshot delete. [VERIFIED: m011_quantum_outcome.rs:110-121] Copy for `JUSTIFICATIONS_REL`.

Replay path (no store):

```rust
// Source: crates/kutha-runtime/src/fold.rs:296-301 [VERIFIED]
    pub fn replay(events: &[Event]) -> Self {
        let mut fold = Self::default();
        for e in events {
            fold.apply(e);
        }
        fold
    }
```

FIX-03: `GraphFold::replay(rt.log().as_slice()).fingerprint() == rt.fold().fingerprint()` plus equal `live_at` / support counts / `conflict_report_at` at each named cut.

CSR discard (RAM only — nothing to unlink on disk):

```rust
// Source: crates/kutha-runtime/src/quantum.rs:330-338 [VERIFIED]
    pub fn csr_lease_at(&self, tt: u64, vt: u64) -> CsrLease {
        CsrLease::from_fold(&self.fold, tt, vt, self.dict.len())
    }

    pub fn typed_csr_lease_at(&self, tt: u64, vt: u64) -> TypedCsrLease {
        TypedCsrLease::from_fold(&self.fold, tt, vt, self.dict.len())
    }
```

```rust
{
    let _u = rt.csr_lease_at(tt, vt);
    let _t = rt.typed_csr_lease_at(tt, vt);
} // drop
let u2 = rt.csr_lease_at(tt, vt);
let t2 = rt.typed_csr_lease_at(tt, vt);
// neighbors / edges_out equal to pre-drop; fold answers unchanged
```

### S03 emit sketch to glue (do not copy the whole file)

```rust
// Source: crates/kutha-runtime/tests/m011_claim_supports.rs:115-159 [VERIFIED]
    let first = rt.emit(Op::Assert { subject: p, relation: rel, object: true_, valid_from: 2010, valid_to: None, claim: None }).unwrap();
    let claim_p: EventId = first.receipt.event_ids[0];
    // second Assert claim: Some(claim_p)
    let derived = rt.emit(Op::Behavior {
        name: "derive_pq".into(),
        caused_by: claim_p,
        rule_version: String::new(),
        subject: q,
        relation: rel,
        object: true_,
        valid_from: 2010,
        valid_to: None,
    }).unwrap();
```

E2E uses **source subjects** `a`/`b` and object `P` (S04 shape) plus `rule_version: "r1".into()`. `receipt.event_ids[0]` is the appended event id (claim_id for `claim: None`).

### `RuntimeError` today (add `AdmissionDenied` beside `DuplicateResume`)

```rust
// Source: crates/kutha-runtime/src/quantum.rs:12-40 [VERIFIED]
pub enum RuntimeError {
    ReplayDivergence { expected: [u8; 32], actual: [u8; 32] },
    ProvenanceMismatch { expected: [u8; 32], actual: [u8; 32] },
    UnknownFact { fact_seq: u64 },
    UnknownRelation { name: String },
    UnknownClaim { claim: EventId },
    BrokenLineage { caused_by: EventId },
    IntervalPatchRejected { fact_seq: u64 },
    DuplicateResume { resume_of: String },
}
```

`Display` is exhaustive — new variant needs an arm. [VERIFIED: quantum.rs:42-61]

### `lib.rs` re-exports

```rust
// Source: crates/kutha-runtime/src/lib.rs:21-24 [VERIFIED]
pub use quantum::{
    disposition, OutcomeDisposition, PersistedQuantumOutcome, QuantumOutcome, Runtime,
    RuntimeError,
};
```

Add `ConflictReport`, `Justification` to this list so integration tests do not use `kutha_runtime::quantum::`.

### FIX-02 oracle sketch

```rust
#[test]
fn e2e_justification_cites_sources_and_rejects_stale_admission() {
    let mut fx = build_through_t1();
    let jid = fx.rt.record_justification(/* Q, fact_seqs of a and b, "r1", t1, 2017 */);
    fx.rt.check_admission(&jid).unwrap();
    store::persist(&fx.rt, &dir).unwrap();
    std::fs::remove_file(dir.join("snapshot.json")).unwrap();
    let opened = store::open(&dir).unwrap();
    opened.check_admission(&jid).unwrap();
    fx.apply_t2_conflict();
    fx.rt.check_admission(&jid).unwrap_err(); // stale_support
    let jid2 = fx.rt.record_justification(/* reevaluate: cite b only */);
    fx.rt.check_admission(&jid2).unwrap();
    fx.apply_t3_withdraw_b();
    fx.rt.check_admission(&jid2).unwrap_err();
    fx.rt.check_admission(&jid).unwrap_err(); // still cannot renew
}
```

### Inbound persist/open (do not break)

CBM `trace_path` inbound `store::persist`: S05 tests, M010 open tests, H2 tenant, `kutha-tenant` `main`, quantum `persist_open_round_trip`. New file after persist is additive; `open` must tolerate missing justifications.

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Observations 1–5 are documentation | Named e2e oracles on one builder | This phase / S08 | Semantic-contract stop condition becomes executable |
| S03–S07 isolated `m011_*` files | Compose via glue APIs | This phase | Prior files stay regression, not the S08 gate |
| Cites do not exist | `justifications.jsonl` | This phase | FIX-02; same durability class as outcomes |
| Conflict is prose in ADR-013 | `conflict_report_at` | This phase | Report only; still Proposed |
| Snapshot discard proven for intern + outcomes | Also justifications + fold answers | This phase | FIX-03 |

**Deprecated/outdated:**

- Reading S03 `derivation_eligible_at` as “object still P” after interval-patch.
- Treating FF5 or governor green as observations 1–5 satisfied (`semantic-contract-validation.md`).
- Using whole-version `Correct` as if it supplied residuals (S04 already forbids).

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | `receipt.event_ids[0]` on a non-`knows` Assert is the new event/claim id | Pattern 3 | Fixture would cite the wrong support; use `fold.facts().last().claim_id` as a cross-check |
| A2 | `fork_at` dropping justifications (like outcomes) is acceptable | Pattern 2 | Prefix-fork tests that later record cites start empty — document in API comment |
| A3 | FIX-02 ineligible arm may use `derivation_eligible_at` without polarity and still satisfy REQUIREMENTS via `stale_support` after t2 | Pattern 4 / FIX-02 sketch | If a reviewer insists on `derivation_eligible_at == false` at t3, they must be pointed at Pattern 4 rather than a fold change |

**If A1 is accepted:** builder stores ids from both receipt and fold after each emit.

## Open Questions (RESOLVED)

1. **Q1 — Should t2 CorrectInterval use a new claim_id for `not-P`?**
   - RESOLVED: **No** (D-F7). Report polarity via `conflict_report_at`.

2. **Q2 — Must t3 make `derivation_eligible_at` false?**
   - RESOLVED: **No.** Empty positive-P supports + live Q history. Keep S03 function unchanged.

3. **Q3 — Justification fields?**
   - RESOLVED: `justification_id`, `target_claim`, `source_claim_ids`, `source_event_ids`, `source_fact_seqs`, `rule_version`, `tt`, `vt`. Live-ness oracle is **fact_seq**.

4. **Q4 — Honeycomb evidence cells?**
   - RESOLVED: Append FIX fn names to ADR-013, ADR-011, ADR-012, ADR-040; `map: Proposed`.

5. **Q5 — Wave split?**
   - RESOLVED: Wave 1 = sidecar + `conflict_report_at` + three oracles + Product changelog. Wave 2 = GATE YAML + Process/Trajectory + `uv run kutha-gov ci`. GATE-02/03 verified in Wave 2 close (STATE still S08; freeze; Proposed).

6. **Q6 — Extra serde fixture as GATE needle?**
   - RESOLVED: Optional missing-file / empty-sidecar behavior is covered inside FIX-03 persist/open, **not** a fourth observe name.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| cargo / rustc | crate tests | ✓ | 1.98.1 | — |
| uv | kutha-gov ci | ✓ | 0.12.13 | — |
| Python pin | harness | ✓ | 3.13 via `uv run python` (`.python-version`); system `python3` is 3.12.3 | uv only, not system python3 |
| kutha-gov | GATE-01 / D-F5 | ✓ | workspace `scripts/kutha-gov` | — |
| codebase-memory-mcp | execute verification | ✓ | project indexed 2026-09-30T01:45:29Z; several product paths `metadata_changed` | Read source; do not `index_repository` unless parent asks |
| Context7 / Exa MCP | research-plan fetch | ✗ this runtime | — | In-repo serde/store patterns (used) |
| graphify | GSD graph overlay | disabled | — | CBM + Read |
| New crates | — | N/A | — | Must not install |
| RocksDB / Cypher / HNSW | — | frozen | — | Out of scope |

**Missing dependencies with no fallback:** none

**Missing dependencies with fallback:** Context7/Exa documentation fetch — in-repo `#[serde(default)]` and `store::{persist,open}` used instead.

Step 2.6: external tools required are cargo + uv only — both present.

## Validation Architecture

> `workflow.nyquist_validation` is **true** in `.planning/config.json`.

### Test Framework

| Property | Value |
|----------|-------|
| Framework | Rust `cargo test` (lib + integration) |
| Config file | workspace `Cargo.toml` / crate defaults (no jest/pytest for product) |
| Quick run command | `cargo test -p kutha-runtime --offline --test m011_e2e_fixture -- --nocapture` |
| Full suite command | `cargo test --workspace --offline` then `uv run kutha-gov ci` |

### Phase Requirements → Test Map

| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| FIX-01 | Supports at (t1,2017); b survives t2; conflict report both sides; t3 empty positive-P; Q history live; residuals 2012/2021 | integration | `cargo test -p kutha-runtime --offline e2e_fixture_supports_and_conflict_at_named_cuts -- --exact` | ❌ Wave 0 — create `tests/m011_e2e_fixture.rs` |
| FIX-02 | Cite sources + `rule_version`; persist/open without snapshot keeps row; t2/t3 stale cannot renew; mint new row after reevaluation | integration | `cargo test -p kutha-runtime --offline e2e_justification_cites_sources_and_rejects_stale_admission -- --exact` | ❌ Wave 0 — same file |
| FIX-03 | Incremental vs `replay` vs open-without-snapshot vs CSR drop-rebuild agree on values/supports/eligibility report/completeness | integration | `cargo test -p kutha-runtime --offline e2e_incremental_matches_reconstruct_after_discarding_leases -- --exact` | ❌ Wave 0 — same file |
| GATE-01 | FSM observe + check needles; ci HIGH 0 | harness | `uv run kutha-gov ci` | ❌ Wave 0 — YAML rows after tests exist |
| GATE-02 | Active Slice S08 while executing | process | Read `.kutha/STATE.md` (do not edit) | ✅ already `**Active Slice:** S08` |
| GATE-03 | Freeze + honeycomb Proposed | harness / review | `uv run kutha-gov map`; no Rocks/Cypher crates | ✅ map Proposed; Wave 2 must not flip Accepted |
| Regression | S03 last-support eligibility | integration | `cargo test -p kutha-runtime --offline derived_q_loses_eligibility_when_last_premise_support_withdrawn -- --exact` | ✅ `tests/m011_claim_supports.rs` |
| Regression | S04 residuals | integration | `cargo test -p kutha-runtime --offline interval_patch_leaves_vt_2012_and_2021_residuals -- --exact` | ✅ `tests/m011_partial_correction.rs` |
| Regression | S05 snapshot-discard outcomes | integration | `cargo test -p kutha-runtime --offline budgets_0_1_2_distinguish_zero_partial_full_after_persist_open -- --exact` | ✅ `tests/m011_quantum_outcome.rs` |
| Regression | S06 CSR drop-rebuild | integration | `cargo test -p kutha-runtime --offline untyped_csr_neighbor_set_and_ff5_still_hold -- --exact` | ✅ `tests/m011_typed_csr.rs` |
| Regression | S07 provenance | integration | `cargo test -p kutha-runtime --offline provenance_detects_rule_version_change_when_state_fingerprint_matches -- --exact` | ✅ `tests/m011_provenance.rs` |
| Regression | M010 open without snapshot | integration | `cargo test -p kutha-runtime --offline open_without_snapshot_recovers_intern_meanings -- --exact` | ✅ `tests/m010_semantic_open.rs` |

### Sampling Rate

- **Per task commit:** `cargo test -p kutha-runtime --offline --test m011_e2e_fixture` (after Wave 1) plus `derived_q_loses_eligibility_when_last_premise_support_withdrawn` and `interval_patch_leaves_vt_2012_and_2021_residuals` when touching fold/eligibility
- **Per wave merge:** `cargo test --workspace --offline` + `uv run kutha-gov ci` (Wave 2+)
- **Phase gate:** Full suite green before `/gsd-verify-work`

### Wave 0 Gaps

- [ ] `crates/kutha-runtime/tests/m011_e2e_fixture.rs` — FIX-01 + FIX-02 + FIX-03 named oracles + shared builder
- [ ] `Justification` / `conflict_report_at` / `record_justification` / `check_admission` / `JUSTIFICATIONS_REL` — product API under test
- [ ] `Runtime` + `from_snapshot` / `from_dict_and_events` / `fork_at` justifications field
- [ ] `.kutha/dictionaries/{fsm,checks,bridges,honeycomb}.yaml` — GATE-01 registration (after oracles exist)
- [ ] Product + Process (+ Trajectory for honeycomb evidence) `CHANGELOG.md` entries — docs-coupling
- [ ] `RuntimeError` `Display` arm for `AdmissionDenied`

*(Existing infrastructure: cargo workspace tests, S03–S07 `m011_*` files, governor `observe_cargo`, M010 snapshot-discard — reuse; no new test framework install.)*

## Security Domain

> `security_enforcement` enabled (ASVS level 1) in `.planning/config.json`.

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | no | No user auth surface in S08 |
| V3 Session Management | no | No sessions |
| V4 Access Control | no (thin only) | `check_admission` is cite freshness, **not** ABAC / ADR-080; do not add role/tenant checks |
| V5 Input Validation | yes | Unknown `justification_id` / unknown `fact_seq` cites fail-closed; fixtures construct events, they do not parse untrusted HTTP |
| V6 Cryptography | no new | Do not hand-roll a MAC over justifications; do not call SHA-256 provenance a grant of admission |

### Known Threat Patterns for Kutha e2e admission cites

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Replay stale justification after supports withdrawn | Tampering / Elevation of privilege (thin) | `check_admission` requires live `source_fact_seqs`; old row cannot renew |
| Store cites only in droppable CSR/snapshot | Tampering | Sidecar like outcomes (D-F2) |
| Treat conflict report as automatic deny/allow | Elevation of privilege | Report only (D-F3); no winner |
| Invent Full/admit from missing sidecar | Spoofing | Missing file = empty vec, not success (clone S05) |
| Change only `rule_version` on Behavior, replay old cite | Tampering | Compare cited pin to live Behavior field |
| Accidental Accepted/ADR promotion via evidence | Spoofing (process) | honeycomb `map: Proposed`; Trajectory text |
| Untrusted JSONL as instructions | Tampering | `serde` typed rows; ignore unknown fields via explicit struct (no `Value` catch-all policy engine) |

## Sources

### Primary (HIGH confidence)

- CBM `list_projects` / `search_graph` / `trace_path` / `get_code_snippet` / `check_index_coverage` on `kutha-graph` (2026-09-30 full index; `quantum.rs`/`event.rs` `metadata_changed` — discrete values from **Read**)
- `crates/kutha-runtime/src/store.rs` — `OUTCOMES_REL`, `persist`, `open`, `load_outcomes` [VERIFIED: lines 10-31, 33-88, 124-147]
- `crates/kutha-runtime/src/quantum.rs` — `Runtime`, outcomes buffer, `derivation_eligible_at`, CSR leases, `RuntimeError`, constructors [VERIFIED: lines 12-40, 124-231, 259-338, 443-451, 496-578]
- `crates/kutha-runtime/src/fold.rs` — `live_supports`, `CorrectInterval` `claim_id` copy, `replay` [VERIFIED: lines 145-159, 243-301]
- `crates/kutha-runtime/src/snapshot.rs` — no outcomes/justifications fields [VERIFIED: lines 6-14]
- `crates/kutha-common/src/event.rs` — `CorrectInterval`, `Behavior.rule_version` [VERIFIED: lines 40-60]
- `crates/kutha-runtime/tests/m011_claim_supports.rs` — supports + P→Q [VERIFIED: lines 6-189]
- `crates/kutha-runtime/tests/m011_partial_correction.rs` — residuals [VERIFIED: constants 8-13, test 16-113]
- `crates/kutha-runtime/tests/m011_quantum_outcome.rs` — persist/open/snapshot discard [VERIFIED: lines 108-124]
- `crates/kutha-runtime/tests/m011_typed_csr.rs` — CSR drop-rebuild [VERIFIED: lines 87-133]
- `crates/kutha-runtime/tests/m011_provenance.rs` — `rule_version` `"r1"` [VERIFIED: lines 40-50]
- `crates/kutha-runtime/tests/m010_semantic_open.rs` — open without snapshot [VERIFIED: lines 25-49]
- `.kutha/dictionaries/{fsm,checks,bridges,honeycomb}.yaml` — GATE-01 Phase 7 pattern
- `docs/architecture/semantic-contract-validation.md` — observations 1–5 [VERIFIED: lines 33-45]
- `docs/ADR/ADR-013-bitemporal-facts-invalidation.md` — `(positive_supports, negative_supports)` [VERIFIED: line 114]
- `.kutha/STATE.md` — Active Slice S08
- `.planning/phases/07-provenance-and-rule-version-check/07-RESEARCH.md` — structure / GATE / wave split
- `.planning/phases/05-persisted-quantum-outcome/05-CONTEXT.md` — D-O1 sidecar
- Workspace `sha2` [VERIFIED: Cargo.toml:11; Cargo.lock:244-245]

### Secondary (MEDIUM confidence)

- Phase 7 RESEARCH process inheritance (D-F5 parallel)
- Research-plan seam requested context7 + exa; those MCP providers were **not** available in this runtime. Digests stored from curated in-repo sources (`research-store put` keys `3dceccd2…`, `866d9c21…`, `b88f7b32…`)
- `gsd_run query classify-confidence --provider curated --verified` returned `LOW` for the seam’s package classifier; in-repo Read+CBM claims above are still HIGH under the discrete-value rule
- Recommended `reason: &'static str` on `AdmissionDenied` — not yet in tree

### Tertiary (LOW confidence)

- A1 receipt vs fold claim id identity on multi-event quanta (Assert without `knows` follow-on should be one id)
- Official serde.rs HTML not fetched this session; in-repo `#[serde(default)]` is the binding pattern if a later field is added to `Justification`

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — no new packages; workspace crates and `sha2` lock verified
- Architecture: HIGH — D-F1–F7 plus Read of persist/open/fold/eligibility; polarity composition documented
- Pitfalls: HIGH — sidecar clone, fact_seq vs claim_id, GATE drift, CorrectInterval claim_id copy evidenced by source

**CBM coverage (this session):** `store.rs`/`fold.rs`/`csr.rs` `metadata_match`; `quantum.rs`/`event.rs`/`m011_claim_supports.rs` `metadata_changed`; `m011_provenance.rs` `not_tracked`. Discrete API claims use Read, not stale snippets.

**Research date:** 2026-09-30
**Valid until:** 2026-10-30 (stable in-repo spike; re-check if `store::open` paths or `CorrectInterval` claim_id semantics change)
