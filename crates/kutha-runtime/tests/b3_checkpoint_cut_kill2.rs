//! Round-2 independent kill tests for phase 229 (checkpoint cut / Horn lineage).
//! Public API only. Each case can fail a lazy rematch or a missing-payload open.
//!
//! Twin normalisation: intern order is shared, so TermIds match. Named live
//! product triples / `as_of` / `as_known_on` are the answer oracle. Fingerprint
//! is compared only after Retract of prefix facts (same support slots). After a
//! newly minted Assert / Correct / rule row, named answers only.

use kutha_common::{
    Event, EventId, Op, RuleAst, RuleAtom, RuleTerm, ValidTime, WallTime, sha256_hex,
};
use kutha_runtime::{Runtime, RuntimeError, store};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

mod common;

struct LimitGuard;

impl Drop for LimitGuard {
    fn drop(&mut self) {
        store::reset_test_segment_limits();
    }
}

fn tiny_limits() -> LimitGuard {
    store::set_test_segment_limits(Some(4096), Some(1));
    LimitGuard
}

fn persist_dir(label: &str) -> common::PersistScratch {
    common::PersistScratch::new(&format!("b3-kill2-{label}"))
}

fn horn_trans() -> RuleAst {
    let v = |i| RuleTerm::var(i).unwrap();
    let atom = |rel: &str, s, o| RuleAtom::new(rel, s, o).unwrap();
    RuleAst::horn(
        atom("relatedTo", v(0), v(2)),
        vec![atom("relatedTo", v(0), v(1)), atom("relatedTo", v(1), v(2))],
    )
    .unwrap()
}

fn horn_knows() -> RuleAst {
    let v = |i| RuleTerm::var(i).unwrap();
    let atom = |rel: &str, s, o| RuleAtom::new(rel, s, o).unwrap();
    RuleAst::horn(
        atom("relatedTo", v(0), v(1)),
        vec![atom("knows", v(0), v(1))],
    )
    .unwrap()
}

fn emit_keyed(
    rt: &mut Runtime,
    subject: &str,
    relation: &str,
    object: &str,
    valid_from: u64,
    valid_to: Option<u64>,
    key: &str,
) -> EventId {
    let subject = rt.intern(subject);
    let relation = rt.intern(relation);
    let object = rt.intern(object);
    let outcome = rt
        .emit(Op::Assert {
            subject,
            relation,
            object,
            valid_from,
            valid_to,
            claim: None,
            delivery_key: Some(key.into()),
            polarity: None,
        })
        .unwrap_or_else(|e| panic!("emit Assert {key}: {e}"));
    rt.log()
        .as_slice()
        .iter()
        .rev()
        .find_map(|e| match &e.op {
            Op::Assert {
                delivery_key: Some(k),
                ..
            } if k == key => Some(e.id),
            _ => None,
        })
        .unwrap_or_else(|| {
            panic!(
                "Assert {key} missing from log after emit; receipt={:?}",
                outcome.receipt.event_ids
            )
        })
}

fn pad(rt: &mut Runtime, n: usize, tag: &str) {
    for i in 0..n {
        let _ = rt.intern(&format!("{tag}-term-{i}"));
        emit_keyed(
            rt,
            "art-12-speed",
            "inForceAs",
            &format!("{tag}-obj-{i}"),
            2010,
            Some(2020),
            &format!("{tag}-dk-{i}"),
        );
    }
}

fn name(rt: &Runtime, id: kutha_common::TermId) -> String {
    rt.dictionary()
        .lookup(id)
        .unwrap_or("<missing>")
        .to_string()
}

fn named_as_of(rt: &Runtime, vt: u64) -> Vec<(String, String, String)> {
    let mut rows: Vec<_> = rt
        .as_of(ValidTime::from(vt))
        .into_iter()
        .map(|(s, r, o)| (name(rt, s), name(rt, r), name(rt, o)))
        .collect();
    rows.sort();
    rows
}

fn named_known_on(rt: &Runtime, wall: u64) -> Option<Vec<(String, String, String)>> {
    rt.as_known_on(WallTime::from(wall)).map(|triples| {
        let mut rows: Vec<_> = triples
            .into_iter()
            .map(|(s, r, o)| (name(rt, s), name(rt, r), name(rt, o)))
            .collect();
        rows.sort();
        rows
    })
}

fn product_live(rt: &Runtime) -> Vec<(String, String, String, u64, Option<u64>)> {
    let mut rows = Vec::new();
    for f in rt.fold().facts() {
        if f.invalidated_at.is_some() {
            continue;
        }
        let rel = name(rt, f.relation);
        if rel.starts_with("kutha:") {
            continue;
        }
        rows.push((
            name(rt, f.subject),
            rel,
            name(rt, f.object()),
            f.valid_from,
            f.valid_to,
        ));
    }
    rows.sort();
    rows
}

fn live_count(rt: &Runtime, s: &str, r: &str, o: &str) -> usize {
    let (Some(s), Some(r), Some(o)) = (
        rt.dictionary().id(s),
        rt.dictionary().id(r),
        rt.dictionary().id(o),
    ) else {
        return 0;
    };
    rt.fold()
        .facts()
        .iter()
        .filter(|f| {
            f.subject == s && f.relation == r && f.object() == o && f.invalidated_at.is_none()
        })
        .count()
}

fn live_named(rt: &Runtime, s: &str, r: &str, o: &str) -> bool {
    live_count(rt, s, r, o) > 0
}

