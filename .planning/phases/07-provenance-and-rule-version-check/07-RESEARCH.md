# Phase 7: Provenance and rule-version check - Research

**Researched:** 2026-09-30
**Domain:** Kutha log lineage / ADR-060 provenance verification apart from state fold fingerprint
**Confidence:** HIGH (in-repo `Op::Behavior`, `replay_check`, `GraphFold::fingerprint`, GATE trio, WAL/JSONL serde); MEDIUM (exact `provenance_check` error display string — resolved by recommendation below)

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

- **D-P1:** Primary oracle is **swap to another valid prior**, not ghost. Fixture: at least two earlier events (e.g. Assert A1 and Assert A2); emit `Op::Behavior` with `caused_by = A1` and a fixed triple; build a second log identical except `caused_by = A2`. Assert `fold.fingerprint()` (or `replay_check` state path) **matches** across both, and **provenance** verification **fails** (or provenance digests differ) on the swapped log. Keep existing `replay_rejects_behavior_without_prior_cause` (ghost / absent prior → `BrokenLineage`) as regression — it does **not** satisfy PROV-01 alone, because today's `replay_check` already accepts any earlier id. Construct logs via `Runtime::from_dict_and_events` (or equivalent); do not invent a production mutate-in-place API for live SoT. — **Reversibility:** reversible — test + check surface

- **D-P2:** Add **`rule_version: String`** on `Op::Behavior` (serde `#[serde(default)]` so existing fixtures deserialize). Fold continues to **ignore** `name`, `caused_by`, and `rule_version` when projecting Facts (fingerprint stays state-only). Empty default is allowed for legacy rows; the PROV-02 oracle uses non-empty pins (e.g. `"r1"` → `"r2"`) with identical triple and `caused_by`. Do **not** put rule version only in a droppable lease or only in quantum outcomes sidecar. Do **not** implement ADR-050 six dictionaries or meta_prompt binding. — **Reversibility:** costly — once Behavior events carry `rule_version` in the log/WAL schema, removing the field needs forever-compat decode or a migration

- **D-P3:** Keep **`Runtime::replay_check`** as today's **state** obligation: rebuild fold, compare `GraphFold::fingerprint`, plus `caused_by` **exists earlier** (`BrokenLineage`). Add a **separate provenance surface** (working names: `provenance_fingerprint() -> [u8; 32]` and/or `provenance_check` / `assert_provenance_matches(expected)`): hash Behavior lineage fields in log order — at least `(event.id, caused_by, name, rule_version)` — so PROV-01/02 oracles fail when only those fields change while state fingerprint matches. Do **not** merge execution replay into this API. Do **not** weaken `replay_check` to skip BrokenLineage. Exact error variant names are planner discretion (`ProvenanceMismatch` or compare digests in tests). — **Reversibility:** reversible — additive API; fingerprint mix is local

- **D-P4:** Ship **two named oracles** only: (1) PROV-01 — swap `caused_by` among valid priors, state fingerprint equal, provenance diverges; (2) PROV-02 — change only `rule_version`, state fingerprint equal, provenance diverges; both confirm `replay_check` still `Ok` on the tampered logs when priors exist (state path green). Suggested names (planner may shorten; GATE needles = exact fn names): `provenance_detects_caused_by_swap_when_state_fingerprint_matches`, `provenance_detects_rule_version_change_when_state_fingerprint_matches`. Do **not** implement summary/action citation records, stale-cache admission, or the full semantic-contract t1–t3 narrative (S08 / FIX-*). — **Reversibility:** reversible — test scope

- **D-P5:** Inherit Phase 2 **D-G1…D-G3** and **D-10 / D-11 / D-15**, and Phase 4–6 wave-close: every execute wave runs `uv run kutha-gov ci` (HIGH stops) + `explain trajectory` + D-10 Trajectory in SUMMARY; cargo on crate-touching waves. — **Reversibility:** reversible

- **D-P6:** Named cargo tests registered in governor (FSM observe + check/bridge needles), GATE-01, `ci` HIGH 0. Active Slice remains **S07** for the phase (GATE-02). Freeze / Proposed honeycomb (GATE-03). Do not edit `.kutha/STATE.md` during delivery. — **Reversibility:** reversible

- **D-P7:** Do not change Assert/Retract/Correct/CorrectInterval fold semantics; Behavior still projects the same Fact triple. Typed CSR and quantum outcomes stay untouched. — **Reversibility:** reversible

### Claude's Discretion

User said «реши эти вопросы разумно и при необходимости доисследуй» and required **codebase-memory-mcp** during execution → D-P1…D-P4 (plus D-P5…D-P7). Planner/researcher/executor MUST use CBM (`list_projects` first; `search_graph` / `trace_path` / `get_code_snippet`; `check_index_coverage` on touched paths; `detect_changes` after edits). Exact hash field order, error enums, and fixture term constants are discretionary; must not thaw freeze, expand into S08 FIX-*, or implement execution replay.

### Deferred Ideas (OUT OF SCOPE)

- Summary/action records citing source revisions + rule version (FIX-02 / Phase 8)
- Execution replay / CA-cache rerun (ADR-060 obligation 3)
- Full provenance polynomials / recursive provenance representation (ADR-011)
- ADR-050 meta_prompt_version / six dictionaries
- Cross-branch wrong-claim references (semantic-contract probe; ADR-061)
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| PROV-01 | Verification detects a change to only a Behavior `caused_by` reference even when the state fingerprint still matches | Clone log via `from_dict_and_events`; swap `caused_by` to another **earlier** event id; `fold.fingerprint()` equal; `provenance_fingerprint` differs; `replay_check` still `Ok` (D-P1, D-P3, D-P4). Ghost/`BrokenLineage` test stays regression only |
| PROV-02 | Verification detects a change to only a pinned rule version the same way; execution replay is not required | `rule_version: String` on `Op::Behavior` with `#[serde(default)]`; fold `..` still ignores it; oracle `"r1"` → `"r2"` same triple/`caused_by`/event id (D-P2, D-P4). No CA-cache rerun |
| GATE-01 | Named cargo tests registered in governor; `uv run kutha-gov ci` stays 0 HIGH | Copy Phase 6 trio: `fsm.yaml` `observe_cargo.required` + `checks.yaml` `m011-provenance` + `bridges.yaml` `B-m011-provenance`; docs-coupling CHANGELOG (D-P5/D-P6) |
| GATE-02 | Execute only while Active Slice is S07 | `.kutha/STATE.md` already names **Active Slice: S07** — executable; do not edit STATE / do not check ROADMAP S07 during delivery |
| GATE-03 | Freeze unstarted; honeycomb stays Proposed | No Rocks/Cypher/HNSW/ADR-050 six dicts; ADR-060/ADR-011 `map: Proposed`; evidence list append only |
</phase_requirements>

