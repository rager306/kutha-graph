---
id: ruvector-rvf-cow-seal
source: ruvector
axes: [Packaging, Time, Verify]
usefulness: med
optimality: med
demand: med
confidence: code
layer5: "Scientific/legal export: sealed portable slice of a revision, not live adjacency"
status: closed
channels_failed: []
---

# RVF copy-on-write freeze / portable seal

Evidence: `examples/rvf/examples/snapshot_freeze.rs` (`CowEngine.freeze()`, `snapshot_epoch`); WitnessChain types in examples. ADR-090/093 already reject RVF as SoT.

## 1. Raw idea

COW engine freeze/fork into a portable cognitive/witness container.

## 2. STCA applicability

Packaging (ADR-091): export of a fold, not primary store. Time: snapshot epoch. Verify: witness receipts vs quantum receipts.

## 3. Quality / cost

Real freeze examples exist; product surface is also marketing-heavy. Cost: schema bridge to event-segment + dict@version without making RVF the log.

## 4. Demand

Portable “take this belief slice elsewhere” without standing up the engine.

## 5. Niche → effect

Scientific/legal export: sealed revision slice; live graph remains a fold of the log.