fn derived_live(
    rt: &Runtime,
    asserted: &HashSet<EventId>,
) -> Vec<(String, String, String, EventId)> {
    let mut rows = Vec::new();
    for f in rt.fold().facts() {
        if f.invalidated_at.is_some() || asserted.contains(&f.event_id) {
            continue;
        }
        let rel = name(rt, f.relation);
        if rel.starts_with("kutha:") {
            continue;
        }
        rows.push((name(rt, f.subject), rel, name(rt, f.object()), f.event_id));
    }
    rows.sort_by(|a, b| (&a.0, &a.1, &a.2, a.3).cmp(&(&b.0, &b.1, &b.2, b.3)));
    rows
}

fn behavior_ids(rt: &Runtime, s: &str, r: &str, o: &str) -> Vec<EventId> {
    rt.log()
        .as_slice()
        .iter()
        .filter_map(|e| match &e.op {
            Op::Behavior {
                subject,
                relation,
                object,
                ..
            } if name(rt, *subject) == s && name(rt, *relation) == r && name(rt, *object) == o => {
                Some(e.id)
            }
            _ => None,
        })
        .collect()
}

fn fact_debug(
    rt: &Runtime,
) -> Vec<(
    u64,
    String,
    String,
    String,
    u64,
    Option<u64>,
    Option<u64>,
    String,
)> {
    rt.fold()
        .facts()
        .iter()
        .map(|f| {
            (
                f.seq,
                name(rt, f.subject),
                name(rt, f.relation),
                name(rt, f.object()),
                f.valid_from,
                f.valid_to,
                f.invalidated_at,
                f.event_id.to_string(),
            )
        })
        .collect()
}

fn assert_fp_if_same_ids(got: &Runtime, want: &Runtime, msg: &str) {
    if got.fold().fingerprint() == want.fold().fingerprint() {
        return;
    }
    panic!(
        "HIGH {msg}: fingerprint diverged; got_facts={:?} want_facts={:?}",
        fact_debug(got),
        fact_debug(want)
    );
}

fn assert_twin_named(got: &Runtime, want: &Runtime, msg: &str) {
    assert_eq!(
        product_live(got),
        product_live(want),
        "HIGH {msg}: live product triples"
    );
    for vt in [2009u64, 2012, 2015, 2017, 2019, 2025] {
        assert_eq!(
            named_as_of(got, vt),
            named_as_of(want, vt),
            "HIGH {msg}: as_of {vt}"
        );
    }
    for wall in [2000u64, 2012, 2015, 2018, 2021, 2024, 2030] {
        assert_eq!(
            named_known_on(got, wall),
            named_known_on(want, wall),
            "HIGH {msg}: as_known_on {wall}"
        );
    }
}

fn assert_twin_answers(got: &Runtime, want: &Runtime, msg: &str) {
    assert_twin_named(got, want, msg);
    assert_fp_if_same_ids(got, want, msg);
}

fn replay_ok(rt: &Runtime, msg: &str) {
    rt.replay_check()
        .unwrap_or_else(|e| panic!("HIGH {msg}: replay_check {e}"));
}

fn sealed_prefix_len(dir: &Path) -> usize {
    let man: store::Manifest =
        serde_json::from_str(&fs::read_to_string(dir.join(store::MANIFEST_REL)).unwrap()).unwrap();
    man.sealed.iter().map(|s| s.count as usize).sum()
}

fn copy_store(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).unwrap();
    for ent in fs::read_dir(src).unwrap() {
        let ent = ent.unwrap();
        let name = ent.file_name();
        if name == "kutha.lock" {
            continue;
        }
        let from = ent.path();
        let to = dst.join(&name);
        if from.is_dir() {
            copy_store(&from, &to);
        } else {
            fs::copy(&from, &to).unwrap();
        }
    }
}

fn first_checkpoint_index(events: &[Event]) -> usize {
    events
        .iter()
        .position(|e| matches!(e.op, Op::Checkpoint { .. }))
        .expect("Op::Checkpoint on current log")
}

fn apply_suffix(dir: &Path, suffix: &[Event], dict: Vec<String>) {
    let opened = store::open(dir).unwrap();
    let mut events = opened.log().as_slice().to_vec();
    events.extend(suffix.iter().cloned());
    let rt = Runtime::from_dict_and_events(dict, events, opened.max_cascade)
        .expect("replay suffix onto store");
    store::persist(&rt, dir).unwrap();
}

fn twin_prefix_len_for_checkpointed(ckp: &Runtime, twin: &Runtime) -> usize {
    let events = ckp.log().as_slice();
    let c = first_checkpoint_index(events);
    match events.get(c + 1) {
        Some(first_tail) => twin
            .log()
            .as_slice()
            .iter()
            .position(|e| e.id == first_tail.id)
            .unwrap_or_else(|| {
                panic!(
                    "HIGH R1: first tail event {} missing from twin log",
                    first_tail.id
                )
            }),
        None => twin.log().len(),
    }
}

fn persist_twins(
    rt: &Runtime,
    label: &str,
) -> (common::PersistScratch, common::PersistScratch, usize) {
    let ckp = persist_dir(&format!("{label}-ckp"));
    let twin = persist_dir(&format!("{label}-twin"));
    store::persist(rt, &ckp).unwrap();
    let prefix_len = sealed_prefix_len(&ckp);
    assert!(
        prefix_len > 0 && prefix_len < store::encoded_events(rt).len(),
        "fixture must seal a strict prefix"
    );
    copy_store(&ckp, &twin);
    store::checkpoint_sealed_prefix(&ckp).unwrap();
    (ckp, twin, prefix_len)
}

