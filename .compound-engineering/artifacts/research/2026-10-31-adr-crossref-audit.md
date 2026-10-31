# ADR cross-reference and index consistency audit — 2026-10-31

Repository: `/root/kutha-graph`, branch `work/main-line`, HEAD `73e2c82` ("docs(process): draft the dogfood scratch-exemption intake").

Mode: **read-only**. No tracked file was edited, no commit or push, no cargo, no `graph index`, `.kutha/STATE.md` / `docs/ADR/` / `.planning/` / `crates/` / `scripts/` untouched. Working tree was clean at start (`git status --porcelain` empty); this report is the only file created.

Method: all checks are text/YAML analysis over the working tree, plus reading the governor's own check definitions in `.kutha/dictionaries/checks.yaml` and `scripts/kutha_gov/kinds.py`. The governor was **not executed** (read-only contract); its rules are quoted as evidence, and the same invariants were re-implemented locally. Every finding cites file:line.

## 0. Snapshot note — the brief's numbers are one commit-series old

The task brief named HEAD `2680cd9` and the map counts "ADR-010…094, 32 files, `delivery: spike 13 / map-only 12 / frozen 7`, `capability: named 2 / none 30`". Those numbers belong to an older tree. HEAD is `73e2c82`; `git log --oneline 2680cd9..HEAD` lists 11 commits, three of which touch the ADR surface:

- `f240dbd` `docs(adr): align parser, receipt, arity and thaw-map claims with HEAD` — edits `.kutha/STATE.md` and `docs/ADR/ADR-000`, `ADR-011`, `ADR-014`, `ADR-070`.
- `d430a14` `docs(map): record named capability evidence for ten cells` — flips `capability` `none -> named` on ten cells.
- `b353678` `docs(adr): record the shipped workspace shape as ADR-094` — adds the 33rd ADR file and its cell.

Recomputed from `git show d430a14^:.kutha/dictionaries/honeycomb.yaml`: 32 cells, `named 2 / none 30`, `spike 13 / map-only 12 / frozen 7` — i.e. exactly the brief. At HEAD: **33 cells, `named 12 / none 21`, `spike 13 / map-only 13 / frozen 7`**. All counts below are HEAD `73e2c82`.

## (a) Counts table

| Check | Item | Count | Result |
|---|---|---|---|
| 1 | ADR files `docs/ADR/ADR-*.md` | 33 | — |
| 1 | README index rows (3 spine + 30 honeycomb) | 33 | files not listed: **0**; rows without a file: **0** |
| 1 | README `Status` cell vs file `## Status` leading tag | 33 | mismatches: **0** |
| 1 | README `Date` cell vs file `## Date` | 33 | mismatches: **0** |
| 1 | README `Title` cell vs file H1 suffix | 33 | mismatches: **0** |
| 1 | README row link target resolves; basename id = row id | 33 | broken links: **0**; id mismatches: **0** |
| 2 | honeycomb cells | 33 | unique ids: 33/33; files not in a cell: **0**; cells without a file: **0** |
| 2 | cell `path` resolves and basename id = cell id | 33 | failures: **0**; duplicate paths: **0** |
| 2 | cell `map` = file `## Status` leading tag | 33 | mismatches: **0** |
| 2 | cell required fields (`id axis must path map delivery capability depends_on locks evidence`) | 33 | missing-field cells: **0** |
| 3 | `## Related Decisions` sections present | 33/33 | missing: **0** |
| 3 | distinct citations from `## Related Decisions` | 154 | unresolved `ADR-0NN`: **0** |
| 3 | reciprocal citation pairs | 39 | — |
| 3 | one-way citation edges (informational) | 76 | see Appendix B |
| 3 | self-citations | 0 | — |
| 4 | ADR mentions in `docs/architecture/**/*.md` | 105 | broken: **0** |
| 4 | ADR mentions in `docs/process/**/*.md` | 23 | broken: **0** |
| 4 | ADR mentions in `AGENTS.md` | 10 | broken: **0**; **stale range** `ADR-010–093` at :44 |
| 4 | ADR mentions in root `README.md` / `.kutha/**/*.md` | 2 / 56 | broken: 0 / 0 |
| 4 | ADR mentions inside `docs/ADR/**` | 620 | 18 non-Kutha ids, all explainable (Appendix A) |
| 5 | `depends_on` edges | 91 | unresolved: **0**; cycles: **0**; self-loops: **0**; isolated cells: **0** |
| 5 | dependency depth / longest chain | 9 | `ADR-071 -> 070 -> 043 -> 041 -> 040 -> 014 -> 010 -> 002 -> 001 -> 000` |
| 6 | ADR files with `## Date` | 33/33 | missing: **0** |
| 6 | `## Date` = README date cell | 33 | mismatches: **0** |

## Item results (1–6)

### 1. README index vs files on disk

