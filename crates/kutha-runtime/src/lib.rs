//! In-memory P0 runtime: log = SoT, fold = picture, quantum = emit→idle (ADR-010).

mod allow;
mod csr;
mod fold;
mod leapfrog;
mod log;
mod materializer;
mod admission;
mod fold_query;
mod hydrate;
mod justification;
mod pipeline;
mod provenance;
mod quantum;
mod receipt;
mod snapshot;
pub mod store;
mod tenant;
mod wal;

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
