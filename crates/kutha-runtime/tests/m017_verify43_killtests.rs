//! Phase 43 independent kill-tests (public API). Own Mix64 seeds, not the executor's.

use kutha_common::{Op, RuleAst, RuleAtom, RuleTerm, TermId};
use kutha_runtime::{FoldIndexMode, GraphFold, Runtime};
use std::fs;
use std::path::PathBuf;

/// SplitMix64-ish stepper (not xorshift; seeds distinct from typical 42 / DEADBEEF).
struct Mix64(u64);

impl Mix64 {
    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }

    fn below(&mut self, n: usize) -> usize {
        (self.next_u64() as usize) % n.max(1)
    }
}

const SEEDS: [u64; 3] = [0x43A1C0DEF17E9E37, 0xA11CE0437E573143, 0xB0A751ED43F1D801];

fn sorted(mut v: Vec<(TermId, TermId, TermId)>) -> Vec<(TermId, TermId, TermId)> {
    v.sort_unstable();
    v
}

fn opt_sorted(v: Option<Vec<(TermId, TermId, TermId)>>) -> Option<Vec<(TermId, TermId, TermId)>> {
    v.map(sorted)
}

fn persist_dir(label: &str) -> PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("tmp")
        .join(format!("p43-vfy-{label}-{}-{nanos}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn ingest_mix(rt: &mut Runtime, seed: u64) {
    let mut rng = Mix64(seed);
    rt.emit_allow("v43Link", 0, None).unwrap();
    let rel = rt.intern("v43Link");
    let mut nodes = Vec::with_capacity(8);
    for i in 0..8 {
        nodes.push(rt.intern(&format!("v43node{i}")));
    }
    let mut plan: Vec<(usize, usize)> = (0..8).map(|i| (i, (i + 3) % 8)).collect();
    for i in (1..plan.len()).rev() {
        let j = rng.below(i + 1);
        plan.swap(i, j);
    }
    rt.emit_tt_anchor(1995u64).unwrap();
    let _ = rt.intern("v43ghost0");
    let mut minted: Vec<kutha_common::EventId> = Vec::new();
    for (k, (si, oi)) in plan.iter().copied().enumerate() {
        if k == 2 {
            rt.emit_tt_anchor(2005u64).unwrap();
        }
        if k % 3 == 1 {
            let _ = rt.intern(&format!("v43ghost{k}"));
        }
        let out = rt
            .emit(Op::Assert {
                subject: nodes[si],
                relation: rel,
                object: nodes[oi],
                valid_from: 2000,
                valid_to: None,
                claim: None,
                delivery_key: None,
                polarity: None,
            })
            .unwrap();
        minted.push(out.receipt.event_ids[0]);
        if k == 3 {
            let patched = rt.intern("v43patched");
            rt.emit(Op::CorrectInterval {
                event_id: minted[0],
                object: patched,
                patch_from: 2004,
                patch_to: Some(2012),
            })
            .unwrap();
        }
        if k == 5 {
            rt.emit(Op::Retract {
                event_id: minted[1],
            })
            .unwrap();
        }
    }
    rt.emit_tt_anchor(2014u64).unwrap();
    let _ = rt.intern("v43ghost_tail");
    rt.emit_tt_anchor(2020u64).unwrap();
}

fn assert_replay_matches(rt: &Runtime) {
    let replayed = GraphFold::replay(rt.log().as_slice());
    assert_eq!(
        replayed.fingerprint(),
        rt.fold().fingerprint(),
        "GraphFold::replay(log).fingerprint must equal live fold"
    );
}

fn view_bundle(
    rt: &Runtime,
) -> (
    Vec<(TermId, TermId, TermId)>,
    Vec<(TermId, TermId, TermId)>,
    Option<Vec<(TermId, TermId, TermId)>>,
    Option<Vec<(TermId, TermId, TermId)>>,
    Option<Vec<(TermId, TermId, TermId)>>,
    Vec<(TermId, TermId, TermId)>,
    Vec<(TermId, TermId, TermId)>,
) {
    let mid_tt = rt
        .fold()
        .facts()
        .iter()
        .map(|f| f.ingested_at)
        .max()
        .unwrap_or(0)
        / 2;
    (
        sorted(rt.as_of(2000u64)),
        sorted(rt.as_of(2008u64)),
        opt_sorted(rt.as_known_on(1995u64)),
        opt_sorted(rt.as_known_on(2014u64)),
        opt_sorted(rt.as_known_on(2020u64)),
        sorted(rt.live_at(u64::MAX, 2010u64)),
        sorted(rt.live_at(mid_tt, 2008u64)),
    )
}

fn assert_log_mix(rt: &Runtime) {
    let log = rt.log().as_slice();
    assert!(
        log.iter()
            .any(|e| matches!(e.op, Op::CorrectInterval { .. })),
        "mix must contain CorrectInterval"
    );
    assert!(
        log.iter().any(|e| matches!(e.op, Op::Retract { .. })),
        "mix must contain Retract"
    );
    assert!(
        log.iter().any(|e| matches!(e.op, Op::TtAnchor { .. })),
        "mix must contain TtAnchor"
    );
    assert!(
        log.iter().any(|e| matches!(e.op, Op::Define { .. })),
        "mix must contain intern/Define"
    );
    assert!(
        log.iter().any(|e| matches!(e.op, Op::Assert { .. })),
        "mix must contain Assert"
    );
}

#[test]
fn v43_shuffled_mix_replay_fp_eager_lazy_persist_open() {
    for seed in SEEDS {
        let mut eager = Runtime::with_index_mode(32, FoldIndexMode::Eager);
        let mut lazy = Runtime::with_index_mode(32, FoldIndexMode::Lazy);
        ingest_mix(&mut eager, seed);
        ingest_mix(&mut lazy, seed);
        assert_log_mix(&eager);
        assert_replay_matches(&eager);
        assert_replay_matches(&lazy);
        let ve = view_bundle(&eager);
        let vl = view_bundle(&lazy);
        assert_eq!(ve, vl, "Eager == Lazy views seed={seed:#x}");

        let d_e = persist_dir(&format!("e{seed:x}"));
        let d_l = persist_dir(&format!("l{seed:x}"));
        eager.persist(&d_e).unwrap();
        lazy.persist(&d_l).unwrap();
        let opened_e = Runtime::open(&d_e).unwrap();
        let opened_l = Runtime::open(&d_l).unwrap();
        assert_eq!(opened_e.index_mode(), FoldIndexMode::Eager);
        assert_eq!(opened_l.index_mode(), FoldIndexMode::Lazy);
        assert_replay_matches(&opened_e);
        assert_replay_matches(&opened_l);
        let voe = view_bundle(&opened_e);
        let vol = view_bundle(&opened_l);
        assert_eq!(ve, voe, "persist/open Eager views seed={seed:#x}");
        assert_eq!(vl, vol, "persist/open Lazy views seed={seed:#x}");
        assert_eq!(voe, vol, "opened Eager == opened Lazy seed={seed:#x}");
        let _ = fs::remove_dir_all(&d_e);
        let _ = fs::remove_dir_all(&d_l);
    }
}

fn mentors_horn() -> RuleAst {
    let v = |i| RuleTerm::var(i).unwrap();
    let atom = |rel: &str, s, o| RuleAtom::new(rel, s, o).unwrap();
    RuleAst::horn(
        atom("mentors", v(0), v(2)),
        vec![atom("mentors", v(0), v(1)), atom("mentors", v(1), v(2))],
    )
    .unwrap()
}

fn has_triple(view: &[(TermId, TermId, TermId)], s: TermId, r: TermId, o: TermId) -> bool {
    view.iter().any(|t| *t == (s, r, o))
}

fn live_derived(rt: &Runtime, s: TermId, r: TermId, o: TermId) -> bool {
    rt.fold()
        .facts()
        .iter()
        .any(|f| f.subject == s && f.relation == r && f.object() == o && f.invalidated_at.is_none())
}

#[test]
fn v43_horn_two_prem_as_known_on_characterization() {
    let mut rt = Runtime::default();
    rt.emit_allow("mentors", 0, None).unwrap();
    rt.emit_rule_ast(&mentors_horn(), 2010, None).unwrap();
    let ada = rt.intern("v43Ada");
    let bea = rt.intern("v43Bea");
    let cid = rt.intern("v43Cid");
    let rel = rt.intern("mentors");
    rt.emit_tt_anchor(2014u64).unwrap();
    rt.emit(Op::Assert {
        subject: ada,
        relation: rel,
        object: bea,
        valid_from: 2010,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    rt.emit(Op::Assert {
        subject: bea,
        relation: rel,
        object: cid,
        valid_from: 2010,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    rt.emit_tt_anchor(2020u64).unwrap();

    assert!(
        live_derived(&rt, ada, rel, cid),
        "derived mentors(Ada,Cid) must be live on the current fold"
    );
    let k2014 = rt.as_known_on(2014u64).expect("anchor 2014");
    assert!(
        !has_triple(&k2014, ada, rel, bea),
        "as_known_on(2014) hides the later premise"
    );
    assert!(
        !has_triple(&k2014, ada, rel, cid),
        "as_known_on(2014) hides the derived row"
    );
    let k2020 = rt.as_known_on(2020u64).expect("anchor 2020");
    assert!(
        has_triple(&k2020, ada, rel, bea),
        "as_known_on(2020) shows the premise"
    );
    for (label, cut) in [("2014", &k2014), ("2020", &k2020)] {
        let p1 = has_triple(cut, ada, rel, bea);
        let p2 = has_triple(cut, bea, rel, cid);
        let d = has_triple(cut, ada, rel, cid);
        assert_eq!(
            d,
            p1 && p2,
            "derived as_known_on({label}) must match conjunction of premises"
        );
    }
}

#[test]
fn v43_horn_retract_premise_drops_derived() {
    let mut rt = Runtime::default();
    rt.emit_allow("mentors", 0, None).unwrap();
    rt.emit_rule_ast(&mentors_horn(), 2010, None).unwrap();
    let ada = rt.intern("v43Ada");
    let bea = rt.intern("v43Bea");
    let cid = rt.intern("v43Cid");
    let rel = rt.intern("mentors");
    rt.emit_tt_anchor(2014u64).unwrap();
    rt.emit(Op::Assert {
        subject: ada,
        relation: rel,
        object: bea,
        valid_from: 2010,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    let second = rt
        .emit(Op::Assert {
            subject: bea,
            relation: rel,
            object: cid,
            valid_from: 2010,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap();
    let premise_id = second.receipt.event_ids[0];
    assert!(live_derived(&rt, ada, rel, cid));
    rt.emit(Op::Retract {
        event_id: premise_id,
    })
    .unwrap();
    assert!(
        !live_derived(&rt, ada, rel, cid),
        "retracting one premise must drop derived support at the current cut"
    );
}
