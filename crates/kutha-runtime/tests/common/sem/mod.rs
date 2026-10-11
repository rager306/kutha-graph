//! M019 S07 shared LCG, stamps, mix ingest, persist layouts.
//! Split from a flat helper (CE whole-repo #12). Behavior-preserving.
#![allow(dead_code)]

pub mod generator;
pub mod oracles;
pub mod persistence;
pub mod shrink;
