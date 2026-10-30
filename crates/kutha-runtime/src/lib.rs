//! In-memory P0 runtime: log = SoT, fold = picture, quantum = emit→idle (ADR-010).

mod admission;
mod allow;
mod commit;
mod csr;
mod eval;
mod fold;
mod fold_diff;
mod fold_query;
mod gpml;
mod hydrate;
mod justification;
mod leapfrog;
mod log;
mod materializer;
mod persist_append;
mod pipeline;
mod provenance;
mod quantum;
mod query_ir;
mod receipt;
mod segment;
mod snapshot;
pub mod store;
mod tenant;
mod wal;

#[cfg(feature = "rocks-index")]
pub mod rocks_index;
#[cfg(feature = "rocks-index-spike")]
pub mod rocks_spike;

pub use csr::{CsrLease, TypedCsrLease, TypedEdge};
pub use fold::{Fact, FoldIndexMode, GraphFold};
pub use fold_diff::{diff_folds, FactChange, FoldDiff};
pub use leapfrog::{AdjacencyIter, leapfrog_intersect};
pub use log::EventLog;
pub use materializer::{CsrMaterializer, Materializer};
pub use quantum::{
    ActionRecord, ConflictReport, Justification, NormTriple, NormView, OutcomeDisposition,
    PersistedQuantumOutcome, QuantumOutcome, Runtime, RuntimeError, disposition,
};
pub use query_ir::{
    execute, prepare, try_hops_named, Prepared, PreparedHops, PreparedScan, QueryError, QueryIr,
    QueryRows, Workload,
};
pub use gpml::{parse_gpml, GpmlError, RuntimeBinder, VocabBinder};
pub use receipt::{QuantumReceipt, digest_to_hex};
pub use snapshot::Snapshot;
pub use tenant::{IngestReport, TenantError, ingest_harness_jsonl, ingest_harness_jsonl_str};
#[doc(hidden)]
pub use wal::write_json_payload_wal;
