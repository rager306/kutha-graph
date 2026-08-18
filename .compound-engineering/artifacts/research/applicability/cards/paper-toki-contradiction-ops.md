---
id: paper-toki-contradiction-ops
source: paper
axes: [Time, Composition, Verify]
usefulness: high
optimality: high
demand: high
confidence: paper
layer5: "Legal/regulated belief revision: losers kept as audit rows; isolation declared at write time"
status: closed
channels_failed: []
---

# Isolation-typed contradiction operators (TOKI)

Paper: [TOKI](https://arxiv.org/abs/2606.06240) (Wang, 2026). Consensus: https://consensus.app/papers/details/e5829cf539415e7997779f68c5aaa02c/?utm_source=cursor

## 1. Raw idea

LWW / evidence-merge / await-confirm / policy as one bitemporal operator family; each has an isolation precondition; losers stay in an audit row. LLM judge on the write path is logged or the three write-time anomalies appear.

## 2. STCA applicability

Time: contradiction is concurrency control, not a chat heuristic. Composition: operator pipelines. Verify: provenance + replay. Event log remains SoT; TOKI is the *write contract* on the fold.

## 3. Quality / cost

High theoretical fit (already in `patterns.md`). Cost: formal isolation before P0; judge-on-write fights “LLM not SoT” unless CA-cached.

## 4. Demand

Any store that silently LWW-overwrites facts will fail legal/agent contradiction probes.

## 5. Niche → effect

Legal/regulated belief revision: declared isolation + preserved losers; not a GTM filter on intake.