## Summary

Phase 7 delivers M011 **S07** (Active Slice leased): **provenance verification** that detects a change to only a Behavior `caused_by` (PROV-01) or only a pinned `rule_version` (PROV-02) while the **state** fold fingerprint still matches — **without** execution replay. Named cargo tests + governor registration (GATE-01).

Today `Runtime::replay_check` rebuilds the fold, compares `GraphFold::fingerprint()` (Facts only), then checks that each `Op::Behavior.caused_by` appears as an **earlier** event id (`BrokenLineage` if not). Any prior id is accepted, so swapping A1→A2 still returns `Ok`. `GraphFold::apply` on Behavior already ignores lineage via `..` and projects subject/relation/object/VT only. `rule_version` does not exist yet. The semantic-contract probe “Change only a causal reference or rule version” is therefore **not** executable. ADR-060 obligation 2 is the cell; obligation 3 stays unimplemented.

The phase is additive: one field on `Op::Behavior`, a parallel `Runtime` provenance API beside unchanged `replay_check`, two named integration tests, GATE-01 needles. No new crates, no fold/CSR/outcomes changes, no polynomials, no S08 summary cites.

**Primary recommendation:** Add `rule_version: String` with `#[serde(default)]` on `Op::Behavior`; keep the Behavior fold arm on `..`; implement `Runtime::provenance_fingerprint() -> [u8; 32]` hashing Behavior rows in log order as domain-tagged `(event.id, caused_by, name, rule_version)` with length-prefixed strings; add `provenance_check(expected)` + `RuntimeError::ProvenanceMismatch { expected, actual }` mirroring `ReplayDivergence`; ship the two D-P4 oracles under `tests/m011_provenance.rs`; register GATE-01 as `m011-provenance` / `B-m011-provenance`. Build oracles by **cloning** `Event` vecs (stable ids) then `from_dict_and_events` — never `Event::new` the swapped Behavior (a new UUID would move `Fact.claim_id` and the state fingerprint).

## Architectural Responsibility Map

| Capability | Primary Tier | Secondary Tier | Rationale |
|------------|-------------|----------------|-----------|
| Event log SoT (`Event` / `Op::Behavior`) | Database / Storage (`EventLog`, WAL/JSONL serde) | — | D-P2: `rule_version` is a logged field, not a lease |
| State picture (`GraphFold` / `Fact`) | API / Backend (fold) | — | D-P7: Behavior still projects the same triple; fingerprint stays Facts-only |
| State replay (`Runtime::replay_check`) | API / Backend | — | ADR-060 obligation 1 + exists-earlier `caused_by`; do not merge obligation 2 |
| Provenance digest (`provenance_fingerprint` / `provenance_check`) | API / Backend (`quantum.rs`) | — | ADR-060 obligation 2; separate surface (D-P3) |
| Receipt mix (`Event::digest_bytes`) | API / Backend (`kutha-common`) | Quantum receipt | Include `rule_version` bytes so receipts are not silent on PROV-02; not the oracle |
| Quantum outcomes sidecar | API / Backend (`PersistedQuantumOutcome`) | — | D-P2/D-P7: not the rule-version carrier; leave untouched |
| Typed/untyped CSR | API / Backend (`csr.rs`) | — | D-P7: out of scope |
| PROV-01/PROV-02 oracles | Test runner (integration tests) | — | Named cargo fns are fitness evidence |
| GATE-01 registration | Harness dictionaries (YAML) | — | Observe names; bridge cites product tests |
| Freeze / honeycomb map | Harness + ADR status | — | GATE-03; map stays Proposed |

## Project Constraints (from .cursor/rules/)

- Chat with the user is Russian; this RESEARCH.md and all planning artifacts stay English.
- Do **not** load LifeOS / PAI / notify endpoints (`quiet-no-lifeos`).
- Structural search: `codebase-memory-mcp`; `list_projects` first; graph before Grep for symbols; `check_index_coverage` on cited paths; `detect_changes` after edits. **Do not** call `index_repository` (unless missing/stale/user-asked), `delete_project`, or `manage_adr` (`code-graph-cbm.mdc`).
- CBM this session: project `kutha-graph` present (`nodes=17834`, `index_mode: full`, `indexed_at: 2026-09-30T01:45:29Z`). Cited paths `event.rs` / `fold.rs` / `quantum.rs` / `m011_claim_supports.rs` / `lib.rs` / `intern.rs` / `log.rs` / `receipt.rs` / `wal.rs` / `store.rs` reported `no_recorded_issue` + `freshness: metadata_match`. Claims below that quote discrete values cite **Read** line ranges.
- Graphify overlay is **disabled** in GSD config (`gsd_run graphify status` → `"disabled": true`) — no `graphify query`.
- GitNexus is secondary — not queried for this phase.
- CE skill routing for Russian verbs stays on `.cursor/rules/ce-skills-ru.mdc` (коммит → `ce-commit`, changelog → `kutha-changelog`) at execute time — not a research deliverable.
- Product plane: `crates/kutha-*`. Harness: `scripts/kutha_gov`, `.kutha/`. No Python in `kutha-runtime`. No repo-root hexagon folders.
- Freeze until STATE names M002: no RocksDB, Cypher/GPML, HNSW, ADR-050 six dictionaries, legal pack.
- Honeycomb cells stay **Proposed**. Governor green ≠ ADR Accepted ≠ `L_capability`.
- New governor check = YAML row. Bridge cites product tests. Do not add `scripts/kutha_gov/checks/*.py`.
- `docs-coupling`: crate diffs and harness dictionary diffs need `CHANGELOG.md` (`kutha-changelog` skill).
- Do **not** edit `.kutha/STATE.md` during S07 delivery. ROADMAP S07 checkbox stays unchecked while leased.

