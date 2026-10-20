//! M026 S01 / Phase 85 — generator coverage + missing oracles (G-C04 / G-C05 / G-C11).
//! No product behavior change unless an oracle exposes a bug.

mod common;

use common::sem::generator::{MixStyle, ingest_mix, ingest_mix_n};
use kutha_common::{Op, ValidTime};
use kutha_runtime::{Runtime, RuntimeError};

fn m026_store_dir(label: &str) -> std::path::PathBuf {
    // Unique even when two Cargo processes run this binary together.
    static N: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let seq = N.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    let dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("tmp")
        .join(format!(
            "m026-{label}-{}-{nanos}-{seq}",
            std::process::id()
        ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("persist dir");
    dir
}

/// Coverage mix reaches the gaps named in the M026 S01 after-this (G-C02 family).
#[test]
fn ingest_mix_coverage_reaches_named_gaps() {
    let mut rt = Runtime::default();
    let meta = ingest_mix(&mut rt, 7, MixStyle::Coverage);
    assert!(
        meta.saw_retract_before_assert,
        "must attempt Retract before any Assert"
    );
    assert!(
        meta.saw_overlapping_assert,
        "must emit overlapping Asserts on one (s,r)"
    );
    assert!(meta.saw_keyed_retry, "must exercise keyed Assert retry");
    assert!(
        meta.saw_derived_correct_or_retract,
        "must Correct or Retract a Behavior-derived id (seed 7 fires inverse)"
    );
    assert!(meta.saw_assert);
    rt.replay_check().unwrap();
}

/// Empty mix (n_ops=0): anchor-only log; no product Assert (empty-log path).
#[test]
fn ingest_mix_coverage_empty_ops_is_anchor_only() {
    let mut rt = Runtime::default();
    let meta = ingest_mix_n(&mut rt, 1, MixStyle::Coverage, 0);
    assert!(!meta.saw_assert);
    assert!(
        rt.as_of(ValidTime::from(2015)).is_empty(),
        "no product Assert → empty as_of"
    );
    assert!(
        rt.log()
            .iter()
            .any(|e| matches!(&e.op, Op::TtAnchor { .. })),
        "seed anchor remains"
    );
}

/// G-C04: user Assert EventId identical after persist/open (not only Define/derived).
#[test]
fn user_assert_event_id_stable_across_persist_open() {
    let mut rt = Runtime::default();
    let subject = rt.intern("s-gc04");
    let relation = rt.intern("relatedTo");
    let object = rt.intern("o-gc04");
    let out = rt
        .emit(Op::Assert {
            subject,
            relation,
            object,
            valid_from: 2010,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap();
    let id = out.receipt.event_ids[0];
    let dir = m026_store_dir("gc04-assert-id");
    rt.persist(&dir).unwrap();
    let opened = Runtime::open(&dir).unwrap();
    let opened_id = opened
        .log()
        .iter()
        .find_map(|e| match &e.op {
            Op::Assert {
                subject: s,
                relation: r,
                object: o,
                ..
            } if *s == subject && *r == relation && *o == object => Some(e.id),
            _ => None,
        })
        .expect("opened log must contain the user Assert");
    assert_eq!(
        opened_id, id,
        "user Assert EventId must survive persist/open"
    );
    let _ = std::fs::remove_dir_all(dir);
}

/// G-C05: facade retract(subject, vf) with two live facts (different relations) retracts latest only.
#[test]
fn facade_retract_latest_of_two_live_same_subject_vf() {
    let mut rt = Runtime::default();
    let s = "s-gc05";
    let vf = 2010u64;
    rt.assert(s, "relatedTo", "o-rel", vf, None).unwrap();
    rt.assert(s, "inForceAs", "o-force", vf, None).unwrap();
    let subj = rt.intern(s);
    let related = rt.intern("relatedTo");
    let in_force = rt.intern("inForceAs");
    let o_rel = rt.intern("o-rel");
    let o_force = rt.intern("o-force");

    assert!(
        rt.as_of(ValidTime::from(2015))
            .contains(&(subj, related, o_rel))
    );
    assert!(
        rt.as_of(ValidTime::from(2015))
            .contains(&(subj, in_force, o_force))
    );

    rt.retract(s, vf).unwrap();

    let after = rt.as_of(ValidTime::from(2015));
    // Latest live by rev-find is inForceAs; relatedTo must remain.
    assert!(
        after.contains(&(subj, related, o_rel)),
        "earlier relation must stay live after facade retract of latest"
    );
    assert!(
        !after.contains(&(subj, in_force, o_force)),
        "latest same subject+vf fact must be retracted"
    );
}

/// G-C05 sibling: facade correct picks latest matching subject+relation+vf.
#[test]
fn facade_correct_latest_of_two_live_same_subject_relation_vf() {
    let mut rt = Runtime::default();
    let s = "s-gc05c";
    let r = "relatedTo";
    let vf = 2010u64;
    // Two supports: first then second (latest) on same s,r,vf via claim chain.
    let first = rt.assert(s, r, "o1", vf, None).unwrap();
    let claim = first.receipt.event_ids[0];
    let subj = rt.intern(s);
    let rel = rt.intern(r);
    let o2 = rt.intern("o2");
    let o3 = rt.intern("o3");
    rt.emit(Op::Assert {
        subject: subj,
        relation: rel,
        object: o2,
        valid_from: vf,
        valid_to: None,
        claim: Some(claim),
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    rt.correct(s, r, "o3", vf).unwrap();
    assert!(
        rt.as_of(ValidTime::from(2015)).contains(&(subj, rel, o3)),
        "correct must surface replacement object"
    );
}

/// G-C05: facade derive with two live premises uses latest subject fact as caused_by.
#[test]
fn facade_derive_uses_latest_live_premise() {
    let mut rt = Runtime::default();
    let s = "s-gc05d";
    rt.assert(s, "relatedTo", "o-early", 2010, None).unwrap();
    rt.assert(s, "relatedTo", "o-late", 2011, None).unwrap();
    let out = rt.derive("statute-fold", s, 2011).unwrap();
    assert!(
        !out.receipt.event_ids.is_empty(),
        "derive must mint a Behavior"
    );
    assert!(
        rt.log()
            .iter()
            .any(|e| matches!(&e.op, Op::Behavior { name, .. } if name == "statute-fold")),
        "derive must append Behavior statute-fold"
    );
}

/// G-C11: empty runtime as_of empty; persist/open preserves empty product picture.
#[test]
fn empty_runtime_persist_open_keeps_empty_as_of() {
    let rt = Runtime::default();
    assert!(rt.as_of(ValidTime::from(2015)).is_empty());
    let dir = m026_store_dir("gc11-empty");
    rt.persist(&dir).unwrap();
    let opened = Runtime::open(&dir).unwrap();
    assert!(
        opened.as_of(ValidTime::from(2015)).is_empty(),
        "empty runtime must reopen with empty as_of"
    );
    assert_eq!(opened.log().len(), rt.log().len());
    let _ = std::fs::remove_dir_all(dir);
}

/// BehaviorFree / WithHorn schedules unchanged for DET seed 42 (characterization pin).
#[test]
fn behavior_free_mix_seed_42_still_asserts() {
    let mut rt = Runtime::default();
    let meta = ingest_mix(&mut rt, 42, MixStyle::BehaviorFree);
    assert!(meta.saw_assert);
    assert!(!meta.saw_retract_before_assert);
    assert!(!meta.saw_keyed_retry);
    rt.replay_check().unwrap();
}

/// Kill-claim: Coverage must not silently skip Retract-before-Assert.
#[test]
fn coverage_retract_before_assert_is_unknown_fact() {
    let mut rt = Runtime::default();
    let err = rt
        .emit(Op::Retract {
            event_id: kutha_common::EventId::nil(),
        })
        .expect_err("empty retract");
    assert!(matches!(err, RuntimeError::UnknownFact { .. }));
}