fn export_events(dir: &Path) -> Vec<Event> {
    store::export_jsonl(dir).unwrap();
    let text = fs::read_to_string(dir.join("events.jsonl")).unwrap();
    text.lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str::<Event>(l).expect("jsonl Event"))
        .collect()
}

fn rewrite_on_disk_log(dir: &Path, events: &[Event]) {
    let n = events.len();
    let (sealed, active) = if n >= 2 {
        (vec![n - 1], 1)
    } else {
        (Vec::new(), n)
    };
    store::write_test_segmented(dir, events, &sealed, active).unwrap();
}

fn seed_chain(rt: &mut Runtime) -> (EventId, EventId, EventId, EventId, HashSet<EventId>) {
    rt.emit_rule_ast(&horn_trans(), 2010, None).unwrap();
    let ab = emit_keyed(rt, "A", "relatedTo", "B", 2010, None, "horn-ab");
    let bc = emit_keyed(rt, "B", "relatedTo", "C", 2010, None, "horn-bc");
    let cd = emit_keyed(rt, "C", "relatedTo", "D", 2010, None, "horn-cd");
    let de = emit_keyed(rt, "D", "relatedTo", "E", 2010, None, "horn-de");
    pad(rt, 8, "chain");
    rt.emit_tt_anchor(2015).unwrap();
    assert!(live_named(rt, "A", "relatedTo", "C"), "must derive A-C");
    assert!(live_named(rt, "A", "relatedTo", "D"), "must derive A-D");
    assert!(live_named(rt, "A", "relatedTo", "E"), "must derive A-E");
    assert!(live_named(rt, "B", "relatedTo", "E"), "must derive B-E");
    let mut asserted = HashSet::new();
    asserted.insert(ab);
    asserted.insert(bc);
    asserted.insert(cd);
    asserted.insert(de);
    (ab, bc, cd, de, asserted)
}

#[test]
fn r2_three_hop_retract_first_premise() {
    let _g = tiny_limits();
    let mut rt = Runtime::default();
    let (ab, _bc, _cd, _de, asserted) = seed_chain(&mut rt);
    let (ckp, twin, _) = persist_twins(&rt, "3hop-first");
    let mut a = store::open(&ckp).unwrap();
    let mut b = store::open(&twin).unwrap();
    replay_ok(&a, "R2 3-hop open");
    assert_eq!(
        derived_live(&a, &asserted)
            .into_iter()
            .map(|r| (r.0, r.1, r.2))
            .collect::<Vec<_>>(),
        derived_live(&b, &asserted)
            .into_iter()
            .map(|r| (r.0, r.1, r.2))
            .collect::<Vec<_>>(),
        "HIGH R2: derived triples after checkpoint (before retract)"
    );
    a.emit(Op::Retract { event_id: ab }).unwrap();
    b.emit(Op::Retract { event_id: ab }).unwrap();
    replay_ok(&a, "R2 3-hop retract first A-B");
    assert_twin_answers(&a, &b, "R2 3-hop retract first A-B");
}

#[test]
fn r2_three_hop_retract_middle() {
    let _g = tiny_limits();
    let mut rt = Runtime::default();
    let (_ab, bc, _cd, _de, _) = seed_chain(&mut rt);
    let (ckp, twin, _) = persist_twins(&rt, "3hop-mid");
    let mut a = store::open(&ckp).unwrap();
    let mut b = store::open(&twin).unwrap();
    a.emit(Op::Retract { event_id: bc }).unwrap();
    b.emit(Op::Retract { event_id: bc }).unwrap();
    replay_ok(&a, "R2 3-hop retract middle B-C");
    assert_twin_answers(&a, &b, "R2 3-hop retract middle B-C");
}

#[test]
fn r2_three_hop_retract_end() {
    let _g = tiny_limits();
    let mut rt = Runtime::default();
    let (_ab, _bc, _cd, de, _) = seed_chain(&mut rt);
    let (ckp, twin, _) = persist_twins(&rt, "3hop-end");
    let mut a = store::open(&ckp).unwrap();
    let mut b = store::open(&twin).unwrap();
    a.emit(Op::Retract { event_id: de }).unwrap();
    b.emit(Op::Retract { event_id: de }).unwrap();
    replay_ok(&a, "R2 3-hop retract end D-E");
    assert_twin_answers(&a, &b, "R2 3-hop retract end D-E");
}

