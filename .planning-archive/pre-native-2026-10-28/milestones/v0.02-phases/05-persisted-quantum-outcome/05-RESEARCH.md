# Phase 5: Persisted quantum outcome - Research

**Researched:** 2026-09-29
**Domain:** Kutha event-log SoT / quantum budgets & recoverable outcomes (ADR-010/014 store sidecar)
**Confidence:** HIGH (in-repo emit/receipt/store/governor patterns); MEDIUM (JSON digest encoding choice); LOW (exact resume API name)

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

- **D-O1:** Persist quantum outcomes as an **authoritative durable file** beside the log (working name `quantum_outcomes.jsonl` under the store directory), written by `store::persist` and always loaded by `store::open`. This file is **not** a droppable lease (unlike `snapshot.json` / CSR). Discarding snapshot must not erase outcome history. Do **not** put outcomes only in `QuantumOutcome` RAM or only in snapshot. Do **not** invent a product graph fact for receipts. A new fold-no-op `Op` is allowed only if research proves the sidecar cannot survive the same open-without-snapshot path as Define — default is **sidecar**. — **Reversibility:** costly — once directories on disk carry the file, open/persist contracts depend on it

- **D-O2:** Persist a disposition that maps to OUT-01 as:
  - **Zero:** budget abort with **no** events committed in that quantum (`events_in_quantum == 0`, e.g. `max_cascade = 0`)
  - **Partial:** budget abort with a **non-empty** committed prefix (`aborted_on_budget && events_in_quantum > 0`, e.g. `max_cascade = 1` on inverse-`knows` storm)
  - **Full:** quantum reached idle without budget abort (`!aborted_on_budget`)
  Call `Ok(QuantumOutcome)` remains **not** completion proof (ADR-014). Planner may name the enum (`Zero`/`Partial`/`Full` or `BudgetStopped`/`Completed`) as long as the three fixtures are distinguishable after persist→open. — **Reversibility:** reversible — encoding only

- **D-O3:** Crash-after-prefix fixture (OUT-02): after a quantum commits a prefix to the log and persist writes events **without** a terminal **Full** (or Completed) outcome for that quantum, `open` must report **incomplete/unknown** for that quantum — **never** inferred success. Resume requires an **explicit** continuation/resume outcome record (new row in the outcomes file, or a named API that appends one) that cites the original quantum identity (receipt digest and/or first event id). `open` must **not** auto-resume. — **Reversibility:** reversible — test and API surface

- **D-O4:** Ship **two named oracles** only: (1) budgets 0/1/2 → Zero/Partial/Full after persist→open; (2) crash-after-prefix → no terminal success + explicit resume record. Do **not** implement full ADR-062 remote exactly-once, duplicate-delivery across forks, or cryptographic authenticator fields in this phase. Thin anti-duplicate for the same quantum id on a second resume attempt may be a single fail-closed assert if cheap; otherwise defer. — **Reversibility:** reversible — test scope

- **D-O5:** Inherit Phase 2 **D-G1…D-G3** and **D-10 / D-11 / D-15**, and Phase 4 wave-close pattern: every execute wave runs `uv run kutha-gov ci` (HIGH stops) + `explain trajectory` + D-10 Trajectory in SUMMARY; cargo on crate-touching waves. — **Reversibility:** reversible

- **D-O6:** Named cargo tests registered in governor (FSM observe + check/bridge needles), GATE-01, `ci` HIGH 0. Active Slice remains **S05** for the phase (GATE-02). Freeze / Proposed honeycomb (GATE-03). — **Reversibility:** reversible

- **D-O7:** Do not change in-memory `emit` cascade semantics except as needed to **record** outcomes for persist; whole-version Correct and CorrectInterval stay untouched. — **Reversibility:** reversible

### Claude's Discretion

User said «реши эти вопросы разумно и при необходимости доисследуй» for gray areas 1–4 → D-O1…D-O4 (plus D-O5…D-O7). Planner/researcher may refine file schema fields (quantum id, committed event ids, disposition, resume_of) and exact budget fixture ops; must not thaw freeze, expand into S06–S08, or treat call `Ok` as completion.

### Deferred Ideas (OUT OF SCOPE)

