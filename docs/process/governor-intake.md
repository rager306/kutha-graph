# Governor intake (control loop → check)

Process contract, not an ADR. Companion: `.kutha/META.md`.

This is the fail-closed analog of ADR-050 D050-2 on the **harness** plane: the LLM may propose a check; the dictionaries validate; silence is not a requirement. It does not implement product dictionary entities (STATE freeze).

Three surfaces stay distinct. Mixing them in one ledger is a plane collapse.

| Surface | Owns | Lives in | Governor role |
|---------|------|----------|----------------|
| **Control loop** | Trajectory, freeze-as-lease, META, FSM, docs coupling, plane-mix | `.kutha/dictionaries/invariants.yaml` | Interpreter may enforce |
| **Bridges** | A process check that *cites* a product freeze or named test | `.kutha/dictionaries/bridges.yaml` | Cite only; do not copy L_map / L_capability |
| **Map** | Honeycomb cells, D1–D10, stages, depends_on | `.kutha/dictionaries/honeycomb.yaml` | Validate + `kutha-gov map`; does not accept cells |

ADR bodies stay narrative. `honeycomb.yaml` is the compact L_map index. `map` / `delivery` / `capability` stay orthogonal. Dump for context: `uv run kutha-gov map` (optional cell id; `--format json`).

## Principle

A **control-loop** “must” becomes a governor requirement only after it has a **disposition** in the process ledger. A **bridge** becomes a governor requirement only after it has a `cites` row. A honeycomb cell is tracked in `honeycomb.yaml`, not as a YAML check. Adding a check without a home in exactly one of invariants or bridges, or a Python `Check` subclass, is out of protocol. Silence is not a requirement. Kutha requirements stay in the map index + ADR bodies + named tests.

```text
control-loop claim (STATE / META / process doc)
  → invariants.yaml row
      disposition ∈ {deferred | check | kind}
  → if check: append checks.yaml using an allowed kind

bridge (cite product freeze or named test; do not copy its state machine)
  → bridges.yaml row (id, claim, cites, check)
  → append checks.yaml using an allowed kind

map cell (ADR opened or stage/edge changed)
  → honeycomb.yaml row (map, delivery, capability, depends_on, locks, evidence)
  → ADR Status remains the narrative twin; governor matches map to **Status**
  → not a YAML check; not Accepted
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

1. Decide the surface. Product cell → ADR file + `honeycomb.yaml`. Control loop → invariants.yaml. A fence that cites product without owning it → bridges.yaml.
2. Control loop: append `.kutha/dictionaries/invariants.yaml` with `id`, `claim`, `source`, `disposition`.
3. If `disposition: check`, append `.kutha/dictionaries/checks.yaml` using a kind from META. Same git diff must include the ledger row (`docs-coupling`).
4. Bridge: append `.kutha/dictionaries/bridges.yaml` and the YAML check. Do not copy honeycomb Status or fitness into the bridge row.
5. Map: append `.kutha/dictionaries/honeycomb.yaml`. Stages: `map` (ADR Status) · `delivery` (`map-only`/`frozen`/`spike`) · `capability` (`none`/`named`). Links: `depends_on`, `locks`, `evidence`. Run `uv run kutha-gov map`.
6. If no allowed kind fits, set `disposition: kind` on the **control-loop** ledger, then change META + `kinds.py` + a test. Do not add `scripts/kutha_gov/checks/*.py`.
7. Run `uv run kutha-gov explain <check-id>` and `uv run kutha-gov precommit --check invariants-ledger` (or `--check honeycomb-ledger`).
8. Record the process change under CHANGELOG **Process** (not as a GitHub Release).

## What these ledgers are not

- Not a clone of law-nexus ADR frontmatter, D098 tags, or `document-freshness-triggers.json` catalogs.
- Not reverse-audit of every ADR sentence (daily-archive `adr_rules.yaml`).
- Not Keep a Changelog / SemVer / release-plz.
- Not H4-as-legal-pack, ADR-090 ontology, or ADR-050 six product dictionaries.
- Not a second honeycomb markdown: the compact index is `honeycomb.yaml`; cells stay **Proposed** in `docs/ADR/` until the cell is in the running engine.