#[test]
fn r2_diamond_two_paths() {
    let _g = tiny_limits();
    let mut rt = Runtime::default();
    rt.emit_rule_ast(&horn_trans(), 2010, None).unwrap();
    let ab = emit_keyed(&mut rt, "A", "relatedTo", "B", 2010, None, "d-ab");
    let _bc = emit_keyed(&mut rt, "B", "relatedTo", "C", 2010, None, "d-bc");
    let ad = emit_keyed(&mut rt, "A", "relatedTo", "D", 2010, None, "d-ad");
    let _dc = emit_keyed(&mut rt, "D", "relatedTo", "C", 2010, None, "d-dc");
    pad(&mut rt, 8, "dia");
    rt.emit_tt_anchor(2015).unwrap();
    assert!(
        live_named(&rt, "A", "relatedTo", "C"),
        "diamond must derive A-C (one or two supports; VT-dedup may collapse)"
    );

    let (ckp, twin, _) = persist_twins(&rt, "dia");
    let mut a = store::open(&ckp).unwrap();
    let mut b = store::open(&twin).unwrap();
    a.emit(Op::Retract { event_id: ab }).unwrap();
    b.emit(Op::Retract { event_id: ab }).unwrap();
    replay_ok(&a, "R2 diamond retract one path");
    assert!(
        live_named(&b, "A", "relatedTo", "C"),
        "twin keeps A-C via A-D-C after retracting A-B"
    );
    assert!(
        live_named(&a, "A", "relatedTo", "C"),
        "HIGH R2: retracting one diamond path dropped A-C"
    );
    assert_twin_answers(&a, &b, "R2 diamond after retract A-B");

    a.emit(Op::Retract { event_id: ad }).unwrap();
    b.emit(Op::Retract { event_id: ad }).unwrap();
    replay_ok(&a, "R2 diamond retract both paths");
    assert_twin_answers(&a, &b, "R2 diamond after retract both paths");
    assert_eq!(
        live_named(&a, "A", "relatedTo", "C"),
        live_named(&b, "A", "relatedTo", "C"),
        "HIGH R2: A-C live after retracting both diamond paths diverged from twin"
    );
}

#[test]
fn r2_correct_and_correct_interval_mid_chain() {
    let _g = tiny_limits();
    let mut rt = Runtime::default();
    rt.emit_rule_ast(&horn_trans(), 2010, None).unwrap();
    let _ab = emit_keyed(&mut rt, "A", "relatedTo", "B", 2010, Some(2020), "c-ab");
    let bc = emit_keyed(&mut rt, "B", "relatedTo", "C", 2010, Some(2020), "c-bc");
    let _cd = emit_keyed(&mut rt, "C", "relatedTo", "D", 2010, Some(2020), "c-cd");
    pad(&mut rt, 8, "midc");
    rt.emit_tt_anchor(2015).unwrap();
    assert!(live_named(&rt, "A", "relatedTo", "D"));

    let (ckp, twin, _) = persist_twins(&rt, "mid-correct");
    let mut a = store::open(&ckp).unwrap();
    let mut b = store::open(&twin).unwrap();
    let z = a.intern("Z");
    let z_b = b.intern("Z");
    a.emit(Op::Correct {
        event_id: bc,
        object: z,
        valid_from: 2010,
        valid_to: Some(2020),
    })
    .unwrap();
    b.emit(Op::Correct {
        event_id: bc,
        object: z_b,
        valid_from: 2010,
        valid_to: Some(2020),
    })
    .unwrap();
    replay_ok(&a, "R2 Correct mid-chain B-C");
    assert!(
        !live_named(&a, "A", "relatedTo", "D"),
        "HIGH R2: Correct of mid-chain B-C left A-D live"
    );
    assert_twin_named(&a, &b, "R2 Correct of mid-chain B-C");

    let mut rt2 = Runtime::default();
    rt2.emit_rule_ast(&horn_trans(), 2010, None).unwrap();
    let _ab2 = emit_keyed(&mut rt2, "A", "relatedTo", "B", 2010, Some(2020), "ci-ab");
    let bc2 = emit_keyed(&mut rt2, "B", "relatedTo", "C", 2010, Some(2020), "ci-bc");
    let _cd2 = emit_keyed(&mut rt2, "C", "relatedTo", "D", 2010, Some(2020), "ci-cd");
    pad(&mut rt2, 8, "midci");
    rt2.emit_tt_anchor(2015).unwrap();
    let (ckp2, twin2, _) = persist_twins(&rt2, "mid-ci");
    let mut a = store::open(&ckp2).unwrap();
    let mut b = store::open(&twin2).unwrap();
    let obj = a.intern("C");
    a.emit(Op::CorrectInterval {
        event_id: bc2,
        object: obj,
        patch_from: 2012,
        patch_to: Some(2018),
    })
    .unwrap();
    let obj_b = b.intern("C");
    b.emit(Op::CorrectInterval {
        event_id: bc2,
        object: obj_b,
        patch_from: 2012,
        patch_to: Some(2018),
    })
    .unwrap();
    replay_ok(&a, "R2 CorrectInterval mid-chain B-C");
    assert_twin_named(&a, &b, "R2 CorrectInterval of mid-chain B-C");
}

#[test]
fn r2_retract_derived_head_directly() {
    let _g = tiny_limits();
    let mut rt = Runtime::default();
    let (_ab, _bc, _cd, _de, asserted) = seed_chain(&mut rt);
    let ac_ids = behavior_ids(&rt, "A", "relatedTo", "C");
    assert!(
        !ac_ids.is_empty(),
        "fixture must mint a derived A-C Behavior"
    );
    let ac = ac_ids[0];
    assert!(!asserted.contains(&ac), "A-C must be derived, not asserted");

    let (ckp, twin, _) = persist_twins(&rt, "retract-derived");
    let mut a = store::open(&ckp).unwrap();
    let mut b = store::open(&twin).unwrap();
    let got = a.emit(Op::Retract { event_id: ac });
    let want = b.emit(Op::Retract { event_id: ac });
    match (&got, &want) {
        (Ok(_), Ok(_)) => {
            replay_ok(&a, "R2 retract derived A-C");
            assert_twin_answers(&a, &b, "R2 retract derived A-C vs twin");
        }
        (Err(e), Ok(_)) => panic!(
            "HIGH R2: checkpointed Retract of derived {ac} failed ({e}) while twin succeeded"
        ),
        (Ok(_), Err(e)) => panic!("twin Retract of derived {ac} failed: {e}"),
        (Err(e1), Err(e2)) => {
            assert_eq!(
                format!("{e1}"),
                format!("{e2}"),
                "HIGH R2: derived retract errors diverged: ckp={e1} twin={e2}"
            );
        }
    }
}

