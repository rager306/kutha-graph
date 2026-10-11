//! Candidate reduction, corpus, and failure reporting (CE #12 / #20).
#![allow(dead_code)]

use super::generator::brief_ops;
use super::oracles::{is_product_rel, runtime_stamps};
use kutha_common::{Event, EventId, Op, TermId, sha256_hex};
use kutha_runtime::Runtime;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::{env, fs};

pub fn shrink_events<F>(events: &[Event], mut pred_fails: F) -> Vec<Event>
where
    F: FnMut(&[Event]) -> bool,
{
    let mut best = events.to_vec();
    for n in 1..=events.len() {
        if pred_fails(&events[..n]) {
            best = events[..n].to_vec();
            break;
        }
    }
    let mut i = 0;
    while i < best.len() {
        let mut cand = best.clone();
        cand.remove(i);
        if cand.is_empty() {
            i += 1;
            continue;
        }
        if pred_fails(&cand) {
            best = cand;
        } else {
            i += 1;
        }
    }
    best
}

/// Referential deps for shrink candidates (test-only). Hydrate `.ok()` is not enough:
/// `from_dict_and_events` still applies orphan Retract/Correct without Err.
pub fn candidate_deps_hold(prefix: &[Event]) -> bool {
    use std::collections::BTreeSet;
    let ids: BTreeSet<EventId> = prefix.iter().map(|e| e.id).collect();
    for e in prefix {
        match &e.op {
            Op::Retract { event_id }
            | Op::Correct { event_id, .. }
            | Op::CorrectInterval { event_id, .. } => {
                if !ids.contains(event_id) {
                    return false;
                }
            }
            _ => {}
        }
    }
    true
}

/// Chunk-ddmin after `shrink_events`. Invalid candidates (deps) are skipped — not
/// treated as oracle success. Oracle panics propagate (no catch_unwind).
pub fn shrink_events_ddmin<F>(events: &[Event], mut pred_fails: F) -> Vec<Event>
where
    F: FnMut(&[Event]) -> bool,
{
    let mut gated = |cand: &[Event]| -> bool { candidate_deps_hold(cand) && pred_fails(cand) };
    let mut best = shrink_events(events, &mut gated);
    let mut gran = 2usize;
    while gran <= best.len() {
        let chunk = best.len().div_ceil(gran);
        let mut reduced = false;
        let mut start = 0;
        while start < best.len() {
            let end = (start + chunk).min(best.len());
            let mut cand = best.clone();
            cand.drain(start..end);
            if cand.is_empty() {
                start = end;
                continue;
            }
            if !candidate_deps_hold(&cand) {
                start = end;
                continue;
            }
            if pred_fails(&cand) {
                best = cand;
                reduced = true;
                break;
            }
            start = end;
        }
        if reduced {
            gran = 2;
        } else {
            gran = gran.saturating_mul(2);
        }
    }
    best
}

pub fn replay_runtime(rt: &Runtime, events: &[Event]) -> Runtime {
    Runtime::from_dict_and_events(
        rt.dictionary().strings().to_vec(),
        events.to_vec(),
        rt.max_cascade,
    )
    .expect("from_dict_and_events")
}

/// Product Assert/Behavior plus Retract/Correct/CorrectInterval (fold-affecting).
pub fn fold_affecting_user<'a>(events: &'a [Event], rt: &Runtime) -> Vec<(usize, &'a Event)> {
    events
        .iter()
        .enumerate()
        .filter(|(_, e)| match &e.op {
            Op::Assert { relation, .. } | Op::Behavior { relation, .. } => {
                is_product_rel(rt, *relation)
            }
            Op::Retract { .. } | Op::Correct { .. } | Op::CorrectInterval { .. } => true,
            _ => false,
        })
        .collect()
}

/// (subject, relation) keys an event writes or patches on the live fold.
pub fn event_touch_sr(rt: &Runtime, e: &Event) -> BTreeSet<(TermId, TermId)> {
    match &e.op {
        Op::Assert {
            subject, relation, ..
        }
        | Op::Behavior {
            subject, relation, ..
        } => BTreeSet::from([(*subject, *relation)]),
        Op::Retract { event_id }
        | Op::Correct { event_id, .. }
        | Op::CorrectInterval { event_id, .. } => rt
            .fold()
            .facts()
            .iter()
            .filter(|f| f.event_id == *event_id)
            .map(|f| (f.subject, f.relation))
            .collect(),
        _ => BTreeSet::new(),
    }
}

