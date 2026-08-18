---
id: paper-object-capabilities
source: paper
axes: [Security, Agent, Composition]
usefulness: high
optimality: med
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# Authority is an unforgeable reference — not an ACL row and not a WASM sandbox

Papers: [Capability Myths Demolished](https://consensus.app/papers/details/d47742499ea157a087949184f03dbeba/?utm_source=cursor) (Miller et al., 2003) ACL ≉ capabilities; capabilities give least privilege and avoid confused deputy [3]; [access control vs capabilities](https://consensus.app/papers/details/3653013b63505a73b27877c8af654c87/?utm_source=cursor) (Rajani et al., 2016, CSF) they are *fundamentally different*; capabilities do **not** prevent all confused-deputy attacks; provenance semantics can [1]; Wagner: object reference = capability; unforgeability; privilege-separated components [2]; [ScopeGate](https://consensus.app/papers/details/7acf73b3461b59eda1c057505e27d261/?utm_source=cursor) (Zuvic, 2026) LLM agent frameworks gate *tool exposure*, not per-call value authorization — capability gate ≠ authorization [7]. Distinct from `paper-wasm-udf-sandbox` (isolate untrusted *code*), `oxify-zanzibar-rebac` (relation tuples), `paper-xacml4g-path-abac` (query rewrite on hop shape), `paper-pact-argument-provenance` (argument receipts), CHERI (queued hardware).

## 1. Raw idea

A **capability** is an unforgeable, transferable right to invoke operations on a specific object [2][3]. ACLs answer “who may this object?”; capabilities answer “which object may this holder touch?” Confused deputy: a privileged component is tricked into using *its* authority on the attacker’s target [1][3]. Agent stacks today expose tools as RPCs; that is a capability *list*, not a fail-closed check of the *arguments* [7].

## 2. STCA applicability

Security 051 / Agent: a pack holds capabilities (append-to-log, rebuild-CSR, call-remote), not a role string. Composition: passing a capability into a plugin is the port. Verify: confused-deputy freedom may need provenance (Rajani), not just “has the tool.” Query/path-ABAC still answers “may this hop return?”; ReBAC answers “does this tuple hold?”; capabilities answer “may *this* agent invoke *this* operator on *this* object now?” Do not store capabilities as a second ACL table.

## 3. Quality / cost

Usefulness high: honeycomb 051 is empty without the noun. Optimality med: classic OCap theory is solid; graph-DB implementations are thin; ScopeGate is the agent-shaped demand. Cost: P0 = fail-closed operator allow-list; honeycomb = unforgeable capability tokens on the log (grant events); skip membranes-as-manual-wrappers and CHERI until a hardware spike.

## 4. Demand

Tool-using packs will be confused deputies over AS-OF and MERGE. Engine demand: per-call argument authorization, not “the agent has Cypher.”

## 5. Niche → effect

`no niche`