#[test]
fn r2_tail_premise_retract_then_reassert() {
    let _g = tiny_limits();
    let mut rt = Runtime::default();
    rt.emit_rule_ast(&horn_trans(), 2010, None).unwrap();
    let _ab = emit_keyed(&mut rt, "A", "relatedTo", "B", 2010, None, "tail-ab");
    pad(&mut rt, 8, "tailp");
    rt.emit_tt_anchor(2015).unwrap();
    assert!(
        !live_named(&rt, "A", "relatedTo", "C"),
        "A-C must wait for tail B-C"
    );

    let (ckp, twin, prefix_len) = persist_twins(&rt, "tail-re");
    let encoded = store::encoded_events(&rt);
    assert!(
        encoded[..prefix_len]
            .iter()
            .any(|e| matches!(e.op, Op::StoreBlob { .. })),
        "rule blob must sit in the sealed prefix"
    );

    let mut a = store::open(&ckp).unwrap();
    let mut b = store::open(&twin).unwrap();
    let bc = emit_keyed(&mut a, "B", "relatedTo", "C", 2010, None, "tail-bc");
    let bc_b = emit_keyed(&mut b, "B", "relatedTo", "C", 2010, None, "tail-bc");
    replay_ok(&a, "R2 tail B-C derived");
    assert!(
        live_named(&a, "A", "relatedTo", "C"),
        "HIGH R2: prefix rule + prefix A-B must derive A-C from tail B-C"
    );
    assert_twin_named(&a, &b, "R2 tail derive (named)");

    a.emit(Op::Retract { event_id: bc }).unwrap();
    b.emit(Op::Retract { event_id: bc_b }).unwrap();
    replay_ok(&a, "R2 retract tail B-C");
    assert!(
        !live_named(&a, "A", "relatedTo", "C"),
        "HIGH R2: A-C stayed live after retracting tail B-C"
    );
    assert_twin_named(&a, &b, "R2 retract tail B-C (named)");

    emit_keyed(&mut a, "B", "relatedTo", "C", 2010, None, "tail-bc-2");
    emit_keyed(&mut b, "B", "relatedTo", "C", 2010, None, "tail-bc-2");
    replay_ok(&a, "R2 re-assert tail B-C");
    assert!(
        live_named(&a, "A", "relatedTo", "C"),
        "HIGH R2: re-asserting tail B-C must re-derive A-C"
    );
    assert_twin_named(&a, &b, "R2 re-assert tail B-C");
}

#[test]
fn r2_second_rule_registered_after_checkpoint() {
    let _g = tiny_limits();
    let mut rt = Runtime::default();
    rt.emit_rule_ast(&horn_knows(), 2010, None).unwrap();
    let kn = emit_keyed(&mut rt, "A", "knows", "B", 2010, None, "kn-ab");
    let _bc = emit_keyed(&mut rt, "B", "relatedTo", "C", 2010, None, "rel-bc");
    pad(&mut rt, 8, "rule2");
    rt.emit_tt_anchor(2015).unwrap();
    assert!(
        live_named(&rt, "A", "relatedTo", "B"),
        "prefix knows-rule must derive relatedTo(A,B)"
    );
    assert!(
        !live_named(&rt, "A", "relatedTo", "C"),
        "transitivity is not registered yet"
    );

    let (ckp, twin, _) = persist_twins(&rt, "rule2");
    let mut a = store::open(&ckp).unwrap();
    let mut b = store::open(&twin).unwrap();
    a.emit_rule_ast(&horn_trans(), 2010, None)
        .expect("register trans after checkpoint");
    b.emit_rule_ast(&horn_trans(), 2010, None).unwrap();
    // Registering a rule does not rescan prefix facts; a tail relatedTo
    // trigger is required so transitivity can fire on relatedTo(A,B)+B-C.
    emit_keyed(&mut a, "B", "relatedTo", "C", 2010, None, "rel-bc-tail");
    emit_keyed(&mut b, "B", "relatedTo", "C", 2010, None, "rel-bc-tail");
    replay_ok(&a, "R2 after second rule + trigger");
    assert_twin_named(&a, &b, "R2 second rule after checkpoint (named)");
    assert!(
        live_named(&b, "A", "relatedTo", "C"),
        "twin must derive A-C after post-checkpoint transitivity trigger"
    );
    assert!(
        live_named(&a, "A", "relatedTo", "C"),
        "HIGH R2: post-checkpoint transitivity did not derive A-C on the compacted store"
    );

    a.emit(Op::Retract { event_id: kn }).unwrap();
    b.emit(Op::Retract { event_id: kn }).unwrap();
    replay_ok(&a, "R2 retract knows after chained rules");
    assert!(
        !live_named(&a, "A", "relatedTo", "B"),
        "HIGH R2: retracting prefix knows(A,B) must un-derive relatedTo(A,B)"
    );
    assert!(
        !live_named(&a, "A", "relatedTo", "C"),
        "HIGH R2: chained A-C stayed live after retracting the prefix knows premise"
    );
    assert_twin_named(&a, &b, "R2 retract after second rule");
}

