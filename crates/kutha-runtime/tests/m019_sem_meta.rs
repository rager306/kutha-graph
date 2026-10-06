//! SEM-cascade-order, metamorphic oracles, SEM-horn-correct-char (HORN-CORR-01).
//! Default ≥64 seeds; `KUTHA_SEEDS=<n>` may raise (persist/tamper = max(12, n/5)).
//! Characterization is labelled. Failures print seed, case, shrunk ops.

mod common;

use common::sem::{
    MixStyle, PersistLayout, assert_export_preserves_event_order, assert_load_bearing,
    assert_same_stamps_shrunk, assert_seed_budget, assert_swap_not_silent,
    ensure_tamper_swap_bites, fail_seed, fail_shrunk, flip_manifest_chain,
    flip_middle_segment_payload, open_fails_closed, open_must_fail_invalid_data, persist_dir,
    persist_schedule, persist_seed_count, persist_segmented_rollover, persist_with_layout,
    pin_error_detector, replay_check_or_shrink, replay_runtime, rm_dir, runtime_stamps,
    seeded_persist_runtime, seeded_runtime, seeds,
};
use kutha_common::{Op, RuleAst, RuleAtom, RuleTerm, TermId};
use kutha_runtime::{FoldIndexMode, Runtime, store};
use std::fs;

fn inverse_behaviors(rt: &Runtime) -> Vec<(TermId, TermId, TermId)> {
    rt.log()
        .iter()
        .filter_map(|e| match &e.op {
            Op::Behavior {
                name,
                subject,
                relation,
                object,
                ..
            } if name == "inverse_knows" => Some((*subject, *relation, *object)),
            _ => None,
        })
        .collect()
}

fn also_knows_horn() -> RuleAst {
    let v = |i| RuleTerm::var(i).unwrap();
    let atom = |rel: &str, s, o| RuleAtom::new(rel, s, o).unwrap();
    RuleAst::horn(
        atom("alsoKnows", v(0), v(1)),
        vec![atom("knows", v(0), v(1))],
    )
    .unwrap()
}

