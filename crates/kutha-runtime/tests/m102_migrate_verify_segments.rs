//! M102 S01 / Phase 173 — failed migrate verify must not leave seg files
//! (CE whole-repo 2026-10-15 residual).

use kutha_common::Op;
use kutha_runtime::{Runtime, store};
use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;

fn persist_dir(label: &str) -> PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("tmp")
        .join(format!(
            "p173-store-{label}-{}-{nanos}",
            std::process::id()
        ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn seg_logs(dir: &std::path::Path) -> Vec<String> {
    let mut names = Vec::new();
    for ent in fs::read_dir(dir).unwrap() {
        let ent = ent.unwrap();
        let name = ent.file_name().to_string_lossy().into_owned();
        if name.starts_with("seg-") && name.ends_with(".log") {
            names.push(name);
        }
    }
    names.sort();
    names
}

/// Verify failure drops the segmented generation and leaves the legacy log openable.
#[test]
fn migrate_verify_fault_leaves_no_segment_files() {
    let mut rt = Runtime::default();
    let s = rt.intern("m102-s");
    let r = rt.intern("knows");
    let o = rt.intern("m102-o");
    rt.emit(Op::Assert {
        subject: s,
        relation: r,
        object: o,
        valid_from: 2010,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();

    let dir = persist_dir("verify-segs");
    store::ensure_legacy_dir(&dir).unwrap();
    store::persist(&rt, &dir).unwrap();

    let err = store::migrate_with_verify_fault(&dir, store::MigrateVerifyFault::FlipSealedByte)
        .expect_err("verify fault must fail");
    assert_eq!(err.kind(), ErrorKind::InvalidData);
    assert!(
        !dir.join(store::MANIFEST_REL).exists(),
        "failed verify removes the manifest"
    );
    let left = seg_logs(&dir);
    assert!(
        left.is_empty(),
        "failed verify must remove seg files, left {left:?}"
    );
    assert!(dir.join("events.jsonl").exists());
    assert!(dir.join("events.wal").exists());
    let opened = store::open(&dir).expect("legacy open after failed migrate");
    assert!(
        !opened.log().as_slice().is_empty(),
        "legacy log stays readable after the segmented generation is dropped"
    );
    let _ = fs::remove_dir_all(&dir);
}
