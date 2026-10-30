//! In-memory P0 runtime: log = SoT, fold = picture, quantum = emit→idle (ADR-010).

mod admission;
mod allow;
mod commit;
mod csr;
mod enrichment;
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
mod pack_lifecycle;
mod pack_manifest;
mod pack_registry;
mod persist_append;
mod pipeline;
mod provenance;
mod quantum;
mod query_ir;
mod receipt;
mod security;
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
pub use enrichment::{EnrichmentError, EnrichmentRecord, derive_enrichment_ledger};
pub use fold::{Fact, FoldIndexMode, GraphFold};
pub use fold_diff::{FactChange, FoldDiff, diff_folds};
pub use gpml::{GpmlError, RuntimeBinder, VocabBinder, parse_gpml};
pub use leapfrog::{AdjacencyIter, leapfrog_intersect};
pub use log::EventLog;
pub use materializer::{CsrMaterializer, Materializer};
pub use pack_lifecycle::{PackLifecycle, PackLifecycleState, derive_pack_lifecycle};
pub use pack_manifest::{
    PACK_MANIFEST_SCHEMA, PackManifest, PackManifestError, declared_operators, load_manifest,
    valid_operator_name,
};
pub use pack_registry::{PackOperator, derive_pack_operators};
pub use quantum::{
    ActionRecord, ConflictReport, Justification, NormTriple, NormView, OutcomeDisposition,
    PersistedQuantumOutcome, QuantumOutcome, Runtime, RuntimeError, disposition,
};
pub use query_ir::{
    Prepared, PreparedHops, PreparedScan, QueryError, QueryIr, QueryRows, Workload, execute,
    prepare, try_hops_named,
};
pub use receipt::{QuantumReceipt, digest_to_hex};
pub use security::{
    AuthorizedQuery, PathPolicy, SecurityError, compile_path_policy, execute_authorized,
};
pub use snapshot::Snapshot;
pub use tenant::{IngestReport, TenantError, ingest_harness_jsonl, ingest_harness_jsonl_str};
#[doc(hidden)]
pub use wal::write_json_payload_wal;
