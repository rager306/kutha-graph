---
id: paper-r3-record-replay-retroaction
source: paper
axes: [Time, Verify, Agent]
usefulness: med
optimality: med
demand: med
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# Transaction-granularity record, replay, and retroaction (R3)

Paper: [R3: Record-Replay-Retroaction for Database-Backed Applications](https://doi.org/10.14778/3611479.3611510) (Li et al., 2023). Consensus: https://consensus.app/papers/details/b4e56a5b2d5b5d50bfe6df67feaf226b/?utm_source=cursor

## 1. Raw idea

Faithful replay and retroactive execution of *modified* code over a recorded trace, at transaction granularity (snapshot isolation), without capturing fine-grained thread interleavings. Always-on recording overhead is small for complex transactions.

## 2. STCA applicability

Verify: fork-diff / “what if this behavior had been different” on the log, not a debugger sidecar. Agent: LLM/tool effects that went through the store can be retroacted; effects that bypassed the log cannot. Distinct from Yankin’s four query *mechanisms*: R3 is an application-level time-travel contract over SI.

## 3. Quality / cost

Useful existence proof that retroaction is practical if the cut is transactions, not instructions. Cost: assumes DBMS SI; Kutha’s SoT is the event log, not an RDBMS interceptor. Do not clone R3’s interceptor as the core.

## 4. Demand

Post-hoc bugfix and “rerun the agent with a patched tool” without claiming bitwise thread replay.

## 5. Niche → effect

`no niche`
