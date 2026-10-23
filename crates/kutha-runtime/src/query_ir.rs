//! Thin typed query IR over existing read APIs (M157 / Phase 231).
//!
//! Interned plain data, not on-log (FoldDiff pattern: no serde). `execute` is
//! read-only: `dictionary().id` for named lookup, never `intern` / `admit`.

use crate::quantum::Runtime;
use kutha_common::{TermId, TransactionTime, ValidTime};
use std::collections::BTreeSet;
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

fn term_name(rt: &Runtime, id: TermId) -> String {
    rt.dictionary()
        .lookup(id)
        .map(str::to_string)
        .unwrap_or_else(|| id.to_string())
}

fn require_term(rt: &Runtime, id: TermId) -> Result<(), QueryError> {
    if rt.dictionary().lookup(id).is_some() {
        Ok(())
    } else {
        Err(QueryError::UnknownTerm {
            name: term_name(rt, id),
        })
    }
}

fn require_rels_shape(rels: &[TermId], k: u32) -> Result<(), QueryError> {
    let n = rels.len();
    if n == 0 || n == 1 || n == k as usize {
        Ok(())
    } else {
        Err(QueryError::UnknownRelation {
            name: "rels/k mismatch".to_string(),
        })
    }
}

fn apply_limit<T>(mut rows: Vec<T>, limit: usize) -> Vec<T> {
    if limit > 0 && rows.len() > limit {
        rows.truncate(limit);
    }
    rows
}

/// Named hops ctor. Looks up `dictionary().id` only; never `intern`.
pub fn try_hops_named(
    rt: &Runtime,
    start: &str,
    rels: &[&str],
    k: u32,
    tt: TransactionTime,
    vt: ValidTime,
    limit: usize,
) -> Result<QueryIr, QueryError> {
    let start_id = rt
        .dictionary()
        .id(start)
        .ok_or_else(|| QueryError::UnknownTerm {
            name: start.to_string(),
        })?;
    let mut rel_ids = Vec::with_capacity(rels.len());
    for name in rels {
        let id = rt
            .dictionary()
            .id(name)
            .ok_or_else(|| QueryError::UnknownRelation {
                name: (*name).to_string(),
            })?;
        rel_ids.push(id);
    }
    require_rels_shape(&rel_ids, k)?;
    Ok(QueryIr::Hops {
        start: start_id,
        rels: rel_ids,
        k,
        tt,
        vt,
        limit,
    })
}

/// Interpret `q` over `rt` read APIs (`live_at` / CSR leases). Read-only.
pub fn execute(rt: &Runtime, q: &QueryIr) -> Result<QueryRows, QueryError> {
    match q {
        QueryIr::Scan { tt, vt, limit } => {
            let mut rows = rt.live_at(*tt, *vt);
            rows.sort_unstable();
            Ok(QueryRows::Triples(apply_limit(rows, *limit)))
        }
        QueryIr::Hops {
            start,
            rels,
            k,
            tt,
            vt,
            limit,
        } => {
            require_term(rt, *start)?;
            require_rels_shape(rels, *k)?;
            for id in rels {
                require_term(rt, *id)?;
            }
            if *k == 0 {
                return Ok(QueryRows::Nodes(apply_limit(vec![*start], *limit)));
            }
            let mut frontier: BTreeSet<TermId> = BTreeSet::from([*start]);
            if rels.is_empty() {
                let csr = rt.csr_lease_at(*tt, *vt);
                for _ in 0..*k {
                    let mut next = BTreeSet::new();
                    for v in frontier {
                        next.extend(csr.neighbors(v).iter().copied());
                    }
                    frontier = next;
                }
            } else {
                let typed = rt.typed_csr_lease_at(*tt, *vt);
                for hop in 0..*k {
                    let rel = if rels.len() == 1 {
                        rels[0]
                    } else {
                        rels[hop as usize]
                    };
                    let mut next = BTreeSet::new();
                    for v in frontier {
                        for e in typed.edges_out(v) {
                            if e.relation == rel {
                                next.insert(e.object);
                            }
                        }
                    }
                    frontier = next;
                }
            }
            let nodes: Vec<TermId> = frontier.into_iter().collect();
            Ok(QueryRows::Nodes(apply_limit(nodes, *limit)))
        }
    }
}
