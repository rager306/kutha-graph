---
id: samyama-agentic-enrichment-gak
source: samyama
axes: [Agent, Composition]
usefulness: med
optimality: med
demand: med
confidence: observed
layer5: no niche
status: closed
channels_failed: []
---

# Generation-augmented knowledge (GAK) enrichment as add-on

Evidence: `examples/agentic_enrichment_demo.rs`; README: optional Claude CLI. Skill text: agentic enrichment, not kernel SoT.

## 1. Raw idea

LLM enrichment writes derived graph content after the store exists.

## 2. STCA applicability

Agent: optional pack. Composition: must be reversible / CA-cached. Aligns with Raven pattern but Samyama keeps it out of the trust core.

## 3. Quality / cost

Observed demo, not audited as fail-closed. Cost: sync extract latency (Memanto D6) if it sits on the write path.

## 4. Demand

Users want “fill the graph from text”; Kutha must not make that mandatory for truth.

## 5. Niche → effect

`no niche`
