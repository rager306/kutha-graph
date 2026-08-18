---
id: paper-graph-watermark-fingerprint
source: paper
axes: [Security, Verify, Data]
usefulness: high
optimality: med
demand: high
confidence: paper
layer5: "Legal/enterprise: prove a leaked fold is yours — watermark/fingerprint is a perturbation or external signature lease, not SoT and not a crypto receipt"
status: closed
channels_failed: []
---

# A watermark is a detectable perturbation — not SoT and not STE

Papers: [watermarking maps](https://consensus.app/papers/details/f41b367015065ec09b1b3a4ece9008b9/?utm_source=cursor) (Khanna et al., 2000) unique length perturbations on a road graph; owner identifies which copy a *query-only* provider is using from shortest-path answers; modifications are not independent because paths accumulate error [11]; [graph-DB watermark via pseudo-nodes](https://consensus.app/papers/details/00b18f01e9ae5cefbc81dfec700cc795/?utm_source=cursor) (Hristov et al., 2023) randomized dummy vertices instead of GA-optimized fakes [4]; [Fing](https://consensus.app/papers/details/16adcfa51a6b5e9db9fa0764d476bb5b/?utm_source=cursor) (Drosis et al., 2025) *fingerprint without mutating* the graph: NP-hard Factor-r Sum Subsets signature stored in an external timestamped DB; robust to thousands of edge deletions [8]; [KGMark](https://consensus.app/papers/details/0607bfd52aba5decba0c3124ee281406/?utm_source=cursor) diffusion fingerprints for *dynamic* KGs (spatial alignment + redundant embed) [5]; [DRGW](https://consensus.app/papers/details/5e40e429b8ad5ea2aaa8d5e1d34a9cea/?utm_source=cursor) disentangled carrier vs structure; invertible embed [10]; relational watermark surveys for ownership/tamper/traitor-tracing [9][16]. Distinct from `paper-constant-size-evidence` (integrity receipt of the *log*), `paper-graph-structured-encryption` (query an untrusted lease), `paper-graph-differential-privacy` (noise for publish, not ownership). Diffusion-image / DNN watermarks stay queued. Software-CFG watermarks (Collberg) are not data watermarks.

## 1. Raw idea

Each distributed copy of a graph gets a hidden mark: tiny weight edits [11], dummy nodes [4], or an *external* fingerprint that never touches the edges [8]. Later, query answers or a signature check say whose copy leaked. The mark is **not truth**.

## 2. STCA applicability

Security: watermark is a pack over a *sealed export* (RVF cousin), not ABAC. Verify: detection is a hypothesis test / NP-hard extract, not a quantum receipt of events. Data: mutating watermarks are a **lossy overlay**; Fing is a verify lease beside the log. Time: KGMark’s temporal robustness is about surviving updates, not valid-time. Agent: LLM does not pick the mark. Do not append watermark bits as SoT events unless they are explicit compensating facts.

## 3. Quality / cost

Usefulness high: dumps get copied. Optimality med: map-watermarking and Fing are real; dummy-node schemes distort MATCH. Cost: P0 = external fingerprint (Fing-shaped) of a sealed snapshot; honeycomb = optional perturbative mark on an *export*, never on the live log. Do not vendor KGMark diffusion into the core.

## 4. Demand

A leaked legal/science dump must be attributable. Engine demand: fingerprint a sealed fold; fail-closed so the live log stays unmarked.

## 5. Niche → effect

Legal/enterprise: prove a leaked fold is yours — watermark/fingerprint is a perturbation or external signature lease, not SoT and not a crypto receipt