## Standard Stack

### Core

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| Rust edition | `2021` (workspace) | Language | Existing workspace |
| `kutha-common` | `0.0.0` path | `Event`, `Op`, `EventId` | Log SoT types; add `rule_version` here |
| `kutha-runtime` | `0.0.0` path | fold, `Runtime::replay_check`, provenance API | Sole implementation surface |
| `sha2` | workspace `0.10` (lock `0.10.9`) | `Sha256` fingerprints | Already used by `GraphFold::fingerprint` and `Event::digest_bytes` |
| `serde` / `serde_json` | workspace `1` | WAL + `events.jsonl` Event codec | `#[serde(default)]` for missing `rule_version` |

### Supporting

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `uuid` | workspace `1` (v7/std/serde) | Stable `Event.id` on cloned fixtures | Do **not** mint a new id when swapping lineage fields |
| cargo / uv / kutha-gov | local toolchain | Tests + governor | GATE-01 / D-P5 |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Separate `provenance_fingerprint` | Fold lineage into `replay_check` | Forbidden by D-P3 — would conflate ADR-060 obligations 1 and 2 |
| `rule_version` on quantum outcomes / CSR lease | Field on `Op::Behavior` | Forbidden by D-P2 — leases are droppable; outcomes are S05 progress, not rule pins |
| Reuse `Behavior.name` as the pin | Dedicated `rule_version` | Forbidden by CONTEXT specifics — name stays behavior id |
| Ghost `caused_by` as PROV-01 | Swap among valid priors | Forbidden by D-P1 — ghost already fails `BrokenLineage` |
| Provenance polynomials / Green semiring | Thin digest of lineage fields | Deferred (ADR-011 / S08) |
| Execution replay / CA-cache | Provenance digest only | Deferred (ADR-060 obligation 3) |

**Installation:**

```bash
# No new crates. Workspace members stay kutha-common + kutha-runtime only.
cargo test -p kutha-runtime --offline
uv run kutha-gov ci
```

**Version verification:** Workspace `Cargo.toml` members `kutha-common`, `kutha-runtime`; `edition = "2021"`; `sha2 = "0.10"`. Lockfile `name = "sha2"` / `version = "0.10.9"`. No `cargo search` / PyPI installs for this phase. [VERIFIED: Cargo.toml:1-14] [VERIFIED: Cargo.lock:243-245]

## Package Legitimacy Audit

> Phase installs **no** external packages. Legitimacy gate N/A for new names. `gsd_run query package-legitimacy check` requires at least one package argument — not invoked.

| Package | Registry | Age | Downloads | Source Repo | Verdict | Disposition |
|---------|----------|-----|-----------|-------------|---------|-------------|
| — | — | — | — | — | N/A | No installs |

**Packages removed due to [SLOP] verdict:** none
**Packages flagged as suspicious [SUS]:** none

*Do not add polynomial/provenance crates, Rocks, Cypher parsers, or ADR-050 dictionary runtimes in this phase.*

## Architecture Patterns

### System Architecture Diagram

```text
                    Op::Assert / Behavior / …
                            │
                            ▼
                     EventLog (SoT)
                            │
              ┌─────────────┴──────────────┐
              ▼                            ▼
        GraphFold::apply              provenance mix
        Behavior { subject,           Behavior lineage only:
          relation, object, VT, .. }    id, caused_by, name,
              │                         rule_version
              ▼                            │
     GraphFold::fingerprint                ▼
     (Facts: seq, s/r/o, VT, TT,     provenance_fingerprint
      claim_id)                      [u8; 32]
              │                            │
              ▼                            ▼
     Runtime::replay_check           provenance_check(expected)
     fingerprint match +             ProvenanceMismatch
     caused_by ∈ earlier ids         (does not rerun rules)
     BrokenLineage if ghost
              │
              └─ PROV-01/02: replay_check Ok on swapped
                 valid priors; provenance digest moves
```

### Recommended Project Structure

```text
crates/kutha-common/src/
└── event.rs                 # Op::Behavior + rule_version; digest_bytes mix

crates/kutha-runtime/
├── src/
│   ├── fold.rs              # Behavior arm stays `..` (unchanged semantics)
│   ├── quantum.rs           # + provenance_fingerprint / provenance_check
│   │                        #   + ProvenanceMismatch; replay_check unchanged
│   └── lib.rs               # RuntimeError already re-exported
└── tests/
    └── m011_provenance.rs   # PROV-01 + PROV-02 named oracles

.kutha/dictionaries/
├── fsm.yaml                 # + two observe_cargo.required names
├── checks.yaml              # + m011-provenance
├── bridges.yaml             # + B-m011-provenance
└── honeycomb.yaml           # ADR-060 + ADR-011 evidence append; map Proposed
```

### Pattern 1: Dual verification from one log
**What:** State replay and provenance are independent hashes of the same `EventLog`.
**When to use:** Always for S07 — never fold lineage into `GraphFold::fingerprint` and never skip `BrokenLineage`.
**Example:**

```rust
// Source: crates/kutha-runtime/src/quantum.rs:490-509 [VERIFIED]
pub fn replay_check(&self) -> Result<GraphFold, RuntimeError> {
    let rebuilt = GraphFold::replay(self.log.as_slice());
    let expected = self.fold.fingerprint();
    let actual = rebuilt.fingerprint();
    if expected != actual {
        return Err(RuntimeError::ReplayDivergence { expected, actual });
    }
    let mut seen = HashSet::new();
    for e in self.log.iter() {
        if let Op::Behavior { caused_by, .. } = &e.op {
            if !seen.contains(caused_by) {
                return Err(RuntimeError::BrokenLineage {
                    caused_by: *caused_by,
                });
            }
        }
        seen.insert(e.id);
    }
    Ok(rebuilt)
}

// Additive (recommended) — do not call this from replay_check:
pub fn provenance_fingerprint(&self) -> [u8; 32] { /* see Code Examples */ }
pub fn provenance_check(&self, expected: [u8; 32]) -> Result<(), RuntimeError> {
    let actual = self.provenance_fingerprint();
    if expected != actual {
        return Err(RuntimeError::ProvenanceMismatch { expected, actual });
    }
    Ok(())
}
```

