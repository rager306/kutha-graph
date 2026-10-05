//! SEM-bitemporal-model: naive ADR-013 list-scan vs GraphFold::live_at / as_of.
//! Independent of fold.rs apply (no GraphFold::apply, no Fact::is_live_at).
//! Default ≥64 seeds; `KUTHA_SEEDS=<n>` may raise. Added runtime budget ≤30s default.

mod common;

use common::sem::{
    MixStyle, assert_seed_budget, fail_seed, ingest_mix, replay_runtime, seeds, shrink_events,
    sorted_as_of, sorted_live, vt_cuts,
};
use kutha_common::{Event, EventId, Op, TermId};
use kutha_runtime::{FoldIndexMode, Runtime};
use std::collections::BTreeMap;

/// Naive bitemporal picture (ADR-013 + FF5). Half-open VT; losers stay; list scan.
/// Targeting uses EventId only because this replay uses the same event list.
struct Row {
    subject: TermId,
    relation: TermId,
    object: TermId,
    valid_from: u64,
    valid_to: Option<u64>,
    ingested_at: u64,
    invalidated_at: Option<u64>,
    event_id: EventId,
}

struct Naive {
    rows: Vec<Row>,
    /// Wall → tt (ADR-013 as_known_on). Filled from TtAnchor; product oracles use live_at/as_of.
    #[allow(dead_code)]
    anchors: BTreeMap<u64, u64>,
}

impl Naive {
    fn from_events(events: &[Event]) -> Self {
        let mut p = Self {
            rows: Vec::new(),
            anchors: BTreeMap::new(),
        };
        for e in events {
            p.apply(e);
        }
        p
    }

    fn find_live(&self, event_id: EventId) -> Option<usize> {
        self.rows
            .iter()
            .enumerate()
            .filter(|(_, r)| r.event_id == event_id)
            .find(|(_, r)| r.invalidated_at.is_none())
            .map(|(i, _)| i)
            .or_else(|| self.rows.iter().rposition(|r| r.event_id == event_id))
    }

    #[allow(clippy::too_many_arguments)]
    fn push(
        &mut self,
        subject: TermId,
        relation: TermId,
        object: TermId,
        valid_from: u64,
        valid_to: Option<u64>,
        ingested_at: u64,
        event_id: EventId,
    ) {
        self.rows.push(Row {
            subject,
            relation,
            object,
            valid_from,
            valid_to,
            ingested_at,
            invalidated_at: None,
            event_id,
        });
    }

    fn apply(&mut self, event: &Event) {
        match &event.op {
            Op::Assert {
                subject,
                relation,
                object,
                valid_from,
                valid_to,
                ..
            }
            | Op::Behavior {
                subject,
                relation,
                object,
                valid_from,
                valid_to,
                ..
            } => self.push(
                *subject,
                *relation,
                *object,
                *valid_from,
                *valid_to,
                event.ingested_at,
                event.id,
            ),
            Op::Retract { event_id } => {
                if let Some(i) = self.find_live(*event_id)
                    && self.rows[i].invalidated_at.is_none()
                {
                    self.rows[i].invalidated_at = Some(event.ingested_at);
                }
            }
            Op::Correct {
                event_id,
                object,
                valid_from,
                valid_to,
            } => {
                let Some(i) = self.find_live(*event_id) else {
                    return;
                };
                if self.rows[i].invalidated_at.is_some() {
                    return;
                }
                let s = self.rows[i].subject;
                let r = self.rows[i].relation;
                self.rows[i].invalidated_at = Some(event.ingested_at);
                self.push(
                    s,
                    r,
                    *object,
                    *valid_from,
                    *valid_to,
                    event.ingested_at,
                    event.id,
                );
            }
            Op::CorrectInterval {
                event_id,
                object,
                patch_from,
                patch_to,
            } => {
                let Some(i) = self.find_live(*event_id) else {
                    return;
                };
                if self.rows[i].invalidated_at.is_some() {
                    return;
                }
                let old_from = self.rows[i].valid_from;
                let old_to = self.rows[i].valid_to;
                let Some((a, b)) = intersect(old_from, old_to, *patch_from, *patch_to) else {
                    return;
                };
                let s = self.rows[i].subject;
                let r = self.rows[i].relation;
                let old_o = self.rows[i].object;
                self.rows[i].invalidated_at = Some(event.ingested_at);
                if old_from < a {
                    self.push(s, r, old_o, old_from, Some(a), event.ingested_at, event.id);
                }
                self.push(s, r, *object, a, b, event.ingested_at, event.id);
                if let Some(end) = b {
                    let old_end = old_to.unwrap_or(u64::MAX);
                    if end < old_end {
                        self.push(s, r, old_o, end, old_to, event.ingested_at, event.id);
                    }
                }
            }
            Op::TtAnchor { wall } => {
                self.anchors.insert(*wall, event.ingested_at);
            }
            Op::Define { .. }
            | Op::QuantumOutcome { .. }
            | Op::JustificationCite { .. }
            | Op::StoreBlob { .. } => {}
        }
    }