- Full constant-size cryptographic receipt / Merkle authenticator (ADR-014 open questions)
- ADR-050 meta_prompt_version on receipt tuples
- ADR-062 remote exactly-once / cross-branch duplicate delivery
- Typed CSR (S06), provenance-only check (S07), end-to-end fixture (S08)
- Rocks / Cypher / HNSW / legal pack
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| OUT-01 | Developer can tell zero, partial, and full progress apart for budgets 0/1/2 from persisted quantum outcome records | Sidecar `quantum_outcomes.jsonl` (D-O1) + disposition map from `aborted_on_budget` × `events_in_quantum` (D-O2). Three `Runtime::new(0|1|default)` inverse-`knows` emits → persist → open → assert Zero/Partial/Full. |
| OUT-02 | Developer can reopen a log after a crash following a committed prefix and see no terminal-success record; resume is an explicit record, never inferred | Persist events with committed prefix and **no** `Full` row for that quantum; `open` must not invent Full; `record_resume` (or equivalent) appends explicit resume citing quantum id; second resume may fail-closed if cheap (D-O3/D-O4). |
| GATE-01 | Named cargo tests registered in governor; `uv run kutha-gov ci` stays 0 HIGH | Copy Phase 4 trio: `fsm.yaml` `observe_cargo.required` + `checks.yaml` `file_contains` + `bridges.yaml` row; `docs-coupling` CHANGELOG. |
| GATE-02 | Execute only while Active Slice is S05 | `.kutha/STATE.md` already names **Active Slice: S05** — phase is executable; do not edit STATE / do not check ROADMAP S05 during delivery. |
| GATE-03 | Freeze unstarted; honeycomb stays Proposed | No Rocks/Cypher/HNSW/ADR-050 six dicts; ADR-014 `map: Proposed`; evidence list append only. |
</phase_requirements>

## Summary

Phase 5 is M011 **S05** (leased Active Slice): make **quantum progress durable** so budgets **0 / 1 / 2** are readable after `persist`→`open` as zero / partial / full (OUT-01), and a crash that leaves a committed event prefix **without** a terminal Full outcome never looks like success — resume is an **explicit** outcomes-file row (OUT-02).

Today `Runtime::emit` already returns `Ok(QuantumOutcome { receipt, events_in_quantum })` with `receipt.aborted_on_budget`, and unit tests prove Partial (`max_cascade=1`, 1 event) and Full (idle, 2 events on knows). `store::persist` writes `events.wal`, `events.jsonl`, `terms.jsonl`, and `snapshot.json` only — **no receipt/outcome file**. ADR-014 Clarification and the semantic-contract probe name this exact gap. ADR-010 requires quantum outcomes to survive dropping leases, so outcomes must **not** live only in `snapshot.json`.

**Primary recommendation:** Keep the sidecar (D-O1). Buffer outcomes on `Runtime` when `emit` finishes; rewrite `quantum_outcomes.jsonl` from that buffer in `persist` (same JSONL loop as `events.jsonl`); load the file in every `open` path (with or without snapshot). Map dispositions with `Zero`/`Partial`/`Full` (+ `Resume` kind; Incomplete = missing Full, not invented). Two integration tests under `crates/kutha-runtime/tests/m011_quantum_outcome.rs`, then GATE-01 needles. No new crates. Do not change cascade math except recording. Do not mark ROADMAP S05 done while STATE still leases S05.

## Architectural Responsibility Map

| Capability | Primary Tier | Secondary Tier | Rationale |
|------------|-------------|----------------|-----------|
| Emit→idle quantum + budget abort | API / Backend (`Runtime::emit`) | — | Cascade already commits prefix; D-O7 forbids semantic change |
| In-memory outcome buffer | API / Backend (`Runtime`) | — | Bridge between emit and persist; not SoT until written |
| Durable outcome records | Database / Storage (`store::persist` / `open` + `quantum_outcomes.jsonl`) | — | Authoritative beside log; not a lease (D-O1 / ADR-010) |
| Event log / WAL | Database / Storage (`events.jsonl` / `events.wal`) | — | Graph SoT; crash fixture reuses committed prefix |
| Snapshot / CSR | CDN / Static (droppable lease) | — | Must remain discardable without erasing outcomes |
| Disposition query after open | API / Backend (`Runtime` getters) | Test runner | OUT-01/02 oracles read persisted state |
| Explicit resume record | API / Backend (named record API) | Storage (append/rewrite outcomes file) | D-O3: never auto-resume on open |
| GATE-01 registration | Harness dictionaries (YAML) | — | Observe names; bridge cites product tests |

