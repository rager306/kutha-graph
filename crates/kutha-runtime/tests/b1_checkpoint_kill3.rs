//! Third-pass independent kills for M154 sealed-prefix checkpoint (phase 228).
//! Public API only. Covers gaps the first two rounds did not: prefix
//! Correct/CI/Retract residuals, windowed pins + allowlist, Horn un-derive,
//! as_known_on at the last prefix TT-anchor wall, empty vs non-empty active,
//! writer-lock vs checkpoint, migrate_compat / kutha-migrate, post-checkpoint
//! two-segment rollover, re-checkpoint receipt count, receipt-field determinism,
//! and remap_legacy_ops rejection of Checkpoint.

use kutha_common::{
    Event, EventId, KUTHA_RESERVED_RELS, Op, OpMigrateError, RuleAst, RuleAtom, RuleTerm,
    ValidTime, WallTime, policy_version_hash, remap_legacy_ops,
};
use kutha_runtime::{Runtime, RuntimeError, store};
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
    common::PersistScratch::new(&format!("b1-kill3-{label}"))
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

fn sealed_prefix_len(dir: &Path) -> usize {
    let man: store::Manifest =
        serde_json::from_str(&fs::read_to_string(dir.join(store::MANIFEST_REL)).unwrap()).unwrap();
    man.sealed.iter().map(|s| s.count as usize).sum()
}

fn sealed_count(dir: &Path) -> usize {
    let man: store::Manifest =
        serde_json::from_str(&fs::read_to_string(dir.join(store::MANIFEST_REL)).unwrap()).unwrap();
    man.sealed.len()
}

fn export_events(dir: &Path) -> Vec<Event> {
    store::export_jsonl(dir).unwrap();
    let text = fs::read_to_string(dir.join("events.jsonl")).unwrap();
    text.lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str::<Event>(l).expect("jsonl Event"))
        .collect()
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