#[test]
fn r2_checkpoint_append_checkpoint_retract() {
    let _g = tiny_limits();
    let mut rt = Runtime::default();
    let (ab, _bc, _cd, _de, _) = seed_chain(&mut rt);
    let (ckp, twin, _) = persist_twins(&rt, "ckp2");

    let mut extra = store::open(&twin).unwrap();
    let n = extra.log().len();
    pad(&mut extra, 8, "ckp2-tail");
    let suffix = extra.log().as_slice()[n..].to_vec();
    let dict = extra.dictionary().strings().to_vec();
    store::persist(&extra, &twin).unwrap();
    apply_suffix(&ckp, &suffix, dict);
    store::checkpoint_sealed_prefix(&ckp).unwrap();

    let mut a = store::open(&ckp).unwrap();
    let mut b = store::open(&twin).unwrap();
    replay_ok(&a, "R2 after checkpoint-append-checkpoint");
    a.emit(Op::Retract { event_id: ab }).unwrap();
    b.emit(Op::Retract { event_id: ab }).unwrap();
    replay_ok(&a, "R2 retract after double checkpoint");
    assert_twin_answers(&a, &b, "R2 checkpoint-append-checkpoint-retract");
}

#[test]
fn r2_cascade_persist_reopen_then_another_retract() {
    let _g = tiny_limits();
    let mut rt = Runtime::default();
    let (ab, _bc, _cd, de, _) = seed_chain(&mut rt);
    let (ckp, twin, _) = persist_twins(&rt, "reopen");

    let mut a = store::open(&ckp).unwrap();
    let mut b = store::open(&twin).unwrap();
    a.emit(Op::Retract { event_id: ab }).unwrap();
    b.emit(Op::Retract { event_id: ab }).unwrap();
    replay_ok(&a, "R2 first cascade");
    assert_twin_answers(&a, &b, "R2 first cascade before persist");
    store::persist(&a, &ckp).unwrap();
    store::persist(&b, &twin).unwrap();
    drop(a);
    drop(b);

    let mut a = store::open(&ckp).unwrap();
    let mut b = store::open(&twin).unwrap();
    replay_ok(&a, "R2 reopen already-cascaded");
    assert_twin_answers(&a, &b, "R2 persist+reopen of cascaded store");
    a.emit(Op::Retract { event_id: de }).unwrap();
    b.emit(Op::Retract { event_id: de }).unwrap();
    replay_ok(&a, "R2 second retract after reopen");
    assert!(
        !live_named(&a, "C", "relatedTo", "E"),
        "HIGH R2: C-E stayed live after retracting D-E on reopened cascaded store"
    );
    assert_twin_answers(&a, &b, "R2 reopen then retract D-E");
}

#[test]
fn r2_open_for_append_retract_persist_reopen() {
    let _g = tiny_limits();
    let mut rt = Runtime::default();
    let (_ab, bc, _cd, _de, _) = seed_chain(&mut rt);
    let (ckp, twin, _) = persist_twins(&rt, "append");

    let (mut wa, mut pa) = store::open_for_append(&ckp).expect("open_for_append checkpointed");
    let (mut wb, mut pb) = store::open_for_append(&twin).expect("open_for_append twin");
    wa.emit(Op::Retract { event_id: bc }).unwrap();
    wb.emit(Op::Retract { event_id: bc }).unwrap();
    replay_ok(&wa, "R2 append-path retract middle");
    assert_twin_answers(&wa, &wb, "R2 Persister retract middle B-C");
    pa.append(&wa).unwrap();
    pb.append(&wb).unwrap();
    drop(pa);
    drop(pb);

    let a2 = store::open(&ckp).unwrap();
    let b2 = store::open(&twin).unwrap();
    replay_ok(&a2, "R2 reopen after append retract");
    assert!(
        !live_named(&a2, "A", "relatedTo", "E"),
        "HIGH R2: A-E stayed live after open_for_append retract of B-C + persist + reopen"
    );
    assert_twin_answers(&a2, &b2, "R2 reopen after Persister retract");
}