- 33 files in `docs/ADR/`: `ADR-000`, `ADR-001`, `ADR-002`, `ADR-010`–`ADR-014`, `ADR-020`–`ADR-023`, `ADR-030`–`ADR-031`, `ADR-040`–`ADR-043`, `ADR-050`–`ADR-052`, `ADR-060`–`ADR-062`, `ADR-070`–`ADR-071`, `ADR-080`–`ADR-081`, `ADR-090`–`ADR-094`.
- Spine table: heading `docs/ADR/README.md:52`, rows `:56–58` (ADR-000/001/002).
- Honeycomb table: heading `docs/ADR/README.md:60`, rows `:64–93` (30 rows, ADR-010…094, including the ADR-094 row added at `:93`).
- Every row's link resolves and the link basename carries the row's id (checked as `ADR-<row id>-*.md`); no row points at another cell's file.
- `Status`: 33/33 README cells are `Proposed`; every file's `## Status` value begins `**Proposed**` with a qualifier (`docs/ADR/ADR-000-kutha-hybrid-architecture-research.md:5`, `docs/ADR/ADR-094-workspace-shape-as-shipped.md:5–6`). The README collapses the qualifier by design (`docs/ADR/README.md:26` defines the vocabulary) — match is on the leading tag, 0 mismatches.
- `Date`: 33/33 match, e.g. `docs/ADR/ADR-023-test-strategy-and-oracle-matrix.md:9` (2026-10-05) vs `docs/ADR/README.md:72`; `docs/ADR/ADR-094-workspace-shape-as-shipped.md:10` vs `docs/ADR/README.md:93`.
- `Title`: 33/33 README titles equal the H1 suffix (`# ADR-NNN: <title>`); all H1 ids also equal their file ids (0 mismatches).
- **Defect:** the README's own prose still says 32 files and stops the opened band at 093 (D1 below).

### 2. `honeycomb.yaml` vs files

- 33 cells under `cells:` (`.kutha/dictionaries/honeycomb.yaml:112`, first cell `:113`, last cell ADR-094 at `:713`); 33 files; 1:1 both directions.
- ids unique (33/33), no duplicate `path`, all `path` values resolve, each path basename starts with its cell id.
- `map` is `Proposed` for 33/33 and matches the `**Proposed**` tag in each ADR's `## Status` (e.g. `docs/ADR/ADR-000-kutha-hybrid-architecture-research.md:5`, `docs/ADR/ADR-094-workspace-shape-as-shipped.md:5`).
- Required fields present on every cell (`id axis must path map delivery capability depends_on locks evidence`); `freeze_as` present on exactly the 7 `delivery: frozen` cells (`:492`, `:515`, `:617`, `:645`, `:657`, `:669`, `:708`).
- `locks` block `.kutha/dictionaries/honeycomb.yaml:80–101` declares D1–D10, all pointing at `docs/ADR/ADR-000-kutha-hybrid-architecture-research.md`; every lock id used by a cell is in that set; each D-number has a heading/target in ADR-000 (D1–D10 sections start at `ADR-000:90`).
- The governor already enforces most of this: file→cell coverage (`checks.yaml:1084–1088`), required fields + unique id (`:1095–1101`), path existence (`:1102–1107`), `map` vocabulary and presence in the ADR file (`:1108–1113`, `:1132–1139`), axis vocabulary (`:1126–1131`), `freeze_as` presence on frozen cells and needle presence in `STATE.md` (`:1140–1155`), non-empty evidence for `named` (`:1156–1162`), `depends_on`/`locks` id resolution (`:1174–1187`). This audit re-implemented those invariants and found no violation.
- **Defect (hygiene):** the file's restage log has no entry for the two 2026-10-31 commits that changed it (D2 below).

### 3. `## Related Decisions`