### Pattern 2: Clone events, then `from_dict_and_events` (D-P1)
**What:** Stable `Event.id` so Behavior `Fact.claim_id` (set to `event.id`) does not move when only lineage fields change.
**When to use:** Both PROV oracles. Equivalent to building with `emit` then cloning `log().as_slice()`.
**Example:**

```rust
// Ghost regression already uses from_dict_and_events [VERIFIED: m011_claim_supports.rs:75-97]
let dict = rt.dictionary().strings().to_vec();
let events = rt.log().as_slice().to_vec();
let mut swapped = events.clone();
// patch only Op::Behavior.caused_by or rule_version; keep id, ingested_at, triple
let rt2 = Runtime::from_dict_and_events(dict, swapped, rt.max_cascade).unwrap();
assert_eq!(rt.fold().fingerprint(), rt2.fold().fingerprint());
assert_ne!(rt.provenance_fingerprint(), rt2.provenance_fingerprint());
rt.replay_check().unwrap();
rt2.replay_check().unwrap();
```

`from_dict_and_events` requires intern strings `"knows"` and `"knownBy"` in the dict vector. [VERIFIED: crates/kutha-runtime/src/quantum.rs:273-282]

### Pattern 3: GATE-01 trio (Phase 6)
**What:** FSM observe names + `file_contains` check + bridge citing the test file.
**When to use:** Every M011 slice close (D-P6).
**Example needles (exact fn names must match observe + check):**

```yaml
# fsm.yaml observe_cargo.required append:
# - provenance_detects_caused_by_swap_when_state_fingerprint_matches
# - provenance_detects_rule_version_change_when_state_fingerprint_matches

# checks.yaml:
# - id: m011-provenance
#   steps: file_contains on crates/kutha-runtime/tests/m011_provenance.rs
#   category: m011-s07

# bridges.yaml:
# - id: B-m011-provenance
#   check: m011-provenance
#   cites: crates/kutha-runtime/tests/m011_provenance.rs
```

Keep existing `replay_rejects_behavior_without_prior_cause` in `observe_cargo.required` (already listed). [VERIFIED: .kutha/dictionaries/fsm.yaml:42]

### Anti-Patterns to Avoid

- **Using ghost `caused_by` as PROV-01:** Already `BrokenLineage`; does not prove state-fingerprint-match + provenance fail (D-P1).
- **`Event::new` on the swapped Behavior:** New UUID v7 → `Fact.claim_id` changes → state fingerprint moves; oracle is invalid.
- **Merging provenance into `replay_check`:** Forbidden (D-P3); inbound callers include FF5/FF6/H2/H4/M010/M011 suites (CBM `trace_path` inbound).
- **Putting `rule_version` only on outcomes / CSR:** Forbidden (D-P2).
- **Reusing `name` as the rule pin:** Forbidden (CONTEXT specifics).
- **Hashing Fact triples into the provenance digest as the *only* mix:** Redundant with state; keep lineage fields so the oracle stays sharp. Domain tag still required so the two 32-byte arrays cannot be confused.
- **Changing Assert/Retract/Correct/CorrectInterval or typed CSR:** Forbidden (D-P7).
- **Checking ROADMAP S07 `[x]` or editing STATE while leased:** Trajectory honesty / GATE-02.
- **Treating governor green as ADR Accepted:** D-10 Trajectory must say so.
- **Implementing execution replay / FIX-02 cites:** Out of scope (D-P4 / deferred).

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| State fold identity | Custom fact serializer | `GraphFold::fingerprint` | Already SHA-256 over live Fact rows |
| Exists-earlier lineage | New graph walk | Keep `replay_check` `seen` set | Ghost oracle already shipped |
| Provenance of Behavior refs | Green–Karvounarakis polynomials | `provenance_fingerprint` over log order | S07 is thin refs + rule pin, not how-provenance |
| Missing JSON field on old WAL | Manual migrate-all events | `#[serde(default)]` like `Assert.claim` | Store/WAL already `serde_json` Event |
| GATE registration | Ad-hoc scripts / Python Check class | YAML check/bridge/fsm trio | Harness constitution |
| Rule pin in receipts-only | New outcome column | `Op::Behavior.rule_version` | D-P2: log owns the pin |
| Synthetic mutate-in-place SoT API | `Runtime::set_caused_by` | Clone `Vec<Event>` + `from_dict_and_events` | D-P1 |

**Key insight:** The hard part is **not** hashing — it is keeping three verifications honest: state fingerprint (Facts), exists-earlier `caused_by` (`replay_check`), and lineage digest (new). Swapping a valid prior must leave the first two green and move the third.

## Runtime State Inventory

> Schema addition on persisted `Event` JSON (WAL / `events.jsonl`), not a rename. After source is updated, old files without `rule_version` must still `open`.

| Category | Items Found | Action Required |
|----------|-------------|------------------|
| Stored data | `store::persist` writes `events.wal` via `serde_json::to_vec(e)` and `events.jsonl` via `serde_json::to_writer`; `open` / `wal::recover_events` deserialize `Event` | Code: `#[serde(default)]` on `rule_version`. No rewrite of existing files. Empty string for legacy rows (D-P2) |
| Live service config | None — verified by product having no n8n/Datadog-style UI config for Op fields | none |
| OS-registered state | None — no systemd/pm2 unit names bound to Behavior schema | none |
| Secrets/env vars | `KUTHA_MAX_CASCADE` / governor env unrelated to `rule_version` | none |
| Build artifacts | Cargo target dir rebuilds after `kutha-common` field add | Re-run `cargo test --workspace --offline` |

**Nothing found in category:** Live service / OS / secrets — verified by this being an in-process log schema field with no external registrar.

## Common Pitfalls

