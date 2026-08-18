# ADR-051: Capability Security

## Status

**Proposed** (Agent/Security — unforgeable authority references; not path-ABAC and not WASM)

## Date

2026-08-18

## Honeycomb coordinates

- Axes: **Agent** (primary) · **Security** · **Composition**
- Depends on: ADR-000 (D3), ADR-050
- Anticipates: ADR-080 (path rewrite / grants), ADR-081 (code sandbox)

## Context

ADR-050 requires `validate(+security)` without specifying the kernel. Capabilities answer “which object may this holder invoke *this* operator on *now*?” ACLs answer “who may this object?” Path-ABAC answers “may this hop appear in a result?” WASM answers “can this procedure eat the process?”

Grounding cards:

- `paper-object-capabilities` — `.compound-engineering/artifacts/research/applicability/cards/paper-object-capabilities.md`
- `paper-pact-argument-provenance` — `.compound-engineering/artifacts/research/applicability/cards/paper-pact-argument-provenance.md`
- `paper-mas-isolation-lattice` — `.compound-engineering/artifacts/research/applicability/cards/paper-mas-isolation-lattice.md`

Miller: capabilities ≠ ACLs; least privilege; confused deputy. Rajani: capabilities do not prevent all confused-deputy; provenance semantics can. ScopeGate: exposing a tool list is not per-call value authorization. PACT: authority-bearing *arguments* need provenance. MAS isolation lattice: concurrency/tool-effect anomalies — cousin, not this noun. CHERI queued.

## Decision

### D051-1. Authority is an unforgeable reference

Agents and packs hold capabilities (append-to-log, rebuild-CSR, call-remote, read-named-graph), not a role string alone. Passing a capability into a plugin *is* the Port. Do not store capabilities as a second ACL table that the LLM edits.

### D051-2. Tool exposure ≠ argument authorization

An allow-list of tools is necessary and insufficient. PACT-grain: untrusted content must not bind authority-bearing arguments. Fail closed.

### D051-3. P0 = operator allow-list; honeycomb = grant events

Unforgeable tokens on the log (capability grant/revoke as events, VT×TT like 013). Skip membranes-as-manual-wrappers and CHERI until a hardware spike.

**Hard separations:**

```text
Capability             ≠  ACL / RBAC row
Capability             ≠  Path-ABAC rewrite (080)
Capability             ≠  WASM isolation (081)
Tool list (ScopeGate)  ≠  Per-call argument check (PACT)
MAS lattice            ≠  This noun
```

## Consequences

### Positive

- 050 validate has a named kernel.
- Confused-deputy is a design test, not an afterthought.

### Negative / risks

- Graph-DB OCap implementations are thin; over-specify tokens before P0.

### Non-goals

- Path rewrite (080). WASM (081). CHERI.

## Alternatives Considered

| Alternative | Why not |
|-------------|---------|
| RBAC as the only model | D3 already wants policy-as-graph; capabilities are the agent grain |
| ScopeGate-as-authz | Card: gates exposure, not values |
| Collapse into 080 | Different question (hops vs invoke) |

## Open Research Questions

1. Token encoding vs interned ids on grant events.
2. Where PACT inference stops and 080 rewrite starts.
3. MAS isolation lattice vs capability attenuation.

## Related Decisions

- ADR-050, ADR-080, ADR-081
