# Phase 47: Horn determinism and closure — Pattern Map

**Mapped:** 2026-10-06 | **Files:** 5 | **Analogs:** 5/5 (git-tracked)

## File Classification

| New/Modified File | Role | Data Flow | Closest Analog | Match |
|-------------------|------|-----------|----------------|-------|
| `crates/kutha-common/src/event.rs` | model | transform | `Event::stable_define` | exact |
| `crates/kutha-runtime/src/eval.rs` | service | event-driven | `rule_follow_ons` / `retract_derived` | exact |
| `crates/kutha-runtime/src/pipeline.rs` | service | event-driven | `commit_user_cascade` | exact |
| `crates/kutha-runtime/tests/m019_sem_meta.rs` | test | batch | `horn_correct_char_*`, `cascade_order_eager_lazy_*` | exact |
| `docs/architecture/test-matrix.yaml` | config | — | `SEM-horn-correct-char` row | exact |

## Pattern Assignments

### `event.rs` — stable derived EventId (DET-02)

**Analog:** `Event::stable_define` (`event.rs` 286–305): `Sha256` + domain prefix → first 16 bytes → `Uuid::from_bytes`; user path stays `Event::new` (v7, 255–284). Add constructor for derived `Op::Behavior` / follow-on `Op::Retract` with pin, `caused_by`, binding TermIds, op tag in preimage.

### `eval.rs` — Correct re-derive + Retract branch (HORN-CORR-01)

**Analog Retract:** `rule_follow_ons` → `Op::Retract` → `retract_derived` (28–55, 59–91): scan log for Horn `Behavior` with `caused_by == premise`, live fact → mint `Op::Retract { event_id: e.id }` via `Event::new` (swap to stable id in Phase 47).

**Change:** extend match (54–55) so `Op::Correct` / `Op::CorrectInterval` call `retract_derived` then `horn_fires` on corrected triple (mirror Assert/Behavior arms, 30–53). Unknown head/premise relation: `RuntimeError` not `continue` (110–112 today).

**Analog mint:** `head_event` (192–225) — gate duplicate triple, map `knownBy` → `inverse_knows`; replace `Event::new` with stable derived helper.

### `pipeline.rs` — admit every cascade hop (CASC-01)

**Analog:** `commit_user_cascade` (243–276): LIFO `pending.pop()`; today only first event’s follow-ons get `admit` (245–247) before loop; inner hops commit without admit (262–266). **Copy:** `pipeline_events` pre-loop admit (217–218). Admit each follow-on before `pending.push` (or before `commit_event`).

### `m019_sem_meta.rs` — RED / oracle pins

**Analog cascade (keep):** `cascade_order_eager_lazy_inverse_knows` (81–110): `cascade_fixture`, `inverse_behaviors`, `fail_seed` — Eager vs Lazy order parity.

**Analog horn (invert):** `horn_correct_char_derived_survives_premise_correct` (424–479) documents today’s skip; flip to **kill** after pin fails: old derived not live, re-mint or none; pair `horn_correct_interval_char_derived_survives_patch` (486+). Helpers: `live_triple`, `live_known_by`, `inverse_behaviors` (20–34, 412–422).

### `test-matrix.yaml`

Flip `SEM-horn-correct-char` `status: characterization` → `killed`; update `kill_mutation` to match new `rule_follow_ons` Correct path (715–726).

## Shared Patterns

- **Stable ids:** `stable_define` + `tests/common/sem.rs` / `store.rs` callers (796, 764) — same ingest → same id.
- **Fail closed:** `pipeline_kind` admits user op before cascade (190–202); blob/admission errors → `RuntimeError`.
- **Tests:** module header + `common::sem` seeds (m019_sem_meta.rs 1–18); do not weaken FF5 / other SEM rows.

## No Analog Found

None — all scoped files have in-repo analogs.

## Metadata

**Scope:** `crates/kutha-{common,runtime}/`, `docs/architecture/test-matrix.yaml` | **CBM:** kutha-graph (source read; index stale on eval/pipeline/tests)
