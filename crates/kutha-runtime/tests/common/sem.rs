//! M019 S07 shared LCG, stamps, mix ingest, persist layouts.
//! Default ≥64 seeds; `KUTHA_SEEDS=<n>` may raise (never lowers).
//! Persist/tamper/export use `persist_seed_count()` = max(12, n/5), so the env
//! raises those families too (default 64 → 12; 256 → 51). Two persist passes per
//! seed so any window of 16 persist indices covers legacy, segmented-multi-segment
//! (rollover), migrated, and export-imported; pass 1 puts WithHorn through
//! segmented/migrated. No wall-clock oracle; persist scratch names are pid + atomic.
#![allow(dead_code)]

use kutha_common::{Event, EventId, Op, TermId, ValidTime, WallTime};
use kutha_runtime::{FoldIndexMode, GraphFold, Runtime, RuntimeError, store};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::{env, fs};

pub const DEFAULT_SEEDS: usize = 64;
pub const N_OPS: u32 = 8;
pub const N_PERSIST_OPS: u32 = 4;
const PATCH_FROM: u64 = 2015;
const PATCH_TO: u64 = 2020;
/// Explicit edge cut: always before mix `valid_from` (≥2010). Not an inside cut.
pub const EDGE_VT: u64 = 2009;
const AFTER_VT: u64 = 2025;

/// Fixed table of 64 seeds (m017 LCG cousin). Extra seeds are LCG-derived.
pub const BASE_SEEDS: [u32; DEFAULT_SEEDS] = [
    1,
    7,
    13,
    42,
    99,
    12345,
    0xA5A5_A5A5,
    0x00C0_FFEE,
    3,
    11,
    17,
    19,
    23,
    29,
    31,
    37,
    41,
    43,
    47,
    53,
    59,
    61,
    67,
    71,
    73,
    79,
    83,
    89,
    97,
    101,
    103,
    107,
    109,
    113,
    127,
    131,
    137,
    139,
    149,
    151,
    157,
    163,
    167,
    173,
    179,
    181,
    191,
    193,
    197,
    199,
    211,
    223,
    227,
    229,
    233,
    239,
    241,
    251,
    257,
    263,
    269,
    271,
    277,
    281,
];

static DIR_SEQ: AtomicU64 = AtomicU64::new(1);

pub struct Lcg(pub u32);

impl Lcg {
    pub fn from_seed(seed: u32) -> Self {
        Self(seed)
    }

    pub fn next_u32(&mut self) -> u32 {
        self.0 = self.0.wrapping_mul(1664525).wrapping_add(1013904223);
        self.0 >> 16
    }

    pub fn draw100(&mut self) -> u32 {
        self.next_u32() % 100
    }

    pub fn pick(&mut self, n: usize) -> usize {
        (self.next_u32() as usize) % n.max(1)
    }
}