### Pitfall 1: Ghost prior as PROV-01
**What goes wrong:** Reuse `replay_rejects_behavior_without_prior_cause`; claim PROV-01 done.
**Why it happens:** Both mention `caused_by`.
**How to avoid:** D-P1 swap among **two recorded** Assert (or Behavior) ids; both `replay_check` `Ok`; only provenance digest moves.
**Warning signs:** Test expects `BrokenLineage` on the “tampered” log.

### Pitfall 2: New event id on the swapped Behavior
**What goes wrong:** State fingerprints diverge; PROV-01 cannot be shown.
**Why it happens:** `Event::new` always assigns `Uuid::now_v7()`.
**How to avoid:** `Clone` the `Event`; patch `op` fields only. `Fact.claim_id` for Behavior is `event.id`. [VERIFIED: crates/kutha-runtime/src/fold.rs:196-205]
**Warning signs:** `assert_eq!(fp1, fp2)` fails before provenance is even compared.

### Pitfall 3: Exhaustive `Op::Behavior` matches miss the new field
**What goes wrong:** Compile error, or `digest_bytes` silently omits `rule_version`.
**Why it happens:** `follow_ons` and `digest_bytes` list fields without `..`.
**How to avoid:** Update every constructor: `follow_ons` inverse_knows (`rule_version: String::new()`), both sites in `m011_claim_supports.rs`, `Event::digest_bytes` Behavior arm. Matches that already use `..` (`apply`, `replay_check`, `op_relation`, `Event::new` object_ids, `derivation_eligible_at`) keep compiling.
**Warning signs:** rustc E0027 / E0063 on `Op::Behavior`.

### Pitfall 4: String concatenation in the provenance mix
**What goes wrong:** `name="ab"` + `rule_version="c"` collides with `name="a"` + `rule_version="bc"`.
**Why it happens:** Bare `h.update(name.as_bytes()); h.update(rule_version.as_bytes())`.
**How to avoid:** Length-prefix each string (`u64` LE length then bytes). Domain tag `b"kutha-prov-v1"` so the digest cannot be mistaken for `GraphFold::fingerprint`.
**Warning signs:** Two semantically different lineage tuples share a digest.

### Pitfall 5: GATE needle / fn name drift
**What goes wrong:** Test renamed but `observe_cargo.required` or `file_contains` needles not updated → `ci` HIGH (`observe-required-fn`).
**Why it happens:** Planner shortens D-P4 names inconsistently.
**How to avoid:** Lock D-P4 names; copy Phase 6 pattern; Wave 2 adds needles after Wave 1 fns exist.
**Warning signs:** `FSM required names missing as fn in crates` in ci output.

### Pitfall 6: Weakening `replay_check` or treating ci green as Accepted
**What goes wrong:** Skip `BrokenLineage`; flip honeycomb `map: Accepted`; edit STATE.
**Why it happens:** “Unify” verification; success-criteria confusion.
**How to avoid:** D-P3/D-P6/D-P7; GATE-03 Proposed; D-10 Trajectory text.
**Warning signs:** `replay_rejects_behavior_without_prior_cause` fails; `map: Accepted`; STATE diff.

## Code Examples

### Current `Op::Behavior` (add `rule_version` here)

```rust
// Source: crates/kutha-common/src/event.rs:48-56 [VERIFIED]
    Behavior {
        name: String,
        caused_by: EventId,
        subject: TermId,
        relation: TermId,
        object: TermId,
        valid_from: ValidTime,
        valid_to: Option<ValidTime>,
    },
```

Serde default pattern already on Assert:

```rust
// Source: crates/kutha-common/src/event.rs:28-29 [VERIFIED]
        #[serde(default)]
        claim: Option<EventId>,
```

Recommended addition (same attribute, `String` default `""`):

```rust
        #[serde(default)]
        rule_version: String,
```

### Fold ignores lineage (`..`) — keep this arm

```rust
// Source: crates/kutha-runtime/src/fold.rs:186-207 [VERIFIED]
            Op::Behavior {
                subject,
                relation,
                object,
                valid_from,
                valid_to,
                ..
            } => {
                let seq = self.next_seq;
                self.next_seq += 1;
                self.facts.push(Fact {
                    seq,
                    subject: *subject,
                    relation: *relation,
                    object: *object,
                    valid_from: *valid_from,
                    valid_to: *valid_to,
                    ingested_at: event.ingested_at,
                    invalidated_at: None,
                    claim_id: event.id,
                });
            }
```

### State fingerprint is Facts-only (no `name` / `caused_by`)

```rust
// Source: crates/kutha-runtime/src/fold.rs:129-143 [VERIFIED]
    pub fn fingerprint(&self) -> [u8; 32] {
        let mut h = Sha256::new();
        for f in &self.facts {
            h.update(f.seq.to_le_bytes());
            h.update(f.subject.to_le_bytes());
            h.update(f.relation.to_le_bytes());
            h.update(f.object.to_le_bytes());
            h.update(f.valid_from.to_le_bytes());
            h.update(f.valid_to.unwrap_or(u64::MAX).to_le_bytes());
            h.update(f.ingested_at.to_le_bytes());
            h.update(f.invalidated_at.unwrap_or(u64::MAX).to_le_bytes());
            h.update(f.claim_id.as_bytes());
        }
        h.finalize().into()
    }
```

### `RuntimeError` today (add `ProvenanceMismatch` beside `ReplayDivergence`)

```rust
// Source: crates/kutha-runtime/src/quantum.rs:12-35 [VERIFIED]
pub enum RuntimeError {
    ReplayDivergence {
        expected: [u8; 32],
        actual: [u8; 32],
    },
    UnknownFact {
        fact_seq: u64,
    },
    UnknownRelation {
        name: String,
    },
    UnknownClaim {
        claim: EventId,
    },
    BrokenLineage {
        caused_by: EventId,
    },
    IntervalPatchRejected {
        fact_seq: u64,
    },
    DuplicateResume {
        resume_of: String,
    },
}
```

`Display` is an exhaustive `match` — new variant needs an arm (mirror `"ReplayDivergenceError"` → `"ProvenanceMismatchError"`). [VERIFIED: crates/kutha-runtime/src/quantum.rs:37-55]

