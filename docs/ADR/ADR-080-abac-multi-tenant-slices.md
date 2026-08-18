# ADR-080: ABAC & Multi-Tenant Slices

## Status

**Proposed** (Security — path rewrite + temporal grants + tenant slices + vacuum authority; one cell, not three products)

## Date

2026-08-18

## Honeycomb coordinates

- Axes: **Security** (primary) · **Space** · **Time** · **Query**
- Depends on: ADR-000 (R7), ADR-013, ADR-050, ADR-070
- Anticipates: ADR-081 (sandbox), ADR-012 (who may vacuum)

## Context

STRATEGY buyer constraint: ABAC, temporal policies, multi-tenancy, immutable audit — sketched after core physics. Synthesis: open 080 from path-ABAC + temporal grants + vacuum holds as **one** security cell. Capabilities (051) and WASM (081) stay adjacent.

Grounding cards:

- `paper-xacml4g-path-abac` — `.compound-engineering/artifacts/research/applicability/cards/paper-xacml4g-path-abac.md`
- `paper-temporal-grants-as-facts` — `.compound-engineering/artifacts/research/applicability/cards/paper-temporal-grants-as-facts.md`
- `oxify-zanzibar-rebac` — `.compound-engineering/artifacts/research/applicability/cards/oxify-zanzibar-rebac.md` (`confidence: code` overlay, not SoT)
- `paper-graph-tenant-isolation` — `.compound-engineering/artifacts/research/applicability/cards/paper-graph-tenant-isolation.md`
- `paper-event-log-vacuum-legal-hold` — `.compound-engineering/artifacts/research/applicability/cards/paper-event-log-vacuum-legal-hold.md`

XACML4G / Cypher rewrite: unauthorized subgraphs must never return. Grants are bi-temporal facts (Bertino/TRBAC/HO(T)-ReBAC). Zanzibar tuples are a **grant-graph overlay**. Tenant = Space slice + noisy-neighbor quotas, not path-ABAC. Vacuum/hold is who may physically drop SoT. DP/STE/k-anonymity are publish/outsource leases, not this PEP. CMK/air-gap is GTM, not a graph noun.

## Decision

### D080-1. Policy enforcement is query rewrite on the GPML IR

ABAC/path policy is a Port. Compile policy into the plan (070 IR) so the engine never materializes unauthorized hops for the caller. App-layer post-filters are insufficient. LLM is not the PEP.

### D080-2. Grants are log facts with VT×TT

`Grant` / `Revoke` / role-enable are events like TOKI facts. AS-OF query applies the grant in force at T, not today’s role table. ReBAC/Zanzibar-shaped tuples may overlay as a **lease** of the grant fold — oxify is evidence of a kernel, not Kutha SoT.

### D080-3. Tenant is a slice of one fold

Named-graph/bank descriptor + resource quotas on leases. Not a second Postgres per counsel. Path-ABAC answers “may this hop return?”; tenancy answers “is this vertex in the slice, and does this tenant starve another?” Vector-plane silo is 071’s cousin.

### D080-4. Vacuum/hold authority is policy-as-graph

Who may vacuum or set a legal hold is a grant-shaped fact (012 physics, this cell’s PEP). LLM does not choose forget.

### D080-5. P0 may be single-tenant fail-open-on-absent-policy is forbidden for enterprise

Closed policy default for regulated packs: no policy compiled → no data. P0 spike may stay single-tenant.

**Hard separations:**

```text
Path-ABAC rewrite      ≠  Table ACL
Grant @T               ≠  Today’s RBAC snapshot
Tenant slice           ≠  Path constraint
ReBAC overlay          ≠  Event-log SoT
Vacuum hold            ≠  LSM tombstone
Capability (051)       ≠  This rewrite
DP / STE / k-anon      ≠  PEP (publish/outsource leases)
CMK / air-gap          ≠  Graph feature
```

## Consequences

### Positive

- Legal/HIPAA AS-OF of *permission* has a home.
- 012 vacuum is no longer ad hoc.

### Negative / risks

- Rewrite cost grows with policy complexity (card: med optimality).
- One ReBAC crate (`code`) vs paper-shaped path-ABAC.

### Non-goals

- Another path-ABAC paper. Local/GNN DP. Implementing WASM (081).

## Alternatives Considered

| Alternative | Why not |
|-------------|---------|
| Three products (ABAC vs grants vs tenant) | User/synthesis: one cell |
| Post-filter in the agent | Unauthorized data would still exist in the result pipeline |
| oxify as SoT | Overlay only |
| Geo-Raft isolation as tenancy | Trap / wrong noun |

## Open Research Questions

1. Rewrite-enforcer spike (synthesis: would move Security more than Query 103).
2. Open vs closed policy default per pack.
3. Negative permissions / deny edges.
4. How 051 capabilities gate *who may compile* a rewrite vs 080 *what the rewrite allows*.

## Related Decisions

- ADR-012, ADR-013, ADR-050, ADR-051, ADR-070, ADR-071
