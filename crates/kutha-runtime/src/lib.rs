//! In-memory P0 runtime: log = SoT, fold = picture, quantum = emit→idle (ADR-010).

mod allow;
mod csr;
mod fold;
mod leapfrog;
mod log;
mod materializer;
mod quantum;
mod receipt;
mod snapshot;
pub mod store;
mod tenant;
mod wal;

pub use csr::CsrLease;
pub use fold::{Fact, GraphFold};
pub use leapfrog::{leapfrog_intersect, AdjacencyIter};
pub use log::EventLog;
pub use materializer::{CsrMaterializer, Materializer};
pub use quantum::{QuantumOutcome, Runtime, RuntimeError};
pub use receipt::QuantumReceipt;
pub use snapshot::Snapshot;
pub use tenant::{ingest_harness_jsonl, ingest_harness_jsonl_str, IngestReport, TenantError};
