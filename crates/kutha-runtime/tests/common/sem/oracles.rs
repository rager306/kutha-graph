//! Stamp, CSR, temporal-view, and hydration oracles (CE #12).
#![allow(dead_code)]

use super::generator::{MixMeta, brief_ops};
use super::persistence::{
    PersistLayout, expected_open_event_ids, persist_dir, rm_dir, try_persist_with_layout,
};
use super::shrink::{candidate_deps_hold, fail_seed, fail_shrunk, replay_runtime};
use kutha_common::{Event, EventId, Op, TermId, ValidTime, WallTime};
use kutha_runtime::{GraphFold, Runtime, store};

pub const EDGE_VT: u64 = 2009;
const AFTER_VT: u64 = 2025;
pub type Stamp = (TermId, TermId, TermId, u64, Option<u64>, u64, Option<u64>);

pub fn is_product_rel(rt: &Runtime, rel: TermId) -> bool {
    !rt.dictionary()
        .lookup(rel)
        .is_some_and(|n| n.starts_with("kutha:"))
}

pub fn fold_stamps(fold: &GraphFold, rt: &Runtime) -> Vec<Stamp> {
    let mut rows: Vec<Stamp> = fold
        .facts()
        .iter()
        .filter(|f| is_product_rel(rt, f.relation))
        .map(|f| {
            (
                f.subject,
                f.relation,
                f.object(),
                f.valid_from,
                f.valid_to,
                f.ingested_at,
                f.invalidated_at,
            )
        })
        .collect();
    rows.sort_unstable();
    rows
}

pub fn runtime_stamps(rt: &Runtime) -> Vec<Stamp> {
    fold_stamps(rt.fold(), rt)
}

pub fn sorted_live(rt: &Runtime, tt: u64, vt: u64) -> Vec<(TermId, TermId, TermId)> {
    let mut t = rt.live_at(tt, vt);
    t.sort_unstable();
    t
}

pub fn sorted_as_of(rt: &Runtime, vt: u64) -> Vec<(TermId, TermId, TermId)> {
    let mut t = rt.as_of(ValidTime::from(vt));
    t.sort_unstable();
    t
}

/// Four cuts: explicit 2009 edge, two strictly inside mix `valid_from`, one after.
pub fn vt_cuts_for(min_vf: u64, max_vf: u64) -> [u64; 4] {
    let lo = if max_vf > min_vf { min_vf + 1 } else { min_vf };
    let hi = if max_vf > min_vf + 1 {
        max_vf - 1
    } else {
        max_vf
    };
    [EDGE_VT, lo, hi, AFTER_VT]
}

pub fn vt_cut_inside(cut: u64, min_vf: u64, max_vf: u64) -> bool {
    cut > min_vf && cut < max_vf
}
pub fn typed_csr_triples(rt: &Runtime, tt: u64, vt: u64) -> Vec<(TermId, TermId, TermId)> {
    let typed = rt.typed_csr_lease_at(tt, vt);
    let mut from_typed = Vec::new();
    for v in 0..rt.dictionary().len() as u32 {
        for e in typed.edges_out(v) {
            from_typed.push((v, e.relation, e.object));
        }
    }
    from_typed.sort_unstable();
    from_typed
}

pub fn csr_disagrees(rt: &Runtime, events: &[Event], tt: u64, vt: u64) -> bool {
    let r = replay_runtime(rt, events);
    typed_csr_triples(&r, tt, vt) != sorted_live(&r, tt, vt)
}

pub fn assert_csr_matches_fold(rt: &Runtime, tt: u64, vt: u64, seed: u32, case: usize) {
    let want = sorted_live(rt, tt, vt);
    let from_typed = typed_csr_triples(rt, tt, vt);
    if from_typed != want {
        fail_shrunk(
            seed,
            case,
            rt.log().as_slice(),
            |prefix| csr_disagrees(rt, prefix, tt, vt),
            "typed CSR edges must equal live_at triples",
        );
    }
    let untyped = rt.csr_lease_at(tt, vt);
    let mut buckets: Vec<Vec<TermId>> = vec![Vec::new(); rt.dictionary().len()];
    for (s, _, o) in &want {
        let i = *s as usize;
        if i < buckets.len() {
            buckets[i].push(*o);
        }
    }
    for row in &mut buckets {
        row.sort_unstable();
        row.dedup();
    }
    for v in 0..rt.dictionary().len() as u32 {
        if untyped.neighbors(v) != buckets[v as usize].as_slice() {
            fail_shrunk(
                seed,
                case,
                rt.log().as_slice(),
                |prefix| csr_disagrees(rt, prefix, tt, vt),
                "untyped CSR neighbors must equal unique live objects",
            );
        }
    }
}