## Project Constraints (from .cursor/rules/)

- Chat with the user is Russian; this RESEARCH.md and all planning artifacts stay English.
- Do **not** load LifeOS / PAI / notify endpoints.
- Structural search: `codebase-memory-mcp`; `list_projects` first; graph before Grep for symbols; `check_index_coverage` on cited paths. **Do not** call `index_repository`, `delete_project`, or `manage_adr`.
- CBM this session: project `kutha-graph` present; cited paths `quantum.rs` / `store.rs` / governor YAML reported `metadata_changed` or `not_tracked` (`indexed_at` `2026-09-16T07:18:52Z`). Claims below are from **Read of source**, not from the stale graph as SoT. Integrator may reindex later; this researcher must not.
- Graphify overlay is **disabled** in GSD config — no `graphify query`.
- GitNexus is secondary — not queried.
- Product plane: `crates/kutha-*`. Harness: `scripts/kutha_gov`, `.kutha/`. No Python in `kutha-runtime`. No repo-root hexagon folders.
- Freeze until STATE names M002: no RocksDB, Cypher/GPML, HNSW, ADR-050 six dictionaries, legal pack.
- Honeycomb cells stay **Proposed**. Governor green ≠ ADR Accepted ≠ `L_capability`.
- New governor check = YAML row (allowed kinds in `.kutha/META.md`). Bridge cites product tests. Do not add `scripts/kutha_gov/checks/*.py`.
- `docs-coupling`: crate diffs need `CHANGELOG.md`; harness dictionary diffs need `CHANGELOG.md` (and bridge/invariant pairing as today).
- Do **not** edit `.kutha/STATE.md` during S05 delivery (lease cite only). Trajectory requires Active Slice S05 to remain an **unchecked** ROADMAP line `- [ ] **S05:` while leased.
- Compound Engineering verbs stay on CE / `kutha-changelog` skills at execute time.

## Standard Stack

### Core

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| Rust edition | `2021` (workspace) | Language | Existing |
| `kutha-common` | path | `Event`, `EventId`, `Op` | Log SoT types |
| `kutha-runtime` | path | emit, receipt, store | Product runtime |
| `serde` | `1.0.229` (`Cargo.lock`) | Serialize outcome records | Same as events |
| `serde_json` | `1.0.151` (`Cargo.lock`) | JSONL codec | Mirror `store::persist` events loop |
| `uuid` | `1.24.1`, features `v7`/`serde` | `EventId` in outcome rows | Existing |
| `sha2` | `0.10.9` | Receipt digest (already) | Existing |

### Supporting

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `crc32fast` | `1.5.0` | WAL framing | Unchanged; not for outcomes file |
| `uv` + Python | `uv 0.12.13`, Python `3.13.12` via `uv run` | `kutha-gov ci` | GATE-01 / D-O5 |
| `cargo` / `rustc` | `1.98.1` | `cargo test --workspace --offline` | FSM `observe_cargo` |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Sidecar `quantum_outcomes.jsonl` | Fold-no-op `Op` on event log | D-O1 default rejects; pollutes graph SoT with control evidence; only reconsider if open-without-snapshot cannot load sidecar — research shows open already reads arbitrary dir files (`terms.jsonl`); sidecar works |
| Sidecar | Fields inside `snapshot.json` | **Forbidden by D-O1 / ADR-010** — snapshot is a droppable lease |
| Sidecar | Only RAM `QuantumOutcome` | Lost on crash; fails OUT-01/02 |
| Hand JSONL (`to_writer` + `\n`) | New crate `serde-jsonlines` | Unnecessary dep; existing store already hand-rolls JSONL [CITED: jsonl.co/guide/jsonl-rust] |
| `Zero`/`Partial`/`Full` | Only `aborted_on_budget` bool | Cannot distinguish Zero vs Partial (both aborted) |

**Installation:** none — do not add crates or Python packages.

**Version verification:** `Cargo.lock` packages `serde` `1.0.229`, `serde_json` `1.0.151`, `uuid` `1.24.1`, `sha2` `0.10.9`, `crc32fast` `1.5.0`. [VERIFIED: Cargo.lock]

