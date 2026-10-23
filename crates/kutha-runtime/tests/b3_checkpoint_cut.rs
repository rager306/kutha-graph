//! Phase 229 / M155 S01 — checkpoint is a cut, not a hole (b3).
//! `fork_at` is `Result<Self, RuntimeError::ForkCutUnavailable>`. Horn and
//! migrate tests stay RED for plan 02.

use kutha_common::{
    Event, EventId, KUTHA_ALLOWS, KUTHA_POLICY, KUTHA_RULE, Op, RuleAst, RuleAtom, RuleTerm,
    ValidTime,
};
use kutha_runtime::{Runtime, RuntimeError, store};
use std::collections::BTreeMap;
use std::fs;
use std::io::ErrorKind;
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
    common::PersistScratch::new(&format!("b3-cut-{label}"))
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
    rt.emit(Op::Assert {
        subject,
        relation,
        object,
        valid_from,
        valid_to,
        claim: None,
        delivery_key: Some(key.into()),
        polarity: None,
    })
    .unwrap()
    .receipt
    .event_ids[0]
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

fn horn_related() -> RuleAst {
    let v = |i| RuleTerm::var(i).unwrap();
    let atom = |rel: &str, s, o| RuleAtom::new(rel, s, o).unwrap();
    RuleAst::horn(
        atom("relatedTo", v(0), v(2)),
        vec![atom("relatedTo", v(0), v(1)), atom("relatedTo", v(1), v(2))],
    )
    .unwrap()
}