pub fn persist_stamps_disagree(
    rt: &Runtime,
    events: &[Event],
    layout: PersistLayout,
    seed: u32,
) -> bool {
    if !candidate_deps_hold(events) {
        return false;
    }
    let live = match Runtime::from_dict_and_events(
        rt.dictionary().strings().to_vec(),
        events.to_vec(),
        rt.max_cascade,
    ) {
        Ok(r) => r,
        Err(_) => return false,
    };
    let dir = persist_dir(&format!("shrink-{seed}"));
    if !try_persist_with_layout(&live, &dir, layout) {
        rm_dir(&dir);
        return false;
    }
    let bad = match store::open(&dir) {
        Ok(opened) => runtime_stamps(&live) != runtime_stamps(&opened),
        Err(_) => true,
    };
    rm_dir(&dir);
    bad
}

/// Opened EventId order disagrees with non-Define live ids (persist+open path).
pub fn persist_ids_disagree(
    rt: &Runtime,
    events: &[Event],
    layout: PersistLayout,
    seed: u32,
) -> bool {
    if !candidate_deps_hold(events) {
        return false;
    }
    let live = match Runtime::from_dict_and_events(
        rt.dictionary().strings().to_vec(),
        events.to_vec(),
        rt.max_cascade,
    ) {
        Ok(r) => r,
        Err(_) => return false,
    };
    let dir = persist_dir(&format!("ids-{seed}"));
    if !try_persist_with_layout(&live, &dir, layout) {
        rm_dir(&dir);
        return false;
    }
    let want = expected_open_event_ids(&live);
    let bad = match store::open(&dir) {
        Ok(opened) => {
            let got: Vec<EventId> = opened.log().iter().map(|e| e.id).collect();
            got != want
        }
        Err(_) => true,
    };
    rm_dir(&dir);
    bad
}

pub fn assert_same_stamps_shrunk(
    a: &Runtime,
    b: &Runtime,
    seed: u32,
    case: usize,
    layout: PersistLayout,
    msg: &str,
) {
    if runtime_stamps(a) != runtime_stamps(b) {
        fail_shrunk(
            seed,
            case,
            a.log().as_slice(),
            |prefix| persist_stamps_disagree(a, prefix, layout, seed),
            msg,
        );
    }
}

pub fn replay_check_or_shrink(rt: &Runtime, seed: u32, case: usize) {
    if let Err(e) = rt.replay_check() {
        fail_shrunk(
            seed,
            case,
            rt.log().as_slice(),
            |prefix| replay_runtime(rt, prefix).replay_check().is_err(),
            &format!("replay_check: {e}"),
        );
    }
}

pub fn expected_next_tt(events: &[Event]) -> u64 {
    events
        .iter()
        .map(|e| e.ingested_at.saturating_add(1))
        .max()
        .unwrap_or(0)
}

pub fn qo_count(events: &[Event]) -> usize {
    events
        .iter()
        .filter(|e| matches!(e.op, Op::QuantumOutcome { .. }))
        .count()
}

/// Every log event is load-bearing: count, committed prefix, dictionary, outcomes, next_tt.
#[derive(Debug, PartialEq, Eq)]
pub struct LoadBearing {
    pub n_events: usize,
    pub n_committed: usize,
    pub dict: Vec<String>,
    pub n_outcomes: usize,
    pub graph_len: usize,
    pub next_tt: u64,
    pub ops: String,
}

pub fn load_bearing(rt: &Runtime) -> LoadBearing {
    let events = rt.log().as_slice();
    LoadBearing {
        n_events: events.len(),
        n_committed: store::committed_prefix(events.to_vec()).len(),
        dict: rt.dictionary().strings().to_vec(),
        n_outcomes: rt.outcome_records().len(),
        graph_len: rt.graph_len(),
        next_tt: rt.snapshot().next_tt,
        ops: brief_ops(events),
    }
}

