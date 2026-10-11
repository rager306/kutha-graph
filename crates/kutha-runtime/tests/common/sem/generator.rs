//! Seeds, LCG, mix ingest, coverage (CE #12).
#![allow(dead_code)]

use super::oracles::{vt_cut_inside, vt_cuts_for};
use kutha_common::{Event, EventId, Op, TermId};
use kutha_runtime::{FoldIndexMode, Runtime, RuntimeError};
use std::env;

pub const DEFAULT_SEEDS: usize = 64;
pub const N_OPS: u32 = 8;
pub const N_PERSIST_OPS: u32 = 4;
const PATCH_FROM: u64 = 2015;
const PATCH_TO: u64 = 2020;
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
            let first = emit_assert_keyed(rt, subj, related, obj_a, 2012, None, Some(key.clone()));
            let before = rt.log().len();
            let second = emit_assert_keyed(rt, subj, related, obj_a, 2012, None, Some(key));
            assert_eq!(first, second, "keyed retry must return the same EventId");
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
                    if rt
                        .emit(Op::Retract {
                            event_id: derived_id,
                        })
                        .is_ok()
                    {
                        meta.saw_derived_correct_or_retract = true;
                    }
                }
                Err(e) => panic!("Correct derived seed coverage: {e}"),
            }
            if !meta.saw_derived_correct_or_retract {
                if rt
                    .emit(Op::Retract {
                        event_id: derived_id,
                    })
                    .is_ok()
                {
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
            if coverage_step(
                rt, &mut meta, &mut pool, &subjects, related, knows, obj_a, obj_b, i,
            ) {
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