fn cascade_fixture(mode: FoldIndexMode) -> Runtime {
    let mut rt = Runtime::with_index_mode(32, mode);
    rt.emit_allow("alsoKnows", 0, None).unwrap();
    rt.emit_rule_ast(&also_knows_horn(), 2010, None).unwrap();
    let knows = rt.intern("knows");
    let pairs = [("Alice", "Bob"), ("Carol", "Dana"), ("Eve", "Frank")];
    for (a, b) in pairs {
        let s = rt.intern(a);
        let o = rt.intern(b);
        rt.emit(Op::Assert {
            subject: s,
            relation: knows,
            object: o,
            valid_from: 2010,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap();
    }
    rt
}

fn derived_names_in_order(rt: &Runtime) -> Vec<String> {
    rt.log()
        .iter()
        .filter_map(|e| match &e.op {
            Op::Behavior { name, .. } => Some(name.clone()),
            _ => None,
        })
        .collect()
}

#[test]
fn cascade_order_eager_lazy_inverse_knows() {
    assert_seed_budget();
    let eager = cascade_fixture(FoldIndexMode::Eager);
    let lazy = cascade_fixture(FoldIndexMode::Lazy);
    let e_inv = inverse_behaviors(&eager);
    let l_inv = inverse_behaviors(&lazy);
    if e_inv != l_inv {
        fail_seed(
            0,
            0,
            eager.log().as_slice(),
            "inverse_knows derived triples must match in order Eager vs Lazy",
        );
    }
    if e_inv.len() != 3 {
        fail_seed(
            0,
            1,
            eager.log().as_slice(),
            "three knows Asserts mint three inverses",
        );
    }
    if derived_names_in_order(&eager) != derived_names_in_order(&lazy) {
        fail_seed(
            0,
            2,
            eager.log().as_slice(),
            "Behavior name order must be stable across index modes",
        );
    }
    let e_live: Vec<_> = {
        let mut v = eager.as_of(2017);
        v.sort_unstable();
        v
    };
    let l_live: Vec<_> = {
        let mut v = lazy.as_of(2017);
        v.sort_unstable();
        v
    };
    if e_live != l_live {
        fail_seed(0, 3, eager.log().as_slice(), "as_of picture Eager == Lazy");
    }
}

fn named_vt(rt: &Runtime) -> Vec<(String, String, String, u64, Option<u64>)> {
    let mut rows: Vec<_> = rt
        .fold()
        .facts()
        .iter()
        .filter(|f| common::sem::is_product_rel(rt, f.relation) && f.invalidated_at.is_none())
        .map(|f| {
            (
                rt.dictionary().lookup(f.subject).unwrap_or("?").to_string(),
                rt.dictionary()
                    .lookup(f.relation)
                    .unwrap_or("?")
                    .to_string(),
                rt.dictionary()
                    .lookup(f.object())
                    .unwrap_or("?")
                    .to_string(),
                f.valid_from,
                f.valid_to,
            )
        })
        .collect();
    rows.sort();
    rows
}

#[test]
fn order_independence_disjoint_subjects() {
    assert_seed_budget();
    for (case, seed) in seeds().into_iter().enumerate() {
        let mut lcg = common::sem::Lcg::from_seed(seed);
        let facts = [("u0", "o0"), ("u1", "o1"), ("u2", "o2"), ("u3", "o3")];
        let emit = |rt: &mut Runtime, pairs: &[(&str, &str)]| {
            let rel = rt.intern("relatedTo");
            for (s, o) in pairs {
                let subject = rt.intern(s);
                let object = rt.intern(o);
                rt.emit(Op::Assert {
                    subject,
                    relation: rel,
                    object,
                    valid_from: 2010,
                    valid_to: None,
                    claim: None,
                    delivery_key: None,
                    polarity: None,
                })
                .unwrap();
            }
        };
        let mut a = Runtime::with_index_mode(32, FoldIndexMode::Eager);
        emit(&mut a, &facts);
        let mut perm = facts;
        for i in (1..perm.len()).rev() {
            let j = lcg.pick(i + 1);
            perm.swap(i, j);
        }
        let mut b = Runtime::with_index_mode(32, FoldIndexMode::Eager);
        emit(&mut b, &perm);
        if named_vt(&a) != named_vt(&b) {
            fail_seed(
                seed,
                case,
                a.log().as_slice(),
                "permuting disjoint-subject Asserts must not change (s,r,o,vt) picture",
            );
        }
    }
}

#[test]
fn replay_idempotence_fresh_runtime_twice() {
    assert_seed_budget();
    for (case, seed) in seeds().into_iter().enumerate() {
        let (rt, _) = seeded_runtime(seed, MixStyle::WithHorn, FoldIndexMode::Eager);
        replay_check_or_shrink(&rt, seed, case);
        let events = rt.log().as_slice().to_vec();
        let once = replay_runtime(&rt, &events);
        let twice = replay_runtime(&rt, &events);
        if runtime_stamps(&once) != runtime_stamps(&twice) {
            fail_shrunk(
                seed,
                case,
                &events,
                |prefix| {
                    let x = replay_runtime(&rt, prefix);
                    let y = replay_runtime(&rt, prefix);
                    runtime_stamps(&x) != runtime_stamps(&y)
                },
                "two fresh replays must match",
            );
        }
        assert_load_bearing(&rt, &once, seed, case);
        assert_load_bearing(&once, &twice, seed, case);
        if runtime_stamps(&rt) != runtime_stamps(&once) {
            fail_shrunk(
                seed,
                case,
                &events,
                |prefix| {
                    let x = replay_runtime(&rt, prefix);
                    runtime_stamps(&rt) != runtime_stamps(&x)
                },
                "replay stamps must equal live fold",
            );
        }
        replay_check_or_shrink(&once, seed, case);
    }
}

#[test]
fn persist_roundtrip_random_layouts() {
    assert_seed_budget();
    let mut saw_legacy = false;
    let mut saw_rollover = false;
    let mut saw_migrated = false;
    let mut saw_export = false;
    let mut saw_horn_seg = false;
    let mut saw_horn_mig = false;
    for (i, seed) in seeds().into_iter().take(persist_seed_count()).enumerate() {
        for (pass, case) in persist_schedule(i).into_iter().enumerate() {
            let (rt, _) = seeded_persist_runtime(seed, case.style);
            let dir = persist_dir("meta-p");
            persist_with_layout(&rt, &dir, case.layout);
            let opened = store::open(&dir).unwrap_or_else(|e| {
                fail_shrunk(
                    seed,
                    i * 2 + pass,
                    rt.log().as_slice(),
                    |prefix| common::sem::persist_stamps_disagree(&rt, prefix, case.layout, seed),
                    &format!("persist+open {:?}: {e}", case.layout),
                );
            });
            assert_same_stamps_shrunk(
                &rt,
                &opened,
                seed,
                i * 2 + pass,
                case.layout,
                "persist+open stamps must match in-memory fold",
            );
            replay_check_or_shrink(&opened, seed, i * 2 + pass);
            match case.layout {
                PersistLayout::Legacy => saw_legacy = true,
                PersistLayout::Rollover => saw_rollover = true,
                PersistLayout::Migrated => saw_migrated = true,
                PersistLayout::Segmented => {}
            }
            if case.style == MixStyle::WithHorn && case.layout == PersistLayout::Segmented {
                saw_horn_seg = true;
            }
            if case.style == MixStyle::WithHorn && case.layout == PersistLayout::Migrated {
                saw_horn_mig = true;
            }
            if case.export {
                let dst = persist_dir("meta-exp");
                let imported = common::sem::open_export_imported(&dir, &dst);
                assert_same_stamps_shrunk(
                    &rt,
                    &imported,
                    seed,
                    i * 2 + pass,
                    case.layout,
                    "export_jsonl-imported stamps must match",
                );
                assert_export_preserves_event_order(&dst, &rt, seed, 400 + i * 2 + pass);
                saw_export = true;
                rm_dir(&dst);
            }
            rm_dir(&dir);
        }
    }
    assert!(
        saw_legacy && saw_rollover && saw_migrated && saw_export,
        "persist schedule must cover legacy, rollover, migrated, export-imported"
    );
    assert!(
        saw_horn_seg && saw_horn_mig,
        "persist schedule must put WithHorn through segmented and migrated"
    );
}

/// Tamper: flip / swap / drop one fold-affecting event. Must fail closed or fold must differ.
/// Never silently equal. A swap that leaves the fold equal is allowed only when the two
/// events touch disjoint (subject, relation) sets (`commutes`); overlapping keys + equal
/// fold is a defect. Kind is named in the assertion message.
/// Segmented: flip a middle sealed payload — fail-closed InvalidData (any message) AND
/// named detector pin `CRC mismatch` (skip-CRC dies on the needle; JSON parse is fallback).
/// Manifest chain nibble: open fails closed `manifest disagrees`.
#[test]
fn replay_tamper_flip_swap_drop_never_silently_equal() {
    assert_seed_budget();
    for (case, seed) in seeds().into_iter().take(persist_seed_count()).enumerate() {
        let (mut rt, _) =
            seeded_runtime(seed, persist_schedule(case)[0].style, FoldIndexMode::Eager);
        ensure_tamper_swap_bites(&mut rt);
        let orig = runtime_stamps(&rt);
        let events = rt.log().as_slice().to_vec();
        let users = common::sem::fold_affecting_user(&events, &rt);
        assert!(
            users.len() >= 2,
            "seed={seed}: need ≥2 fold-affecting product ops to tamper"
        );

        let drop_i = users[users.len() / 2].0;
        let mut dropped = events.clone();
        dropped.remove(drop_i);
        let after_drop = replay_runtime(&rt, &dropped);
        assert_ne!(
            orig,
            runtime_stamps(&after_drop),
            "drop: fold must differ (not silently equal) seed={seed} case={case} idx={drop_i}"
        );

        assert_swap_not_silent(&rt, &events, seed, case);

        let mut flipped = events.clone();
        let mut did_flip = false;
        for e in &mut flipped {
            if let Op::Assert { valid_from, .. } = &mut e.op
                && *valid_from >= 2010
            {
                *valid_from += 17;
                did_flip = true;
                break;
            }
        }
        assert!(
            did_flip,
            "seed={seed}: mix must contain a product Assert to flip"
        );
        let after_flip = replay_runtime(&rt, &flipped);
        assert_ne!(
            orig,
            runtime_stamps(&after_flip),
            "flip: fold must differ (not silently equal) seed={seed} case={case}"
        );

        let dest = persist_dir("tamper-flip");
        fs::create_dir_all(&dest).unwrap();
        let jsonl_path = dest.join("events.jsonl");
        let mut body = String::new();
        for e in &flipped {
            body.push_str(&serde_json::to_string(e).unwrap());
            body.push('\n');
        }
        fs::write(&jsonl_path, body).unwrap();
        match store::open(&dest) {
            Err(e) => {
                assert_eq!(
                    e.kind(),
                    std::io::ErrorKind::InvalidData,
                    "flip persist: fail closed InvalidData seed={seed} err={e}"
                );
            }
            Ok(opened) => {
                assert_ne!(
                    orig,
                    runtime_stamps(&opened),
                    "flip persist: open succeeded so fold must differ seed={seed}"
                );
            }
        }
        rm_dir(&dest);
    }

    // Segmented layout: middle-file CRC + manifest chain (two seeds, rollover).
    // Fail-closed InvalidData is the property; `CRC mismatch` pins the detector.
    for (case, seed) in seeds().into_iter().take(2).enumerate() {
        let (rt, _) = seeded_persist_runtime(seed, MixStyle::WithHorn);
        let dir = persist_dir("tamper-crc");
        persist_segmented_rollover(&rt, &dir);
        flip_middle_segment_payload(&dir);
        let err = open_must_fail_invalid_data(&dir, seed, 200 + case);
        pin_error_detector(&err, "CRC mismatch", seed, 200 + case);
        rm_dir(&dir);
    }
    let seed = seeds()[0];
    let (rt, _) = seeded_persist_runtime(seed, MixStyle::WithHorn);
    let dir = persist_dir("tamper-chain");
    persist_segmented_rollover(&rt, &dir);
    flip_manifest_chain(&dir);
    open_fails_closed(&dir, "manifest disagrees", seed, 210);
    rm_dir(&dir);
}

fn live_triple(rt: &Runtime, s: TermId, r: TermId, o: TermId, vt: u64) -> bool {
    rt.fold()
        .facts()
        .iter()
        .any(|f| f.subject == s && f.relation == r && f.object() == o && f.is_live_at(u64::MAX, vt))
}

fn live_known_by(rt: &Runtime, s: TermId, o: TermId) -> bool {
    let kb = rt.dictionary().id("knownBy").expect("knownBy");
    live_triple(rt, s, kb, o, 2017)
}

/// DET-02 / D-DET-02: two fresh `Runtime` ingest sequences of genesis
/// `inverse_knows` (Alice/Bob `knows`) mint equal derived Behavior EventIds.
/// User Assert stays `Event::new` (UUID v7) and is not compared.
#[test]
fn det02_two_ingest_derived_event_ids_equal() {
    fn ingest_derived_behavior_ids() -> Vec<kutha_common::EventId> {
        let mut rt = Runtime::default();
        let alice = rt.intern("Alice");
        let bob = rt.intern("Bob");
        let knows = rt.intern("knows");
        rt.emit(Op::Assert {
            subject: alice,
            relation: knows,
            object: bob,
            valid_from: 2010,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap();
        rt.log()
            .iter()
            .filter_map(|e| match &e.op {
                Op::Behavior { .. } => Some(e.id),
                _ => None,
            })
            .collect()
    }
    let a = ingest_derived_behavior_ids();
    let b = ingest_derived_behavior_ids();
    assert!(
        !a.is_empty(),
        "genesis inverse_knows must mint at least one derived Behavior"
    );
    assert_eq!(
        a, b,
        "two ingest of the same Horn fixture must mint equal derived Behavior EventIds"
    );
}

/// Characterization (HORN-CORR-01): Correct of a Horn premise does not re-mint or
/// retract the derived inverse today (`rule_follow_ons` skips Correct).
/// Also pins current fold behaviour: the superseded premise fact is not live
/// at the post-correction cut (not an owner choice of HORN-CORR-01).
#[test]
fn horn_correct_char_derived_survives_premise_correct() {
    let mut rt = Runtime::default();
    let alice = rt.intern("Alice");
    let bob = rt.intern("Bob");
    let carol = rt.intern("Carol");
    let knows = rt.intern("knows");
    let out = rt
        .emit(Op::Assert {
            subject: alice,
            relation: knows,
            object: bob,
            valid_from: 2010,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap();
    let prem = out.receipt.event_ids[0];
    assert!(
        live_known_by(&rt, bob, alice),
        "characterization: inverse_knows mints Bob knownBy Alice"
    );
    let n_inv_before = inverse_behaviors(&rt).len();
    rt.emit(Op::Correct {
        event_id: prem,
        object: carol,
        valid_from: 2010,
        valid_to: None,
    })
    .unwrap();
    assert!(
        live_known_by(&rt, bob, alice),
        "characterization HORN-CORR-01: derived inverse stays live after Correct of the premise"
    );
    assert!(
        !live_known_by(&rt, carol, alice),
        "characterization HORN-CORR-01: Correct does not mint a new inverse for the replacement object"
    );
    assert_eq!(
        n_inv_before,
        inverse_behaviors(&rt).len(),
        "characterization HORN-CORR-01: no extra inverse_knows Behavior is appended on Correct"
    );
    assert!(
        !live_triple(&rt, alice, knows, bob, 2017),
        "characterization (current fold): superseded premise is not live at the post-correction cut"
    );
    assert!(
        live_triple(&rt, alice, knows, carol, 2017),
        "characterization (current fold): replacement premise is live at the post-correction cut"
    );
}

/// Characterization (HORN-CORR-01): CorrectInterval of a Horn premise likewise
/// leaves the derived inverse in place (no retract_derived, no re-mint).
#[test]
fn horn_correct_interval_char_derived_survives_patch() {
    let mut rt = Runtime::default();
    let alice = rt.intern("Alice");
    let bob = rt.intern("Bob");
    let carol = rt.intern("Carol");
    let knows = rt.intern("knows");
    let out = rt
        .emit(Op::Assert {
            subject: alice,
            relation: knows,
            object: bob,
            valid_from: 2010,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap();
    let prem = out.receipt.event_ids[0];
    assert!(live_known_by(&rt, bob, alice));
    rt.emit(Op::CorrectInterval {
        event_id: prem,
        object: carol,
        patch_from: 2015,
        patch_to: Some(2020),
    })
    .unwrap();
    assert!(
        live_known_by(&rt, bob, alice),
        "characterization HORN-CORR-01: CorrectInterval does not retract the derived inverse"
    );
    assert_eq!(
        1,
        inverse_behaviors(&rt).len(),
        "characterization HORN-CORR-01: CorrectInterval does not append inverse_knows"
    );
    assert!(
        !live_triple(&rt, alice, knows, bob, 2017),
        "characterization (current fold): original premise is not live inside the patched interval"
    );
    assert!(
        live_triple(&rt, alice, knows, carol, 2017),
        "characterization (current fold): patched object is live inside the patched interval"
    );
}
