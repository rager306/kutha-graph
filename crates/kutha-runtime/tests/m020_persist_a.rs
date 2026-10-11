//! M020 S03 / Phase 65 — atomic segment rewrite (PA-01 / PA-02).

use kutha_common::Op;
use kutha_runtime::{store, Runtime};
use std::ffi::OsString;
use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};

const VF_FIRST: u64 = 2010;
const T_OLD: u64 = 2015;

fn persist_dir(label: &str) -> PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("tmp")
        .join(format!("p65-store-{label}-{nanos}"));
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
        valid_to: Some(2020),
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    rt.emit_tt_anchor(T_OLD).unwrap();
    rt
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

fn active_seg(dir: &Path) -> PathBuf {
    let man = fs::read_to_string(dir.join(store::MANIFEST_REL)).unwrap();
    let v: serde_json::Value = serde_json::from_str(&man).unwrap();
    dir.join(v["active"].as_str().expect("manifest.active"))
}

fn tmp_sibling(path: &Path) -> PathBuf {
    let mut os: OsString = path.as_os_str().to_owned();
    os.push(".tmp");
    PathBuf::from(os)
}

/// PA-01: re-persist must mint a generation-unique active name (inode changes).
#[test]
fn segment_inode_changes_on_persist_rename() {
    let mut rt = seed_runtime();
    let dir = persist_dir("seg-ino");
    store::persist(&rt, &dir).unwrap();
    let path = active_seg(&dir);
    let name1 = path.file_name().unwrap().to_string_lossy().into_owned();
    let ino1 = fs::metadata(&path).unwrap().ino();
    emit_product_assert(&mut rt, "p65-ino-article", "p65-ino-80");
    store::persist(&rt, &dir).unwrap();
    let path2 = active_seg(&dir);
    let name2 = path2.file_name().unwrap().to_string_lossy().into_owned();
    let ino2 = fs::metadata(&path2).unwrap().ino();
    assert_ne!(
        name1, name2,
        "re-persist must not reuse the previous live segment name"
    );
    assert!(
        !path.exists(),
        "obsolete generation must be deleted after the new manifest is durable"
    );
    assert_ne!(ino1, ino2, "new generation active must be a distinct inode");
    let _ = fs::remove_dir_all(&dir);
}

/// PA-02: abort before rename leaves dest bytes and an openable store.
#[test]
fn abort_segment_replace_keeps_dest_and_opens() {
    let mut rt = seed_runtime();
    let dir = persist_dir("seg-abort");
    store::persist(&rt, &dir).unwrap();
    let path = active_seg(&dir);
    let before = fs::read(&path).unwrap();
    let fp_before = store::open(&dir).unwrap().fold().fingerprint();

    emit_product_assert(&mut rt, "p65-abort-article", "p65-abort-80");
    let events: Vec<_> = rt.log().iter().cloned().collect();
    store::abort_write_segment_before_rename(&path, &events, false, [0u8; 32])
        .expect_err("abort seam must not rename");

    let after = fs::read(&path).unwrap();
    assert_eq!(before, after, "dest bytes must stay intact across abort");
    let opened = store::open(&dir).unwrap();
    assert_eq!(
        opened.fold().fingerprint(),
        fp_before,
        "store must still open with pre-abort fold fingerprint"
    );
    assert!(
        tmp_sibling(&path).exists(),
        "tmp sibling should remain after abort-before-rename"
    );
    let _ = fs::remove_dir_all(&dir);
}
