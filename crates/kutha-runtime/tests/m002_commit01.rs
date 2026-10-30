//! M002 S05 / COMMIT-01 oracles (Phase 53). Option A: QuantumOutcome is the marker.
//! D-07: RED records current failures; FF5/FF6/M010–M017 / m002_snap01 / m002_append01 stay.

use kutha_common::{
    Event, EventId, KUTHA_ALLOWS, KUTHA_RESERVED_RELS, Op, OutcomeDisposition, ValidTime,
};
use kutha_runtime::{Runtime, Snapshot, store};
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

const T_OLD: u64 = 2015;
const VF_FIRST: u64 = 2010;
const VF_SECOND: u64 = 2020;

fn persist_dir(label: &str) -> PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("tmp")
        .join(format!("p53-store-{label}-{nanos}"));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn seed_runtime() -> Runtime {
    let mut rt = Runtime::default();
    let article = rt.intern("art-12-speed");
    let in_force = rt.intern("inForceAs");
    let limit_50 = rt.intern("50-kmh");
    rt.emit(Op::Assert {
        subject: article,
        relation: in_force,
        object: limit_50,
        valid_from: VF_FIRST,
        valid_to: Some(VF_SECOND),
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    rt.emit_tt_anchor(T_OLD).unwrap();
    rt
}

fn log_ids(rt: &Runtime) -> Vec<EventId> {
    rt.log()
        .iter()
        .filter(|e| !matches!(e.op, Op::Define { .. }))
        .map(|e| e.id)
        .collect()
}

fn emit_product_assert(rt: &mut Runtime, subject: &str, object: &str) {
    let s = rt.intern(subject);
    let o = rt.intern(object);
    let r = rt.intern("inForceAs");
    rt.emit(Op::Assert {
        subject: s,
        relation: r,
        object: o,
        valid_from: VF_FIRST,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
}

fn crash_omit_last(p: &mut store::Persister, rt: &Runtime) {
    let n = store::pending_append_len(p, rt).unwrap();
    assert!(
        n >= 2,
        "pending batch must include payload and QuantumOutcome; n={n}"
    );
    store::append_with_event_failpoint(p, rt, n - 1)
        .expect_err("event failpoint must inject after N-1 events");
}

fn copy_store(src: &Path, dst: &Path) {
    let _ = fs::remove_dir_all(dst);
    fs::create_dir_all(dst).unwrap();
    for ent in fs::read_dir(src).unwrap() {
        let ent = ent.unwrap();
        if ent.file_type().unwrap().is_file() {
            fs::copy(ent.path(), dst.join(ent.file_name())).unwrap();
        }
    }
}

fn product_assert_op(relation: u32) -> Op {
    Op::Assert {
        subject: 0,
        relation,
        object: 0,
        valid_from: 0,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    }
}

#[test]
fn payload_without_outcome_is_not_visible() {
    let mut rt = seed_runtime();
    let dir = persist_dir("payload");
    store::persist(&rt, &dir).unwrap();
    let committed_fp = rt.fold().fingerprint();
    let committed_as_of = rt.as_of(ValidTime::from(VF_FIRST));
    let committed_ids = log_ids(&rt);
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    emit_product_assert(&mut rt, "p53-payload-article", "p53-payload-80");
    crash_omit_last(&mut p, &rt);

    let opened = store::open(&dir).expect("open after payload-without-QO");
    assert_eq!(
        opened.fold().fingerprint(),
        committed_fp,
        "uncommitted payload must not fold"
    );
    assert_eq!(opened.as_of(ValidTime::from(VF_FIRST)), committed_as_of);
    assert_eq!(log_ids(&opened), committed_ids);
    opened.replay_check().unwrap();

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn complete_quantum_with_outcome_is_visible() {
    let mut rt = seed_runtime();
    let dir = persist_dir("complete");
    store::persist(&rt, &dir).unwrap();
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    emit_product_assert(&mut rt, "p53-complete-article", "p53-complete-80");
    p.append(&rt).unwrap();
    let opened = store::open(&dir).unwrap();
    assert_eq!(opened.fold().fingerprint(), rt.fold().fingerprint());
    assert_eq!(
        opened.as_of(ValidTime::from(VF_FIRST)),
        rt.as_of(ValidTime::from(VF_FIRST))
    );
    assert_eq!(log_ids(&opened), log_ids(&rt));
    opened.replay_check().unwrap();

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn intern_only_suffix_survives() {
    let mut rt = seed_runtime();
    let dir = persist_dir("intern");
    store::persist(&rt, &dir).unwrap();
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    let t_a = rt.intern("p53-intern-a");
    let t_b = rt.intern("p53-intern-b");
    p.append(&rt).unwrap();
    let opened = store::open(&dir).unwrap();
    assert_eq!(opened.dictionary().id("p53-intern-a"), Some(t_a));
    assert_eq!(opened.dictionary().id("p53-intern-b"), Some(t_b));
    assert_eq!(opened.fold().fingerprint(), rt.fold().fingerprint());
    opened.replay_check().unwrap();

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn tt_anchor_only_suffix_survives() {
    let mut rt = seed_runtime();
    let dir = persist_dir("tt");
    store::persist(&rt, &dir).unwrap();
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    rt.emit_tt_anchor(2021).unwrap();
    p.append(&rt).unwrap();
    let opened = store::open(&dir).unwrap();
    assert_eq!(log_ids(&opened), log_ids(&rt));
    assert_eq!(opened.fold().fingerprint(), rt.fold().fingerprint());
    opened.replay_check().unwrap();

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn typed_justification_pair_survives() {
    let mut rt = seed_runtime();
    rt.emit_policy("p53-leased-policy", VF_FIRST, None).unwrap();
    let claim = rt
        .log()
        .iter()
        .find(|e| matches!(e.op, Op::Assert { .. }))
        .map(|e| e.id)
        .expect("seed assert");
    let dir = persist_dir("typed");
    store::persist(&rt, &dir).unwrap();
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    let jid = rt
        .record_justification(
            claim,
            vec![claim],
            vec![claim],
            "p53-rule",
            kutha_common::TransactionTime::MAX,
            kutha_common::ValidTime::from(VF_FIRST),
        )
        .unwrap();
    p.append(&rt).unwrap();
    let opened = store::open(&dir).unwrap();
    assert!(
        opened
            .justification_records()
            .iter()
            .any(|j| j.justification_id == jid),
        "JustificationCite must survive without a QuantumOutcome"
    );
    assert_eq!(opened.fold().fingerprint(), rt.fold().fingerprint());
    opened.replay_check().unwrap();

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn genesis_and_dictionary_asserts_survive_without_outcome() {
    let rt = Runtime::default();
    let dir = persist_dir("genesis");
    store::persist(&rt, &dir).unwrap();
    let opened = store::open(&dir).unwrap();
    let allows = opened
        .dictionary()
        .id(KUTHA_ALLOWS)
        .expect("kutha:allows interned at genesis");
    let dict_cut = opened.fold().dictionary_as_of(0);
    assert!(
        dict_cut.iter().any(|(_, r, _)| *r == allows),
        "genesis kutha:allows must survive a log with no QuantumOutcome"
    );
    for name in KUTHA_RESERVED_RELS {
        assert!(opened.dictionary().id(name).is_some(), "missing {name}");
    }
    opened.replay_check().unwrap();

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn product_assert_without_any_outcome_is_hidden() {
    let mut rt = Runtime::default();
    let dir = persist_dir("no-qo");
    store::persist(&rt, &dir).unwrap();
    let committed_fp = rt.fold().fingerprint();
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    emit_product_assert(&mut rt, "p53-noqo-article", "p53-noqo-80");
    crash_omit_last(&mut p, &rt);
    let opened = store::open(&dir).unwrap();
    assert_eq!(opened.fold().fingerprint(), committed_fp);
    assert!(
        opened.dictionary().id("p53-noqo-80").is_none()
            || !opened
                .as_of(ValidTime::from(VF_FIRST))
                .iter()
                .any(|(_, _, o)| opened.dictionary().lookup(*o) == Some("p53-noqo-80")),
        "product Assert without any QuantumOutcome must not be visible"
    );

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn snapshot_ahead_of_committed_prefix_fails_closed() {
    let rt = seed_runtime();
    let dir = persist_dir("snap-ahead");
    store::persist(&rt, &dir).unwrap();
    let snap_path = dir.join("snapshot.json");
    let mut snap: Snapshot = serde_json::from_reader(fs::File::open(&snap_path).unwrap()).unwrap();
    snap.log_offset = snap.log_offset.saturating_add(1);
    let tmp = dir.join("snapshot.json.tmp");
    serde_json::to_writer(fs::File::create(&tmp).unwrap(), &snap).unwrap();
    fs::rename(&tmp, &snap_path).unwrap();

    match store::open(&dir) {
        Ok(_) => panic!("D-04: snapshot ahead of committed prefix must fail closed"),
        Err(e) => assert_eq!(e.kind(), ErrorKind::InvalidData),
    }

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn open_report_counts_dropped() {
    let mut rt = seed_runtime();
    let dir = persist_dir("report");
    store::persist(&rt, &dir).unwrap();
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    emit_product_assert(&mut rt, "p53-report-article", "p53-report-80");
    crash_omit_last(&mut p, &rt);
    let (_opened, report) = store::open_report(&dir).unwrap();
    assert!(
        report.dropped_uncommitted > 0,
        "open_report must count hidden uncommitted events; got 0"
    );

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn open_for_append_heals_then_appends() {
    let mut rt = seed_runtime();
    let dir = persist_dir("heal");
    store::persist(&rt, &dir).unwrap();
    let committed_fp = rt.fold().fingerprint();
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    emit_product_assert(&mut rt, "p53-heal-article", "p53-heal-80");
    crash_omit_last(&mut p, &rt);

    let opened = store::open(&dir).unwrap();
    match store::Persister::attach(&opened, &dir) {
        Ok(_) => panic!("attach must stay fail-closed on a dirty uncommitted tail"),
        Err(e) => assert_eq!(e.kind(), ErrorKind::InvalidData),
    }

    // Crash = process gone: Drop unlocks. Live writers keep kutha.lock on Other (M030).
    drop(p);
    let (mut healed, mut p) = store::open_for_append(&dir)
        .expect("open_for_append must persist the committed prefix then attach");
    assert_eq!(healed.fold().fingerprint(), committed_fp);
    emit_product_assert(&mut healed, "p53-heal2-article", "p53-heal2-90");
    p.append(&healed).unwrap();
    let again = store::open(&dir).unwrap();
    assert_eq!(again.fold().fingerprint(), healed.fold().fingerprint());
    again.replay_check().unwrap();

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn heal_crash_matrix_reopens_to_committed_prefix() {
    let mut rt = seed_runtime();
    let template = persist_dir("heal-crash-src");
    store::persist(&rt, &template).unwrap();
    let committed_ids = log_ids(&rt);
    let committed_fp = rt.fold().fingerprint();
    let mut p = store::Persister::attach(&rt, &template).unwrap();
    emit_product_assert(&mut rt, "p53-hcm-article", "p53-hcm-80");
    crash_omit_last(&mut p, &rt);

    let steps = [
        store::PersistStep::Wal,
        store::PersistStep::Jsonl,
        store::PersistStep::Terms,
        store::PersistStep::Snapshot,
        store::PersistStep::Outcomes,
        store::PersistStep::Justifications,
    ];
    for step in steps {
        let dir = persist_dir(&format!("heal-crash-{step:?}"));
        copy_store(&template, &dir);
        match store::open_for_append_with_failpoint(&dir, step) {
            Ok(_) => panic!("heal failpoint must inject, step={step:?}"),
            Err(_) => {}
        }
        match store::open(&dir) {
            Ok(opened) => {
                assert_eq!(log_ids(&opened), committed_ids, "step={step:?}");
                assert_eq!(opened.fold().fingerprint(), committed_fp, "step={step:?}");
                opened.replay_check().unwrap();
            }
            Err(e) => {
                assert_eq!(e.kind(), ErrorKind::InvalidData, "step={step:?}: {e}");
                assert!(
                    matches!(step, store::PersistStep::Wal),
                    "only a WAL-first heal crash may fail closed (jsonl still append-shaped); step={step:?}: {e}"
                );
            }
        }
        let _ = fs::remove_dir_all(&dir);
    }

    let _ = fs::remove_dir_all(&template);
}

#[test]
fn every_op_variant_is_classified() {
    use store::CommitClass::*;
    let nil = EventId::nil();
    let table: [(Op, store::CommitClass); 11] = [
        (product_assert_op(1), NeedsOutcome),
        (Op::Retract { event_id: nil }, NeedsOutcome),
        (
            Op::Correct {
                event_id: nil,
                object: 0,
                valid_from: 0,
                valid_to: None,
            },
            NeedsOutcome,
        ),
        (
            Op::CorrectInterval {
                event_id: nil,
                object: 0,
                patch_from: 0,
                patch_to: None,
            },
            NeedsOutcome,
        ),
        (
            Op::Behavior {
                name: "x".into(),
                caused_by: nil,
                rule_version: String::new(),
                subject: 0,
                relation: 0,
                object: 0,
                valid_from: 0,
                valid_to: None,
            },
            NeedsOutcome,
        ),
        (Op::Define { name: "t".into() }, SelfCommit),
        (
            Op::QuantumOutcome {
                quantum_id: String::new(),
                disposition: OutcomeDisposition::Full,
                aborted_on_budget: false,
                events_in_quantum: 0,
                event_ids: vec![],
                receipt_digest_hex: String::new(),
                resume_of: None,
            },
            SelfCommit,
        ),
        (
            Op::JustificationCite {
                justification_id: String::new(),
                target_claim: nil,
                source_claim_ids: vec![],
                source_event_ids: vec![],
                rule_version: String::new(),
                tt: 0,
                vt: 0,
            },
            SelfCommit,
        ),
        (
            Op::StoreBlob {
                digest: "d".into(),
                bytes: vec![],
            },
            SelfCommit,
        ),
        (Op::TtAnchor { wall: 1 }, SelfCommit),
        (
            Op::Checkpoint {
                first_event_id: nil,
                last_event_id: nil,
                dropped_count: 0,
                fact_count: 0,
                fingerprint_hex: String::new(),
                blob_digest: String::new(),
            },
            SelfCommit,
        ),
    ];
    for (op, want) in table {
        assert_eq!(store::op_commit_class(&op), want, "op={op:?}");
    }

    let kutha = Event::stable_define(KUTHA_ALLOWS, 0);
    let product_name = Event::stable_define("inForceAs", 0);
    let kutha_assert = Event::new(product_assert_op(0), 0);
    let product_assert = Event::new(product_assert_op(1), 0);
    let kept = store::committed_prefix(vec![kutha, product_name, kutha_assert, product_assert]);
    assert!(
        kept.iter()
            .any(|e| matches!(&e.op, Op::Assert { relation: 0, .. })),
        "kutha:* Assert without QuantumOutcome must stay"
    );
    assert!(
        !kept
            .iter()
            .any(|e| matches!(&e.op, Op::Assert { relation: 1, .. })),
        "product Assert without QuantumOutcome must be dropped"
    );
}

fn prefix_tags(events: &[Event]) -> Vec<&'static str> {
    events
        .iter()
        .map(|e| match &e.op {
            Op::Assert { .. } => "Assert",
            Op::Define { .. } => "Define",
            Op::TtAnchor { .. } => "TtAnchor",
            Op::StoreBlob { .. } => "StoreBlob",
            Op::JustificationCite { .. } => "Cite",
            Op::QuantumOutcome { .. } => "QuantumOutcome",
            Op::Retract { .. } => "Retract",
            Op::Correct { .. } => "Correct",
            Op::CorrectInterval { .. } => "CorrectInterval",
            Op::Behavior { .. } => "Behavior",
            Op::Checkpoint { .. } => "Checkpoint",
            Op::PackInstall { .. } | Op::PackActivate { .. } | Op::PackRetire { .. } => {
                "PackLifecycle"
            }
        })
        .collect()
}

/// D-03: first NeedsOutcome in the tail cuts that op and every later op,
/// including later SelfCommit (`Define` / `TtAnchor` / `StoreBlob` / cite).
/// Hiding a later `Define` is safe on Runtime intern-then-emit order: `intern`
/// writes the Define before any referring op, so a kept SelfCommit cannot name
/// a term whose Define sits after the cut. Reverse-order synthetics are not a
/// Runtime log; this oracle does not change the rule.
#[test]
fn needs_outcome_hides_later_self_commit_tail() {
    let product = Event::new(product_assert_op(1), 0);
    let later_define = Event::stable_define("p53-after-cut-orphan", 0);
    let later_tt = Event::new(Op::TtAnchor { wall: 9 }, 0);
    let later_blob = Event::new(
        Op::StoreBlob {
            digest: "p53-orphan-blob".into(),
            bytes: vec![],
        },
        0,
    );
    let kept = store::committed_prefix(vec![product, later_define, later_tt, later_blob]);
    assert_eq!(
        prefix_tags(&kept),
        Vec::<&str>::new(),
        "tail [product Assert, Define, TtAnchor, StoreBlob] must hide all four"
    );
}

#[test]
fn leading_self_commit_survives_until_first_needs_outcome() {
    let lead = Event::stable_define("p53-lead-self-commit", 0);
    let product = Event::new(product_assert_op(1), 0);
    let later_tt = Event::new(Op::TtAnchor { wall: 9 }, 0);
    let kept = store::committed_prefix(vec![lead, product, later_tt]);
    assert_eq!(
        prefix_tags(&kept),
        vec!["Define"],
        "tail [Define, product Assert, TtAnchor] keeps only the leading Define"
    );
    assert!(matches!(
        &kept[0].op,
        Op::Define { name } if name == "p53-lead-self-commit"
    ));
}

#[test]
fn unreferenced_later_self_commit_is_hidden() {
    let nil = EventId::nil();
    let product = Event::new(product_assert_op(1), 0);
    let orphan_cite = Event::new(
        Op::JustificationCite {
            justification_id: "p53-orphan-cite".into(),
            target_claim: nil,
            source_claim_ids: vec![],
            source_event_ids: vec![],
            rule_version: String::new(),
            tt: 0,
            vt: 0,
        },
        0,
    );
    let kept = store::committed_prefix(vec![product, orphan_cite]);
    assert_eq!(
        prefix_tags(&kept),
        Vec::<&str>::new(),
        "later SelfCommit referenced by nothing must still be cut with the NeedsOutcome"
    );
}

#[test]
fn retract_correct_user_quanta_follow_the_same_rule() {
    let mut rt = seed_runtime();
    let dir = persist_dir("retract");
    store::persist(&rt, &dir).unwrap();
    let committed_fp = rt.fold().fingerprint();
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    let target = rt
        .log()
        .as_slice()
        .iter()
        .rev()
        .find(|e| matches!(e.op, Op::Assert { .. }))
        .map(|e| e.id)
        .expect("seed assert");
    rt.emit(Op::Retract { event_id: target }).unwrap();
    crash_omit_last(&mut p, &rt);
    let opened = store::open(&dir).unwrap();
    assert_eq!(opened.fold().fingerprint(), committed_fp);
    let _ = fs::remove_dir_all(&dir);

    let mut rt = seed_runtime();
    let dir = persist_dir("correct");
    store::persist(&rt, &dir).unwrap();
    let committed_fp = rt.fold().fingerprint();
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    let target = rt
        .log()
        .as_slice()
        .iter()
        .rev()
        .find(|e| matches!(e.op, Op::Assert { .. }))
        .map(|e| e.id)
        .expect("seed assert");
    let new_obj = rt.intern("p53-corrected");
    rt.emit(Op::Correct {
        event_id: target,
        object: new_obj,
        valid_from: VF_FIRST,
        valid_to: Some(VF_SECOND),
    })
    .unwrap();
    crash_omit_last(&mut p, &rt);
    let opened = store::open(&dir).unwrap();
    assert_eq!(opened.fold().fingerprint(), committed_fp);
    let _ = fs::remove_dir_all(&dir);

    let mut rt = seed_runtime();
    let dir = persist_dir("cinterval");
    store::persist(&rt, &dir).unwrap();
    let committed_fp = rt.fold().fingerprint();
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    let target = rt
        .log()
        .as_slice()
        .iter()
        .rev()
        .find(|e| matches!(e.op, Op::Assert { .. }))
        .map(|e| e.id)
        .expect("seed assert");
    let obj = rt.intern("50-kmh");
    rt.emit(Op::CorrectInterval {
        event_id: target,
        object: obj,
        patch_from: VF_FIRST,
        patch_to: Some(VF_SECOND),
    })
    .unwrap();
    crash_omit_last(&mut p, &rt);
    let opened = store::open(&dir).unwrap();
    assert_eq!(opened.fold().fingerprint(), committed_fp);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn horn_followon_quantum_is_atomic() {
    let mut rt = seed_runtime();
    let dir = persist_dir("horn");
    store::persist(&rt, &dir).unwrap();
    let committed_fp = rt.fold().fingerprint();
    let committed_ids = log_ids(&rt);
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    let alice = rt.intern("p53-Alice");
    let bob = rt.intern("p53-Bob");
    let knows = rt.intern("knows");
    rt.emit(Op::Assert {
        subject: alice,
        relation: knows,
        object: bob,
        valid_from: VF_FIRST,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    let has_behavior = rt
        .log()
        .iter()
        .any(|e| matches!(&e.op, Op::Behavior { name, .. } if name == "inverse_knows"));
    assert!(has_behavior, "knows Assert must mint inverse_knows");
    crash_omit_last(&mut p, &rt);
    let opened = store::open(&dir).unwrap();
    assert_eq!(opened.fold().fingerprint(), committed_fp);
    assert_eq!(
        log_ids(&opened),
        committed_ids,
        "cutting inside payload+follow-ons+outcome must hide the whole quantum"
    );

    let _ = fs::remove_dir_all(&dir);
}
