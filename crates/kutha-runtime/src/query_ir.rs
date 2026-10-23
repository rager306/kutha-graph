//! Thin typed query IR over existing read APIs (M157 / Phase 231).
//!
//! Interned plain data, not on-log (FoldDiff pattern: no serde). `execute` is
//! read-only: `dictionary().id` for named lookup, never `intern` / `admit`.
//!
//! [`prepare`] validates names/shape and snapshots the Scan bag or CSR lease
//! at the query's `(tt, vt)` cut. [`Prepared::run`] walks that snapshot.
//! `execute(rt, q)` is `prepare` then `run` (same rows). A lease is bound to
//! its cut and fold picture; it is not live-updating. A query at a different
//! cut cannot be run through this snapshot (`run` does not take a [`QueryIr`]),
//! so mismatched-cut reuse is unrepresentable rather than silently applied.

use crate::csr::{CsrLease, TypedCsrLease};
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

/// Fail-closed intern-map miss or hops `rels` shape. Relation Display copies [`crate::RuntimeError`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum QueryError {
    UnknownRelation {
        name: String,
    },
    UnknownTerm {
        name: String,
    },
    /// `rels.len()` is not 0 (unlabeled), 1 (same rel every hop), or `k` (per-hop).
    RelsShape {
        rels: usize,
        k: usize,
    },
}

impl fmt::Display for QueryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QueryError::UnknownRelation { name } => {
                write!(f, "unknown relation {name} (not in allowlist)")
            }
            QueryError::UnknownTerm { name } => write!(f, "unknown term {name}"),
            QueryError::RelsShape { rels, k } => {
                write!(
                    f,
                    "rels length {rels} is not 0, 1, or k={k} (unlabeled, repeated, or per-hop)"
                )
            }
        }
    }
}

impl std::error::Error for QueryError {}

/// CSR snapshot used by a prepared hops walk. Bound to one `(tt, vt)` cut.
#[derive(Clone, Debug)]
enum HopCsr {
    Untyped(CsrLease),
    Typed(TypedCsrLease),
}

/// Sorted Scan bag captured at prepare time (one fold picture, one cut).
#[derive(Clone, Debug)]
pub struct PreparedScan {
    rows: Vec<(TermId, TermId, TermId)>,
}

impl PreparedScan {
    /// Return the snapshot bag. Not live-updating.
    pub fn run(&self) -> QueryRows {
        QueryRows::Triples(self.rows.clone())
    }
}

/// Hops walk over a CSR lease captured at prepare time.
///
/// The lease is bound to the query's `(tt, vt)` cut and the fold as of
/// [`prepare`]. It is not live-updating. Reuse it for many starts at that
/// same cut via [`PreparedHops::run_from`] (bench: n layer-0 starts).
#[derive(Clone, Debug)]
pub struct PreparedHops {
    start: TermId,
    rels: Vec<TermId>,
    k: u32,
    limit: usize,
    /// `None` when `k == 0` (no walk, no lease).
    csr: Option<HopCsr>,
    dict_len: usize,
}

impl PreparedHops {
    /// Walk from the start captured at [`prepare`].
    pub fn run(&self) -> QueryRows {
        walk_hops(
            self.start,
            &self.rels,
            self.k,
            self.limit,
            self.csr.as_ref(),
        )
    }

    /// Walk from `start` on this snapshot. Same cut, same lease, new frontier.
    ///
    /// Fail-closed if `start` was not in the dictionary at prepare (out of
    /// range `TermId` is [`QueryError::UnknownTerm`]). Does not accept a
    /// [`QueryIr`], so a different `(tt, vt)` cannot be applied.
    pub fn run_from(&self, start: TermId) -> Result<QueryRows, QueryError> {
        if (start as usize) >= self.dict_len {
            return Err(QueryError::UnknownTerm {
                name: start.to_string(),
            });
        }
        Ok(walk_hops(
            start,
            &self.rels,
            self.k,
            self.limit,
            self.csr.as_ref(),
        ))
    }
}

/// Interpreter snapshot at one `(tt, vt)` cut and one fold picture.
///
/// Not live-updating. [`Prepared::run`] does not take a [`QueryIr`]: a query
/// whose cut differs from this snapshot cannot silently reuse it.
#[derive(Clone, Debug)]
pub enum Prepared {
    Scan(PreparedScan),
    Hops(PreparedHops),
}

impl Prepared {
    /// Run the prepared query as of the snapshot (same start/limit/shape).
    pub fn run(&self) -> QueryRows {
        match self {
            Prepared::Scan(s) => s.run(),
            Prepared::Hops(h) => h.run(),
        }
    }
}

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
        Err(QueryError::RelsShape {
            rels: n,
            k: k as usize,
        })
    }
}

fn apply_limit<T>(mut rows: Vec<T>, limit: usize) -> Vec<T> {
    if limit > 0 && rows.len() > limit {
        rows.truncate(limit);
    }
    rows
}

fn walk_hops(
    start: TermId,
    rels: &[TermId],
    k: u32,
    limit: usize,
    csr: Option<&HopCsr>,
) -> QueryRows {
    if k == 0 {
        return QueryRows::Nodes(apply_limit(vec![start], limit));
    }
    let mut frontier: BTreeSet<TermId> = BTreeSet::from([start]);
    match csr {
        Some(HopCsr::Untyped(csr)) => {
            for _ in 0..k {
                let mut next = BTreeSet::new();
                for v in frontier {
                    next.extend(csr.neighbors(v).iter().copied());
                }
                frontier = next;
            }
        }
        Some(HopCsr::Typed(typed)) => {
            for hop in 0..k {
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
        None => {
            // k > 0 always builds a lease in `prepare`.
            return QueryRows::Nodes(Vec::new());
        }
    }
    let nodes: Vec<TermId> = frontier.into_iter().collect();
    QueryRows::Nodes(apply_limit(nodes, limit))
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

/// Validate names/shape and snapshot Scan rows or the CSR lease at `q`'s cut.
///
/// The snapshot is bound to that `(tt, vt)` and the current fold. It is not
/// live-updating; call `prepare` again after `emit` or for a different cut.
pub fn prepare(rt: &Runtime, q: &QueryIr) -> Result<Prepared, QueryError> {
    match q {
        QueryIr::Scan { tt, vt, limit } => {
            let mut rows = rt.live_at(*tt, *vt);
            rows.sort_unstable();
            Ok(Prepared::Scan(PreparedScan {
                rows: apply_limit(rows, *limit),
            }))
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
            let dict_len = rt.dictionary().len();
            let csr = if *k == 0 {
                None
            } else if rels.is_empty() {
                Some(HopCsr::Untyped(rt.csr_lease_at(*tt, *vt)))
            } else {
                Some(HopCsr::Typed(rt.typed_csr_lease_at(*tt, *vt)))
            };
            Ok(Prepared::Hops(PreparedHops {
                start: *start,
                rels: rels.clone(),
                k: *k,
                limit: *limit,
                csr,
                dict_len,
            }))
        }
    }
}

/// Interpret `q` over `rt` read APIs (`live_at` / CSR leases). Read-only.
///
/// Equal to `prepare(rt, q)?.run()` (same rows).
pub fn execute(rt: &Runtime, q: &QueryIr) -> Result<QueryRows, QueryError> {
    Ok(prepare(rt, q)?.run())
}