- 33/33 files have exactly one level-2 `## Related Decisions` section (e.g. `docs/ADR/ADR-000-kutha-hybrid-architecture-research.md:418`, `docs/ADR/ADR-094-workspace-shape-as-shipped.md:131`).
- 154 distinct citations after expanding shorthand list/range forms (`docs/ADR/ADR-023-test-strategy-and-oracle-matrix.md:150` "ADR-010 / 012 / 014", `docs/ADR/ADR-043-hybrid-query-planner.md:96` "ADR-040–042", `:98` "ADR-070/071"); **0 unresolved** — every referenced id has a file.
- 39 unordered pairs cite each other; **76 one-way edges** remain (A cites B, B's `## Related Decisions` does not cite A). Asymmetry is informational; the full list is Appendix B.
- Structural asymmetries worth knowing: ADR-000 says `- None yet (root research ADR)` (`ADR-000:420`) but is cited by 11 cells — it is a lock source, not a hub. Cells with zero inbound `Related Decisions` citations: ADR-023, ADR-062, ADR-092, ADR-094.
- Only ADR-000 has zero outbound citations (`ADR-000:420`); the busiest citing cell is ADR-050 (10 outbound, `docs/ADR/ADR-050-meta-prompt-dictionaries.md:167–174`).
- Two `Related Decisions` lines cite another project's ADRs on purpose and were excluded from the Kutha graph: `docs/ADR/ADR-090-legal-reference-pack.md:254` (law-nexus) and `docs/ADR/ADR-093-scientific-archive-pack.md:265` (daily-archive).

### 4. Cross-document ADR mentions

- `docs/architecture/**/*.md`: 105 mentions, **0 broken** (e.g. `docs/architecture/benchmark-baseline.md:265` ADR-040, `:317` ADR-010/013).
- `docs/process/**/*.md`: 23 mentions, **0 broken** (e.g. `docs/process/governor-intake.md:150`, `docs/process/kutha-harness.md:50–51`).
- `AGENTS.md`: 10 mentions, **0 broken**. Root `README.md`: 2 mentions (ADR-092), 0 broken. `.kutha/**/*.md`: 56 mentions, 0 broken.
- Automated coverage gap: the governor's `refs_resolve` step scans `AGENTS.md`, `README.md`, `.kutha/**/*.md`, `.kutha/dictionaries/*.yaml`, `docs/process/*.md` (`.kutha/dictionaries/checks.yaml:1300–1306`) — **not** `docs/architecture/` and **not** `docs/ADR/`.
- Range handling is endpoints-only by design (`checks.yaml:1293–1294`), so the stale range `ADR-010–093` passes: the endpoints 010 and 093 exist. Occurrences: `AGENTS.md:44`, `docs/process/kutha-harness.md:29`, `.kutha/STATE.md:22`, `docs/ADR/README.md:157` (last one is inside the patch scope, D1).

### 5. Honeycomb `depends_on`

- 91 edges, 0 unresolved, 0 self-loops, 0 cycles (DFS three-colour), 0 isolated cells.
- Longest chain (depth 9): `ADR-071 -> ADR-070 -> ADR-043 -> ADR-041 -> ADR-040 -> ADR-014 -> ADR-010 -> ADR-002 -> ADR-001 -> ADR-000`.
- The YAML `depends_on` set equals the `- Depends on:` line in the ADR body for all 30 honeycomb cells (e.g. `docs/ADR/ADR-094-workspace-shape-as-shipped.md:15` = `[ADR-000 (D6), ADR-020, ADR-022]` = `honeycomb.yaml:720`). The three spine files ADR-000/001/002 have no `Depends on:` line; the ledger nevertheless declares `ADR-001 <- ADR-000` and `ADR-002 <- ADR-000, ADR-001` (`honeycomb.yaml:131`, `:142`), which matches the idea-stack diagram at `docs/ADR/README.md:9–10`. Informational, not a defect.
- `depends_on` and `Related Decisions` are different relations and legitimately differ (ADR-094 `depends_on` 000/020/022 but cites 011 as related at `ADR-094:135`).

### 6. Dates

- 33/33 ADRs have a `## Date` heading and an ISO `YYYY-MM-DD` value (e.g. `docs/ADR/ADR-023-test-strategy-and-oracle-matrix.md:9`; `ADR-000:31–33` places `## Date` after its clarification block, `ADR-094:8–10` after `## Status`).
- 33/33 README date cells equal the file value; 0 mismatches; no future dates (max 2026-10-31 = ADR-094).

## (b) Defects and exact patch proposals

Proposals only — nothing here is applied. Patches are unified-diff-style hunks against HEAD `73e2c82`; line numbers are HEAD line numbers. README and `honeycomb.yaml` are the two files the task allows exact edit proposals for; everything else is listed as a proposal for the owner.

### D1 — `docs/ADR/README.md`: prose still says 32 files and stops the opened band at 093 (medium)

The ADR-094 commit added the table row and nothing else: `git show b353678 --stat` reports `docs/ADR/README.md | 1 +`, and the diff adds exactly the row now at `docs/ADR/README.md:93`. The five prose claims below were left at the pre-094 state, so the index contradicts itself (33 rows listed, "32 files").

| File:line | Current text (excerpt) | Problem |
|---|---|---|
| `docs/ADR/README.md:99` | `This index lists **32** ADR files` | 33 files exist and 33 rows are listed |
| `docs/ADR/README.md:101` | `## Status recheck (2026-10-23)` | recheck predates ADR-094 (2026-10-31) |
| `docs/ADR/README.md:103` | `32 files, 32 index rows.` | same count error |
| `docs/ADR/README.md:140` | `Opened 090–093` | ADR-094 is open at `docs/ADR/README.md:93` |
| `docs/ADR/README.md:157` | `(000–002, 010–093)` | ADR-094 is Proposed but excluded from the range |

```diff
--- a/docs/ADR/README.md
+++ b/docs/ADR/README.md
@@ -98,7 +98,7 @@
 
-This index lists **32** ADR files; all are **Proposed**.
+This index lists **33** ADR files; all are **Proposed**.
 
-## Status recheck (2026-10-23)
+## Status recheck (2026-10-31)
 
-32 files, 32 index rows. Every `## Status` is **Proposed**. None are Accepted, Deprecated, or Superseded. Honeycomb stays a map.
+33 files, 33 index rows. Every `## Status` is **Proposed**. None are Accepted, Deprecated, or Superseded. Honeycomb stays a map.
 
@@ -139,3 +139,3 @@
 | **080–089** | Security / tenancy | Opened 080–081 |
-| **090–099** | Vertical / packaging | Opened 090–093 |
+| **090–099** | Vertical / packaging | Opened 090–094. **ADR-094 is Space-primary** (`honeycomb.yaml` `axis: Space`) though numbered in this Vertical band |
 
@@ -156,3 +156,3 @@
 
-Honeycomb and vision ADRs (000–002, 010–093) are **Proposed**: decisions are named and ready to accept. **Accepted** requires an implemented runtime for that cell. P0 spike covers log/fold/quantum/replay/snapshot/CSR-lease/LFTJ-intersect/WAL-cousin/materializer trait plus SEG-01 (`KUTHASEG` + `manifest.json`) and an optional unwired `rocks-index` locator (`rocks_index::SegmentIndex`). Rocks **event store**, Cypher, HNSW, ABAC remain unaccepted.
+Honeycomb and vision ADRs (000–002, 010–094) are **Proposed**: decisions are named and ready to accept. **Accepted** requires an implemented runtime for that cell. P0 spike covers log/fold/quantum/replay/snapshot/CSR-lease/LFTJ-intersect/WAL-cousin/materializer trait plus SEG-01 (`KUTHASEG` + `manifest.json`) and an optional unwired `rocks-index` locator (`rocks_index::SegmentIndex`). Rocks **event store**, Cypher, HNSW, ABAC remain unaccepted.
 
```

Notes: `docs/ADR/README.md:140` follows the existing convention of naming band/axis mismatches for ADR-023 (`:133`) and ADR-043 (`:135`); ADR-094 is the third such case (`honeycomb.yaml:714` `axis: Space` inside the 090–099 band). If the owner prefers to keep `## Status recheck (2026-10-23)` as a historical snapshot, leave `:101` alone and replace `:103` with a dated line instead: `33 files, 33 index rows (recheck 2026-10-31).`

### D2 — `.kutha/dictionaries/honeycomb.yaml`: restage log misses the two 2026-10-31 edits and is out of order (low)

- `d430a14` flipped `capability: none -> named` on ten cells — HEAD lines `:152` (ADR-010), `:198` (012), `:316` (014), `:359` (021), `:447` (040), `:467` (041), `:545` (052), `:562` (060), `:587` (061), `:618` (070) — and added evidence names (`git show d430a14 -- .kutha/dictionaries/honeycomb.yaml`). The restage log says nothing about it.
- `b353678` appended the ADR-094 cell at `.kutha/dictionaries/honeycomb.yaml:713–722` and no restage line mentions it either.
- Ordering: the top block (lines 1–17) is newest-first (10-24, 10-23, 10-23, …), but the `2026-10-31` M177 line sits at line 4. The oldest restage line is 78.

```diff
--- a/.kutha/dictionaries/honeycomb.yaml
+++ b/.kutha/dictionaries/honeycomb.yaml
@@ -1,6 +1,8 @@
-# 2026-10-24 restage (M158 S01 fork then query: Runtime::diff glue Evidence on ADR-061): ADR-061; map/delivery/capability unchanged.
-# 2026-10-23 restage (M157 S01 hop workload: query_ir hops over existing CSR lease; leapfrog still off query path): ADR-040, ADR-041; map/delivery/capability unchanged.
-# 2026-10-23 restage (M156 S01 fork diff: fact-set diff_folds Evidence on ADR-061): ADR-061; map/delivery/capability unchanged.
-# 2026-10-31 restage (M177 C1b pack manifest: ADR-011/012 kept-prefix Pack* note): ADR-011, ADR-012; map/delivery/capability unchanged.
+# 2026-10-31 restage (ADR-094 workspace shape as shipped; map-only, capability none): ADR-094; other cells unchanged.
+# 2026-10-31 restage (C2 capability backfill: capability none -> named on ten cells; evidence names added): ADR-010, ADR-012, ADR-014, ADR-021, ADR-040, ADR-041, ADR-052, ADR-060, ADR-061, ADR-070; map/delivery unchanged.
+# 2026-10-31 restage (M177 C1b pack manifest: ADR-011/012 kept-prefix Pack* note): ADR-011, ADR-012; map/delivery/capability unchanged.
+# 2026-10-24 restage (M158 S01 fork then query: Runtime::diff glue Evidence on ADR-061): ADR-061; map/delivery/capability unchanged.
+# 2026-10-23 restage (M157 S01 hop workload: query_ir hops over existing CSR lease; leapfrog still off query path): ADR-040, ADR-041; map/delivery/capability unchanged.
+# 2026-10-23 restage (M156 S01 fork diff: fact-set diff_folds Evidence on ADR-061): ADR-061; map/delivery/capability unchanged.
 # 2026-10-23 restage (M155 S01 checkpoint cut: Snapshot.behaviors exact Horn lineage): ADR-012; map/delivery/capability unchanged.
 # 2026-10-23 restage (M155 S01 checkpoint cut: fork_at fail-closed, migrate refuse Checkpoint): ADR-012, ADR-060, ADR-061; map/delivery/capability unchanged.
```

The two new lines name only what the commits changed, matching the existing phrasing (`map/delivery…`). Optionally the split log (lines 1–17 above `schema:`, lines 25+ below the header comments) can be merged later; that is cosmetic and not required for consistency.

### D3 — ADR-070 freeze state conflicts with the STATE thaw map (needs an owner/harness decision; not an index edit)

| Evidence | Claim |
|---|---|
| `.kutha/dictionaries/honeycomb.yaml:616` | ADR-070 cell `delivery: frozen` |
| `.kutha/dictionaries/honeycomb.yaml:617` | `freeze_as: Cypher` |
| `.kutha/STATE.md:37` | `\| Cypher/GPML parser \| Thawed (M003a leased 2026-10-30): read-only subset parser; M003b binder swap stays gated on C2 \|` |
| `docs/ADR/README.md:124` | `\| 070 / 042 / 080 / 090 / 093 \| frozen by .kutha/STATE.md thaw map \| do not thaw from this note \|` |

Two separate problems:

1. **Frozen vs Thawed.** The ledger ties `freeze_as: Cypher` to a thaw-map row that now says *Thawed*. The governor cannot see this: the `freeze_as` step only checks that the needle appears somewhere in `.kutha/STATE.md` (`.kutha/dictionaries/checks.yaml:1147–1155`), and "Cypher" does appear — inside a `Thawed` row.
2. **Incomplete frozen list in README.** `honeycomb.yaml` has 7 frozen cells — ADR-042, ADR-050, ADR-070, ADR-080, ADR-081, ADR-090, ADR-093 (`freeze_as` lines `:492`, `:515`, `:617`, `:645`, `:657`, `:669`, `:708`) — while `docs/ADR/README.md:124` lists only `070 / 042 / 080 / 090 / 093`, omitting ADR-050 and ADR-081.

This is a map-honesty question, not a typo, so the audit does not choose a side. Two mutually exclusive proposals:

- **Option A — the cell stays frozen (the cell is the full Cypher/AS OF surface, not the parser).** Then clarify the two prose sites: replace `docs/ADR/README.md:124` with the row `| 042 / 050 / 080 / 081 / 090 / 093 | frozen by .kutha/STATE.md thaw map (ADR-070's read-only parser item is marked Thawed at .kutha/STATE.md:37; the cell covers the full surface) | do not thaw from this note |` (the pipes are that table row's separators), and add the comment `# M003a leased 2026-10-30: read-only parser thawed; cell covers the full surface.` above `freeze_as: Cypher` at `.kutha/dictionaries/honeycomb.yaml:617`.
- **Option B — the cell is no longer frozen.** Then `honeycomb.yaml:616` `delivery: frozen` -> `delivery: spike` and drop `freeze_as` at `:617`; no README frozen row change is needed for 070, but the 050/081 omission above still applies.

### D4 — stale `010–093` ranges outside the patch scope (proposals only; three files)

| File:line | Old | New |
|---|---|---|
| `AGENTS.md:44` | `honeycomb cells (ADR-010–093, all **Proposed**)` | `honeycomb cells (ADR-010–094, all **Proposed**)` |
| `docs/process/kutha-harness.md:29` | `Honeycomb ADR-010–093 is a **map**.` | `Honeycomb ADR-010–094 is a **map**.` |
| `.kutha/STATE.md:22` | `\| L_map \| ADR-000–002 and 010–093 are **Proposed** \|` | `\| L_map \| ADR-000–002 and 010–094 are **Proposed** \|` |

`.kutha/STATE.md` is the harness lease file and was not touched; its l_map row is a controller edit. These three pass the governor today because ranges are checked by endpoints only (`.kutha/dictionaries/checks.yaml:1293–1294`).

### D5 — ADR-body references to other projects' ADR numbers (informational; ADR text proposals only)

Kutha and two neighbouring projects all use `ADR-NNN`. 18 mentions inside `docs/ADR/` name ids with no Kutha file; all are intentional (Appendix A). Only three occurrences omit the other project's name on the same line, so only those are worth labelling:

| File:line | Old | New |
|---|---|---|
| `docs/ADR/ADR-093-scientific-archive-pack.md:44` | `(ADR-046-compatible)` | `(daily-archive ADR-046-compatible)` |
| `docs/ADR/ADR-093-scientific-archive-pack.md:246` | `(rejected in ADR-058)` | `(rejected in daily-archive ADR-058)` |
| `docs/ADR/ADR-093-scientific-archive-pack.md:252` | `already motivational in ADR-058` | `already motivational in daily-archive ADR-058` |

The other two groups need no edit: `docs/ADR/ADR-000-kutha-hybrid-architecture-research.md:377–383` sits under `**Superseded numbering (2026-08-16):**` at `:371`, and `:412–414` sit under `### Related Samyama ADRs (borrow / contrast)` at `:410`.

Template variance, recorded and not proposed for change: ADR-023 uses `## Decisions` (`:25`) and `## Open questions` (`:130`) where the README template says `## Decision` (`docs/ADR/README.md:42`) and `## Open Research Questions` (`:45`); ADR-094 has no `Open Research Questions` section and is the only ADR whose `## Status` value wraps to a second line (`docs/ADR/ADR-094-workspace-shape-as-shipped.md:5–6`). Governor status parsing is unaffected (`scripts/kutha_gov/kinds.py:489–492` matches `**Proposed**` on the value's first line).

### Clean areas (no defect)

- 33/33 file↔row↔cell coverage, all links and paths resolve, ids unique, no phantom rows.
- 33/33 status, title and date matches between README and files.
- 0 unresolved `Related Decisions` citations; 0 unresolved `depends_on` edges; 0 dependency cycles; 0 self-references.
- 0 broken ADR mentions in `docs/architecture/`, `docs/process/`, `AGENTS.md`, root `README.md`, `.kutha/**/*.md`.

## (c) Verdict

The ADR index is **structurally healthy and internally consistent at the record level, and stale at the prose level**. All 33 files have exactly one README row and one `honeycomb.yaml` cell, every link and `path` resolves to a real file, ids are unique, every status/title/date cell matches its source file, all 91 `depends_on` edges resolve into an acyclic graph of depth 9, and all 154 `Related Decisions` citations point at existing ADRs (76 of them one-way, which is normal map behaviour, not an error). Nothing needs a structural repair, and the governor's ledger checks already cover most of these invariants. What the audit does find is decay after the one-day-old ADR-094 landing: five prose lines in `docs/ADR/README.md` still assert 32 files and an `010–093` range, the `honeycomb.yaml` restage log records neither the ADR-094 cell nor the ten-cell capability backfill (and has one date out of order), and one semantic question is now open — ADR-070 remains `delivery: frozen` with `freeze_as: Cypher` while `.kutha/STATE.md:37` marks the Cypher parser thawed, a conflict the governor's needle-only check cannot detect. Fix D1 and D2 to make the index truthful again; decide D3 before the next milestone cites ADR-070's status. Until then, trust the tables and distrust the counts.

## Appendix A — the 18 mentions with no Kutha ADR file

| # | File:line | Mention | Classification |
|---|---|---|---|
| 1 | `docs/ADR/ADR-000-kutha-hybrid-architecture-research.md:377` | ADR-003 | Historical remap table (ADR-000:371 says the old 001–012 sketch numbering is superseded; the row's right column names the live cell) |
| 2 | `docs/ADR/ADR-000-kutha-hybrid-architecture-research.md:378` | ADR-004 | Historical remap table (ADR-000:371 says the old 001–012 sketch numbering is superseded; the row's right column names the live cell) |
| 3 | `docs/ADR/ADR-000-kutha-hybrid-architecture-research.md:379` | ADR-005 | Historical remap table (ADR-000:371 says the old 001–012 sketch numbering is superseded; the row's right column names the live cell) |
| 4 | `docs/ADR/ADR-000-kutha-hybrid-architecture-research.md:380` | ADR-006 | Historical remap table (ADR-000:371 says the old 001–012 sketch numbering is superseded; the row's right column names the live cell) |
| 5 | `docs/ADR/ADR-000-kutha-hybrid-architecture-research.md:381` | ADR-007 | Historical remap table (ADR-000:371 says the old 001–012 sketch numbering is superseded; the row's right column names the live cell) |
| 6 | `docs/ADR/ADR-000-kutha-hybrid-architecture-research.md:382` | ADR-008 | Historical remap table (ADR-000:371 says the old 001–012 sketch numbering is superseded; the row's right column names the live cell) |
| 7 | `docs/ADR/ADR-000-kutha-hybrid-architecture-research.md:383` | ADR-009 | Historical remap table (ADR-000:371 says the old 001–012 sketch numbering is superseded; the row's right column names the live cell) |
| 8 | `docs/ADR/ADR-000-kutha-hybrid-architecture-research.md:412` | ADR-006 | External: Samyama ADRs (section heading ADR-000:410 “Related Samyama ADRs”) |
| 9 | `docs/ADR/ADR-000-kutha-hybrid-architecture-research.md:413` | ADR-015 | External: Samyama ADRs (section heading ADR-000:410 “Related Samyama ADRs”) |
| 10 | `docs/ADR/ADR-000-kutha-hybrid-architecture-research.md:414` | ADR-016 | External: Samyama ADRs (section heading ADR-000:410 “Related Samyama ADRs”) |
| 11 | `docs/ADR/ADR-093-scientific-archive-pack.md:21` | ADR-046 | External: daily-archive ADRs |
| 12 | `docs/ADR/ADR-093-scientific-archive-pack.md:21` | ADR-058 | External: daily-archive ADRs |
| 13 | `docs/ADR/ADR-093-scientific-archive-pack.md:44` | ADR-046 | External: daily-archive ADRs |
| 14 | `docs/ADR/ADR-093-scientific-archive-pack.md:90` | ADR-058 | External: daily-archive ADRs |
| 15 | `docs/ADR/ADR-093-scientific-archive-pack.md:119` | ADR-046 | External: daily-archive ADRs |
| 16 | `docs/ADR/ADR-093-scientific-archive-pack.md:206` | ADR-046 | External: daily-archive ADRs |
| 17 | `docs/ADR/ADR-093-scientific-archive-pack.md:246` | ADR-058 | External: daily-archive ADRs |
| 18 | `docs/ADR/ADR-093-scientific-archive-pack.md:252` | ADR-058 | External: daily-archive ADRs |


## Appendix B — 76 one-way `Related Decisions` edges

Edge exists in A's `## Related Decisions` but not in B's; asymmetry is informational. The other 39 unordered pairs cite each other mutually (78 directed edges) and are omitted. 154 directed edges total.

| Citing cell | Cited cell | Citing file:line |
|---|---|---|
| ADR-001 | ADR-000 | `docs/ADR/ADR-001-kutha-overall-vision.md:69` |
| ADR-002 | ADR-000 | `docs/ADR/ADR-002-stca-paradigm.md:104` |
| ADR-010 | ADR-000 | `docs/ADR/ADR-010-event-log-runtime-quantum.md:182` |
| ADR-010 | ADR-001 | `docs/ADR/ADR-010-event-log-runtime-quantum.md:183` |
| ADR-011 | ADR-000 | `docs/ADR/ADR-011-lean-event-schema-lineage.md:136` |
| ADR-012 | ADR-040 | `docs/ADR/ADR-012-snapshots-tiers-vacuum.md:179` |
| ADR-012 | ADR-042 | `docs/ADR/ADR-012-snapshots-tiers-vacuum.md:180` |
| ADR-013 | ADR-000 | `docs/ADR/ADR-013-bitemporal-facts-invalidation.md:124` |
| ADR-013 | ADR-010 | `docs/ADR/ADR-013-bitemporal-facts-invalidation.md:125` |
| ADR-014 | ADR-002 | `docs/ADR/ADR-014-cascade-budgets-quantum-receipts.md:176` |
| ADR-014 | ADR-071 | `docs/ADR/ADR-014-cascade-budgets-quantum-receipts.md:181` |
| ADR-020 | ADR-002 | `docs/ADR/ADR-020-cargo-workspace-port-isolation.md:89` |
| ADR-020 | ADR-080 | `docs/ADR/ADR-020-cargo-workspace-port-isolation.md:90` |
| ADR-021 | ADR-014 | `docs/ADR/ADR-021-pack-plugin-lifecycle.md:82` |
| ADR-021 | ADR-040 | `docs/ADR/ADR-021-pack-plugin-lifecycle.md:82` |
| ADR-021 | ADR-050 | `docs/ADR/ADR-021-pack-plugin-lifecycle.md:82` |
| ADR-021 | ADR-090 | `docs/ADR/ADR-021-pack-plugin-lifecycle.md:82` |
| ADR-021 | ADR-093 | `docs/ADR/ADR-021-pack-plugin-lifecycle.md:82` |
| ADR-022 | ADR-021 | `docs/ADR/ADR-022-vertical-slice-module-conventions.md:77` |
| ADR-022 | ADR-090 | `docs/ADR/ADR-022-vertical-slice-module-conventions.md:77` |
| ADR-022 | ADR-093 | `docs/ADR/ADR-022-vertical-slice-module-conventions.md:77` |
| ADR-023 | ADR-000 | `docs/ADR/ADR-023-test-strategy-and-oracle-matrix.md:149` |
| ADR-023 | ADR-010 | `docs/ADR/ADR-023-test-strategy-and-oracle-matrix.md:150` |
| ADR-023 | ADR-013 | `docs/ADR/ADR-023-test-strategy-and-oracle-matrix.md:151` |
| ADR-023 | ADR-020 | `docs/ADR/ADR-023-test-strategy-and-oracle-matrix.md:152` |
| ADR-023 | ADR-060 | `docs/ADR/ADR-023-test-strategy-and-oracle-matrix.md:153` |
| ADR-023 | ADR-090 | `docs/ADR/ADR-023-test-strategy-and-oracle-matrix.md:154` |
| ADR-030 | ADR-002 | `docs/ADR/ADR-030-cui-budget-traits.md:90` |
| ADR-030 | ADR-014 | `docs/ADR/ADR-030-cui-budget-traits.md:90` |
| ADR-030 | ADR-043 | `docs/ADR/ADR-030-cui-budget-traits.md:90` |
| ADR-031 | ADR-014 | `docs/ADR/ADR-031-pack-scheduling-under-v.md:82` |
| ADR-031 | ADR-043 | `docs/ADR/ADR-031-pack-scheduling-under-v.md:82` |
| ADR-031 | ADR-080 | `docs/ADR/ADR-031-pack-scheduling-under-v.md:82` |
| ADR-040 | ADR-000 | `docs/ADR/ADR-040-materialization-plugin-protocol.md:146` |
| ADR-040 | ADR-010 | `docs/ADR/ADR-040-materialization-plugin-protocol.md:147` |
| ADR-041 | ADR-011 | `docs/ADR/ADR-041-csr-graphblas-hot-path.md:124` |
| ADR-041 | ADR-012 | `docs/ADR/ADR-041-csr-graphblas-hot-path.md:125` |
| ADR-042 | ADR-013 | `docs/ADR/ADR-042-hnsw-access-method-fence.md:96` |
| ADR-042 | ADR-050 | `docs/ADR/ADR-042-hnsw-access-method-fence.md:99` |
| ADR-050 | ADR-000 | `docs/ADR/ADR-050-meta-prompt-dictionaries.md:167` |
| ADR-050 | ADR-010 | `docs/ADR/ADR-050-meta-prompt-dictionaries.md:168` |
| ADR-050 | ADR-013 | `docs/ADR/ADR-050-meta-prompt-dictionaries.md:170` |
| ADR-050 | ADR-090 | `docs/ADR/ADR-050-meta-prompt-dictionaries.md:173` |
| ADR-050 | ADR-093 | `docs/ADR/ADR-050-meta-prompt-dictionaries.md:173` |
| ADR-052 | ADR-021 | `docs/ADR/ADR-052-genai-enrichment-pack.md:96` |
| ADR-052 | ADR-042 | `docs/ADR/ADR-052-genai-enrichment-pack.md:96` |
| ADR-052 | ADR-050 | `docs/ADR/ADR-052-genai-enrichment-pack.md:96` |
| ADR-060 | ADR-010 | `docs/ADR/ADR-060-strict-replay.md:150` |
| ADR-060 | ADR-012 | `docs/ADR/ADR-060-strict-replay.md:150` |
| ADR-060 | ADR-014 | `docs/ADR/ADR-060-strict-replay.md:150` |
| ADR-061 | ADR-012 | `docs/ADR/ADR-061-fork-and-diff.md:109` |
| ADR-061 | ADR-013 | `docs/ADR/ADR-061-fork-and-diff.md:109` |
| ADR-062 | ADR-010 | `docs/ADR/ADR-062-regimes-gated-loop.md:83` |
| ADR-062 | ADR-014 | `docs/ADR/ADR-062-regimes-gated-loop.md:83` |
| ADR-062 | ADR-050 | `docs/ADR/ADR-062-regimes-gated-loop.md:83` |
| ADR-070 | ADR-013 | `docs/ADR/ADR-070-cypher-temporal-as-of.md:110` |
| ADR-070 | ADR-041 | `docs/ADR/ADR-070-cypher-temporal-as-of.md:110` |
| ADR-070 | ADR-050 | `docs/ADR/ADR-070-cypher-temporal-as-of.md:110` |
| ADR-071 | ADR-042 | `docs/ADR/ADR-071-hybrid-retrieval-composition.md:84` |
| ADR-071 | ADR-043 | `docs/ADR/ADR-071-hybrid-retrieval-composition.md:84` |
| ADR-080 | ADR-013 | `docs/ADR/ADR-080-abac-multi-tenant-slices.md:102` |
| ADR-081 | ADR-010 | `docs/ADR/ADR-081-agent-sandbox.md:83` |
| ADR-081 | ADR-021 | `docs/ADR/ADR-081-agent-sandbox.md:83` |
| ADR-090 | ADR-000 | `docs/ADR/ADR-090-legal-reference-pack.md:249` |
| ADR-090 | ADR-002 | `docs/ADR/ADR-090-legal-reference-pack.md:251` |
| ADR-091 | ADR-012 | `docs/ADR/ADR-091-rvf-portable-capsules.md:83` |
| ADR-091 | ADR-014 | `docs/ADR/ADR-091-rvf-portable-capsules.md:83` |
| ADR-092 | ADR-000 | `docs/ADR/ADR-092-naming-and-license.md:82` |
| ADR-092 | ADR-001 | `docs/ADR/ADR-092-naming-and-license.md:83` |
| ADR-093 | ADR-000 | `docs/ADR/ADR-093-scientific-archive-pack.md:259` |
| ADR-093 | ADR-002 | `docs/ADR/ADR-093-scientific-archive-pack.md:261` |
| ADR-093 | ADR-013 | `docs/ADR/ADR-093-scientific-archive-pack.md:264` |
| ADR-093 | ADR-090 | `docs/ADR/ADR-093-scientific-archive-pack.md:262` |
| ADR-094 | ADR-011 | `docs/ADR/ADR-094-workspace-shape-as-shipped.md:135` |
| ADR-094 | ADR-020 | `docs/ADR/ADR-094-workspace-shape-as-shipped.md:133` |
| ADR-094 | ADR-022 | `docs/ADR/ADR-094-workspace-shape-as-shipped.md:134` |


## Appendix C — method, parsing constants, and limits

- Commands used: `git log/show/blame`, `rg`, and Python text parsing plus `yaml.safe_load` over `.kutha/dictionaries/honeycomb.yaml`. No governor run, no cargo, no `graph index`, no MCP call.
- `status` value = first non-blank line after a line equal to `## Status`, with `*`, `_`, backticks stripped; compared by first token (so `**Proposed** (…)` matches `Proposed`). 33/33.
- `title` = text after the first `: ` in the H1 (`# ADR-NNN: Title`); 33/33 equal README titles.
- `## Related Decisions` = the single level-2 heading with that exact text; body ends at the next level-2 heading. All 33 files have exactly one.
- Reference regex `(?<![\dA-Za-z-])ADR-(\d{3})(?!\d)`: rejects `ADR-0009` and `ADR-0100`. Shorthand expanded: slash lists (`ADR-010 / 012 / 014`) and en-dash ranges (`ADR-040–042` -> 040, 041, 042). The governor treats ranges as endpoints only (`.kutha/dictionaries/checks.yaml:1293–1294`); this audit expands ranges inclusively, which can only add edges, and the one range inside `Related Decisions` (`docs/ADR/ADR-043-hybrid-query-planner.md:96`) does not change any conclusion because 041 is already listed in that file's `Related Decisions` at `:97`.
- Lines containing `daily-archive` or `law-nexus` were treated as external and excluded from the Kutha citation graph (`docs/ADR/ADR-093-scientific-archive-pack.md:265`, `docs/ADR/ADR-090-legal-reference-pack.md:254`).
- `depends_on` cycles were checked by DFS three-colour over the 33-cell graph; chains recomputed with memoised depth.
- Not verified here (out of task scope): whether capability evidence test names actually exist in `crates/` (governor `rust_test_asserts`, `.kutha/dictionaries/checks.yaml:1163–1173`); whether frozen-cell `freeze_as` values still describe intent (D3); any code-level claim. Line numbers are HEAD `73e2c82` and drift with later commits.