## Package Legitimacy Audit

This phase **does not install external packages**. Existing workspace pins stay. Legitimacy seam checked for `serde`, `serde_json`, `sha2` → all `OK` (already in tree). Do **not** add `serde-jsonlines`.

| Package | Registry | Age | Downloads | Source Repo | Verdict | Disposition |
|---------|----------|-----|-----------|-------------|---------|-------------|
| — | — | — | — | — | — | No new packages |

**Packages removed due to [SLOP] verdict:** none
**Packages flagged as suspicious [SUS]:** none

## Architecture Patterns

### System Architecture Diagram

```text
Developer / test
    |  Runtime::new(0|1|default) ; emit(Assert knows)
    v
Runtime::emit  (UNCHANGED cascade math — D-O7)
    |  budget loop: if used >= max_cascade → aborted=true
    |  Ok(QuantumOutcome { receipt, events_in_quantum })
    |  ALSO push PersistedQuantumRecord into Runtime.outcomes buffer
    v
store::persist(runtime, dir)
    |  events.wal + events.jsonl   (SoT bytes)
    |  terms.jsonl                 (derived intern picture)
    |  snapshot.json               (DROPPABLE lease)
    |  quantum_outcomes.jsonl      (AUTHORITATIVE — rewrite from buffer)
    |  Prefer write outcomes AFTER events so mid-crash ⇒ incomplete, not orphan Full
    v
Crash / discard snapshot.json / reopen
    v
store::open(dir)
    |  recover events (WAL or jsonl)
    |  restore Runtime (snapshot OR Define OR terms)
    |  ALWAYS load quantum_outcomes.jsonl if present
    |  missing file ⇒ empty outcomes (never invent Full)
    |  do NOT auto-append Resume
    v
Query dispositions / record_resume(original_id) → new Resume row → persist
    v
Named cargo tests → fsm observe_cargo.required → kutha-gov ci
```

### Recommended Project Structure

```text
crates/kutha-runtime/src/receipt.rs          # optional: Persistable helpers; keep QuantumReceipt fields
crates/kutha-runtime/src/quantum.rs          # buffer on Runtime; record_resume; disposition helper; emit records
crates/kutha-runtime/src/store.rs            # write/load quantum_outcomes.jsonl; OUTCOMES_REL const
crates/kutha-runtime/src/lib.rs              # re-export disposition / record types if public
crates/kutha-runtime/tests/m011_quantum_outcome.rs   # OUT-01 + OUT-02 (new)
.kutha/dictionaries/fsm.yaml                 # observe_cargo.required names
.kutha/dictionaries/checks.yaml              # m011-quantum-outcome file_contains
.kutha/dictionaries/bridges.yaml             # B-m011-quantum-outcome
.kutha/dictionaries/honeycomb.yaml           # ADR-014 evidence append only; map stays Proposed
CHANGELOG.md                                 # Product + Process (docs-coupling)
```

Do **not** add files under `scripts/kutha_gov/checks/`. Do **not** add Rocks/Cypher/HNSW crates. Do **not** edit `.kutha/STATE.md`.

### Pattern 1: Disposition from existing QuantumOutcome fields

**What:** Derive a persisted disposition from fields already produced by emit.
**When to use:** Every successful `emit` that should survive persist (all of them for OUT-01).
**Example:**

```rust
// Source: crates/kutha-runtime/src/quantum.rs:74-78, 336-355 (verbatim field names)
// QuantumOutcome { receipt, events_in_quantum }
// receipt.aborted_on_budget; events_in_quantum == used
fn disposition(aborted: bool, events_in_quantum: usize) -> OutcomeDisposition {
    match (aborted, events_in_quantum) {
        (true, 0) => OutcomeDisposition::Zero,
        (true, _) => OutcomeDisposition::Partial,
        (false, _) => OutcomeDisposition::Full,
    }
}
```

### Pattern 2: JSONL sidecar (mirror events.jsonl)

**What:** Rewrite authoritative JSONL beside the log; load line-by-line on open.
**When to use:** `persist` / `open` (D-O1).
**Example:**

```rust
// Source pattern: crates/kutha-runtime/src/store.rs:17-21 (events.jsonl loop)
const OUTCOMES_REL: &str = "quantum_outcomes.jsonl";
// File::create(dir.join(OUTCOMES_REL))?;
// for row in runtime.outcome_records() {
//     serde_json::to_writer(&mut f, row).map_err(json_err)?;
//     f.write_all(b"\n")?;
// }
```

