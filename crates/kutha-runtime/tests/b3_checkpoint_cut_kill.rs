//! Independent kill tests for phase 229 (checkpoint is a cut).
//! Public API only. Each case is capable of failing a lazy implementation.
//!
//! Twin normalisation: copies share intern order, so TermIds match. Fingerprint
//! hashes fact `event_id`s — compare it after Retract/Correct of prefix facts
//! (same support slots). After a newly minted Assert / Behavior / rule row,
//! compare named live product triples, `as_of`, and `as_known_on` only.

use kutha_common::{Event, EventId, Op, RuleAst, RuleAtom, RuleTerm, ValidTime, WallTime};
use kutha_runtime::{store, Runtime, RuntimeError};
use std::collections::BTreeMap;
use std::fs;
use std::io::ErrorKind;
use std::path::Path;
use std::process::Command;

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
    common::PersistScratch::new(&format!("b3-kill-{label}"))
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

fn horn_inverse() -> RuleAst {
    let v = |i| RuleTerm::var(i).unwrap();
    let atom = |rel: &str, s, o| RuleAtom::new(rel, s, o).unwrap();
    RuleAst::horn(
        atom("relatedTo", v(1), v(0)),
        vec![atom("relatedTo", v(0), v(1))],
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
    let id = rt
        .log()
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
        });
    assert!(
        rt.fold().facts().iter().any(|f| f.event_id == id),
        "HIGH: Assert {key} id={id} not in fold after emit"
    );
    id
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

