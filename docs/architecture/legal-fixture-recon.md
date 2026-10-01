# Legal fixture reconnaissance (2026-10-01)

**Proposed review, not an ADR**, not delivery authorization. Honeycomb stays **Proposed**. Sister corpus is read in place under `LAW_NEXUS_ROOT` (default `/root/law-nexus`); nothing is copied into kutha-graph.

Purpose: probe whether today's two axes (`ValidTime` / `TransactionTime` as `u64`, Assert / Retract / Correct / CorrectInterval / claim supports / `as_of` / `live_at` / `derivation_eligible_at`) can express real normative text before durability work (F13 / R1).

## Inventory (re-counted 2026-10-01)

| Tree | Count / names |
|------|----------------|
| `crates/ln-decode/tests/fixtures/npa/` | 45 files: `fz44-001.txt`…`fz44-042.txt`, `syn-001.txt`, `syn-002.txt`, `fz44_npa_tokens.json` |
| `…/npa-hostile/` | `hostile-bounds.txt` |
| `…/npa-lawref/` | 180 `npa-frag-*.txt` + `lawref_seed.json` (181 entries listed) |
| `…/npa-members/` | `anchor-parity-fixture.tsv`, `typed-parity-fixture.tsv` |
| `law-source/consultant/` | 85 entries (README + XML; edition date in file name, e.g. FZ-44 `red-ot-28-12-2025`) |

CBM `law-nexus` locates fixture consumers (`npa_lexer_contract.syn_001_*`); text evidence is the files themselves.

## Model clocks (D-05)

Named clocks touched by the cases below: **promulgation**, **effective**, **application**, **edition**, **known-on**.

Clock conflation: today's two `u64` axes silently fold promulgation, effective, application, edition, and known-on into VT/TT, so “as known on D” and “norm applies in year Y” cannot be told apart without `tt_anchor` / typed clocks (M014) or an executable transitional rule (M015).

## Cases (prose)

### C01 — Sunset (syn-001)

Synthetic: «пункт 15.1 действует до 01.01.2028». Application end-date is not a Retract of a support; needs an application clock or a transitional rule. Class **T** (also R-shaped).

### C02 — Year-bound applicability (fz44-010)

«В 2025 г. возможна закупка…». Calendar-year application window. Class **T**.

### C03 — Specific-over-general derogation (fz44-010)

«кроме ч. 4» carve-out. Needs rule semantics over article parts. Class **R**.

### C04 — Hostile article-bounds (npa-hostile)

«статьями 7.32 - 7.29 того же Кодекса» (descending range). Decode/hostile, not a graph fact shape. Class **X**.

### C05 — Edition pin (fz44-001)

«в ред. ФЗ от 08.08.2024 N 318-ФЗ» with application from 01.01.2025. Edition vs known-on conflation. Class **T**.

### C06–C10 — Amendment inserts, formula, citation chain, lawref, consultant edition stamp

See QUOTE-INDEX. Insert markers («введен») are expressible as Assert at a VT cut (**E**) but lose edition/known-on distinction without typed clocks. Formula text (**E** as opaque object). Consultant `(ред. от 28.12.2025)` is edition metadata on the source file name/body (**T** for known-on).

## Inexpressible set (input for M014 / M015)

- Application end-dates and year-bound windows without conflating VT (**T** → M014).
- Derogation / except-part rules (**R** → M015).
- Hostile descending article ranges (**X** — decode concern, not fold).
- “As known on D” over an edition stamp (**T** → M014 `tt_anchor`).

## M015 golden fixture proposal

Prefer **C01** (`syn-001.txt`): smallest real-shaped sunset text («пункт 15.1 действует до 01.01.2028»). Needs (1) a transitional / sunset rule AST, (2) an as-known-on query distinct from VT, (3) expected eligibility before and after 01.01.2028 at a fixed known-on. Lease decides; this phase only proposes.

Paths below are **relative to LAW_NEXUS_ROOT**.

QUOTE-INDEX
crates/ln-decode/tests/fixtures/npa/syn-001.txt	пункт 15.1 действует до 01.01.2028	T
crates/ln-decode/tests/fixtures/npa/fz44-010.txt	В 2025 г. возможна закупка	T
crates/ln-decode/tests/fixtures/npa/fz44-010.txt	ст. 111.4 (кроме ч. 4)	R
crates/ln-decode/tests/fixtures/npa-hostile/hostile-bounds.txt	статьями 7.32 - 7.29 того же Кодекса	X
crates/ln-decode/tests/fixtures/npa/fz44-001.txt	в ред. ФЗ от 08.08.2024 N 318-ФЗ	T
crates/ln-decode/tests/fixtures/npa/fz44-003.txt	п. 9.1 введен Федеральным законом от 14.11.2023 N 531-ФЗ	E
crates/ln-decode/tests/fixtures/npa/fz44-005.txt	п. 6.1 введен Федеральным законом от 31.12.2017 N 504-ФЗ	E
crates/ln-decode/tests/fixtures/npa/syn-002.txt	формуле 5.1 при условии фиксации ставки	E
crates/ln-decode/tests/fixtures/npa-lawref/npa-frag-001.txt	изложить в следующей редакции	E
crates/ln-decode/tests/fixtures/npa/fz44-017.txt	часть 4.1 введена Федеральным законом от 08.12.2020 N 429-ФЗ	E
crates/ln-decode/tests/fixtures/npa/fz44-002.txt	от 29.07.2017 N 231-ФЗ	E
law-source/consultant/federalnyi-zakon-ot-05-04-2013-n-44-fz-red-ot-28-12-2025-o-kontraktnoi-sisteme-v-sfere-zakupok-tovarov-rabot-uslug-dlya-obespecheniya-g--f9c8ca4c.xml	(ред. от 28.12.2025)	T
QUOTE-INDEX-END