### Pattern 3: GATE-01 trio (from Phase 4)

**What:** Identical test fn name in (1) `fsm.yaml` `observe_cargo.required`, (2) `checks.yaml` `file_contains` needles, (3) `bridges.yaml` cites the test file.
**When to use:** Every M011 slice closing GATE-01.
**Example check id:** `m011-quantum-outcome` / bridge `B-m011-quantum-outcome` — do **not** pile onto `m011-partial-correction`.

### Anti-Patterns to Avoid

- **Treating `Ok(QuantumOutcome)` as completion:** ADR-014 Clarification; tests must assert on persisted disposition / `aborted_on_budget`, not only `is_ok()`.
- **Storing outcomes only in `Snapshot`:** Violates D-O1 and ADR-010 lease survival.
- **Auto-resume inside `open`:** Violates D-O3.
- **Inferring Full when outcomes file missing but events exist:** Must report incomplete/unknown.
- **Changing `Op::Correct` / `CorrectInterval`:** Out of scope (D-O7).
- **New Python check kinds:** YAML rows only.
- **Adding `serde-jsonlines`:** Hand-roll like existing store.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| JSONL write/read | Custom framing/CRC for outcomes | Same `serde_json::to_writer` + `\n` as `events.jsonl` | Already proven in `store.rs`; WAL CRC is for events.wal only |
| Cascade budget fixtures | New rule engine | Existing inverse-`knows` + `Runtime::new(n)` | `budget_aborts_storm` / `cascade_idles_without_inverse_loop` already prove Partial/Full |
| Receipt digest | New crypto layout | Existing `QuantumReceipt::from_events` | ADR-014 full authenticator deferred |
| Governor registration | New check Python class | `file_contains` + `observe_cargo.required` + bridge | Phase 4 pattern |
| Crash simulation | Real process kill | Test: persist events, omit/skip Full row, reopen | Thin fixture (D-O4) |

**Key insight:** The hard product rule is **evidence authority and disposition**, not a new cascade. Emit already distinguishes Zero/Partial/Full in RAM; the phase is making that distinction survive disk and crash without inventing success.

## Common Pitfalls

### Pitfall 1: `KUTHA_MAX_CASCADE=0` via env does not yield Zero

**What goes wrong:** Env `0` is filtered out; cascade becomes 32.
**Why it happens:** `cascade_limit()` uses `.filter(|n: &usize| *n > 0).unwrap_or(32)`. [VERIFIED: crates/kutha-runtime/src/quantum.rs:65-71]
**How to avoid:** OUT-01 Zero fixture must call `Runtime::new(0)`, not rely on env.
**Warning signs:** Unexpected Full with 2 inverse events when “budget 0” was intended.

### Pitfall 2: Outcomes lost when snapshot discarded

**What goes wrong:** Loading only from snapshot path forgets sidecar.
**Why it happens:** `open` early-returns on `snapshot.json` today without reading other durable files except events. [VERIFIED: crates/kutha-runtime/src/store.rs:50-57]
**How to avoid:** Load `quantum_outcomes.jsonl` on **every** successful open path (snapshot, Define, terms, empty).
**Warning signs:** OUT-01 fails after `remove_file(snapshot.json)`.

### Pitfall 3: Writing outcomes before events

**What goes wrong:** Crash leaves Full row with empty/partial log → false completion signal.
**Why it happens:** Ordering error in `persist`.
**How to avoid:** Write events (and WAL) first; outcomes last. Crash after events ⇒ incomplete (correct for OUT-02).
**Warning signs:** Outcomes file newer than events with unmatched digests.

### Pitfall 4: Serde of `[u8; 32]` digest

**What goes wrong:** Raw byte array JSON is ugly or fails without a serde helper.
**Why it happens:** `QuantumReceipt.digest` is `[u8; 32]` and the struct is **not** currently `Serialize`. [VERIFIED: crates/kutha-runtime/src/receipt.rs:6-10]
**How to avoid:** Persist digest as lowercase hex `String` (64 chars) or serde bytes; keep in-memory receipt as `[u8; 32]`.
**Warning signs:** Compile error on `Serialize` derive for outcome row.

