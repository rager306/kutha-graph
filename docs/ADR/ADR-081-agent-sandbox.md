# ADR-081: Agent Sandbox

## Status

**Proposed** (Security/Agent — isolate untrusted pack/UDF code; not capabilities and not ABAC)

## Date

2026-08-18

## Honeycomb coordinates

- Axes: **Security** (primary) · **Agent** · **Composition**
- Depends on: ADR-051, ADR-052, ADR-021
- Anticipates: none required (CHERI/TEE later)

## Context

Packs will ship UDFs (enrichment, custom hops). Process/containers are too heavy per tuple. WASM/SFI is the literature sandbox; tax is copy+layout. Capabilities (051) decide *which Ports* the module sees; the sandbox decides *the procedure cannot eat the process or raw-write the log*. Cordon (062 cousin) decides when an effect commits.

Grounding cards:

- `paper-wasm-udf-sandbox` — `.compound-engineering/artifacts/research/applicability/cards/paper-wasm-udf-sandbox.md`
- `paper-object-capabilities` — `.compound-engineering/artifacts/research/applicability/cards/paper-object-capabilities.md`

WASI is not kernel isolation unless decoupled. CHERI/TEE are hardware poles, not P0. ScopeGate is not this cell.

## Decision

### D081-1. User/LLM-generated procedures run sandboxed

Compile Cypher/GPML in-process (trusted engine). Run pack UDFs behind WASM (or later CHERI) with a **capability-limited** WASI: only Ports granted (051). A UDF writes the event log only through TOKI/receipt paths (010/013/014).

### D081-2. Shared-memory views are a cost requirement

Naive copy of CSR rows will lose to native. Honeycomb: shared-memory Arrow/CSR views (WAF-shaped). No SGX-as-SoT.

### D081-3. Sandbox ≠ semantic transaction ≠ ABAC

Cordon-style commit of *tool effects* is orthogonal. Path-ABAC (080) still rewrites queries. Both can wrap a sandboxed UDF.

**Hard separations:**

```text
WASM/SFI               ≠  Capability token (051)
WASM                   ≠  Path-ABAC (080)
WASI syscalls          ≠  Kernel isolation
TEE / SGX              ≠  Event-log SoT
Container per hop      ≠  P0 design
```

## Consequences

### Positive

- 052 enrichment can run untrusted model glue.
- 021 packs are not equivalent to native `unsafe`.

### Negative / risks

- Marshalling tax; CHERI temptation.

### Non-goals

- Implementing WAMR. PQC placeholders (oxify skip class).

## Alternatives Considered

| Alternative | Why not |
|-------------|---------|
| Trust every pack as native | Unshippable for enterprise (D8/R7) |
| One container per UDF | Too heavy (card) |
| Collapse into 051 | Isolation of code vs authority of reference |

## Open Research Questions

1. WASM runtime choice as a spike, not this ADR.
2. How CSR views map into linear memory without copies.
3. Whether P0 forbids UDFs entirely (engine-only) until 081 spikes.

## Related Decisions

- ADR-051, ADR-052, ADR-021, ADR-010
