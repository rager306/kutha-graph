# Phase 92-01 SUMMARY

`Runtime::derive` premise lookup requires `invalidated_at.is_none()`, matching retract/correct. Sole retracted premise → `UnknownFact`; later retract leaves prior live Assert as `caused_by`. Honeycomb Proposed. Oracles: `m032_derive_live_premise.rs`.