### `Event::digest_bytes` Behavior arm (must list `rule_version`)

```rust
// Source: crates/kutha-common/src/event.rs:149-166 [VERIFIED]
            Op::Behavior {
                name,
                caused_by,
                subject,
                relation,
                object,
                valid_from,
                valid_to,
            } => {
                h.update(b"behavior");
                h.update(name.as_bytes());
                h.update(caused_by.as_bytes());
                h.update(subject.to_le_bytes());
                h.update(relation.to_le_bytes());
                h.update(object.to_le_bytes());
                h.update(valid_from.to_le_bytes());
                h.update(valid_to.unwrap_or(u64::MAX).to_le_bytes());
            }
```

Recommended: after `caused_by.as_bytes()`, `h.update(rule_version.as_bytes())` with **no** extra domain tag, so the empty default adds zero bytes.

### Cascade constructor that must compile after the field add

```rust
// Source: crates/kutha-runtime/src/quantum.rs:472-481 [VERIFIED]
                vec![Event::new(
                    Op::Behavior {
                        name: "inverse_knows".into(),
                        caused_by: event.id,
                        subject: *object,
                        relation: self.known_by,
                        object: *subject,
                        valid_from: *valid_from,
                        valid_to: *valid_to,
                    },
                    event.ingested_at,
                )]
```

Add `rule_version: String::new()` (legacy empty pin; not a PROV-02 fixture).

### Recommended `provenance_fingerprint` mix (discretion locked below)

```rust
use sha2::{Digest, Sha256};

pub fn provenance_fingerprint(&self) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(b"kutha-prov-v1");
    for e in self.log.iter() {
        if let Op::Behavior {
            name,
            caused_by,
            rule_version,
            ..
        } = &e.op
        {
            h.update(e.id.as_bytes());
            h.update(caused_by.as_bytes());
            h.update((name.len() as u64).to_le_bytes());
            h.update(name.as_bytes());
            h.update((rule_version.len() as u64).to_le_bytes());
            h.update(rule_version.as_bytes());
        }
    }
    h.finalize().into()
}
```

Hash **Behavior events only**, in `EventLog` order (`iter` is vec order). [VERIFIED: crates/kutha-runtime/src/log.rs:22-24]

### PROV-01 / PROV-02 fixture sketch (D-P4 names)

```rust
#[test]
fn provenance_detects_caused_by_swap_when_state_fingerprint_matches() {
    // dict via Runtime::default + intern; two Asserts A1, A2; Behavior caused_by A1,
    // name "derive_pq", rule_version "r1", fixed Q triple.
    // clone events; set caused_by = A2; from_dict_and_events.
    // fold.fingerprint equal; provenance_fingerprint differs;
    // both replay_check Ok.
}

#[test]
fn provenance_detects_rule_version_change_when_state_fingerprint_matches() {
    // same baseline; clone; rule_version "r1" -> "r2" only.
    // fold.fingerprint equal; provenance_fingerprint differs;
    // both replay_check Ok.
}
```

Optional (not a GATE needle): JSON without `rule_version` deserializes to `""`.

### Inbound `replay_check` callers (CBM `trace_path`, include_tests)

Do **not** change the success contract (`Ok` iff fingerprint match and every Behavior cause is earlier). Callers include `ff5_as_of_t1_differs_from_as_of_t2_on_statute_log`, `ff6_allowlisted_relation_still_appends`, H2/H4 tenant tests, M010 open tests, `m011_claim_supports` (including ghost + P→Q), `m011_partial_correction`, quantum unit tests (`replay_detects_tamper`, `persist_open_round_trip`, …), and `kutha-tenant` `main`. Adding `ProvenanceMismatch` is additive; existing `matches!(err, RuntimeError::…)` arms stay valid.

### GATE-01 Phase 6 template to copy

```yaml
# Source: .kutha/dictionaries/checks.yaml:327-337 [VERIFIED]
  - id: m011-typed-csr
    description: M011 S06 — typed CSR lease preserves relation labels and support multiplicity; untyped neighbor-set and FF5 still hold
    steps:
      - kind: file_contains
        path: crates/kutha-runtime/tests/m011_typed_csr.rs
        needles:
          - "fn typed_csr_preserves_relation_labels_and_support_multiplicity"
          - "fn untyped_csr_neighbor_set_and_ff5_still_hold"
        require: all
        category: m011-s06
        message: "M011 typed-csr tests missing {missing}"
```

Optional extra needles (Phase 6 also cited API symbols in `csr.rs` / `quantum.rs`): `rule_version` in `event.rs`, `provenance_fingerprint` in `quantum.rs`.

Honeycomb today:

```yaml
# Source: .kutha/dictionaries/honeycomb.yaml:273-282 [VERIFIED]
  - id: ADR-060
    axis: Verify
    must: "strict replay of the event log; unavailable outcome proposed beyond fingerprint check"
    path: docs/ADR/ADR-060-strict-replay.md
    map: Proposed
    delivery: spike
    capability: none
    depends_on: [ADR-000, ADR-010, ADR-014]
    locks: [D9]
    evidence: [replay_rejects_behavior_without_prior_cause]
```

Append the two oracle fn names to **ADR-060** and **ADR-011** `evidence`; leave `map: Proposed`. ADR-011 evidence already includes `replay_rejects_behavior_without_prior_cause`. [VERIFIED: .kutha/dictionaries/honeycomb.yaml:86-95]

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| `replay_check` = fold fingerprint + exists-earlier `caused_by` | Same state path **plus** separate provenance digest | Phase 7 / S07 | PROV-01/02 become executable without obligation 3 |
| ADR-060 Clarification documents three obligations | Code implements obligation 2 thinly | This phase | Obligation 3 remains unimplemented; passing `replay_check` still does not prove execution replay |
| Probe “Change only a causal reference or rule version” is documentation | Named oracles on cloned logs | This phase | Observation 4 (summary/action cites) stays S08 |

**Deprecated/outdated:**

