# eCFR third-edition reconnaissance — what a third normative edition would require

Date: 2026-10-31. Author: task H researcher (AllSpark child agent, read-only on the repo).
Scope: research note, not an ADR, not a legal pack, not legal advice. No product change proposed.
Lease state at the time of writing: `Active=None`; nothing here asks for a product write.

Premise correction up front: the task brief said the fixture covers "2015-12-18 and 2021-01-01".
It does not. The committed fixture is **2015-12-18, 2017-01-23, 2017-03-14** — see
`crates/kutha-runtime/tests/fixtures/ecfr/README.md:11`, `tools/fetch_ecfr_fixture.py:24`,
`crates/kutha-runtime/tests/m167_ecfr_pit.rs:30`. `2021-01-01` appears nowhere in the fixture; it is
one of the dates this note tested as a *candidate* third edition, and it was rejected (see §3, §5).

---

## 0. Evidence base (every fetch, status, and field read)

All fetches were made on 2026-10-31 by the author. Raw response bodies were kept in the gitignored
scratch directory `tmp/ecfr-third-edition-recon/` (not committed).

| # | Request | HTTP | Bytes | Fields read |
|---|---------|------|-------|-------------|
| E1 | `GET https://www.ecfr.gov/api/versioner/v1/versions/title-1.json` | 200 | 102147 | `meta.title=1`, `meta.result_count=422`, `meta.latest_amendment_date=2022-12-29`, `meta.latest_issue_date=2026-08-10`; all 47 `content_versions` rows with `part=304` (`date`, `amendment_date`, `identifier`, `name`, `substantive`, `removed`, `type`) |
| E2 | `GET https://www.federalregister.gov/api/v1/documents.json?conditions[cfr][title]=1&conditions[cfr][part]=304&per_page=100&order=newest&fields[]=document_number&fields[]=title&fields[]=publication_date&fields[]=effective_on&fields[]=type&fields[]=citation&fields[]=html_url` | 200 | 987 | `count=3`, `total_pages=1`; all 3 result rows |
| E3 | same as E2 + `conditions[publication_date][gte]=2021-01-01` | 200 | 97 | `count=0` |
| E4 | `GET https://www.federalregister.gov/api/v1/documents/2017-00891.json?fields[]=…` | 200 | — | `type=Rule`, `action="Direct final rule."`, `publication_date=2017-01-23`, `effective_on=2017-03-14`, `citation=82 FR 7631`, `cfr_references=[{title:1, part:304}]`, `correction_of=null`, `corrections=[]` |
| E5 | `GET https://www.ecfr.gov/api/versioner/v1/full/{date}/title-1.xml?part=304&section={s}` for `s ∈ {304.1, 304.2, 304.10}` × `date ∈ {2015-12-18, 2017-01-23, 2017-03-14, 2021-01-01, 2022-12-29}` | 200 (after retries, see §2c) | 797–2389 | raw `DIV8`/`HEAD`/`P`/`XREF`/`CITA` XML; text via the committed `extract()` |
| E6 | `GET …/full/2015-12-17/…` and `…/full/2011-06-01/…` (304.1) | **404** | 36 | body `{"error":"No matching content found."}` |
| E7 | second, independent `GET …/full/2017-03-14/…section=304.1` | 200 | 2295 | sha256 `63d846e112c9b8be78bc74428f2e05d19d40efc405d1bcaaf03e952e79dba140`, equal to the first fetch |
| E8 | `GET https://www.govinfo.gov/about` | 200 | — | public-domain notice text (quoted in §4) |
| E9 | `GET https://www.ecfr.gov/about`, `…/reader-aids`, `…/developers/documentation/api/v1` | 200 / 200 / 200 | 10595 / — / 10595 | no occurrence of "public domain", "copyright", or "17 U.S.C" in server-rendered HTML |
| E10 | `GET …/versions/title-{7,12,21,40}.json` (candidate sweep) | 200 each | — | `meta.per_page=1000`, `total_pages` 33/19/16/61; section+date sets (page 1 only) |
| E11 | `GET …/full/{date}/title-12.xml?part=1003&section=1003.3` for `2017-01-01, 2018-01-01, 2019-10-29, 2020-01-01, 2022-01-01` | 200 each | 1442–9243 | raw XML + extracted text (candidate, §3) |
| E12 | `GET …/full/{date}/title-21.xml?part=1100&section=1100.5` for `2017-01-09, 2017-03-20, 2018-03-19` | 200 each | 349–1293 | raw XML + extracted text (placeholder finding, §5) |
| E13 | `GET https://www.federalregister.gov/api/v1/documents.json?conditions[cfr][title]=12&conditions[cfr][part]=1003&per_page=20&order=newest` | 200 | — | `count=35`, 20 rows read (`document_number`, `publication_date`, `effective_on`, `type`, `citation`, `title`) |
| E14 | bare `curl` of a versioner URL without `Accept-Encoding` | **406** | 171 | `{"error":{"message":"This endpoint requires response compression. Send an Accept-Encoding header that permits compression.","supportCode":11}}` |

