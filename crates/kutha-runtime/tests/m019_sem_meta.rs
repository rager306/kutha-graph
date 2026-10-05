//! SEM-cascade-order, metamorphic oracles, SEM-horn-correct-char (HORN-CORR-01).
//! Default ≥64 seeds; `KUTHA_SEEDS=<n>` may raise. Characterization is labelled.

mod common;

use common::sem::{
    Lcg, MixStyle, assert_same_stamps, assert_seed_budget, fail_seed, open_export_imported,
    persist_dir, persist_legacy, persist_migrated, persist_seed_count, persist_segmented,
    persist_segmented_rollover, replay_runtime, rm_dir, runtime_stamps, seeded_runtime, seeds,
    shrink_events,
};
use kutha_common::{Event, Op, RuleAst, RuleAtom, RuleTerm, TermId};
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
    let eager = cascade_fixture(FoldIndexMode::Eager);
    let lazy = cascade_fixture(FoldIndexMode::Lazy);
    let e_inv = inverse_behaviors(&eager);
    let l_inv = inverse_behaviors(&lazy);
    assert_eq!(
        e_inv, l_inv,
        "inverse_knows derived triples must match in order Eager vs Lazy"
    );
    assert_eq!(e_inv.len(), 3, "three knows Asserts mint three inverses");
    assert_eq!(
        derived_names_in_order(&eager),
        derived_names_in_order(&lazy),
        "Behavior name order must be stable across index modes"
    );
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
    assert_eq!(e_live, l_live, "as_of picture Eager == Lazy");
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
        let mut lcg = Lcg::from_seed(seed);
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
        rt.replay_check()
            .unwrap_or_else(|e| panic!("live replay_check seed={seed}: {e}"));
        let events = rt.log().as_slice().to_vec();
        let once = replay_runtime(&rt, &events);
        let twice = replay_runtime(&rt, &events);
        if runtime_stamps(&once) != runtime_stamps(&twice) {
            let shrunk = shrink_events(&events, |prefix| {
                let x = replay_runtime(&rt, prefix);
                let y = replay_runtime(&rt, prefix);
                runtime_stamps(&x) != runtime_stamps(&y)
            });
            eprintln!("shrunk ops={}", common::sem::brief_ops(&shrunk));
            fail_seed(seed, case, &shrunk, "two fresh replays must match");
        }
        assert_same_stamps(&rt, &once, seed, case, "replay stamps must equal live fold");
        once.replay_check()
            .unwrap_or_else(|e| panic!("replayed replay_check seed={seed}: {e}"));
    }
}

#[test]
fn persist_roundtrip_random_layouts() {
    assert_seed_budget();
    for seed in seeds().into_iter().take(persist_seed_count()) {
        let (rt, _) = seeded_runtime(seed, MixStyle::BehaviorFree, FoldIndexMode::Eager);
        type PersistFn = fn(&Runtime, &std::path::Path);
        let layouts: [(&str, PersistFn); 3] = [
            ("legacy", persist_legacy),
            ("segmented", persist_segmented),
            ("migrated", persist_migrated),
        ];
        for (label, persist_fn) in layouts {
            let dir = persist_dir(label);
            persist_fn(&rt, &dir);
            let opened = store::open(&dir).unwrap();
            assert_same_stamps(
                &rt,
                &opened,
                seed,
                0,
                "persist+open stamps must match in-memory fold",
            );
            opened
                .replay_check()
                .unwrap_or_else(|e| panic!("opened replay_check {label} seed={seed}: {e}"));
            rm_dir(&dir);
        }
        let roll = persist_dir("roll");
        persist_segmented_rollover(&rt, &roll);
        let opened = store::open(&roll).unwrap();
        assert_same_stamps(
            &rt,
            &opened,
            seed,
            1,
            "rollover persist+open stamps must match",
        );
        rm_dir(&roll);
        let src = persist_dir("exp-src");
        persist_segmented(&rt, &src);
        let dst = persist_dir("exp-dst");
        let imported = open_export_imported(&src, &dst);
        assert_same_stamps(
            &rt,
            &imported,
            seed,
            2,
            "export_jsonl-imported stamps must match",
        );
        rm_dir(&src);
        rm_dir(&dst);
    }
}

fn fold_affecting_user<'a>(events: &'a [Event], rt: &Runtime) -> Vec<(usize, &'a Event)> {
    events
        .iter()
        .enumerate()
        .filter(|(_, e)| match &e.op {
            Op::Assert { relation, .. } | Op::Behavior { relation, .. } => {
                common::sem::is_product_rel(rt, *relation)
            }
            Op::Retract { .. } | Op::Correct { .. } | Op::CorrectInterval { .. } => true,
            _ => false,
        })
        .collect()
}

/// Tamper: flip / swap / drop one fold-affecting event. Must fail closed or fold must differ.
/// Never silently equal. Kind is named in the assertion message.
#[test]
fn replay_tamper_flip_swap_drop_never_silently_equal() {
    assert_seed_budget();
    for (case, seed) in seeds().into_iter().take(persist_seed_count()).enumerate() {
        let (rt, _) = seeded_runtime(seed, MixStyle::BehaviorFree, FoldIndexMode::Eager);
        let orig = runtime_stamps(&rt);
        let events = rt.log().as_slice().to_vec();
        let users = fold_affecting_user(&events, &rt);
        assert!(
            users.len() >= 2,
            "seed={seed}: need ≥2 fold-affecting product ops to tamper"
        );

        // drop
        let drop_i = users[users.len() / 2].0;
        let mut dropped = events.clone();
        dropped.remove(drop_i);
        let after_drop = replay_runtime(&rt, &dropped);
        assert_ne!(
            orig,
            runtime_stamps(&after_drop),
            "drop: fold must differ (not silently equal) seed={seed} case={case} idx={drop_i}"
        );

        // swap two fold-affecting events
        let i = users[0].0;
        let j = users[users.len() - 1].0;
        if i != j {
            let mut swapped = events.clone();
            swapped.swap(i, j);
            let after_swap = replay_runtime(&rt, &swapped);
            let swap_stamps = runtime_stamps(&after_swap);
            let swap_ok = orig != swap_stamps;
            assert!(
                swap_ok,
                "swap: fold must differ (not silently equal) seed={seed} case={case} i={i} j={j}"
            );
        }

        // flip a fold-affecting Assert.valid_from (same event list otherwise)
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

        // persist jsonl-only with the flipped Assert: fail closed or fold differs
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
}

fn live_known_by(rt: &Runtime, s: TermId, o: TermId) -> bool {
    let kb = rt.dictionary().id("knownBy").expect("knownBy");
    rt.fold().facts().iter().any(|f| {
        f.subject == s
            && f.relation == kb
            && f.object() == o
            && f.invalidated_at.is_none()
            && f.is_live_at(u64::MAX, 2017)
    })
}

/// Characterization (HORN-CORR-01): Correct of a Horn premise does not re-mint or
/// retract the derived inverse today (`rule_follow_ons` skips Correct).
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
}