pub fn hydrate_skips_event(rt: &Runtime, events: &[Event]) -> bool {
    let r = replay_runtime(rt, events);
    r.log().as_slice().len() != events.len()
        || r.outcome_records().len() != qo_count(events)
        || r.snapshot().next_tt != expected_next_tt(events)
        || store::committed_prefix(r.log().as_slice().to_vec()).len()
            != store::committed_prefix(events.to_vec()).len()
}

pub fn assert_load_bearing(live: &Runtime, replayed: &Runtime, seed: u32, case: usize) {
    let events = live.log().as_slice();
    if replayed.log().as_slice().len() != events.len()
        || load_bearing(live) != load_bearing(replayed)
        || replayed.snapshot().next_tt != expected_next_tt(events)
        || replayed.outcome_records().len() != qo_count(events)
    {
        fail_shrunk(
            seed,
            case,
            events,
            |prefix| hydrate_skips_event(live, prefix),
            "hydrate must keep event count, committed prefix, dictionary, outcomes, next_tt",
        );
    }
}

pub fn assert_views_stable(rt: &Runtime, meta: &MixMeta, seed: u32, case: usize) {
    let cuts = vt_cuts_for(meta.min_vf, meta.max_vf);
    for (i, vt) in cuts.iter().enumerate() {
        let as_of = sorted_as_of(rt, *vt);
        let live_max = sorted_live(rt, u64::MAX, *vt);
        if as_of != live_max {
            fail_shrunk(
                seed,
                case + i,
                rt.log().as_slice(),
                |prefix| {
                    let r = replay_runtime(rt, prefix);
                    sorted_as_of(&r, *vt) != sorted_live(&r, u64::MAX, *vt)
                },
                "as_of(vt) must equal live_at(MAX, vt)",
            );
        }
        assert_csr_matches_fold(rt, u64::MAX, *vt, seed, case + 10 + i);
    }
    if let Some(wall) = meta.last_wall {
        let Some(mut known) = rt.as_known_on(WallTime::from(wall)) else {
            fail_seed(
                seed,
                case + 30,
                rt.log().as_slice(),
                "as_known_on must find the mix TtAnchor",
            );
        };
        known.sort_unstable();
        let mut scan: Vec<(TermId, TermId, TermId)> = rt
            .fold()
            .facts()
            .iter()
            .filter(|f| {
                is_product_rel(rt, f.relation)
                    && f.ingested_at <= wall_tt(rt, wall)
                    && f.invalidated_at.is_none_or(|inv| inv > wall_tt(rt, wall))
            })
            .map(|f| (f.subject, f.relation, f.object()))
            .collect();
        scan.sort_unstable();
        if known != scan {
            fail_seed(
                seed,
                case + 31,
                rt.log().as_slice(),
                "as_known_on must match TT-only product scan",
            );
        }
    }
    let tts: Vec<u64> = {
        let mut v: Vec<u64> = rt
            .fold()
            .facts()
            .iter()
            .filter(|f| is_product_rel(rt, f.relation))
            .map(|f| f.ingested_at)
            .collect();
        v.sort_unstable();
        v.dedup();
        v
    };
    if let Some(&mid) = tts.get(tts.len() / 2) {
        let inside = vt_cuts_for(meta.min_vf, meta.max_vf)[1];
        assert_csr_matches_fold(rt, mid, inside, seed, case + 40);
    }
}

fn wall_tt(rt: &Runtime, wall: u64) -> u64 {
    rt.log()
        .iter()
        .filter_map(|e| match &e.op {
            Op::TtAnchor { wall: w } if *w <= wall => Some((*w, e.ingested_at)),
            _ => None,
        })
        .max_by_key(|(w, _)| *w)
        .map(|(_, tt)| tt)
        .expect("TtAnchor")
}

pub fn assert_same_stamps(a: &Runtime, b: &Runtime, seed: u32, case: usize, msg: &str) {
    if runtime_stamps(a) != runtime_stamps(b) {
        fail_seed(seed, case, a.log().as_slice(), msg);
    }
}