fn apply_suffix(dir: &Path, suffix: &[Event], dict: Vec<String>) {
    let opened = store::open(dir).unwrap();
    let mut events = opened.log().as_slice().to_vec();
    events.extend(suffix.iter().cloned());
    let rt = Runtime::from_dict_and_events(dict, events, opened.max_cascade)
        .expect("replay identical suffix onto checkpointed store");
    store::persist(&rt, dir).unwrap();
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

fn object_at(rt: &Runtime, subject: &str, relation: &str, vt: u64) -> Option<String> {
    let s = rt.dictionary().id(subject)?;
    let r = rt.dictionary().id(relation)?;
    let triples = rt.as_of(ValidTime::from(vt));
    let tid = triples.iter().find(|(a, b, _)| *a == s && *b == r)?.2;
    rt.dictionary().lookup(tid).map(str::to_string)
}

fn checkpoint_receipts(events: &[Event]) -> Vec<&Op> {
    events
        .iter()
        .filter(|e| matches!(e.op, Op::Checkpoint { .. }))
        .map(|e| &e.op)
        .collect()
}

fn receipt_fields(op: &Op) -> (EventId, EventId, u64, u64, String, String) {
    match op {
        Op::Checkpoint {
            first_event_id,
            last_event_id,
            dropped_count,
            fact_count,
            fingerprint_hex,
            blob_digest,
        } => (
            *first_event_id,
            *last_event_id,
            *dropped_count,
            *fact_count,
            fingerprint_hex.clone(),
            blob_digest.clone(),
        ),
        _ => panic!("expected Op::Checkpoint"),
    }
}

/// Prefix contains Retract, Correct, and a multi-residual CorrectInterval of
/// prefix facts. After checkpoint, as_of at leftover/patch/suffix windows and
/// Retract/Correct pictures must match an un-checkpointed twin.
#[test]
fn r3_prefix_correct_ci_retract_multi_residual_vs_twin() {
    let _g = tiny_limits();
    let mut rt = Runtime::default();
    rt.emit_rule("derive_pq", 2010, None).unwrap();
    rt.emit_policy("leased-policy", 2010, None).unwrap();

    let retract_id = emit_keyed(
        &mut rt,
        "art-retract",
        "inForceAs",
        "50-kmh",
        2010,
        None,
        "retract-target",
    );
    rt.emit(Op::Retract {
        event_id: retract_id,
    })
    .unwrap();

    let correct_id = emit_keyed(
        &mut rt,
        "art-correct",
        "inForceAs",
        "50-kmh",
        2010,
        Some(2020),
        "correct-target",
    );
    let obj_90 = rt.intern("90-kmh");
    rt.emit(Op::Correct {
        event_id: correct_id,
        object: obj_90,
        valid_from: 2010,
        valid_to: Some(2020),
    })
    .unwrap();

    let ci_id = emit_keyed(
        &mut rt,
        "art-ci-multi",
        "inForceAs",
        "50-kmh",
        2010,
        Some(2020),
        "ci-multi",
    );
    let obj_80 = rt.intern("80-kmh");
    rt.emit(Op::CorrectInterval {
        event_id: ci_id,
        object: obj_80,
        patch_from: 2012,
        patch_to: Some(2018),
    })
    .unwrap();

    pad(&mut rt, 8, "ci");
    rt.emit_tt_anchor(2015).unwrap();

    assert_eq!(
        object_at(&rt, "art-ci-multi", "inForceAs", 2011).as_deref(),
        Some("50-kmh")
    );
    assert_eq!(
        object_at(&rt, "art-ci-multi", "inForceAs", 2015).as_deref(),
        Some("80-kmh")
    );
    assert_eq!(
        object_at(&rt, "art-ci-multi", "inForceAs", 2019).as_deref(),
        Some("50-kmh")
    );
    assert!(object_at(&rt, "art-retract", "inForceAs", 2015).is_none());
    assert_eq!(
        object_at(&rt, "art-correct", "inForceAs", 2015).as_deref(),
        Some("90-kmh")
    );

    let ckp = persist_dir("ci-ckp");
    let twin = persist_dir("ci-twin");
    store::persist(&rt, &ckp).unwrap();
    assert!(sealed_count(&ckp) >= 1, "fixture must seal a prefix");
    copy_store(&ckp, &twin);
    store::checkpoint_sealed_prefix(&ckp).unwrap();

    let a = store::open(&ckp).expect("checkpointed CI store");
    let b = store::open(&twin).expect("twin");
    a.replay_check()
        .expect("HIGH R3: replay_check after prefix CI/Retract/Correct");
    b.replay_check().unwrap();
    assert_eq!(
        a.fold().fingerprint(),
        b.fold().fingerprint(),
        "HIGH R3: CI fingerprint"
    );
    for vt in [2011u64, 2015, 2019] {
        assert_eq!(
            object_at(&a, "art-ci-multi", "inForceAs", vt),
            object_at(&b, "art-ci-multi", "inForceAs", vt),
            "HIGH R3: CI-multi residual as_of({vt})"
        );
        assert_eq!(
            a.as_of(ValidTime::from(vt)),
            b.as_of(ValidTime::from(vt)),
            "HIGH R3: as_of({vt}) vs twin"
        );
    }
    assert!(object_at(&a, "art-retract", "inForceAs", 2015).is_none());
    assert_eq!(
        object_at(&a, "art-correct", "inForceAs", 2015).as_deref(),
        Some("90-kmh")
    );
    let _ = fs::remove_dir_all(&ckp);
    let _ = fs::remove_dir_all(&twin);
}

/// Reserved `kutha:*` TermIds, windowed `emit_allow` (delivery_key allowlist),
/// and windowed rule/policy pins must answer like the twin.
#[test]
fn r3_reserved_allowlist_and_windowed_pins_vs_twin() {
    let _g = tiny_limits();
    let mut rt = Runtime::default();
    let horn = horn_related();
    let rule_pin = horn.hash();
    rt.emit_rule_ast(&horn, 2010, Some(2016)).unwrap();
    rt.emit_policy("win-policy", 2012, Some(2018)).unwrap();
    rt.emit_allow("customRel", 2010, Some(2018)).unwrap();
    emit_keyed(
        &mut rt,
        "cust-s",
        "customRel",
        "cust-o",
        2012,
        Some(2017),
        "custom-dk",
    );
    pad(&mut rt, 8, "allow");
    rt.emit_tt_anchor(2015).unwrap();
    let policy_pin = policy_version_hash("win-policy");

    let ckp = persist_dir("allow-ckp");
    let twin = persist_dir("allow-twin");
    store::persist(&rt, &ckp).unwrap();
    assert!(sealed_count(&ckp) >= 1);
    copy_store(&ckp, &twin);
    store::checkpoint_sealed_prefix(&ckp).unwrap();

    let a = store::open(&ckp).expect("checkpointed allow/pin store");
    let b = store::open(&twin).expect("twin");
    a.replay_check().unwrap();
    b.replay_check().unwrap();
    for name in KUTHA_RESERVED_RELS {
        assert_eq!(
            a.dictionary().id(name),
            b.dictionary().id(name),
            "HIGH R3: reserved TermId {name}"
        );
        assert!(
            a.dictionary().id(name).is_some(),
            "HIGH R3: {name} interned"
        );
    }
    assert_eq!(a.fold().fingerprint(), b.fold().fingerprint());
    assert_eq!(
        a.fold().relation_allowed_at("customRel", u64::MAX, 2012),
        b.fold().relation_allowed_at("customRel", u64::MAX, 2012),
        "HIGH R3: customRel allowlist inside window"
    );
    assert!(
        a.fold().relation_allowed_at("customRel", u64::MAX, 2012),
        "HIGH R3: customRel must stay allowed inside the prefix window"
    );
    assert_eq!(
        a.fold().relation_allowed_at("customRel", u64::MAX, 2019),
        b.fold().relation_allowed_at("customRel", u64::MAX, 2019),
        "HIGH R3: customRel allowlist outside window"
    );
    assert!(
        !a.fold().relation_allowed_at("customRel", u64::MAX, 2019),
        "HIGH R3: customRel must be denied outside the prefix window"
    );
    assert_eq!(
        a.fold().rule_hash_live_at(&rule_pin, u64::MAX, 2012),
        b.fold().rule_hash_live_at(&rule_pin, u64::MAX, 2012)
    );
    assert!(a.fold().rule_hash_live_at(&rule_pin, u64::MAX, 2012));
    assert!(!a.fold().rule_hash_live_at(&rule_pin, u64::MAX, 2017));
    assert_eq!(
        a.fold().live_policy_pin_at(u64::MAX, 2015),
        b.fold().live_policy_pin_at(u64::MAX, 2015)
    );
    assert_eq!(
        a.fold().live_policy_pin_at(u64::MAX, 2015).as_deref(),
        Some(policy_pin.as_str())
    );
    assert!(a.fold().live_policy_pin_at(u64::MAX, 2011).is_none());
    assert!(a.fold().live_policy_pin_at(u64::MAX, 2019).is_none());
    assert_eq!(
        object_at(&a, "cust-s", "customRel", 2014).as_deref(),
        Some("cust-o")
    );

    let (mut wr, mut persister) = store::open_for_append(&ckp).unwrap();
    let s = wr.intern("cust-s2");
    let r = wr.intern("customRel");
    let o = wr.intern("cust-o2");
    wr.emit(Op::Assert {
        subject: s,
        relation: r,
        object: o,
        valid_from: 2013,
        valid_to: None,
        claim: None,
        delivery_key: Some("custom-dk-2".into()),
        polarity: None,
    })
    .expect("HIGH R3: emit allowed customRel after checkpoint");
    let late = wr.emit(Op::Assert {
        subject: s,
        relation: r,
        object: o,
        valid_from: 2019,
        valid_to: None,
        claim: None,
        delivery_key: Some("custom-dk-late".into()),
        polarity: None,
    });
    assert!(
        matches!(late, Err(RuntimeError::UnknownRelation { .. })),
        "HIGH R3: customRel emit outside allow window must fail, got {late:?}"
    );
    persister.append(&wr).unwrap();
    drop(persister);
    let _ = fs::remove_dir_all(&ckp);
    let _ = fs::remove_dir_all(&twin);
}

/// Prefix Horn fixpoint (A-B and B-C both sealed) derives A-C. Tail retract of
/// the A-B premise must un-derive A-C (live_triples / except-set) like the twin.
#[test]
fn r3_prefix_horn_fixpoint_tail_retract_underives_vs_twin() {
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
    assert!(sealed_count(&ckp) >= 1);
    let encoded = store::encoded_events(&rt);
    let prefix_len = sealed_prefix_len(&ckp);
    assert!(
        encoded[..prefix_len].iter().any(|e| e.id == ab),
        "A-B premise must sit in the sealed prefix"
    );
    copy_store(&ckp, &twin);
    store::checkpoint_sealed_prefix(&ckp).unwrap();

    let orig_n = rt.log().len();
    rt.emit(Op::Retract { event_id: ab })
        .expect("retract prefix Horn premise");
    let suffix = rt.log().as_slice()[orig_n..].to_vec();
    store::persist(&rt, &twin).unwrap();
    apply_suffix(&ckp, &suffix, rt.dictionary().strings().to_vec());

    let a = store::open(&ckp).expect("checkpointed horn store");
    let b = store::open(&twin).expect("twin");
    a.replay_check()
        .expect("HIGH R3: replay_check after tail retract of prefix Horn premise");
    b.replay_check().unwrap();
    assert_eq!(
        a.fold().fingerprint(),
        b.fold().fingerprint(),
        "HIGH R3: Horn un-derive fingerprint vs twin"
    );
    assert!(
        !live_triple(&a, "A", "relatedTo", "C"),
        "HIGH R3: relatedTo(A,C) must un-derive after retracting prefix A-B"
    );
    assert_eq!(
        live_triple(&a, "A", "relatedTo", "C"),
        live_triple(&b, "A", "relatedTo", "C")
    );
    assert_eq!(
        a.as_of(ValidTime::from(2015)),
        b.as_of(ValidTime::from(2015)),
        "HIGH R3: as_of after Horn un-derive"
    );
    let _ = fs::remove_dir_all(&ckp);
    let _ = fs::remove_dir_all(&twin);
}

/// as_known_on at the exact wall of the last prefix TtAnchor (restored from the
/// checkpoint blob; the TtAnchor event itself is dropped).
#[test]
fn r2_as_known_on_at_last_prefix_tt_anchor_wall() {
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
    rt.emit_tt_anchor(2015).unwrap();
    pad(&mut rt, 6, "tt");
    rt.emit_tt_anchor(2021).unwrap();
    pad(&mut rt, 4, "tt2");

    let ckp = persist_dir("tt-ckp");
    let twin = persist_dir("tt-twin");
    store::persist(&rt, &ckp).unwrap();
    copy_store(&ckp, &twin);
    let encoded = store::encoded_events(&rt);
    let prefix_len = sealed_prefix_len(&ckp);
    assert!(prefix_len > 0 && prefix_len < encoded.len());
    let prefix_anchor_walls: Vec<u64> = encoded[..prefix_len]
        .iter()
        .filter_map(|e| match e.op {
            Op::TtAnchor { wall } => Some(wall),
            _ => None,
        })
        .collect();
    assert!(
        !prefix_anchor_walls.is_empty(),
        "sealed prefix must contain a TtAnchor"
    );
    let last_wall = *prefix_anchor_walls.last().unwrap();
    store::checkpoint_sealed_prefix(&ckp).unwrap();

    let a = store::open(&ckp).unwrap();
    let b = store::open(&twin).unwrap();
    a.replay_check().unwrap();
    assert!(
        !a.log()
            .iter()
            .any(|e| matches!(e.op, Op::TtAnchor { wall } if wall == last_wall)),
        "R2: prefix TtAnchor {last_wall} must leave the log"
    );
    let got = a.as_known_on(WallTime::from(last_wall));
    let want = b.as_known_on(WallTime::from(last_wall));
    assert!(
        got.is_some(),
        "HIGH R2: as_known_on at last prefix TtAnchor wall {last_wall} must resolve"
    );
    assert_eq!(got, want, "HIGH R2: as_known_on({last_wall}) vs twin");
    if last_wall > 0 {
        assert_eq!(
            a.as_known_on(WallTime::from(last_wall - 1)),
            b.as_known_on(WallTime::from(last_wall - 1)),
            "HIGH R2: as_known_on just before last prefix wall"
        );
    }
    let _ = fs::remove_dir_all(&ckp);
    let _ = fs::remove_dir_all(&twin);
}

/// Checkpoint a store whose active segment is empty (whole log sealed) and one
/// whose active tail is non-empty. Answers must match the twin either way.
#[test]
fn r1_empty_active_and_nonempty_active_checkpoint() {
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
    pad(&mut rt, 6, "act");
    rt.emit_tt_anchor(2015).unwrap();

    let nonempty = persist_dir("act-nonempty");
    let twin = persist_dir("act-twin");
    store::persist(&rt, &nonempty).unwrap();
    copy_store(&nonempty, &twin);
    assert!(sealed_count(&nonempty) >= 1);
    let man: store::Manifest =
        serde_json::from_str(&fs::read_to_string(nonempty.join(store::MANIFEST_REL)).unwrap())
            .unwrap();
    let active_events = {
        let encoded = export_events(&nonempty);
        encoded.len() - man.sealed.iter().map(|s| s.count as usize).sum::<usize>()
    };
    assert!(
        active_events > 0,
        "non-empty active fixture must have tail events (got {active_events})"
    );
    let before_fp = rt.fold().fingerprint();
    store::checkpoint_sealed_prefix(&nonempty).unwrap();
    let opened = store::open(&nonempty).unwrap();
    opened.replay_check().unwrap();
    assert_eq!(opened.fold().fingerprint(), before_fp);

    let empty_active = persist_dir("act-empty");
    copy_store(&twin, &empty_active);
    let events = export_events(&empty_active);
    let n = events.len();
    assert!(n >= 2);
    store::write_test_segmented(&empty_active, &events, &[n], 0).unwrap();
    let man: store::Manifest =
        serde_json::from_str(&fs::read_to_string(empty_active.join(store::MANIFEST_REL)).unwrap())
            .unwrap();
    assert!(man.sealed.iter().map(|s| s.count).sum::<u64>() == n as u64);
    store::open(&empty_active).expect("empty-active layout must open before checkpoint");
    store::checkpoint_sealed_prefix(&empty_active)
        .expect("HIGH R1: checkpoint of empty-active (empty tail) store");
    let opened = store::open(&empty_active).expect("HIGH R1: open after empty-tail checkpoint");
    opened.replay_check().unwrap();
    let twin_rt = store::open(&twin).unwrap();
    assert_eq!(
        opened.fold().fingerprint(),
        twin_rt.fold().fingerprint(),
        "HIGH R1: empty-tail checkpoint fingerprint vs twin"
    );
    assert_eq!(
        opened.as_of(ValidTime::from(2015)),
        twin_rt.as_of(ValidTime::from(2015))
    );
    let _ = fs::remove_dir_all(&nonempty);
    let _ = fs::remove_dir_all(&empty_active);
    let _ = fs::remove_dir_all(&twin);
}

/// Holding `open_for_append`'s writer lock must make `checkpoint_sealed_prefix` Err.
#[test]
fn r4_checkpoint_while_open_for_append_lock_held() {
    let _g = tiny_limits();
    let dir = persist_dir("lock");
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
    pad(&mut rt, 4, "lock");
    store::persist(&rt, &dir).unwrap();
    assert!(sealed_count(&dir) >= 1);
    let (_rt, persister) = store::open_for_append(&dir).unwrap();
    let err = store::checkpoint_sealed_prefix(&dir).expect_err(
        "HIGH R4: checkpoint_sealed_prefix must Err while open_for_append holds kutha.lock",
    );
    assert_eq!(
        err.kind(),
        ErrorKind::WouldBlock,
        "HIGH R4: expected WouldBlock, got {err}"
    );
    assert!(
        err.to_string().contains(store::STORE_LOCK_REL),
        "HIGH R4: error must name the lock file; got {err}"
    );
    drop(persister);
    store::checkpoint_sealed_prefix(&dir)
        .expect("checkpoint must succeed after the writer lock is released");
    let _ = fs::remove_dir_all(&dir);
}

/// `migrate_compat` / `kutha-migrate` on a checkpointed dir must fail closed
/// (`InvalidData`) and leave payload files unchanged (D-04).
#[test]
fn r4_migrate_compat_and_kutha_migrate_on_checkpointed() {
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
    let dir = persist_dir("mig-lib");
    store::persist(&rt, &dir).unwrap();
    store::checkpoint_sealed_prefix(&dir).unwrap();
    let before_files = payload_listing(&dir);

    let err = store::migrate_compat(&dir).expect_err("migrate_compat must refuse Checkpoint");
    assert_eq!(err.kind(), ErrorKind::InvalidData);
    assert_eq!(
        payload_listing(&dir),
        before_files,
        "HIGH R4: migrate_compat Err must leave payload files unchanged"
    );

    let cli_dir = persist_dir("mig-cli");
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
    pad(&mut rt, 6, "migc");
    store::persist(&rt, &cli_dir).unwrap();
    store::checkpoint_sealed_prefix(&cli_dir).unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_kutha-migrate"))
        .arg(&cli_dir)
        .output()
        .expect("spawn kutha-migrate");
    assert!(
        !out.status.success(),
        "HIGH R4: kutha-migrate must not succeed on a checkpointed store"
    );
    let _ = fs::remove_dir_all(&dir);
    let _ = fs::remove_dir_all(&cli_dir);
}

/// After checkpoint, Persister appends with the segment-limits helper must
/// rollover at least two new sealed segments and stay exact vs a twin.
#[test]
fn r3_two_sealed_segments_rollover_after_checkpoint() {
    let _g = tiny_limits();
    let mut rt = Runtime::default();
    rt.emit_rule_ast(&horn_related(), 2010, None).unwrap();
    emit_keyed(&mut rt, "A", "relatedTo", "B", 2010, None, "horn-ab");
    pad(&mut rt, 6, "roll");
    rt.emit_tt_anchor(2015).unwrap();

    let ckp = persist_dir("roll-ckp");
    let twin = persist_dir("roll-twin");
    store::persist(&rt, &ckp).unwrap();
    copy_store(&ckp, &twin);
    store::checkpoint_sealed_prefix(&ckp).unwrap();
    let sealed_before = sealed_count(&ckp);
    assert!(sealed_before >= 1);

    let orig_n = rt.log().len();
    pad(&mut rt, 8, "roll-tail");
    let suffix = rt.log().as_slice()[orig_n..].to_vec();
    store::persist(&rt, &twin).unwrap();

    let (mut opened, mut persister) = store::open_for_append(&ckp).unwrap();
    let mut events = opened.log().as_slice().to_vec();
    events.extend(suffix.iter().cloned());
    let replayed = Runtime::from_dict_and_events(
        opened.dictionary().strings().to_vec(),
        events,
        opened.max_cascade,
    )
    .unwrap();
    opened = replayed;
    persister.append(&opened).unwrap();
    drop(persister);
    let sealed_after = sealed_count(&ckp);
    assert!(
        sealed_after >= sealed_before + 2,
        "HIGH R3: expected ≥2 new sealed segments after checkpoint rollover (before={sealed_before} after={sealed_after})"
    );

    let a = store::open(&ckp).unwrap();
    let b = store::open(&twin).unwrap();
    a.replay_check().unwrap();
    b.replay_check().unwrap();
    assert_eq!(
        a.fold().fingerprint(),
        b.fold().fingerprint(),
        "HIGH R3: rollover-after-checkpoint fingerprint vs twin"
    );
    assert_eq!(
        a.as_of(ValidTime::from(2015)),
        b.as_of(ValidTime::from(2015))
    );
    let _ = fs::remove_dir_all(&ckp);
    let _ = fs::remove_dir_all(&twin);
}

/// Re-checkpoint leaves exactly one `Op::Checkpoint` (old receipt is folded).
#[test]
fn r5_recheckpoint_leaves_exactly_one_receipt() {
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
    pad(&mut rt, 6, "r5");
    rt.emit_tt_anchor(2015).unwrap();
    let twice = persist_dir("r5-twice");
    store::persist(&rt, &twice).unwrap();
    store::checkpoint_sealed_prefix(&twice).unwrap();
    store::checkpoint_sealed_prefix(&twice).unwrap();
    let opened = store::open(&twice).unwrap();
    opened.replay_check().unwrap();
    let n = checkpoint_receipts(opened.log().as_slice()).len();
    assert_eq!(
        n, 1,
        "HIGH R5: checkpoint twice must leave exactly one receipt, got {n}"
    );

    let ckp = persist_dir("r5-append");
    let twin = persist_dir("r5-twin");
    store::persist(&rt, &ckp).unwrap();
    copy_store(&ckp, &twin);
    store::checkpoint_sealed_prefix(&ckp).unwrap();
    let orig_n = rt.log().len();
    pad(&mut rt, 4, "r5b");
    let suffix = rt.log().as_slice()[orig_n..].to_vec();
    store::persist(&rt, &twin).unwrap();
    apply_suffix(&ckp, &suffix, rt.dictionary().strings().to_vec());
    store::checkpoint_sealed_prefix(&ckp).unwrap();
    let a = store::open(&ckp).unwrap();
    let b = store::open(&twin).unwrap();
    a.replay_check().unwrap();
    assert_eq!(
        checkpoint_receipts(a.log().as_slice()).len(),
        1,
        "HIGH R5: checkpoint-append-checkpoint must leave exactly one receipt"
    );
    assert_eq!(a.fold().fingerprint(), b.fold().fingerprint());
    let _ = fs::remove_dir_all(&twice);
    let _ = fs::remove_dir_all(&ckp);
    let _ = fs::remove_dir_all(&twin);
}

/// Two checkpoints of byte-identical stores: receipt *fields* match.
/// Minted StoreBlob/Checkpoint `Event.id`s differ (Event::new uses UUID v7).
#[test]
fn r6_determinism_byte_identical_stores_same_receipt_fields() {
    let _g = tiny_limits();
    let mut rt = Runtime::default();
    rt.emit_rule_ast(&horn_related(), 2010, None).unwrap();
    emit_keyed(&mut rt, "A", "relatedTo", "B", 2010, None, "horn-ab");
    pad(&mut rt, 6, "det");
    rt.emit_tt_anchor(2015).unwrap();
    let a_dir = persist_dir("det-a");
    let b_dir = persist_dir("det-b");
    store::persist(&rt, &a_dir).unwrap();
    copy_store(&a_dir, &b_dir);
    store::checkpoint_sealed_prefix(&a_dir).unwrap();
    store::checkpoint_sealed_prefix(&b_dir).unwrap();
    let ra = store::open(&a_dir).unwrap();
    let rb = store::open(&b_dir).unwrap();
    ra.replay_check().unwrap();
    rb.replay_check().unwrap();
    assert_eq!(
        ra.fold().fingerprint(),
        rb.fold().fingerprint(),
        "HIGH R6: byte-identical checkpoints must fold the same"
    );
    let oa = checkpoint_receipts(ra.log().as_slice());
    let ob = checkpoint_receipts(rb.log().as_slice());
    assert_eq!(oa.len(), 1);
    assert_eq!(ob.len(), 1);
    let (fa, la, da, ca, ha, ba) = receipt_fields(oa[0]);
    let (fb, lb, db, cb, hb, bb) = receipt_fields(ob[0]);
    assert_eq!(
        (fa, la, da, ca, ha),
        (fb, lb, db, cb, hb),
        "HIGH R6: receipt fields"
    );
    let id_a = ra
        .log()
        .iter()
        .find(|e| matches!(e.op, Op::Checkpoint { .. }))
        .map(|e| e.id);
    let id_b = rb
        .log()
        .iter()
        .find(|e| matches!(e.op, Op::Checkpoint { .. }))
        .map(|e| e.id);
    eprintln!(
        "kill3 determinism blob_digest_eq={} checkpoint_event_id_eq={}",
        ba == bb,
        id_a == id_b
    );
    // blob_digest hashes Snapshot JSON. If serde of a HashSet is unordered it
    // may differ without changing answers; that is LOW. Divergence of opened
    // fingerprints is HIGH (already asserted). Equal blob_digest is the happy path.
    if ba != bb {
        assert_eq!(
            ra.as_of(ValidTime::from(2015)),
            rb.as_of(ValidTime::from(2015)),
            "HIGH R6: blob_digest differed but answers must still match"
        );
    }
    let a_ids: Vec<EventId> = ra
        .log()
        .iter()
        .filter(|e| matches!(e.op, Op::Checkpoint { .. } | Op::StoreBlob { .. }))
        .map(|e| e.id)
        .collect();
    let b_ids: Vec<EventId> = rb
        .log()
        .iter()
        .filter(|e| matches!(e.op, Op::Checkpoint { .. } | Op::StoreBlob { .. }))
        .map(|e| e.id)
        .collect();
    // Legitimate difference: UUID v7 on Event::new for the minted blob + receipt.
    let _ = (a_ids, b_ids, ba, bb);
    let _ = fs::remove_dir_all(&a_dir);
    let _ = fs::remove_dir_all(&b_dir);
}

/// R6: Checkpoint is a modern serde variant; remap_legacy_ops must not grow an arm.
#[test]
fn r6_remap_legacy_ops_rejects_checkpoint() {
    let value = serde_json::json!({
        "Checkpoint": {
            "first_event_id": EventId::nil(),
            "last_event_id": EventId::nil(),
            "dropped_count": 1,
            "fact_count": 2,
            "fingerprint_hex": "aa",
            "blob_digest": "bb"
        }
    });
    let err = remap_legacy_ops(&value, |_| 0).expect_err("R6: remap must not accept Checkpoint");
    assert!(
        matches!(err, OpMigrateError::UnknownVariant(ref s) if s == "Checkpoint"),
        "R6: expected UnknownVariant(Checkpoint), got {err:?}"
    );
}

/// Next-seq / fact seq after checkpoint must match the twin (nothing that
/// changes a later append's seq is lost).
#[test]
fn r3_fact_seq_and_next_append_vs_twin() {
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
    pad(&mut rt, 6, "seq");
    let ckp = persist_dir("seq-ckp");
    let twin = persist_dir("seq-twin");
    store::persist(&rt, &ckp).unwrap();
    copy_store(&ckp, &twin);
    store::checkpoint_sealed_prefix(&ckp).unwrap();
    let orig_n = rt.log().len();
    emit_keyed(
        &mut rt,
        "art-post",
        "inForceAs",
        "110-kmh",
        2010,
        Some(2020),
        "post-seq",
    );
    let suffix = rt.log().as_slice()[orig_n..].to_vec();
    store::persist(&rt, &twin).unwrap();
    apply_suffix(&ckp, &suffix, rt.dictionary().strings().to_vec());
    let a = store::open(&ckp).unwrap();
    let b = store::open(&twin).unwrap();
    a.replay_check().unwrap();
    let seqs_a: Vec<u64> = a.fold().facts().iter().map(|f| f.seq).collect();
    let seqs_b: Vec<u64> = b.fold().facts().iter().map(|f| f.seq).collect();
    assert_eq!(
        seqs_a, seqs_b,
        "HIGH R3: fact seqs vs twin after post-checkpoint append"
    );
    assert_eq!(a.fold().fingerprint(), b.fold().fingerprint());
    let _ = fs::remove_dir_all(&ckp);
    let _ = fs::remove_dir_all(&twin);
}