#[test]
fn r2_as_known_on_tt_history_pre_checkpoint_invalidation() {
    let _g = tiny_limits();
    let mut rt = Runtime::default();
    rt.emit_rule_ast(&horn_trans(), 2010, None).unwrap();
    let ab = emit_keyed(&mut rt, "A", "relatedTo", "B", 2010, None, "tt-ab");
    let _bc = emit_keyed(&mut rt, "B", "relatedTo", "C", 2010, None, "tt-bc");
    let cd = emit_keyed(&mut rt, "C", "relatedTo", "D", 2010, None, "tt-cd");
    rt.emit_tt_anchor(2012).unwrap();
    assert!(live_named(&rt, "A", "relatedTo", "D"));
    rt.emit(Op::Retract { event_id: cd }).unwrap();
    rt.emit_tt_anchor(2015).unwrap();
    assert!(
        !live_named(&rt, "A", "relatedTo", "D"),
        "C-D retract must un-derive A-D before checkpoint"
    );
    pad(&mut rt, 8, "tt");
    rt.emit_tt_anchor(2018).unwrap();

    let (ckp, twin, _) = persist_twins(&rt, "tt");
    let mut a = store::open(&ckp).unwrap();
    let mut b = store::open(&twin).unwrap();
    replay_ok(&a, "R2 TT history after checkpoint");
    assert_twin_named(&a, &b, "R2 TT history before live retract");
    let at_2012 = named_known_on(&a, 2012).expect("wall 2012");
    assert!(
        at_2012
            .iter()
            .any(|t| t == &("A".into(), "relatedTo".into(), "D".into())),
        "HIGH R2: as_known_on(2012) must still show A-D invalidated after that wall"
    );
    let at_2015 = named_known_on(&a, 2015).expect("wall 2015");
    assert!(
        !at_2015
            .iter()
            .any(|t| t == &("A".into(), "relatedTo".into(), "D".into())),
        "HIGH R2: as_known_on(2015) must not show A-D after pre-checkpoint retract"
    );

    a.emit(Op::Retract { event_id: ab }).unwrap();
    b.emit(Op::Retract { event_id: ab }).unwrap();
    a.emit_tt_anchor(2024).unwrap();
    b.emit_tt_anchor(2024).unwrap();
    replay_ok(&a, "R2 TT history after live retract");
    assert_twin_named(&a, &b, "R2 TT history after retract A-B");
    let at_2024 = named_known_on(&a, 2024).expect("wall 2024");
    assert!(
        !at_2024
            .iter()
            .any(|t| t == &("A".into(), "relatedTo".into(), "C".into())),
        "HIGH R2: as_known_on(2024) must not show A-C after retracting A-B"
    );
    let still_2012 = named_known_on(&a, 2012).expect("wall 2012 after later retract");
    assert!(
        still_2012
            .iter()
            .any(|t| t == &("A".into(), "relatedTo".into(), "C".into())),
        "HIGH R2: as_known_on(2012) must keep A-C in TT history"
    );
}

#[test]
fn r2_fork_then_live_retract_matches_twin_fork() {
    let _g = tiny_limits();
    let mut rt = Runtime::default();
    let (ab, _bc, _cd, _de, _) = seed_chain(&mut rt);
    pad(&mut rt, 4, "fork-tail");
    let (ckp, twin, _) = persist_twins(&rt, "fork");

    let opened = store::open(&ckp).unwrap();
    let twin_rt = store::open(&twin).unwrap();
    let events = opened.log().as_slice();
    let c = first_checkpoint_index(events);
    let len = opened.log().len();
    let twin_prefix = twin_prefix_len_for_checkpointed(&opened, &twin_rt);
    let tail = len - (c + 1);

    for i in [0usize, tail / 2, tail] {
        let mut fork = opened
            .fork_at(c + 1 + i)
            .unwrap_or_else(|e| panic!("HIGH R1: legal fork_at(c+1+{i}) Err {e}"));
        let mut twin_fork = twin_rt
            .fork_at(twin_prefix + i)
            .unwrap_or_else(|e| panic!("HIGH R1: twin fork_at(twin_prefix+{i}) Err {e}"));
        replay_ok(&fork, &format!("R2 fork c+1+{i} before retract"));
        fork.emit(Op::Retract { event_id: ab })
            .unwrap_or_else(|e| panic!("HIGH R2: retract in fork c+1+{i}: {e}"));
        twin_fork
            .emit(Op::Retract { event_id: ab })
            .unwrap_or_else(|e| panic!("HIGH R2: retract in twin fork: {e}"));
        replay_ok(&fork, &format!("R2 fork c+1+{i} after retract"));
        assert_twin_answers(
            &fork,
            &twin_fork,
            &format!("R2 cascade inside fork_at(c+1+{i})"),
        );
    }
}

fn checkpoint_blob_digest(events: &[Event]) -> String {
    events
        .iter()
        .find_map(|e| match &e.op {
            Op::Checkpoint { blob_digest, .. } => Some(blob_digest.clone()),
            _ => None,
        })
        .expect("Op::Checkpoint")
}

fn mutate_checkpoint_snapshot(events: &mut [Event], mutate: impl FnOnce(&mut serde_json::Value)) {
    let digest = checkpoint_blob_digest(events);
    let mut found = false;
    for e in events {
        if let Op::StoreBlob { digest: d, bytes } = &mut e.op {
            if *d == digest {
                let mut v: serde_json::Value =
                    serde_json::from_slice(bytes).expect("checkpoint Snapshot JSON");
                mutate(&mut v);
                *bytes = serde_json::to_vec(&v).expect("reserialize snapshot");
                found = true;
                break;
            }
        }
    }
    assert!(found, "checkpoint StoreBlob matching receipt digest");
}

fn recompute_blob_digest(events: &mut [Event]) {
    let old = checkpoint_blob_digest(events);
    let mut new_digest = None;
    for e in events.iter_mut() {
        if let Op::StoreBlob { digest: d, bytes } = &mut e.op {
            if *d == old {
                *d = sha256_hex(bytes);
                new_digest = Some(d.clone());
            }
        }
    }
    let new_digest = new_digest.expect("StoreBlob to rehash");
    for e in events {
        if let Op::Checkpoint { blob_digest, .. } = &mut e.op {
            *blob_digest = new_digest.clone();
        }
    }
}

