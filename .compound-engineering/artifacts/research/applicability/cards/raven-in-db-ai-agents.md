---
id: raven-in-db-ai-agents
source: raven
axes: [Agent, Composition, Security]
usefulness: high
optimality: med
demand: high
confidence: spec
layer5: "Enterprise in-DB agents: tools + conversation stay inside the server; graph remains weak — Kutha inverts that"
status: closed
channels_failed: []
---

# Server-side AI agents with query/action tools

Evidence: RavenDB 7.2 docs (Jina): https://docs.ravendb.net/7.2/ai-integration/ai-agents/creating-ai-agents/creating-ai-agents_api — agent config (prompt, tools, output schema) registered on the server; query tools; conversation API. Closed kernel → never `code` (AE1).

## 1. Raw idea

Agents live in the database process: tools are first-class, not an app sidecar.

## 2. STCA applicability

Agent: in-DB runtime (ADR-000). Composition: tool policy / quantum of effects. LLM still must not be SoT — Raven does not solve that.

## 3. Quality / cost

Strong productization of agents; weak native graph (ADR-000). Cost: copying Studio wizards vs copying the *trust boundary* (tools vs model).

## 4. Demand

“Agent in the same ACID conversation as the data” is the wedge vs Graphiti-class sidecars.

## 5. Niche → effect

Enterprise in-DB agents: guarded tools next to documents; Kutha’s delta is graph+time, not the agent host itself.