---

## 1. Are there normative changes to 1 CFR part 304 after 2021-01-01, or between 2017 and 2021?

**No — verified negative, from two independent sources.**

### 1a. eCFR Versioner list (E1)

Part 304 has 47 `content_versions` rows. The distinct dates are exactly three:
`2015-12-18`, `2017-01-23`, `2017-03-14`. **The maximum `date` for any part-304 row is 2017-03-14.**
No row exists between 2017-03-14 and 2021-01-01, and none after 2021-01-01.

Per-section shape (E1):

| section | dates | note |
|---------|-------|------|
| 304.1 | 2015-12-18, 2017-01-23, 2017-03-14 | 3 rows |
| 304.2 | 2015-12-18, 2017-01-23, 2017-03-14 | `name` changes at 2017-03-14: "Public reading room" → "Proactive disclosures" |
| 304.10 | 2015-12-18, 2017-01-23, 2017-03-14 | 3 rows |
| 304.3, 304.5, 304.6, 304.9 | 2015-12-18, 2017-01-23, 2017-03-14 | each also has a **second** 2017-03-14 row with `substantive=false` |
| 304.7 | 2015-12-18 | two rows, same date, `substantive` true + false |
| 304.4, 304.11, 304.20–304.34 | 2015-12-18 only | single row each |

Two consequences for the fixture's design, both measurable in E1:

- a *duplicate row on one date* (`substantive=false` next to `true`) is not a second edition — the
  same date cannot produce a second text;
- the `substantive` boolean is **not** an effect classifier: it is `true` for the 2017-01-23 row of
  every part-304 section, although that row is citation-only (proven in §1c). Do not build a rule on
  this field.

### 1b. Federal Register API (E2, E3, E4)

Documents affecting 1 CFR 304, newest first — `count=3`, i.e. the whole history:

| publication_date | effective_on | type | document_number | citation | title |
|---|---|---|---|---|---|
| 2017-01-23 | 2017-03-14 | Rule | **2017-00891** | 82 FR 7631 | Revisions to Freedom of Information Act Regulations |
| 2011-04-05 | 2011-04-05 | Rule | 2011-7976 | 76 FR 18635 | Disclosure of Records or Information |
| 2011-01-11 | — | Proposed Rule | 2011-146 | 76 FR 1542 | Disclosure of Records or Information |

With `publication_date >= 2021-01-01` the same query returns `count=0` (E3). Document 2017-00891
carries `action="Direct final rule."`, `correction_of=null`, `corrections=[]`, and
`cfr_references=[{title:1, part:304}]` (E4) — so the 2017-01-23 → 2017-03-14 gap is the standard
direct-final-rule publication/effect gap, and there is no later correction document.

Between 2017 and 2021: nothing. After 2021-01-01: nothing. Title 1 as a whole was amended later
(`meta.latest_amendment_date=2022-12-29`, E1) — that is a different part, not 304.

### 1c. Why the 2017-01-23 row is citation-only (re-check, as asked)

Raw XML diff, `n=1` unified diff over the whole document, 2015-12-18 vs 2017-01-23 (E5). Each of the
three sections differs by **exactly one added line**:

```
304.1   <XREF ID="20170123" REFID="1">Link to an amendment published at 82 FR 7632, Jan. 23, 2017.</XREF>
304.2   <XREF ID="20170123" REFID="2">Link to an amendment published at 82 FR 7632, Jan. 23, 2017.</XREF>
304.10  <XREF ID="20170123" REFID="8">Link to an amendment published at 82 FR 7635, Jan. 23, 2017.</XREF>
```

