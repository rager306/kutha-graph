//! M010: open recovers intern meanings; live intern logs Op::Define (ADR-011).

use kutha_common::Op;
use kutha_runtime::{store, Runtime};

#[test]
fn intern_appends_define_for_new_terms_only() {
    let mut rt = Runtime::default();
    assert_eq!(0, rt.log().len(), "bootstrap knows/knownBy must not log Define");
    let a = rt.intern("Alice");
    assert_eq!(1, rt.log().len());
    assert!(matches!(
        &rt.log().as_slice()[0].op,
        Op::Define { name } if name == "Alice"
    ));
    assert_eq!(a, rt.intern("Alice"), "re-intern is stable");
    assert_eq!(1, rt.log().len(), "existing term must not append again");
    let _ = rt.intern("knows");
    assert_eq!(1, rt.log().len(), "bootstrap relation re-intern stays silent");
    assert_eq!(0, rt.graph_len());
    rt.replay_check().unwrap();
}

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

        claim: None,
        delivery_key: None,
        polarity: None,
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

#[test]
fn open_without_snapshot_or_terms_file_recovers_from_define_ops() {
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

        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    let dir = std::env::temp_dir().join(format!("kutha-m010-s02-{}", uuid_like()));
    store::persist(&rt, &dir).unwrap();
    std::fs::remove_file(dir.join("snapshot.json")).unwrap();
    std::fs::remove_file(dir.join("terms.jsonl")).unwrap();
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