fn live_triple(rt: &Runtime, s: &str, r: &str, o: &str) -> bool {
    let (Some(s), Some(r), Some(o)) = (
        rt.dictionary().id(s),
        rt.dictionary().id(r),
        rt.dictionary().id(o),
    ) else {
        return false;
    };
    rt.fold()
        .facts()
        .iter()
        .any(|f| f.subject == s && f.relation == r && f.object() == o && f.invalidated_at.is_none())
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

fn payload_listing(dir: &Path) -> BTreeMap<String, Vec<u8>> {
    let mut out = BTreeMap::new();
    for ent in fs::read_dir(dir).unwrap() {
        let ent = ent.unwrap();
        let name = ent.file_name();
        if name == "kutha.lock" {
            continue;
        }
        let path = ent.path();
        if path.is_file() {
            out.insert(
                name.to_string_lossy().into_owned(),
                fs::read(&path).unwrap(),
            );
        }
    }
    out
}

fn first_checkpoint_index(events: &[Event]) -> usize {
    events
        .iter()
        .position(|e| matches!(e.op, Op::Checkpoint { .. }))
        .expect("Op::Checkpoint on current log")
}

fn receipt_fact_count(op: &Op) -> usize {
    match op {
        Op::Checkpoint { fact_count, .. } => *fact_count as usize,
        _ => panic!("expected Op::Checkpoint"),
    }
}

#[test]
fn fork_at_interior_of_checkpointed_prefix_is_err() {
    let _g = tiny_limits();
    let dir = persist_dir("interior");
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
    pad(&mut rt, 6, "interior");
    rt.emit_tt_anchor(2015).unwrap();
    store::persist(&rt, &dir).unwrap();
    store::checkpoint_sealed_prefix(&dir).unwrap();
    let opened = store::open(&dir).unwrap();
    let events = opened.log().as_slice();
    let c = first_checkpoint_index(events);
    let fact_count = receipt_fact_count(&events[c].op);
    let len = opened.log().len();

    let empty = opened.fork_at(0).unwrap();
    assert_eq!(empty.fold().facts().len(), 0, "n==0 empty fold");
    assert_eq!(empty.log().len(), 0);

    for n in 1..=c {
        let err = match opened.fork_at(n) {
            Err(e) => e,
            Ok(_) => panic!("interior cut must be ForkCutUnavailable"),
        };
        assert!(
            matches!(
                err,
                RuntimeError::ForkCutUnavailable { cut, log_len } if cut == n && log_len == len
            ),
            "{err:?}"
        );
        assert_eq!(
            err.to_string(),
            format!("fork cut {n} unavailable (log_len {len})")
        );
    }

    let past = match opened.fork_at(len + 1) {
        Err(e) => e,
        Ok(_) => panic!("past end must be ForkCutUnavailable"),
    };
    assert!(
        matches!(
            past,
            RuntimeError::ForkCutUnavailable { cut, log_len } if cut == len + 1 && log_len == len
        ),
        "{past:?}"
    );

    for n in (c + 1)..=len {
        let fork = opened.fork_at(n).unwrap();
        if n == c + 1 {
            assert_eq!(
                fork.fold().facts().len(),
                fact_count,
                "fork_at(c+1) facts must equal receipt fact_count"
            );
        }
    }
}

#[test]
fn fork_at_past_log_end_is_err_without_checkpoint() {
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
    assert!(
        !rt.log()
            .as_slice()
            .iter()
            .any(|e| matches!(e.op, Op::Checkpoint { .. })),
        "no receipt"
    );
    let len = rt.log().len();
    let err = match rt.fork_at(len + 1) {
        Err(e) => e,
        Ok(_) => panic!("past end without checkpoint must be ForkCutUnavailable"),
    };
    assert!(
        matches!(
            err,
            RuntimeError::ForkCutUnavailable { cut, log_len } if cut == len + 1 && log_len == len
        ),
        "{err:?}"
    );
    assert_eq!(
        err.to_string(),
        format!("fork cut {} unavailable (log_len {len})", len + 1)
    );
}

#[test]
fn retract_prefix_horn_premise_after_checkpoint_matches_twin() {
    let _g = tiny_limits();
    let mut rt = Runtime::default();
    rt.emit_rule_ast(&horn_related(), 2010, None).unwrap();
    let ab = emit_keyed(&mut rt, "A", "relatedTo", "B", 2010, None, "horn-ab");
    let _bc = emit_keyed(&mut rt, "B", "relatedTo", "C", 2010, None, "horn-bc");
    pad(&mut rt, 8, "horn");
    rt.emit_tt_anchor(2015).unwrap();
    assert!(
        live_triple(&rt, "A", "relatedTo", "C"),
        "fixture must derive relatedTo(A,C) in the prefix"
    );

    let ckp = persist_dir("horn-ckp");
    let twin = persist_dir("horn-twin");
    store::persist(&rt, &ckp).unwrap();
    let encoded = store::encoded_events(&rt);
    let prefix_len = sealed_prefix_len(&ckp);
    assert!(
        encoded[..prefix_len].iter().any(|e| e.id == ab),
        "A-B premise must sit in the sealed prefix"
    );
    copy_store(&ckp, &twin);
    store::checkpoint_sealed_prefix(&ckp).unwrap();

    let mut a = store::open(&ckp).expect("checkpointed horn store");
    let mut b = store::open(&twin).expect("twin");
    a.emit(Op::Retract { event_id: ab })
        .expect("emit Retract of prefix A-B on checkpointed runtime");
    b.emit(Op::Retract { event_id: ab })
        .expect("emit Retract of prefix A-B on twin");

    assert_eq!(
        a.fold().fingerprint(),
        b.fold().fingerprint(),
        "Horn un-derive fingerprint vs twin after live emit"
    );
    assert!(
        !live_triple(&a, "A", "relatedTo", "C"),
        "relatedTo(A,C) must un-derive after retracting prefix A-B"
    );
    assert_eq!(
        live_triple(&a, "A", "relatedTo", "C"),
        live_triple(&b, "A", "relatedTo", "C")
    );
    assert_eq!(
        a.as_of(ValidTime::from(2015)),
        b.as_of(ValidTime::from(2015)),
        "as_of 2015 after Horn un-derive"
    );
}

#[test]
fn migrate_compat_refuses_checkpointed_store_and_leaves_dir_unchanged() {
    let _g = tiny_limits();
    let dir = persist_dir("migrate");
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
    store::persist(&rt, &dir).unwrap();
    store::checkpoint_sealed_prefix(&dir).unwrap();
    let before = payload_listing(&dir);
    let err = store::migrate_compat(&dir).expect_err("migrate_compat must refuse Checkpoint");
    assert_eq!(err.kind(), ErrorKind::InvalidData);
    assert_eq!(
        payload_listing(&dir),
        before,
        "migrate_compat Err must leave payload files unchanged"
    );
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

fn live_keyed(rt: &Runtime, rel: &str, key: &str) -> usize {
    let Some(rel) = rt.dictionary().id(rel) else {
        return 0;
    };
    rt.fold()
        .facts()
        .iter()
        .filter(|f| {
            f.relation == rel
                && f.delivery_key.as_deref() == Some(key)
                && f.invalidated_at.is_none()
        })
        .count()
}

fn persist_twins(rt: &Runtime, label: &str) -> (common::PersistScratch, common::PersistScratch) {
    let ckp = persist_dir(&format!("{label}-ckp"));
    let twin = persist_dir(&format!("{label}-twin"));
    store::persist(rt, &ckp).unwrap();
    copy_store(&ckp, &twin);
    store::checkpoint_sealed_prefix(&ckp).unwrap();
    (ckp, twin)
}

/// Prefix Horn, then the same `emit_rule_ast` after the cut, then Retract and
/// re-assert of A-B. Compacted `kutha:rule` pin must retry like the twin.
#[test]
fn reregister_same_horn_after_checkpoint_matches_twin() {
    let _g = tiny_limits();
    let mut rt = Runtime::default();
    rt.emit_rule_ast(&horn_related(), 2010, None).unwrap();
    let ab = emit_keyed(&mut rt, "A", "relatedTo", "B", 2010, None, "rr-ab");
    let _bc = emit_keyed(&mut rt, "B", "relatedTo", "C", 2010, None, "rr-bc");
    pad(&mut rt, 8, "rr");
    rt.emit_tt_anchor(2015).unwrap();
    assert_eq!(live_count(&rt, "A", "relatedTo", "C"), 1);
    let (ckp, twin) = persist_twins(&rt, "rr-horn");
    let mut a = store::open(&ckp).unwrap();
    let mut b = store::open(&twin).unwrap();
    let rc = a.emit_rule_ast(&horn_related(), 2010, None).unwrap();
    let rp = b.emit_rule_ast(&horn_related(), 2010, None).unwrap();
    assert_eq!(
        rc.events_in_quantum, rp.events_in_quantum,
        "re-register kutha:rule must retry (events_in_quantum C={} P={})",
        rc.events_in_quantum, rp.events_in_quantum
    );
    assert_eq!(
        live_keyed(&a, KUTHA_RULE, &horn_related().hash()),
        live_keyed(&b, KUTHA_RULE, &horn_related().hash()),
        "live kutha:rule pins after re-register"
    );
    a.emit(Op::Retract { event_id: ab }).unwrap();
    b.emit(Op::Retract { event_id: ab }).unwrap();
    emit_keyed(&mut a, "A", "relatedTo", "B", 2010, None, "rr-ab-re");
    emit_keyed(&mut b, "A", "relatedTo", "B", 2010, None, "rr-ab-re");
    assert_eq!(
        live_count(&a, "A", "relatedTo", "C"),
        live_count(&b, "A", "relatedTo", "C"),
        "after re-assert A-B, live relatedTo(A,C) must match twin"
    );
    assert_eq!(live_count(&a, "A", "relatedTo", "C"), 1);
}

/// Prefix product Assert with a delivery_key must retry after the cut.
#[test]
fn reassert_prefix_delivery_key_after_checkpoint_is_retry() {
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
    pad(&mut rt, 6, "dk");
    rt.emit_tt_anchor(2015).unwrap();
    let (ckp, twin) = persist_twins(&rt, "rr-dk");
    let mut a = store::open(&ckp).unwrap();
    let mut b = store::open(&twin).unwrap();
    let before_c = live_keyed(&a, "inForceAs", "statute-50");
    let sa = a.intern("art-12-speed");
    let ra = a.intern("inForceAs");
    let oa = a.intern("50-kmh");
    let rc = a
        .emit(Op::Assert {
            subject: sa,
            relation: ra,
            object: oa,
            valid_from: 2010,
            valid_to: Some(2020),
            claim: None,
            delivery_key: Some("statute-50".into()),
            polarity: None,
        })
        .unwrap();
    let sb = b.intern("art-12-speed");
    let rb = b.intern("inForceAs");
    let ob = b.intern("50-kmh");
    let rp = b
        .emit(Op::Assert {
            subject: sb,
            relation: rb,
            object: ob,
            valid_from: 2010,
            valid_to: Some(2020),
            claim: None,
            delivery_key: Some("statute-50".into()),
            polarity: None,
        })
        .unwrap();
    assert_eq!(rc.events_in_quantum, 0, "C must retry prefix delivery_key");
    assert_eq!(rp.events_in_quantum, 0, "P must retry prefix delivery_key");
    assert_eq!(live_keyed(&a, "inForceAs", "statute-50"), before_c);
    assert_eq!(
        live_keyed(&a, "inForceAs", "statute-50"),
        live_keyed(&b, "inForceAs", "statute-50")
    );
}

/// Prefix `kutha:allows` pin must retry `emit_allow` after the cut.
#[test]
fn reregister_same_allow_after_checkpoint_is_retry() {
    let _g = tiny_limits();
    let mut rt = Runtime::default();
    rt.emit_allow("b3-cut-rel", 2010, None).unwrap();
    pad(&mut rt, 6, "al");
    rt.emit_tt_anchor(2015).unwrap();
    let (ckp, twin) = persist_twins(&rt, "rr-allow");
    let mut a = store::open(&ckp).unwrap();
    let mut b = store::open(&twin).unwrap();
    let rc = a.emit_allow("b3-cut-rel", 2010, None).unwrap();
    let rp = b.emit_allow("b3-cut-rel", 2010, None).unwrap();
    assert_eq!(rc.events_in_quantum, 0);
    assert_eq!(rp.events_in_quantum, 0);
    assert_eq!(
        live_keyed(&a, KUTHA_ALLOWS, "b3-cut-rel"),
        live_keyed(&b, KUTHA_ALLOWS, "b3-cut-rel")
    );
    assert_eq!(live_keyed(&a, KUTHA_ALLOWS, "b3-cut-rel"), 1);
}

/// Prefix `kutha:policy` pin must retry `emit_policy` after the cut.
#[test]
fn reregister_same_policy_after_checkpoint_is_retry() {
    let _g = tiny_limits();
    let mut rt = Runtime::default();
    rt.emit_policy("b3-cut-policy", 2010, None).unwrap();
    pad(&mut rt, 6, "pol");
    rt.emit_tt_anchor(2015).unwrap();
    let (ckp, twin) = persist_twins(&rt, "rr-pol");
    let mut a = store::open(&ckp).unwrap();
    let mut b = store::open(&twin).unwrap();
    let rc = a.emit_policy("b3-cut-policy", 2010, None).unwrap();
    let rp = b.emit_policy("b3-cut-policy", 2010, None).unwrap();
    assert_eq!(rc.events_in_quantum, 0, "C policy Assert must retry");
    assert_eq!(rp.events_in_quantum, 0, "P policy Assert must retry");
    let pin = kutha_common::policy_version_hash("b3-cut-policy");
    assert_eq!(
        live_keyed(&a, KUTHA_POLICY, &pin),
        live_keyed(&b, KUTHA_POLICY, &pin)
    );
    assert_eq!(live_keyed(&a, KUTHA_POLICY, &pin), 1);
}