Nothing else changes — no `HEAD`, no `P`. `extract()` keeps `HEAD` and `P` only
(`tools/fetch_ecfr_fixture.py:43-60`, `for p in div.iter("P")` at `:54`), so the operative text is
unchanged. This reproduces the earlier finding in `docs/architecture/ecfr-edition-ambiguity.md:17-24`
and the fixture note at `crates/kutha-runtime/tests/fixtures/ecfr/README.md:31`.

An eCFR-only detail the earlier note did not record: the 2017-03-14 XML also carries a `CITA`
element with the amendment credit `[82 FR 7632, Jan. 23, 2017]`, which `extract()` also drops. So the
committed text has no amendment history at all — "identical" in the README means identical after both
`XREF` and `CITA` are dropped.

---

## 2. Can each candidate date be obtained legitimately, and does the committed extractor reproduce it?

### 2a. Legality / licensing

- 1 CFR and the Federal Register are US Government works. The statutory basis is 17 U.S.C. §105;
  GPO states it for GovInfo content: "In general, GovInfo documents fall under Title 17, Section 105,
  United States Code … The intent of the section is to place in the public domain all work of the
  United States Government" (E8, `https://www.govinfo.gov/about`).
- **The eCFR site itself did not state this in anything I could read.** `/about`, `/reader-aids`, and
  `/developers/documentation/api/v1` (E9) contain no "public domain", "copyright", or "17 U.S.C"
  string in server-rendered HTML. Caveat, stated as a caveat: those pages are client-rendered; my
  text extraction saw 1182 characters of text from `/reader-aids`, so this is *not proof of absence*,
  only proof that I could not read such a statement. The fixture README currently says "the eCFR site
  describes the content as public domain" (`README.md:20`) — that sentence is **not** backed by
  anything I could fetch and should be replaced by the statutory/GPO citation (§4).
- No API key, registration, or click-through terms are required for the versioner endpoints used
  (E1, E5 returned content without credentials). The API requires `Accept-Encoding: gzip` (E14).

### 2b. Determinism of the committed extractor

I re-ran the committed extractor's own `fetch`/`extract` functions (imported from
`tools/fetch_ecfr_fixture.py`, not re-implemented) against the committed URLs, and compared to the
committed files:

| file | committed bytes | re-extracted chars | identical |
|---|---|---|---|
| `304.1_2015-12-18.txt` | 2049 | 2028 | yes |
| `304.1_2017-01-23.txt` | 2049 | 2028 | yes |
| `304.1_2017-03-14.txt` | 2006 | 1985 | yes |
| `304.2_2015-12-18.txt` | 545 | 544 | yes |
| `304.2_2017-01-23.txt` | 545 | 544 | yes |
| `304.2_2017-03-14.txt` | 1072 | 1071 | yes |
| `304.10_2015-12-18.txt` | 805 | 800 | yes |
| `304.10_2017-01-23.txt` | 805 | 800 | yes |
| `304.10_2017-03-14.txt` | 806 | 801 | yes |

9 of 9 byte-identical on 2026-10-31, two years after the original fetch. A second independent fetch of
`304.1` at 2017-03-14 produced the same sha256 (E7). Determinism claim: **reproducible content, given
the same URL set**.

Determinism limits that a third edition would inherit:

- **Rate limiting is real.** 5 of 15 requests returned **HTTP 429** when issued 6-wide; all succeeded
  on sequential retry. `fetch_one` already retries 5 times with 2/4/6/8 s sleeps on `URLError`
  (`tools/fetch_ecfr_fixture.py:63-76`) and `HTTPError` is a `URLError` subclass, so the retry path
  covers 429 — but the run is not fast, and `--check` mode does not touch the network at all.
- **Compression is mandatory** (E14); a client that omits `Accept-Encoding` gets 406, not content.
- **Left edge is 2015-12-18.** `2015-12-17` and `2011-06-01` return 404
  `{"error":"No matching content found."}` (E6). So the 2011 rule (2011-7976) is *not* reachable
  through the historical API — the earliest obtainable part-304 text is 2015-12-18.
- **Extraction silently drops non-`HEAD`/`P` text.** Part 304 XML contains only
  `DIV8, HEAD, P, I, XREF, CITA` (E5 tag inventory), so nothing operative is lost today. A section
  that used `FP`, `EXTRACT`, `NOTE`, or `GPOTABLE` would lose text with no error. Any candidate
  section needs a tag inventory check before commit — this is a required pre-commit step, not a
  hypothetical.

### 2c. Per-candidate verdict