- Reading `replay_check` `Ok` as “provenance verified” — ADR-060 Clarification: it does not compare swapped valid causes or rule pins.
- Using Behavior `name` as if it were `meta_prompt_version` / ADR-050 — freeze.

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | SHA-256 `update` of an empty slice does not change the digest, so `digest_bytes` + empty `rule_version` stays byte-identical to today | Code Examples (`digest_bytes`) | If wrong, inverse_knows receipt hex tests (if any appear later) could flake; no current test freezes Behavior receipt hex |
| A2 | Optional serde round-trip test is not a GATE needle | Validation | Weak WAL compat signal; still recommended in the test file |
| A3 | Display string `"ProvenanceMismatchError"` is the right parallel to `"ReplayDivergenceError"` | Architecture | Cosmetic; tests should `matches!` the variant, not the Display string |

**If A1 is accepted:** do not add a `b"rv"` tag in `digest_bytes` (that would change all Behavior receipts even when the string is empty).

## Open Questions (RESOLVED)

1. **Q1 — Fingerprint-only in tests vs `ProvenanceMismatch`?**
   - RESOLVED: Implement both `provenance_fingerprint` and `provenance_check`; add `RuntimeError::ProvenanceMismatch { expected, actual }`. Oracles may `assert_ne` fingerprints (clearer) and still call `replay_check`. Locked for planner.

2. **Q2 — Hash mix details?**
   - RESOLVED: Domain tag `b"kutha-prov-v1"`; Behavior rows only; order `(event.id, caused_by, name, rule_version)` with `u64` LE length prefixes on `name` and `rule_version`. Do not mix Fact triples. Locked for planner.

3. **Q3 — Honeycomb evidence: ADR-060, ADR-011, or both?**
   - RESOLVED: Append both oracle fn names to **ADR-060** and **ADR-011** evidence; `map: Proposed` unchanged.

4. **Q4 — `Event::digest_bytes` and cascade `rule_version`?**
   - RESOLVED: Mix `rule_version.as_bytes()` into `digest_bytes` with no extra tag; `follow_ons` / existing tests use `String::new()`. Locked for planner.

5. **Q5 — Changelog / wave split?**
   - RESOLVED: Wave 1 = crate field + API + oracles + Product changelog; Wave 2 = GATE YAML + Process/Trajectory + honeycomb evidence + `uv run kutha-gov ci` (Phase 6 pattern). GATE-01 needles stay on Wave 2.

6. **Q6 — Extra serde fixture as GATE needle?**
   - RESOLVED: Optional `behavior_without_rule_version_field_deserializes` in `m011_provenance.rs` is **not** a GATE observe name (D-P4 two oracles only).

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| cargo / rustc | crate tests | ✓ | 1.98.1 | — |
| uv | kutha-gov ci | ✓ | 0.12.13 | — |
| Python pin | harness | ✓ | 3.13 (`.python-version`) | uv only, not system python3 |
| kutha-gov | GATE-01 / D-P5 | ✓ | workspace `scripts/kutha-gov` | — |
| codebase-memory-mcp | execute verification | ✓ | project indexed 2026-09-30T01:45:29Z | Grep only if coverage gap |
| Context7 / Exa MCP | research-plan fetch | ✗ this runtime | — | In-repo serde/`sha2` patterns (used) |
| graphify | GSD graph overlay | disabled | — | CBM + Read |
| New crates / polynomials | — | N/A | — | Must not install |
| RocksDB / Cypher / HNSW | — | frozen | — | Out of scope |

**Missing dependencies with no fallback:** none

**Missing dependencies with fallback:** Context7/Exa documentation fetch — in-repo `#[serde(default)]` and `Sha256` patterns used instead.

Step 2.6: external tools required are cargo + uv only — both present.

## Validation Architecture

> `workflow.nyquist_validation` is **true** in `.planning/config.json`.

### Test Framework

| Property | Value |
|----------|-------|
| Framework | Rust `cargo test` (lib + integration) |
| Config file | workspace `Cargo.toml` / crate defaults (no jest/pytest for product) |
| Quick run command | `cargo test -p kutha-runtime --offline provenance -- --nocapture` |
| Full suite command | `cargo test --workspace --offline` then `uv run kutha-gov ci` |

### Phase Requirements → Test Map

| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| PROV-01 | Swap `caused_by` among valid priors; state FP match; provenance diverges; `replay_check` Ok | integration | `cargo test -p kutha-runtime --offline provenance_detects_caused_by_swap_when_state_fingerprint_matches -- --exact` | ❌ Wave 0 — create `tests/m011_provenance.rs` |
| PROV-02 | Change only `rule_version`; state FP match; provenance diverges; no execution replay | integration | `cargo test -p kutha-runtime --offline provenance_detects_rule_version_change_when_state_fingerprint_matches -- --exact` | ❌ Wave 0 — same file |
| GATE-01 | FSM observe + check needles; ci HIGH 0 | harness | `uv run kutha-gov ci` | ❌ Wave 0 — YAML rows after tests exist |
| GATE-02 | Active Slice S07 while executing | process | Read `.kutha/STATE.md` (do not edit) | ✅ already `**Active Slice:** S07` |
| GATE-03 | Freeze + honeycomb Proposed | harness / review | `uv run kutha-gov map`; no Rocks/Cypher crates | ✅ map Proposed; Wave 2 must not flip Accepted |
| Regression | Ghost `caused_by` still `BrokenLineage` | integration | `cargo test -p kutha-runtime --offline replay_rejects_behavior_without_prior_cause -- --exact` | ✅ `tests/m011_claim_supports.rs` |
| Regression | P→Q eligibility still keys on premise claim | integration | `cargo test -p kutha-runtime --offline derived_q_loses_eligibility_when_last_premise_support_withdrawn -- --exact` | ✅ same file; `Op::Behavior { caused_by, .. }` |
| Regression | State tamper still `ReplayDivergence` | unit | `cargo test -p kutha-runtime --offline --lib replay_detects_tamper` | ✅ `quantum.rs` tests |

### Sampling Rate

- **Per task commit:** `cargo test -p kutha-runtime --offline provenance` (after Wave 1) plus `replay_rejects_behavior_without_prior_cause` when touching `replay_check` / Behavior
- **Per wave merge:** `cargo test --workspace --offline` + `uv run kutha-gov ci` (Wave 2+)
- **Phase gate:** Full suite green before `/gsd-verify-work`

