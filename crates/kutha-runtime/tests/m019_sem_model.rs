//! SEM-bitemporal-model: naive ADR-013 list-scan vs GraphFold::live_at / as_of.
//! Independent of fold.rs apply (no GraphFold::apply, no Fact::is_live_at).
//! Default ≥64 seeds; `KUTHA_SEEDS=<n>` may raise. Added runtime budget ≤30s default.
//! Row+Naive+intersect ≤200 physical lines.

mod common;

use common::sem::{
    EDGE_VT, MixStyle, assert_seed_budget, fail_seed, fail_shrunk, fold_stamps, ingest_mix,
    is_product_rel, replay_runtime, runtime_stamps, seeds, sorted_as_of, sorted_live, vt_cuts_for,
};
use kutha_common::{Event, EventId, Op, TermId};
use kutha_runtime::{FoldIndexMode, GraphFold, Runtime};

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
}

impl Naive {
    fn from_events(events: &[Event]) -> Self {
        let mut p = Self { rows: Vec::new() };
        for e in events {
            p.apply(e);
        }
        p
    }

    fn push(&mut self, r: Row) {
        self.rows.push(r);
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
            } => self.push(Row {
                subject: *subject,
                relation: *relation,
                object: *object,
                valid_from: *valid_from,
                valid_to: *valid_to,
                ingested_at: event.ingested_at,
                invalidated_at: None,
                event_id: event.id,
            }),
            Op::Retract { event_id } => {
                // Match GraphFold: invalidate every live residual sharing the id
                // (CorrectInterval may mint prefix/patch/suffix under one event_id).
                let live: Vec<usize> = self
                    .rows
                    .iter()
                    .enumerate()
                    .filter(|(_, r)| r.event_id == *event_id && r.invalidated_at.is_none())
                    .map(|(i, _)| i)
                    .collect();
                for i in live {
                    self.rows[i].invalidated_at = Some(event.ingested_at);
                }
            }
            Op::Correct {
                event_id,
                object,
                valid_from,
                valid_to,
            } => {
                // Match GraphFold G-A02: invalidate all live residuals, mint one.
                let live: Vec<usize> = self
                    .rows
                    .iter()
                    .enumerate()
                    .filter(|(_, r)| r.event_id == *event_id && r.invalidated_at.is_none())
                    .map(|(i, _)| i)
                    .collect();
                let Some(&head) = live.first() else {
                    return;
                };
                let s = self.rows[head].subject;
                let r = self.rows[head].relation;
                for i in live {
                    self.rows[i].invalidated_at = Some(event.ingested_at);
                }
                self.push(Row {
                    subject: s,
                    relation: r,
                    object: *object,
                    valid_from: *valid_from,
                    valid_to: *valid_to,
                    ingested_at: event.ingested_at,
                    invalidated_at: None,
                    event_id: event.id,
                });
            }
            Op::CorrectInterval {
                event_id,
                object,
                patch_from,
                patch_to,
            } => {
                // Match pipeline G-A02: multi-residual CI is rejected (no-op here).
                let live: Vec<usize> = self
                    .rows
                    .iter()
                    .enumerate()
                    .filter(|(_, r)| r.event_id == *event_id && r.invalidated_at.is_none())
                    .map(|(i, _)| i)
                    .collect();
                if live.len() != 1 {
                    return;
                }
                let i = live[0];
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
                    self.push(Row {
                        subject: s,
                        relation: r,
                        object: old_o,
                        valid_from: old_from,
                        valid_to: Some(a),
                        ingested_at: event.ingested_at,
                        invalidated_at: None,
                        event_id: event.id,
                    });
                }
                self.push(Row {
                    subject: s,
                    relation: r,
                    object: *object,
                    valid_from: a,
                    valid_to: b,
                    ingested_at: event.ingested_at,
                    invalidated_at: None,
                    event_id: event.id,
                });
                if let Some(end) = b {
                    let old_end = old_to.unwrap_or(u64::MAX);
                    if end < old_end {
                        self.push(Row {
                            subject: s,
                            relation: r,
                            object: old_o,
                            valid_from: end,
                            valid_to: old_to,
                            ingested_at: event.ingested_at,
                            invalidated_at: None,
                            event_id: event.id,
                        });
                    }
                }
            }
            Op::TtAnchor { .. }
            | Op::Define { .. }
            | Op::QuantumOutcome { .. }
            | Op::JustificationCite { .. }
            | Op::StoreBlob { .. } => {}
        }
    }

    fn is_live(row: &Row, tt: u64, vt: u64) -> bool {
        row.ingested_at <= tt
            && row.invalidated_at.is_none_or(|inv| inv > tt)
            && vt >= row.valid_from
            && row.valid_to.is_none_or(|to| vt < to)
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

fn mix_vf_range(naive: &Naive, rt: &Runtime) -> (u64, u64) {
    let mut min_vf = u64::MAX;
    let mut max_vf = 0u64;
    for r in &naive.rows {
        if rt
            .dictionary()
            .lookup(r.relation)
            .is_some_and(|n| n.starts_with("kutha:"))
        {
            continue;
        }
        if r.valid_from < 2010 {
            continue;
        }
        min_vf = min_vf.min(r.valid_from);
        max_vf = max_vf.max(r.valid_from);
    }
    if min_vf == u64::MAX {
        (2010, 2014)
    } else {
        (min_vf, max_vf.max(min_vf))
    }
}

fn pin_is_live_at_vs_naive(naive: &Naive, rt: &Runtime, seed: u32, case: usize) {
    let tt = u64::MAX;
    let vt = EDGE_VT;
    for f in rt.fold().facts() {
        if !is_product_rel(rt, f.relation) {
            continue;
        }
        let Some(row) = naive.rows.iter().find(|r| {
            r.event_id == f.event_id
                && r.subject == f.subject
                && r.relation == f.relation
                && r.object == f.object()
                && r.ingested_at == f.ingested_at
                && r.valid_from == f.valid_from
                && r.valid_to == f.valid_to
        }) else {
            continue;
        };
        if Naive::is_live(row, tt, vt) != f.is_live_at(tt, vt) {
            fail_seed(
                seed,
                case,
                rt.log().as_slice(),
                "Fact::is_live_at must match naive VT/TT at EDGE_VT (skip valid_from dies; Eager live_at via vt_by_from would mask it)",
            );
        }
    }
}

#[test]
fn bitemporal_model_matches_fold_live_at_and_as_of() {
    assert_seed_budget();
    for (case, seed) in seeds().into_iter().enumerate() {
        let mut rt = Runtime::with_index_mode(32, FoldIndexMode::Eager);
        ingest_mix(&mut rt, seed, MixStyle::BehaviorFree);
        let events = rt.log().as_slice();
        let naive = Naive::from_events(events);
        let replayed = GraphFold::replay(events);
        if fold_stamps(&replayed, &rt) != runtime_stamps(&rt) {
            fail_shrunk(
                seed,
                case,
                events,
                |prefix| {
                    let r = replay_runtime(&rt, prefix);
                    fold_stamps(&GraphFold::replay(prefix), &r) != runtime_stamps(&r)
                },
                "GraphFold::replay stamps must equal ingest (Retract-first replay dies)",
            );
        }
        pin_is_live_at_vs_naive(&naive, &rt, seed, case);
        let (min_vf, max_vf) = mix_vf_range(&naive, &rt);
        for vt in vt_cuts_for(min_vf, max_vf) {
            let fold_as_of = sorted_as_of(&rt, vt);
            let model_as_of = product_live(&naive, &rt, u64::MAX, vt);
            if fold_as_of != model_as_of {
                fail_shrunk(
                    seed,
                    case,
                    events,
                    |prefix| {
                        let r = replay_runtime(&rt, prefix);
                        let n = Naive::from_events(prefix);
                        sorted_as_of(&r, vt) != product_live(&n, &r, u64::MAX, vt)
                    },
                    "model as_of must match GraphFold::as_of (triples are (s,r,o); object is compared)",
                );
            }
            let fold_live = sorted_live(&rt, u64::MAX, vt);
            if fold_live != fold_as_of {
                fail_seed(seed, case, events, "as_of must equal live_at(MAX, vt)");
            }
        }
        let tts: Vec<u64> = naive.rows.iter().map(|r| r.ingested_at).collect();
        if let Some(&mid) = tts.get(tts.len() / 2) {
            for vt in vt_cuts_for(min_vf, max_vf) {
                let fold_live = sorted_live(&rt, mid, vt);
                let model_live = product_live(&naive, &rt, mid, vt);
                if fold_live != model_live {
                    fail_shrunk(
                        seed,
                        case,
                        events,
                        |prefix| {
                            let r = replay_runtime(&rt, prefix);
                            let n = Naive::from_events(prefix);
                            sorted_live(&r, mid, vt) != product_live(&n, &r, mid, vt)
                        },
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

/// Characterization (M026 S02): Naive::as_of is defined as live_at(MAX, vt) — mirror, not a fold kill.
#[test]
fn model_as_of_alias_is_live_at_max() {
    let mut rt = Runtime::default();
    ingest_mix(&mut rt, 42, MixStyle::BehaviorFree);
    let n = Naive::from_events(rt.log().as_slice());
    assert_eq!(n.as_of(2017), n.live_at(u64::MAX, 2017));
}

/// Forces CorrectInterval residuals then Retract of the CI id so a first-live
/// naive Retract would leave prefix/suffix rows and disagree with GraphFold.
#[test]
fn naive_retract_clears_all_correct_interval_residuals() {
    let mut rt = Runtime::default();
    let s = rt.intern("Alice");
    let r = rt.intern("relatedTo");
    let o = rt.intern("Bob");
    let o2 = rt.intern("Carol");
    let mint = rt
        .emit(Op::Assert {
            subject: s,
            relation: r,
            object: o,
            valid_from: 2010,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap()
        .receipt
        .event_ids[0];
    let ci = rt
        .emit(Op::CorrectInterval {
            event_id: mint,
            object: o2,
            patch_from: 2015,
            patch_to: Some(2020),
        })
        .unwrap()
        .receipt
        .event_ids[0];
    rt.emit(Op::Retract { event_id: ci }).unwrap();
    let naive = Naive::from_events(rt.log().as_slice());
    let live_ci: Vec<_> = naive
        .rows
        .iter()
        .filter(|row| row.event_id == ci && row.invalidated_at.is_none())
        .collect();
    assert!(
        live_ci.is_empty(),
        "naive Retract must clear every CI residual (first-live alone leaves prefix/suffix)"
    );
    assert_eq!(
        product_live(&naive, &rt, u64::MAX, 2012),
        sorted_as_of(&rt, 2012)
    );
    assert_eq!(
        product_live(&naive, &rt, u64::MAX, 2017),
        sorted_as_of(&rt, 2017)
    );
}

/// Pins that model/fold triples include the object, not only subject.
#[test]
fn bitemporal_model_compares_object_not_only_subject() {
    assert_seed_budget();
    let mut rt = Runtime::default();
    let alice = rt.intern("Alice");
    let bob = rt.intern("Bob");
    let rel = rt.intern("relatedTo");
    rt.emit(Op::Assert {
        subject: alice,
        relation: rel,
        object: bob,
        valid_from: 2010,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    let naive = Naive::from_events(rt.log().as_slice());
    let fold = sorted_as_of(&rt, 2017);
    let model = product_live(&naive, &rt, u64::MAX, 2017);
    assert_eq!(fold, model, "model and fold must agree on (s,r,o)");
    assert!(
        fold.contains(&(alice, rel, bob)),
        "live triple must keep Assert object, not only subject"
    );
    assert!(
        !fold.contains(&(bob, rel, alice)),
        "Assert subject/object swap must not be invisible to the model/fold diff"
    );
}
