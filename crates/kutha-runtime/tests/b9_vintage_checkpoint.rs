//! Phase 237 / M163 S01 — vintage checkpoint lineage fails closed (b9).
//! A blob without `behaviors_exact` must fail `store::open` even when digest,
//! fact_count, and fingerprint match. A twin that keeps the mark opens.

use kutha_common::{Event, Op, sha256_hex};
use kutha_runtime::{Runtime, store};
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
    common::PersistScratch::new(&format!("b9-vintage-{label}"))
}

fn no_horn_runtime() -> Runtime {
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

fn checkpoint_blob_digest(events: &[Event]) -> String {
    events
        .iter()
        .find_map(|e| match &e.op {
            Op::Checkpoint { blob_digest, .. } => Some(blob_digest.clone()),
            _ => None,
        })
        .expect("Op::Checkpoint")
}

fn strip_behaviors_exact(events: &mut [Event]) {
    let digest = checkpoint_blob_digest(events);
    let mut found = false;
    for e in events {
        if let Op::StoreBlob { digest: d, bytes } = &mut e.op {
            if *d == digest {
                let mut v: serde_json::Value =
                    serde_json::from_slice(bytes).expect("checkpoint Snapshot JSON");
                v.as_object_mut()
                    .expect("snapshot object")
                    .remove("behaviors_exact");
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

#[test]
fn unmarked_checkpoint_blob_fails_open() {
    let _g = tiny_limits();
    let rt = no_horn_runtime();

    let marked = persist_dir("marked");
    store::persist(&rt, &marked).unwrap();
    store::checkpoint_sealed_prefix(&marked).unwrap();
    store::open(&marked).expect("twin that keeps the mark must open");

    let unmarked = persist_dir("unmarked");
    copy_store(&marked, &unmarked);
    let mut events = export_events(&unmarked);
    strip_behaviors_exact(&mut events);
    recompute_blob_digest(&mut events);
    rewrite_on_disk_log(&unmarked, &events);

    match store::open(&unmarked) {
        Err(_) => {}
        Ok(_) => panic!("unmarked checkpoint blob must fail open (got Ok)"),
    }
}
