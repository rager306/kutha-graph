# Semantic contract validation

Date: 2026-09-13. **Proposed validation design, not implemented capability or delivery authorization.**

This note connects existing honeycomb contracts; it is not another ADR, research wave, or milestone list. The normative homes remain the linked ADRs. `.kutha/STATE.md` still names H3, M001 S01–S03 done, and H4 as the next thin process overlay. Nothing here starts M002, a legal/science pack, a query parser, or new dictionaries. ADR-000/001/002 and the 163-card literature boundary are unchanged.

## What needs strengthening

The architecture does not need another database or a larger algorithm catalog. It needs a testable agreement between three consumers of the same history:

| Consumer | Must preserve | Must not infer |
|---|---|---|
| Knowledge base | Claim/version identity, source supports, VT × TT, admission policy | An observed statement is automatically an admitted fact |
| Agent trace | Runs/attempts, observed inputs, typed dependencies, actions and outcomes | Log order or a read edge proves causality, entailment, or authority |
| Agent context | Evidence references, cut, policy, dependency revisions, completeness | A summary or cached answer is fresher or more authoritative than its evidence |

Claim support, computational dependency, and permission remain distinct relations, even if represented by the same graph engine. A dependency DAG is sufficient as an initial representation for acyclic computations; recursive rule provenance needs its own bounded representation. Do not require a full provenance-polynomial engine just to retain explicit references.

## Contract ownership and current limits

| Contract home | Clarified requirement | Current evidence / limit |
|---|---|---|
| [ADR-011](../ADR/ADR-011-lean-event-schema-lineage.md), [012](../ADR/ADR-012-snapshots-tiers-vacuum.md) | Recover term meanings from retained authoritative history without leases | [store.rs](../../crates/kutha-runtime/src/store.rs) still requires a snapshot for the intern map |
| [ADR-011](../ADR/ADR-011-lean-event-schema-lineage.md), [013](../ADR/ADR-013-bitemporal-facts-invalidation.md) | Independent supports survive withdrawal of another; disagreement is not implicit supersession | [quantum.rs](../../crates/kutha-runtime/src/quantum.rs) has a small inverse behavior, not support maintenance |
| [ADR-013](../ADR/ADR-013-bitemporal-facts-invalidation.md), [070](../ADR/ADR-070-cypher-temporal-as-of.md) | Explicit TT/VT and whole-version versus interval correction | [fold.rs](../../crates/kutha-runtime/src/fold.rs) replaces a whole version; [FF5](../../crates/kutha-runtime/tests/ff5_legal_pit.rs) compares two VT cuts |
| [ADR-010](../ADR/ADR-010-event-log-runtime-quantum.md), [014](../ADR/ADR-014-cascade-budgets-quantum-receipts.md) | Saturation, budget stop, and unknown recovery are distinguishable | A budget-stopped emit can retain events; its receipt is not persisted by the file store |
| [ADR-040](../ADR/ADR-040-materialization-plugin-protocol.md), [060](../ADR/ADR-060-strict-replay.md) | Exact lease identity and separate state/provenance/execution checks | [CSR](../../crates/kutha-runtime/src/csr.rs) is untyped neighbor-set adjacency; replay currently checks fold fingerprints |
| [ADR-051](../ADR/ADR-051-capability-security.md), [052](../ADR/ADR-052-genai-enrichment-pack.md), [080](../ADR/ADR-080-abac-multi-tenant-slices.md) | Admission, current authorization, and source-revision eligibility | Proposed contracts, not implemented ABAC or agent memory |
| [ADR-061](../ADR/ADR-061-fork-and-diff.md), [062](../ADR/ADR-062-regimes-gated-loop.md) | Branch-safe references and explicit external-effect outcomes | Prefix fork is not merge; local replay is not remote exactly-once execution |
| [ADR-030](../ADR/ADR-030-cui-budget-traits.md) | Separable scalar allocation only where its assumptions hold | Shared work and peak RAM require explicit accounting; no general allocator claimed |

## One candidate acceptance fixture

Use a synthetic rule and two evidence sources, not a legal corpus. This is a future fixture specification, not a test already run and not the next active slice.

At TT `t1`, admitted source versions `a` and `b` independently support proposition `P` over VT `[2010,infinity)`. A pinned deterministic rule `r` derives `Q` from `P`; no other rule or source supports `Q` in this fixture. A summary and proposed agent action cite that derivation. At `t2`, source `a` explicitly withdraws its support for `P` over `[2015,2020)` and proposes a replacement `P'` that does not entail `P`. Source `b` has not changed. At `t3`, `b` withdraws support for `P` on that interval. In the conflict variant, set `P' = not-P`. Admission of the replacement and permission to act are explicit policy decisions, not automatic effects of ingestion.

Expected observations:

1. At `(t1,2017)`, retain the original two supports. At `(t2,2017)`, `b` still supports `P` and `Q`; correcting `a` does not erase `b`. If `a` now supports the opposite proposition, preserve both sides and report conflict to the admission policy.
2. At `(t3,2017)`, the last positive support is gone: `Q` loses eligibility through this derivation. Withdrawal alone does not prove the opposite. Earlier TT cuts remain unchanged.
3. Under an explicit interval-patch contract, VT 2012 and 2021 retain residual versions of `a`. The current whole-version `Correct` API must not be used as if it supplied those residuals automatically.
4. The summary/action justification records the exact source revisions and rule version it used. Changed dependencies require current reevaluation; old output remains historical evidence. An alternative valid derivation can restore eligibility, but a stale cached output cannot renew its own admission.
5. At each named cut, incremental maintenance and a clean reconstruction agree on values, active supports, and completeness for this exact query contract. Discarded CSR/snapshots do not change those answers once semantic recovery is implemented. Approximate retrieval has a separate quality contract (ADR-040).

These expectations are proposed oracles. They are not satisfied merely because the existing FF5 fixture or governor checks pass.

## Small adversarial probes, not additional subsystems

| Probe | Required observation |
|---|---|
| Remove all leases, retain authoritative history | Resolve the same terms, claims, and outcomes; missing authoritative objects fail explicitly |
| Budgets 0/1/2; crash after a committed prefix | Zero/partial/full progress is distinguishable; absence of terminal evidence is not success; resume is explicit |
| Change only a causal reference or rule version | Provenance/execution verification detects the change even if state replay still matches |
| Duplicate delivery; two branches use the same local sequence | No duplicated support for one delivery; cross-branch references cannot target the wrong claim |
| Two sources plus one combining task; swap independent arrivals | Preserve the same dependency set under stable identities, without claiming identical log order or hashes |
| Positive rule cycle; fresh-entity generator | Finite set saturation terminates; unbounded generation is rejected/bounded; recursive provenance is not assumed finite |
| Same endpoints, different relations/supports | Typed queries do not lose labels or support multiplicity through a neighbor-set CSR |
| `A->B [1,2)`, `B->C [3,4)` | Snapshot/co-temporal path absent; journey may exist under explicit waiting rules |
| Two n-ary interactions with equal participants | Incidence round-trip preserves interaction identity, roles, positions, and multiplicity |
| Present access revoked; old data/grants requested | AS OF does not bypass current authorization; authorized historical audit is a separate question |
| Remote success, then crash before local outcome | Reconcile via stable effect identity or keep outcome unknown; no blind retry or fictional rollback |
| One CSR build shared by two queries | Charge shared work once; independent peak-RAM and deadline constraints still hold |

## Remaining decisions before implementation

The clarified invariants are more stable than their encoding. Future implementation must still select the logged term-definition format, stable claim/support references, quantum outcome/continuation records, partial-correction API, and admitted rule subset. Those choices should be made for one authorized fixture, not by building every honeycomb cell.

The first technical prerequisite is semantic recovery without snapshots. Next in dependency order are claim/support identity and explicit completion/replay guarantees. This is a dependency assessment, **not a replacement for H4 or a new delivery order**. General multi-resource optimization, four-valued reasoning, full recursive provenance, and hypergraph-native storage are not prerequisites for the initial fixture.

Stop expanding the design when one authorized end-to-end fixture can distinguish preserved history, current evidence, and allowed action. Add architecture only in response to a failing oracle or a concrete consumer requirement.

## Evidence boundary

The 2026-09-13 review used Tier 2 graph discovery for `kutha-graph` (index generation `2026-08-18T12:44:42Z`) and direct source verification. Relevant pre-edit coverage reported no recorded gaps; that is best-effort evidence, not an exhaustive correctness proof. Edited documentation is newer than that index. Runtime observations above are limited to the named source paths; the proposed probes are not executable capability claims.

Validation of this documentation change:

- Independent review corrected path-objective terminology, exact versus approximate lease equivalence, and the fixture's explicit withdrawal premise.
- Existing Rust suite: 26 tests passed with `cargo --config 'build.rustc-wrapper=""' test --workspace`.
- Existing harness suite: 26 tests passed with `RUSTC_WRAPPER= uv run --no-sync pytest` (Python 3.13). Its integration cases execute governor quanta and append ordinary process observations/tenant events; no lifecycle lease was edited.
- The default Rust invocation failed because globally configured `sccache` returned `Operation not permitted`; it also caused the first harness run's two failures. The override was per invocation, not a global configuration change.
- Documentation validation confirmed 163 unique cards matching the matrix, unchanged card metadata and Proposed statuses, valid local links, and unchanged runtime code, spine ADRs, STATE, and ROADMAP. `git diff --check` passed.

These are regression/documentation checks, not execution of the proposed semantic probes above.
