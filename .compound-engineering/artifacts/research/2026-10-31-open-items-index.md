# Open items index — 2026-10-31 parallel audit wave

One page that points at the durable reports and separates what is applied, what needs a lease,
and what needs a governor intake. Nothing here is a capability claim; ADRs stay Proposed.

## Applied today (docs / map / evidence)

| Commit | What |
|---|---|
| `957a8d5..c2fe193` | M186 A/B2 medians corrected, reproducibility limits, independent PASS |
| `f240dbd` | ADR-070/011/014 clarifications, ADR-000 D6 marker, thaw-map wording, test-matrix known-absences, gap register committed |
| `d430a14` | `capability: named` for ten cells; C2 design spike |
| `b353678` | ADR-094 workspace shape as shipped (Proposed) |
| `89bc325`, `8acee31` | v1.73–v1.75 claim wording corrected; v1.73/v1.74 verification re-issued as `gaps_found` after mutation checks |
| `4141abe`, `35d2930` | eCFR third-edition recon (no edition added), licensing basis corrected, C2 milestone sketch |

## Open defects — need a product lease (crates/)

1. **v1.73 vacuity**: `m180_replay_meta.rs:164,190` still pass `source_event_ids: vec![]`; `enrichment.rs:108`
   rejects before the checks they name. Fix: real `source_event(0).id` + assert reason.
2. **v1.73 replay gaps**: pack-level version mismatch and double activate/retire have no replay test.
3. **v1.74 falsifier**: `m167_ecfr_pit.rs:141-159` — keep the count assert and drop the equality asserts,
   or drop the count assert; the comment at `:141-145` states the false direction.
4. **Cross-run tmp collisions**: `m178_enrichment.rs:199,223`, `m179_gate_gc.rs:102,127`,
   `m185_enrichment_admission.rs:242` use fixed relative `tmp/` dirs plus `remove_dir_all`; concurrent
   workspace runs delete a peer's store. Make them unique or serialise.
5. **Wall-clock oracles**: `m182_meta_threats.rs::replay_meta_is_not_quadratic` (3 s absolute) and
   `m183_digest_falsifiers.rs::manifest_replay_scales_linearly_with_packs` (ratio + 50 ms) — replace with
   structural assertions (counters/op counts), as M184 did.

## Harness change — needs a governor-intake row

- `dogfood` (`.kutha/dictionaries/checks.yaml:180`) globs `**/*.egg-info/*` from the repo root with no
  scratch exemption, so a killed test run under `tmp/` turns the live tree red. Options: skip the declared
  scratch root, clean up in tests, or both. Kernel/check semantics change → `.kutha/META.md` +
  `docs/process/governor-intake.md` row before code.

## Decisions pending (owner)

1. **Lease `M187` test-honesty** — closes items 1–5 above; sketch of the work is in
   `2026-10-31-c2-vocabulary-milestone-sketch.md` §1.1 (lease text) and needs its own small plan.
2. **Lease `C2 dictionaries` (vocabulary-first)** — sketch ready; GSD dry-run confirmed v1.80 / phases 275–277.
3. **ADR-094** — already recorded as Proposed; accepting it is a separate act and changes nothing in code.
4. **eCFR** — no third edition for 1 CFR 304; a real three-state corpus is A1/A2 with a new source (12 CFR 1003.3 candidate).

## Reports (all durable)

- `.compound-engineering/artifacts/research/2026-10-31-architecture-gap-register.md`
- `.compound-engineering/artifacts/research/2026-10-31-adr-conformance-and-draft-094.md`
- `.compound-engineering/artifacts/research/2026-10-31-honeycomb-capability-backfill.md`
- `.compound-engineering/artifacts/research/2026-10-31-c2-dictionaries-design-spike.md`
- `.compound-engineering/artifacts/research/2026-10-31-security-threat-model.md`
- `.compound-engineering/artifacts/research/2026-10-31-claims-audit-v170-v175.md`
- `.compound-engineering/artifacts/research/2026-10-31-ecfr-third-edition-recon.md`
- `.compound-engineering/artifacts/research/2026-10-31-test-health-audit.md`
- `.compound-engineering/artifacts/plans/2026-10-31-c2-vocabulary-milestone-sketch.md`
- `.compound-engineering/artifacts/reviews/2026-10-31-v173-v174-reverification.md`
- `.compound-engineering/artifacts/reviews/2026-10-31-m186-correction-verification.md`
