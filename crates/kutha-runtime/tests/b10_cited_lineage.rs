//! Phase 238 / M164 S01 — harvest checkpoint behaviors only from a cited blob.
//! An ordinary StoreBlob that merely parses as a Snapshot is not lineage.

use kutha_common::{Event, EventId, Op, RuleAst, RuleAtom, RuleTerm, sha256_hex};
use kutha_runtime::{Runtime, Snapshot, store};
use std::fs;
use std::path::Path;

mod common;

fn persist_dir(label: &str) -> common::PersistScratch {
    common::PersistScratch::new(&format!("b10-cite-{label}"))
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

fn seed_runtime() -> Runtime {
    let mut rt = Runtime::default();
    let article = rt.intern("art-12-speed");
    let in_force = rt.intern("inForceAs");
    for i in 0..6 {
        let object = rt.intern(&format!("obj-{i}"));
        rt.emit(Op::Assert {
            subject: article,
            relation: in_force,
            object,
            valid_from: 2010,
            valid_to: Some(2020),
            claim: None,
            delivery_key: Some(format!("dk-{i}")),
            polarity: None,
        })
        .unwrap();
    }
    rt
}

#[test]
fn ordinary_snapshot_blob_is_not_harvested() {
    let mut rt = seed_runtime();
    let subject = rt.intern("art-12-speed");
    let relation = rt.intern("inForceAs");
    let object = rt.intern("obj-0");
    let mut forged = Event::new(
        Op::Behavior {
            name: "forged-uncited-snapshot".into(),
            caused_by: EventId::nil(),
            rule_version: "never-on-log".into(),
            subject,
            relation,
            object,
            valid_from: 2010,
            valid_to: None,
        },
        0u64,
    );
    let forged_id = "00000000-0000-4000-8000-000000000b10"
        .parse::<EventId>()
        .expect("fixed forged EventId");
    forged.id = forged_id;
    assert!(
        rt.log().iter().all(|e| e.id != forged_id),
        "forged Behavior id must not occur on the log"
    );

    let mut snap = rt.snapshot();
    assert!(snap.behaviors_exact);
    snap.behaviors = vec![forged];
    let bytes = serde_json::to_vec(&snap).expect("forged Snapshot JSON");
    let digest = sha256_hex(&bytes);
    rt.emit(Op::StoreBlob {
        digest: digest.clone(),
        bytes,
    })
    .unwrap();

    let extra = rt.intern("obj-extra");
    rt.emit(Op::Assert {
        subject,
        relation,
        object: extra,
        valid_from: 2010,
        valid_to: Some(2020),
        claim: None,
        delivery_key: Some("dk-extra".into()),
        polarity: None,
    })
    .unwrap();

    let dir = persist_dir("ordinary");
    store::persist(&rt, &dir).unwrap();
    let events = export_events(&dir);
    assert!(
        events
            .iter()
            .any(|e| matches!(&e.op, Op::StoreBlob { digest: d, .. } if *d == digest)),
        "ordinary StoreBlob on disk"
    );
    assert!(
        events.iter().all(|e| !matches!(
            &e.op,
            Op::Checkpoint { blob_digest, .. } if *blob_digest == digest
        )),
        "no Checkpoint cites the ordinary blob"
    );
    rewrite_on_disk_log(&dir, &events);
    store::checkpoint_sealed_prefix(&dir).unwrap();

    let opened =
        store::open(&dir).expect("checkpointed store with ordinary Snapshot blob must open");
    assert!(
        opened
            .snapshot()
            .behaviors
            .iter()
            .all(|e| e.id != forged_id),
        "ordinary uncited Snapshot blob must not harvest Behavior {forged_id}"
    );
}

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

fn horn_runtime() -> Runtime {
    let mut rt = Runtime::default();
    rt.emit_allow("derive_pq", 0, None).unwrap();
    let head = RuleAtom::new(
        "relatedTo",
        RuleTerm::var(0).unwrap(),
        RuleTerm::constant("derived").unwrap(),
    )
    .unwrap();
    let prem = RuleAtom::new(
        "relatedTo",
        RuleTerm::var(0).unwrap(),
        RuleTerm::constant("true").unwrap(),
    )
    .unwrap();
    let ast = RuleAst::horn(head, vec![prem]).unwrap();
    let rule_pin = ast.hash();
    rt.emit_rule_ast(&ast, 2010, None).unwrap();
    let p = rt.intern("P");
    let rel = rt.intern("relatedTo");
    let true_ = rt.intern("true");
    let derived = rt.intern("derived");
    let asserted = rt
        .emit(Op::Assert {
            subject: p,
            relation: rel,
            object: true_,
            valid_from: 2010,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap();
    let cause = asserted.receipt.event_ids[0];
    rt.emit(Op::Behavior {
        name: "derive_pq".into(),
        caused_by: cause,
        rule_version: rule_pin,
        subject: p,
        relation: rel,
        object: derived,
        valid_from: 2010,
        valid_to: None,
    })
    .unwrap();
    let relation = rt.intern("inForceAs");
    for i in 0..6 {
        let subject = rt.intern(&format!("art-{i}"));
        let object = rt.intern(&format!("obj-{i}"));
        rt.emit(Op::Assert {
            subject,
            relation,
            object,
            valid_from: 2010,
            valid_to: Some(2020),
            claim: None,
            delivery_key: Some(format!("dk-{i}")),
            polarity: None,
        })
        .unwrap();
    }
    rt
}

fn snapshot_blob_count(events: &[Event]) -> usize {
    events
        .iter()
        .filter(|e| match &e.op {
            Op::StoreBlob { bytes, .. } => serde_json::from_slice::<Snapshot>(bytes).is_ok(),
            _ => false,
        })
        .count()
}

fn behavior_name_count(rt: &Runtime, name: &str) -> usize {
    rt.snapshot()
        .behaviors
        .iter()
        .filter(|e| matches!(&e.op, Op::Behavior { name: n, .. } if n == name))
        .count()
}

/// Prefix `Op::Behavior` is copied even when the prefix has no Snapshot blob.
#[test]
fn prefix_behavior_copied_without_snapshot_blob() {
    let _g = tiny_limits();
    let rt = horn_runtime();
    let dir = persist_dir("prefix-behavior");
    store::persist(&rt, &dir).unwrap();
    let events = export_events(&dir);
    assert!(
        events
            .iter()
            .any(|e| matches!(&e.op, Op::Behavior { name, .. } if name == "derive_pq")),
        "prefix must carry Op::Behavior before checkpoint"
    );
    assert!(
        events
            .iter()
            .all(|e| !matches!(e.op, Op::Checkpoint { .. })),
        "first-checkpoint prefix has no Op::Checkpoint"
    );
    assert_eq!(
        snapshot_blob_count(&events),
        0,
        "first-checkpoint prefix has no StoreBlob that parses as Snapshot"
    );
    store::checkpoint_sealed_prefix(&dir).unwrap();
    let opened = store::open(&dir).expect("Horn checkpoint with prefix Behavior must open");
    assert_eq!(
        behavior_name_count(&opened, "derive_pq"),
        1,
        "prefix Op::Behavior must be copied into the next checkpoint Snapshot"
    );
}

/// Cited checkpoint sibling plus an extra uncited Snapshot blob: only the cited blob contributes.
#[test]
fn uncited_snapshot_sibling_of_cited_checkpoint_is_not_harvested() {
    let _g = tiny_limits();
    let mut rt = horn_runtime();
    let subject = rt.intern("art-0");
    let relation = rt.intern("inForceAs");
    let object = rt.intern("obj-0");
    let mut forged = Event::new(
        Op::Behavior {
            name: "forged-uncited-sibling".into(),
            caused_by: EventId::nil(),
            rule_version: "never-on-log".into(),
            subject,
            relation,
            object,
            valid_from: 2010,
            valid_to: None,
        },
        0u64,
    );
    let forged_id = "00000000-0000-4000-8000-000000000b15"
        .parse::<EventId>()
        .expect("fixed forged EventId");
    forged.id = forged_id;
    assert!(
        rt.log().iter().all(|e| e.id != forged_id),
        "forged Behavior id must not occur on the log"
    );

    let mut snap = rt.snapshot();
    assert!(snap.behaviors_exact);
    snap.behaviors = vec![forged];
    let bytes = serde_json::to_vec(&snap).expect("forged Snapshot JSON");
    let poison_digest = sha256_hex(&bytes);
    rt.emit(Op::StoreBlob {
        digest: poison_digest.clone(),
        bytes,
    })
    .unwrap();

    let dir = persist_dir("sibling");
    store::persist(&rt, &dir).unwrap();
    store::checkpoint_sealed_prefix(&dir).unwrap();

    let after_first = export_events(&dir);
    assert!(
        after_first.iter().any(|e| matches!(
            &e.op,
            Op::StoreBlob { digest, .. } if *digest == poison_digest
        )),
        "uncited Snapshot blob must remain on the rewritten log"
    );
    assert!(
        after_first
            .iter()
            .any(|e| matches!(e.op, Op::Checkpoint { .. })),
        "first checkpoint receipt must be on the log"
    );
    assert!(
        after_first.iter().all(|e| !matches!(
            &e.op,
            Op::Checkpoint { blob_digest, .. } if *blob_digest == poison_digest
        )),
        "no Checkpoint may cite the poison blob"
    );

    rewrite_on_disk_log(&dir, &after_first);
    store::checkpoint_sealed_prefix(&dir).unwrap();

    let opened = store::open(&dir).expect("re-checkpoint with uncited sibling must open");
    let snap = opened.snapshot();
    assert!(
        snap.behaviors.iter().all(|e| e.id != forged_id),
        "uncited Snapshot sibling must not launder Behavior {forged_id}"
    );
    assert_eq!(
        snap.behaviors
            .iter()
            .filter(|e| matches!(&e.op, Op::Behavior { name, .. } if name == "derive_pq"))
            .count(),
        1,
        "cited checkpoint blob must still contribute Horn behaviors"
    );
}
