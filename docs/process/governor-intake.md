# Governor intake (control loop → check)

Process contract, not an ADR. Companion: `.kutha/META.md`.

This is the fail-closed analog of ADR-050 D050-2 on the **harness** plane: the LLM may propose a check; the dictionaries validate; silence is not a requirement. It does not implement product dictionary entities (STATE freeze).

Three surfaces stay distinct. Mixing them in one ledger is a plane collapse.

| Surface | Owns | Lives in | Governor role |
|---------|------|----------|----------------|
| **Control loop** | Trajectory, freeze-as-lease, META, FSM, docs coupling, plane-mix | `.kutha/dictionaries/invariants.yaml` | Interpreter may enforce |
| **Bridges** | A process check that *cites* a product freeze or named test | `.kutha/dictionaries/bridges.yaml` | Cite only; do not copy L_map / L_capability |
| **Kutha requirements** | Engine truth, honeycomb cells, fitness | ADRs, STATE freeze, `crates/` tests | Not a harness dictionary |

Kutha requirements stay in ADRs, STATE, and cargo tests. Do not transcribe honeycomb cells into `.kutha/dictionaries/`.

## Principle

A **control-loop** “must” becomes a governor requirement only after it has a **disposition** in the process ledger. A **bridge** becomes a governor requirement only after it has a `cites` row. Adding a YAML check without a home in exactly one of those two files, or a Python `Check` subclass, is out of protocol. Silence is not a requirement.

```text
control-loop claim (STATE / META / process doc)
  → invariants.yaml row
      disposition ∈ {deferred | check | kind}
  → if check: append checks.yaml using an allowed kind

bridge (cite product freeze or named test; do not copy its state machine)
  → bridges.yaml row (id, claim, cites, check)
  → append checks.yaml using an allowed kind

Kutha requirement (ADR cell, fitness pit)
  → ADR / STATE freeze / crates test
  → not invariants.yaml, not a map-only honeycomb catalog
```

Governor green ≠ ADR Accepted ≠ capability. A ledger row does not promote a honeycomb cell. Bridges may cite; they may not copy state machines.

## Control-loop dispositions (closed)

| Disposition | Means | Required extra fields |
|-------------|--------|------------------------|
| `deferred` | Process rung not yet on; STATE must name it | `until` |
| `check` | `kutha-gov` enforces this control-loop claim now | `check` (id in `checks.yaml`) |
| `kind` | Existing kinds cannot express it (last responsible moment) | `kind` (name to add to META) |

Unknown disposition → HIGH (`invariants-ledger`). `map-only` and `capability` are **not** process dispositions: L_map is ADRs; L_capability is crates tests / FSM `required` evidence.

## Bridges

Required fields: `id`, `claim`, `cites`, `check`. No `disposition`. A bridge always names a live check and a product path it cites (STATE freeze, a crate test, or a product dictionary). Unknown extra lifecycle copied from an ADR → that belongs in the ADR, not here.

A check id appears in **exactly one** ledger: invariants (`disposition: check`) or bridges (`check:`), then in `checks.yaml`. Overlap → HIGH.

## Intake steps

1. Decide the surface. Product architecture → ADR / STATE / crate test. Control loop → invariants.yaml. A fence that cites product without owning it → bridges.yaml.
2. Control loop: append `.kutha/dictionaries/invariants.yaml` with `id`, `claim`, `source`, `disposition`.
3. If `disposition: check`, append `.kutha/dictionaries/checks.yaml` using a kind from META. Same git diff must include the ledger row (`docs-coupling`).
4. Bridge: append `.kutha/dictionaries/bridges.yaml` and the YAML check. Do not copy honeycomb Status or fitness into the bridge row.
5. If no allowed kind fits, set `disposition: kind` on the **control-loop** ledger, then change META + `kinds.py` + a test. Do not add `scripts/kutha_gov/checks/*.py`.
6. Run `uv run kutha-gov explain <check-id>` and `uv run kutha-gov precommit --check invariants-ledger`.
7. Record the process change under CHANGELOG **Process** (not as a GitHub Release).

## What these ledgers are not

- Not a clone of law-nexus ADR frontmatter, D098 tags, or `document-freshness-triggers.json` catalogs.
- Not reverse-audit of every ADR sentence (daily-archive `adr_rules.yaml`).
- Not Keep a Changelog / SemVer / release-plz.
- Not H4-as-legal-pack, ADR-090 ontology, or ADR-050 six product dictionaries.
- Not a second honeycomb: cells stay **Proposed** in `docs/ADR/` until the cell is in the running engine.
