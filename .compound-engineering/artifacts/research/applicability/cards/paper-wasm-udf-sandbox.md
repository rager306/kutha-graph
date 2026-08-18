---
id: paper-wasm-udf-sandbox
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

# Untrusted graph procedures run in a sandbox — WASM/SFI, not another semantic transaction

Papers: [WAF](https://consensus.app/papers/details/fc78bba9a56552028dfbd0257c89b55d/?utm_source=cursor) (Huang et al., 2025, ICDE) WASM UDFs: language-neutral, isolate deps, cross-engine; overhead is **copy + layout**; shared memory + compile-time layout cuts WASM UDF cost 3.1× vs naive, 18.1× vs containers [3]; [zero-cost SFI transitions](https://consensus.app/papers/details/f75f85f77a65571c8b11083c7d6a5e9b/?utm_source=cursor) (Kolosick et al., 2021) Wasm/Lucet context-switch tax [1]; [cWAMR/CHERI](https://consensus.app/papers/details/4db115b6ac9154879c38309c56d06dc7/?utm_source=cursor) hardware capabilities on WAMR [2]; [Twine](https://consensus.app/papers/details/e30fe7760fc954788f910caec29b538b/?utm_source=cursor) Wasm-in-TEE two-way sandbox [4]; [WaSC](https://consensus.app/papers/details/b262140e295d584ba24a16f455312f23/?utm_source=cursor) WASI is not kernel isolation; daemon-decoupled syscalls [6]. Distinct from `paper-cordon-semantic-tx` (commit/rollback of *tool effects*), `paper-pact-argument-provenance` (which *argument* may bind), `paper-mas-isolation-lattice` (concurrency anomalies). MCP-SandboxScan [9] is demand that agent tools need a witness, not a second card.

## 1. Raw idea

Packs will ship UDFs (enrichment, custom hops, mental-model jobs). Process/container isolation is too heavy for per-tuple graph work. WASM gives a portable sandbox; the tax is marshalling rows/edges into linear memory [3]. SFI can make the transition a call [1]. WASI still talks to the host kernel unless you decouple the interface [6]. CHERI/TEE are the hardware pole, not P0.

## 2. STCA applicability

Security 081 / Agent: a pack’s code is **untrusted relative to the log**. Compile Cypher/GPML in-process; run user/LLM-generated procedures behind a WASM (or later CHERI) fence with a *capability* WASI — only Ports they were granted. Do not let a UDF write the event log except through the same TOKI/receipt path. Cordon still owns *when* an effect commits; the sandbox owns *that the procedure cannot eat the process*.

## 3. Quality / cost

Usefulness high once packs exist. Optimality med: copy-elimination is mandatory or WASM loses to native. Cost: honeycomb 081 = WASM runtime + shared-memory Arrow/CSR views; no SGX-as-SoT.

## 4. Demand

Hindsight/Dify-class packs and agent tools will ask to “run this Python on the graph.” Engine demand: UDF port with a sandbox, not `eval` in the query worker.

## 5. Niche → effect

`no niche`
