//! Phase 238 / M164 S01 — harvest checkpoint behaviors only from a cited blob.
//! An ordinary StoreBlob that merely parses as a Snapshot is not lineage.

use kutha_common::{Event, EventId, Op, sha256_hex};
use kutha_runtime::{Runtime, store};
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