pub fn seed_count() -> usize {
    env::var("KUTHA_SEEDS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_SEEDS)
        .max(DEFAULT_SEEDS)
}

/// Env seed floor (`KUTHA_SEEDS`), not domain semantics (M026 S02 characterization).
/// Governor `rust_test_asserts`: named tests must contain `assert*` in-body.
pub fn assert_seed_budget() {
    let n = seed_count();
    assert!(
        n >= DEFAULT_SEEDS,
        "KUTHA_SEEDS must not lower the {DEFAULT_SEEDS}-seed floor"
    );
    assert_eq!(seeds().len(), n);
    assert!(
        persist_seed_count() >= 12,
        "persist/tamper floor is 12 seeds"
    );
}

pub fn seeds() -> Vec<u32> {
    let n = seed_count();
    let mut out = BASE_SEEDS.to_vec();
    let mut lcg = Lcg::from_seed(0xC0FF_EE11);
    while out.len() < n {
        out.push(lcg.next_u32().max(1));
    }
    out.truncate(n);
    out
}

/// Persist/tamper/export: `max(12, seed_count()/5)`. Default 64 → 12; env raises.
pub fn persist_seed_count() -> usize {
    (seed_count() / 5).max(12)
}

pub struct LimitGuard;

impl Drop for LimitGuard {
    fn drop(&mut self) {
        store::reset_test_segment_limits();
    }
}

pub fn tiny_segment_limits() -> LimitGuard {
    store::set_test_segment_limits(Some(4096), Some(3));
    LimitGuard
}

pub fn persist_dir(label: &str) -> PathBuf {
    let n = DIR_SEQ.fetch_add(1, Ordering::Relaxed);
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("tmp")
        .join("p62-sem")
        .join(format!("{label}-{n}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

pub fn rm_dir(dir: &Path) {
    let _ = fs::remove_dir_all(dir);
}

/// Product fact stamps: (s,r,o, valid_from, valid_to, ingested_at, invalidated_at). Never EventId.
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MixStyle {
    /// `relatedTo` only — no inverse_knows cascade.
    BehaviorFree,
    /// Mix `relatedTo` and `knows` (genesis inverse_knows may fire).
    WithHorn,
    /// M026 S01: reaches Retract-before-Assert, overlap, keyed retry, derived Correct/Retract.
    /// Does not alter BehaviorFree/WithHorn LCG schedules used by DET pins.
    Coverage,
}

pub fn persist_style(i: usize) -> MixStyle {
    if i.is_multiple_of(2) {
        MixStyle::BehaviorFree
    } else {
        MixStyle::WithHorn
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PersistLayout {
    Legacy,
    Segmented,
    Rollover,
    Migrated,
}

pub fn persist_layout(i: usize) -> PersistLayout {
    match i % 4 {
        0 => PersistLayout::Legacy,
        1 => PersistLayout::Segmented,
        2 => PersistLayout::Rollover,
        _ => PersistLayout::Migrated,
    }
}

/// One persist+open (and optional export_jsonl) case.
#[derive(Clone, Copy, Debug)]
pub struct PersistCase {
    pub style: MixStyle,
    pub layout: PersistLayout,
    /// Run export_jsonl + import + log-order pin (kills middle-event swap).
    pub export: bool,
}

fn layout_exports(layout: PersistLayout) -> bool {
    matches!(layout, PersistLayout::Segmented | PersistLayout::Rollover)
}

/// Two deterministic passes per persist index.
/// Pass 0: `persist_layout(i)` + `persist_style(i)`.
/// Pass 1: `persist_layout(i+2)` and WithHorn on segmented/migrated/rollover.
/// Together, any 16 consecutive indices cover all four layouts and export-imported.
pub fn persist_schedule(i: usize) -> [PersistCase; 2] {
    let lay0 = persist_layout(i);
    let lay1 = persist_layout(i + 2);
    let style1 = match lay1 {
        PersistLayout::Segmented | PersistLayout::Migrated | PersistLayout::Rollover => {
            MixStyle::WithHorn
        }
        PersistLayout::Legacy => persist_style(i),
    };
    [
        PersistCase {
            style: persist_style(i),
            layout: lay0,
            export: layout_exports(lay0),
        },
        PersistCase {
            style: style1,
            layout: lay1,
            export: layout_exports(lay1),
        },
    ]
}

pub struct MixMeta {
    pub last_wall: Option<u64>,
    pub saw_assert: bool,
    pub min_vf: u64,
    pub max_vf: u64,
    /// Coverage-style only: Retract attempted while pool empty (fail-closed).
    pub saw_retract_before_assert: bool,
    /// Coverage-style only: two Asserts same (s,r), different objects.
    pub saw_overlapping_assert: bool,
    /// Coverage-style only: keyed Assert short-circuit retry.
    pub saw_keyed_retry: bool,
    /// Coverage-style only: Correct or Retract of a Behavior-derived id.
    pub saw_derived_correct_or_retract: bool,
}

fn emit_assert(
    rt: &mut Runtime,
    subject: TermId,
    relation: TermId,
    object: TermId,
    valid_from: u64,
    valid_to: Option<u64>,
) -> EventId {
    emit_assert_keyed(rt, subject, relation, object, valid_from, valid_to, None)
}

fn emit_assert_keyed(
    rt: &mut Runtime,
    subject: TermId,
    relation: TermId,
    object: TermId,
    valid_from: u64,
    valid_to: Option<u64>,
    delivery_key: Option<String>,
) -> EventId {
    let out = rt
        .emit(Op::Assert {
            subject,
            relation,
            object,
            valid_from,
            valid_to,
            claim: None,
            delivery_key,
            polarity: None,
        })
        .unwrap_or_else(|e| panic!("Assert failed: {e}"));
    out.receipt.event_ids[0]
}

/// Explicit Coverage draws (M026 S01). Returns true when the step handled `i`.
fn coverage_step(
    rt: &mut Runtime,
    meta: &mut MixMeta,
    pool: &mut Vec<EventId>,
    subjects: &[TermId],
    related: TermId,
    knows: TermId,
    obj_a: TermId,
    obj_b: TermId,
    i: u32,
) -> bool {
    match i {
        0 => {
            // Retract before any Assert: empty pool → fail-closed UnknownFact.
            let err = rt
                .emit(Op::Retract {
                    event_id: EventId::nil(),
                })
                .expect_err("Retract before Assert must fail closed");
            assert!(
                matches!(err, RuntimeError::UnknownFact { .. }),
                "expected UnknownFact, got {err:?}"
            );
            meta.saw_retract_before_assert = true;
            true
        }
        1 => {
            // Overlapping Asserts: same (s,r), different objects, overlapping VT.
            let subj = subjects[0];
            let a = emit_assert(rt, subj, related, obj_a, 2010, Some(2022));
            let b = emit_assert(rt, subj, related, obj_b, 2015, None);
            pool.push(a);
            pool.push(b);
            meta.saw_assert = true;
            meta.saw_overlapping_assert = true;
            note_vf(meta, 2010);
            note_vf(meta, 2015);
            true
        }
        2 => {
            // Keyed retry: second identical delivery_key short-circuits (no second mint).
            let subj = subjects[1];
            let key = "m026-coverage-key".to_string();
            let first = emit_assert_keyed(
                rt,
                subj,
                related,
                obj_a,
                2012,
                None,
                Some(key.clone()),
            );
            let before = rt.log().len();
            let second = emit_assert_keyed(rt, subj, related, obj_a, 2012, None, Some(key));
            assert_eq!(
                first, second,
                "keyed retry must return the same EventId"
            );
            assert_eq!(
                before,
                rt.log().len(),
                "keyed retry must not append a second Assert"
            );
            pool.push(first);
            meta.saw_assert = true;
            meta.saw_keyed_retry = true;
            note_vf(meta, 2012);
            true
        }
        3 => {
            // Assert knows → inverse Behavior; Correct then Retract the derived id.
            let subj = subjects[2];
            let prem = emit_assert(rt, subj, knows, obj_a, 2011, None);
            pool.push(prem);
            meta.saw_assert = true;
            note_vf(meta, 2011);
            let derived = rt
                .log()
                .iter()
                .filter(|e| matches!(&e.op, Op::Behavior { .. }))
                .map(|e| e.id)
                .last();
            let Some(derived_id) = derived else {
                // Genesis may not fire inverse for this subject; skip softly.
                return true;
            };
            match rt.emit(Op::Correct {
                event_id: derived_id,
                object: obj_b,
                valid_from: 2011,
                valid_to: None,
            }) {
                Ok(out) => {
                    if let Some(new_id) = out.receipt.event_ids.first() {
                        pool.push(*new_id);
                    }
                    meta.saw_derived_correct_or_retract = true;
                }
                Err(RuntimeError::UnknownFact { .. }) => {
                    // Fall through to Retract of the derived id.
                    if rt.emit(Op::Retract { event_id: derived_id }).is_ok() {
                        meta.saw_derived_correct_or_retract = true;
                    }
                }
                Err(e) => panic!("Correct derived seed coverage: {e}"),
            }
            if !meta.saw_derived_correct_or_retract {
                if rt.emit(Op::Retract { event_id: derived_id }).is_ok() {
                    meta.saw_derived_correct_or_retract = true;
                }
            }
            true
        }
        _ => false,
    }
}

fn note_vf(meta: &mut MixMeta, vf: u64) {
    meta.min_vf = meta.min_vf.min(vf);
    meta.max_vf = meta.max_vf.max(vf);
}

/// Seeded mix: Assert / Retract / Correct / CorrectInterval / TtAnchor. No wall clock.
pub fn ingest_mix_n(rt: &mut Runtime, seed: u32, style: MixStyle, n_ops: u32) -> MixMeta {
    let mut lcg = Lcg::from_seed(seed);
    let related = rt.intern("relatedTo");
    let knows = rt.intern("knows");
    let obj_a = rt.intern("P");
    let obj_b = rt.intern("P-prime");
    let subjects: Vec<TermId> = (0..4).map(|i| rt.intern(&format!("s{i}"))).collect();
    let mut pool: Vec<EventId> = Vec::new();
    let mut meta = MixMeta {
        last_wall: Some(2014),
        saw_assert: false,
        min_vf: u64::MAX,
        max_vf: 0,
        saw_retract_before_assert: false,
        saw_overlapping_assert: false,
        saw_keyed_retry: false,
        saw_derived_correct_or_retract: false,
    };
    let _ = rt.emit_tt_anchor(2014u64);

    // Empty-log path: n_ops == 0 leaves only the seed anchor (G-C11 sibling).
    if n_ops == 0 {
        if meta.min_vf == u64::MAX {
            meta.min_vf = 2010;
            meta.max_vf = 2014;
        }
        return meta;
    }

    for i in 0..n_ops {
        if style == MixStyle::Coverage {
            if coverage_step(rt, &mut meta, &mut pool, &subjects, related, knows, obj_a, obj_b, i)
            {
                continue;
            }
        }
        let draw = lcg.draw100();
        let subj = subjects[lcg.pick(subjects.len())];
        let rel = match style {
            MixStyle::BehaviorFree => related,
            MixStyle::WithHorn | MixStyle::Coverage if draw.is_multiple_of(3) => knows,
            MixStyle::WithHorn | MixStyle::Coverage => related,
        };
        let valid_from = 2010 + u64::from(i % 5);
        let valid_to = if draw.is_multiple_of(5) {
            Some(2019)
        } else {
            None
        };
        let want_assert = pool.is_empty() || draw < 40;
        if want_assert {
            let id = emit_assert(rt, subj, rel, obj_a, valid_from, valid_to);
            pool.push(id);
            meta.saw_assert = true;
            note_vf(&mut meta, valid_from);
            continue;
        }
        if draw < 55 {
            let event_id = pool.remove(0);
            match rt.emit(Op::Retract { event_id }) {
                Ok(_) => {}
                Err(RuntimeError::UnknownFact { .. }) => {
                    let id = emit_assert(rt, subj, rel, obj_a, valid_from, valid_to);
                    pool.push(id);
                    meta.saw_assert = true;
                    note_vf(&mut meta, valid_from);
                }
                Err(e) => panic!("Retract seed={seed} i={i}: {e}"),
            }
            continue;
        }
        if draw < 70 {
            let mut patched = false;
            for (idx, event_id) in pool.iter().copied().enumerate() {
                match rt.emit(Op::CorrectInterval {
                    event_id,
                    object: obj_b,
                    patch_from: PATCH_FROM,
                    patch_to: Some(PATCH_TO),
                }) {
                    Ok(out) => {
                        pool.remove(idx);
                        if let Some(new_id) = out.receipt.event_ids.first() {
                            pool.push(*new_id);
                        }
                        patched = true;
                        break;
                    }
                    Err(RuntimeError::IntervalPatchRejected { .. })
                    | Err(RuntimeError::UnknownFact { .. }) => {}
                    Err(e) => panic!("CorrectInterval seed={seed} i={i}: {e}"),
                }
            }
            if !patched {
                let id = emit_assert(rt, subj, rel, obj_a, valid_from, valid_to);
                pool.push(id);
                meta.saw_assert = true;
                note_vf(&mut meta, valid_from);
            }
            continue;
        }
        if draw < 85 {
            let mut done = false;
            for (idx, event_id) in pool.iter().copied().enumerate() {
                match rt.emit(Op::Correct {
                    event_id,
                    object: obj_b,
                    valid_from,
                    valid_to,
                }) {
                    Ok(out) => {
                        pool.remove(idx);
                        if let Some(new_id) = out.receipt.event_ids.first() {
                            pool.push(*new_id);
                        }
                        done = true;
                        note_vf(&mut meta, valid_from);
                        break;
                    }
                    Err(RuntimeError::UnknownFact { .. }) => {}
                    Err(e) => panic!("Correct seed={seed} i={i}: {e}"),
                }
            }
            if !done {
                let id = emit_assert(rt, subj, rel, obj_a, valid_from, valid_to);
                pool.push(id);
                meta.saw_assert = true;
                note_vf(&mut meta, valid_from);
            }
            continue;
        }
        let wall = 2015 + u64::from(i);
        match rt.emit_tt_anchor(wall) {
            Ok(_) => meta.last_wall = Some(wall),
            Err(RuntimeError::TtAnchorRejected { .. }) => {}
            Err(e) => panic!("TtAnchor seed={seed} i={i}: {e}"),
        }
    }
    if meta.min_vf == u64::MAX {
        meta.min_vf = 2010;
        meta.max_vf = 2014;
    }
    meta
}

pub fn ingest_mix(rt: &mut Runtime, seed: u32, style: MixStyle) -> MixMeta {
    ingest_mix_n(rt, seed, style, N_OPS)
}

pub fn seeded_runtime_n(
    seed: u32,
    style: MixStyle,
    mode: FoldIndexMode,
    n_ops: u32,
) -> (Runtime, MixMeta) {
    let mut rt = Runtime::with_index_mode(32, mode);
    let meta = ingest_mix_n(&mut rt, seed, style, n_ops);
    assert!(
        meta.saw_assert,
        "seed={seed} mix must include a product Assert"
    );
    (rt, meta)
}

pub fn seeded_runtime(seed: u32, style: MixStyle, mode: FoldIndexMode) -> (Runtime, MixMeta) {
    seeded_runtime_n(seed, style, mode, N_OPS)
}

pub fn seeded_persist_runtime(seed: u32, style: MixStyle) -> (Runtime, MixMeta) {
    seeded_runtime_n(seed, style, FoldIndexMode::Eager, N_PERSIST_OPS)
}

pub fn brief_ops(events: &[Event]) -> String {
    events
        .iter()
        .map(|e| match &e.op {
            Op::Assert { .. } => "Assert",
            Op::Retract { .. } => "Retract",
            Op::Correct { .. } => "Correct",
            Op::CorrectInterval { .. } => "CorrectInterval",
            Op::Behavior { name, .. } => name.as_str(),
            Op::TtAnchor { .. } => "TtAnchor",
            Op::Define { .. } => "Define",
            Op::QuantumOutcome { .. } => "QO",
            Op::StoreBlob { .. } => "Blob",
            Op::JustificationCite { .. } => "Cite",
        })
        .collect::<Vec<_>>()
        .join(",")
}

/// Smallest prefix that still fails `pred`, then drop-one-event passes.
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

pub fn fail_seed(seed: u32, case: usize, events: &[Event], msg: &str) -> ! {
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
    let shrunk = shrink_events(events, pred_fails);
    fail_seed(seed, case, &shrunk, msg);
}

pub fn persist_legacy(rt: &Runtime, dir: &Path) {
    store::ensure_legacy_dir(dir).unwrap();
    store::persist(rt, dir).unwrap();
}

pub fn persist_segmented(rt: &Runtime, dir: &Path) {
    store::persist(rt, dir).unwrap();
    assert!(
        dir.join(store::MANIFEST_REL).exists(),
        "empty dir must persist as segments"
    );
}

pub fn sealed_count(dir: &Path) -> usize {
    let v: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(dir.join(store::MANIFEST_REL)).unwrap()).unwrap();
    v["sealed"].as_array().map(|a| a.len()).unwrap_or(0)
}

pub fn persist_segmented_rollover(rt: &Runtime, dir: &Path) -> usize {
    let _g = tiny_segment_limits();
    persist_segmented(rt, dir);
    let n = sealed_count(dir);
    assert!(
        n >= 2,
        "rollover seam must force several sealed segments; sealed={n}"
    );
    n
}

pub fn persist_migrated(rt: &Runtime, dir: &Path) {
    persist_legacy(rt, dir);
    store::migrate(dir).unwrap();
    assert!(dir.join(store::MANIFEST_REL).exists());
}

pub fn persist_with_layout(rt: &Runtime, dir: &Path, layout: PersistLayout) -> usize {
    match layout {
        PersistLayout::Legacy => {
            persist_legacy(rt, dir);
            0
        }
        PersistLayout::Segmented => {
            persist_segmented(rt, dir);
            sealed_count(dir)
        }
        PersistLayout::Rollover => persist_segmented_rollover(rt, dir),
        PersistLayout::Migrated => {
            persist_migrated(rt, dir);
            sealed_count(dir)
        }
    }
}

pub fn jsonl_event_ids(path: &Path) -> Vec<EventId> {
    let text = fs::read_to_string(path).unwrap();
    text.lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let e: Event = serde_json::from_str(l).expect("events.jsonl line");
            e.id
        })
        .collect()
}

/// export_jsonl into a fresh jsonl-only dir (no manifest / WAL). Open uses jsonl as SoT.
/// Pins log event-id order so a middle-event swap in `export_jsonl` dies even when
/// the swapped prefix commutes under `runtime_stamps`.
pub fn open_export_imported(src: &Path, dest: &Path) -> Runtime {
    store::export_jsonl(src).unwrap();
    fs::create_dir_all(dest).unwrap();
    fs::copy(src.join("events.jsonl"), dest.join("events.jsonl")).unwrap();
    store::open(dest).unwrap()
}

pub fn expected_persist_event_ids(rt: &Runtime) -> Vec<EventId> {
    let mut out: Vec<EventId> = rt
        .dictionary()
        .strings()
        .iter()
        .map(|name| Event::stable_define(name.clone(), 0).id)
        .collect();
    for e in rt.log().iter() {
        if !matches!(e.op, Op::Define { .. }) {
            out.push(e.id);
        }
    }
    out
}

pub fn assert_export_preserves_event_order(src: &Path, rt: &Runtime, seed: u32, case: usize) {
    let ids = jsonl_event_ids(&src.join("events.jsonl"));
    let live = expected_persist_event_ids(rt);
    if ids.len() < 3 {
        fail_seed(
            seed,
            case,
            rt.log().as_slice(),
            "export_jsonl order pin needs ≥3 events so a middle swap is observable",
        );
    }
    if ids != live {
        fail_seed(
            seed,
            case,
            rt.log().as_slice(),
            "export_jsonl must preserve persist-encoded event-id order (middle-event swap dies)",
        );
    }
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
    let live = replay_runtime(rt, events);
    let dir = persist_dir(&format!("shrink-{seed}"));
    persist_with_layout(&live, &dir, layout);
    let bad = match store::open(&dir) {
        Ok(opened) => runtime_stamps(&live) != runtime_stamps(&opened),
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

fn is_seal_payload(payload: &[u8]) -> bool {
    serde_json::from_slice::<serde_json::Value>(payload)
        .ok()
        .and_then(|v| v.get("k").and_then(|k| k.as_str()).map(|s| s == "seal"))
        .unwrap_or(false)
}

pub fn event_frame_ranges(bytes: &[u8]) -> Vec<(usize, usize)> {
    let hdr = store::SEGMENT_HEADER_LEN as usize;
    assert!(bytes.len() >= hdr, "segment header");
    assert_eq!(&bytes[0..8], b"KUTHASEG");
    let mut off = hdr;
    let mut out = Vec::new();
    while off + 8 <= bytes.len() {
        let mut len_buf = [0u8; 4];
        len_buf.copy_from_slice(&bytes[off..off + 4]);
        let len = u32::from_le_bytes(len_buf) as usize;
        let end = off + 8 + len;
        if end > bytes.len() {
            break;
        }
        if is_seal_payload(&bytes[off + 8..end]) {
            break;
        }
        out.push((off, end));
        off = end;
    }
    out
}

pub fn sealed_path(dir: &Path, i: usize) -> PathBuf {
    let v: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(dir.join(store::MANIFEST_REL)).unwrap()).unwrap();
    dir.join(
        v["sealed"][i]["name"]
            .as_str()
            .unwrap_or_else(|| panic!("sealed[{i}].name")),
    )
}

/// XOR one payload byte in a middle sealed segment. Open must fail closed: CRC mismatch.
pub fn flip_middle_segment_payload(dir: &Path) -> usize {
    let n = sealed_count(dir);
    assert!(
        n >= 3,
        "need ≥3 sealed segments to flip a middle file; n={n}"
    );
    let mid = n / 2;
    let path = sealed_path(dir, mid);
    let mut bytes = fs::read(&path).unwrap();
    let ranges = event_frame_ranges(&bytes);
    assert!(
        ranges.len() >= 2,
        "middle sealed {} must have a payload frame to flip",
        path.display()
    );
    let payload = ranges[1].0 + 8;
    bytes[payload] ^= 0x01;
    fs::write(&path, &bytes).unwrap();
    mid
}

/// XOR one nibble of the middle sealed entry's manifest `chain` field.
pub fn flip_manifest_chain(dir: &Path) -> usize {
    let path = dir.join(store::MANIFEST_REL);
    let mut v: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
    let n = v["sealed"].as_array().map(|a| a.len()).unwrap_or(0);
    assert!(n >= 3, "need ≥3 sealed entries to flip middle chain; n={n}");
    let mid = n / 2;
    let chain = v["sealed"][mid]["chain"]
        .as_str()
        .expect("sealed.chain")
        .to_string();
    let mut chars: Vec<char> = chain.chars().collect();
    if let Some(last) = chars.last_mut() {
        *last = if *last == '0' { '1' } else { '0' };
    }
    v["sealed"][mid]["chain"] = serde_json::Value::String(chars.into_iter().collect());
    fs::write(&path, serde_json::to_vec(&v).unwrap()).unwrap();
    mid
}

/// Fail-closed property: `open` must return `InvalidData` (message unrestricted).
/// Skip-CRC still fail-closes here (`sealed frame is not an Event`).
pub fn open_must_fail_invalid_data(dir: &Path, seed: u32, case: usize) -> std::io::Error {
    match store::open(dir) {
        Ok(_) => fail_seed(seed, case, &[], "open must fail closed (InvalidData)"),
        Err(e) => {
            assert_eq!(
                e.kind(),
                std::io::ErrorKind::InvalidData,
                "seed={seed} case={case}: fail closed InvalidData, got {e}"
            );
            e
        }
    }
}

/// Named-detector pin (not generic fail-closed). CRC-disabled dies on this needle;
/// the JSON-parse fallback is a different message and fails this assert.
pub fn pin_error_detector(err: &std::io::Error, needle: &str, seed: u32, case: usize) {
    let msg = err.to_string();
    assert!(
        msg.contains(needle),
        "seed={seed} case={case}: detector pin {needle:?}; got {msg}"
    );
}

pub fn open_fails_closed(dir: &Path, needle: &str, seed: u32, case: usize) {
    let err = open_must_fail_invalid_data(dir, seed, case);
    pin_error_detector(&err, needle, seed, case);
}

#[derive(Default)]
pub struct MixCoverage {
    pub n: usize,
    pub retract: usize,
    pub correct: usize,
    pub interval: usize,
    pub horn: usize,
    pub vt_inside: usize,
    pub vt_total: usize,
}

pub fn scan_mix_coverage(rt: &Runtime, meta: &MixMeta, style: MixStyle) -> MixCoverage {
    let mut saw_r = false;
    let mut saw_c = false;
    let mut saw_i = false;
    let mut saw_h = false;
    for e in rt.log().iter() {
        match &e.op {
            Op::Retract { .. } => saw_r = true,
            Op::Correct { .. } => saw_c = true,
            Op::CorrectInterval { .. } => saw_i = true,
            Op::Behavior { name, .. } if name == "inverse_knows" || name == "alsoKnows" => {
                saw_h = true;
            }
            _ => {}
        }
    }
    let cuts = vt_cuts_for(meta.min_vf, meta.max_vf);
    MixCoverage {
        n: 1,
        retract: usize::from(saw_r),
        correct: usize::from(saw_c),
        interval: usize::from(saw_i),
        horn: usize::from(saw_h && matches!(style, MixStyle::WithHorn | MixStyle::Coverage)),
        vt_inside: cuts
            .iter()
            .filter(|vt| vt_cut_inside(**vt, meta.min_vf, meta.max_vf))
            .count(),
        vt_total: cuts.len(),
    }
}
