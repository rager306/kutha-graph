//! Thin typed query IR over existing read APIs (M157 / Phase 231).
//!
//! Interned plain data, not on-log (FoldDiff pattern: no serde). `execute` is
//! read-only: `dictionary().id` for named lookup, never `intern` / `admit`.

use crate::quantum::Runtime;
use kutha_common::{TermId, TransactionTime, ValidTime};
use std::fmt;

/// Workload alias: the hop/scan IR is the leased B3-lite workload.
pub type Workload = QueryIr;

/// Interned scan or k-hop over a named (tt, vt) cut.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum QueryIr {
    /// Product `live_at` bag, sorted; `limit == 0` is unlimited.
    /// `tt == TransactionTime::MAX` equals `as_of(vt)`.
    Scan {
        tt: TransactionTime,
        vt: ValidTime,
        limit: usize,
    },
    /// Unique end nodes after `k` hops. Empty `rels` = unlabeled; len 1 = same
    /// rel every hop; `len == k` = per-hop; otherwise error. `k == 0` is `{start}`.
    Hops {
        start: TermId,
        rels: Vec<TermId>,
        k: u32,
        tt: TransactionTime,
        vt: ValidTime,
        limit: usize,
    },
}

/// Scan returns a sorted triple bag; hops return sorted unique end nodes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum QueryRows {
    Triples(Vec<(TermId, TermId, TermId)>),
    Nodes(Vec<TermId>),
}

/// Fail-closed intern-map miss. Relation Display copies [`crate::RuntimeError`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum QueryError {
    UnknownRelation { name: String },
    UnknownTerm { name: String },
}

impl fmt::Display for QueryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QueryError::UnknownRelation { name } => {
                write!(f, "unknown relation {name} (not in allowlist)")
            }
            QueryError::UnknownTerm { name } => write!(f, "unknown term {name}"),
        }
    }
}

impl std::error::Error for QueryError {}

/// Named hops ctor. Looks up `dictionary().id` only; never `intern`.
pub fn try_hops_named(
    _rt: &Runtime,
    _start: &str,
    _rels: &[&str],
    k: u32,
    tt: TransactionTime,
    vt: ValidTime,
    limit: usize,
) -> Result<QueryIr, QueryError> {
    Ok(QueryIr::Hops {
        start: 0,
        rels: Vec::new(),
        k,
        tt,
        vt,
        limit,
    })
}

/// Interpret `q` over `rt` read APIs. Stub: empty triple bag (RED).
pub fn execute(_rt: &Runtime, _q: &QueryIr) -> Result<QueryRows, QueryError> {
    Ok(QueryRows::Triples(vec![]))
}