fn assert_twin_answers(got: &Runtime, want: &Runtime, msg: &str) {
    assert_twin_named(got, want, msg);
    assert_fp_if_same_ids(got, want, msg);
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
    for wall in [2000u64, 2015, 2018, 2021, 2030] {
        assert_eq!(
            named_known_on(got, wall),
            named_known_on(want, wall),
            "HIGH {msg}: as_known_on {wall}"
        );
    }
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

fn payload_tree(dir: &Path) -> BTreeMap<String, Vec<u8>> {
    let mut out = BTreeMap::new();
    walk_payload(dir, dir, &mut out);
    out
}

fn walk_payload(root: &Path, dir: &Path, out: &mut BTreeMap<String, Vec<u8>>) {
    for ent in fs::read_dir(dir).unwrap() {
        let ent = ent.unwrap();
        let name = ent.file_name();
        if name == "kutha.lock" {
            continue;
        }
        let path = ent.path();
        let rel = path
            .strip_prefix(root)
            .unwrap()
            .to_string_lossy()
            .into_owned();
        if path.is_dir() {
            walk_payload(root, &path, out);
        } else {
            out.insert(rel, fs::read(&path).unwrap());
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

fn fork_cut_err(rt: &Runtime, n: usize) -> RuntimeError {
    match rt.fork_at(n) {
        Err(e) => e,
        Ok(_) => {
            panic!("HIGH R1: fork_at({n}) was Ok (interior/past-end must be ForkCutUnavailable)")
        }
    }
}

fn seed_trans_ab_bc(rt: &mut Runtime) -> (EventId, EventId) {
    rt.emit_rule_ast(&horn_trans(), 2010, None).unwrap();
    let ab = emit_keyed(rt, "A", "relatedTo", "B", 2010, None, "horn-ab");
    let bc = emit_keyed(rt, "B", "relatedTo", "C", 2010, None, "horn-bc");
    pad(rt, 8, "pad");
    rt.emit_tt_anchor(2015).unwrap();
    assert!(
        live_named(rt, "A", "relatedTo", "C"),
        "fixture must derive A-C"
    );
    (ab, bc)
}

#[test]
fn r1_fork_cut_bounds_and_no_interior_reconstruction() {
    let _g = tiny_limits();
    let mut rt = Runtime::default();
    emit_keyed(
        &mut rt,
        "art-12-speed",
        "inForceAs",
        "50-kmh",
        2010,
        Some(2020),
        "statute-50",
    );
    pad(&mut rt, 8, "r1");
    rt.emit_tt_anchor(2015).unwrap();
    let (ckp, twin, _prefix_len) = persist_twins(&rt, "r1");

    let opened = store::open(&ckp).unwrap();
    let events = opened.log().as_slice();
    let c = first_checkpoint_index(events);
    let len = opened.log().len();

    let empty = opened.fork_at(0).expect("n==0 must be Ok");
    assert_eq!(
        empty.fold().facts().len(),
        0,
        "HIGH R1: n==0 must be an empty fold"
    );
    assert_eq!(empty.log().len(), 0);

    for n in 1..=c {
        let err = fork_cut_err(&opened, n);
        assert!(
            matches!(
                err,
                RuntimeError::ForkCutUnavailable { cut, log_len } if cut == n && log_len == len
            ),
            "HIGH R1: interior n={n} got {err:?}"
        );
        assert_eq!(
            err.to_string(),
            format!("fork cut {n} unavailable (log_len {len})")
        );
    }

    let past = fork_cut_err(&opened, len + 1);
    assert!(
        matches!(
            past,
            RuntimeError::ForkCutUnavailable { cut, log_len } if cut == len + 1 && log_len == len
        ),
        "HIGH R1: past-end got {past:?}"
    );

    let twin_rt = store::open(&twin).unwrap();
    let twin_prefix = twin_prefix_len_for_checkpointed(&opened, &twin_rt);
    let tail = len - (c + 1);

    for i in 0..=tail {
        let fork = opened
            .fork_at(c + 1 + i)
            .unwrap_or_else(|e| panic!("HIGH R1: legal fork_at(c+1+{i}) Err {e}"));
        let twin_fork = twin_rt
            .fork_at(twin_prefix + i)
            .unwrap_or_else(|e| panic!("HIGH R1: twin fork_at(twin_prefix+{i}) Err {e}"));
        replay_ok(&fork, &format!("R1 fork c+1+{i}"));
        replay_ok(&twin_fork, &format!("R1 twin prefix+{i}"));
        assert_twin_answers(
            &fork,
            &twin_fork,
            &format!("R1 fork_at compacted-prefix+{i} vs twin prefix+{i}"),
        );
    }

    let mut no_ckp = Runtime::default();
    emit_keyed(
        &mut no_ckp,
        "art-12-speed",
        "inForceAs",
        "50-kmh",
        2010,
        None,
        "plain-50",
    );
    pad(&mut no_ckp, 3, "plain");
    assert!(!no_ckp
        .log()
        .as_slice()
        .iter()
        .any(|e| matches!(e.op, Op::Checkpoint { .. })));
    let nlen = no_ckp.log().len();
    no_ckp
        .fork_at(1)
        .expect("HIGH R1: interior fork_at without Checkpoint must stay Ok");
    no_ckp
        .fork_at(nlen)
        .expect("HIGH R1: fork_at(len) without Checkpoint must stay Ok");
    let err = fork_cut_err(&no_ckp, nlen + 1);
    assert!(
        matches!(
            err,
            RuntimeError::ForkCutUnavailable { cut, log_len } if cut == nlen + 1 && log_len == nlen
        ),
        "HIGH R1: past-end without checkpoint {err:?}"
    );
}

#[test]
fn r2_retract_prefix_premise_via_open_and_open_for_append() {
    let _g = tiny_limits();
    let mut rt = Runtime::default();
    let (ab, _) = seed_trans_ab_bc(&mut rt);
    let encoded = store::encoded_events(&rt);

    let (ckp, twin, prefix_len) = persist_twins(&rt, "r2-open");
    assert!(
        encoded[..prefix_len].iter().any(|e| e.id == ab),
        "A-B must sit in the sealed prefix"
    );

    let mut a = store::open(&ckp).unwrap();
    let mut b = store::open(&twin).unwrap();
    replay_ok(&a, "R2 open after checkpoint");
    a.emit(Op::Retract { event_id: ab })
        .expect("Retract prefix A-B on checkpointed");
    b.emit(Op::Retract { event_id: ab })
        .expect("Retract prefix A-B on twin");
    replay_ok(&a, "R2 after Retract (open)");
    replay_ok(&b, "R2 twin after Retract (open)");
    assert_twin_answers(&a, &b, "R2 Runtime::open retract prefix Horn premise");
    assert!(
        !live_named(&a, "A", "relatedTo", "C"),
        "HIGH R2: A-C must un-derive"
    );

    let (ckp2, twin2, _) = persist_twins(&rt, "r2-append");
    let (mut wa, mut pa) = store::open_for_append(&ckp2).expect("open_for_append checkpointed");
    let (mut wb, mut pb) = store::open_for_append(&twin2).expect("open_for_append twin");
    wa.emit(Op::Retract { event_id: ab }).unwrap();
    wb.emit(Op::Retract { event_id: ab }).unwrap();
    replay_ok(&wa, "R2 after Retract (append)");
    assert_twin_answers(&wa, &wb, "R2 Persister open_for_append retract");
    pa.append(&wa).unwrap();
    pb.append(&wb).unwrap();
    drop(pa);
    drop(pb);
    let a2 = store::open(&ckp2).unwrap();
    let b2 = store::open(&twin2).unwrap();
    replay_ok(&a2, "R2 reopen after append retract");
    assert_twin_answers(&a2, &b2, "R2 reopen after open_for_append retract");
}

#[test]
fn r2_two_rules_two_premises_chain_and_no_live_premise() {
    let _g = tiny_limits();
    let mut rt = Runtime::default();
    rt.emit_rule_ast(&horn_trans(), 2010, None).unwrap();
    rt.emit_rule_ast(&horn_inverse(), 2010, None).unwrap();
    let ab = emit_keyed(&mut rt, "A", "relatedTo", "B", 2010, None, "horn-ab");
    let _bc = emit_keyed(&mut rt, "B", "relatedTo", "C", 2010, None, "horn-bc");
    pad(&mut rt, 8, "multi");
    rt.emit_tt_anchor(2015).unwrap();
    assert!(live_named(&rt, "A", "relatedTo", "C"));
    assert!(live_named(&rt, "B", "relatedTo", "A"), "inverse A-B → B-A");
    assert!(live_named(&rt, "C", "relatedTo", "B"), "inverse B-C → C-B");

    let (ckp, twin, _) = persist_twins(&rt, "r2-multi");
    let mut a = store::open(&ckp).unwrap();
    let mut b = store::open(&twin).unwrap();
    replay_ok(&a, "R2 multi open");
    assert_twin_named(&a, &b, "R2 two-rule/chain after checkpoint before retract");

    a.emit(Op::Retract { event_id: ab }).unwrap();
    b.emit(Op::Retract { event_id: ab }).unwrap();
    replay_ok(&a, "R2 after retract first premise");
    assert!(
        live_named(&b, "C", "relatedTo", "B"),
        "twin keeps inverse of still-live B-C"
    );
    assert!(
        live_named(&a, "C", "relatedTo", "B"),
        "HIGH R2: retracting prefix A-B dropped inverse C-B of still-live B-C"
    );
    assert_twin_named(&a, &b, "R2 two-rule/chain after retract A-B");
}

#[test]
fn r2_chain_two_hops_retract_first_premise() {
    let _g = tiny_limits();
    let mut rt = Runtime::default();
    rt.emit_rule_ast(&horn_trans(), 2010, None).unwrap();
    let ab = emit_keyed(&mut rt, "A", "relatedTo", "B", 2010, None, "horn-ab");
    let _bc = emit_keyed(&mut rt, "B", "relatedTo", "C", 2010, None, "horn-bc");
    let _cd = emit_keyed(&mut rt, "C", "relatedTo", "D", 2010, None, "horn-cd");
    pad(&mut rt, 8, "chain");
    rt.emit_tt_anchor(2015).unwrap();
    assert!(live_named(&rt, "A", "relatedTo", "C"));
    assert!(live_named(&rt, "A", "relatedTo", "D"));
    assert!(live_named(&rt, "B", "relatedTo", "D"));
    let (ckp, twin, _) = persist_twins(&rt, "r2-chain");
    let mut a = store::open(&ckp).unwrap();
    let mut b = store::open(&twin).unwrap();
    a.emit(Op::Retract { event_id: ab }).unwrap();
    b.emit(Op::Retract { event_id: ab }).unwrap();
    replay_ok(&a, "R2 chain retract");
    assert!(
        !live_named(&b, "A", "relatedTo", "D"),
        "twin must un-derive second-hop A-D"
    );
    assert!(
        !live_named(&a, "A", "relatedTo", "D"),
        "HIGH R2: chain A-D stayed live after retracting prefix A-B"
    );
    assert_twin_answers(&a, &b, "R2 chain of two Horn steps after retract A-B");
}

#[test]
fn r2_tail_premise_with_prefix_rule() {
    let _g = tiny_limits();
    let mut rt = Runtime::default();
    rt.emit_rule_ast(&horn_trans(), 2010, None).unwrap();
    pad(&mut rt, 8, "tailp");
    rt.emit_tt_anchor(2015).unwrap();
    let (ckp, twin, prefix_len) = persist_twins(&rt, "r2-tail");
    let encoded = store::encoded_events(&rt);
    assert!(
        encoded[..prefix_len]
            .iter()
            .any(|e| matches!(e.op, Op::StoreBlob { .. })),
        "rule StoreBlob must sit in the sealed prefix"
    );

    let mut a = store::open(&ckp).unwrap();
    let mut b = store::open(&twin).unwrap();
    let ab = emit_keyed(&mut a, "A", "relatedTo", "B", 2010, None, "tail-ab");
    let ab_b = emit_keyed(&mut b, "A", "relatedTo", "B", 2010, None, "tail-ab");
    let _ = emit_keyed(&mut a, "B", "relatedTo", "C", 2010, None, "tail-bc");
    let _ = emit_keyed(&mut b, "B", "relatedTo", "C", 2010, None, "tail-bc");
    replay_ok(&a, "R2 tail premises derived");
    // New Asserts mint distinct EventIds; compare named answers only.
    assert_twin_named(&a, &b, "R2 tail premises (named)");
    assert!(
        live_named(&a, "A", "relatedTo", "C"),
        "HIGH R2: prefix rule must fire on tail premises"
    );

    a.emit(Op::Retract { event_id: ab }).unwrap_or_else(|e| {
        let rows: Vec<_> = a
            .fold()
            .facts()
            .iter()
            .filter(|f| f.event_id == ab)
            .map(|f| (f.invalidated_at, name(&a, f.subject), name(&a, f.object())))
            .collect();
        panic!("HIGH R2: Retract tail A-B {ab}: {e}; fold_rows={rows:?}")
    });
    b.emit(Op::Retract { event_id: ab_b }).unwrap();
    replay_ok(&a, "R2 retract tail premise");
    assert_twin_named(&a, &b, "R2 retract tail premise (named)");
    assert!(!live_named(&a, "A", "relatedTo", "C"));
}

#[test]
fn r2_correct_correct_interval_reassert_and_direct_head() {
    let _g = tiny_limits();
    let mut rt = Runtime::default();
    rt.emit_rule_ast(&horn_trans(), 2010, None).unwrap();
    let ab = emit_keyed(&mut rt, "A", "relatedTo", "B", 2010, Some(2020), "horn-ab");
    let _bc = emit_keyed(&mut rt, "B", "relatedTo", "C", 2010, Some(2020), "horn-bc");
    let _direct = emit_keyed(
        &mut rt,
        "A",
        "relatedTo",
        "C",
        2010,
        Some(2020),
        "direct-ac",
    );
    pad(&mut rt, 8, "corr");
    rt.emit_tt_anchor(2015).unwrap();
    assert_eq!(
        live_count(&rt, "A", "relatedTo", "C"),
        2,
        "derived + direct A-C"
    );

    let (ckp, twin, _) = persist_twins(&rt, "r2-corr");
    let mut a = store::open(&ckp).unwrap();
    let mut b = store::open(&twin).unwrap();
    a.emit(Op::Retract { event_id: ab }).unwrap();
    b.emit(Op::Retract { event_id: ab }).unwrap();
    replay_ok(&a, "R2 retract while direct head remains");
    assert_twin_answers(&a, &b, "R2 head also asserted directly");
    assert_eq!(
        live_count(&a, "A", "relatedTo", "C"),
        1,
        "HIGH R2: direct A-C must remain after un-derive"
    );

    let mut rt2 = Runtime::default();
    rt2.emit_rule_ast(&horn_trans(), 2010, None).unwrap();
    let ab2 = emit_keyed(&mut rt2, "A", "relatedTo", "B", 2010, Some(2020), "horn-ab");
    let _ = emit_keyed(&mut rt2, "B", "relatedTo", "C", 2010, Some(2020), "horn-bc");
    pad(&mut rt2, 8, "corr2");
    rt2.emit_tt_anchor(2015).unwrap();
    let (ckp2, twin2, _) = persist_twins(&rt2, "r2-correct");
    let mut a = store::open(&ckp2).unwrap();
    let mut b = store::open(&twin2).unwrap();
    let z = a.intern("Z");
    let z_b = b.intern("Z");
    assert_eq!(z, z_b);
    a.emit(Op::Correct {
        event_id: ab2,
        object: z,
        valid_from: 2010,
        valid_to: Some(2020),
    })
    .unwrap();
    b.emit(Op::Correct {
        event_id: ab2,
        object: z_b,
        valid_from: 2010,
        valid_to: Some(2020),
    })
    .unwrap();
    replay_ok(&a, "R2 Correct prefix premise");
    // Correct mints a new support EventId on both sides independently.
    assert_twin_named(&a, &b, "R2 Correct of prefix A-B");
    assert!(
        !live_named(&a, "A", "relatedTo", "C"),
        "HIGH R2: Correct off B must un-derive A-C"
    );
    assert!(live_named(&a, "A", "relatedTo", "Z"));

    let mut rt3 = Runtime::default();
    rt3.emit_rule_ast(&horn_trans(), 2010, None).unwrap();
    let ab3 = emit_keyed(&mut rt3, "A", "relatedTo", "B", 2010, Some(2020), "horn-ab");
    let _ = emit_keyed(&mut rt3, "B", "relatedTo", "C", 2010, Some(2020), "horn-bc");
    pad(&mut rt3, 8, "ci");
    rt3.emit_tt_anchor(2015).unwrap();
    let (ckp3, twin3, _) = persist_twins(&rt3, "r2-ci");
    let mut a = store::open(&ckp3).unwrap();
    let mut b = store::open(&twin3).unwrap();
    let obj = a.intern("B");
    a.emit(Op::CorrectInterval {
        event_id: ab3,
        object: obj,
        patch_from: 2012,
        patch_to: Some(2018),
    })
    .unwrap();
    let obj_b = b.intern("B");
    b.emit(Op::CorrectInterval {
        event_id: ab3,
        object: obj_b,
        patch_from: 2012,
        patch_to: Some(2018),
    })
    .unwrap();
    replay_ok(&a, "R2 CorrectInterval prefix premise");
    // Leftover halves may mint new EventIds; named answers only.
    assert_twin_named(&a, &b, "R2 CorrectInterval of prefix A-B");

    let mut rt4 = Runtime::default();
    rt4.emit_rule_ast(&horn_trans(), 2010, None).unwrap();
    let ab4 = emit_keyed(&mut rt4, "A", "relatedTo", "B", 2010, None, "horn-ab");
    let _ = emit_keyed(&mut rt4, "B", "relatedTo", "C", 2010, None, "horn-bc");
    pad(&mut rt4, 8, "re");
    rt4.emit_tt_anchor(2015).unwrap();
    let (ckp4, twin4, _) = persist_twins(&rt4, "r2-reassert");
    let mut a = store::open(&ckp4).unwrap();
    let mut b = store::open(&twin4).unwrap();
    a.emit(Op::Retract { event_id: ab4 }).unwrap();
    b.emit(Op::Retract { event_id: ab4 }).unwrap();
    assert!(!live_named(&a, "A", "relatedTo", "C"));
    emit_keyed(&mut a, "A", "relatedTo", "B", 2010, None, "horn-ab-2");
    emit_keyed(&mut b, "A", "relatedTo", "B", 2010, None, "horn-ab-2");
    replay_ok(&a, "R2 re-assert prefix premise");
    assert_twin_named(&a, &b, "R2 retract then re-assert");
    assert!(
        live_named(&a, "A", "relatedTo", "C"),
        "HIGH R2: re-assert must re-derive A-C"
    );
}

#[test]
fn r2_pin_window_reregister_double_checkpoint_reopen() {
    let _g = tiny_limits();
    let mut rt = Runtime::default();
    rt.emit_policy("leased-policy", 2010, Some(2020)).unwrap();
    rt.emit_rule_ast(&horn_knows(), 2015, Some(2020)).unwrap();
    let kn = emit_keyed(&mut rt, "Alice", "knows", "Bob", 2010, None, "knows-ab");
    pad(&mut rt, 8, "pin");
    rt.emit_tt_anchor(2015).unwrap();
    assert!(
        live_named(&rt, "Alice", "relatedTo", "Bob"),
        "windowed pin must derive on overlap"
    );
    assert!(
        !rt.fold().facts().iter().any(|f| {
            name(&rt, f.subject) == "Alice"
                && name(&rt, f.relation) == "relatedTo"
                && name(&rt, f.object()) == "Bob"
                && f.is_live_at(u64::MAX, 2012)
        }),
        "head absent before pin start"
    );

    let (ckp, twin, _) = persist_twins(&rt, "r2-pin");
    let mut a = store::open(&ckp).unwrap();
    let mut b = store::open(&twin).unwrap();
    a.emit(Op::Retract { event_id: kn }).unwrap();
    b.emit(Op::Retract { event_id: kn }).unwrap();
    replay_ok(&a, "R2 pin-window retract");
    assert_twin_answers(&a, &b, "R2 rule pin + PinPolicy window");
    assert!(!live_named(&a, "Alice", "relatedTo", "Bob"));

    let mut rt2 = Runtime::default();
    let (ab, _) = seed_trans_ab_bc(&mut rt2);
    let (ckp2, twin2, _) = persist_twins(&rt2, "r2-rereg");
    let mut a = store::open(&ckp2).unwrap();
    let mut b = store::open(&twin2).unwrap();
    a.emit_rule_ast(&horn_inverse(), 2010, None)
        .expect("re-register inverse after checkpoint");
    b.emit_rule_ast(&horn_inverse(), 2010, None).unwrap();
    let _ = emit_keyed(&mut a, "E", "relatedTo", "F", 2010, None, "tail-ef");
    let _ = emit_keyed(&mut b, "E", "relatedTo", "F", 2010, None, "tail-ef");
    replay_ok(&a, "R2 after re-register");
    assert_twin_named(&a, &b, "R2 re-register inverse (named)");
    assert!(
        live_named(&a, "F", "relatedTo", "E"),
        "HIGH R2: re-registered inverse must fire on a tail premise"
    );
    a.emit(Op::Retract { event_id: ab }).unwrap();
    b.emit(Op::Retract { event_id: ab }).unwrap();
    replay_ok(&a, "R2 retract after re-register");
    assert_twin_named(&a, &b, "R2 retract after re-register");
    assert!(!live_named(&a, "A", "relatedTo", "C"));
    assert!(live_named(&a, "F", "relatedTo", "E"));

    let mut rt3 = Runtime::default();
    let (ab3, bc3) = seed_trans_ab_bc(&mut rt3);
    let (ckp3, twin3, _) = persist_twins(&rt3, "r2-twice");
    let mut extra = store::open(&twin3).unwrap();
    let n = extra.log().len();
    pad(&mut extra, 8, "twice-tail");
    let suffix = extra.log().as_slice()[n..].to_vec();
    let dict = extra.dictionary().strings().to_vec();
    store::persist(&extra, &twin3).unwrap();
    apply_suffix(&ckp3, &suffix, dict);
    store::checkpoint_sealed_prefix(&ckp3).unwrap();
    let mut a = store::open(&ckp3).unwrap();
    let mut b = store::open(&twin3).unwrap();
    a.emit(Op::Retract { event_id: ab3 }).unwrap();
    b.emit(Op::Retract { event_id: ab3 }).unwrap();
    replay_ok(&a, "R2 after checkpoint twice");
    assert_twin_answers(&a, &b, "R2 checkpoint twice then retract");

    store::persist(&a, &ckp3).unwrap();
    store::persist(&b, &twin3).unwrap();
    let mut a = store::open(&ckp3).unwrap();
    let mut b = store::open(&twin3).unwrap();
    replay_ok(&a, "R2 persist+reopen");
    a.emit(Op::Retract { event_id: bc3 }).unwrap();
    b.emit(Op::Retract { event_id: bc3 }).unwrap();
    replay_ok(&a, "R2 retract again after reopen");
    assert_twin_answers(&a, &b, "R2 persist+reopen+retract again");
}

#[test]
fn r2_no_horn_checkpoint_then_retract_matches_twin() {
    let _g = tiny_limits();
    let mut rt = Runtime::default();
    let statute = emit_keyed(
        &mut rt,
        "art-12-speed",
        "inForceAs",
        "50-kmh",
        2010,
        Some(2020),
        "statute-50",
    );
    pad(&mut rt, 8, "nohorn");
    rt.emit_tt_anchor(2015).unwrap();
    let (ckp, twin, _) = persist_twins(&rt, "r2-nohorn");
    let before_a = store::open(&ckp).unwrap();
    let before_b = store::open(&twin).unwrap();
    assert_twin_answers(&before_a, &before_b, "R2 no-Horn after checkpoint");
    drop(before_a);
    drop(before_b);

    let mut a = store::open(&ckp).unwrap();
    let mut b = store::open(&twin).unwrap();
    a.emit(Op::Retract { event_id: statute }).unwrap();
    b.emit(Op::Retract { event_id: statute }).unwrap();
    replay_ok(&a, "R2 no-Horn retract");
    assert_twin_answers(&a, &b, "R2 no-Horn retract after checkpoint");
}

fn migrate_english(err: &std::io::Error) {
    assert_eq!(err.kind(), ErrorKind::InvalidData);
    let msg = err.to_string();
    assert!(
        msg.contains("Op::Checkpoint") && msg.contains("unchanged"),
        "HIGH R3: English refuse message, got {msg:?}"
    );
}

#[test]
fn r3_migrate_compat_op_wire_and_cli_refuse_byte_identical() {
    let _g = tiny_limits();
    let mut rt = Runtime::default();
    emit_keyed(
        &mut rt,
        "art-12-speed",
        "inForceAs",
        "50-kmh",
        2010,
        Some(2020),
        "statute-50",
    );
    pad(&mut rt, 6, "mig");
    rt.emit_tt_anchor(2015).unwrap();

    let dir = persist_dir("r3-compat");
    store::persist(&rt, &dir).unwrap();
    store::checkpoint_sealed_prefix(&dir).unwrap();
    let before = payload_tree(&dir);
    let err = store::migrate_compat(&dir).expect_err("HIGH R3: migrate_compat must Err");
    migrate_english(&err);
    assert_eq!(
        payload_tree(&dir),
        before,
        "HIGH R3: migrate_compat must leave listing+bytes unchanged"
    );

    let dir2 = persist_dir("r3-wire");
    store::persist(&rt, &dir2).unwrap();
    store::checkpoint_sealed_prefix(&dir2).unwrap();
    let before2 = payload_tree(&dir2);
    let err = store::migrate_op_wire(&dir2).expect_err("HIGH R3: migrate_op_wire must Err");
    migrate_english(&err);
    assert_eq!(
        payload_tree(&dir2),
        before2,
        "HIGH R3: migrate_op_wire must leave listing+bytes unchanged"
    );

    let cli = persist_dir("r3-cli");
    store::persist(&rt, &cli).unwrap();
    store::checkpoint_sealed_prefix(&cli).unwrap();
    let before_cli = payload_tree(&cli);
    let out = Command::new(env!("CARGO_BIN_EXE_kutha-migrate"))
        .arg(&*cli)
        .output()
        .expect("spawn kutha-migrate");
    assert!(
        !out.status.success(),
        "HIGH R3: kutha-migrate must not succeed on a checkpointed store"
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("Op::Checkpoint") && stderr.contains("unchanged"),
        "HIGH R3: kutha-migrate stderr must be English refuse, got {stderr:?}"
    );
    assert_eq!(
        payload_tree(&cli),
        before_cli,
        "HIGH R3: kutha-migrate must leave listing+bytes unchanged"
    );
}

#[test]
fn r3_non_checkpointed_and_legacy_still_migrate() {
    let _g = tiny_limits();
    let mut rt = Runtime::default();
    emit_keyed(
        &mut rt,
        "art-12-speed",
        "inForceAs",
        "50-kmh",
        2010,
        Some(2020),
        "statute-50",
    );
    pad(&mut rt, 6, "okmig");
    let dir = persist_dir("r3-plain");
    store::persist(&rt, &dir).unwrap();
    assert!(!store::open(&dir)
        .unwrap()
        .log()
        .as_slice()
        .iter()
        .any(|e| matches!(e.op, Op::Checkpoint { .. })));
    store::migrate_compat(&dir).expect("HIGH R3: non-checkpointed migrate_compat must stay Ok");

    let legacy = persist_dir("r3-legacy");
    store::ensure_legacy_dir(&legacy).unwrap();
    let genesis = Runtime::default();
    store::persist(&genesis, &legacy).unwrap();
    let id = EventId::nil();
    let line = format!(
        r#"{{"id":"{id}","op":{{"AllowRelation":{{"name":"extraRel","valid_from":0,"valid_to":null}}}},"ingested_at":999,"object_ids":[]}}"#
    );
    use std::io::Write;
    {
        let mut f = fs::OpenOptions::new()
            .append(true)
            .open(legacy.join("events.jsonl"))
            .unwrap();
        writeln!(f, "{line}").unwrap();
    }
    store::migrate_op_wire(&legacy)
        .expect("HIGH R3: legacy AllowRelation migrate_op_wire must stay Ok");
}

#[test]
fn r4_checkpoint_has_no_serde_alias() {
    let raw = r#"{"CheckpointCut":{"first_event_id":"00000000-0000-0000-0000-000000000000","last_event_id":"00000000-0000-0000-0000-000000000000","dropped_count":0,"fact_count":0,"fingerprint_hex":"","blob_digest":""}}"#;
    assert!(
        serde_json::from_str::<Op>(raw).is_err(),
        "HIGH R4: serde alias CheckpointCut must not deserialize"
    );
    let ok = r#"{"Checkpoint":{"first_event_id":"00000000-0000-0000-0000-000000000000","last_event_id":"00000000-0000-0000-0000-000000000000","dropped_count":0,"fact_count":0,"fingerprint_hex":"","blob_digest":""}}"#;
    serde_json::from_str::<Op>(ok).expect("Op::Checkpoint must still deserialize");
}