| candidate | obtainable | extractor reproduces | distinct operative text |
|---|---|---|---|
| 2021-01-01 (part 304) | yes, 200 | yes — identical to 2017-03-14 (1985 / 1071 / 801 chars) | **no** |
| 2022-12-29 (part 304) | yes, 200 | yes — identical to 2017-03-14 | **no** |
| any date 2018-01-01…2020-12-31 (per the earlier note) | — | — | no (same text) |
| pre-2015-12-18 (part 304) | **no**, 404 | n/a | n/a |
| 12 CFR 1003.3, 2018-01-01 / 2020-01-01 | yes, 200 | yes (4504 / 8564 chars) | yes (§3) |

---

## 3. What a third edition would add to the AS OF probe

Probe shape today (`crates/kutha-runtime/tests/m167_ecfr_pit.rs`): `SECTIONS` at `:9-28`,
`DATES = [20151218, 20170123, 20170314]` at `:30`, seven `AS_OF` cuts at `:32-40`
(`20151217 → None`, `20151218 → 0`, `20170122 → 0`, `20170123 → 1`, `20170313 → 1`,
`20170314 → 2`, `20180101 → 2`), the two `assert_eq!`/`assert_ne!` equality asserts and the
`BTreeSet::len() == 2` count assert at `:146-159` (failure message "fixture must keep exactly two
distinct editions", `:158`).

What a genuinely third text buys:

1. **A second VT interval closure.** Today the last row is open-ended (`valid_to: None`) and the
   newest cut (`20180101`) can only prove "still edition 2". A third edition adds a closed interval
   `2017-03-14 → D3` and one open tail, so `as_of` must pick the right one of *three* rows. That is
   the first time a wrong `valid_to` on a middle row is observable.
2. **Two new discriminating cuts** — `D3-1` and `D3` — and they can be the *same day* pair that the
   existing fixture uses (`20170313`/`20170314`), which is what makes the day-scale VT probe sharp.
3. **A stronger knowledge/force separation.** `norm_view` currently tests exactly one asymmetry
   (`late force 20170314` × `early wall 20151218` must hide edition 2, `:169-186`). With three
   editions the same test becomes non-degenerate: there is a *middle* edition that a wall-time cut
   can hide while a later one is visible only after its own wall. `as_known_on` similarly gains a
   third "must be absent" edge (`:196-214`).
4. **The `len() == 2` assert must become `3`.** This is the honest bookkeeping change: the count
   assert is what catches a lost edition, and it is currently pinned to two.

What it does **not** buy in part 304: nothing, because no third text exists (§1, §6). Adding
`2021-01-01` to `DATES` would make the array length 4 while the count assert stays `2`; the test
would still pass and the probe would gain zero discrimination — the "fabricated third edition" that
`docs/architecture/ecfr-edition-ambiguity.md:47-49` forbids, in its most tempting form (a date that
looks like a clean millennial boundary and returns HTTP 200).

**The only in-scope substitute** (verified, not recommended for a fixture write today): 12 CFR 1003.3
("Exempt institutions…", CFPB Regulation C) has four distinct extracted texts across five dates —
`2017-01-01` 1065 chars, `2018-01-01` 4504, `2019-10-29` 4504, `2020-01-01` 8564, `2022-01-01` 8565
(E11). Its XML tag inventory is `DIV8, HEAD, P, I, XREF, CITA` only, so the committed extractor works
on it unchanged apart from the hard-coded `title-1 … part=304` in `BASE`
(`tools/fetch_ecfr_fixture.py:25`). Three effective-date states (`2018-01-01`, `2020-01-01`,
`2022-01-01`) give 4504 / 8564 / 8565 — three distinct texts, but note the third boundary is a
one-number change (asset-size exemption threshold), i.e. the same *kind* of thin boundary as
304.10's one-citation change (`README.md:27`). The wider pair `2017-01-01` / `2018-01-01` /
`2020-01-01` (1065 / 4504 / 8564) is a stronger probe but reaches back to a date whose FR driver I
did not verify (see §7).

Cross-check that the candidate is a real corpus, not a lucky pair: FR documents affecting 12 CFR 1003
number 35 (E13); the ones adjacent to the chosen boundaries include `2019-22561` (published
2019-10-29, effective 2020-01-01, 84 FR 57946), `2021-27899` (2021-12-23 → 2022-01-01, 86 FR 72818),
`2018-19244` (2018-09-07 → 2018-09-07, 83 FR 45325), `2018-28373` (2019-01-31, 84 FR 513).

And a *second independent confirmation* of the publication/effect gap, in a different title:
1003.3 at `2019-10-29` extracts to exactly the `2018-01-01` text (4504 chars), with 3 `XREF` notes in
the raw XML. The 2019-10-29 publication added notes, not text; the text moved on 2020-01-01. So the
citation-only pattern is a property of the eCFR model, not a quirk of 1 CFR 304.

---

## 4. Licensing / attribution: what the fixture README must say if a third edition is added

Minimum additions, in the README's own voice (it already says "not a legal pack, not legal advice",
`README.md:1-6`):

