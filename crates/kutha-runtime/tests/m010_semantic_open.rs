//! M010 S01: open recovers intern meanings without snapshot.json (ADR-011).

use kutha_common::Op;
use kutha_runtime::{store, Runtime};

#[test]
fn open_without_snapshot_recovers_intern_meanings() {
    let mut rt = Runtime::default();
    let a = rt.intern("A");
    let b = rt.intern("B");
    let r = rt.intern("knows");
    rt.emit(Op::Assert {
        subject: a,
        relation: r,
        object: b,
        valid_from: 0,
        valid_to: None,
    })
    .unwrap();
    let dir = std::env::temp_dir().join(format!("kutha-m010-{}", uuid_like()));
    store::persist(&rt, &dir).unwrap();
    std::fs::remove_file(dir.join("snapshot.json")).unwrap();
    let opened = store::open(&dir).unwrap();
    assert_eq!(opened.fold().fingerprint(), rt.fold().fingerprint());
    assert_eq!(opened.dictionary().lookup(a), Some("A"));
    assert_eq!(opened.dictionary().lookup(b), Some("B"));
    assert_eq!(opened.dictionary().lookup(r), Some("knows"));
    opened.replay_check().unwrap();
    let _ = std::fs::remove_dir_all(&dir);
}

fn uuid_like() -> String {
    format!(
        "{:x}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    )
}
