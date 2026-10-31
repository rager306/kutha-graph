//! Bounded C4 foundation: closed-default path-policy decisions over existing
//! typed [`QueryIr`].
//!
//! This module is deliberately narrow. It answers "may this compiled query
//! use this path?" before `prepare` and does not implement grants-as-facts,
//! tenant slices, vacuum authority, capabilities, WASM, a policy language,
//! or all of ADR-080. Policy decisions are compile-time; ADR-080 stays
//! Proposed.

use crate::quantum::Runtime;
use crate::query_ir::{QueryIr, QueryRows, execute};
use std::collections::BTreeSet;
use std::fmt;

/// Closed-default path policy. An absent permission is a denial.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PathPolicy {
    allowed_relations: BTreeSet<String>,
    allow_scan: bool,
}

impl PathPolicy {
    /// Policy with no permissions; every scan/unlabeled walk denies.
    pub fn closed() -> Self {
        Self::default()
    }

    /// Closed policy with an explicit relation allowlist.
    pub fn allow_relations<I, S>(mut self, names: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.allowed_relations
            .extend(names.into_iter().map(Into::into));
        self
    }

    /// Explicitly permit `QueryIr::Scan`; absent means denied.
    pub fn allow_scan(mut self) -> Self {
        self.allow_scan = true;
        self
    }

    fn relation_allowed(&self, name: &str) -> bool {
        self.allowed_relations.contains(name)
    }
}

/// Compile-time decision. `Denied` must not be prepared or executed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AuthorizedQuery {
    Allowed(QueryIr),
    Denied { reason: String },
}

/// Typed security failure. Denied queries are not represented as empty rows.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SecurityError {
    PolicyDenied { reason: String },
    UnknownRelation { name: String },
    QueryRejected { reason: String },
}

impl fmt::Display for SecurityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PolicyDenied { reason } => {
                write!(f, "path policy denied query: {reason}")
            }
            Self::UnknownRelation { name } => {
                write!(f, "unknown relation {name} in path policy")
            }
            Self::QueryRejected { reason } => {
                write!(f, "authorized query rejected by query engine: {reason}")
            }
        }
    }
}

impl std::error::Error for SecurityError {}

/// Compile an existing typed query against a closed-default path policy.
///
/// This is a compile-time decision. It does not prepare a lease, materialize
/// rows, or change `QueryIr` semantics. `Scan` and unlabeled walks deny by
/// default; labeled hops require every relation to be allowed.
pub fn compile_path_policy(
    rt: &Runtime,
    q: &QueryIr,
    policy: &PathPolicy,
) -> Result<AuthorizedQuery, SecurityError> {
    match q {
        QueryIr::Scan { .. } => {
            if policy.allow_scan {
                Ok(AuthorizedQuery::Allowed(q.clone()))
            } else {
                Ok(AuthorizedQuery::Denied {
                    reason: "scan is denied by closed-default path policy".to_string(),
                })
            }
        }
        QueryIr::Hops { rels, k, .. } => {
            // A zero-hop query exposes one named start without a relation
            // constraint; closed default denies it with this foundation.
            if *k == 0 || rels.is_empty() {
                return Ok(AuthorizedQuery::Denied {
                    reason: "unlabeled walk is denied by closed-default path policy".to_string(),
                });
            }
            for rel in rels {
                let Some(name) = rt.dictionary().lookup(*rel) else {
                    return Err(SecurityError::UnknownRelation {
                        name: rel.to_string(),
                    });
                };
                let name = name.to_string();
                if !policy.relation_allowed(&name) {
                    return Ok(AuthorizedQuery::Denied {
                        reason: format!("relation {name} is not allowed"),
                    });
                }
            }
            Ok(AuthorizedQuery::Allowed(q.clone()))
        }
    }
}

/// Compile, then execute only an allowed query.
///
/// Denied queries return [`SecurityError::PolicyDenied`] before `prepare`;
/// they are never represented as empty [`QueryRows`].
pub fn execute_authorized(
    rt: &Runtime,
    q: &QueryIr,
    policy: &PathPolicy,
) -> Result<QueryRows, SecurityError> {
    match compile_path_policy(rt, q, policy)? {
        AuthorizedQuery::Allowed(allowed) => {
            execute(rt, &allowed).map_err(SecurityError::from_query_error)
        }
        AuthorizedQuery::Denied { reason } => Err(SecurityError::PolicyDenied { reason }),
    }
}

impl SecurityError {
    fn from_query_error(err: crate::query_ir::QueryError) -> Self {
        match err {
            crate::query_ir::QueryError::UnknownRelation { name } => Self::UnknownRelation { name },
            crate::query_ir::QueryError::UnknownTerm { name } => Self::QueryRejected {
                reason: format!("unknown term {name}"),
            },
            crate::query_ir::QueryError::RelsShape { rels, k } => Self::QueryRejected {
                reason: format!("invalid relation shape rels={rels} k={k}"),
            },
        }
    }
}