### Pitfall 5: Governor `docs-coupling` / bridge ledger

**What goes wrong:** `ci` HIGH because CHANGELOG or bridges.yaml omitted.
**Why it happens:** Crate or checks.yaml diff without paired changelog / bridge.
**How to avoid:** Same commit: Product changelog with crate; Process with dictionary; bridge row for new check.
**Warning signs:** `docs-coupling` or `bridges-ledger` HIGH.

### Pitfall 6: Checking ROADMAP S05 or editing STATE mid-slice

**What goes wrong:** Trajectory / lease honesty breaks (Phase 4 lesson).
**Why it happens:** Confusing delivery-done with Active Slice lease.
**How to avoid:** Leave `- [ ] **S05:` unchecked; do not edit `.kutha/STATE.md` in execute waves.
**Warning signs:** Trajectory explain fails Active Slice id.

## Code Examples

### Existing emit budget loop (do not change semantics)

```rust
// Source: crates/kutha-runtime/src/quantum.rs:336-355
while let Some(event) = pending.pop() {
    if used >= self.max_cascade {
        aborted = true;
        break;
    }
    used += 1;
    ids.push(event.id);
    digests.push(event.digest_bytes());
    let follow = self.follow_ons(&event);
    self.fold.apply(&event);
    self.log.append(event);
    for e in follow {
        pending.push(e);
    }
}

Ok(QuantumOutcome {
    receipt: QuantumReceipt::from_events(ids, &digests, aborted),
    events_in_quantum: used,
})
```

### Existing Partial / Full unit oracles (reuse pattern)

```rust
// Source: crates/kutha-runtime/src/quantum.rs:523-540 (Partial)
let mut rt = Runtime::new(1);
// emit Assert knows → aborted_on_budget == true, events_in_quantum == 1

// Source: crates/kutha-runtime/src/quantum.rs:501-519 (Full)
let mut rt = Runtime::default();
// emit Assert knows → !aborted_on_budget, events_in_quantum == 2
```

### Recommended persisted row shape (planner may rename fields)

```rust
// Recommended — not yet in tree [ASSUMED field names for planner]
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub enum OutcomeDisposition {
    Zero,
    Partial,
    Full,
    Resume,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct PersistedQuantumOutcome {
    pub quantum_id: String,           // hex(receipt.digest) preferred
    pub disposition: OutcomeDisposition,
    pub aborted_on_budget: bool,
    pub events_in_quantum: usize,
    pub event_ids: Vec<kutha_common::EventId>,
    pub receipt_digest_hex: String,
    pub resume_of: Option<String>,    // Some(quantum_id) iff Resume
}
```

### Crash + resume fixture sketch

```rust
// OUT-02 sketch — integration test
// 1) rt = Runtime::new(1); emit knows Assert → Partial prefix on log
// 2) persist to temp dir
// 3) Optionally delete/rewrite outcomes file to drop any Full (Partial alone is not Full)
//    Stronger crash: write events+terms via persist helper that skips outcomes, or
//    remove quantum_outcomes.jsonl after persist then open
// 4) opened = open(dir); assert no disposition Full for that quantum_id
// 5) opened.record_resume(quantum_id); persist; reopen
// 6) assert Resume row with resume_of == quantum_id exists
// 7) open must not have invented Full between steps 4–6
```

### GATE-01 needles (Phase 4 template)