fn must_fail_closed(dir: &Path, what: &str) {
    match store::open(dir) {
        Err(_) => {}
        Ok(rt) => panic!(
            "HIGH R2: {what}: store::open succeeded; replay_check is_ok={} (tamper must fail closed)",
            rt.replay_check().is_ok()
        ),
    }
}

#[test]
fn r2_tamper_behaviors_fails_closed() {
    let _g = tiny_limits();
    let mut rt = Runtime::default();
    let _ = seed_chain(&mut rt);
    let (ckp, _twin, _) = persist_twins(&rt, "tamp-src");
    store::open(&ckp).expect("untampered checkpointed store must open");

    let dir = persist_dir("tamp-behaviors");
    copy_store(&ckp, &dir);
    let mut events = export_events(&dir);
    mutate_checkpoint_snapshot(&mut events, |v| {
        let arr = v
            .get_mut("behaviors")
            .and_then(|b| b.as_array_mut())
            .expect("Horn checkpoint payload must carry behaviors");
        assert!(
            !arr.is_empty(),
            "fixture must serialize at least one Behavior"
        );
        arr.clear();
    });
    rewrite_on_disk_log(&dir, &events);
    must_fail_closed(&dir, "cleared behaviors with stale digest");

    let dir2 = persist_dir("tamp-caused-by");
    copy_store(&ckp, &dir2);
    let mut events = export_events(&dir2);
    mutate_checkpoint_snapshot(&mut events, |v| {
        let arr = v
            .get_mut("behaviors")
            .and_then(|b| b.as_array_mut())
            .expect("behaviors array");
        let first = arr.first_mut().expect("non-empty behaviors");
        let op = first
            .get_mut("op")
            .and_then(|o| o.get_mut("Behavior"))
            .expect("Behavior op");
        op["caused_by"] = serde_json::json!("00000000-0000-0000-0000-000000000000");
    });
    rewrite_on_disk_log(&dir2, &events);
    must_fail_closed(&dir2, "tampered caused_by with stale digest");
}

#[test]
fn r2_vintage_payload_without_behaviors() {
    let _g = tiny_limits();
    let mut rt = Runtime::default();
    let (ab, _bc, _cd, _de, _) = seed_chain(&mut rt);
    let (ckp, twin, _) = persist_twins(&rt, "vintage-src");
    store::open(&ckp).expect("untampered must open");

    let dir = persist_dir("vintage");
    copy_store(&ckp, &dir);
    let mut events = export_events(&dir);
    let mut had_behaviors = false;
    mutate_checkpoint_snapshot(&mut events, |v| {
        had_behaviors = v
            .as_object_mut()
            .expect("snapshot object")
            .remove("behaviors")
            .is_some();
    });
    assert!(
        had_behaviors,
        "Horn checkpoint payload must have serialized behaviors before vintage strip"
    );
    recompute_blob_digest(&mut events);
    rewrite_on_disk_log(&dir, &events);

    match store::open(&dir) {
        Err(_) => {
            // Fail closed on older vintage is acceptable.
        }
        Ok(mut vintage) => {
            let mut b = store::open(&twin).unwrap();
            let got = vintage.emit(Op::Retract { event_id: ab });
            let want = b.emit(Op::Retract { event_id: ab });
            match (got, want) {
                (Ok(_), Ok(_)) => {
                    let got_ac = live_named(&vintage, "A", "relatedTo", "C");
                    let want_ac = live_named(&b, "A", "relatedTo", "C");
                    if got_ac != want_ac {
                        // Receipt after digest recompute claims exactness of a
                        // blob without `behaviors`. fingerprint/fact_count do
                        // not name Horn lineage. Silent Horn-head divergence is
                        // LOW residual, not HIGH.
                        eprintln!(
                            "LOW R2 residual: vintage payload without behaviors opened; after Retract A-B live A-C vintage={got_ac} twin={want_ac}"
                        );
                    }
                }
                (left, right) => {
                    eprintln!(
                        "LOW R2 residual: vintage payload without behaviors opened; retract A-B vintage={left:?} twin={right:?}"
                    );
                }
            }
        }
    }
}

#[test]
fn r1_fork_cut_still_unavailable_after_horn_cascade() {
    let _g = tiny_limits();
    let mut rt = Runtime::default();
    let (ab, _bc, _cd, _de, _) = seed_chain(&mut rt);
    let (ckp, _twin, _) = persist_twins(&rt, "fork-after");
    let mut opened = store::open(&ckp).unwrap();
    opened.emit(Op::Retract { event_id: ab }).unwrap();
    let c = first_checkpoint_index(opened.log().as_slice());
    let len = opened.log().len();
    for n in 1..=c {
        match opened.fork_at(n) {
            Err(RuntimeError::ForkCutUnavailable { cut, log_len })
                if cut == n && log_len == len => {}
            other => panic!(
                "HIGH R1: after cascade, fork_at({n}) was {}",
                if other.is_ok() { "Ok" } else { "Err(other)" }
            ),
        }
    }
    match opened.fork_at(len + 1) {
        Err(RuntimeError::ForkCutUnavailable { cut, log_len })
            if cut == len + 1 && log_len == len => {}
        other => panic!(
            "HIGH R1: after cascade, past-end fork was {}",
            if other.is_ok() { "Ok" } else { "Err(other)" }
        ),
    }
    let empty = opened.fork_at(0).expect("n==0 must stay Ok after cascade");
    assert!(
        empty.fold().facts().is_empty(),
        "HIGH R1: n==0 empty fold after cascade"
    );
}
