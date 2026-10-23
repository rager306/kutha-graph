//! Fact-set picture diff: what turns fold `a` into fold `b` (ADR-061).
//! Identity is `(event_id, valid_from, valid_to)` plus rank; `seq` does not participate.
use kutha_common::{EventId, SupportPolarity};
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};

use crate::fold::{Fact, GraphFold};

/// Fact-set picture diff: what turns fold `a` into fold `b` (ADR-061).
/// Identity is `(event_id, valid_from, valid_to)` plus rank; `seq` does not participate.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FoldDiff {
    pub added: Vec<Fact>,
    pub removed: Vec<Fact>,
    pub changed: Vec<FactChange>,
}

impl Default for FoldDiff {
    fn default() -> Self {
        Self {
            added: Vec::new(),
            removed: Vec::new(),
            changed: Vec::new(),
        }
    }
}

/// One `(key, rank)` pair whose non-`seq` fields differ between the two folds.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FactChange {
    pub before: Fact,
    pub after: Fact,
}

impl Default for FactChange {
    fn default() -> Self {
        Self {
            before: Fact::blank(),
            after: Fact::blank(),
        }
    }
}

/// `None` (open interval / no invalidation) is distinct from `Some(u64::MAX)` and ranks greater than any `Some`.
fn opt_u64_cmp(a: Option<u64>, b: Option<u64>) -> Ordering {
    match (a, b) {
        (None, None) => Ordering::Equal,
        (None, Some(_)) => Ordering::Greater,
        (Some(_), None) => Ordering::Less,
        (Some(x), Some(y)) => x.cmp(&y),
    }
}

fn polarity_rank(p: Option<SupportPolarity>) -> u8 {
    match p {
        None => 0,
        Some(SupportPolarity::Positive) => 1,
        Some(SupportPolarity::Negative) => 2,
    }
}

fn delivery_key_cmp(a: &Option<String>, b: &Option<String>) -> Ordering {
    match (a, b) {
        (None, None) => Ordering::Equal,
        (None, Some(_)) => Ordering::Less,
        (Some(_), None) => Ordering::Greater,
        (Some(x), Some(y)) => x.as_bytes().cmp(y.as_bytes()),
    }
}

/// Canonical total order over every Fact field except `seq` (D-02 / D-03).
fn canon_cmp(a: &Fact, b: &Fact) -> Ordering {
    a.event_id
        .cmp(&b.event_id)
        .then(a.valid_from.cmp(&b.valid_from))
        .then(opt_u64_cmp(a.valid_to, b.valid_to))
        .then(a.object().cmp(&b.object()))
        .then(a.relation.cmp(&b.relation))
        .then(a.subject.cmp(&b.subject))
        .then(a.claim_id.cmp(&b.claim_id))
        .then(delivery_key_cmp(&a.delivery_key, &b.delivery_key))
        .then(polarity_rank(a.polarity).cmp(&polarity_rank(b.polarity)))
        .then(a.ingested_at.cmp(&b.ingested_at))
        .then(opt_u64_cmp(a.invalidated_at, b.invalidated_at))
}

fn identity_key(f: &Fact) -> (EventId, u64, Option<u64>) {
    (f.event_id, f.valid_from, f.valid_to)
}

fn ranked_picture(facts: &[Fact]) -> BTreeMap<((EventId, u64, Option<u64>), usize), Fact> {
    let mut groups: BTreeMap<(EventId, u64, Option<u64>), Vec<Fact>> = BTreeMap::new();
    for f in facts {
        groups.entry(identity_key(f)).or_default().push(f.clone());
    }
    let mut out = BTreeMap::new();
    for (key, mut group) in groups {
        group.sort_by(canon_cmp);
        for (rank, fact) in group.into_iter().enumerate() {
            out.insert((key, rank), fact);
        }
    }
    out
}

fn same_except_seq(a: &Fact, b: &Fact) -> bool {
    a.subject == b.subject
        && a.relation == b.relation
        && a.object() == b.object()
        && a.valid_from == b.valid_from
        && a.valid_to == b.valid_to
        && a.ingested_at == b.ingested_at
        && a.invalidated_at == b.invalidated_at
        && a.claim_id == b.claim_id
        && a.event_id == b.event_id
        && a.delivery_key == b.delivery_key
        && a.polarity == b.polarity
}

/// What turns `a` into `b`. Keyed by `(event_id, valid_from, valid_to)` plus rank; `seq` never keys.
pub fn diff_folds(a: &GraphFold, b: &GraphFold) -> FoldDiff {
    let left = ranked_picture(a.facts());
    let right = ranked_picture(b.facts());
    let mut added = Vec::new();
    let mut removed = Vec::new();
    let mut changed = Vec::new();
    let ids: BTreeSet<_> = left.keys().chain(right.keys()).copied().collect();
    for id in ids {
        match (left.get(&id), right.get(&id)) {
            (None, Some(bf)) => added.push(bf.clone()),
            (Some(af), None) => removed.push(af.clone()),
            (Some(af), Some(bf)) => {
                if !same_except_seq(af, bf) {
                    changed.push(FactChange {
                        before: af.clone(),
                        after: bf.clone(),
                    });
                }
            }
            (None, None) => {}
        }
    }
    added.sort_by(canon_cmp);
    removed.sort_by(canon_cmp);
    changed.sort_by(|x, y| {
        let c = canon_cmp(&x.after, &y.after);
        if c != Ordering::Equal {
            c
        } else {
            canon_cmp(&x.before, &y.before)
        }
    });
    FoldDiff {
        added,
        removed,
        changed,
    }
}

impl GraphFold {
    /// Thin wrapper around [`diff_folds`].
    pub fn diff(&self, other: &GraphFold) -> FoldDiff {
        diff_folds(self, other)
    }
}
