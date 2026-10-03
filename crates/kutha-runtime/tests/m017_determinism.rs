//! M017 S03 / DET-01: seeded LCG mix (D-01). No proptest/bolero.
//! Eager vs Lazy compare sorted triples, not fingerprints (EventIds differ).

use kutha_common::{EventId, Op, ValidTime};
use kutha_runtime::{FoldIndexMode, GraphFold, Runtime, RuntimeError};
use std::fs;
use std::path::PathBuf;

const SEEDS: [u32; 8] = [1, 7, 13, 42, 99, 12345, 0xA5A5_A5A5, 0xC0FFEE];
const KILL_SEED: u32 = 42;
const N: u32 = 24;
const PATCH_FROM: u64 = 2015;
const PATCH_TO: u64 = 2020;

struct Lcg(u32);

impl Lcg {
    fn from_seed(seed: u32) -> Self {
        Self(seed)
    }

    fn next_u32(&mut self) -> u32 {
        self.0 = self.0.wrapping_mul(1664525).wrapping_add(1013904223);
        self.0 >> 16
    }

    fn draw100(&mut self) -> u32 {
        self.next_u32() % 100
    }
}

#[derive(Clone, Copy)]
struct MixMeta {
    last_wall: Option<u64>,
    saw_assert: bool,
    saw_anchor: bool,
}

fn persist_dir(label: &str) -> PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("tmp")
        .join(format!("p43-01-{label}-{nanos}"));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn sorted(mut triples: Vec<(u32, u32, u32)>) -> Vec<(u32, u32, u32)> {
    triples.sort_unstable();
    triples
}

fn view_cuts(rt: &Runtime, last_wall: u64) -> [Vec<(u32, u32, u32)>; 4] {
    [
        sorted(rt.as_of(ValidTime::from(2012))),
        sorted(rt.as_of(ValidTime::from(2017))),
        sorted(rt.as_known_on(last_wall).unwrap_or_default()),
        sorted(rt.live_at(u64::MAX, ValidTime::from(2012))),
    ]
}

fn emit_assert(rt: &mut Runtime, i: u32, related: u32, object: u32) -> EventId {
    let subject = rt.intern(&format!("s{i}"));
    let valid_from = 2010 + u64::from(i % 5);
    let keyed = i % 4 == 0;
    let op = |delivery_key: Option<String>| Op::Assert {
        subject,
        relation: related,
        object,
        valid_from,
        valid_to: None,
        claim: None,
        delivery_key,
        polarity: None,
    };
    let out = match rt.emit(op(keyed.then(|| format!("k{}", i % 3)))) {
        Ok(out) => out,
        // k{i%3} collides across distinct s{i}; mix continues unkeyed (retry oracle is separate).
        Err(RuntimeError::DeliveryKeyConflict { .. }) if keyed => rt
            .emit(op(None))
            .unwrap_or_else(|e| panic!("DET-01 unkeyed Assert failed i={i}: {e}")),
        Err(e) => panic!("DET-01 Assert failed i={i}: {e}"),
    };
    out.receipt.event_ids[0]
}

fn ingest_mix(rt: &mut Runtime, seed: u32) -> MixMeta {
    let mut lcg = Lcg::from_seed(seed);
    let related = rt.intern("relatedTo");
    let object = rt.intern("P");
    let mut pool: Vec<EventId> = Vec::new();
    let mut last_wall = None;
    let mut saw_assert = false;
    let mut saw_anchor = false;
    for i in 0..N {
        let draw = lcg.draw100();
        let want_assert = draw < 40
            || (draw < 55 && pool.is_empty())
            || (draw >= 55 && draw < 70 && pool.is_empty());
        if want_assert {
            let id = emit_assert(rt, i, related, object);
            pool.push(id);
            saw_assert = true;
            continue;
        }
        if draw < 55 {
            let event_id = pool.remove(0);
            rt.emit(Op::Retract { event_id }).unwrap_or_else(|e| {
                eprintln!("DET-01 fail seed={seed} op={i} Retract");
                panic!("{e}");
            });
            continue;
        }
        if draw < 70 {
            let p_prime = rt.intern("P-prime");
            let mut patched = false;
            for (idx, event_id) in pool.iter().copied().enumerate() {
                match rt.emit(Op::CorrectInterval {
                    event_id,
                    object: p_prime,
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
                    Err(RuntimeError::IntervalPatchRejected { .. }) => {}
                    Err(e) => {
                        eprintln!("DET-01 fail seed={seed} op={i} CorrectInterval");
                        panic!("{e}");
                    }
                }
            }
            if !patched {
                let id = emit_assert(rt, i, related, object);
                pool.push(id);
                saw_assert = true;
            }
            continue;
        }
        if draw < 85 {
            let wall = 2014 + u64::from(i);
            match rt.emit_tt_anchor(wall) {
                Ok(_) => {
                    last_wall = Some(wall);
                    saw_anchor = true;
                }
                Err(RuntimeError::TtAnchorRejected { .. }) => {}
                Err(e) => {
                    eprintln!("DET-01 fail seed={seed} op={i} TtAnchor");
                    panic!("{e}");
                }
            }
            continue;
        }
        let _ = rt.intern(&format!("t{i}"));
    }
    MixMeta {
        last_wall,
        saw_assert,
        saw_anchor,
    }
}

fn assert_replay_and_views(seed: u32, persist_label: &str) {
    let _ = KILL_SEED;
    let mut eager = Runtime::with_index_mode(32, FoldIndexMode::Eager);
    let meta = ingest_mix(&mut eager, seed);
    if !meta.saw_assert || !meta.saw_anchor {
        eprintln!("DET-01 fail seed={seed} missing Assert or TtAnchor");
        panic!("mix must include Assert+TtAnchor (T-43-02)");
    }
    assert!(
        !eager.fold().facts().is_empty(),
        "DET-01 seed={seed}: empty fold (T-43-02)"
    );
    let replayed = GraphFold::replay(eager.log().as_slice());
    if replayed.fingerprint() != eager.fold().fingerprint() {
        eprintln!("DET-01 fail seed={seed} replay!=fold");
        panic!("replay fingerprint must equal live fold");
    }

    let mut lazy = Runtime::with_index_mode(32, FoldIndexMode::Lazy);
    let lazy_meta = ingest_mix(&mut lazy, seed);
    assert_eq!(meta.last_wall, lazy_meta.last_wall);
    let last_wall = meta.last_wall.expect("TtAnchor wall");
    let eager_views = view_cuts(&eager, last_wall);
    let lazy_views = view_cuts(&lazy, last_wall);
    if eager_views != lazy_views {
        eprintln!("DET-01 fail seed={seed} Eager!=Lazy views");
        panic!("Eager/Lazy sorted triples must match");
    }

    let dir = persist_dir(persist_label);
    eager.persist(&dir).unwrap();
    let opened = Runtime::open(&dir).unwrap();
    if opened.fold().fingerprint() != eager.fold().fingerprint() {
        eprintln!("DET-01 fail seed={seed} persist+open fingerprint");
        panic!("opened fingerprint must match pre-persist");
    }
    if view_cuts(&opened, last_wall) != eager_views {
        eprintln!("DET-01 fail seed={seed} persist+open views");
        panic!("opened views must match pre-persist");
    }
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn det01_seed1_replay_eager_lazy_persist_open() {
    assert_replay_and_views(SEEDS[0], "s1");
}