/// True when some fold-affecting swap changes `runtime_stamps`.
pub fn swap_changes_fold(rt: &Runtime, events: &[Event]) -> bool {
    let orig = runtime_stamps(rt);
    let users = fold_affecting_user(events, rt);
    for &(i, _) in &users {
        for &(j, _) in &users {
            if i >= j {
                continue;
            }
            let mut swapped = events.to_vec();
            swapped.swap(i, j);
            if orig != runtime_stamps(&replay_runtime(rt, &swapped)) {
                return true;
            }
        }
    }
    false
}

/// Tamper generator: if every in-log swap commutes, retract one live product Assert
/// so Assert↔Retract is a load-bearing pair (KUTHA_SEEDS=256 seed=79).
pub fn ensure_tamper_swap_bites(rt: &mut Runtime) {
    let events = rt.log().as_slice().to_vec();
    if swap_changes_fold(rt, &events) {
        return;
    }
    let ids: Vec<EventId> = rt
        .log()
        .iter()
        .filter_map(|e| match &e.op {
            Op::Assert { relation, .. } if is_product_rel(rt, *relation) => Some(e.id),
            _ => None,
        })
        .collect();
    for event_id in ids {
        if rt.emit(Op::Retract { event_id }).is_ok() {
            let events = rt.log().as_slice().to_vec();
            assert!(
                swap_changes_fold(rt, &events),
                "Retract of a live Assert must yield a non-commuting swap pair"
            );
            return;
        }
    }
    panic!("tamper mix could not mint a dependent Retract for a non-commuting swap");
}

pub fn assert_swap_not_silent(rt: &Runtime, events: &[Event], seed: u32, case: usize) {
    let orig = runtime_stamps(rt);
    let users = fold_affecting_user(events, rt);
    assert!(
        users.len() >= 2,
        "seed={seed}: need ≥2 fold-affecting product ops to tamper"
    );
    let mut changing = None;
    for &(i, _) in &users {
        for &(j, _) in &users {
            if i >= j {
                continue;
            }
            let mut swapped = events.to_vec();
            swapped.swap(i, j);
            if orig != runtime_stamps(&replay_runtime(rt, &swapped)) {
                changing = Some((i, j));
                break;
            }
        }
        if changing.is_some() {
            break;
        }
    }
    if let Some((i, j)) = changing {
        let mut swapped = events.to_vec();
        swapped.swap(i, j);
        assert_ne!(
            orig,
            runtime_stamps(&replay_runtime(rt, &swapped)),
            "swap: fold must differ (not silently equal) seed={seed} case={case} i={i} j={j}"
        );
        return;
    }
    for &(i, ei) in &users {
        for &(j, ej) in &users {
            if i >= j {
                continue;
            }
            let a = event_touch_sr(rt, ei);
            let b = event_touch_sr(rt, ej);
            assert!(
                !a.is_empty() && !b.is_empty() && a.is_disjoint(&b),
                "swap: equal-fold pair must commute on disjoint subjects/relations \
                 seed={seed} case={case} i={i} j={j} a={a:?} b={b:?}"
            );
        }
    }
}

/// Content-addressed path under `root`: `seed-{seed}-{sha12}.json`.
pub fn save_shrunk_to(root: &Path, seed: u32, events: &[Event]) -> PathBuf {
    let bytes = serde_json::to_vec(events).expect("serialize shrunk events");
    let digest = sha256_hex(&bytes);
    let _ = fs::create_dir_all(root);
    let path = root.join(format!("seed-{seed}-{}.json", &digest[..12]));
    fs::write(&path, &bytes).expect("write shrunk corpus");
    path
}

pub fn load_shrunk(path: &Path) -> Result<Vec<Event>, String> {
    let bytes = fs::read(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    serde_json::from_slice(&bytes).map_err(|e| format!("decode {}: {e}", path.display()))
}

pub fn maybe_save_shrunk(seed: u32, events: &[Event]) {
    let Ok(root) = env::var("KUTHA_SAVE_SHRUNK") else {
        return;
    };
    let _ = save_shrunk_to(Path::new(&root), seed, events);
}

pub fn fail_seed(seed: u32, case: usize, events: &[Event], msg: &str) -> ! {
    maybe_save_shrunk(seed, events);
    eprintln!(
        "SEM fail seed={seed} case={case} n={} ops={}",
        events.len(),
        brief_ops(events)
    );
    eprintln!("shrunk ops={}", brief_ops(events));
    panic!("{msg}");
}

pub fn fail_shrunk<F>(seed: u32, case: usize, events: &[Event], pred_fails: F, msg: &str) -> !
where
    F: FnMut(&[Event]) -> bool,
{
    let shrunk = shrink_events_ddmin(events, pred_fails);
    fail_seed(seed, case, &shrunk, msg);
}