1. **Statutory basis, not a site claim.** Replace "the eCFR site describes the content as public
   domain" (`README.md:20`) with: 17 U.S.C. §105; eCFR and Federal Register text are US Government
   works; GPO's public-domain notice is the citation (`https://www.govinfo.gov/about`); no copyright
   is claimed by this repository over the extracted text.
2. **Provenance per file.** For each of the N files: exact URL pattern
   (`https://www.ecfr.gov/api/versioner/v1/full/{date}/title-{T}.xml?part={P}&section={S}`), HTTP
   status 200, the fetch date (UTC), and a content hash (sha256 of the extracted text, or of the raw
   XML; raw XML stays out of git under `tmp/ecfr-raw/`).
3. **A statement of what "edition" means.** The date is the eCFR **amendment/effective** date, not
   the Federal Register publication date; when the two differ, the publication-date version carries
   only `XREF`/`CITA` notes and extracts identically to its predecessor. Cite one worked example
   (2017-00891: published 2017-01-23, effective 2017-03-14).
4. **A statement of what extraction drops.** `HEAD` and `P` only (`fetch_ecfr_fixture.py:43-60`);
   `XREF`, `CITA`, `I`-wrapped inline markup is flattened, notes are lost. "Identical" in the README
   means identical *extracted* text.
5. **Rate-limit and reproducibility note.** The API requires `Accept-Encoding: gzip`; 429 occurs
   under concurrency; retries are bounded; a re-fetch reproduces content but not timing.
6. **No-endorsement line.** eCFR/GPO/NARA have not reviewed this fixture; the section selection is
   ours.
7. **No legal-advice line** — already present; keep it.

If the edition comes from a different title, the README must also say which title/part/section set
and that the original 1 CFR 304 probe remains as-is (do not silently re-label the fixture).

---

## 5. "Do not" list

1. **`2017-01-23` is not an edition.** Re-checked today: raw XML differs from 2015-12-18 by exactly
   one `XREF` line per section (§1c); extracted text is byte-identical; no `HEAD`/`P` change. The
   Versioner's `substantive=true` on that row (E1) must not be read as a normative change.
2. **`2021-01-01` is not an edition.** HTTP 200 from the API, and it extracts to the 2017-03-14 text
   for all three sections (1985 / 1071 / 801 chars). A 200 response is not a text change.
