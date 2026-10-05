//! M019 S07 shared LCG, stamps, mix ingest, persist layouts.
//! Default ≥64 seeds; `KUTHA_SEEDS=<n>` may raise (never lowers). No wall-clock oracle.
#![allow(dead_code)]

use kutha_common::{Event, EventId, Op, TermId};
use kutha_runtime::{FoldIndexMode, GraphFold, Runtime, RuntimeError, store};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::{env, fs};

pub const DEFAULT_SEEDS: usize = 64;
pub const N_OPS: u32 = 8;
const PATCH_FROM: u64 = 2015;
const PATCH_TO: u64 = 2020;

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

/// Persist/tamper use this many seeds (first of `seeds()`). Scales with `KUTHA_SEEDS`.
pub fn persist_seed_count() -> usize {
    seed_count().min(12)
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
        .join(format!("p62-{label}-{n}"));
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
    let mut t = rt.as_of(vt);
    t.sort_unstable();
    t
}

pub fn vt_cuts() -> [u64; 4] {
    [2009, 2012, 2017, 2025]
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MixStyle {
    /// `relatedTo` only — no inverse_knows cascade.
    BehaviorFree,
    /// Mix `relatedTo` and `knows` (genesis inverse_knows may fire).
    WithHorn,
}

pub struct MixMeta {
    pub last_wall: Option<u64>,
    pub saw_assert: bool,
}

fn emit_assert(
    rt: &mut Runtime,
    subject: TermId,
    relation: TermId,
    object: TermId,
    valid_from: u64,
    valid_to: Option<u64>,
) -> EventId {
    let out = rt
        .emit(Op::Assert {
            subject,
            relation,
            object,
            valid_from,
            valid_to,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap_or_else(|e| panic!("Assert failed: {e}"));
    out.receipt.event_ids[0]
}

/// Seeded mix: Assert / Retract / Correct / CorrectInterval / TtAnchor. No wall clock.
pub fn ingest_mix(rt: &mut Runtime, seed: u32, style: MixStyle) -> MixMeta {
    let mut lcg = Lcg::from_seed(seed);
    let related = rt.intern("relatedTo");
    let knows = rt.intern("knows");
    let obj_a = rt.intern("P");
    let obj_b = rt.intern("P-prime");
    let subjects: Vec<TermId> = (0..4).map(|i| rt.intern(&format!("s{i}"))).collect();
    let mut pool: Vec<EventId> = Vec::new();
    let mut last_wall = Some(2014);
    let mut saw_assert = false;
    let _ = rt.emit_tt_anchor(2014u64);

    for i in 0..N_OPS {
        let draw = lcg.draw100();
        let subj = subjects[lcg.pick(subjects.len())];
        let rel = match style {
            MixStyle::BehaviorFree => related,
            MixStyle::WithHorn if draw % 3 == 0 => knows,
            MixStyle::WithHorn => related,
        };
        let valid_from = 2010 + u64::from(i % 5);
        let valid_to = if draw % 5 == 0 { Some(2019) } else { None };
        let want_assert = pool.is_empty() || draw < 40;
        if want_assert {
            let id = emit_assert(rt, subj, rel, obj_a, valid_from, valid_to);
            pool.push(id);
            saw_assert = true;
            continue;
        }
        if draw < 55 {
            let event_id = pool.remove(0);
            match rt.emit(Op::Retract { event_id }) {
                Ok(_) => {}
                Err(RuntimeError::UnknownFact { .. }) => {
                    let id = emit_assert(rt, subj, rel, obj_a, valid_from, valid_to);
                    pool.push(id);
                    saw_assert = true;
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
                saw_assert = true;
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
                        break;
                    }
                    Err(RuntimeError::UnknownFact { .. }) => {}
                    Err(e) => panic!("Correct seed={seed} i={i}: {e}"),
                }
            }
            if !done {
                let id = emit_assert(rt, subj, rel, obj_a, valid_from, valid_to);
                pool.push(id);
                saw_assert = true;
            }
            continue;
        }
        let wall = 2015 + u64::from(i);
        match rt.emit_tt_anchor(wall) {
            Ok(_) => last_wall = Some(wall),
            Err(RuntimeError::TtAnchorRejected { .. }) => {}
            Err(e) => panic!("TtAnchor seed={seed} i={i}: {e}"),
        }
    }
    MixMeta {
        last_wall,
        saw_assert,
    }
}

pub fn seeded_runtime(seed: u32, style: MixStyle, mode: FoldIndexMode) -> (Runtime, MixMeta) {
    let mut rt = Runtime::with_index_mode(32, mode);
    let meta = ingest_mix(&mut rt, seed, style);
    assert!(
        meta.saw_assert,
        "seed={seed} mix must include a product Assert"
    );
    (rt, meta)
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

pub fn fail_seed(seed: u32, case: usize, events: &[Event], msg: &str) -> ! {
    eprintln!(
        "SEM fail seed={seed} case={case} n={} ops={}",
        events.len(),
        brief_ops(events)
    );
    panic!("{msg}");
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

/// export_jsonl into a fresh jsonl-only dir (no manifest / WAL). Open uses jsonl as SoT.
pub fn open_export_imported(src: &Path, dest: &Path) -> Runtime {
    store::export_jsonl(src).unwrap();
    fs::create_dir_all(dest).unwrap();
    fs::copy(src.join("events.jsonl"), dest.join("events.jsonl")).unwrap();
    store::open(dest).unwrap()
}

pub fn assert_csr_matches_fold(rt: &Runtime, tt: u64, vt: u64, seed: u32, case: usize) {
    let want = sorted_live(rt, tt, vt);
    let typed = rt.typed_csr_lease_at(tt, vt);
    let mut from_typed = Vec::new();
    for v in 0..rt.dictionary().len() as u32 {
        for e in typed.edges_out(v) {
            from_typed.push((v, e.relation, e.object));
        }
    }
    from_typed.sort_unstable();
    if from_typed != want {
        fail_seed(
            seed,
            case,
            rt.log().as_slice(),
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
            fail_seed(
                seed,
                case,
                rt.log().as_slice(),
                "untyped CSR neighbors must equal unique live objects",
            );
        }
    }
}

pub fn assert_views_stable(rt: &Runtime, last_wall: Option<u64>, seed: u32, case: usize) {
    for (i, vt) in vt_cuts().iter().enumerate() {
        let as_of = sorted_as_of(rt, *vt);
        let live_max = sorted_live(rt, u64::MAX, *vt);
        if as_of != live_max {
            fail_seed(
                seed,
                case + i,
                rt.log().as_slice(),
                "as_of(vt) must equal live_at(MAX, vt)",
            );
        }
        assert_csr_matches_fold(rt, u64::MAX, *vt, seed, case + 10 + i);
    }
    if let Some(wall) = last_wall {
        let Some(mut known) = rt.as_known_on(wall) else {
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
        assert_csr_matches_fold(rt, mid, 2017, seed, case + 40);
        assert_csr_matches_fold(rt, mid, 2012, seed, case + 41);
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