```yaml
# fsm.yaml observe_cargo.required — append:
# - budgets_0_1_2_distinguish_zero_partial_full_after_persist_open
# - crash_after_prefix_has_no_terminal_success_until_explicit_resume

# checks.yaml — new id m011-quantum-outcome, category m011-s05
# bridges.yaml — B-m011-quantum-outcome cites tests/m011_quantum_outcome.rs
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Receipt only in RAM `QuantumOutcome` | Persist sidecar outcomes (this phase) | M011 S05 | OUT-01/02 falsifiable |
| `Ok(_)` read as done | `aborted_on_budget` + disposition | ADR-014 Clarification 2026-09-13 | Call success ≠ idle completion |
| Snapshots as sole recovery | Snapshot = lease; Define/terms/outcomes survive drop | M010 + S05 | Outcomes must not ride snapshot |

**Deprecated/outdated:**

- Syslog-only / snapshot-only “proof” of quantum completion (ADR-014 Alternatives / Clarification).
- Inferring success from “events exist on disk” without terminal outcome evidence.

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | Persist digest as lowercase hex string in JSONL | Code Examples | Alternate encoding still OK if round-trip stable; planner must pick one and test |
| A2 | Public API name `record_resume` / `OutcomeDisposition::{Zero,Partial,Full,Resume}` | Architecture | Rename OK under D-O2/D-O3 as long as fixtures distinguish |
| A3 | Thin fail-closed on duplicate resume is in-scope if one assert | D-O4 | If expensive, defer; OUT-02 still passes with single resume |
| A4 | Missing `quantum_outcomes.jsonl` ⇒ empty list, Runtime still opens | Pitfalls | Fail-closed open would break M010 empty-outcome dirs; confirm in plan |
| A5 | `Runtime::new(0)` yields Zero without needing emit follow-ons | OUT-01 | If future emit short-circuits empty quantum differently, Zero fixture breaks — verify with named test |

**If wrong:** Prefer A1–A3 as discussable encoding; A4–A5 must be verified by Wave 0 / first implement wave tests.

## Open Questions (RESOLVED)

1. **Should `open` return outcomes only via `Runtime` getters, or also a side channel?**
   - RESOLVED: Prefer `Runtime::outcome_records()` (and `attach_outcomes` / equivalent on open). No separate public `store::load_outcomes` side channel required for OUT-01/OUT-02; crash fixtures go through persist→open→`outcome_records()`. Locked in `05-01-PLAN.md`.

2. **Incomplete as explicit enum variant vs absence of Full?**
   - RESOLVED: **Absence / non-Full** is enough for OUT-02; do not add an `Incomplete` disposition. `OutcomeDisposition` is `{ Zero, Partial, Full, Resume }` only. Resume is the only new positive row kind beyond Zero/Partial/Full. Locked in `05-01-PLAN.md`.

3. **Honeycomb ADR-014 evidence timing**
   - RESOLVED: Append both named test fn names to ADR-014 `evidence` in the GATE-01 wave (`05-02-PLAN.md`); keep `map: Proposed`; do not edit ADR-014 markdown body.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| `cargo` / `rustc` | Product tests | ✓ | 1.98.1 | — |
| `uv` + Python 3.13 | `kutha-gov ci` | ✓ | uv 0.12.13 / 3.13.12 via `uv run` | Do not use system `python3` (3.12.3) |
| `serde` / `serde_json` | JSONL outcomes | ✓ | in workspace | — |
| RocksDB / Cypher / HNSW | — | N/A | — | Frozen — must not use |

**Missing dependencies with no fallback:** none

**Missing dependencies with fallback:** none

Step 2.6 external services: SKIPPED beyond toolchain (code/config-only phase).

## Validation Architecture

> `workflow.nyquist_validation` is `true` in `.planning/config.json`.

### Test Framework

| Property | Value |
|----------|-------|
| Framework | Rust `cargo test` (lib unit + integration under `crates/kutha-runtime/tests/`) |
| Config file | workspace `Cargo.toml` / crate `Cargo.toml` (no pytest for product) |
| Quick run command | `cargo test -p kutha-runtime --test m011_quantum_outcome --offline` |
| Full suite command | `cargo test --workspace --offline` && `uv run kutha-gov ci` |

### Phase Requirements → Test Map

| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| OUT-01 | Budgets 0/1/2 → Zero/Partial/Full after persist→open (and after discarding snapshot) | integration | `cargo test -p kutha-runtime --test m011_quantum_outcome budgets_0_1_2 --offline` | ❌ Wave 0 |
| OUT-02 | Crash after committed prefix → no Full; explicit resume visible; open does not auto-resume | integration | `cargo test -p kutha-runtime --test m011_quantum_outcome crash_after_prefix --offline` | ❌ Wave 0 |
| GATE-01 | FSM required names + check needles + bridge; ci HIGH 0 | harness | `uv run kutha-gov precommit` / `uv run kutha-gov ci` | ❌ Wave 0 (needles) |
| Regression | Existing Partial/Full unit tests still green | unit | `cargo test -p kutha-runtime --lib budget_aborts_storm cascade_idles_without_inverse_loop --offline` | ✅ |
| GATE-03 | Freeze members / ADR Proposed | harness | `uv run kutha-gov precommit --check freeze` (+ adr-status) | ✅ existing |

Suggested exact fn names (planner may shorten, but GATE needles must match byte-for-byte):

- `budgets_0_1_2_distinguish_zero_partial_full_after_persist_open`
- `crash_after_prefix_has_no_terminal_success_until_explicit_resume`

### Sampling Rate

- **Per task commit:** `cargo test -p kutha-runtime --test m011_quantum_outcome --offline` (once file exists) or `--lib` filters while scaffolding
- **Per wave merge:** `cargo test --workspace --offline` + `uv run kutha-gov ci`
- **Phase gate:** Full suite green + Trajectory block before `/gsd-verify-work`

### Wave 0 Gaps

- [ ] `crates/kutha-runtime/tests/m011_quantum_outcome.rs` — covers OUT-01, OUT-02
- [ ] `Runtime` outcome buffer + `store` read/write of `quantum_outcomes.jsonl` — product surface under test
- [ ] FSM / checks / bridges needles — GATE-01
- [ ] Optional: unit test `Runtime::new(0)` Zero without full persist (fast smoke)

*(Existing `budget_aborts_storm` / `cascade_idles_without_inverse_loop` cover RAM Partial/Full only — not OUT-01 persist.)*

## Security Domain

> `security_enforcement` enabled (`security_asvs_level: 1`).

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|------------------|
| V2 Authentication | no | Local library / file store; no auth surface |
| V3 Session Management | no | — |
| V4 Access Control | no | No multi-tenant ABAC this phase (ADR-080 frozen) |
| V5 Input Validation | yes | `serde_json` typed deserialize of outcome rows; invalid JSON → `InvalidData` like events |
| V6 Cryptography | partial | Reuse `sha2` receipt digest; **do not** hand-roll new crypto / PQ signatures (deferred ADR-014) |

### Known Threat Patterns for this stack

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Forged “success” after crash | Spoofing / Tampering | Never infer Full; require explicit Full or Resume rows |
| Truncated / corrupt outcomes JSONL | Tampering | Fail line parse like events.jsonl; do not invent dispositions |
| Path traversal via store `dir` | Elevation | Callers pass directory; no user URL — keep same trust model as today's `persist`/`open` |
| Duplicate resume side effects | Replay | Optional single fail-closed assert on second Resume for same `resume_of` (D-O4 thin) |

## Sources

### Primary (HIGH confidence)

- `crates/kutha-runtime/src/quantum.rs` — `QuantumOutcome`, `emit` budget loop, `cascade_limit`, unit tests (Read this session)
- `crates/kutha-runtime/src/store.rs` — `persist` / `open` directory contract (Read)
- `crates/kutha-runtime/src/receipt.rs` — `QuantumReceipt` fields (Read)
- `crates/kutha-runtime/src/snapshot.rs` — Snapshot is fold lease only (Read)
- `docs/ADR/ADR-014-cascade-budgets-quantum-receipts.md` Clarification (Read)
- `docs/ADR/ADR-010-event-log-runtime-quantum.md` D010-1 lease survival (grep + Read excerpt)
- `docs/architecture/semantic-contract-validation.md` budgets 0/1/2 probe (Read)
- `.kutha/STATE.md` Active Slice S05 (Read)
- `.kutha/dictionaries/fsm.yaml` / `checks.yaml` / `bridges.yaml` Phase 4 GATE pattern (Read)
- `.planning/phases/04-…/04-03-SUMMARY.md` GATE-01 trio (Read)
- CBM `list_projects`, `search_graph`, `trace_path`, `check_index_coverage` (stale metadata — source Read authoritative)

### Secondary (MEDIUM confidence)

- [CITED: jsonl.co/guide/jsonl-rust] / serde_jsonlines docs — community JSONL write pattern; **not** adopted as a new dependency

### Tertiary (LOW confidence)

- Exact public identifier spellings for resume API and hex field names (discretion)

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — reuse existing crates; no installs
- Architecture: HIGH — D-O1 sidecar + existing store/emit seams verified in source
- Pitfalls: HIGH — cascade_limit(0) env trap, snapshot early-return, write ordering verified or strongly evidenced

**Research date:** 2026-09-29
**Valid until:** 2026-10-29 (stable in-repo contracts; revisit if store/open rewritten)