3. **`2022-12-29` (title 1's latest amendment date) is not a part-304 edition.** Same extraction as
   2017-03-14; the amendment is in another part of title 1.
4. **Pre-`2015-12-18` dates do not exist in this API.** 404 `{"error":"No matching content found."}`
   for 2015-12-17 and 2011-06-01. The 2011 rule (76 FR 18635) is therefore unusable as a left-hand
   edition, however attractive it looks in the FR list.
5. **Placeholder editions.** eCFR can serve a section as a stub while an amendment is pending:
   21 CFR 1100.5 at `2017-01-09` is `<HEAD>§ 1100.5 xxx</HEAD>` plus one `XREF` note, and extracts to
   13 characters; at `2017-03-20` it is the same stub plus a second note, "This amendment was delayed
   until Mar. 21, 2017, at 82 FR 9501, Feb. 7, 2017." Only `2018-03-19` has real text (979 chars)
   (E12). Never take a third edition from a `xxx` stub, and never assume an announced effective date
   was honoured — delays are published as notes.
6. **Same-date duplicate rows.** 304.3, 304.5, 304.6, 304.9 each have two 2017-03-14 rows
   (`substantive` true and false), and 304.7 has two 2015-12-18 rows. Two rows ≠ two editions.
7. **Do not use `substantive` as an effect classifier.** Falsified by item 1.
8. **Do not take text from third-party annotated corpora** (Cornell LII, Justia, law-firm summaries,
   Wikipedia). They add editorial text, their own licence terms, and their own error surface; only
   eCFR versioner, the eCFR site, the Federal Register API, and GovInfo are acceptable text sources.
9. **Do not treat an error body as content.** 429 and 406 responses are short XML/JSON documents; a
   client that ignores status can "extract" an error message. `extract()` guards this partially by
   requiring `<DIV8` and `<HEAD>` (`fetch_ecfr_fixture.py:44-45`), which rejects the observed 429 and
   406 bodies — but the guard is a shape check, not a status check.
10. **Do not add a date to make the probe look stronger.** Keep
    `docs/architecture/ecfr-edition-ambiguity.md:47-49` and `README.md:37-38` as the ruling.

---

## 6. Sourcing plan and recommendation

**Recommendation: add no third edition to the 1 CFR part 304 fixture. Keep M167 as it is.**

Reasons, all verified above:

- part 304 has exactly two distinct operative texts reachable through the API (2015-12-18 and
  2017-03-14), and no reachable third state exists on either side — later dates return 200 with
  unchanged text, earlier dates return 404 (§1, §2c);
- the only part-304 "new" date in the window a reader would expect (`2021-01-01`) is a duplicate of
  2017-03-14, so it would satisfy `len() == 2` while being advertised as a third edition — the exact
  dishonesty the ambiguity note forbids;
- 1 CFR 456.12 does list three dates, but its third row is `removed: true` at 2017-10-20 (E1), i.e. a
  removal, not a third text, and extraction of a removed section would lose the section entirely.

If a genuinely three-state AS OF probe is later required, it is a **new corpus decision**, not a
fixture patch (this matches `docs/architecture/ecfr-edition-ambiguity.md:57-61`: an A1/A2 legal-pack
question). The cheapest verified candidate is 12 CFR 1003.3 with effective-date boundaries; ranked
options:

| option | dates | distinct texts | cost | risk |
|---|---|---|---|---|
| A. 12 CFR 1003.3, wide | 2017-01-01, 2018-01-01, 2020-01-01 | 1065 / 4504 / 8564 | new fixture dir + parameterised `BASE` (`fetch_ecfr_fixture.py:25`) + one new test | the 2017-01-01 driver is not yet mapped to an FR document (§7) |
| B. 12 CFR 1003.3, conservative | 2018-01-01, 2020-01-01, 2022-01-01 | 4504 / 8564 / 8565 | same | third boundary is a single-number change — thin discriminator |
| C. stay with part 304, keep two texts | — | 2028/1985, 544/1071, 800/801 | zero | probe never shows three states |

Recommendation among these: **C now**, and if the milestone truly needs three states, **A** after the
FR-document mapping is verified and after a decision is recorded in the GSD layer (which owns phase
planning) — not by editing `crates/`, which is out of scope for this task and has no lease.

What must be decided by someone other than the fixture author: whether a second corpus is worth a new
fixture at all, given that `I-legal-pack-not-next` is recorded as standing policy
(`docs/architecture/ecfr-edition-ambiguity.md:60-61`).

---

## 7. What I did not verify (stated so it is not mistaken for evidence)

- **FR document → boundary mapping for 12 CFR 1003.3.** I read the part-1003 document list (E13) and
  the versioner date list, but I did not prove which document produces each of the four distinct
  texts. The 2019-10-29 → 2020-01-01 pair is strongly supported (publication 2019-10-29, effective
  2020-01-01, count 35 documents, extracted text flipping on 2020-01-01), but that is an inference
  from dates, not a proven chain.
- **Census across titles.** The title 7/12/21/40 sweep (E10) read **page 1 of 33/19/16/61** pages of
  1000 rows each. It is a candidate finder, not a census of the CFR. Title 1's list (E1,
  `result_count=422`) was complete.
- **eCFR site terms.** `/about`, `/reader-aids`, and the API docs are client-rendered; my extracted
  text was 1182 characters. I found no copyright/public-domain statement, but I cannot call that
  absence proven (§2a).
- **Anything about ADR / honeycomb status.** Out of scope here; `.kutha/dictionaries/honeycomb.yaml`
  contains no `ecfr` entry, so this note makes no delivery or capability claim.
- **No Rust was built or run.** No cargo, no `graph index`. Claims about the probe are read from
  `crates/kutha-runtime/tests/m167_ecfr_pit.rs` source at the cited lines; I did not execute the test.
- **No `2021-01-01`-based product change was proposed.** The finding is a negative and needs no
  action.