### Wave 0 Gaps

- [ ] `crates/kutha-runtime/tests/m011_provenance.rs` — covers PROV-01 + PROV-02 named oracles
- [ ] `Op::Behavior.rule_version` + `Runtime::provenance_fingerprint` / `provenance_check` — product API under test
- [ ] `.kutha/dictionaries/{fsm,checks,bridges,honeycomb}.yaml` — GATE-01 registration (after oracles exist)
- [ ] Product + Process (+ Trajectory for honeycomb evidence) `CHANGELOG.md` entries — docs-coupling
- [ ] `RuntimeError` `Display` arm for `ProvenanceMismatch`

*(Existing infrastructure: cargo workspace tests, `m011_claim_supports.rs` ghost oracle, governor `observe_cargo` — reuse; no new test framework install.)*

## Security Domain

> `security_enforcement` enabled (ASVS level 1) in `.planning/config.json`.

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | no | No user auth surface in S07 |
| V3 Session Management | no | No sessions |
| V4 Access Control | no | ABAC / ADR-080 frozen; verification is in-process |
| V5 Input Validation | yes | `caused_by` must remain an `EventId` already in the log for `replay_check`; fixtures construct events, they do not parse untrusted HTTP |
| V6 Cryptography | yes (integrity hash only) | Reuse `sha2::Sha256` already in-tree; **do not** hand-roll a hash or call this digest a MAC/authenticator (ADR-014 receipts stay separate) |

### Known Threat Patterns for Kutha provenance checks

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Swap valid cause, keep triple — state replay still matches | Tampering | `provenance_fingerprint` over lineage fields (the S07 oracle) |
| Ghost cause admitted as “any UUID” | Tampering | Keep `BrokenLineage` in `replay_check` (do not weaken) |
| Treat provenance SHA-256 as authentication | Spoofing | Not a keyed MAC; no new crypto protocol |
| Stash rule pin only in droppable CSR / outcomes | Tampering / Information disclosure | Field on `Op::Behavior` in the log (D-P2) |
| Concatenation collision on name∥version | Tampering | Length-prefixed mix (Pitfall 4) |
| Accidental Accepted/ADR promotion via evidence | Spoofing (process) | honeycomb `map: Proposed` only; Trajectory text |

## Sources

### Primary (HIGH confidence)

- CBM `list_projects` / `search_graph` / `trace_path` / `get_code_snippet` / `check_index_coverage` on `kutha-graph` (2026-09-30 full index; cited paths `metadata_match`)
- `crates/kutha-common/src/event.rs` — `Op::Behavior`, `Event::digest_bytes`, `#[serde(default)]` on `claim` [VERIFIED: lines 17-61, 97-173]
- `crates/kutha-runtime/src/fold.rs` — `fingerprint`, Behavior `apply` arm, `replay` [VERIFIED: lines 129-143, 186-207, 296-301]
- `crates/kutha-runtime/src/quantum.rs` — `RuntimeError`, `from_dict_and_events`, `follow_ons` inverse_knows, `replay_check`, `derivation_eligible_at` [VERIFIED: lines 12-35, 267-300, 472-481, 490-524]
- `crates/kutha-runtime/tests/m011_claim_supports.rs` — ghost oracle + emit `Op::Behavior` [VERIFIED: lines 75-155]
- `crates/kutha-runtime/src/log.rs` — `EventLog::iter` / `from_events` [VERIFIED: lines 22-38]
- `crates/kutha-runtime/src/wal.rs` / `store.rs` — JSON Event persist/open [VERIFIED: wal.rs:20-22, 79; store.rs:18-22, 48]
- `crates/kutha-common/src/intern.rs` — `strings` / `from_strings` [VERIFIED: lines 34-44]
- `.kutha/dictionaries/{fsm,checks,bridges,honeycomb}.yaml` — GATE-01 Phase 6 pattern
- `docs/ADR/ADR-060-strict-replay.md` Clarification — three obligations [VERIFIED: lines 42-48]
- `docs/architecture/semantic-contract-validation.md` — probe “Change only a causal reference or rule version” [VERIFIED: line 55]
- `docs/ADR/ADR-011-lean-event-schema-lineage.md` — `caused_by` / polynomials deferred
- `.kutha/STATE.md` — Active Slice S07
- `.planning/phases/06-typed-csr-lease/06-RESEARCH.md` — structure / GATE / wave split
- Workspace `sha2` [VERIFIED: Cargo.toml:11; Cargo.lock:243-245]

### Secondary (MEDIUM confidence)

- Phase 6 RESEARCH / 06-02 PLAN process inheritance (D-P5 parallel)
- Research-plan seam requested context7 + exa; those MCP providers were **not** available in this runtime. Digests stored from curated in-repo sources (`research-store put` keys `6828e1a7…`, `81ade15b…`)
- `gsd_run query classify-confidence --provider curated --verified` returned `LOW` for the seam’s package classifier; in-repo Read+CBM claims above are still HIGH under the discrete-value rule

### Tertiary (LOW confidence)

- A1 empty SHA-256 update no-op (Assumptions Log)
- Official serde.rs HTML not fetched this session; in-repo `#[serde(default)]` is the binding pattern

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — no new packages; workspace crates and `sha2` lock verified
- Architecture: HIGH — dual verification forced by D-P1–D-P3 and current `replay_check` / fold `..`
- Pitfalls: HIGH — ghost oracle, `Event::new` id shift, exhaustive matches, GATE drift evidenced by existing code + Phase 6

**CBM coverage (this session):** `event.rs`, `fold.rs`, `quantum.rs`, `m011_claim_supports.rs`, `lib.rs`, `intern.rs`, `log.rs`, `receipt.rs`, `wal.rs`, `store.rs` → `metadata_match` (`indexed_at: 2026-09-30T01:45:29Z`).

**Research date:** 2026-09-30
**Valid until:** 2026-10-30 (stable in-repo spike; re-check if `Op` / `replay_check` heavily refactored)
