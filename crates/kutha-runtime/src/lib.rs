//! In-memory P0 runtime: log = SoT, fold = picture, quantum = emit→idle (ADR-010).

mod admission;
mod allow;
mod commit;
mod csr;
mod eval;
mod fold;
mod fold_query;
mod hydrate;
mod justification;
mod leapfrog;
mod log;
mod materializer;
mod persist_append;
mod pipeline;
mod provenance;
mod quantum;
mod receipt;
mod snapshot;
pub mod store;
mod tenant;
mod wal;

#[cfg(feature = "rocks-index-spike")]
pub mod rocks_spike;

pub use csr::{CsrLease, TypedCsrLease, TypedEdge};
pub use fold::{Fact, FoldIndexMode, GraphFold};
pub use leapfrog::{leapfrog_intersect, AdjacencyIter};
pub use log::EventLog;
pub use materializer::{CsrMaterializer, Materializer};
pub use quantum::{
    disposition, ActionRecord, ConflictReport, Justification, OutcomeDisposition,
    PersistedQuantumOutcome, QuantumOutcome, Runtime, RuntimeError,
};
pub use receipt::{digest_to_hex, QuantumReceipt};
pub use snapshot::Snapshot;
pub use tenant::{ingest_harness_jsonl, ingest_harness_jsonl_str, IngestReport, TenantError};
