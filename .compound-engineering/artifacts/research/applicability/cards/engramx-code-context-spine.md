---
id: engramx-code-context-spine
source: engramx
axes: [Agent, Query, Time, Data]
usefulness: high
optimality: med
demand: high
confidence: code
layer5: no niche
status: closed
channels_failed: []
---

# Code-graph intercept is a context lease — not Wang Engram memory and not Kutha SoT

User URL: https://github.com/nickcirv/engram (cloned `/tmp/user-url-scout/engram`, not CBM-indexed). **Not** the Wang 2026 Engram paper (`paper-engram-bitemporal-memory`). This is Engramx: local SQLite code KG + IDE Read intercept. Read: `src/graph/schema.ts` — nodes `file|class|function|…|mistake` with `validUntil` / `invalidatedByCommit` and v4 mistake axes `thenBelieved` / `foundFalseAt` / `truthNow` / `appliesTo`; edges `calls|imports|contains|extends|tested_by|…`; `src/graph/store.ts` — sql.js SQLite, not a WAL; `src/miners/git-revert-miner.ts` — pair `git revert` with the reverted SHA as a bi-temporal mistake; `src/intercept/handlers/read.ts` — PreToolUse:Read deny+summary when the packet is smaller than the raw file (`never-worse`); `src/graph/pagerank.ts` — personalized PageRank over `calls`. Distinct from `paper-engram-bitemporal-memory` (agent episode KG), `hindsight-four-network-tempr` (four epistemic nets), `paper-user-as-code-log` (user actions as events), GitOfThoughts (queued git-as-memory cousin).

## 1. Raw idea

Index the repo once. At the agent boundary, replace whole-file Read with a structural packet (imports/calls/summaries). Git reverts become **invalidate-not-delete mistakes**: what you believed then, when it was found false, what is true now. SQLite is a cache of the code graph, not truth — git is.

## 2. STCA applicability

Agent: a **coding-session pack** over the fold of *this* repo. Query: intercept is hybrid retrieve (graph + AST), not WCOJ. Time: revert-miner is TOKI-shaped supersession of a *belief about code*, with valid-time on the mistake node — still not the Kutha event log. Data: `calls` CSR/PageRank is a lease; do not make sql.js the SoT. Verify: `never-worse` is a fail-open size gate, not a quantum receipt. LLM stays off the write path of the miner (git is the source).

## 3. Quality / cost

Usefulness high: structural context reduction is a real agent demand. Optimality med: real miners + PageRank; TypeScript/SQLite is not the Kutha core (ADR-000). Cost: steal the *mistake axes* and intercept contract; do not vendor Engramx; do not confuse with Wang Engram.

## 4. Demand

Agents re-read files and re-apply reverted bugs. Engine demand: dated belief facts + graph-adjacent file lease. Product: optional coding pack, never the quantum.

## 5. Niche → effect

`no niche`
