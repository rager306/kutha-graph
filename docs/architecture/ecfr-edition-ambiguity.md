# eCFR edition ambiguity — explained, not fabricated

Process note, not an ADR and not a legal pack. It records why the M167 fixture
has only one normative boundary and why adding a "third distinct edition" was
rejected.

## Observation

M167 commits three eCFR editions per section: 2015-12-18, 2017-01-23,
2017-03-14. Extracted text is identical for the first two and differs at the
third. That looked like a fixture weakness: only 2017-03-13/14 discriminates.

## Check against the source

`https://www.ecfr.gov/api/versioner/v1/versions/title-1.json` lists the three
amendment dates for 1 CFR 304.1, 304.2, 304.3, 304.5, 304.6, 304.8, 304.9, and
304.10. For every one of those sections, the raw XML at 2017-01-23 differs from
2015-12-18 by exactly one added element:

```xml
<XREF ID="20170123" REFID="3">Link to an amendment published at 82 FR 7632, Jan. 23, 2017.</XREF>
```

That is a citation-only amendment: the regulation text is unchanged, and the
note is not part of the operative text. `tools/fetch_ecfr_fixture.py` extracts
`HEAD` and `P` text only, so the note is correctly dropped. Extraction at
2017-03-14 shows the substantive rewrite (for example 304.3 gains a format
clause and renumbers the fee paragraph).

**Why the middle row exists at all.** Federal Register document 2017-00891
("Revisions to FOIA Regulations", 82 FR 7631) was *published* on 2017-01-23 and
*effective* on 2017-03-14. The eCFR version list therefore has a 2017-01-23
entry, but the operative text only changes on the effective date. So the
identical pair is not an extraction artifact or a fixture mistake: it is the
publication/effective-date gap, visible in the data.

Checked alternatives: 304.1, 304.2, 304.3, 304.5, 304.6, 304.8, 304.9, and
304.10 all collapse to the same two distinct texts at these dates.

Later dates add no further change. For 304.1, 304.3, 304.5, and 304.10 this was
checked by fetching raw XML at 2018-01-01, 2019-01-01, and 2020-01-01. For the
other four it follows from `versions/title-1.json`: their last content version
is 2017-03-14. The two evidence sources are named because they are not equal.

## Decision

Do **not** fabricate a third edition, and do not relabel the citation-only
amendment as a normative change. Keep the fixture honest:

- the equality asserts catch a *lost* and a *gained* edition; the `len == 2`
  assert restates them and cannot fail while they pass;
- the 2017-03-14 boundary is the discriminating pair;
- `as_known_on` still shows the 2017-01-23 row, because the log records that
  edition was published even though its operative text did not change.

## Consequence

The fixture cannot demonstrate three normative editions, and Gate GC / Arc A
must not cite it as if it did. A real three-edition corpus needs a section with
three substantive amendments, which is an A1/A2 legal-pack question, not a
fixture patch. `I-legal-pack-not-next` stays.