    fn is_live(row: &Row, tt: u64, vt: u64) -> bool {
        if row.ingested_at > tt {
            return false;
        }
        if row.invalidated_at.is_some_and(|inv| inv <= tt) {
            return false;
        }
        if vt < row.valid_from {
            return false;
        }
        if row.valid_to.is_some_and(|to| vt >= to) {
            return false;
        }
        true
    }

    fn live_at(&self, tt: u64, vt: u64) -> Vec<(TermId, TermId, TermId)> {
        let mut out: Vec<_> = self
            .rows
            .iter()
            .filter(|r| Self::is_live(r, tt, vt))
            .map(|r| (r.subject, r.relation, r.object))
            .collect();
        out.sort_unstable();
        out
    }

    fn as_of(&self, vt: u64) -> Vec<(TermId, TermId, TermId)> {
        self.live_at(u64::MAX, vt)
    }
}

fn intersect(
    a_from: u64,
    a_to: Option<u64>,
    b_from: u64,
    b_to: Option<u64>,
) -> Option<(u64, Option<u64>)> {
    let from = a_from.max(b_from);
    let end = a_to.unwrap_or(u64::MAX).min(b_to.unwrap_or(u64::MAX));
    if from >= end {
        return None;
    }
    Some((from, (end != u64::MAX).then_some(end)))
}

fn product_live(naive: &Naive, rt: &Runtime, tt: u64, vt: u64) -> Vec<(TermId, TermId, TermId)> {
    let mut out: Vec<_> = naive
        .rows
        .iter()
        .filter(|r| {
            Naive::is_live(r, tt, vt)
                && !rt
                    .dictionary()
                    .lookup(r.relation)
                    .is_some_and(|n| n.starts_with("kutha:"))
        })
        .map(|r| (r.subject, r.relation, r.object))
        .collect();
    out.sort_unstable();
    out
}

#[test]
fn bitemporal_model_matches_fold_live_at_and_as_of() {
    assert_seed_budget();
    for (case, seed) in seeds().into_iter().enumerate() {
        let mut rt = Runtime::with_index_mode(32, FoldIndexMode::Eager);
        ingest_mix(&mut rt, seed, MixStyle::BehaviorFree);
        let events = rt.log().as_slice();
        let naive = Naive::from_events(events);
        for vt in vt_cuts() {
            let fold_as_of = sorted_as_of(&rt, vt);
            let model_as_of = product_live(&naive, &rt, u64::MAX, vt);
            if fold_as_of != model_as_of {
                let shrunk = shrink_events(events, |prefix| {
                    let r = replay_runtime(&rt, prefix);
                    let n = Naive::from_events(prefix);
                    sorted_as_of(&r, vt) != product_live(&n, &r, u64::MAX, vt)
                });
                eprintln!("shrunk ops={}", common::sem::brief_ops(&shrunk));
                fail_seed(
                    seed,
                    case,
                    &shrunk,
                    "model as_of must match GraphFold::as_of",
                );
            }
            let fold_live = sorted_live(&rt, u64::MAX, vt);
            if fold_live != fold_as_of {
                fail_seed(seed, case, events, "as_of must equal live_at(MAX, vt)");
            }
        }
        let tts: Vec<u64> = naive.rows.iter().map(|r| r.ingested_at).collect();
        if let Some(&mid) = tts.get(tts.len() / 2) {
            for vt in vt_cuts() {
                let fold_live = sorted_live(&rt, mid, vt);
                let model_live = product_live(&naive, &rt, mid, vt);
                if fold_live != model_live {
                    fail_seed(
                        seed,
                        case,
                        events,
                        "model live_at(mid tt) must match GraphFold::live_at",
                    );
                }
            }
        }
    }
}

#[test]
fn bitemporal_model_agrees_on_ff5_statute() {
    const T_OLD: u64 = 2015;
    const T_NEW: u64 = 2021;
    let mut rt = Runtime::default();
    let article = rt.intern("art-12-speed");
    let in_force = rt.intern("inForceAs");
    let limit_50 = rt.intern("50-kmh");
    let limit_30 = rt.intern("30-kmh");
    rt.emit(Op::Assert {
        subject: article,
        relation: in_force,
        object: limit_50,
        valid_from: 2010,
        valid_to: Some(2020),
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    rt.emit(Op::Assert {
        subject: article,
        relation: in_force,
        object: limit_30,
        valid_from: 2020,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    let naive = Naive::from_events(rt.log().as_slice());
    let old = product_live(&naive, &rt, u64::MAX, T_OLD);
    let new = product_live(&naive, &rt, u64::MAX, T_NEW);
    assert_ne!(old, new, "FF5: two VT windows must differ");
    assert_eq!(old, sorted_as_of(&rt, T_OLD));
    assert_eq!(new, sorted_as_of(&rt, T_NEW));
    assert!(old.contains(&(article, in_force, limit_50)));
    assert!(new.contains(&(article, in_force, limit_30)));
}

// Silence unused Naive::as_of / dict_rel (kept as the ADR as_of = live_at(MAX) alias).
#[test]
fn model_as_of_alias_is_live_at_max() {
    let mut rt = Runtime::default();
    ingest_mix(&mut rt, 42, MixStyle::BehaviorFree);
    let n = Naive::from_events(rt.log().as_slice());
    assert_eq!(n.as_of(2017), n.live_at(u64::MAX, 2017));
}
