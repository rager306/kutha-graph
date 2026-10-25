//! Phase 234 / M160 S01 — StoreBlob digest on open (b8).
//! Same-length rewrite of an ordinary blob must fail `store::open`.

use kutha_common::{Event, EventId, Op, sha256_hex};
use kutha_runtime::{Runtime, RuntimeError, store};
use std::fs;
use std::path::Path;

mod common;

fn persist_dir(label: &str) -> common::PersistScratch {
    common::PersistScratch::new(&format!("b8-blob-{label}"))
}

fn ordinary_blob_runtime() -> (Runtime, String, Vec<u8>) {
    let mut rt = Runtime::default();
    let bytes = b"ordinary-blob-payload-m160".to_vec();
    let digest = sha256_hex(&bytes);
    rt.emit(Op::StoreBlob {
        digest: digest.clone(),
        bytes: bytes.clone(),
    })
    .unwrap();
    (rt, digest, bytes)
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

fn flip_one_blob_byte(events: &mut [Event], digest: &str) {
    let mut found = false;
    for e in events {
        if let Op::StoreBlob { digest: d, bytes } = &mut e.op {
            if d == digest && !bytes.is_empty() {
                let last = bytes.len() - 1;
                bytes[last] ^= 0x01;
                found = true;
                break;
            }
        }
    }
    assert!(found, "ordinary StoreBlob to flip");
}

#[test]
fn tampered_storeblob_same_length_fails_open() {
    let (rt, digest, bytes) = ordinary_blob_runtime();
    let orig_len = bytes.len();

    let twin = persist_dir("twin");
    store::persist(&rt, &twin).unwrap();
    store::open(&twin).expect("untouched ordinary StoreBlob must open");

    let bad = persist_dir("tamper");
    store::persist(&rt, &bad).unwrap();
    let mut events = export_events(&bad);
    flip_one_blob_byte(&mut events, &digest);
    let flipped = events
        .iter()
        .find_map(|e| match &e.op {
            Op::StoreBlob {
                digest: d,
                bytes: b,
            } if d == &digest => Some(b.clone()),
            _ => None,
        })
        .expect("flipped blob");
    assert_eq!(flipped.len(), orig_len, "same-length rewrite");
    assert_ne!(flipped, bytes);
    rewrite_on_disk_log(&bad, &events);

    match store::open(&bad) {
        Err(err) => {
            assert_eq!(err.kind(), std::io::ErrorKind::InvalidData);
            let expected = RuntimeError::UnknownRuleVersion {
                pin: digest.clone(),
            }
            .to_string();
            assert_eq!(err.to_string(), expected, "got {err}");
        }
        Ok(_) => panic!("tampered same-length StoreBlob must fail open (got Ok)"),
    }
}

fn knows_inverse_runtime() -> Runtime {
    let mut rt = Runtime::default();
    let a = rt.intern("Alice");
    let b = rt.intern("Bob");
    let knows = rt.intern("knows");
    rt.emit(Op::Assert {
        subject: a,
        relation: knows,
        object: b,
        valid_from: 2010,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    rt
}

fn inverse_behavior(rt: &Runtime) -> (EventId, String) {
    rt.log()
        .iter()
        .find_map(|e| match &e.op {
            Op::Behavior {
                name, rule_version, ..
            } if name == "inverse_knows" => Some((e.id, rule_version.clone())),
            _ => None,
        })
        .expect("inverse_knows Behavior")
}

#[test]
fn missing_horn_blob_stays_execution_replay_failed() {
    let twin = knows_inverse_runtime();
    let (behavior_id, pin) = inverse_behavior(&twin);
    assert!(!pin.is_empty(), "Horn pin must be nonempty");
    twin.replay_check()
        .expect("twin with Horn blob present must replay_check Ok");

    let events: Vec<Event> = twin
        .log()
        .as_slice()
        .iter()
        .filter(|e| !matches!(&e.op, Op::StoreBlob { digest, .. } if digest == &pin))
        .cloned()
        .collect();
    let missing = Runtime::from_dict_and_events(
        twin.dictionary().strings().to_vec(),
        events,
        twin.max_cascade,
    )
    .unwrap();
    let err = missing
        .replay_check()
        .expect_err("missing nonempty pin blob must fail closed");
    assert!(
        matches!(err, RuntimeError::ExecutionReplayFailed { event_id } if event_id == behavior_id),
        "expected ExecutionReplayFailed for {behavior_id}, got {err:?}"
    );
}
